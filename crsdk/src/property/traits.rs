//! Core trait for property value types.

use std::fmt::Display;

/// Trait for all property value types.
///
/// This trait ensures consistent conversion between raw SDK values and
/// typed Rust representations. All property value types must implement
/// both this trait and `Display` for formatting.
///
/// # Example
///
/// ```ignore
/// use crsdk::property::{PropertyValue, Aperture};
///
/// let aperture = Aperture::from_raw(280).unwrap();
/// assert_eq!(aperture.to_string(), "f/2.8");
/// assert_eq!(aperture.to_raw(), 280);
/// ```
pub trait PropertyValue: Display + Clone {
    /// Create a typed value from a raw SDK value.
    ///
    /// Returns `None` if the raw value is invalid or unrecognized.
    /// The SDK always provides values as `u64`, even for signed types;
    /// implementations handle the reinterpretation internally.
    fn from_raw(raw: u64) -> Option<Self>
    where
        Self: Sized;

    /// Convert back to a raw SDK value.
    ///
    /// This is the inverse of `from_raw` and can be used when setting
    /// property values on the camera.
    fn to_raw(&self) -> u64;
}
