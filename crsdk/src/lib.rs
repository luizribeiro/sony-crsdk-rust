//! # Sony Camera Remote SDK - Rust Wrapper
//!
//! Safe, idiomatic Rust bindings for the Sony Camera Remote SDK.
//!
//! ## Features
//!
//! - **Type-safe API** - Rust types for camera properties and operations
//! - **Async/await** - Non-blocking operations using Tokio
//! - **Builder pattern** - Ergonomic camera connection setup
//! - **Network & USB** - Connect via Ethernet/WiFi or USB
//! - **SSH support** - Secure connections with authentication
//!
//! ## Quick Start
//!
//! ```no_run
//! use crsdk::{CameraDevice, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let camera = CameraDevice::builder()
//!         .ip_address("192.168.1.100".parse()?)
//!         .mac_address("00:00:00:00:00:00".parse()?)
//!         .connect()
//!         .await?;
//!
//!     println!("Connected to {}", camera.model());
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! This crate provides a safe wrapper around the low-level FFI bindings in `crsdk-sys`.
//! All unsafe operations are encapsulated and made safe through:
//!
//! - RAII resource management
//! - Type-safe enums and structs
//! - Result-based error handling
//! - Async operations via Tokio
//!
//! ## Phase 1: Foundation (Current)
//!
//! ✅ SDK initialization and lifecycle
//! ✅ Camera discovery (coming soon)
//! ✅ Basic connection (IP + MAC)
//! ✅ Error handling
//!
//! Future phases will add:
//! - Property system (ISO, aperture, shutter speed, etc.)
//! - Shooting operations (capture, movie recording)
//! - Live view streaming
//! - Event callbacks
//! - Content transfer (download images)
//! - Advanced features (firmware update, settings management)

#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]

pub mod blocking;
mod device;
mod error;
mod property;
mod sdk;
mod types;

// Re-exports for async API (default)
pub use device::{CameraDevice, CameraDeviceBuilder};
pub use error::{Error, Result};
pub use property::{DataType, DeviceProperty, EnableFlag, FocusMode, PropertyCode, WhiteBalance};
pub use sdk::Sdk;
pub use types::{CameraModel, ConnectionInfo, MacAddr};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_addr_parse() {
        let mac: MacAddr = "00:00:00:00:00:00".parse().unwrap();
        assert_eq!(mac.to_string(), "10:32:2C:7D:C7:B3");
    }

    #[test]
    fn test_error_from_sdk() {
        let err = Error::from_sdk_error(0x8200);
        assert!(matches!(err, Error::ConnectionFailed(_)));
    }
}
