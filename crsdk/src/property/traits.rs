//! Core trait for property value types.

use crate::types::{FromCrsdk, ToCrsdk};
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
pub trait PropertyValue: ToCrsdk<u64> + FromCrsdk<u64> + Display + Clone {
    /// Create a typed value from a raw SDK value.
    ///
    /// Returns `None` if the raw value is invalid or unrecognized.
    /// This is a convenience wrapper around `from_crsdk` for cases
    /// where unknown values should be handled gracefully.
    fn from_raw(raw: u64) -> Option<Self>
    where
        Self: Sized,
    {
        Self::from_crsdk(raw).ok()
    }

    /// Convert back to a raw SDK value.
    ///
    /// This is the inverse of `from_raw` and can be used when setting
    /// property values on the camera.
    fn to_raw(&self) -> u64 {
        self.to_crsdk()
    }
}
