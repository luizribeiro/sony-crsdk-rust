//! White balance property types and metadata.

use super::PropertyValueType;
use crsdk_sys::DevicePropertyCode;

/// White balance settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum WhiteBalance {
    Auto = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_AWB,
    Daylight = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Daylight,
    Shade = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Shadow,
    Cloudy = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Cloudy,
    Tungsten = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Tungsten,
    Fluorescent = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent,
    FluorescentWarmWhite =
        crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_WarmWhite,
    FluorescentCoolWhite =
        crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_CoolWhite,
    FluorescentDayWhite =
        crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_DayWhite,
    FluorescentDaylight =
        crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_Daylight,
    Flash = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Flush,
    UnderwaterAuto = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Underwater_Auto,
    ColorTemp = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_ColorTemp,
    Custom1 = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Custom_1,
    Custom2 = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Custom_2,
    Custom3 = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Custom_3,
    Custom = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Custom,
}

impl WhiteBalance {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        use crsdk_sys::SCRSDK::*;
        let value = value as u16;
        Some(match value {
            CrWhiteBalanceSetting_CrWhiteBalance_AWB => Self::Auto,
            CrWhiteBalanceSetting_CrWhiteBalance_Daylight => Self::Daylight,
            CrWhiteBalanceSetting_CrWhiteBalance_Shadow => Self::Shade,
            CrWhiteBalanceSetting_CrWhiteBalance_Cloudy => Self::Cloudy,
            CrWhiteBalanceSetting_CrWhiteBalance_Tungsten => Self::Tungsten,
            CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent => Self::Fluorescent,
            CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_WarmWhite => {
                Self::FluorescentWarmWhite
            }
            CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_CoolWhite => {
                Self::FluorescentCoolWhite
            }
            CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_DayWhite => Self::FluorescentDayWhite,
            CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_Daylight => Self::FluorescentDaylight,
            CrWhiteBalanceSetting_CrWhiteBalance_Flush => Self::Flash,
            CrWhiteBalanceSetting_CrWhiteBalance_Underwater_Auto => Self::UnderwaterAuto,
            CrWhiteBalanceSetting_CrWhiteBalance_ColorTemp => Self::ColorTemp,
            CrWhiteBalanceSetting_CrWhiteBalance_Custom_1 => Self::Custom1,
            CrWhiteBalanceSetting_CrWhiteBalance_Custom_2 => Self::Custom2,
            CrWhiteBalanceSetting_CrWhiteBalance_Custom_3 => Self::Custom3,
            CrWhiteBalanceSetting_CrWhiteBalance_Custom => Self::Custom,
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
        DevicePropertyCode::CustomWBCapture => {
            "Captures a custom white balance reference. Point at a white or gray card to measure the color temperature."
        }
        DevicePropertyCode::CustomWBCaptureOperation => {
            "Triggers the custom white balance capture process. Initiates measurement of the reference target."
        }
        DevicePropertyCode::CustomWBExecutionState => {
            "Current state of custom white balance capture. Shows if capture is in progress, complete, or failed."
        }
        DevicePropertyCode::CustomWBCapturableArea => {
            "Shows the area of the frame used for custom white balance measurement."
        }
        DevicePropertyCode::CustomWBCaptureFrameSize => {
            "Size of the capture frame for custom white balance measurement."
        }
        DevicePropertyCode::CustomWBCaptureStandby => {
            "Puts the camera in standby mode for custom white balance capture."
        }
        DevicePropertyCode::CustomWBCaptureStandbyCancel => {
            "Cancels the custom white balance capture standby mode."
        }
        DevicePropertyCode::CustomWBSizeSetting => {
            "Size setting for the custom white balance capture area."
        }
        DevicePropertyCode::ColortempStep => {
            "The step size for color temperature adjustments. Smaller steps allow finer control."
        }
        DevicePropertyCode::WhiteBalanceModeSetting => {
            "Controls whether white balance is set automatically or manually."
        }
        DevicePropertyCode::WhiteBalanceSwitch => {
            "Switches between white balance modes or presets."
        }
        DevicePropertyCode::WhiteBalanceOffsetSetting => {
            "Fine-tunes white balance with offset adjustments from the base setting."
        }
        DevicePropertyCode::WhiteBalanceOffsetColorTemp => {
            "Offset adjustment to the color temperature in Kelvin."
        }
        DevicePropertyCode::WhiteBalanceOffsetColorTempATW => {
            "Offset adjustment for auto-tracking white balance color temperature."
        }
        DevicePropertyCode::WhiteBalanceOffsetTintATW => {
            "Offset adjustment for auto-tracking white balance tint (green-magenta)."
        }
        DevicePropertyCode::WhiteBalanceTint => {
            "Tint adjustment on the green-magenta axis. Corrects for fluorescent lighting color casts."
        }
        DevicePropertyCode::WhiteBalanceTintStep => {
            "The step size for tint adjustments. Smaller steps allow finer control."
        }
        DevicePropertyCode::WhiteBalanceRGain => {
            "Red channel gain for white balance. Increases or decreases red in the image."
        }
        DevicePropertyCode::WhiteBalanceBGain => {
            "Blue channel gain for white balance. Increases or decreases blue in the image."
        }
        DevicePropertyCode::WhiteBalancePresetColorTemperature => {
            "Preset color temperature values available for quick white balance selection."
        }
        DevicePropertyCode::AWB => {
            "Auto white balance status. Shows the current automatically determined color temperature."
        }
        // Routes here due to "bw" in name
        DevicePropertyCode::PictureProfileDetailAdjustBWBalance => {
            "Detail/sharpening settings. Controls edge enhancement and texture rendering."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::WhiteBalance => "WB",
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
        DevicePropertyCode::CustomWBCapture => "Custom WB Cap",
        DevicePropertyCode::CustomWBCaptureOperation => "Custom WB Op",
        DevicePropertyCode::CustomWBExecutionState => "Custom WB Status",
        DevicePropertyCode::CustomWBCapturableArea => "Custom WB Area",
        DevicePropertyCode::CustomWBCaptureFrameSize => "Custom WB Frame",
        DevicePropertyCode::CustomWBCaptureStandby => "Custom WB Standby",
        DevicePropertyCode::CustomWBCaptureStandbyCancel => "Custom WB Cancel",
        DevicePropertyCode::CustomWBSizeSetting => "Custom WB Size",
        DevicePropertyCode::WhiteBalanceOffsetColorTempATW => "WB Offset ATW (K)",
        DevicePropertyCode::WhiteBalanceOffsetTintATW => "WB Offset ATW (Tint)",
        // Routes here due to "bw" in name
        DevicePropertyCode::PictureProfileDetailAdjustBWBalance => "PP Detail B/W",
        _ => code.name(),
    }
}

pub fn value_type(code: DevicePropertyCode) -> Option<PropertyValueType> {
    use PropertyValueType as V;

    Some(match code {
        DevicePropertyCode::WhiteBalance => V::WhiteBalance,
        DevicePropertyCode::Colortemp
        | DevicePropertyCode::WhiteBalancePresetColorTemperature
        | DevicePropertyCode::WhiteBalanceOffsetColorTemp
        | DevicePropertyCode::WhiteBalanceOffsetColorTempATW => V::ColorTemperature,
        DevicePropertyCode::PrioritySetInAWB => V::PrioritySetInAWB,
        DevicePropertyCode::AWBL => V::LockIndicator,
        DevicePropertyCode::ColortempStep
        | DevicePropertyCode::WhiteBalanceTintStep
        | DevicePropertyCode::ColorTuningAB
        | DevicePropertyCode::ColorTuningGM
        | DevicePropertyCode::WhiteBalanceTint
        | DevicePropertyCode::WhiteBalanceOffsetTintATW
        | DevicePropertyCode::WhiteBalanceRGain
        | DevicePropertyCode::WhiteBalanceBGain
        | DevicePropertyCode::CustomWBCapturableArea
        | DevicePropertyCode::CustomWBCaptureFrameSize
        | DevicePropertyCode::CustomWBSizeSetting
        | DevicePropertyCode::CustomWBExecutionState => V::Integer,
        DevicePropertyCode::WhiteBalanceSwitch | DevicePropertyCode::CustomWBCaptureStandby => {
            V::Switch
        }
        DevicePropertyCode::AWB
        | DevicePropertyCode::WhiteBalanceModeSetting
        | DevicePropertyCode::WhiteBalanceOffsetSetting
        | DevicePropertyCode::CustomWBCapture
        | DevicePropertyCode::CustomWBCaptureOperation
        | DevicePropertyCode::CustomWBCaptureStandbyCancel
        // Routes here due to "bw" in name
        | DevicePropertyCode::PictureProfileDetailAdjustBWBalance => V::Integer,
        _ => return None,
    })
}
