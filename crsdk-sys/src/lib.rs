//! Low-level FFI bindings to Sony Camera Remote SDK
//!
//! This crate provides unsafe bindings to the Sony Camera Remote SDK C++ library.
//! For a safe, idiomatic Rust API, use the `crsdk` crate instead.
//!
//! # Safety
//!
//! All functions in this crate are `unsafe` as they directly call into C++ code.
//! Users must ensure proper initialization, memory management, and error handling.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// Include the generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Re-export SCRSDK namespace at crate root for convenience
pub use root::SCRSDK;

// Callback shim from callback_shim.cpp
extern "C" {
    /// Get a pointer to a minimal IDeviceCallback implementation
    pub fn crsdk_get_minimal_callback() -> *mut SCRSDK::IDeviceCallback;
}

// Camera enumeration shims for virtual method access
extern "C" {
    /// Get the count of discovered cameras
    pub fn crsdk_enum_camera_get_count(enum_info: *const SCRSDK::ICrEnumCameraObjectInfo) -> u32;

    /// Get camera info at the specified index
    pub fn crsdk_enum_camera_get_info(
        enum_info: *const SCRSDK::ICrEnumCameraObjectInfo,
        index: u32,
    ) -> *const SCRSDK::ICrCameraObjectInfo;

    /// Release the enumeration object
    pub fn crsdk_enum_camera_release(enum_info: *mut SCRSDK::ICrEnumCameraObjectInfo);

    /// Get the camera model name
    pub fn crsdk_camera_info_get_model(info: *const SCRSDK::ICrCameraObjectInfo) -> *const i8;

    /// Get the camera model name size
    pub fn crsdk_camera_info_get_model_size(info: *const SCRSDK::ICrCameraObjectInfo) -> u32;

    /// Get the camera device name
    pub fn crsdk_camera_info_get_name(info: *const SCRSDK::ICrCameraObjectInfo) -> *const i8;

    /// Get the camera device name size
    pub fn crsdk_camera_info_get_name_size(info: *const SCRSDK::ICrCameraObjectInfo) -> u32;

    /// Get the connection status
    pub fn crsdk_camera_info_get_connection_status(info: *const SCRSDK::ICrCameraObjectInfo)
        -> u32;

    /// Get the connection type name (e.g., "Ethernet", "USB")
    pub fn crsdk_camera_info_get_connection_type(
        info: *const SCRSDK::ICrCameraObjectInfo,
    ) -> *const i8;

    /// Get the IP address as a packed u32
    pub fn crsdk_camera_info_get_ip_address(info: *const SCRSDK::ICrCameraObjectInfo) -> u32;

    /// Get the IP address as a string
    pub fn crsdk_camera_info_get_ip_address_str(
        info: *const SCRSDK::ICrCameraObjectInfo,
    ) -> *const i8;

    /// Get the MAC address bytes
    pub fn crsdk_camera_info_get_mac_address(info: *const SCRSDK::ICrCameraObjectInfo)
        -> *const u8;

    /// Get the MAC address size
    pub fn crsdk_camera_info_get_mac_address_size(info: *const SCRSDK::ICrCameraObjectInfo) -> u32;

    /// Get SSH support flag (1 = supported)
    pub fn crsdk_camera_info_get_ssh_support(info: *const SCRSDK::ICrCameraObjectInfo) -> u32;

    /// Get USB product ID
    pub fn crsdk_camera_info_get_usb_pid(info: *const SCRSDK::ICrCameraObjectInfo) -> i16;

    /// Release a camera info object
    pub fn crsdk_camera_info_release(info: *mut SCRSDK::ICrCameraObjectInfo);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdk_version() {
        // This test just verifies the bindings link correctly
        // We don't call any actual SDK functions as they require initialization
        let version_func_exists = SCRSDK::GetSDKVersion as usize;
        assert_ne!(version_func_exists, 0);
    }
}
