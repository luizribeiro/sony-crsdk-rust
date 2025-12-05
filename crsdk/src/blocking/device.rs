//! Blocking camera device connection and control

#[allow(unused_imports)]
use asyncwrap::async_wrap;
use asyncwrap::blocking_impl;

use crate::command::{CommandId, CommandParam, LockIndicator};
use crate::error::{Error, Result};
use crate::event::CameraEvent;
use crate::event_sender::EventSender;
use crate::property::{
    device_property_from_sdk, device_property_from_sdk_debug, DeviceProperty, DriveMode,
    ExposureProgram, FlashMode, FocusArea, FocusMode, MeteringMode, WhiteBalance,
};
use crate::types::{
    ip_to_sdk_format, CameraModel, ConnectionInfo, ConnectionType, DiscoveredCamera, MacAddr,
};
use crate::Sdk;
use crsdk_sys::DevicePropertyCode;
use std::ffi::{c_void, CString};
use std::net::Ipv4Addr;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::mpsc;

static SDK_INITIALIZED: AtomicBool = AtomicBool::new(false);

fn ensure_sdk_initialized() -> Result<()> {
    if !SDK_INITIALIZED.load(Ordering::Acquire) {
        let sdk = Sdk::init()?;
        std::mem::forget(sdk); // Keep SDK alive for program lifetime
        SDK_INITIALIZED.store(true, Ordering::Release);
    }
    Ok(())
}

/// Discover cameras connected via network and USB
///
/// This function enumerates all cameras that are currently connected and
/// visible to the SDK. It searches on both network (Ethernet/WiFi) and USB.
///
/// # Arguments
///
/// * `timeout_secs` - How long to scan for cameras (1-10 seconds recommended)
///
/// # Returns
///
/// A vector of discovered cameras. Empty if no cameras found.
pub fn discover_cameras(timeout_secs: u8) -> Result<Vec<DiscoveredCamera>> {
    ensure_sdk_initialized()?;

    let mut enum_ptr: *mut crsdk_sys::SCRSDK::ICrEnumCameraObjectInfo = ptr::null_mut();

    let result = unsafe { crsdk_sys::SCRSDK::EnumCameraObjects(&mut enum_ptr, timeout_secs) };

    if result != 0 {
        return Err(Error::from_sdk_error(result as u32));
    }

    if enum_ptr.is_null() {
        return Ok(Vec::new());
    }

    let count = unsafe { crsdk_sys::crsdk_enum_camera_get_count(enum_ptr) };
    let mut cameras = Vec::with_capacity(count as usize);

    for i in 0..count {
        let info_ptr = unsafe { crsdk_sys::crsdk_enum_camera_get_info(enum_ptr, i) };
        if info_ptr.is_null() {
            continue;
        }

        let camera = camera_info_from_sdk(info_ptr);
        cameras.push(camera);
    }

    unsafe {
        crsdk_sys::crsdk_enum_camera_release(enum_ptr);
    }

    Ok(cameras)
}

/// Parse a string field from SDK camera info.
///
/// # Safety
/// The caller must ensure `info` is a valid pointer to ICrCameraObjectInfo
/// that remains valid for the duration of this call.
unsafe fn parse_sdk_string(
    info: *const crsdk_sys::SCRSDK::ICrCameraObjectInfo,
    get_ptr: unsafe extern "C" fn(*const crsdk_sys::SCRSDK::ICrCameraObjectInfo) -> *const i8,
    get_size: unsafe extern "C" fn(*const crsdk_sys::SCRSDK::ICrCameraObjectInfo) -> u32,
) -> String {
    // SAFETY: Caller guarantees info is valid. The SDK returns a pointer to
    // internal buffer that is valid for the lifetime of the enumeration.
    let ptr = unsafe { get_ptr(info) };
    let size = unsafe { get_size(info) };
    if ptr.is_null() || size == 0 {
        String::new()
    } else {
        // SAFETY: We verified ptr is non-null and size is from the SDK.
        // The SDK guarantees the buffer contains at least `size` bytes.
        let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, size as usize) };
        String::from_utf8_lossy(slice)
            .trim_end_matches('\0')
            .to_string()
    }
}

fn camera_info_from_sdk(info: *const crsdk_sys::SCRSDK::ICrCameraObjectInfo) -> DiscoveredCamera {
    // SAFETY: info pointer validity is guaranteed by the SDK's enumeration API.
    // GetCameraObjectInfo returns valid pointers for indices 0..GetCount()-1.
    let model = unsafe {
        parse_sdk_string(
            info,
            crsdk_sys::crsdk_camera_info_get_model,
            crsdk_sys::crsdk_camera_info_get_model_size,
        )
    };

    let name = unsafe {
        parse_sdk_string(
            info,
            crsdk_sys::crsdk_camera_info_get_name,
            crsdk_sys::crsdk_camera_info_get_name_size,
        )
    };

    let connection_type = unsafe {
        // SAFETY: info is valid per caller contract
        let ptr = crsdk_sys::crsdk_camera_info_get_connection_type(info);
        if ptr.is_null() {
            tracing::warn!("Connection type pointer is null");
            ConnectionType::Unknown
        } else {
            // SAFETY: SDK guarantees null-terminated string if pointer is non-null
            let cstr = std::ffi::CStr::from_ptr(ptr);
            let type_str = cstr.to_str().unwrap_or("");
            match type_str.to_lowercase().as_str() {
                s if s.contains("ether") || s.contains("network") || s.contains("ip") => {
                    ConnectionType::Network
                }
                s if s.contains("usb") => ConnectionType::Usb,
                other => {
                    if !other.is_empty() {
                        tracing::debug!("Unknown connection type: {}", other);
                    }
                    ConnectionType::Unknown
                }
            }
        }
    };

    // SAFETY: info is valid per caller contract
    let ip_address = unsafe {
        let ip_packed = crsdk_sys::crsdk_camera_info_get_ip_address(info);
        if ip_packed == 0 {
            None
        } else {
            // SDK stores IP in little-endian format
            let bytes = ip_packed.to_le_bytes();
            Some(Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]))
        }
    };

    // SAFETY: info is valid per caller contract
    let mac_address = unsafe {
        let mac_ptr = crsdk_sys::crsdk_camera_info_get_mac_address(info);
        let mac_size = crsdk_sys::crsdk_camera_info_get_mac_address_size(info);
        if mac_ptr.is_null() || mac_size < 6 {
            None
        } else {
            let mut bytes = [0u8; 6];
            // SAFETY: We verified mac_ptr is non-null and mac_size >= 6
            ptr::copy_nonoverlapping(mac_ptr, bytes.as_mut_ptr(), 6);
            Some(MacAddr::new(bytes))
        }
    };

    // SAFETY: info is valid per caller contract
    let ssh_supported = unsafe { crsdk_sys::crsdk_camera_info_get_ssh_support(info) != 0 };

    // SAFETY: info is valid per caller contract
    let usb_pid = unsafe {
        let pid = crsdk_sys::crsdk_camera_info_get_usb_pid(info);
        if pid == 0 {
            None
        } else {
            Some(pid)
        }
    };

    DiscoveredCamera {
        model,
        name,
        connection_type,
        ip_address,
        mac_address,
        ssh_supported,
        usb_pid,
    }
}

fn create_camera_info(
    ip: Ipv4Addr,
    mac: MacAddr,
    model: CameraModel,
    ssh_enabled: bool,
) -> Result<*mut crsdk_sys::SCRSDK::ICrCameraObjectInfo> {
    let mut camera_info_ptr: *mut crsdk_sys::SCRSDK::ICrCameraObjectInfo = ptr::null_mut();

    let ip_sdk = ip_to_sdk_format(ip);
    let mac_bytes = mac.as_bytes();
    let ssh_support = if ssh_enabled { 1 } else { 0 };
    let model_sdk = model.to_sdk_value();

    let result = unsafe {
        crsdk_sys::SCRSDK::CreateCameraObjectInfoEthernetConnection(
            &mut camera_info_ptr,
            model_sdk,
            ip_sdk,
            mac_bytes.as_ptr() as *mut u8,
            ssh_support,
        )
    };

    if result != 0 || camera_info_ptr.is_null() {
        return Err(Error::from_sdk_error(result as u32));
    }

    Ok(camera_info_ptr)
}

/// A connected camera device (blocking/synchronous API)
pub struct CameraDevice {
    handle: i64,
    model: CameraModel,
    /// Event receiver - events from SDK callbacks arrive here
    event_receiver: mpsc::UnboundedReceiver<CameraEvent>,
    /// Callback pointer - must be destroyed when device is dropped
    callback_ptr: *mut crsdk_sys::SCRSDK::IDeviceCallback,
    /// Event sender pointer - must be reclaimed when device is dropped
    event_sender_ptr: *mut c_void,
}

// SAFETY: CameraDevice can be sent between threads because:
// - handle is just an i64
// - model is Copy
// - event_receiver is Send
// - callback_ptr and event_sender_ptr are only accessed in Drop
unsafe impl Send for CameraDevice {}

// SAFETY: CameraDevice can be shared between threads because:
// - All mutable state access goes through the SDK which handles synchronization
// - The raw pointers (callback_ptr, event_sender_ptr) are only accessed in Drop
// - The event_receiver is accessed via &mut self (exclusive access)
unsafe impl Sync for CameraDevice {}

#[blocking_impl(crate::CameraDevice, strategy = "block_in_place")]
impl CameraDevice {
    /// Create a new builder for configuring camera connection
    pub fn builder() -> CameraDeviceBuilder {
        CameraDeviceBuilder::new()
    }

    /// Get the camera model
    #[async_wrap]
    pub fn model(&self) -> CameraModel {
        self.model
    }

    /// Get a property from the camera
    ///
    /// Returns the property with its current value, possible values, and metadata.
    #[async_wrap]
    pub fn get_property(&self, code: DevicePropertyCode) -> Result<DeviceProperty> {
        let mut properties_ptr: *mut crsdk_sys::SCRSDK::CrDeviceProperty = ptr::null_mut();
        let mut num_properties: i32 = 0;

        let result = unsafe {
            crsdk_sys::SCRSDK::GetDeviceProperties(
                self.handle,
                &mut properties_ptr,
                &mut num_properties,
            )
        };

        if result != 0 {
            return Err(Error::from_sdk_error(result as u32));
        }

        if properties_ptr.is_null() || num_properties == 0 {
            return Err(Error::PropertyNotSupported);
        }

        let target_code = code.as_raw();
        let mut found_property: Option<DeviceProperty> = None;

        unsafe {
            for i in 0..num_properties as usize {
                let prop = &*properties_ptr.add(i);
                if prop.code == target_code {
                    found_property = Some(device_property_from_sdk(prop));
                    break;
                }
            }

            crsdk_sys::SCRSDK::ReleaseDeviceProperties(self.handle, properties_ptr);
        }

        found_property.ok_or(Error::PropertyNotSupported)
    }

    /// Get all properties from the camera
    ///
    /// Returns all properties the camera currently exposes.
    /// Useful for debugging what properties are available.
    #[async_wrap]
    pub fn get_all_properties(&self) -> Result<Vec<DeviceProperty>> {
        let mut properties_ptr: *mut crsdk_sys::SCRSDK::CrDeviceProperty = ptr::null_mut();
        let mut num_properties: i32 = 0;

        let result = unsafe {
            crsdk_sys::SCRSDK::GetDeviceProperties(
                self.handle,
                &mut properties_ptr,
                &mut num_properties,
            )
        };

        if result != 0 {
            return Err(Error::from_sdk_error(result as u32));
        }

        if properties_ptr.is_null() || num_properties == 0 {
            return Ok(Vec::new());
        }

        let mut properties = Vec::with_capacity(num_properties as usize);

        unsafe {
            for i in 0..num_properties as usize {
                let prop = &*properties_ptr.add(i);
                properties.push(device_property_from_sdk(prop));
            }

            crsdk_sys::SCRSDK::ReleaseDeviceProperties(self.handle, properties_ptr);
        }

        Ok(properties)
    }

    /// Get all properties with debug info (for debugging SDK values)
    #[async_wrap]
    pub fn get_all_properties_debug(&self) -> Result<Vec<(DeviceProperty, String)>> {
        let mut properties_ptr: *mut crsdk_sys::SCRSDK::CrDeviceProperty = ptr::null_mut();
        let mut num_properties: i32 = 0;

        let result = unsafe {
            crsdk_sys::SCRSDK::GetDeviceProperties(
                self.handle,
                &mut properties_ptr,
                &mut num_properties,
            )
        };

        if result != 0 {
            return Err(Error::from_sdk_error(result as u32));
        }

        if properties_ptr.is_null() || num_properties == 0 {
            return Ok(Vec::new());
        }

        let mut properties = Vec::with_capacity(num_properties as usize);

        unsafe {
            for i in 0..num_properties as usize {
                let prop = &*properties_ptr.add(i);
                properties.push(device_property_from_sdk_debug(prop));
            }

            crsdk_sys::SCRSDK::ReleaseDeviceProperties(self.handle, properties_ptr);
        }

        Ok(properties)
    }

    /// Set a property on the camera
    ///
    /// The value should be a raw u64 value. Use the enum's `as_raw()` method
    /// for enumerated properties like FocusMode or WhiteBalance.
    #[async_wrap]
    pub fn set_property(&self, code: DevicePropertyCode, value: u64) -> Result<()> {
        let prop = self.get_property(code)?;

        if !prop.is_writable() {
            return Err(Error::PropertyNotWritable);
        }

        if !prop.is_valid_value(value) {
            return Err(Error::InvalidPropertyValue);
        }

        let mut sdk_prop = crsdk_sys::SCRSDK::CrDeviceProperty {
            code: code.as_raw(),
            valueType: 0,
            enableFlag: 0,
            variableFlag: 0,
            currentValue: value,
            currentStr: ptr::null_mut(),
            valuesSize: 0,
            values: ptr::null_mut(),
            getSetValuesSize: 0,
            getSetValues: ptr::null_mut(),
        };

        let result = unsafe { crsdk_sys::SCRSDK::SetDeviceProperty(self.handle, &mut sdk_prop) };

        if result != 0 {
            return Err(Error::from_sdk_error(result as u32));
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Convenience methods for common properties
    // -------------------------------------------------------------------------

    /// Get the current focus mode
    #[async_wrap]
    pub fn focus_mode(&self) -> Result<FocusMode> {
        let prop = self.get_property(DevicePropertyCode::FocusMode)?;
        FocusMode::from_raw(prop.current_value).ok_or(Error::InvalidPropertyValue)
    }

    /// Set the focus mode
    #[async_wrap]
    pub fn set_focus_mode(&self, mode: FocusMode) -> Result<()> {
        self.set_property(DevicePropertyCode::FocusMode, mode.as_raw())
    }

    /// Get the current white balance setting
    #[async_wrap]
    pub fn white_balance(&self) -> Result<WhiteBalance> {
        let prop = self.get_property(DevicePropertyCode::WhiteBalance)?;
        WhiteBalance::from_raw(prop.current_value).ok_or(Error::InvalidPropertyValue)
    }

    /// Set the white balance
    #[async_wrap]
    pub fn set_white_balance(&self, wb: WhiteBalance) -> Result<()> {
        self.set_property(DevicePropertyCode::WhiteBalance, wb.as_raw())
    }

    /// Get the current exposure program mode
    #[async_wrap]
    pub fn exposure_program(&self) -> Result<ExposureProgram> {
        let prop = self.get_property(DevicePropertyCode::ExposureProgramMode)?;
        ExposureProgram::from_raw(prop.current_value).ok_or(Error::InvalidPropertyValue)
    }

    /// Set the exposure program mode
    #[async_wrap]
    pub fn set_exposure_program(&self, program: ExposureProgram) -> Result<()> {
        self.set_property(DevicePropertyCode::ExposureProgramMode, program.as_raw())
    }

    /// Get the current drive mode
    #[async_wrap]
    pub fn drive_mode(&self) -> Result<DriveMode> {
        let prop = self.get_property(DevicePropertyCode::DriveMode)?;
        DriveMode::from_raw(prop.current_value).ok_or(Error::InvalidPropertyValue)
    }

    /// Set the drive mode
    #[async_wrap]
    pub fn set_drive_mode(&self, mode: DriveMode) -> Result<()> {
        self.set_property(DevicePropertyCode::DriveMode, mode.as_raw())
    }

    /// Get the current metering mode
    #[async_wrap]
    pub fn metering_mode(&self) -> Result<MeteringMode> {
        let prop = self.get_property(DevicePropertyCode::MeteringMode)?;
        MeteringMode::from_raw(prop.current_value).ok_or(Error::InvalidPropertyValue)
    }

    /// Set the metering mode
    #[async_wrap]
    pub fn set_metering_mode(&self, mode: MeteringMode) -> Result<()> {
        self.set_property(DevicePropertyCode::MeteringMode, mode.as_raw())
    }

    /// Get the current flash mode
    #[async_wrap]
    pub fn flash_mode(&self) -> Result<FlashMode> {
        let prop = self.get_property(DevicePropertyCode::FlashMode)?;
        FlashMode::from_raw(prop.current_value).ok_or(Error::InvalidPropertyValue)
    }

    /// Set the flash mode
    #[async_wrap]
    pub fn set_flash_mode(&self, mode: FlashMode) -> Result<()> {
        self.set_property(DevicePropertyCode::FlashMode, mode.as_raw())
    }

    /// Get the current focus area
    #[async_wrap]
    pub fn focus_area(&self) -> Result<FocusArea> {
        let prop = self.get_property(DevicePropertyCode::FocusArea)?;
        FocusArea::from_raw(prop.current_value).ok_or(Error::InvalidPropertyValue)
    }

    /// Set the focus area
    #[async_wrap]
    pub fn set_focus_area(&self, area: FocusArea) -> Result<()> {
        self.set_property(DevicePropertyCode::FocusArea, area.as_raw())
    }

    /// Get the current ISO sensitivity (raw value)
    #[async_wrap]
    pub fn iso(&self) -> Result<u64> {
        let prop = self.get_property(DevicePropertyCode::IsoSensitivity)?;
        Ok(prop.current_value)
    }

    /// Set the ISO sensitivity
    #[async_wrap]
    pub fn set_iso(&self, value: u64) -> Result<()> {
        self.set_property(DevicePropertyCode::IsoSensitivity, value)
    }

    /// Get the current aperture/f-number (raw SDK value)
    #[async_wrap]
    pub fn aperture(&self) -> Result<u64> {
        let prop = self.get_property(DevicePropertyCode::FNumber)?;
        Ok(prop.current_value)
    }

    /// Set the aperture/f-number
    #[async_wrap]
    pub fn set_aperture(&self, value: u64) -> Result<()> {
        self.set_property(DevicePropertyCode::FNumber, value)
    }

    /// Get the current shutter speed (raw SDK value)
    #[async_wrap]
    pub fn shutter_speed(&self) -> Result<u64> {
        let prop = self.get_property(DevicePropertyCode::ShutterSpeed)?;
        Ok(prop.current_value)
    }

    /// Set the shutter speed
    #[async_wrap]
    pub fn set_shutter_speed(&self, value: u64) -> Result<()> {
        self.set_property(DevicePropertyCode::ShutterSpeed, value)
    }

    // -------------------------------------------------------------------------
    // Command operations (shooting, recording, AF)
    // -------------------------------------------------------------------------

    /// Send a command to the camera
    fn send_command(&self, command: CommandId, param: CommandParam) -> Result<()> {
        let result = unsafe {
            crsdk_sys::SCRSDK::SendCommand(self.handle, command.as_raw(), param.as_raw() as u16)
        };

        if result != 0 {
            return Err(Error::from_sdk_error(result as u32));
        }

        Ok(())
    }

    /// Set the S1 (half-press shutter) lock state for autofocus
    fn set_s1_lock(&self, lock: LockIndicator) -> Result<()> {
        let mut sdk_prop = crsdk_sys::SCRSDK::CrDeviceProperty {
            code: crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_S1,
            valueType: crsdk_sys::SCRSDK::CrDataType_CrDataType_UInt16 as u32,
            enableFlag: 0,
            variableFlag: 0,
            currentValue: lock.as_raw(),
            currentStr: ptr::null_mut(),
            valuesSize: 0,
            values: ptr::null_mut(),
            getSetValuesSize: 0,
            getSetValues: ptr::null_mut(),
        };

        let result = unsafe { crsdk_sys::SCRSDK::SetDeviceProperty(self.handle, &mut sdk_prop) };

        if result != 0 {
            return Err(Error::from_sdk_error(result as u32));
        }

        Ok(())
    }

    /// Take a photo (shutter release)
    ///
    /// This performs a full shutter release cycle: press down, brief delay, release up.
    /// The camera must be in a mode that supports still capture (Photo mode, not Movie mode).
    #[async_wrap]
    pub fn capture(&self) -> Result<()> {
        self.send_command(CommandId::Release, CommandParam::Down)?;
        std::thread::sleep(Duration::from_millis(35));
        self.send_command(CommandId::Release, CommandParam::Up)?;
        Ok(())
    }

    /// Half-press the shutter to activate autofocus
    ///
    /// This is equivalent to pressing the shutter button halfway on a physical camera.
    /// The camera will attempt to focus on the current subject. Call `release_shutter()`
    /// to release the half-press state.
    #[async_wrap]
    pub fn half_press_shutter(&self) -> Result<()> {
        self.set_s1_lock(LockIndicator::Locked)
    }

    /// Release the half-pressed shutter
    ///
    /// This releases the autofocus lock initiated by `half_press_shutter()`.
    #[async_wrap]
    pub fn release_shutter(&self) -> Result<()> {
        self.set_s1_lock(LockIndicator::Unlocked)
    }

    /// Autofocus and capture in one operation
    ///
    /// Half-presses to focus, waits briefly, then captures the image.
    /// This is a convenience method that combines `half_press_shutter()` + delay + `capture()`.
    #[async_wrap]
    pub fn focus_and_capture(&self) -> Result<()> {
        self.half_press_shutter()?;
        std::thread::sleep(Duration::from_millis(500));
        self.capture()?;
        self.release_shutter()?;
        Ok(())
    }

    /// Start movie recording
    ///
    /// The camera must be in a mode that supports movie recording (Movie mode).
    /// Call `stop_recording()` to stop.
    #[async_wrap]
    pub fn start_recording(&self) -> Result<()> {
        self.send_command(CommandId::MovieRecord, CommandParam::Down)
    }

    /// Stop movie recording
    #[async_wrap]
    pub fn stop_recording(&self) -> Result<()> {
        self.send_command(CommandId::MovieRecord, CommandParam::Up)
    }

    /// Try to receive an event without blocking
    ///
    /// Returns `None` if no events are currently available.
    /// For async code, use `events()` to get a stream instead.
    pub fn try_recv_event(&mut self) -> Option<CameraEvent> {
        self.event_receiver.try_recv().ok()
    }

    /// Take the event receiver for use with async code
    ///
    /// This consumes the receiver from this device. After calling this,
    /// `try_recv_event()` will always return `None`.
    ///
    /// The returned receiver implements `Stream` and can be used with
    /// `while let Some(event) = receiver.recv().await { ... }`
    pub fn take_event_receiver(&mut self) -> mpsc::UnboundedReceiver<CameraEvent> {
        // Replace with a dummy channel - the sender is never used
        let (_, dummy_receiver) = mpsc::unbounded_channel();
        std::mem::replace(&mut self.event_receiver, dummy_receiver)
    }
}

impl Drop for CameraDevice {
    fn drop(&mut self) {
        // IMPORTANT: Order matters here to avoid use-after-free
        //
        // 1. Disconnect() tells the SDK we're done, and blocks until all
        //    pending callbacks complete (per Sony SDK documentation).
        // 2. ReleaseDevice() releases internal SDK resources.
        // 3. Destroy callback - safe because SDK guarantees no more calls
        //    after Disconnect() returns.
        // 4. Reclaim EventSender - safe because callback is destroyed.
        //
        // This order ensures no callbacks can fire after we free memory.

        if self.handle != 0 {
            // SAFETY: handle is valid if non-zero, obtained from SDK Connect
            unsafe {
                crsdk_sys::SCRSDK::Disconnect(self.handle);
                crsdk_sys::SCRSDK::ReleaseDevice(self.handle);
            }
        }

        if !self.callback_ptr.is_null() {
            // SAFETY: callback_ptr was created by crsdk_create_rust_callback
            // and SDK callbacks are complete after Disconnect()
            unsafe {
                crsdk_sys::crsdk_destroy_rust_callback(self.callback_ptr);
            }
        }

        if !self.event_sender_ptr.is_null() {
            // SAFETY: event_sender_ptr was created by EventSender::into_raw()
            // and callback is destroyed so no more sends possible
            unsafe {
                let _ = EventSender::from_raw(self.event_sender_ptr);
            }
        }
    }
}

/// Builder for configuring and connecting to a camera (blocking API)
#[derive(Default)]
pub struct CameraDeviceBuilder {
    info: ConnectionInfo,
    camera_info_ptr: Option<*mut crsdk_sys::SCRSDK::ICrCameraObjectInfo>,
}

impl CameraDeviceBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the camera's IP address
    pub fn ip_address(mut self, ip: Ipv4Addr) -> Self {
        self.info.ip_address = Some(ip);
        self
    }

    /// Set the camera's MAC address
    pub fn mac_address(mut self, mac: MacAddr) -> Self {
        self.info.mac_address = Some(mac);
        self
    }

    /// Set the camera model
    pub fn model(mut self, model: CameraModel) -> Self {
        self.info.model = Some(model);
        self
    }

    /// Enable or disable SSH tunnel
    pub fn ssh_enabled(mut self, enabled: bool) -> Self {
        self.info.ssh_enabled = enabled;
        self
    }

    /// Set SSH credentials (also enables SSH)
    pub fn ssh_credentials(mut self, user: impl Into<String>, password: impl Into<String>) -> Self {
        self.info.ssh_user = Some(user.into());
        self.info.ssh_password = Some(password.into());
        self.info.ssh_enabled = true;
        self
    }

    /// Set the SSH fingerprint for verification
    pub fn ssh_fingerprint(mut self, fingerprint: impl Into<String>) -> Self {
        self.info.ssh_fingerprint = Some(fingerprint.into());
        self
    }

    /// Fetch SSH fingerprint from camera for user confirmation
    ///
    /// This stores the camera info internally and reuses it for connection.
    pub fn fetch_ssh_fingerprint(&mut self) -> Result<String> {
        let ip = self
            .info
            .ip_address
            .ok_or_else(|| Error::InvalidParameter("IP address is required".to_string()))?;
        let mac = self
            .info
            .mac_address
            .ok_or_else(|| Error::InvalidParameter("MAC address is required".to_string()))?;
        let model = self.info.model.unwrap_or(CameraModel::Fx3);

        ensure_sdk_initialized()?;

        let camera_info_ptr = create_camera_info(ip, mac, model, self.info.ssh_enabled)?;

        let mut fingerprint_buf = vec![0u8; 1024];
        let mut fingerprint_len: u32 = 0;

        let result = unsafe {
            crsdk_sys::SCRSDK::GetFingerprint(
                camera_info_ptr,
                fingerprint_buf.as_mut_ptr() as *mut i8,
                &mut fingerprint_len,
            )
        };

        if result != 0 {
            return Err(Error::from_sdk_error(result as u32));
        }

        self.camera_info_ptr = Some(camera_info_ptr);

        let fingerprint =
            String::from_utf8_lossy(&fingerprint_buf[..fingerprint_len as usize]).into_owned();

        Ok(fingerprint)
    }

    /// Connect to the camera (blocks until connected or error)
    pub fn connect(self) -> Result<CameraDevice> {
        let ip = self
            .info
            .ip_address
            .ok_or_else(|| Error::InvalidParameter("IP address is required".to_string()))?;
        let mac = self
            .info
            .mac_address
            .ok_or_else(|| Error::InvalidParameter("MAC address is required".to_string()))?;
        let model = self.info.model.unwrap_or(CameraModel::Fx3);

        ensure_sdk_initialized()?;

        let camera_info_ptr = match self.camera_info_ptr {
            Some(ptr) => ptr,
            None => create_camera_info(ip, mac, model, self.info.ssh_enabled)?,
        };

        // Create event channel and callback
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        let event_sender = EventSender::new(event_sender);
        let event_sender_ptr = event_sender.into_raw();

        // Create the C++ callback that will forward events to our channel
        // SAFETY: event_sender_ptr is a valid pointer from EventSender::into_raw()
        let callback_ptr = unsafe { crsdk_sys::crsdk_create_rust_callback(event_sender_ptr) };

        let mut device_handle: i64 = 0;

        let user_cstr = self
            .info
            .ssh_user
            .as_ref()
            .map(|s| CString::new(s.as_str()).unwrap());
        let pass_cstr = self
            .info
            .ssh_password
            .as_ref()
            .map(|s| CString::new(s.as_str()).unwrap());
        let fp_cstr = self
            .info
            .ssh_fingerprint
            .as_ref()
            .map(|s| CString::new(s.as_str()).unwrap());

        let user_ptr = user_cstr.as_ref().map_or(ptr::null(), |c| c.as_ptr());
        let pass_ptr = pass_cstr.as_ref().map_or(ptr::null(), |c| c.as_ptr());
        let fp_ptr = fp_cstr.as_ref().map_or(ptr::null(), |c| c.as_ptr());
        let fp_len = self
            .info
            .ssh_fingerprint
            .as_ref()
            .map_or(0, |s| s.len() as u32);

        let result = unsafe {
            crsdk_sys::SCRSDK::Connect(
                camera_info_ptr,
                callback_ptr,
                &mut device_handle,
                crsdk_sys::SCRSDK::CrSdkControlMode_CrSdkControlMode_Remote,
                crsdk_sys::SCRSDK::CrReconnectingSet_CrReconnecting_ON,
                user_ptr,
                pass_ptr,
                fp_ptr,
                fp_len,
            )
        };

        if result != 0 {
            // Clean up callback and event sender on failure
            // SAFETY: callback_ptr was just created above
            unsafe {
                crsdk_sys::crsdk_destroy_rust_callback(callback_ptr);
                let _ = EventSender::from_raw(event_sender_ptr);
            }
            return Err(Error::from_sdk_error(result as u32));
        }

        Ok(CameraDevice {
            handle: device_handle,
            model,
            event_receiver,
            callback_ptr,
            event_sender_ptr,
        })
    }
}

impl Drop for CameraDeviceBuilder {
    fn drop(&mut self) {
        if let Some(ptr) = self.camera_info_ptr.take() {
            // SAFETY: The pointer was created by CreateCameraObjectInfoEthernetConnection
            // and we have exclusive ownership. Release() is the SDK's method to free it.
            unsafe {
                crsdk_sys::crsdk_camera_info_release(ptr);
            }
        }
    }
}
