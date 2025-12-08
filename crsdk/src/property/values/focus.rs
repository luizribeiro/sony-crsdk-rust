//! Focus-related property value types.

use std::fmt;

use super::super::traits::PropertyValue;

/// Focus mode settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FocusMode {
    /// Manual focus (MF)
    Manual = 1,
    /// Single AF (AF-S)
    AfSingle = 2,
    /// Continuous AF (AF-C)
    AfContinuous = 6,
    /// Automatic AF mode selection (AF-A)
    AfAutomatic = 3,
    /// Deep learning AF (AF-D)
    AfDeepLearning = 4,
    /// Direct manual focus (DMF)
    DirectManual = 5,
    /// Preset focus position (PF)
    PresetFocus = 7,
}

impl PropertyValue for FocusMode {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u16 {
            1 => Self::Manual,
            2 => Self::AfSingle,
            6 => Self::AfContinuous,
            3 => Self::AfAutomatic,
            4 => Self::AfDeepLearning,
            5 => Self::DirectManual,
            7 => Self::PresetFocus,
            _ => return None,
        })
    }

    fn to_raw(&self) -> u64 {
        *self as u64
    }
}

impl fmt::Display for FocusMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Manual => "MF",
            Self::AfSingle => "AF-S",
            Self::AfContinuous => "AF-C",
            Self::AfAutomatic => "AF-A",
            Self::AfDeepLearning => "AF-D",
            Self::DirectManual => "DMF",
            Self::PresetFocus => "PF",
        };
        write!(f, "{}", s)
    }
}

/// Focus area settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
#[non_exhaustive]
pub enum FocusArea {
    /// Unknown area
    Unknown = 0,
    /// Wide area (full frame)
    Wide = 1,
    /// Zone area
    Zone = 2,
    /// Center area
    Center = 3,
    /// Flexible spot small
    FlexibleSpotS = 4,
    /// Flexible spot medium
    FlexibleSpotM = 5,
    /// Flexible spot large
    FlexibleSpotL = 6,
    /// Expand flexible spot
    ExpandFlexibleSpot = 7,
    /// Flexible spot
    FlexibleSpot = 8,
    /// Tracking wide
    TrackingWide = 257,
    /// Tracking zone
    TrackingZone = 258,
    /// Tracking center
    TrackingCenter = 259,
    /// Tracking spot small
    TrackingFlexibleSpotS = 260,
    /// Tracking spot medium
    TrackingFlexibleSpotM = 261,
    /// Tracking spot large
    TrackingFlexibleSpotL = 262,
    /// Tracking expand spot
    TrackingExpandFlexibleSpot = 263,
    /// Tracking spot
    TrackingFlexibleSpot = 264,
    /// Flexible spot extra small
    FlexibleSpotXS = 265,
    /// Flexible spot extra large
    FlexibleSpotXL = 266,
    /// Flexible spot free size 1
    FlexibleSpotFreeSize1 = 267,
    /// Flexible spot free size 2
    FlexibleSpotFreeSize2 = 268,
    /// Flexible spot free size 3
    FlexibleSpotFreeSize3 = 269,
    /// Tracking spot extra small
    TrackingFlexibleSpotXS = 270,
    /// Tracking spot extra large
    TrackingFlexibleSpotXL = 271,
    /// Tracking free size 1
    TrackingFlexibleSpotFreeSize1 = 272,
    /// Tracking free size 2
    TrackingFlexibleSpotFreeSize2 = 273,
    /// Tracking free size 3
    TrackingFlexibleSpotFreeSize3 = 274,
}

impl PropertyValue for FocusArea {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u16 {
            0 => Self::Unknown,
            1 => Self::Wide,
            2 => Self::Zone,
            3 => Self::Center,
            4 => Self::FlexibleSpotS,
            5 => Self::FlexibleSpotM,
            6 => Self::FlexibleSpotL,
            7 => Self::ExpandFlexibleSpot,
            8 => Self::FlexibleSpot,
            257 => Self::TrackingWide,
            258 => Self::TrackingZone,
            259 => Self::TrackingCenter,
            260 => Self::TrackingFlexibleSpotS,
            261 => Self::TrackingFlexibleSpotM,
            262 => Self::TrackingFlexibleSpotL,
            263 => Self::TrackingExpandFlexibleSpot,
            264 => Self::TrackingFlexibleSpot,
            265 => Self::FlexibleSpotXS,
            266 => Self::FlexibleSpotXL,
            267 => Self::FlexibleSpotFreeSize1,
            268 => Self::FlexibleSpotFreeSize2,
            269 => Self::FlexibleSpotFreeSize3,
            270 => Self::TrackingFlexibleSpotXS,
            271 => Self::TrackingFlexibleSpotXL,
            272 => Self::TrackingFlexibleSpotFreeSize1,
            273 => Self::TrackingFlexibleSpotFreeSize2,
            274 => Self::TrackingFlexibleSpotFreeSize3,
            _ => return None,
        })
    }

    fn to_raw(&self) -> u64 {
        *self as u64
    }
}

impl fmt::Display for FocusArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Unknown => "Unknown",
            Self::Wide => "Wide",
            Self::Zone => "Zone",
            Self::Center => "Center",
            Self::FlexibleSpotS => "Spot S",
            Self::FlexibleSpotM => "Spot M",
            Self::FlexibleSpotL => "Spot L",
            Self::ExpandFlexibleSpot => "Expand Spot",
            Self::FlexibleSpot => "Spot",
            Self::TrackingWide => "Track Wide",
            Self::TrackingZone => "Track Zone",
            Self::TrackingCenter => "Track Center",
            Self::TrackingFlexibleSpotS => "Track Spot S",
            Self::TrackingFlexibleSpotM => "Track Spot M",
            Self::TrackingFlexibleSpotL => "Track Spot L",
            Self::TrackingExpandFlexibleSpot => "Track Expand",
            Self::TrackingFlexibleSpot => "Track Spot",
            Self::FlexibleSpotXS => "Spot XS",
            Self::FlexibleSpotXL => "Spot XL",
            Self::FlexibleSpotFreeSize1 => "Spot Free 1",
            Self::FlexibleSpotFreeSize2 => "Spot Free 2",
            Self::FlexibleSpotFreeSize3 => "Spot Free 3",
            Self::TrackingFlexibleSpotXS => "Track Spot XS",
            Self::TrackingFlexibleSpotXL => "Track Spot XL",
            Self::TrackingFlexibleSpotFreeSize1 => "Track Free 1",
            Self::TrackingFlexibleSpotFreeSize2 => "Track Free 2",
            Self::TrackingFlexibleSpotFreeSize3 => "Track Free 3",
        };
        write!(f, "{}", s)
    }
}

/// Subject recognition autofocus settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SubjectRecognitionAF {
    /// Subject recognition off
    Off = 1,
    /// Detect subjects but don't prioritize
    OnlyAF = 2,
    /// Detect and prioritize subjects
    PriorityAF = 3,
}

impl PropertyValue for SubjectRecognitionAF {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u8 {
            1 => Self::Off,
            2 => Self::OnlyAF,
            3 => Self::PriorityAF,
            _ => return None,
        })
    }

    fn to_raw(&self) -> u64 {
        *self as u64
    }
}

impl fmt::Display for SubjectRecognitionAF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::OnlyAF => write!(f, "Only AF"),
            Self::PriorityAF => write!(f, "Priority AF"),
        }
    }
}

/// AF/Release priority settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PrioritySetInAF {
    /// Wait for focus lock before shooting
    AF = 1,
    /// Shoot immediately regardless of focus
    Release = 2,
    /// Balance focus and responsiveness
    BalancedEmphasis = 3,
}

impl PropertyValue for PrioritySetInAF {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u8 {
            1 => Self::AF,
            2 => Self::Release,
            3 => Self::BalancedEmphasis,
            _ => return None,
        })
    }

    fn to_raw(&self) -> u64 {
        *self as u64
    }
}

impl fmt::Display for PrioritySetInAF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AF => write!(f, "AF"),
            Self::Release => write!(f, "Release"),
            Self::BalancedEmphasis => write!(f, "Balanced"),
        }
    }
}

/// Focus tracking status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FocusTrackingStatus {
    /// Tracking disabled
    Off = 1,
    /// Actively searching for focus
    Focusing = 2,
    /// Actively tracking subject
    Tracking = 3,
}

impl PropertyValue for FocusTrackingStatus {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u8 {
            1 => Self::Off,
            2 => Self::Focusing,
            3 => Self::Tracking,
            _ => return None,
        })
    }

    fn to_raw(&self) -> u64 {
        *self as u64
    }
}

impl fmt::Display for FocusTrackingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::Focusing => write!(f, "Focusing"),
            Self::Tracking => write!(f, "Tracking"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focus_mode_display() {
        assert_eq!(FocusMode::Manual.to_string(), "MF");
        assert_eq!(FocusMode::AfSingle.to_string(), "AF-S");
        assert_eq!(FocusMode::AfContinuous.to_string(), "AF-C");
    }

    #[test]
    fn test_focus_area_display() {
        assert_eq!(FocusArea::Wide.to_string(), "Wide");
        assert_eq!(FocusArea::TrackingWide.to_string(), "Track Wide");
    }

    #[test]
    fn test_subject_recognition_display() {
        assert_eq!(SubjectRecognitionAF::Off.to_string(), "Off");
        assert_eq!(SubjectRecognitionAF::PriorityAF.to_string(), "Priority AF");
    }
}
