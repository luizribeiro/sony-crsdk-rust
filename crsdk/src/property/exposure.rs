//! Exposure-related property types and metadata.

use super::PropertyValueType;
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
        DevicePropertyCode::IrisDisplayUnit => {
            "How aperture values are displayed. F-stop (f/2.8) is standard. T-stop accounts for light transmission loss in the lens."
        }
        DevicePropertyCode::IrisCloseSetting => {
            "Allows closing the iris completely. Used for sensor protection or specific exposure effects."
        }
        DevicePropertyCode::ExposureStep => {
            "Granularity of exposure adjustments. 1/3 EV gives finer control, 1/2 EV gives larger steps."
        }
        DevicePropertyCode::BulbExposureTimeSetting => {
            "Sets the exposure time for bulb mode. Allows precise long exposures without holding the shutter button."
        }
        DevicePropertyCode::ExtendedShutterSpeed => {
            "Enables shutter speeds beyond the standard range. Allows very long exposures for night photography or creative effects."
        }
        DevicePropertyCode::HighIsoNR => {
            "Noise reduction applied at high ISO values. Reduces grain but may soften fine details."
        }
        DevicePropertyCode::LongExposureNR => {
            "Noise reduction for long exposures. Takes a dark frame to subtract hot pixels. Doubles exposure time."
        }
        DevicePropertyCode::IsoAutoMinShutterSpeedMode => {
            "How minimum shutter speed is determined in Auto ISO. Faster keeps shutter quick, Slower prioritizes low ISO."
        }
        DevicePropertyCode::IsoAutoMinShutterSpeedManual => {
            "Manual minimum shutter speed when using Auto ISO. Camera won't go slower than this value."
        }
        DevicePropertyCode::IsoAutoMinShutterSpeedPreset => {
            "Preset minimum shutter speed based on focal length. Helps prevent motion blur from camera shake."
        }
        DevicePropertyCode::IsoAutoRangeLimitMin => {
            "Minimum ISO when using Auto ISO. Keeps images clean in good light by preventing unnecessary sensitivity boost."
        }
        DevicePropertyCode::IsoAutoRangeLimitMax => {
            "Maximum ISO when using Auto ISO. Limits noise by capping how high sensitivity can go."
        }
        DevicePropertyCode::IsoCurrentSensitivity => {
            "Current effective ISO value. May differ from set ISO due to Auto ISO or exposure compensation."
        }
        DevicePropertyCode::ShutterSpeedValue => {
            "Numeric shutter speed value. Upper bits are numerator, lower bits are denominator of the fraction."
        }
        DevicePropertyCode::ShutterSpeedCurrentValue => {
            "Current effective shutter speed. May differ from set value in auto modes or with exposure compensation."
        }
        DevicePropertyCode::ShutterType => {
            "Mechanical vs electronic shutter. Electronic is silent and faster but may cause rolling shutter artifacts."
        }
        DevicePropertyCode::ShutterSelectMode => {
            "Chooses between shutter types. Auto selects based on conditions, Manual lets you force a specific type."
        }
        DevicePropertyCode::ShutterReleaseTimeLagControl => {
            "Controls shutter release delay. Standard mode optimizes image quality, Speed mode minimizes delay."
        }
        DevicePropertyCode::BaseISOSwitchEI => {
            "Switches between base ISO sensitivities on dual-ISO sensors. Each base has optimal dynamic range."
        }
        DevicePropertyCode::GainUnitSetting => {
            "Display gain as ISO values or decibels (dB). dB is common in video workflows."
        }
        DevicePropertyCode::GaindBCurrentValue => {
            "Current gain level in decibels. 0dB is base sensitivity."
        }
        DevicePropertyCode::FacePriorityInMultiMetering => {
            "Prioritizes detected faces when metering exposure. Ensures faces are properly exposed even in challenging lighting."
        }
        DevicePropertyCode::MeteredManualLevel => {
            "Exposure meter reading in manual mode. Shows how current settings compare to metered exposure."
        }
        DevicePropertyCode::IntervalRecShutterType => {
            "Shutter type for interval shooting. Auto selects automatically. Mechanical uses the physical shutter. Electronic is silent but may cause rolling shutter."
        }
        DevicePropertyCode::ShutterECSNumber => {
            "Extended Clear Scan number setting. Fine-tunes the ECS frequency for eliminating banding on specific displays."
        }
        DevicePropertyCode::ShutterECSNumberStep => {
            "Step size for ECS number adjustments. Smaller steps allow more precise tuning."
        }
        DevicePropertyCode::ShutterECSFrequency => {
            "Extended Clear Scan frequency. Match this to your display's refresh rate to eliminate banding."
        }
        DevicePropertyCode::GainBaseSensitivity => {
            "Base sensor sensitivity for cinema cameras. Determines the native ISO/gain starting point."
        }
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterHi => {
            "Frames per second for Hi continuous mode with electronic shutter."
        }
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterHiPlus => {
            "Frames per second for Hi+ continuous mode with electronic shutter. Fastest burst speed."
        }
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterMid => {
            "Frames per second for Mid continuous mode with electronic shutter."
        }
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterLo => {
            "Frames per second for Lo continuous mode with electronic shutter."
        }
        DevicePropertyCode::HighResolutionShutterSpeed => {
            "Fine-tuned shutter speed beyond standard increments. Used to eliminate banding from flickering artificial light sources like LED and fluorescent."
        }
        DevicePropertyCode::HighResolutionShutterSpeedAdjust => {
            "Fine-grained adjustment for high-resolution shutter speeds. Allows decimal values between standard stops to precisely match light source flicker frequency."
        }
        DevicePropertyCode::HighResolutionShutterSpeedAdjustInIntegralMultiples => {
            "Constrains high-resolution shutter speed adjustments to integral multiples. Provides structured fine-tuning in proportional steps."
        }
        DevicePropertyCode::HighResolutionShutterSpeedSetting => {
            "Master control for variable shutter speed functionality. When enabled, allows fine adjustments to counteract light source flickering."
        }
        DevicePropertyCode::ElectricFrontCurtainShutter => {
            "Uses electronic sensor control instead of mechanical movement for the front shutter curtain. Reduces shutter shock and noise."
        }
        DevicePropertyCode::ApertureDriveInAF => {
            "Controls aperture behavior during autofocus. Standard mode adjusts for focus speed, Focus Priority prioritizes tracking, Silent Priority minimizes noise."
        }
        DevicePropertyCode::SilentModeApertureDriveInAF => {
            "Aperture drive behavior when Silent Mode is active. Balances silent operation with focusing performance."
        }
        DevicePropertyCode::AmountOfDefocusSetting => {
            "Controls intensity of background defocus (bokeh). Adjusts blur applied to out-of-focus areas for artistic background separation."
        }
        DevicePropertyCode::ColorSpace => {
            "Selects color gamut for image encoding. sRGB for standard use and web, Adobe RGB for print with wider color reproduction."
        }
        DevicePropertyCode::EmbedLUTFile => {
            "Embeds a LUT file into video recordings for color grading reference. Ensures consistent color when imported into editing software."
        }
        DevicePropertyCode::EstimatePictureSize => {
            "Displays estimated file size before capture based on current settings. Helps monitor remaining card space."
        }
        DevicePropertyCode::SilentModeShutterWhenPowerOff => {
            "Controls shutter blade behavior when powering off in silent mode. Close keeps sensor protected from dust."
        }
        DevicePropertyCode::AntidustShutterWhenPowerOff => {
            "Closes shutter when powering off to protect sensor from dust. Keeps sensor clean during lens changes."
        }
        DevicePropertyCode::PushAutoIris => {
            "Temporarily engages auto iris while button is pressed. Useful for quick exposure checks."
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
        DevicePropertyCode::ExposureStep => "EV Step Size",
        DevicePropertyCode::AutoSlowShutter => "Auto Slow Shutter",
        DevicePropertyCode::ShutterModeSetting => "Shutter Control",
        DevicePropertyCode::ShutterModeStatus => "Shutter Mode",
        DevicePropertyCode::ShutterMode => "Shutter Unit",
        DevicePropertyCode::ShutterAngle => "Shutter Angle (°)",
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
        DevicePropertyCode::IrisDisplayUnit => "Iris Unit (F/T)",
        DevicePropertyCode::IrisCloseSetting => "Iris Close Enable",
        DevicePropertyCode::BaseISOSwitchEI => "Base ISO (EI)",
        DevicePropertyCode::DRO => "D-Range Optimizer",
        DevicePropertyCode::MeteringMode => "Metering Mode",
        DevicePropertyCode::MeteredManualLevel => "Meter Level (M)",
        DevicePropertyCode::FacePriorityInMultiMetering => "Face Priority Metering",
        DevicePropertyCode::BulbExposureTimeSetting => "Bulb Exposure Time",
        DevicePropertyCode::ExtendedShutterSpeed => "Extended SS",
        DevicePropertyCode::HighIsoNR => "High ISO NR",
        DevicePropertyCode::LongExposureNR => "Long Exp NR",
        DevicePropertyCode::IsoAutoMinShutterSpeedMode => "ISO Auto Min SS Mode",
        DevicePropertyCode::IsoAutoMinShutterSpeedManual => "ISO Auto Min SS (Manual)",
        DevicePropertyCode::IsoAutoMinShutterSpeedPreset => "ISO Auto Min SS (Preset)",
        DevicePropertyCode::IsoAutoRangeLimitMin => "ISO Auto Range Min",
        DevicePropertyCode::IsoAutoRangeLimitMax => "ISO Auto Range Max",
        DevicePropertyCode::IsoCurrentSensitivity => "ISO (Current)",
        DevicePropertyCode::ShutterSpeedValue => "SS Value",
        DevicePropertyCode::ShutterSpeedCurrentValue => "SS (Current)",
        DevicePropertyCode::ShutterType => "Shutter (Mech/Elec)",
        DevicePropertyCode::ShutterSelectMode => "Shutter Selection",
        DevicePropertyCode::ShutterReleaseTimeLagControl => "Shutter Release Lag",
        DevicePropertyCode::IntervalRecShutterType => "Interval Shutter",
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterHi => "Cont. Hi FPS (E)",
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterHiPlus => "Cont. Hi+ FPS (E)",
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterMid => "Cont. Mid FPS (E)",
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterLo => "Cont. Lo FPS (E)",
        _ => code.name(),
    }
}

pub fn value_type(code: DevicePropertyCode) -> Option<PropertyValueType> {
    use PropertyValueType as V;

    Some(match code {
        DevicePropertyCode::FNumber => V::Aperture,
        DevicePropertyCode::ShutterSpeed | DevicePropertyCode::ShutterSpeedCurrentValue => {
            V::ShutterSpeed
        }
        DevicePropertyCode::IsoSensitivity => V::Iso,
        DevicePropertyCode::ExposureBiasCompensation => V::ExposureCompensation,
        DevicePropertyCode::ExposureProgramMode => V::ExposureProgram,
        DevicePropertyCode::MeteringMode => V::MeteringMode,
        DevicePropertyCode::ShutterModeStatus => V::ShutterModeStatus,
        DevicePropertyCode::ShutterMode => V::ShutterMode,
        DevicePropertyCode::ExposureCtrlType => V::ExposureCtrlType,
        DevicePropertyCode::AutoSlowShutter
        | DevicePropertyCode::ShutterSetting
        | DevicePropertyCode::ShutterECSSetting
        | DevicePropertyCode::ShutterSlow => V::Switch,
        DevicePropertyCode::IrisModeSetting
        | DevicePropertyCode::ShutterModeSetting
        | DevicePropertyCode::GainControlSetting => V::AutoManual,
        DevicePropertyCode::ShutterSpeedValue => V::ShutterSpeed,
        DevicePropertyCode::IsoCurrentSensitivity
        | DevicePropertyCode::IsoAutoRangeLimitMin
        | DevicePropertyCode::IsoAutoRangeLimitMax
        | DevicePropertyCode::ExposureIndex
        | DevicePropertyCode::GainBaseIsoSensitivity
        | DevicePropertyCode::BaseISOSwitchEI => V::Iso,
        DevicePropertyCode::ShutterAngle => V::Integer,
        DevicePropertyCode::ShutterSlowFrames
        | DevicePropertyCode::ShutterECSFrequency
        | DevicePropertyCode::ShutterECSNumber
        | DevicePropertyCode::ShutterECSNumberStep
        | DevicePropertyCode::BulbExposureTimeSetting
        | DevicePropertyCode::MeteredManualLevel
        | DevicePropertyCode::GaindBValue
        | DevicePropertyCode::ExposureStep => V::Integer,
        DevicePropertyCode::HighIsoNR
        | DevicePropertyCode::LongExposureNR
        | DevicePropertyCode::ExtendedShutterSpeed
        | DevicePropertyCode::FacePriorityInMultiMetering
        | DevicePropertyCode::DRO => V::Switch,
        DevicePropertyCode::IsoAutoMinShutterSpeedMode
        | DevicePropertyCode::ShutterSelectMode
        | DevicePropertyCode::ShutterType
        | DevicePropertyCode::ShutterReleaseTimeLagControl
        | DevicePropertyCode::IrisDisplayUnit
        | DevicePropertyCode::GainUnitSetting
        | DevicePropertyCode::GainBaseSensitivity
        | DevicePropertyCode::IrisCloseSetting => V::Integer,
        DevicePropertyCode::IsoAutoMinShutterSpeedManual
        | DevicePropertyCode::IsoAutoMinShutterSpeedPreset => V::ShutterSpeed,
        DevicePropertyCode::IntervalRecShutterType => V::IntervalRecShutterType,
        DevicePropertyCode::GaindBCurrentValue => V::Integer,
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterHi
        | DevicePropertyCode::ContinuousShootingSpeedInElectricShutterHiPlus
        | DevicePropertyCode::ContinuousShootingSpeedInElectricShutterMid
        | DevicePropertyCode::ContinuousShootingSpeedInElectricShutterLo => V::Integer,
        _ => return None,
    })
}
