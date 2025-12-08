//! Core property infrastructure types.
//!
//! This module contains the fundamental types for working with camera properties:
//! - [`DataType`] - SDK data type classification
//! - [`EnableFlag`] - Property enable/writable status
//! - [`ValueConstraint`] - Constraint on property values (discrete or range)
//! - [`DeviceProperty`] - A camera property with its current value and metadata

mod constraint;
mod data_type;
mod device_property;
mod enable_flag;

pub use constraint::ValueConstraint;
pub use data_type::DataType;
pub use device_property::DeviceProperty;
pub use enable_flag::EnableFlag;

pub(crate) use device_property::{device_property_from_sdk, device_property_from_sdk_debug};
