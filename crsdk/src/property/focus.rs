//! Focus-related property types and metadata.

use super::PropertyValueType;
use crsdk_sys::DevicePropertyCode;

/// Focus mode settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FocusMode {
    Manual = crsdk_sys::SCRSDK::CrFocusMode_CrFocus_MF as u16,
    AfSingle = crsdk_sys::SCRSDK::CrFocusMode_CrFocus_AF_S as u16,
    AfContinuous = crsdk_sys::SCRSDK::CrFocusMode_CrFocus_AF_C as u16,
    AfAutomatic = crsdk_sys::SCRSDK::CrFocusMode_CrFocus_AF_A as u16,
    AfDeepLearning = crsdk_sys::SCRSDK::CrFocusMode_CrFocus_AF_D as u16,
    DirectManual = crsdk_sys::SCRSDK::CrFocusMode_CrFocus_DMF as u16,
    PresetFocus = crsdk_sys::SCRSDK::CrFocusMode_CrFocus_PF as u16,
}

impl FocusMode {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        use crsdk_sys::SCRSDK::*;
        Some(match value as u16 {
            x if x == CrFocusMode_CrFocus_MF as u16 => Self::Manual,
            x if x == CrFocusMode_CrFocus_AF_S as u16 => Self::AfSingle,
            x if x == CrFocusMode_CrFocus_AF_C as u16 => Self::AfContinuous,
            x if x == CrFocusMode_CrFocus_AF_A as u16 => Self::AfAutomatic,
            x if x == CrFocusMode_CrFocus_AF_D as u16 => Self::AfDeepLearning,
            x if x == CrFocusMode_CrFocus_DMF as u16 => Self::DirectManual,
            x if x == CrFocusMode_CrFocus_PF as u16 => Self::PresetFocus,
            _ => return None,
        })
    }
}

/// Focus area settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FocusArea {
    Unknown = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Unknown as u16,
    Wide = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Wide as u16,
    Zone = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Zone as u16,
    Center = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Center as u16,
    FlexibleSpotS = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_S as u16,
    FlexibleSpotM = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_M as u16,
    FlexibleSpotL = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_L as u16,
    ExpandFlexibleSpot = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Expand_Flexible_Spot as u16,
    FlexibleSpot = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot as u16,
    TrackingWide = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Wide as u16,
    TrackingZone = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Zone as u16,
    TrackingCenter = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Center as u16,
    TrackingFlexibleSpotS =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_S as u16,
    TrackingFlexibleSpotM =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_M as u16,
    TrackingFlexibleSpotL =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_L as u16,
    TrackingExpandFlexibleSpot =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Expand_Flexible_Spot as u16,
    TrackingFlexibleSpot = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot as u16,
    FlexibleSpotXS = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_XS as u16,
    FlexibleSpotXL = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_XL as u16,
    FlexibleSpotFreeSize1 =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_FreeSize1 as u16,
    FlexibleSpotFreeSize2 =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_FreeSize2 as u16,
    FlexibleSpotFreeSize3 =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_FreeSize3 as u16,
    TrackingFlexibleSpotXS =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_XS as u16,
    TrackingFlexibleSpotXL =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_XL as u16,
    TrackingFlexibleSpotFreeSize1 =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_FreeSize1 as u16,
    TrackingFlexibleSpotFreeSize2 =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_FreeSize2 as u16,
    TrackingFlexibleSpotFreeSize3 =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_FreeSize3 as u16,
}

impl FocusArea {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        use crsdk_sys::SCRSDK::*;
        Some(match value as u16 {
            CrFocusArea_CrFocusArea_Unknown => Self::Unknown,
            CrFocusArea_CrFocusArea_Wide => Self::Wide,
            CrFocusArea_CrFocusArea_Zone => Self::Zone,
            CrFocusArea_CrFocusArea_Center => Self::Center,
            CrFocusArea_CrFocusArea_Flexible_Spot_S => Self::FlexibleSpotS,
            CrFocusArea_CrFocusArea_Flexible_Spot_M => Self::FlexibleSpotM,
            CrFocusArea_CrFocusArea_Flexible_Spot_L => Self::FlexibleSpotL,
            CrFocusArea_CrFocusArea_Expand_Flexible_Spot => Self::ExpandFlexibleSpot,
            CrFocusArea_CrFocusArea_Flexible_Spot => Self::FlexibleSpot,
            CrFocusArea_CrFocusArea_Tracking_Wide => Self::TrackingWide,
            CrFocusArea_CrFocusArea_Tracking_Zone => Self::TrackingZone,
            CrFocusArea_CrFocusArea_Tracking_Center => Self::TrackingCenter,
            CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_S => Self::TrackingFlexibleSpotS,
            CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_M => Self::TrackingFlexibleSpotM,
            CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_L => Self::TrackingFlexibleSpotL,
            CrFocusArea_CrFocusArea_Tracking_Expand_Flexible_Spot => {
                Self::TrackingExpandFlexibleSpot
            }
            CrFocusArea_CrFocusArea_Tracking_Flexible_Spot => Self::TrackingFlexibleSpot,
            CrFocusArea_CrFocusArea_Flexible_Spot_XS => Self::FlexibleSpotXS,
            CrFocusArea_CrFocusArea_Flexible_Spot_XL => Self::FlexibleSpotXL,
            CrFocusArea_CrFocusArea_Flexible_Spot_FreeSize1 => Self::FlexibleSpotFreeSize1,
            CrFocusArea_CrFocusArea_Flexible_Spot_FreeSize2 => Self::FlexibleSpotFreeSize2,
            CrFocusArea_CrFocusArea_Flexible_Spot_FreeSize3 => Self::FlexibleSpotFreeSize3,
            CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_XS => Self::TrackingFlexibleSpotXS,
            CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_XL => Self::TrackingFlexibleSpotXL,
            CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_FreeSize1 => {
                Self::TrackingFlexibleSpotFreeSize1
            }
            CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_FreeSize2 => {
                Self::TrackingFlexibleSpotFreeSize2
            }
            CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_FreeSize3 => {
                Self::TrackingFlexibleSpotFreeSize3
            }
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SubjectRecognitionAF {
    Off = 1,
    OnlyAF = 2,
    PriorityAF = 3,
}

impl SubjectRecognitionAF {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            1 => Self::Off,
            2 => Self::OnlyAF,
            3 => Self::PriorityAF,
            _ => return None,
        })
    }
}

impl std::fmt::Display for SubjectRecognitionAF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::OnlyAF => write!(f, "Only AF"),
            Self::PriorityAF => write!(f, "Priority AF"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PrioritySetInAF {
    AF = 1,
    Release = 2,
    BalancedEmphasis = 3,
}

impl PrioritySetInAF {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            1 => Self::AF,
            2 => Self::Release,
            3 => Self::BalancedEmphasis,
            _ => return None,
        })
    }
}

impl std::fmt::Display for PrioritySetInAF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AF => write!(f, "AF"),
            Self::Release => write!(f, "Release"),
            Self::BalancedEmphasis => write!(f, "Balanced"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FocusTrackingStatus {
    Off = 1,
    Focusing = 2,
    Tracking = 3,
}

impl FocusTrackingStatus {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            1 => Self::Off,
            2 => Self::Focusing,
            3 => Self::Tracking,
            _ => return None,
        })
    }
}

impl std::fmt::Display for FocusTrackingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::Focusing => write!(f, "Focusing"),
            Self::Tracking => write!(f, "Tracking"),
        }
    }
}

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::FocusMode => {
            "AF-S (Single) locks focus once acquired—good for still subjects. AF-C (Continuous) tracks moving subjects. MF (Manual) gives you direct control via the lens ring."
        }
        DevicePropertyCode::FocusArea => {
            "Where the camera looks for focus. Wide searches the whole frame. Zone uses a portion. Center uses the middle. Spot/Expand Spot use precise points for accuracy."
        }
        DevicePropertyCode::AFTrackingSensitivity => {
            "How quickly AF reacts to subject distance changes. High sensitivity tracks fast-moving subjects but may be distracted. Low sensitivity is more stable but slower to adapt."
        }
        DevicePropertyCode::FocusMagnifierSetting => {
            "Zooms in on the focus point for precise manual focus checking. Essential for critical focus in video and macro work."
        }
        DevicePropertyCode::AFAssist => {
            "Emits a beam or pattern to help autofocus in dark conditions. Useful indoors but may be distracting for subjects."
        }
        DevicePropertyCode::PreAF => {
            "Camera continuously adjusts focus even before half-pressing the shutter. Faster initial focus but uses more battery."
        }
        DevicePropertyCode::SubjectRecognitionInAF => {
            "Enables AI-powered detection of eyes, faces, animals, birds, vehicles, etc. The camera automatically prioritizes these subjects for focus."
        }
        DevicePropertyCode::SubjectRecognitionAF => {
            "Controls how subject recognition affects autofocus. Off disables recognition. Only AF detects but doesn't prioritize. Priority AF both detects and prioritizes recognized subjects."
        }
        DevicePropertyCode::AFTransitionSpeed => {
            "How smoothly focus changes between subjects in video. Slower transitions look more cinematic. Faster is better for action."
        }
        DevicePropertyCode::PrioritySetInAFS => {
            "What takes priority in AF-S mode. AF waits for focus lock before shooting. Release fires immediately. Balanced tries to achieve focus but won't delay too long."
        }
        DevicePropertyCode::PrioritySetInAFC => {
            "What takes priority in AF-C mode. AF waits for focus lock before shooting. Release fires immediately. Balanced tries to achieve focus but won't delay too long."
        }
        DevicePropertyCode::FocusTrackingStatus => {
            "Current state of focus tracking. Off means tracking is disabled. Focusing means actively searching. Tracking means actively following a subject."
        }
        DevicePropertyCode::FocusModeSetting => {
            "Choose between automatic or manual focus mode control. Automatic lets the camera manage focus mode. Manual gives you direct control."
        }
        DevicePropertyCode::AFWithShutter => {
            "When enabled, half-pressing the shutter activates autofocus. Disable to separate focus from shutter for back-button focus workflows."
        }
        DevicePropertyCode::FaceEyeFrameDisplay => {
            "Shows or hides the frame overlay around detected faces and eyes. Useful visual feedback for subject tracking."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::FocusMode => "Focus Mode",
        DevicePropertyCode::FocusModeStatus => "Focus Mode Status",
        DevicePropertyCode::FocusModeSetting => "Focus Mode Setting",
        DevicePropertyCode::FocusArea => "Focus Area",
        DevicePropertyCode::AFTrackingSensitivity => "AF Tracking Sensitivity",
        DevicePropertyCode::AFAreaPosition => "AF Area Position",
        DevicePropertyCode::FocusMagnifierSetting => "Focus Magnifier",
        DevicePropertyCode::NearFar => "Near/Far Adjust",
        DevicePropertyCode::AFAssist => "AF Assist",
        DevicePropertyCode::AFIlluminator => "AF Illuminator",
        DevicePropertyCode::AFWithShutter => "AF with Shutter",
        DevicePropertyCode::PreAF => "Pre-AF",
        DevicePropertyCode::FocusOperation => "Focus Operation",
        DevicePropertyCode::FocusDrivingStatus => "Focus Driving Status",
        DevicePropertyCode::FocusIndication => "Focus Indication",
        DevicePropertyCode::FocusPositionSetting => "Focus Position",
        DevicePropertyCode::FocusPositionCurrentValue => "Focus Position (Current)",
        DevicePropertyCode::AFTransitionSpeed => "AF Transition Speed",
        DevicePropertyCode::AFSubjShiftSens => "AF Subject Shift Sensitivity",
        DevicePropertyCode::AFTrackForSpeedChange => "AF Track Speed Change",
        DevicePropertyCode::PrioritySetInAFS => "Priority in AF-S",
        DevicePropertyCode::PrioritySetInAFC => "Priority in AF-C",
        DevicePropertyCode::PushAutoFocus => "Push Auto Focus",
        DevicePropertyCode::AutoFocusHold => "AF Hold",
        DevicePropertyCode::FocusBracketShotNumber => "Focus Bracket Shots",
        DevicePropertyCode::FocusBracketFocusRange => "Focus Bracket Range",
        DevicePropertyCode::SubjectRecognitionInAF => "Subject Recognition",
        DevicePropertyCode::SubjectRecognitionAF => "Subject Recognition AF",
        DevicePropertyCode::FaceEyeDetectionAFStatus => "Face/Eye Detection Status",
        DevicePropertyCode::FaceEyeFrameDisplay => "Face/Eye Frame Display",
        DevicePropertyCode::FocusTrackingStatus => "Focus Tracking Status",
        _ => code.name(),
    }
}

pub fn value_type(code: DevicePropertyCode) -> Option<PropertyValueType> {
    use PropertyValueType as V;

    Some(match code {
        DevicePropertyCode::FocusMode | DevicePropertyCode::FocusModeSetting => V::FocusMode,
        DevicePropertyCode::FocusArea => V::FocusArea,
        DevicePropertyCode::SubjectRecognitionAF | DevicePropertyCode::SubjectRecognitionInAF => {
            V::SubjectRecognitionAF
        }
        DevicePropertyCode::PrioritySetInAFS | DevicePropertyCode::PrioritySetInAFC => {
            V::PrioritySetInAF
        }
        DevicePropertyCode::FocusTrackingStatus => V::FocusTrackingStatus,
        DevicePropertyCode::AFAssist
        | DevicePropertyCode::PreAF
        | DevicePropertyCode::AFWithShutter
        | DevicePropertyCode::FaceEyeFrameDisplay => V::Switch,
        _ => return None,
    })
}
