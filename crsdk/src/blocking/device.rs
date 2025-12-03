//! Blocking camera device connection and control

use crate::error::{Error, Result};
use crate::types::{ip_to_sdk_format, CameraModel, ConnectionInfo, MacAddr};
use crate::Sdk;
use std::ffi::CString;
use std::net::Ipv4Addr;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};

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

impl CameraDevice {
    /// Create a new builder for configuring camera connection
    pub fn builder() -> CameraDeviceBuilder {
        CameraDeviceBuilder::new()
    }

    /// Get the camera model
    pub fn model(&self) -> CameraModel {
        self.model
    }

    /// Get the device handle (for internal use)
    pub(crate) fn handle(&self) -> i64 {
        self.handle
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ip_address(mut self, ip: Ipv4Addr) -> Self {
        self.info.ip_address = Some(ip);
        self
    }

    pub fn mac_address(mut self, mac: MacAddr) -> Self {
        self.info.mac_address = Some(mac);
        self
    }

    pub fn model(mut self, model: CameraModel) -> Self {
        self.info.model = Some(model);
        self
    }

    pub fn ssh_enabled(mut self, enabled: bool) -> Self {
        self.info.ssh_enabled = enabled;
        self
    }

    pub fn ssh_credentials(mut self, user: impl Into<String>, password: impl Into<String>) -> Self {
        self.info.ssh_user = Some(user.into());
        self.info.ssh_password = Some(password.into());
        self.info.ssh_enabled = true;
        self
    }

    pub fn ssh_fingerprint(mut self, fingerprint: impl Into<String>) -> Self {
        self.info.ssh_fingerprint = Some(fingerprint.into());
        self
    }

    /// Fetch SSH fingerprint from camera for user confirmation
    ///
    /// This stores the camera info internally and reuses it for connection.
    pub fn fetch_ssh_fingerprint(&mut self) -> Result<String> {
        let ip = self.info.ip_address.ok_or_else(|| {
            Error::InvalidParameter("IP address is required".to_string())
        })?;
        let mac = self.info.mac_address.ok_or_else(|| {
            Error::InvalidParameter("MAC address is required".to_string())
        })?;
        let model = self.info.model.unwrap_or(CameraModel::FX3);

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

        let fingerprint = String::from_utf8_lossy(&fingerprint_buf[..fingerprint_len as usize])
            .into_owned();

        Ok(fingerprint)
    }

    /// Connect to the camera (blocks until connected or error)
    pub fn connect(self) -> Result<CameraDevice> {
        let ip = self.info.ip_address.ok_or_else(|| {
            Error::InvalidParameter("IP address is required".to_string())
        })?;
        let mac = self.info.mac_address.ok_or_else(|| {
            Error::InvalidParameter("MAC address is required".to_string())
        })?;
        let model = self.info.model.unwrap_or(CameraModel::FX3);

        ensure_sdk_initialized()?;

        let camera_info_ptr = match self.camera_info_ptr {
            Some(ptr) => ptr,
            None => create_camera_info(ip, mac, model, self.info.ssh_enabled)?,
        };

        let mut device_handle: i64 = 0;

        let user_cstr = self.info.ssh_user.as_ref().map(|s| CString::new(s.as_str()).unwrap());
        let pass_cstr = self.info.ssh_password.as_ref().map(|s| CString::new(s.as_str()).unwrap());
        let fp_cstr = self.info.ssh_fingerprint.as_ref().map(|s| CString::new(s.as_str()).unwrap());

        let user_ptr = user_cstr.as_ref().map_or(ptr::null(), |c| c.as_ptr());
        let pass_ptr = pass_cstr.as_ref().map_or(ptr::null(), |c| c.as_ptr());
        let fp_ptr = fp_cstr.as_ref().map_or(ptr::null(), |c| c.as_ptr());
        let fp_len = self.info.ssh_fingerprint.as_ref().map_or(0, |s| s.len() as u32);

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
