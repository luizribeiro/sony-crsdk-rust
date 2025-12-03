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
