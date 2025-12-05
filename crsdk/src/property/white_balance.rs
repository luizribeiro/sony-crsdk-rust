//! White balance property types and metadata.

use super::PropertyValueType;
use crsdk_sys::DevicePropertyCode;

/// White balance settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum WhiteBalance {
    Auto = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_AWB as u16,
    Daylight = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Daylight as u16,
    Shade = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Shadow as u16,
    Cloudy = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Cloudy as u16,
    Tungsten = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Tungsten as u16,
    Fluorescent = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent as u16,
    FluorescentWarmWhite =
        crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_WarmWhite as u16,
    FluorescentCoolWhite =
        crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_CoolWhite as u16,
    FluorescentDayWhite =
        crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_DayWhite as u16,
    FluorescentDaylight =
        crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_Daylight as u16,
    Flash = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Flush as u16,
    UnderwaterAuto = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Underwater_Auto as u16,
    ColorTemp = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_ColorTemp as u16,
    Custom1 = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Custom_1 as u16,
    Custom2 = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Custom_2 as u16,
    Custom3 = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Custom_3 as u16,
    Custom = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Custom as u16,
}

impl WhiteBalance {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        use crsdk_sys::SCRSDK::*;
        Some(match value as u16 {
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_AWB => Self::Auto,
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_Daylight => Self::Daylight,
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_Shadow => Self::Shade,
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_Cloudy => Self::Cloudy,
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_Tungsten => Self::Tungsten,
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent => Self::Fluorescent,
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_WarmWhite => {
                Self::FluorescentWarmWhite
            }
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_CoolWhite => {
                Self::FluorescentCoolWhite
            }
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_DayWhite => {
                Self::FluorescentDayWhite
            }
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_Daylight => {
                Self::FluorescentDaylight
            }
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_Flush => Self::Flash,
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_Underwater_Auto => Self::UnderwaterAuto,
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_ColorTemp => Self::ColorTemp,
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_Custom_1 => Self::Custom1,
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_Custom_2 => Self::Custom2,
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_Custom_3 => Self::Custom3,
            x if x == CrWhiteBalanceSetting_CrWhiteBalance_Custom => Self::Custom,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum LockIndicator {
    Unknown = 0,
    Unlocked = 1,
    Locked = 2,
}

impl LockIndicator {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u16 {
            0 => Self::Unknown,
            1 => Self::Unlocked,
            2 => Self::Locked,
            _ => return None,
        })
    }
}

impl std::fmt::Display for LockIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Unlocked => write!(f, "Unlocked"),
            Self::Locked => write!(f, "Locked"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PrioritySetInAWB {
    Standard = 1,
    Ambience = 2,
    White = 3,
}

impl PrioritySetInAWB {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            1 => Self::Standard,
            2 => Self::Ambience,
            3 => Self::White,
            _ => return None,
        })
    }
}

impl std::fmt::Display for PrioritySetInAWB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standard => write!(f, "Standard"),
            Self::Ambience => write!(f, "Ambience"),
            Self::White => write!(f, "White"),
        }
    }
}

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::WhiteBalance => {
            "Adjusts color temperature so whites appear neutral. Auto works in most cases. Presets (Daylight, Tungsten, etc.) match specific light sources. Custom/Kelvin allows precise control."
        }
        DevicePropertyCode::Colortemp => {
            "Specific color temperature in Kelvin. Lower values (3200K) are warmer/orange for tungsten light. Higher values (5600K+) are cooler/blue for daylight."
        }
        DevicePropertyCode::ColorTuningAB => {
            "Fine-tunes white balance on the amber-blue axis. Positive shifts toward amber (warm), negative toward blue (cool)."
        }
        DevicePropertyCode::ColorTuningGM => {
            "Fine-tunes white balance on the green-magenta axis. Adjusts for fluorescent lighting or creative color effects."
        }
        DevicePropertyCode::AWBL => {
            "Locks the current auto white balance setting. Useful when you want consistent color across multiple shots in changing light."
        }
        DevicePropertyCode::PrioritySetInAWB => {
            "Controls AWB color bias. Standard is neutral. Ambience preserves warm/cool feeling of the light source. White makes whites appear more neutral."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::WhiteBalance => "White Balance",
        DevicePropertyCode::WhiteBalanceModeSetting => "WB Mode",
        DevicePropertyCode::Colortemp => "Color Temperature",
        DevicePropertyCode::ColortempStep => "Color Temp Step",
        DevicePropertyCode::ColorTuningAB => "WB Shift A-B",
        DevicePropertyCode::ColorTuningGM => "WB Shift G-M",
        DevicePropertyCode::WhiteBalanceSwitch => "WB Switch",
        DevicePropertyCode::WhiteBalanceOffsetSetting => "WB Offset",
        DevicePropertyCode::WhiteBalanceOffsetColorTemp => "WB Offset (Color Temp)",
        DevicePropertyCode::WhiteBalanceTint => "WB Tint",
        DevicePropertyCode::WhiteBalanceTintStep => "WB Tint Step",
        DevicePropertyCode::WhiteBalanceRGain => "WB R Gain",
        DevicePropertyCode::WhiteBalanceBGain => "WB B Gain",
        DevicePropertyCode::WhiteBalancePresetColorTemperature => "WB Preset Color Temp",
        DevicePropertyCode::AWBL => "AWB Lock",
        DevicePropertyCode::AWB => "Auto White Balance",
        DevicePropertyCode::PrioritySetInAWB => "Priority in AWB",
        DevicePropertyCode::CustomWBCapture => "Custom WB Capture",
        DevicePropertyCode::CustomWBCaptureOperation => "Custom WB Operation",
        DevicePropertyCode::CustomWBExecutionState => "Custom WB Status",
        _ => code.name(),
    }
}

pub fn value_type(code: DevicePropertyCode) -> Option<PropertyValueType> {
    use PropertyValueType as V;

    Some(match code {
        DevicePropertyCode::WhiteBalance => V::WhiteBalance,
        DevicePropertyCode::Colortemp => V::ColorTemperature,
        DevicePropertyCode::PrioritySetInAWB => V::PrioritySetInAWB,
        DevicePropertyCode::AWBL => V::LockIndicator,
        _ => return None,
    })
}
