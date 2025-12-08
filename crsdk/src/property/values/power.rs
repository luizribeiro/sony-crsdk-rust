//! Power and battery property value types.

use std::fmt;

use super::super::traits::PropertyValue;

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

impl PropertyValue for BatteryLevel {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(BatteryLevel {
            status: ((raw >> 16) & 0xFFFF) as u16,
            level: (raw & 0xFFFF) as u16,
        })
    }

    fn to_raw(&self) -> u64 {
        self.raw()
    }
}

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
