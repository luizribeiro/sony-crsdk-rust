//! Flash-related property value types.

use std::fmt;

use super::super::traits::PropertyValue;

/// Flash mode settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FlashMode {
    /// Auto flash (fires when needed)
    Auto = 1,
    /// Flash disabled
    Off = 2,
    /// Fill flash (always fires)
    Fill = 3,
    /// External flash sync
    ExternalSync = 4,
    /// Slow sync (ambient light capture)
    SlowSync = 5,
    /// Rear curtain sync
    RearSync = 6,
}

impl PropertyValue for FlashMode {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u16 {
            1 => Self::Auto,
            2 => Self::Off,
            3 => Self::Fill,
            4 => Self::ExternalSync,
            5 => Self::SlowSync,
            6 => Self::RearSync,
            _ => return None,
        })
    }

    fn to_raw(&self) -> u64 {
        *self as u64
    }
}

impl fmt::Display for FlashMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Auto => "Auto",
            Self::Off => "Off",
            Self::Fill => "Fill",
            Self::ExternalSync => "Ext. Sync",
            Self::SlowSync => "Slow Sync",
            Self::RearSync => "Rear Sync",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flash_mode_display() {
        assert_eq!(FlashMode::Auto.to_string(), "Auto");
        assert_eq!(FlashMode::SlowSync.to_string(), "Slow Sync");
        assert_eq!(FlashMode::RearSync.to_string(), "Rear Sync");
    }

    #[test]
    fn test_flash_mode_round_trip() {
        assert_eq!(FlashMode::from_raw(1).unwrap(), FlashMode::Auto);
        assert_eq!(FlashMode::Auto.to_raw(), 1);
    }
}
