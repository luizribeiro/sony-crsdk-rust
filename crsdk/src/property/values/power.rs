//! Power and battery property value types.

use std::fmt;

use super::super::traits::PropertyValue;
use crate::error::{Error, Result};
use crate::types::{FromCrsdk, ToCrsdk};

/// Battery level indicator.
///
/// The SDK encodes battery level as a packed value where:
/// - High word (bits 16+): Battery status/grade indicator
/// - Low word (bits 0-15): Battery level value
///
/// The exact encoding varies by camera, but common patterns include:
/// - Level 1-5 representing empty to full
/// - Percentage values in some cameras
///
/// This type decodes the packed format and displays a meaningful value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BatteryLevel {
    /// The battery level/grade (typically 1-5 or a percentage)
    level: u16,
    /// Status or type indicator from the high word
    status: u16,
}

impl BatteryLevel {
    /// Get the battery level value.
    pub fn level(&self) -> u16 {
        self.level
    }

    /// Get the status indicator from the high word.
    pub fn status(&self) -> u16 {
        self.status
    }

    /// Get the raw packed value.
    pub fn raw(&self) -> u64 {
        ((self.status as u64) << 16) | (self.level as u64)
    }
}

impl ToCrsdk<u64> for BatteryLevel {
    fn to_crsdk(&self) -> u64 {
        self.raw()
    }
}

impl FromCrsdk<u64> for BatteryLevel {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(BatteryLevel {
            status: ((raw >> 16) & 0xFFFF) as u16,
            level: (raw & 0xFFFF) as u16,
        })
    }
}

impl PropertyValue for BatteryLevel {}

impl fmt::Display for BatteryLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display just the level - the status is metadata
        // For value 65541 (0x10005), this shows "5" instead of "65541%"
        write!(f, "{}", self.level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_level_decode() {
        // Test the bug case: raw value 65541 = 0x10005
        // High word: 0x0001 = 1 (status)
        // Low word: 0x0005 = 5 (level)
        let bl = BatteryLevel::from_raw(65541).unwrap();
        assert_eq!(bl.level(), 5);
        assert_eq!(bl.status(), 1);
        assert_eq!(bl.to_string(), "5");
    }

    #[test]
    fn test_battery_level_simple() {
        // Simple case: just a level with no status
        let bl = BatteryLevel::from_raw(100).unwrap();
        assert_eq!(bl.level(), 100);
        assert_eq!(bl.status(), 0);
        assert_eq!(bl.to_string(), "100");
    }

    #[test]
    fn test_battery_level_round_trip() {
        let bl = BatteryLevel::from_raw(65541).unwrap();
        assert_eq!(bl.to_raw(), 65541);
    }
}

/// Camera power status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CameraPowerStatus {
    /// Camera is off
    Off = 0x01,
    /// Camera is in standby mode
    Standby = 0x02,
    /// Camera is powered on
    PowerOn = 0x03,
    /// Camera is transitioning from power on to standby
    TransitioningFromPowerOnToStandby = 0x04,
    /// Camera is transitioning from standby to power on
    TransitioningFromStandbyToPowerOn = 0x05,
}

impl ToCrsdk<u64> for CameraPowerStatus {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for CameraPowerStatus {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Off,
            0x02 => Self::Standby,
            0x03 => Self::PowerOn,
            0x04 => Self::TransitioningFromPowerOnToStandby,
            0x05 => Self::TransitioningFromStandbyToPowerOn,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for CameraPowerStatus {}

impl fmt::Display for CameraPowerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::Standby => write!(f, "Standby"),
            Self::PowerOn => write!(f, "Power On"),
            Self::TransitioningFromPowerOnToStandby => write!(f, "Powering Down"),
            Self::TransitioningFromStandbyToPowerOn => write!(f, "Powering Up"),
        }
    }
}
