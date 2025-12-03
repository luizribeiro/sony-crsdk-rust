//! Blocking camera device connection and control

use asyncwrap::{async_wrap, blocking_impl};

use crate::command::{CommandId, CommandParam, LockIndicator};
use crate::error::{Error, Result};
use crate::property::{
    device_property_from_sdk, DeviceProperty, DriveMode, ExposureProgram, FlashMode, FocusArea,
    FocusMode, MeteringMode, PropertyCode, WhiteBalance,
};
use crate::types::{ip_to_sdk_format, CameraModel, ConnectionInfo, MacAddr};
use crate::Sdk;
use std::ffi::CString;
use std::net::Ipv4Addr;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

static SDK_INITIALIZED: AtomicBool = AtomicBool::new(false);

fn ensure_sdk_initialized() -> Result<()> {
    if !SDK_INITIALIZED.load(Ordering::Acquire) {
        let sdk = Sdk::init()?;
        std::mem::forget(sdk); // Keep SDK alive for program lifetime
        SDK_INITIALIZED.store(true, Ordering::Release);
    }
    Ok(())
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
}

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
    pub fn get_property(&self, code: PropertyCode) -> Result<DeviceProperty> {
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

    /// Set a property on the camera
    ///
    /// The value should be a raw u64 value. Use the enum's `as_raw()` method
    /// for enumerated properties like FocusMode or WhiteBalance.
    #[async_wrap]
    pub fn set_property(&self, code: PropertyCode, value: u64) -> Result<()> {
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
        let prop = self.get_property(PropertyCode::FocusMode)?;
        FocusMode::from_raw(prop.current_value).ok_or(Error::InvalidPropertyValue)
    }

    /// Set the focus mode
    #[async_wrap]
    pub fn set_focus_mode(&self, mode: FocusMode) -> Result<()> {
        self.set_property(PropertyCode::FocusMode, mode.as_raw())
    }

    /// Get the current white balance setting
    #[async_wrap]
    pub fn white_balance(&self) -> Result<WhiteBalance> {
        let prop = self.get_property(PropertyCode::WhiteBalance)?;
        WhiteBalance::from_raw(prop.current_value).ok_or(Error::InvalidPropertyValue)
    }

    /// Set the white balance
    #[async_wrap]
    pub fn set_white_balance(&self, wb: WhiteBalance) -> Result<()> {
        self.set_property(PropertyCode::WhiteBalance, wb.as_raw())
    }

    /// Get the current exposure program mode
    #[async_wrap]
    pub fn exposure_program(&self) -> Result<ExposureProgram> {
        let prop = self.get_property(PropertyCode::ExposureProgram)?;
        ExposureProgram::from_raw(prop.current_value).ok_or(Error::InvalidPropertyValue)
    }

    /// Set the exposure program mode
    #[async_wrap]
    pub fn set_exposure_program(&self, program: ExposureProgram) -> Result<()> {
        self.set_property(PropertyCode::ExposureProgram, program.as_raw())
    }

    /// Get the current drive mode
    #[async_wrap]
    pub fn drive_mode(&self) -> Result<DriveMode> {
        let prop = self.get_property(PropertyCode::DriveMode)?;
        DriveMode::from_raw(prop.current_value).ok_or(Error::InvalidPropertyValue)
    }

    /// Set the drive mode
    #[async_wrap]
    pub fn set_drive_mode(&self, mode: DriveMode) -> Result<()> {
        self.set_property(PropertyCode::DriveMode, mode.as_raw())
    }

    /// Get the current metering mode
    #[async_wrap]
    pub fn metering_mode(&self) -> Result<MeteringMode> {
        let prop = self.get_property(PropertyCode::MeteringMode)?;
        MeteringMode::from_raw(prop.current_value).ok_or(Error::InvalidPropertyValue)
    }

    /// Set the metering mode
    #[async_wrap]
    pub fn set_metering_mode(&self, mode: MeteringMode) -> Result<()> {
        self.set_property(PropertyCode::MeteringMode, mode.as_raw())
    }

    /// Get the current flash mode
    #[async_wrap]
    pub fn flash_mode(&self) -> Result<FlashMode> {
        let prop = self.get_property(PropertyCode::FlashMode)?;
        FlashMode::from_raw(prop.current_value).ok_or(Error::InvalidPropertyValue)
    }

    /// Set the flash mode
    #[async_wrap]
    pub fn set_flash_mode(&self, mode: FlashMode) -> Result<()> {
        self.set_property(PropertyCode::FlashMode, mode.as_raw())
    }

    /// Get the current focus area
    #[async_wrap]
    pub fn focus_area(&self) -> Result<FocusArea> {
        let prop = self.get_property(PropertyCode::FocusArea)?;
        FocusArea::from_raw(prop.current_value).ok_or(Error::InvalidPropertyValue)
    }

    /// Set the focus area
    #[async_wrap]
    pub fn set_focus_area(&self, area: FocusArea) -> Result<()> {
        self.set_property(PropertyCode::FocusArea, area.as_raw())
    }

    /// Get the current ISO sensitivity (raw value)
    #[async_wrap]
    pub fn iso(&self) -> Result<u64> {
        let prop = self.get_property(PropertyCode::IsoSensitivity)?;
        Ok(prop.current_value)
    }

    /// Set the ISO sensitivity
    #[async_wrap]
    pub fn set_iso(&self, value: u64) -> Result<()> {
        self.set_property(PropertyCode::IsoSensitivity, value)
    }

    /// Get the current aperture/f-number (raw SDK value)
    #[async_wrap]
    pub fn aperture(&self) -> Result<u64> {
        let prop = self.get_property(PropertyCode::FNumber)?;
        Ok(prop.current_value)
    }

    /// Set the aperture/f-number
    #[async_wrap]
    pub fn set_aperture(&self, value: u64) -> Result<()> {
        self.set_property(PropertyCode::FNumber, value)
    }

    /// Get the current shutter speed (raw SDK value)
    #[async_wrap]
    pub fn shutter_speed(&self) -> Result<u64> {
        let prop = self.get_property(PropertyCode::ShutterSpeed)?;
        Ok(prop.current_value)
    }

    /// Set the shutter speed
    #[async_wrap]
    pub fn set_shutter_speed(&self, value: u64) -> Result<()> {
        self.set_property(PropertyCode::ShutterSpeed, value)
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
}

impl Drop for CameraDevice {
    fn drop(&mut self) {
        if self.handle != 0 {
            unsafe {
                crsdk_sys::SCRSDK::Disconnect(self.handle);
                crsdk_sys::SCRSDK::ReleaseDevice(self.handle);
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

        let callback = unsafe { crsdk_sys::crsdk_get_minimal_callback() };

        let result = unsafe {
            crsdk_sys::SCRSDK::Connect(
                camera_info_ptr,
                callback,
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
            return Err(Error::from_sdk_error(result as u32));
        }

        Ok(CameraDevice {
            handle: device_handle,
            model,
        })
    }
}
