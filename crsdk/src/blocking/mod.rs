//! Blocking (synchronous) API for Sony Camera Remote SDK
//!
//! Use this module when you don't need async/await or want simpler code.
//! For async usage, use the root `crsdk` types instead.
//!
//! # Example
//!
//! ```no_run
//! use crsdk::blocking::CameraDevice;
//! use crsdk::{CameraModel, Result};
//!
//! fn main() -> Result<()> {
//!     let mut builder = CameraDevice::builder()
//!         .ip_address("192.168.1.100".parse()?)
//!         .mac_address("AA:BB:CC:DD:EE:FF".parse()?)
//!         .model(CameraModel::FX3);
//!
//!     // For SSH connections
//!     builder = builder.ssh_enabled(true);
//!     let fingerprint = builder.fetch_ssh_fingerprint()?;
//!     println!("Fingerprint: {}", fingerprint);
//!     // ... confirm with user ...
//!     builder = builder
//!         .ssh_credentials("user", "password")
//!         .ssh_fingerprint(fingerprint);
//!
//!     let camera = builder.connect()?;
//!     println!("Connected to {}", camera.model());
//!
//!     Ok(())
//! }
//! ```

mod device;

pub use device::{CameraDevice, CameraDeviceBuilder};
