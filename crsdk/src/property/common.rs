//! Common toggle and mode enums used across multiple property categories.

/// A generic on/off switch (Off=1, On=2).
///
/// Used for properties like AutoSlowShutter, SilentMode, and NDFilter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Switch {
    Off = 1,
    On = 2,
}

impl Switch {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            1 => Self::Off,
            2 => Self::On,
            _ => return None,
        })
    }
}

impl std::fmt::Display for Switch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    Off = 0,
    On = 1,
}

impl OnOff {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            0 => Self::Off,
            1 => Self::On,
            _ => return None,
        })
    }
}

impl std::fmt::Display for OnOff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    Automatic = 1,
    Manual = 2,
}

impl AutoManual {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            1 => Self::Automatic,
            2 => Self::Manual,
            _ => return None,
        })
    }
}

impl std::fmt::Display for AutoManual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Automatic => write!(f, "Auto"),
            Self::Manual => write!(f, "Manual"),
        }
    }
}

/// Live view display effect setting (Unknown=0, On=1, Off=2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LiveViewDisplayEffect {
    Unknown = 0,
    On = 1,
    Off = 2,
}

impl LiveViewDisplayEffect {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            0 => Self::Unknown,
            1 => Self::On,
            2 => Self::Off,
            _ => return None,
        })
    }
}

impl std::fmt::Display for LiveViewDisplayEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    NotTarget = 1,
    Standard = 2,
    SilentPriority = 3,
}

impl SilentModeApertureDrive {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            1 => Self::NotTarget,
            2 => Self::Standard,
            3 => Self::SilentPriority,
            _ => return None,
        })
    }
}

impl std::fmt::Display for SilentModeApertureDrive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotTarget => write!(f, "Not Target"),
            Self::Standard => write!(f, "Standard"),
            Self::SilentPriority => write!(f, "Silent Priority"),
        }
    }
}
