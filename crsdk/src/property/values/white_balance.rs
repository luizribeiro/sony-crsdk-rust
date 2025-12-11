//! White balance property value types.

use std::fmt;

use super::super::traits::PropertyValue;
use crate::error::{Error, Result};
use crate::types::{FromCrsdk, ToCrsdk};

/// Color temperature value in Kelvin.
///
/// Used when white balance is set to manual color temperature mode.
/// Typical range is 2500K (warm/tungsten) to 9900K (cool/shade).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ColorTemperature(u64);

impl ColorTemperature {
    /// Get the color temperature in Kelvin.
    pub fn kelvin(&self) -> u64 {
        self.0
    }
}

impl ToCrsdk<u64> for ColorTemperature {
    fn to_crsdk(&self) -> u64 {
        self.0
    }
}

impl FromCrsdk<u64> for ColorTemperature {
    fn from_crsdk(raw: u64) -> Result<Self> {
        if raw == 0 {
            Err(Error::InvalidPropertyValue)
        } else {
            Ok(ColorTemperature(raw))
        }
    }
}

impl PropertyValue for ColorTemperature {}

impl fmt::Display for ColorTemperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}K", self.0)
    }
}

/// White balance settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
#[non_exhaustive]
pub enum WhiteBalance {
    /// Auto white balance (AWB)
    Auto = 0,
    /// Daylight preset (~5500K)
    Daylight = 17,
    /// Shade preset (~7000K)
    Shade = 18,
    /// Cloudy preset (~6000K)
    Cloudy = 19,
    /// Tungsten/incandescent (~3200K)
    Tungsten = 20,
    /// Fluorescent generic
    Fluorescent = 1,
    /// Warm white fluorescent
    FluorescentWarmWhite = 2,
    /// Cool white fluorescent
    FluorescentCoolWhite = 3,
    /// Day white fluorescent
    FluorescentDayWhite = 4,
    /// Daylight fluorescent
    FluorescentDaylight = 5,
    /// Flash white balance
    Flash = 33,
    /// Underwater auto WB
    UnderwaterAuto = 64,
    /// Manual color temperature (K)
    ColorTemp = 256,
    /// Custom preset 1
    Custom1 = 257,
    /// Custom preset 2
    Custom2 = 258,
    /// Custom preset 3
    Custom3 = 259,
    /// Custom from captured reference
    Custom = 272,
}

impl ToCrsdk<u64> for WhiteBalance {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for WhiteBalance {
    fn from_crsdk(raw: u64) -> Result<Self> {
        let value = raw as u16;
        Ok(match value {
            0 => Self::Auto,
            17 => Self::Daylight,
            18 => Self::Shade,
            19 => Self::Cloudy,
            20 => Self::Tungsten,
            1 => Self::Fluorescent,
            2 => Self::FluorescentWarmWhite,
            3 => Self::FluorescentCoolWhite,
            4 => Self::FluorescentDayWhite,
            5 => Self::FluorescentDaylight,
            33 => Self::Flash,
            64 => Self::UnderwaterAuto,
            256 => Self::ColorTemp,
            257 => Self::Custom1,
            258 => Self::Custom2,
            259 => Self::Custom3,
            272 => Self::Custom,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for WhiteBalance {}

impl fmt::Display for WhiteBalance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Auto => "AWB",
            Self::Daylight => "Daylight",
            Self::Shade => "Shade",
            Self::Cloudy => "Cloudy",
            Self::Tungsten => "Tungsten",
            Self::Fluorescent => "Fluorescent",
            Self::FluorescentWarmWhite => "Fluor. Warm White",
            Self::FluorescentCoolWhite => "Fluor. Cool White",
            Self::FluorescentDayWhite => "Fluor. Day White",
            Self::FluorescentDaylight => "Fluor. Daylight",
            Self::Flash => "Flash",
            Self::UnderwaterAuto => "Underwater Auto",
            Self::ColorTemp => "Color Temp",
            Self::Custom1 => "Custom 1",
            Self::Custom2 => "Custom 2",
            Self::Custom3 => "Custom 3",
            Self::Custom => "Custom",
        };
        write!(f, "{}", s)
    }
}

/// Priority setting for auto white balance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PrioritySetInAWB {
    /// Neutral white balance
    Standard = 1,
    /// Preserve ambient light warmth
    Ambience = 2,
    /// Prioritize neutral whites
    White = 3,
}

impl ToCrsdk<u64> for PrioritySetInAWB {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for PrioritySetInAWB {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            1 => Self::Standard,
            2 => Self::Ambience,
            3 => Self::White,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for PrioritySetInAWB {}

impl fmt::Display for PrioritySetInAWB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "Standard"),
            Self::Ambience => write!(f, "Ambience"),
            Self::White => write!(f, "White"),
        }
    }
}

/// White balance switch between preset and memory modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WhiteBalanceSwitch {
    /// Preset white balance mode
    Preset = 1,
    /// Memory A white balance
    MemoryA = 2,
    /// Memory B white balance
    MemoryB = 3,
}

impl ToCrsdk<u64> for WhiteBalanceSwitch {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for WhiteBalanceSwitch {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            1 => Self::Preset,
            2 => Self::MemoryA,
            3 => Self::MemoryB,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for WhiteBalanceSwitch {}

impl fmt::Display for WhiteBalanceSwitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Preset => write!(f, "Preset"),
            Self::MemoryA => write!(f, "Memory A"),
            Self::MemoryB => write!(f, "Memory B"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_temperature_display() {
        assert_eq!(ColorTemperature(5500).to_string(), "5500K");
        assert_eq!(ColorTemperature(3200).to_string(), "3200K");
        assert_eq!(ColorTemperature(9900).to_string(), "9900K");
    }

    #[test]
    fn test_color_temperature_from_raw() {
        assert!(ColorTemperature::from_raw(0).is_none());
        assert_eq!(ColorTemperature::from_raw(5600).unwrap().kelvin(), 5600);
    }

    #[test]
    fn test_white_balance_display() {
        assert_eq!(WhiteBalance::Auto.to_string(), "AWB");
        assert_eq!(WhiteBalance::Daylight.to_string(), "Daylight");
        assert_eq!(WhiteBalance::ColorTemp.to_string(), "Color Temp");
    }

    #[test]
    fn test_white_balance_round_trip() {
        assert_eq!(WhiteBalance::from_raw(0).unwrap(), WhiteBalance::Auto);
        assert_eq!(
            WhiteBalance::from_raw(256).unwrap(),
            WhiteBalance::ColorTemp
        );
        assert_eq!(WhiteBalance::Auto.to_raw(), 0);
    }

    #[test]
    fn test_priority_set_in_awb_display() {
        assert_eq!(PrioritySetInAWB::Standard.to_string(), "Standard");
        assert_eq!(PrioritySetInAWB::Ambience.to_string(), "Ambience");
        assert_eq!(PrioritySetInAWB::White.to_string(), "White");
    }
}
