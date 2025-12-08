//! Common property value types used across multiple domains.

use std::fmt;

use super::super::traits::PropertyValue;
use crate::types::ToCrsdk;

/// How to interpret and format a property's raw value.
///
/// This enum defines the semantic type of a property value, allowing type-safe
/// formatting and parsing. Use [`property_value_type`](super::super::property_value_type)
/// to get the type for a property code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropertyValueType {
    // Formatted numeric values (special encoding)
    /// Aperture f-number (raw value / 100, e.g., 280 → f/2.8)
    Aperture,
    /// Shutter speed (upper 16 bits = numerator, lower 16 = denominator)
    ShutterSpeed,
    /// ISO sensitivity (raw value is ISO number)
    Iso,
    /// Exposure compensation (raw value in 1/3 EV steps)
    ExposureCompensation,
    /// Color temperature in Kelvin
    ColorTemperature,
    /// Movie recording quality/bitrate
    MovieQuality,

    // Enum value types
    /// Exposure program mode (P, A, S, M, etc.)
    ExposureProgram,
    /// Metering mode
    MeteringMode,
    /// Focus mode (AF-S, AF-C, MF, etc.)
    FocusMode,
    /// Focus area
    FocusArea,
    /// Subject recognition for AF
    SubjectRecognitionAF,
    /// AF priority setting
    PrioritySetInAF,
    /// Focus tracking status
    FocusTrackingStatus,
    /// White balance preset
    WhiteBalance,
    /// AWB priority setting
    PrioritySetInAWB,
    /// Drive mode (single, continuous, bracket, timer)
    DriveMode,
    /// Interval recording shutter type
    IntervalRecShutterType,
    /// Flash mode
    FlashMode,
    /// File type (slot assignment)
    FileType,
    /// Still image quality (RAW, JPEG, etc.)
    ImageQuality,
    /// Aspect ratio
    AspectRatio,
    /// Image size
    ImageSize,
    /// Movie file format (XAVC, etc.)
    MovieFileFormat,
    /// Shutter mode status
    ShutterModeStatus,
    /// Shutter mode
    ShutterMode,
    /// Exposure control type
    ExposureCtrlType,
    /// Live view display effect
    LiveViewDisplayEffect,
    /// Silent mode aperture drive
    SilentModeApertureDrive,

    // Generic toggle types
    /// On/Off toggle (0=Off, 1=On)
    OnOff,
    /// Switch toggle (1=Off, 2=On)
    Switch,
    /// Auto/Manual toggle
    AutoManual,
    /// Lock indicator (unlocked/locked)
    LockIndicator,

    // Raw value types
    /// Percentage (0-100)
    Percentage,
    /// Plain integer value
    Integer,
    /// Unknown/untyped value (display as raw hex)
    Unknown,
}

/// A generic integer property value.
///
/// Used for properties that represent raw numeric values without special formatting.
/// The value is interpreted as signed to handle properties like focus position offsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Integer(i64);

impl Integer {
    /// Get the integer value.
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl ToCrsdk<u64> for Integer {
    fn to_crsdk(&self) -> u64 {
        self.0 as u64
    }
}

impl PropertyValue for Integer {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(Integer(raw as i64))
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A percentage value (0-100 or similar ranges).
///
/// Used for properties like battery remaining, volume levels, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Percentage(u64);

impl Percentage {
    /// Get the percentage value.
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl ToCrsdk<u64> for Percentage {
    fn to_crsdk(&self) -> u64 {
        self.0
    }
}

impl PropertyValue for Percentage {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(Percentage(raw))
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

/// A generic on/off switch (Off=1, On=2).
///
/// Used for properties like AutoSlowShutter, SilentMode, and NDFilter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Switch {
    /// Switch is off
    Off = 1,
    /// Switch is on
    On = 2,
}

impl ToCrsdk<u64> for Switch {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for Switch {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u8 {
            1 => Self::Off,
            2 => Self::On,
            _ => return None,
        })
    }
}

impl fmt::Display for Switch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::On => write!(f, "On"),
        }
    }
}

/// A generic on/off toggle (Off=0, On=1).
///
/// Used for properties like RedEyeReduction and AudioRecording.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OnOff {
    /// Setting is off
    Off = 0,
    /// Setting is on
    On = 1,
}

impl ToCrsdk<u64> for OnOff {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for OnOff {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u8 {
            0 => Self::Off,
            1 => Self::On,
            _ => return None,
        })
    }
}

impl fmt::Display for OnOff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::On => write!(f, "On"),
        }
    }
}

/// Automatic/Manual mode setting (Automatic=1, Manual=2).
///
/// Used for IrisModeSetting, ShutterModeSetting, GainControlSetting, and NDFilterModeSetting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AutoManual {
    /// Automatic mode
    Automatic = 1,
    /// Manual mode
    Manual = 2,
}

impl ToCrsdk<u64> for AutoManual {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for AutoManual {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u8 {
            1 => Self::Automatic,
            2 => Self::Manual,
            _ => return None,
        })
    }
}

impl fmt::Display for AutoManual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Automatic => write!(f, "Auto"),
            Self::Manual => write!(f, "Manual"),
        }
    }
}

/// Lock indicator (Unknown=0, Unlocked=1, Locked=2).
///
/// Used for white balance lock and similar features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum LockIndicator {
    /// Lock status is unknown
    Unknown = 0,
    /// Setting is unlocked
    Unlocked = 1,
    /// Setting is locked
    Locked = 2,
}

impl ToCrsdk<u64> for LockIndicator {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for LockIndicator {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u16 {
            0 => Self::Unknown,
            1 => Self::Unlocked,
            2 => Self::Locked,
            _ => return None,
        })
    }
}

impl fmt::Display for LockIndicator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Unlocked => write!(f, "Unlocked"),
            Self::Locked => write!(f, "Locked"),
        }
    }
}

/// Live view display effect setting (Unknown=0, On=1, Off=2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LiveViewDisplayEffect {
    /// Status unknown
    Unknown = 0,
    /// Effect is enabled
    On = 1,
    /// Effect is disabled
    Off = 2,
}

impl ToCrsdk<u64> for LiveViewDisplayEffect {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for LiveViewDisplayEffect {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u8 {
            0 => Self::Unknown,
            1 => Self::On,
            2 => Self::Off,
            _ => return None,
        })
    }
}

impl fmt::Display for LiveViewDisplayEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::On => write!(f, "On"),
            Self::Off => write!(f, "Off"),
        }
    }
}

/// Aperture drive behavior in silent mode during AF.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SilentModeApertureDrive {
    /// Aperture drive not affected by silent mode
    NotTarget = 1,
    /// Standard aperture drive behavior
    Standard = 2,
    /// Prioritize silent operation
    SilentPriority = 3,
}

impl ToCrsdk<u64> for SilentModeApertureDrive {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for SilentModeApertureDrive {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u8 {
            1 => Self::NotTarget,
            2 => Self::Standard,
            3 => Self::SilentPriority,
            _ => return None,
        })
    }
}

impl fmt::Display for SilentModeApertureDrive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotTarget => write!(f, "Not Target"),
            Self::Standard => write!(f, "Standard"),
            Self::SilentPriority => write!(f, "Silent Priority"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_display() {
        assert_eq!(Integer(42).to_string(), "42");
        assert_eq!(Integer(-100).to_string(), "-100");
        assert_eq!(Integer(0).to_string(), "0");
    }

    #[test]
    fn test_integer_signed_from_raw() {
        let raw = (-50i64) as u64;
        let int = Integer::from_raw(raw).unwrap();
        assert_eq!(int.value(), -50);
    }

    #[test]
    fn test_percentage_display() {
        assert_eq!(Percentage(100).to_string(), "100%");
        assert_eq!(Percentage(50).to_string(), "50%");
        assert_eq!(Percentage(0).to_string(), "0%");
    }

    #[test]
    fn test_switch_round_trip() {
        assert_eq!(Switch::from_raw(1).unwrap(), Switch::Off);
        assert_eq!(Switch::from_raw(2).unwrap(), Switch::On);
        assert_eq!(Switch::Off.to_raw(), 1);
        assert_eq!(Switch::On.to_raw(), 2);
    }

    #[test]
    fn test_onoff_round_trip() {
        assert_eq!(OnOff::from_raw(0).unwrap(), OnOff::Off);
        assert_eq!(OnOff::from_raw(1).unwrap(), OnOff::On);
        assert_eq!(OnOff::Off.to_raw(), 0);
        assert_eq!(OnOff::On.to_raw(), 1);
    }

    #[test]
    fn test_auto_manual_display() {
        assert_eq!(AutoManual::Automatic.to_string(), "Auto");
        assert_eq!(AutoManual::Manual.to_string(), "Manual");
    }

    #[test]
    fn test_lock_indicator_display() {
        assert_eq!(LockIndicator::Locked.to_string(), "Locked");
        assert_eq!(LockIndicator::Unlocked.to_string(), "Unlocked");
        assert_eq!(LockIndicator::Unknown.to_string(), "Unknown");
    }
}
