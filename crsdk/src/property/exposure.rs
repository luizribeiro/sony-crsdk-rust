//! Exposure-related property types and metadata.

use crsdk_sys::DevicePropertyCode;

/// Exposure program mode (shooting mode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ExposureProgram {
    Manual = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_M_Manual,
    ProgramAuto = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_P_Auto,
    AperturePriority = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_A_AperturePriority,
    ShutterPriority = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_S_ShutterSpeedPriority,
    ProgramCreative = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Program_Creative,
    ProgramAction = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Program_Action,
    Portrait = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Portrait,
    Auto = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Auto,
    AutoPlus = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Auto_Plus,
    PA = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_P_A,
    PS = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_P_S,
    SportsAction = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Sports_Action,
    Sunset = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Sunset,
    Night = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Night,
    Landscape = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Landscape,
    Macro = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Macro,
    HandheldTwilight = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_HandheldTwilight,
    NightPortrait = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_NightPortrait,
    AntiMotionBlur = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_AntiMotionBlur,
    Pet = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Pet,
    Gourmet = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Gourmet,
    Fireworks = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Fireworks,
    HighSensitivity = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_HighSensitivity,
    MemoryRecall = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_MemoryRecall,
    ContinuousPriorityAE8 =
        crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_ContinuousPriority_AE_8pics,
    ContinuousPriorityAE10 =
        crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_ContinuousPriority_AE_10pics,
    ContinuousPriorityAE12 =
        crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_ContinuousPriority_AE_12pics,
    SweepPanorama3D = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_3D_SweepPanorama,
    SweepPanorama = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_SweepPanorama,
    MovieP = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_P,
    MovieA = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_A,
    MovieS = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_S,
    MovieM = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_M,
    MovieAuto = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_Auto,
    MovieF = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_F,
    MovieSQMotionP = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_SQMotion_P,
    MovieSQMotionA = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_SQMotion_A,
    MovieSQMotionS = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_SQMotion_S,
    MovieSQMotionM = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_SQMotion_M,
    MovieSQMotionAuto = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_SQMotion_AUTO,
    MovieSQMotionF = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_SQMotion_F,
    FlashOff = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Flash_Off,
    PictureEffect = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_PictureEffect,
    HiFrameRateP = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_HiFrameRate_P,
    HiFrameRateA = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_HiFrameRate_A,
    HiFrameRateS = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_HiFrameRate_S,
    HiFrameRateM = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_HiFrameRate_M,
    SQMotionP = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_SQMotion_P,
    SQMotionA = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_SQMotion_A,
    SQMotionS = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_SQMotion_S,
    SQMotionM = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_SQMotion_M,
    Movie = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_MOVIE,
    Still = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_STILL,
    MovieFMode = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_F_Mode,
    MovieIntervalRecF = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_IntervalRec_F,
    MovieIntervalRecP = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_IntervalRec_P,
    MovieIntervalRecA = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_IntervalRec_A,
    MovieIntervalRecS = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_IntervalRec_S,
    MovieIntervalRecM = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_IntervalRec_M,
    MovieIntervalRecAuto = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_IntervalRec_AUTO,
}

impl ExposureProgram {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        use crsdk_sys::SCRSDK::*;
        Some(match value as u32 {
            CrExposureProgram_CrExposure_M_Manual => Self::Manual,
            CrExposureProgram_CrExposure_P_Auto => Self::ProgramAuto,
            CrExposureProgram_CrExposure_A_AperturePriority => Self::AperturePriority,
            CrExposureProgram_CrExposure_S_ShutterSpeedPriority => Self::ShutterPriority,
            CrExposureProgram_CrExposure_Program_Creative => Self::ProgramCreative,
            CrExposureProgram_CrExposure_Program_Action => Self::ProgramAction,
            CrExposureProgram_CrExposure_Portrait => Self::Portrait,
            CrExposureProgram_CrExposure_Auto => Self::Auto,
            CrExposureProgram_CrExposure_Auto_Plus => Self::AutoPlus,
            CrExposureProgram_CrExposure_P_A => Self::PA,
            CrExposureProgram_CrExposure_P_S => Self::PS,
            CrExposureProgram_CrExposure_Sports_Action => Self::SportsAction,
            CrExposureProgram_CrExposure_Sunset => Self::Sunset,
            CrExposureProgram_CrExposure_Night => Self::Night,
            CrExposureProgram_CrExposure_Landscape => Self::Landscape,
            CrExposureProgram_CrExposure_Macro => Self::Macro,
            CrExposureProgram_CrExposure_HandheldTwilight => Self::HandheldTwilight,
            CrExposureProgram_CrExposure_NightPortrait => Self::NightPortrait,
            CrExposureProgram_CrExposure_AntiMotionBlur => Self::AntiMotionBlur,
            CrExposureProgram_CrExposure_Pet => Self::Pet,
            CrExposureProgram_CrExposure_Gourmet => Self::Gourmet,
            CrExposureProgram_CrExposure_Fireworks => Self::Fireworks,
            CrExposureProgram_CrExposure_HighSensitivity => Self::HighSensitivity,
            CrExposureProgram_CrExposure_MemoryRecall => Self::MemoryRecall,
            CrExposureProgram_CrExposure_ContinuousPriority_AE_8pics => Self::ContinuousPriorityAE8,
            CrExposureProgram_CrExposure_ContinuousPriority_AE_10pics => {
                Self::ContinuousPriorityAE10
            }
            CrExposureProgram_CrExposure_ContinuousPriority_AE_12pics => {
                Self::ContinuousPriorityAE12
            }
            CrExposureProgram_CrExposure_3D_SweepPanorama => Self::SweepPanorama3D,
            CrExposureProgram_CrExposure_SweepPanorama => Self::SweepPanorama,
            CrExposureProgram_CrExposure_Movie_P => Self::MovieP,
            CrExposureProgram_CrExposure_Movie_A => Self::MovieA,
            CrExposureProgram_CrExposure_Movie_S => Self::MovieS,
            CrExposureProgram_CrExposure_Movie_M => Self::MovieM,
            CrExposureProgram_CrExposure_Movie_Auto => Self::MovieAuto,
            CrExposureProgram_CrExposure_Movie_F => Self::MovieF,
            CrExposureProgram_CrExposure_Movie_SQMotion_P => Self::MovieSQMotionP,
            CrExposureProgram_CrExposure_Movie_SQMotion_A => Self::MovieSQMotionA,
            CrExposureProgram_CrExposure_Movie_SQMotion_S => Self::MovieSQMotionS,
            CrExposureProgram_CrExposure_Movie_SQMotion_M => Self::MovieSQMotionM,
            CrExposureProgram_CrExposure_Movie_SQMotion_AUTO => Self::MovieSQMotionAuto,
            CrExposureProgram_CrExposure_Movie_SQMotion_F => Self::MovieSQMotionF,
            CrExposureProgram_CrExposure_Flash_Off => Self::FlashOff,
            CrExposureProgram_CrExposure_PictureEffect => Self::PictureEffect,
            CrExposureProgram_CrExposure_HiFrameRate_P => Self::HiFrameRateP,
            CrExposureProgram_CrExposure_HiFrameRate_A => Self::HiFrameRateA,
            CrExposureProgram_CrExposure_HiFrameRate_S => Self::HiFrameRateS,
            CrExposureProgram_CrExposure_HiFrameRate_M => Self::HiFrameRateM,
            CrExposureProgram_CrExposure_SQMotion_P => Self::SQMotionP,
            CrExposureProgram_CrExposure_SQMotion_A => Self::SQMotionA,
            CrExposureProgram_CrExposure_SQMotion_S => Self::SQMotionS,
            CrExposureProgram_CrExposure_SQMotion_M => Self::SQMotionM,
            CrExposureProgram_CrExposure_MOVIE => Self::Movie,
            CrExposureProgram_CrExposure_STILL => Self::Still,
            CrExposureProgram_CrExposure_Movie_F_Mode => Self::MovieFMode,
            CrExposureProgram_CrExposure_Movie_IntervalRec_F => Self::MovieIntervalRecF,
            CrExposureProgram_CrExposure_Movie_IntervalRec_P => Self::MovieIntervalRecP,
            CrExposureProgram_CrExposure_Movie_IntervalRec_A => Self::MovieIntervalRecA,
            CrExposureProgram_CrExposure_Movie_IntervalRec_S => Self::MovieIntervalRecS,
            CrExposureProgram_CrExposure_Movie_IntervalRec_M => Self::MovieIntervalRecM,
            CrExposureProgram_CrExposure_Movie_IntervalRec_AUTO => Self::MovieIntervalRecAuto,
            _ => return None,
        })
    }
}

/// Metering mode settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum MeteringMode {
    Average = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_Average as u16,
    CenterWeightedAverage =
        crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_CenterWeightedAverage as u16,
    MultiSpot = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_MultiSpot as u16,
    CenterSpot = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_CenterSpot as u16,
    Multi = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_Multi as u16,
    CenterWeighted = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_CenterWeighted as u16,
    EntireScreenAverage = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_EntireScreenAverage as u16,
    SpotStandard = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_Spot_Standard as u16,
    SpotLarge = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_Spot_Large as u16,
    HighLightWeighted = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_HighLightWeighted as u16,
    Standard = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_Standard as u16,
    Backlight = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_Backlight as u16,
    Spotlight = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_Spotlight as u16,
}

impl MeteringMode {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        use crsdk_sys::SCRSDK::*;
        Some(match value as u16 {
            CrMeteringMode_CrMetering_Average => Self::Average,
            CrMeteringMode_CrMetering_CenterWeightedAverage => Self::CenterWeightedAverage,
            CrMeteringMode_CrMetering_MultiSpot => Self::MultiSpot,
            CrMeteringMode_CrMetering_CenterSpot => Self::CenterSpot,
            CrMeteringMode_CrMetering_Multi => Self::Multi,
            CrMeteringMode_CrMetering_CenterWeighted => Self::CenterWeighted,
            CrMeteringMode_CrMetering_EntireScreenAverage => Self::EntireScreenAverage,
            CrMeteringMode_CrMetering_Spot_Standard => Self::SpotStandard,
            CrMeteringMode_CrMetering_Spot_Large => Self::SpotLarge,
            CrMeteringMode_CrMetering_HighLightWeighted => Self::HighLightWeighted,
            CrMeteringMode_CrMetering_Standard => Self::Standard,
            CrMeteringMode_CrMetering_Backlight => Self::Backlight,
            CrMeteringMode_CrMetering_Spotlight => Self::Spotlight,
            _ => return None,
        })
    }
}

/// Shutter mode status for cinema cameras
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ShutterModeStatus {
    Off = 1,
    Speed = 2,
    Angle = 3,
    Ecs = 4,
    Auto = 5,
}

impl ShutterModeStatus {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            1 => Self::Off,
            2 => Self::Speed,
            3 => Self::Angle,
            4 => Self::Ecs,
            5 => Self::Auto,
            _ => return None,
        })
    }
}

impl std::fmt::Display for ShutterModeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::Speed => write!(f, "Speed"),
            Self::Angle => write!(f, "Angle"),
            Self::Ecs => write!(f, "ECS"),
            Self::Auto => write!(f, "Auto"),
        }
    }
}

/// Shutter mode selection (Speed vs Angle)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ShutterMode {
    Speed = 1,
    Angle = 2,
}

impl ShutterMode {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            1 => Self::Speed,
            2 => Self::Angle,
            _ => return None,
        })
    }
}

impl std::fmt::Display for ShutterMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Speed => write!(f, "Speed"),
            Self::Angle => write!(f, "Angle"),
        }
    }
}

/// Exposure control type (P/A/S/M vs Flexible Exposure)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExposureCtrlType {
    Pasm = 1,
    FlexibleExposure = 2,
}

impl ExposureCtrlType {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            1 => Self::Pasm,
            2 => Self::FlexibleExposure,
            _ => return None,
        })
    }
}

impl std::fmt::Display for ExposureCtrlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pasm => write!(f, "P/A/S/M"),
            Self::FlexibleExposure => write!(f, "Flexible Exp"),
        }
    }
}

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::FNumber => {
            "Controls the lens aperture opening. Lower values (f/1.4) create shallow depth of field with blurry backgrounds. Higher values (f/16) keep more of the scene in focus but require more light."
        }
        DevicePropertyCode::ShutterSpeed => {
            "How long the sensor is exposed to light. Fast speeds (1/1000s) freeze motion but need more light. Slow speeds (1/30s) allow more light but can cause motion blur."
        }
        DevicePropertyCode::IsoSensitivity => {
            "Sensor light sensitivity. Lower values (100) produce cleaner images in bright light. Higher values (6400+) work in low light but add noise/grain."
        }
        DevicePropertyCode::ExposureBiasCompensation => {
            "Adjusts overall exposure brightness. Positive values (+1, +2) brighten the image. Negative values (-1, -2) darken it. Useful when the camera's metering is fooled by very bright or dark scenes."
        }
        DevicePropertyCode::ExposureProgramMode => {
            "Determines how aperture and shutter speed are set. Manual (M) gives full control. Aperture Priority (A) lets you set aperture while camera picks shutter. Shutter Priority (S) is the opposite. Program (P) automates both."
        }
        DevicePropertyCode::ExposureIndex => {
            "Exposure Index (EI) for cinema cameras. Similar to ISO but specifically for log/cinema workflows where the actual sensor sensitivity remains fixed."
        }
        DevicePropertyCode::MeteringMode => {
            "How the camera measures light. Multi uses the whole frame. Center-weighted prioritizes the middle. Spot measures only a small area, useful for backlit subjects."
        }
        DevicePropertyCode::AutoSlowShutter => {
            "In auto modes, allows the camera to use slower shutter speeds in low light. Helps maintain lower ISO but may introduce motion blur."
        }
        DevicePropertyCode::DRO => {
            "Dynamic Range Optimizer. Automatically adjusts shadows and highlights to preserve detail in high-contrast scenes. Works on JPEGs only."
        }
        DevicePropertyCode::ExposureCtrlType => {
            "P/A/S/M uses traditional exposure modes. Flexible Exposure allows independent control of aperture, shutter, and ISO regardless of the selected mode."
        }
        DevicePropertyCode::ShutterSlow => {
            "Enables extended slow shutter speeds for long exposures. Useful for light trails, smooth water, or low-light photography with a tripod."
        }
        DevicePropertyCode::ShutterSlowFrames => {
            "Number of frames to accumulate when using slow shutter. Higher values create longer effective exposures for creative effects."
        }
        DevicePropertyCode::GainControlSetting => {
            "Gain control method for cinema cameras. Choose between ISO-based or dB-based gain adjustment."
        }
        DevicePropertyCode::GainBaseIsoSensitivity => {
            "Native/base ISO setting. Dual native ISO cameras have two optimal sensitivity levels with minimal noise."
        }
        DevicePropertyCode::GaindBValue => {
            "Gain level in decibels. Common in cinema workflows. 0dB is the base sensitivity, positive values amplify the signal."
        }
        DevicePropertyCode::ShutterAngle => {
            "Shutter timing expressed as an angle (45°-360°). 180° is cinematic standard, giving natural motion blur at 24fps."
        }
        DevicePropertyCode::ShutterModeSetting => {
            "Auto lets the camera control shutter timing. Manual gives you direct control over shutter speed or angle."
        }
        DevicePropertyCode::ShutterModeStatus => {
            "Shows the current shutter mode: Off, Speed (time-based), Angle (degree-based), ECS (Extended Clear Scan), or Auto."
        }
        DevicePropertyCode::ShutterMode => {
            "Choose how shutter is measured: Speed uses time fractions (1/100s), Angle uses degrees (180°) for consistent motion blur across frame rates."
        }
        DevicePropertyCode::ShutterSetting => {
            "Enables or disables manual shutter control. When Off, the camera handles shutter automatically."
        }
        DevicePropertyCode::ShutterECSSetting => {
            "Extended Clear Scan reduces banding when filming monitors and LED screens by syncing shutter to the display's refresh rate."
        }
        DevicePropertyCode::IrisModeSetting => {
            "Aperture control mode. Auto lets the camera adjust. Manual gives full control for consistent exposure."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::FNumber => "Aperture (f-number)",
        DevicePropertyCode::ShutterSpeed => "Shutter Speed",
        DevicePropertyCode::IsoSensitivity => "ISO",
        DevicePropertyCode::ExposureBiasCompensation => "Exposure Comp",
        DevicePropertyCode::ExposureProgramMode => "Exposure Mode",
        DevicePropertyCode::ExposureIndex => "Exposure Index (EI)",
        DevicePropertyCode::ExposureCtrlType => "Exposure Control Type",
        DevicePropertyCode::ExposureStep => "Exposure Step",
        DevicePropertyCode::AutoSlowShutter => "Auto Slow Shutter",
        DevicePropertyCode::ShutterModeSetting => "Shutter Control",
        DevicePropertyCode::ShutterModeStatus => "Shutter Mode",
        DevicePropertyCode::ShutterMode => "Shutter Unit",
        DevicePropertyCode::ShutterAngle => "Shutter Angle",
        DevicePropertyCode::ShutterSetting => "Shutter",
        DevicePropertyCode::ShutterSlow => "Slow Shutter",
        DevicePropertyCode::ShutterSlowFrames => "Slow Shutter Frames",
        DevicePropertyCode::ShutterECSSetting => "ECS Mode",
        DevicePropertyCode::ShutterECSFrequency => "ECS Frequency",
        DevicePropertyCode::ShutterECSNumber => "ECS Number",
        DevicePropertyCode::ShutterECSNumberStep => "ECS Step",
        DevicePropertyCode::GainControlSetting => "Gain Control",
        DevicePropertyCode::GainBaseIsoSensitivity => "Base ISO",
        DevicePropertyCode::GainBaseSensitivity => "Base Sensitivity",
        DevicePropertyCode::GainUnitSetting => "Gain Unit",
        DevicePropertyCode::GaindBCurrentValue => "Gain (dB)",
        DevicePropertyCode::GaindBValue => "Gain Value (dB)",
        DevicePropertyCode::IrisModeSetting => "Iris Mode",
        DevicePropertyCode::IrisDisplayUnit => "Iris Display Unit",
        DevicePropertyCode::IrisCloseSetting => "Iris Close Enable",
        DevicePropertyCode::BaseISOSwitchEI => "Base ISO (EI)",
        DevicePropertyCode::DRO => "D-Range Optimizer",
        DevicePropertyCode::MeteringMode => "Metering Mode",
        DevicePropertyCode::MeteredManualLevel => "Metered Manual Level",
        _ => code.name(),
    }
}
