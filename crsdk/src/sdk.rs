//! SDK lifecycle management (initialization and cleanup)

use crate::error::{Error, Result};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

/// Global SDK initialization state
static SDK_INITIALIZED: AtomicBool = AtomicBool::new(false);
static SDK_LOCK: Mutex<()> = Mutex::new(());

/// Sony Camera Remote SDK instance
///
/// Manages SDK lifecycle. Must be created before using any camera operations.
/// The SDK is automatically released when this is dropped.
///
/// Note: This type is intentionally !Send and !Sync because the underlying
/// C++ SDK is not thread-safe and must be used from a single thread.
pub(crate) struct Sdk {
    // PhantomData<*const ()> makes this type !Send and !Sync
    _marker: PhantomData<*const ()>,
}

impl Sdk {
    /// Initialize the Sony Camera Remote SDK
    ///
    /// This must be called once before any camera operations.
    /// Multiple calls will return an error.
    ///
    pub fn init() -> Result<Self> {
        let _guard = SDK_LOCK.lock().unwrap();

        if SDK_INITIALIZED.load(Ordering::Acquire) {
            return Err(Error::Other("SDK already initialized".to_string()));
        }

        // Safety: We're calling the C++ SDK init function
        // The SDK must be initialized before any other operations
        let result = unsafe { crsdk_sys::SCRSDK::Init(0) };

        if !result {
            return Err(Error::InitFailed);
        }

        SDK_INITIALIZED.store(true, Ordering::Release);

        Ok(Sdk {
            _marker: PhantomData,
        })
    }

    /// Get SDK version
    ///
    /// Returns the SDK version as a 32-bit integer where:
    /// - Bits 24-31: Major version
    /// - Bits 16-23: Minor version
    /// - Bits 8-15: Patch version
    pub fn version(&self) -> u32 {
        unsafe { crsdk_sys::SCRSDK::GetSDKVersion() }
    }

    /// Get SDK version as a formatted string (e.g., "2.0.00").
    pub fn version_string(&self) -> String {
        let version = self.version();
        let major = (version >> 24) & 0xFF;
        let minor = (version >> 16) & 0xFF;
        let patch = (version >> 8) & 0xFF;
        format!("{}.{}.{:02}", major, minor, patch)
    }

    /// Get SDK serial number
    pub fn serial(&self) -> u32 {
        unsafe { crsdk_sys::SCRSDK::GetSDKSerial() }
    }
}

impl Drop for Sdk {
    fn drop(&mut self) {
        let _guard = SDK_LOCK.lock().unwrap();

        if SDK_INITIALIZED.load(Ordering::Acquire) {
            // Safety: Release the SDK resources
            unsafe {
                crsdk_sys::SCRSDK::Release();
            }
            SDK_INITIALIZED.store(false, Ordering::Release);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdk_version_parsing() {
        // Mock version: 2.0.00
        let version = 0x02000000u32;
        let major = (version >> 24) & 0xFF;
        let minor = (version >> 16) & 0xFF;
        let patch = (version >> 8) & 0xFF;

        assert_eq!(major, 2);
        assert_eq!(minor, 0);
        assert_eq!(patch, 0);
    }
}
