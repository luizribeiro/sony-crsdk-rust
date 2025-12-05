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
//!         .ip_address("192.168.1.100".parse().unwrap())
//!         .mac_address("10:32:2c:7d:c7:b3".parse().unwrap())
//!         .connect()
//!         .await?;
//!
//!     println!("Connected to {}", camera.model().await);
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
//! ## Implemented Features
//!
//! ✅ SDK initialization and lifecycle
//! ✅ Camera discovery (network and USB enumeration)
//! ✅ Basic connection (IP + MAC + SSH)
//! ✅ Error handling
//! ✅ Property system (ISO, aperture, shutter speed, focus mode, etc.)
//! ✅ Shooting operations (capture, autofocus, movie recording)
//!
//! ## Planned Features
//!
//! - Live view streaming
//! - Event callbacks
//! - Content transfer (download images)
//! - Advanced features (firmware update, settings management)

#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]

pub mod blocking;
mod command;
mod device;
mod error;
mod event;
mod event_sender;
pub mod format;
mod property;
mod property_description;
mod property_display;
mod sdk;
mod types;

// Re-exports for async API (default)
pub use device::{discover_cameras, CameraDevice, CameraDeviceBuilder};
pub use error::{Error, Result};
pub use event::{warning_code_name, warning_param_description, CameraEvent};
pub use property::{
    format_movie_quality, DataType, DeviceProperty, DriveMode, EnableFlag, ExposureProgram,
    FileType, FlashMode, FocusArea, FocusMode, ImageQuality, MeteringMode, MovieFileFormat, OnOff,
    PropertyCode, Switch, WhiteBalance,
};
pub use sdk::Sdk;
pub use types::{CameraModel, ConnectionInfo, ConnectionType, DiscoveredCamera, MacAddr};

// Re-export generated property codes (complete SDK coverage)
pub use crsdk_sys::{DevicePropertyCode, PropertyCategory};
pub use property_description::property_description;
pub use property_display::property_display_name;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_addr_parse() {
        let mac: MacAddr = "10:32:2c:7d:c7:b3".parse().unwrap();
        assert_eq!(mac.to_string(), "10:32:2C:7D:C7:B3");
    }

    #[test]
    fn test_error_from_sdk() {
        let err = Error::from_sdk_error(0x8200);
        assert!(matches!(err, Error::ConnectionFailed(_)));
    }
}
