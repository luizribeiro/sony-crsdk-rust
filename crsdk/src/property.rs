//! Camera property types and codes

/// Property codes for camera settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum PropertyCode {
    /// F-number (aperture)
    FNumber = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_FNumber,
    /// ISO sensitivity
    IsoSensitivity = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_IsoSensitivity,
    /// Shutter speed
    ShutterSpeed = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_ShutterSpeed,
    /// Exposure bias compensation
    ExposureBias =
        crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_ExposureBiasCompensation,
    /// White balance mode
    WhiteBalance = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_WhiteBalance,
    /// Focus mode
    FocusMode = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_FocusMode,
    /// Focus area
    FocusArea = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_FocusArea,
    /// Exposure program mode (M, P, A, S, etc.)
    ExposureProgram = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_ExposureProgramMode,
    /// Drive mode (single, continuous, timer, bracket)
    DriveMode = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_DriveMode,
    /// Flash mode
    FlashMode = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_FlashMode,
    /// Metering mode
    MeteringMode = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_MeteringMode,
    /// Color temperature (Kelvin)
    ColorTemperature = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_Colortemp,
    /// Image aspect ratio
    AspectRatio = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_AspectRatio,
    /// Still image size
    ImageSize = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_ImageSize,
    /// Still image quality (JPEG compression level)
    ImageQuality = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_StillImageQuality,
    /// Still image file type (RAW/JPEG/RAW+JPEG/HEIF)
    FileType = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_FileType,
    /// Movie file format
    MovieFormat = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_Movie_File_Format,
    /// Movie recording setting
    MovieRecordingSetting =
        crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_Movie_Recording_Setting,
    /// Zoom scale
    ZoomScale = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_Zoom_Scale,

    // Camera info properties
    /// Battery remaining percentage (0-100)
    BatteryRemain = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_BatteryRemain,
    /// Battery level indicator
    BatteryLevel = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_BatteryLevel,
    /// Lens model name
    LensModelName = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_LensModelName,

    // Media slot 1
    /// Media slot 1 status
    MediaSlot1Status = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_MediaSLOT1_Status,
    /// Media slot 1 remaining photos
    MediaSlot1RemainingPhotos =
        crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_MediaSLOT1_RemainingNumber,
    /// Media slot 1 remaining recording time (seconds)
    MediaSlot1RemainingTime =
        crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_MediaSLOT1_RemainingTime,

    // Media slot 2
    /// Media slot 2 status
    MediaSlot2Status = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_MediaSLOT2_Status,
    /// Media slot 2 remaining photos
    MediaSlot2RemainingPhotos =
        crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_MediaSLOT2_RemainingNumber,
    /// Media slot 2 remaining recording time (seconds)
    MediaSlot2RemainingTime =
        crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_MediaSLOT2_RemainingTime,

    // Media slot 3 (some cameras like FX6, FX9 have 3 slots)
    /// Media slot 3 status
    MediaSlot3Status = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_MediaSLOT3_Status,
    /// Media slot 3 remaining recording time (seconds)
    MediaSlot3RemainingTime =
        crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_MediaSLOT3_RemainingTime,

    // Lens and focus info
    /// Focal distance in meters (value in mm, 0xFFFFFFFF = infinity)
    FocalDistanceInMeter =
        crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_FocalDistanceInMeter,
    /// Zoom distance (current focal length)
    ZoomDistance = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_ZoomDistance,

    // Device status
    /// Device overheating state (0=normal, 1=pre-overheating, 2=overheating)
    DeviceOverheatingState =
        crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_DeviceOverheatingState,
}

impl PropertyCode {
    /// Get the raw SDK property code value
    pub fn as_raw(self) -> u32 {
        self as u32
    }

    /// Create from raw SDK property code
    pub fn from_raw(code: u32) -> Option<Self> {
        use crsdk_sys::SCRSDK::*;
        Some(match code {
            CrDevicePropertyCode_CrDeviceProperty_FNumber => Self::FNumber,
            CrDevicePropertyCode_CrDeviceProperty_IsoSensitivity => Self::IsoSensitivity,
            CrDevicePropertyCode_CrDeviceProperty_ShutterSpeed => Self::ShutterSpeed,
            CrDevicePropertyCode_CrDeviceProperty_ExposureBiasCompensation => Self::ExposureBias,
            CrDevicePropertyCode_CrDeviceProperty_WhiteBalance => Self::WhiteBalance,
            CrDevicePropertyCode_CrDeviceProperty_FocusMode => Self::FocusMode,
            CrDevicePropertyCode_CrDeviceProperty_FocusArea => Self::FocusArea,
            CrDevicePropertyCode_CrDeviceProperty_ExposureProgramMode => Self::ExposureProgram,
            CrDevicePropertyCode_CrDeviceProperty_DriveMode => Self::DriveMode,
            CrDevicePropertyCode_CrDeviceProperty_FlashMode => Self::FlashMode,
            CrDevicePropertyCode_CrDeviceProperty_MeteringMode => Self::MeteringMode,
            CrDevicePropertyCode_CrDeviceProperty_Colortemp => Self::ColorTemperature,
            CrDevicePropertyCode_CrDeviceProperty_AspectRatio => Self::AspectRatio,
            CrDevicePropertyCode_CrDeviceProperty_ImageSize => Self::ImageSize,
            CrDevicePropertyCode_CrDeviceProperty_StillImageQuality => Self::ImageQuality,
            CrDevicePropertyCode_CrDeviceProperty_FileType => Self::FileType,
            CrDevicePropertyCode_CrDeviceProperty_Movie_File_Format => Self::MovieFormat,
            CrDevicePropertyCode_CrDeviceProperty_Movie_Recording_Setting => {
                Self::MovieRecordingSetting
            }
            CrDevicePropertyCode_CrDeviceProperty_Zoom_Scale => Self::ZoomScale,
            CrDevicePropertyCode_CrDeviceProperty_BatteryRemain => Self::BatteryRemain,
            CrDevicePropertyCode_CrDeviceProperty_BatteryLevel => Self::BatteryLevel,
            CrDevicePropertyCode_CrDeviceProperty_LensModelName => Self::LensModelName,
            CrDevicePropertyCode_CrDeviceProperty_MediaSLOT1_Status => Self::MediaSlot1Status,
            CrDevicePropertyCode_CrDeviceProperty_MediaSLOT1_RemainingNumber => {
                Self::MediaSlot1RemainingPhotos
            }
            CrDevicePropertyCode_CrDeviceProperty_MediaSLOT1_RemainingTime => {
                Self::MediaSlot1RemainingTime
            }
            CrDevicePropertyCode_CrDeviceProperty_MediaSLOT2_Status => Self::MediaSlot2Status,
            CrDevicePropertyCode_CrDeviceProperty_MediaSLOT2_RemainingNumber => {
                Self::MediaSlot2RemainingPhotos
            }
            CrDevicePropertyCode_CrDeviceProperty_MediaSLOT2_RemainingTime => {
                Self::MediaSlot2RemainingTime
            }
            CrDevicePropertyCode_CrDeviceProperty_MediaSLOT3_Status => Self::MediaSlot3Status,
            CrDevicePropertyCode_CrDeviceProperty_MediaSLOT3_RemainingTime => {
                Self::MediaSlot3RemainingTime
            }
            CrDevicePropertyCode_CrDeviceProperty_FocalDistanceInMeter => {
                Self::FocalDistanceInMeter
            }
            CrDevicePropertyCode_CrDeviceProperty_ZoomDistance => Self::ZoomDistance,
            CrDevicePropertyCode_CrDeviceProperty_DeviceOverheatingState => {
                Self::DeviceOverheatingState
            }
            _ => return None,
        })
    }
}

/// Focus mode settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FocusMode {
    /// Manual focus
    Manual = crsdk_sys::SCRSDK::CrFocusMode_CrFocus_MF as u16,
    /// Single-shot AF
    AfSingle = crsdk_sys::SCRSDK::CrFocusMode_CrFocus_AF_S as u16,
    /// Continuous AF
    AfContinuous = crsdk_sys::SCRSDK::CrFocusMode_CrFocus_AF_C as u16,
    /// Automatic AF (camera chooses AF-S or AF-C)
    AfAutomatic = crsdk_sys::SCRSDK::CrFocusMode_CrFocus_AF_A as u16,
    /// Deep learning AF
    AfDeepLearning = crsdk_sys::SCRSDK::CrFocusMode_CrFocus_AF_D as u16,
    /// Direct manual focus (AF then manual override)
    DirectManual = crsdk_sys::SCRSDK::CrFocusMode_CrFocus_DMF as u16,
    /// Pre-set focus
    PresetFocus = crsdk_sys::SCRSDK::CrFocusMode_CrFocus_PF as u16,
}

impl FocusMode {
    /// Get the raw SDK value
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    /// Create from raw SDK value
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

/// White balance settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum WhiteBalance {
    /// Auto white balance
    Auto = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_AWB as u16,
    /// Daylight
    Daylight = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Daylight as u16,
    /// Shade
    Shade = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Shadow as u16,
    /// Cloudy
    Cloudy = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Cloudy as u16,
    /// Tungsten / Incandescent
    Tungsten = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Tungsten as u16,
    /// Fluorescent (generic)
    Fluorescent = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent as u16,
    /// Fluorescent - Warm White
    FluorescentWarmWhite =
        crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_WarmWhite as u16,
    /// Fluorescent - Cool White
    FluorescentCoolWhite =
        crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_CoolWhite as u16,
    /// Fluorescent - Day White
    FluorescentDayWhite =
        crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_DayWhite as u16,
    /// Fluorescent - Daylight
    FluorescentDaylight =
        crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Fluorescent_Daylight as u16,
    /// Flash
    Flash = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Flush as u16,
    /// Underwater auto
    UnderwaterAuto = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Underwater_Auto as u16,
    /// Color temperature (use with ColorTemperature property)
    ColorTemp = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_ColorTemp as u16,
    /// Custom 1
    Custom1 = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Custom_1 as u16,
    /// Custom 2
    Custom2 = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Custom_2 as u16,
    /// Custom 3
    Custom3 = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Custom_3 as u16,
    /// Custom (generic)
    Custom = crsdk_sys::SCRSDK::CrWhiteBalanceSetting_CrWhiteBalance_Custom as u16,
}

impl WhiteBalance {
    /// Get the raw SDK value
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    /// Create from raw SDK value
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

/// Exposure program mode (shooting mode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ExposureProgram {
    /// Manual exposure (M mode)
    Manual = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_M_Manual,
    /// Program auto (P mode)
    ProgramAuto = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_P_Auto,
    /// Aperture priority (A mode)
    AperturePriority = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_A_AperturePriority,
    /// Shutter speed priority (S mode)
    ShutterPriority = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_S_ShutterSpeedPriority,
    /// Program Creative
    ProgramCreative = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Program_Creative,
    /// Program Action
    ProgramAction = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Program_Action,
    /// Portrait scene mode
    Portrait = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Portrait,
    /// Intelligent auto
    Auto = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Auto,
    /// Intelligent auto plus
    AutoPlus = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Auto_Plus,
    /// P mode variant A
    PA = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_P_A,
    /// P mode variant S
    PS = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_P_S,
    /// Sports action scene mode
    SportsAction = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Sports_Action,
    /// Sunset scene mode
    Sunset = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Sunset,
    /// Night scene mode
    Night = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Night,
    /// Landscape scene mode
    Landscape = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Landscape,
    /// Macro scene mode
    Macro = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Macro,
    /// Handheld twilight scene mode
    HandheldTwilight = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_HandheldTwilight,
    /// Night portrait scene mode
    NightPortrait = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_NightPortrait,
    /// Anti motion blur scene mode
    AntiMotionBlur = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_AntiMotionBlur,
    /// Pet scene mode
    Pet = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Pet,
    /// Gourmet scene mode
    Gourmet = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Gourmet,
    /// Fireworks scene mode
    Fireworks = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Fireworks,
    /// High sensitivity scene mode
    HighSensitivity = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_HighSensitivity,
    /// Memory recall mode
    MemoryRecall = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_MemoryRecall,
    /// Continuous priority AE 8 pictures
    ContinuousPriorityAE8 =
        crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_ContinuousPriority_AE_8pics,
    /// Continuous priority AE 10 pictures
    ContinuousPriorityAE10 =
        crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_ContinuousPriority_AE_10pics,
    /// Continuous priority AE 12 pictures
    ContinuousPriorityAE12 =
        crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_ContinuousPriority_AE_12pics,
    /// 3D sweep panorama mode
    SweepPanorama3D = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_3D_SweepPanorama,
    /// Sweep panorama mode
    SweepPanorama = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_SweepPanorama,
    /// Movie program auto (P mode)
    MovieP = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_P,
    /// Movie aperture priority (A mode)
    MovieA = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_A,
    /// Movie shutter priority (S mode)
    MovieS = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_S,
    /// Movie manual (M mode)
    MovieM = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_M,
    /// Movie auto mode
    MovieAuto = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_Auto,
    /// Movie flexible exposure (F mode)
    MovieF = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_F,
    /// Movie S&Q Motion program auto (P mode)
    MovieSQMotionP = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_SQMotion_P,
    /// Movie S&Q Motion aperture priority (A mode)
    MovieSQMotionA = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_SQMotion_A,
    /// Movie S&Q Motion shutter priority (S mode)
    MovieSQMotionS = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_SQMotion_S,
    /// Movie S&Q Motion manual (M mode)
    MovieSQMotionM = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_SQMotion_M,
    /// Movie S&Q Motion auto
    MovieSQMotionAuto = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_SQMotion_AUTO,
    /// Movie S&Q Motion flexible exposure (F mode)
    MovieSQMotionF = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_SQMotion_F,
    /// Flash off mode
    FlashOff = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Flash_Off,
    /// Picture effect mode
    PictureEffect = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_PictureEffect,
    /// High frame rate program auto (P mode)
    HiFrameRateP = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_HiFrameRate_P,
    /// High frame rate aperture priority (A mode)
    HiFrameRateA = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_HiFrameRate_A,
    /// High frame rate shutter priority (S mode)
    HiFrameRateS = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_HiFrameRate_S,
    /// High frame rate manual (M mode)
    HiFrameRateM = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_HiFrameRate_M,
    /// S&Q Motion program auto (P mode)
    SQMotionP = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_SQMotion_P,
    /// S&Q Motion aperture priority (A mode)
    SQMotionA = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_SQMotion_A,
    /// S&Q Motion shutter priority (S mode)
    SQMotionS = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_SQMotion_S,
    /// S&Q Motion manual (M mode)
    SQMotionM = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_SQMotion_M,
    /// Movie mode
    Movie = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_MOVIE,
    /// Still mode
    Still = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_STILL,
    /// Movie flexible exposure mode (F mode)
    MovieFMode = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_F_Mode,
    /// Movie interval recording flexible exposure (F mode)
    MovieIntervalRecF = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_IntervalRec_F,
    /// Movie interval recording program auto (P mode)
    MovieIntervalRecP = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_IntervalRec_P,
    /// Movie interval recording aperture priority (A mode)
    MovieIntervalRecA = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_IntervalRec_A,
    /// Movie interval recording shutter priority (S mode)
    MovieIntervalRecS = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_IntervalRec_S,
    /// Movie interval recording manual (M mode)
    MovieIntervalRecM = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_IntervalRec_M,
    /// Movie interval recording auto
    MovieIntervalRecAuto = crsdk_sys::SCRSDK::CrExposureProgram_CrExposure_Movie_IntervalRec_AUTO,
}

impl ExposureProgram {
    /// Get the raw SDK value
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    /// Create from raw SDK value
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

/// Drive mode / shooting mode settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DriveMode {
    /// Single shooting
    Single = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single,
    /// Continuous shooting high speed
    ContinuousHi = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Hi,
    /// Continuous shooting high speed plus
    ContinuousHiPlus = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Hi_Plus,
    /// Continuous shooting high speed live view
    ContinuousHiLive = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Hi_Live,
    /// Continuous shooting low speed
    ContinuousLo = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Lo,
    /// Continuous shooting
    Continuous = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous,
    /// Continuous shooting speed priority
    ContinuousSpeedPriority = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_SpeedPriority,
    /// Continuous shooting mid speed
    ContinuousMid = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Mid,
    /// Continuous shooting mid speed live view
    ContinuousMidLive = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Mid_Live,
    /// Continuous shooting low speed live view
    ContinuousLoLive = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Lo_Live,
    /// Single burst shooting low
    SingleBurstShootingLo = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_SingleBurstShooting_lo,
    /// Single burst shooting mid
    SingleBurstShootingMid = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_SingleBurstShooting_mid,
    /// Single burst shooting high
    SingleBurstShootingHi = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_SingleBurstShooting_hi,
    /// Focus bracket shooting
    FocusBracket = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_FocusBracket,
    /// Timelapse shooting
    Timelapse = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Timelapse,
    /// Self-timer 2 seconds
    Timer2s = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Timer_2s,
    /// Self-timer 5 seconds
    Timer5s = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Timer_5s,
    /// Self-timer 10 seconds
    Timer10s = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Timer_10s,
    /// Continuous bracket 0.3 EV, 3 pictures
    ContinuousBracket03Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_3pics,
    /// Continuous bracket 0.3 EV, 5 pictures
    ContinuousBracket03Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_5pics,
    /// Continuous bracket 0.3 EV, 9 pictures
    ContinuousBracket03Ev9Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_9pics,
    /// Continuous bracket 0.5 EV, 3 pictures
    ContinuousBracket05Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_3pics,
    /// Continuous bracket 0.5 EV, 5 pictures
    ContinuousBracket05Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_5pics,
    /// Continuous bracket 0.5 EV, 9 pictures
    ContinuousBracket05Ev9Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_9pics,
    /// Continuous bracket 0.7 EV, 3 pictures
    ContinuousBracket07Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_3pics,
    /// Continuous bracket 0.7 EV, 5 pictures
    ContinuousBracket07Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_5pics,
    /// Continuous bracket 0.7 EV, 9 pictures
    ContinuousBracket07Ev9Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_9pics,
    /// Continuous bracket 1.0 EV, 3 pictures
    ContinuousBracket10Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_3pics,
    /// Continuous bracket 1.0 EV, 5 pictures
    ContinuousBracket10Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_5pics,
    /// Continuous bracket 1.0 EV, 9 pictures
    ContinuousBracket10Ev9Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_9pics,
    /// Continuous bracket 2.0 EV, 3 pictures
    ContinuousBracket20Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_20Ev_3pics,
    /// Continuous bracket 2.0 EV, 5 pictures
    ContinuousBracket20Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_20Ev_5pics,
    /// Continuous bracket 3.0 EV, 3 pictures
    ContinuousBracket30Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_30Ev_3pics,
    /// Continuous bracket 3.0 EV, 5 pictures
    ContinuousBracket30Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_30Ev_5pics,
    /// Continuous bracket 0.3 EV, 2 pictures plus
    ContinuousBracket03Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_2pics_Plus,
    /// Continuous bracket 0.3 EV, 2 pictures minus
    ContinuousBracket03Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_2pics_Minus,
    /// Continuous bracket 0.3 EV, 7 pictures
    ContinuousBracket03Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_03Ev_7pics,
    /// Continuous bracket 0.5 EV, 2 pictures plus
    ContinuousBracket05Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_2pics_Plus,
    /// Continuous bracket 0.5 EV, 2 pictures minus
    ContinuousBracket05Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_2pics_Minus,
    /// Continuous bracket 0.5 EV, 7 pictures
    ContinuousBracket05Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_05Ev_7pics,
    /// Continuous bracket 0.7 EV, 2 pictures plus
    ContinuousBracket07Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_2pics_Plus,
    /// Continuous bracket 0.7 EV, 2 pictures minus
    ContinuousBracket07Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_2pics_Minus,
    /// Continuous bracket 0.7 EV, 7 pictures
    ContinuousBracket07Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_07Ev_7pics,
    /// Continuous bracket 1.0 EV, 2 pictures plus
    ContinuousBracket10Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_2pics_Plus,
    /// Continuous bracket 1.0 EV, 2 pictures minus
    ContinuousBracket10Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_2pics_Minus,
    /// Continuous bracket 1.0 EV, 7 pictures
    ContinuousBracket10Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_10Ev_7pics,
    /// Continuous bracket 1.3 EV, 2 pictures plus
    ContinuousBracket13Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_13Ev_2pics_Plus,
    /// Continuous bracket 1.3 EV, 2 pictures minus
    ContinuousBracket13Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_13Ev_2pics_Minus,
    /// Continuous bracket 1.3 EV, 3 pictures
    ContinuousBracket13Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_13Ev_3pics,
    /// Continuous bracket 1.3 EV, 5 pictures
    ContinuousBracket13Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_13Ev_5pics,
    /// Continuous bracket 1.3 EV, 7 pictures
    ContinuousBracket13Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_13Ev_7pics,
    /// Continuous bracket 1.5 EV, 2 pictures plus
    ContinuousBracket15Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_15Ev_2pics_Plus,
    /// Continuous bracket 1.5 EV, 2 pictures minus
    ContinuousBracket15Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_15Ev_2pics_Minus,
    /// Continuous bracket 1.5 EV, 3 pictures
    ContinuousBracket15Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_15Ev_3pics,
    /// Continuous bracket 1.5 EV, 5 pictures
    ContinuousBracket15Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_15Ev_5pics,
    /// Continuous bracket 1.5 EV, 7 pictures
    ContinuousBracket15Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_15Ev_7pics,
    /// Continuous bracket 1.7 EV, 2 pictures plus
    ContinuousBracket17Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_17Ev_2pics_Plus,
    /// Continuous bracket 1.7 EV, 2 pictures minus
    ContinuousBracket17Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_17Ev_2pics_Minus,
    /// Continuous bracket 1.7 EV, 3 pictures
    ContinuousBracket17Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_17Ev_3pics,
    /// Continuous bracket 1.7 EV, 5 pictures
    ContinuousBracket17Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_17Ev_5pics,
    /// Continuous bracket 1.7 EV, 7 pictures
    ContinuousBracket17Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_17Ev_7pics,
    /// Continuous bracket 2.0 EV, 2 pictures plus
    ContinuousBracket20Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_20Ev_2pics_Plus,
    /// Continuous bracket 2.0 EV, 2 pictures minus
    ContinuousBracket20Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_20Ev_2pics_Minus,
    /// Continuous bracket 2.0 EV, 7 pictures
    ContinuousBracket20Ev7Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_20Ev_7pics,
    /// Continuous bracket 2.3 EV, 2 pictures plus
    ContinuousBracket23Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_23Ev_2pics_Plus,
    /// Continuous bracket 2.3 EV, 2 pictures minus
    ContinuousBracket23Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_23Ev_2pics_Minus,
    /// Continuous bracket 2.3 EV, 3 pictures
    ContinuousBracket23Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_23Ev_3pics,
    /// Continuous bracket 2.3 EV, 5 pictures
    ContinuousBracket23Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_23Ev_5pics,
    /// Continuous bracket 2.5 EV, 2 pictures plus
    ContinuousBracket25Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_25Ev_2pics_Plus,
    /// Continuous bracket 2.5 EV, 2 pictures minus
    ContinuousBracket25Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_25Ev_2pics_Minus,
    /// Continuous bracket 2.5 EV, 3 pictures
    ContinuousBracket25Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_25Ev_3pics,
    /// Continuous bracket 2.5 EV, 5 pictures
    ContinuousBracket25Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_25Ev_5pics,
    /// Continuous bracket 2.7 EV, 2 pictures plus
    ContinuousBracket27Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_27Ev_2pics_Plus,
    /// Continuous bracket 2.7 EV, 2 pictures minus
    ContinuousBracket27Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_27Ev_2pics_Minus,
    /// Continuous bracket 2.7 EV, 3 pictures
    ContinuousBracket27Ev3Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_27Ev_3pics,
    /// Continuous bracket 2.7 EV, 5 pictures
    ContinuousBracket27Ev5Pics =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_27Ev_5pics,
    /// Continuous bracket 3.0 EV, 2 pictures plus
    ContinuousBracket30Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_30Ev_2pics_Plus,
    /// Continuous bracket 3.0 EV, 2 pictures minus
    ContinuousBracket30Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Bracket_30Ev_2pics_Minus,
    /// Single bracket 0.3 EV, 3 pictures
    SingleBracket03Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_3pics,
    /// Single bracket 0.3 EV, 5 pictures
    SingleBracket03Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_5pics,
    /// Single bracket 0.3 EV, 9 pictures
    SingleBracket03Ev9Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_9pics,
    /// Single bracket 0.5 EV, 3 pictures
    SingleBracket05Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_3pics,
    /// Single bracket 0.5 EV, 5 pictures
    SingleBracket05Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_5pics,
    /// Single bracket 0.5 EV, 9 pictures
    SingleBracket05Ev9Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_9pics,
    /// Single bracket 0.7 EV, 3 pictures
    SingleBracket07Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_3pics,
    /// Single bracket 0.7 EV, 5 pictures
    SingleBracket07Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_5pics,
    /// Single bracket 0.7 EV, 9 pictures
    SingleBracket07Ev9Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_9pics,
    /// Single bracket 1.0 EV, 3 pictures
    SingleBracket10Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_3pics,
    /// Single bracket 1.0 EV, 5 pictures
    SingleBracket10Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_5pics,
    /// Single bracket 1.0 EV, 9 pictures
    SingleBracket10Ev9Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_9pics,
    /// Single bracket 2.0 EV, 3 pictures
    SingleBracket20Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_20Ev_3pics,
    /// Single bracket 2.0 EV, 5 pictures
    SingleBracket20Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_20Ev_5pics,
    /// Single bracket 3.0 EV, 3 pictures
    SingleBracket30Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_30Ev_3pics,
    /// Single bracket 3.0 EV, 5 pictures
    SingleBracket30Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_30Ev_5pics,
    /// Single bracket 0.3 EV, 2 pictures plus
    SingleBracket03Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_2pics_Plus,
    /// Single bracket 0.3 EV, 2 pictures minus
    SingleBracket03Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_2pics_Minus,
    /// Single bracket 0.3 EV, 7 pictures
    SingleBracket03Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_03Ev_7pics,
    /// Single bracket 0.5 EV, 2 pictures plus
    SingleBracket05Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_2pics_Plus,
    /// Single bracket 0.5 EV, 2 pictures minus
    SingleBracket05Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_2pics_Minus,
    /// Single bracket 0.5 EV, 7 pictures
    SingleBracket05Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_05Ev_7pics,
    /// Single bracket 0.7 EV, 2 pictures plus
    SingleBracket07Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_2pics_Plus,
    /// Single bracket 0.7 EV, 2 pictures minus
    SingleBracket07Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_2pics_Minus,
    /// Single bracket 0.7 EV, 7 pictures
    SingleBracket07Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_07Ev_7pics,
    /// Single bracket 1.0 EV, 2 pictures plus
    SingleBracket10Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_2pics_Plus,
    /// Single bracket 1.0 EV, 2 pictures minus
    SingleBracket10Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_2pics_Minus,
    /// Single bracket 1.0 EV, 7 pictures
    SingleBracket10Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_10Ev_7pics,
    /// Single bracket 1.3 EV, 2 pictures plus
    SingleBracket13Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_13Ev_2pics_Plus,
    /// Single bracket 1.3 EV, 2 pictures minus
    SingleBracket13Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_13Ev_2pics_Minus,
    /// Single bracket 1.3 EV, 3 pictures
    SingleBracket13Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_13Ev_3pics,
    /// Single bracket 1.3 EV, 5 pictures
    SingleBracket13Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_13Ev_5pics,
    /// Single bracket 1.3 EV, 7 pictures
    SingleBracket13Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_13Ev_7pics,
    /// Single bracket 1.5 EV, 2 pictures plus
    SingleBracket15Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_15Ev_2pics_Plus,
    /// Single bracket 1.5 EV, 2 pictures minus
    SingleBracket15Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_15Ev_2pics_Minus,
    /// Single bracket 1.5 EV, 3 pictures
    SingleBracket15Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_15Ev_3pics,
    /// Single bracket 1.5 EV, 5 pictures
    SingleBracket15Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_15Ev_5pics,
    /// Single bracket 1.5 EV, 7 pictures
    SingleBracket15Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_15Ev_7pics,
    /// Single bracket 1.7 EV, 2 pictures plus
    SingleBracket17Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_17Ev_2pics_Plus,
    /// Single bracket 1.7 EV, 2 pictures minus
    SingleBracket17Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_17Ev_2pics_Minus,
    /// Single bracket 1.7 EV, 3 pictures
    SingleBracket17Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_17Ev_3pics,
    /// Single bracket 1.7 EV, 5 pictures
    SingleBracket17Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_17Ev_5pics,
    /// Single bracket 1.7 EV, 7 pictures
    SingleBracket17Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_17Ev_7pics,
    /// Single bracket 2.0 EV, 2 pictures plus
    SingleBracket20Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_20Ev_2pics_Plus,
    /// Single bracket 2.0 EV, 2 pictures minus
    SingleBracket20Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_20Ev_2pics_Minus,
    /// Single bracket 2.0 EV, 7 pictures
    SingleBracket20Ev7Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_20Ev_7pics,
    /// Single bracket 2.3 EV, 2 pictures plus
    SingleBracket23Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_23Ev_2pics_Plus,
    /// Single bracket 2.3 EV, 2 pictures minus
    SingleBracket23Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_23Ev_2pics_Minus,
    /// Single bracket 2.3 EV, 3 pictures
    SingleBracket23Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_23Ev_3pics,
    /// Single bracket 2.3 EV, 5 pictures
    SingleBracket23Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_23Ev_5pics,
    /// Single bracket 2.5 EV, 2 pictures plus
    SingleBracket25Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_25Ev_2pics_Plus,
    /// Single bracket 2.5 EV, 2 pictures minus
    SingleBracket25Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_25Ev_2pics_Minus,
    /// Single bracket 2.5 EV, 3 pictures
    SingleBracket25Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_25Ev_3pics,
    /// Single bracket 2.5 EV, 5 pictures
    SingleBracket25Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_25Ev_5pics,
    /// Single bracket 2.7 EV, 2 pictures plus
    SingleBracket27Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_27Ev_2pics_Plus,
    /// Single bracket 2.7 EV, 2 pictures minus
    SingleBracket27Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_27Ev_2pics_Minus,
    /// Single bracket 2.7 EV, 3 pictures
    SingleBracket27Ev3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_27Ev_3pics,
    /// Single bracket 2.7 EV, 5 pictures
    SingleBracket27Ev5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_27Ev_5pics,
    /// Single bracket 3.0 EV, 2 pictures plus
    SingleBracket30Ev2PicsPlus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_30Ev_2pics_Plus,
    /// Single bracket 3.0 EV, 2 pictures minus
    SingleBracket30Ev2PicsMinus =
        crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Single_Bracket_30Ev_2pics_Minus,
    /// White balance bracket low
    WbBracketLo = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_WB_Bracket_Lo,
    /// White balance bracket high
    WbBracketHi = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_WB_Bracket_Hi,
    /// DRO bracket low
    DroBracketLo = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_DRO_Bracket_Lo,
    /// DRO bracket high
    DroBracketHi = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_DRO_Bracket_Hi,
    /// Continuous self-timer 3 pictures
    ContinuousTimer3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_3pics,
    /// Continuous self-timer 5 pictures
    ContinuousTimer5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_5pics,
    /// Continuous self-timer 2 seconds, 3 pictures
    ContinuousTimer2s3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_2s_3pics,
    /// Continuous self-timer 2 seconds, 5 pictures
    ContinuousTimer2s5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_2s_5pics,
    /// Continuous self-timer 5 seconds, 3 pictures
    ContinuousTimer5s3Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_5s_3pics,
    /// Continuous self-timer 5 seconds, 5 pictures
    ContinuousTimer5s5Pics = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_Continuous_Timer_5s_5pics,
    /// Low pass filter bracket
    LpfBracket = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_LPF_Bracket,
    /// Remote commander mode
    RemoteCommander = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_RemoteCommander,
    /// Mirror up mode
    MirrorUp = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_MirrorUp,
    /// Self portrait mode 1
    SelfPortrait1 = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_SelfPortrait_1,
    /// Self portrait mode 2
    SelfPortrait2 = crsdk_sys::SCRSDK::CrDriveMode_CrDrive_SelfPortrait_2,
}

impl DriveMode {
    /// Get the raw SDK value
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    /// Create from raw SDK value
    pub fn from_raw(value: u64) -> Option<Self> {
        use crsdk_sys::SCRSDK::*;
        Some(match value as u32 {
            CrDriveMode_CrDrive_Single => Self::Single,
            CrDriveMode_CrDrive_Continuous_Hi => Self::ContinuousHi,
            CrDriveMode_CrDrive_Continuous_Hi_Plus => Self::ContinuousHiPlus,
            CrDriveMode_CrDrive_Continuous_Hi_Live => Self::ContinuousHiLive,
            CrDriveMode_CrDrive_Continuous_Lo => Self::ContinuousLo,
            CrDriveMode_CrDrive_Continuous => Self::Continuous,
            CrDriveMode_CrDrive_Continuous_SpeedPriority => Self::ContinuousSpeedPriority,
            CrDriveMode_CrDrive_Continuous_Mid => Self::ContinuousMid,
            CrDriveMode_CrDrive_Continuous_Mid_Live => Self::ContinuousMidLive,
            CrDriveMode_CrDrive_Continuous_Lo_Live => Self::ContinuousLoLive,
            CrDriveMode_CrDrive_SingleBurstShooting_lo => Self::SingleBurstShootingLo,
            CrDriveMode_CrDrive_SingleBurstShooting_mid => Self::SingleBurstShootingMid,
            CrDriveMode_CrDrive_SingleBurstShooting_hi => Self::SingleBurstShootingHi,
            CrDriveMode_CrDrive_FocusBracket => Self::FocusBracket,
            CrDriveMode_CrDrive_Timelapse => Self::Timelapse,
            CrDriveMode_CrDrive_Timer_2s => Self::Timer2s,
            CrDriveMode_CrDrive_Timer_5s => Self::Timer5s,
            CrDriveMode_CrDrive_Timer_10s => Self::Timer10s,
            CrDriveMode_CrDrive_Continuous_Bracket_03Ev_3pics => Self::ContinuousBracket03Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_03Ev_5pics => Self::ContinuousBracket03Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_03Ev_9pics => Self::ContinuousBracket03Ev9Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_05Ev_3pics => Self::ContinuousBracket05Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_05Ev_5pics => Self::ContinuousBracket05Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_05Ev_9pics => Self::ContinuousBracket05Ev9Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_07Ev_3pics => Self::ContinuousBracket07Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_07Ev_5pics => Self::ContinuousBracket07Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_07Ev_9pics => Self::ContinuousBracket07Ev9Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_10Ev_3pics => Self::ContinuousBracket10Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_10Ev_5pics => Self::ContinuousBracket10Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_10Ev_9pics => Self::ContinuousBracket10Ev9Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_20Ev_3pics => Self::ContinuousBracket20Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_20Ev_5pics => Self::ContinuousBracket20Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_30Ev_3pics => Self::ContinuousBracket30Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_30Ev_5pics => Self::ContinuousBracket30Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_03Ev_2pics_Plus => {
                Self::ContinuousBracket03Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_03Ev_2pics_Minus => {
                Self::ContinuousBracket03Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_03Ev_7pics => Self::ContinuousBracket03Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_05Ev_2pics_Plus => {
                Self::ContinuousBracket05Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_05Ev_2pics_Minus => {
                Self::ContinuousBracket05Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_05Ev_7pics => Self::ContinuousBracket05Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_07Ev_2pics_Plus => {
                Self::ContinuousBracket07Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_07Ev_2pics_Minus => {
                Self::ContinuousBracket07Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_07Ev_7pics => Self::ContinuousBracket07Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_10Ev_2pics_Plus => {
                Self::ContinuousBracket10Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_10Ev_2pics_Minus => {
                Self::ContinuousBracket10Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_10Ev_7pics => Self::ContinuousBracket10Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_13Ev_2pics_Plus => {
                Self::ContinuousBracket13Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_13Ev_2pics_Minus => {
                Self::ContinuousBracket13Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_13Ev_3pics => Self::ContinuousBracket13Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_13Ev_5pics => Self::ContinuousBracket13Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_13Ev_7pics => Self::ContinuousBracket13Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_15Ev_2pics_Plus => {
                Self::ContinuousBracket15Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_15Ev_2pics_Minus => {
                Self::ContinuousBracket15Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_15Ev_3pics => Self::ContinuousBracket15Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_15Ev_5pics => Self::ContinuousBracket15Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_15Ev_7pics => Self::ContinuousBracket15Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_17Ev_2pics_Plus => {
                Self::ContinuousBracket17Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_17Ev_2pics_Minus => {
                Self::ContinuousBracket17Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_17Ev_3pics => Self::ContinuousBracket17Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_17Ev_5pics => Self::ContinuousBracket17Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_17Ev_7pics => Self::ContinuousBracket17Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_20Ev_2pics_Plus => {
                Self::ContinuousBracket20Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_20Ev_2pics_Minus => {
                Self::ContinuousBracket20Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_20Ev_7pics => Self::ContinuousBracket20Ev7Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_23Ev_2pics_Plus => {
                Self::ContinuousBracket23Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_23Ev_2pics_Minus => {
                Self::ContinuousBracket23Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_23Ev_3pics => Self::ContinuousBracket23Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_23Ev_5pics => Self::ContinuousBracket23Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_25Ev_2pics_Plus => {
                Self::ContinuousBracket25Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_25Ev_2pics_Minus => {
                Self::ContinuousBracket25Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_25Ev_3pics => Self::ContinuousBracket25Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_25Ev_5pics => Self::ContinuousBracket25Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_27Ev_2pics_Plus => {
                Self::ContinuousBracket27Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_27Ev_2pics_Minus => {
                Self::ContinuousBracket27Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_27Ev_3pics => Self::ContinuousBracket27Ev3Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_27Ev_5pics => Self::ContinuousBracket27Ev5Pics,
            CrDriveMode_CrDrive_Continuous_Bracket_30Ev_2pics_Plus => {
                Self::ContinuousBracket30Ev2PicsPlus
            }
            CrDriveMode_CrDrive_Continuous_Bracket_30Ev_2pics_Minus => {
                Self::ContinuousBracket30Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_03Ev_3pics => Self::SingleBracket03Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_03Ev_5pics => Self::SingleBracket03Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_03Ev_9pics => Self::SingleBracket03Ev9Pics,
            CrDriveMode_CrDrive_Single_Bracket_05Ev_3pics => Self::SingleBracket05Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_05Ev_5pics => Self::SingleBracket05Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_05Ev_9pics => Self::SingleBracket05Ev9Pics,
            CrDriveMode_CrDrive_Single_Bracket_07Ev_3pics => Self::SingleBracket07Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_07Ev_5pics => Self::SingleBracket07Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_07Ev_9pics => Self::SingleBracket07Ev9Pics,
            CrDriveMode_CrDrive_Single_Bracket_10Ev_3pics => Self::SingleBracket10Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_10Ev_5pics => Self::SingleBracket10Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_10Ev_9pics => Self::SingleBracket10Ev9Pics,
            CrDriveMode_CrDrive_Single_Bracket_20Ev_3pics => Self::SingleBracket20Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_20Ev_5pics => Self::SingleBracket20Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_30Ev_3pics => Self::SingleBracket30Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_30Ev_5pics => Self::SingleBracket30Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_03Ev_2pics_Plus => Self::SingleBracket03Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_03Ev_2pics_Minus => {
                Self::SingleBracket03Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_03Ev_7pics => Self::SingleBracket03Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_05Ev_2pics_Plus => Self::SingleBracket05Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_05Ev_2pics_Minus => {
                Self::SingleBracket05Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_05Ev_7pics => Self::SingleBracket05Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_07Ev_2pics_Plus => Self::SingleBracket07Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_07Ev_2pics_Minus => {
                Self::SingleBracket07Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_07Ev_7pics => Self::SingleBracket07Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_10Ev_2pics_Plus => Self::SingleBracket10Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_10Ev_2pics_Minus => {
                Self::SingleBracket10Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_10Ev_7pics => Self::SingleBracket10Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_13Ev_2pics_Plus => Self::SingleBracket13Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_13Ev_2pics_Minus => {
                Self::SingleBracket13Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_13Ev_3pics => Self::SingleBracket13Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_13Ev_5pics => Self::SingleBracket13Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_13Ev_7pics => Self::SingleBracket13Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_15Ev_2pics_Plus => Self::SingleBracket15Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_15Ev_2pics_Minus => {
                Self::SingleBracket15Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_15Ev_3pics => Self::SingleBracket15Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_15Ev_5pics => Self::SingleBracket15Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_15Ev_7pics => Self::SingleBracket15Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_17Ev_2pics_Plus => Self::SingleBracket17Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_17Ev_2pics_Minus => {
                Self::SingleBracket17Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_17Ev_3pics => Self::SingleBracket17Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_17Ev_5pics => Self::SingleBracket17Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_17Ev_7pics => Self::SingleBracket17Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_20Ev_2pics_Plus => Self::SingleBracket20Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_20Ev_2pics_Minus => {
                Self::SingleBracket20Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_20Ev_7pics => Self::SingleBracket20Ev7Pics,
            CrDriveMode_CrDrive_Single_Bracket_23Ev_2pics_Plus => Self::SingleBracket23Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_23Ev_2pics_Minus => {
                Self::SingleBracket23Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_23Ev_3pics => Self::SingleBracket23Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_23Ev_5pics => Self::SingleBracket23Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_25Ev_2pics_Plus => Self::SingleBracket25Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_25Ev_2pics_Minus => {
                Self::SingleBracket25Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_25Ev_3pics => Self::SingleBracket25Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_25Ev_5pics => Self::SingleBracket25Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_27Ev_2pics_Plus => Self::SingleBracket27Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_27Ev_2pics_Minus => {
                Self::SingleBracket27Ev2PicsMinus
            }
            CrDriveMode_CrDrive_Single_Bracket_27Ev_3pics => Self::SingleBracket27Ev3Pics,
            CrDriveMode_CrDrive_Single_Bracket_27Ev_5pics => Self::SingleBracket27Ev5Pics,
            CrDriveMode_CrDrive_Single_Bracket_30Ev_2pics_Plus => Self::SingleBracket30Ev2PicsPlus,
            CrDriveMode_CrDrive_Single_Bracket_30Ev_2pics_Minus => {
                Self::SingleBracket30Ev2PicsMinus
            }
            CrDriveMode_CrDrive_WB_Bracket_Lo => Self::WbBracketLo,
            CrDriveMode_CrDrive_WB_Bracket_Hi => Self::WbBracketHi,
            CrDriveMode_CrDrive_DRO_Bracket_Lo => Self::DroBracketLo,
            CrDriveMode_CrDrive_DRO_Bracket_Hi => Self::DroBracketHi,
            CrDriveMode_CrDrive_Continuous_Timer_3pics => Self::ContinuousTimer3Pics,
            CrDriveMode_CrDrive_Continuous_Timer_5pics => Self::ContinuousTimer5Pics,
            CrDriveMode_CrDrive_Continuous_Timer_2s_3pics => Self::ContinuousTimer2s3Pics,
            CrDriveMode_CrDrive_Continuous_Timer_2s_5pics => Self::ContinuousTimer2s5Pics,
            CrDriveMode_CrDrive_Continuous_Timer_5s_3pics => Self::ContinuousTimer5s3Pics,
            CrDriveMode_CrDrive_Continuous_Timer_5s_5pics => Self::ContinuousTimer5s5Pics,
            CrDriveMode_CrDrive_LPF_Bracket => Self::LpfBracket,
            CrDriveMode_CrDrive_RemoteCommander => Self::RemoteCommander,
            CrDriveMode_CrDrive_MirrorUp => Self::MirrorUp,
            CrDriveMode_CrDrive_SelfPortrait_1 => Self::SelfPortrait1,
            CrDriveMode_CrDrive_SelfPortrait_2 => Self::SelfPortrait2,
            _ => return None,
        })
    }
}

/// Metering mode settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum MeteringMode {
    /// Average metering
    Average = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_Average as u16,
    /// Center-weighted average metering
    CenterWeightedAverage =
        crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_CenterWeightedAverage as u16,
    /// Multi-spot metering
    MultiSpot = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_MultiSpot as u16,
    /// Center-spot metering
    CenterSpot = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_CenterSpot as u16,
    /// Multi-segment metering
    Multi = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_Multi as u16,
    /// Center-weighted metering
    CenterWeighted = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_CenterWeighted as u16,
    /// Entire screen average metering
    EntireScreenAverage = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_EntireScreenAverage as u16,
    /// Spot metering (standard)
    SpotStandard = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_Spot_Standard as u16,
    /// Spot metering (large)
    SpotLarge = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_Spot_Large as u16,
    /// Highlight-weighted metering
    HighLightWeighted = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_HighLightWeighted as u16,
    /// Standard metering
    Standard = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_Standard as u16,
    /// Backlight metering
    Backlight = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_Backlight as u16,
    /// Spotlight metering
    Spotlight = crsdk_sys::SCRSDK::CrMeteringMode_CrMetering_Spotlight as u16,
}

impl MeteringMode {
    /// Get the raw SDK value
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    /// Create from raw SDK value
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

/// Flash mode settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FlashMode {
    /// Auto flash
    Auto = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_Auto as u16,
    /// Flash off
    Off = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_Off as u16,
    /// Fill flash (always on)
    Fill = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_Fill as u16,
    /// External sync flash
    ExternalSync = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_ExternalSync as u16,
    /// Slow sync flash
    SlowSync = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_SlowSync as u16,
    /// Rear sync flash
    RearSync = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_RearSync as u16,
}

impl FlashMode {
    /// Get the raw SDK value
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    /// Create from raw SDK value
    pub fn from_raw(value: u64) -> Option<Self> {
        use crsdk_sys::SCRSDK::*;
        Some(match value as u16 {
            CrFlashMode_CrFlash_Auto => Self::Auto,
            CrFlashMode_CrFlash_Off => Self::Off,
            CrFlashMode_CrFlash_Fill => Self::Fill,
            CrFlashMode_CrFlash_ExternalSync => Self::ExternalSync,
            CrFlashMode_CrFlash_SlowSync => Self::SlowSync,
            CrFlashMode_CrFlash_RearSync => Self::RearSync,
            _ => return None,
        })
    }
}

/// Focus area settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FocusArea {
    /// Unknown focus area
    Unknown = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Unknown as u16,
    /// Wide focus area
    Wide = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Wide as u16,
    /// Zone focus area
    Zone = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Zone as u16,
    /// Center focus area
    Center = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Center as u16,
    /// Flexible spot small
    FlexibleSpotS = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_S as u16,
    /// Flexible spot medium
    FlexibleSpotM = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_M as u16,
    /// Flexible spot large
    FlexibleSpotL = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_L as u16,
    /// Expand flexible spot
    ExpandFlexibleSpot = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Expand_Flexible_Spot as u16,
    /// Flexible spot (generic)
    FlexibleSpot = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot as u16,
    /// Tracking wide
    TrackingWide = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Wide as u16,
    /// Tracking zone
    TrackingZone = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Zone as u16,
    /// Tracking center
    TrackingCenter = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Center as u16,
    /// Tracking flexible spot small
    TrackingFlexibleSpotS =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_S as u16,
    /// Tracking flexible spot medium
    TrackingFlexibleSpotM =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_M as u16,
    /// Tracking flexible spot large
    TrackingFlexibleSpotL =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_L as u16,
    /// Tracking expand flexible spot
    TrackingExpandFlexibleSpot =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Expand_Flexible_Spot as u16,
    /// Tracking flexible spot (generic)
    TrackingFlexibleSpot = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot as u16,
    /// Flexible spot extra small
    FlexibleSpotXS = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_XS as u16,
    /// Flexible spot extra large
    FlexibleSpotXL = crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_XL as u16,
    /// Flexible spot free size 1
    FlexibleSpotFreeSize1 =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_FreeSize1 as u16,
    /// Flexible spot free size 2
    FlexibleSpotFreeSize2 =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_FreeSize2 as u16,
    /// Flexible spot free size 3
    FlexibleSpotFreeSize3 =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Flexible_Spot_FreeSize3 as u16,
    /// Tracking flexible spot extra small
    TrackingFlexibleSpotXS =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_XS as u16,
    /// Tracking flexible spot extra large
    TrackingFlexibleSpotXL =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_XL as u16,
    /// Tracking flexible spot free size 1
    TrackingFlexibleSpotFreeSize1 =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_FreeSize1 as u16,
    /// Tracking flexible spot free size 2
    TrackingFlexibleSpotFreeSize2 =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_FreeSize2 as u16,
    /// Tracking flexible spot free size 3
    TrackingFlexibleSpotFreeSize3 =
        crsdk_sys::SCRSDK::CrFocusArea_CrFocusArea_Tracking_Flexible_Spot_FreeSize3 as u16,
}

impl FocusArea {
    /// Get the raw SDK value
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    /// Create from raw SDK value
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

/// SDK data type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    String,
    Unknown(u32),
}

impl DataType {
    pub(crate) fn from_sdk(value: u32) -> Self {
        use crsdk_sys::SCRSDK::*;

        // SDK data types can have modifier bits:
        // - ArrayBit (0x2000): indicates an array of values
        // - RangeBit (0x4000): indicates a range (min, max, step)
        // - SignBit  (0x1000): indicates signed type
        // We need to extract the base element type (lower bits)
        const ARRAY_BIT: u32 = 0x2000;
        const RANGE_BIT: u32 = 0x4000;

        // Mask off the array/range bits to get the base type
        let base_type = value & !(ARRAY_BIT | RANGE_BIT);

        match base_type {
            CrDataType_CrDataType_UInt8 => Self::UInt8,
            CrDataType_CrDataType_UInt16 => Self::UInt16,
            CrDataType_CrDataType_UInt32 => Self::UInt32,
            CrDataType_CrDataType_UInt64 => Self::UInt64,
            CrDataType_CrDataType_Int8 => Self::Int8,
            CrDataType_CrDataType_Int16 => Self::Int16,
            CrDataType_CrDataType_Int32 => Self::Int32,
            CrDataType_CrDataType_Int64 => Self::Int64,
            CrDataType_CrDataType_STR => Self::String,
            _ => Self::Unknown(value),
        }
    }
}

/// Property enable/writable status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnableFlag {
    /// Property is not supported by this camera
    NotSupported,
    /// Property is disabled
    Disabled,
    /// Property is readable and writable
    ReadWrite,
    /// Property is read-only (display only)
    ReadOnly,
    /// Property is write-only
    WriteOnly,
}

impl EnableFlag {
    pub(crate) fn from_sdk(value: i16) -> Self {
        use crsdk_sys::SCRSDK::*;
        match value {
            x if x == CrPropertyEnableFlag_CrEnableValue_NotSupported as i16 => Self::NotSupported,
            x if x == CrPropertyEnableFlag_CrEnableValue_False as i16 => Self::Disabled,
            x if x == CrPropertyEnableFlag_CrEnableValue_True as i16 => Self::ReadWrite,
            x if x == CrPropertyEnableFlag_CrEnableValue_DisplayOnly as i16 => Self::ReadOnly,
            x if x == CrPropertyEnableFlag_CrEnableValue_SetOnly as i16 => Self::WriteOnly,
            _ => Self::NotSupported,
        }
    }

    /// Check if the property is readable
    pub fn is_readable(self) -> bool {
        matches!(self, Self::ReadWrite | Self::ReadOnly)
    }

    /// Check if the property is writable
    pub fn is_writable(self) -> bool {
        matches!(self, Self::ReadWrite | Self::WriteOnly)
    }
}

/// A camera property with its current value and metadata
#[derive(Debug, Clone)]
pub struct DeviceProperty {
    /// Property code (raw SDK value)
    pub code: u32,
    /// Data type
    pub data_type: DataType,
    /// Enable/writable status
    pub enable_flag: EnableFlag,
    /// Current value as u64 (for numeric properties)
    pub current_value: u64,
    /// Current value as string (for string properties)
    pub current_string: Option<String>,
    /// Possible values this property can be set to
    pub possible_values: Vec<u64>,
}

impl DeviceProperty {
    /// Check if this property can be read
    pub fn is_readable(&self) -> bool {
        self.enable_flag.is_readable()
    }

    /// Check if this property can be written
    pub fn is_writable(&self) -> bool {
        self.enable_flag.is_writable()
    }

    /// Check if a value is valid for this property
    pub fn is_valid_value(&self, value: u64) -> bool {
        self.possible_values.is_empty() || self.possible_values.contains(&value)
    }
}

/// Parse possible values from SDK property data
pub(crate) fn parse_possible_values(
    data_type: DataType,
    values_ptr: *mut u8,
    values_size: u32,
) -> Vec<u64> {
    if values_ptr.is_null() || values_size == 0 {
        return Vec::new();
    }

    let element_size = match data_type {
        DataType::UInt8 | DataType::Int8 => 1,
        DataType::UInt16 | DataType::Int16 => 2,
        DataType::UInt32 | DataType::Int32 => 4,
        DataType::UInt64 | DataType::Int64 => 8,
        _ => return Vec::new(),
    };

    let count = values_size as usize / element_size;
    let mut result = Vec::with_capacity(count);

    unsafe {
        for i in 0..count {
            let offset = i * element_size;
            let value = match data_type {
                DataType::UInt8 => *values_ptr.add(offset) as u64,
                DataType::Int8 => *values_ptr.add(offset) as i8 as u64,
                DataType::UInt16 => {
                    u16::from_ne_bytes([*values_ptr.add(offset), *values_ptr.add(offset + 1)])
                        as u64
                }
                DataType::Int16 => {
                    i16::from_ne_bytes([*values_ptr.add(offset), *values_ptr.add(offset + 1)])
                        as u64
                }
                DataType::UInt32 => u32::from_ne_bytes([
                    *values_ptr.add(offset),
                    *values_ptr.add(offset + 1),
                    *values_ptr.add(offset + 2),
                    *values_ptr.add(offset + 3),
                ]) as u64,
                DataType::Int32 => i32::from_ne_bytes([
                    *values_ptr.add(offset),
                    *values_ptr.add(offset + 1),
                    *values_ptr.add(offset + 2),
                    *values_ptr.add(offset + 3),
                ]) as u64,
                DataType::UInt64 | DataType::Int64 => u64::from_ne_bytes([
                    *values_ptr.add(offset),
                    *values_ptr.add(offset + 1),
                    *values_ptr.add(offset + 2),
                    *values_ptr.add(offset + 3),
                    *values_ptr.add(offset + 4),
                    *values_ptr.add(offset + 5),
                    *values_ptr.add(offset + 6),
                    *values_ptr.add(offset + 7),
                ]),
                _ => continue,
            };
            result.push(value);
        }
    }

    result
}

/// Parse UTF-16 string from SDK's currentStr pointer
/// SDK format: first u16 is length, followed by that many u16 characters
unsafe fn parse_current_string(str_ptr: *const u16) -> Option<String> {
    if str_ptr.is_null() {
        return None;
    }

    // SAFETY: caller ensures str_ptr is valid
    // First element is the string length
    let len = unsafe { *str_ptr } as usize;
    if len == 0 || len > 1024 {
        return None;
    }

    // String data starts at index 1, length is (len - 1) characters
    // The length includes the length field itself, so actual char count is len - 1
    let char_count = len.saturating_sub(1);
    if char_count == 0 {
        return None;
    }

    // SAFETY: we read len from the first element, string data follows at offset 1
    let slice = unsafe { std::slice::from_raw_parts(str_ptr.add(1), char_count) };
    String::from_utf16(slice).ok()
}

/// Convert SDK CrDeviceProperty to our DeviceProperty
pub(crate) unsafe fn device_property_from_sdk(
    prop: &crsdk_sys::SCRSDK::CrDeviceProperty,
) -> DeviceProperty {
    let data_type = DataType::from_sdk(prop.valueType);

    // Try getSetValues first (settable values), fall back to values (all possible values)
    let possible_values = if prop.getSetValuesSize > 0 && !prop.getSetValues.is_null() {
        parse_possible_values(data_type, prop.getSetValues, prop.getSetValuesSize)
    } else {
        parse_possible_values(data_type, prop.values, prop.valuesSize)
    };

    // Always try to parse string value - some string properties don't report String data type
    // SAFETY: parse_current_string handles null pointers safely
    let current_string = unsafe { parse_current_string(prop.currentStr) };

    DeviceProperty {
        code: prop.code,
        data_type,
        enable_flag: EnableFlag::from_sdk(prop.enableFlag),
        current_value: prop.currentValue,
        current_string,
        possible_values,
    }
}

/// Convert SDK CrDeviceProperty to our DeviceProperty with debug info
pub(crate) unsafe fn device_property_from_sdk_debug(
    prop: &crsdk_sys::SCRSDK::CrDeviceProperty,
) -> (DeviceProperty, String) {
    let data_type = DataType::from_sdk(prop.valueType);

    let values_from_sdk = parse_possible_values(data_type, prop.values, prop.valuesSize);
    let get_set_values = parse_possible_values(data_type, prop.getSetValues, prop.getSetValuesSize);

    // Always try to parse string value - some string properties don't report String data type
    // SAFETY: parse_current_string handles null pointers safely
    let current_string = unsafe { parse_current_string(prop.currentStr) };

    let debug_info = format!(
        "dataType={:?}(raw={}) valuesSize={} values_ptr={:?} getSetValuesSize={} getSetValues_ptr={:?} values={:?} getSetValues={:?} currentStr={:?}",
        data_type,
        prop.valueType,
        prop.valuesSize,
        prop.values,
        prop.getSetValuesSize,
        prop.getSetValues,
        values_from_sdk,
        get_set_values,
        current_string,
    );

    let possible_values = if !get_set_values.is_empty() {
        get_set_values
    } else {
        values_from_sdk
    };

    let device_prop = DeviceProperty {
        code: prop.code,
        data_type,
        enable_flag: EnableFlag::from_sdk(prop.enableFlag),
        current_value: prop.currentValue,
        current_string,
        possible_values,
    };

    (device_prop, debug_info)
}

/// File type for still images (RAW vs JPEG) - used by MediaSLOT_FileType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FileType {
    None = 0,
    Jpeg = 1,
    Raw = 2,
    RawJpeg = 3,
    RawHeif = 4,
    Heif = 5,
}

impl FileType {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u16 {
            0 => Self::None,
            1 => Self::Jpeg,
            2 => Self::Raw,
            3 => Self::RawJpeg,
            4 => Self::RawHeif,
            5 => Self::Heif,
            _ => return None,
        })
    }
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "--",
                Self::Jpeg => "JPEG",
                Self::Raw => "RAW",
                Self::RawJpeg => "RAW+JPEG",
                Self::RawHeif => "RAW+HEIF",
                Self::Heif => "HEIF",
            }
        )
    }
}

/// JPEG image quality level - used by StillImageQuality property
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ImageQuality {
    Unknown = 0,
    Light = 1,
    Standard = 2,
    Fine = 3,
    ExFine = 4,
}

impl ImageQuality {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u16 {
            0 => Self::Unknown,
            1 => Self::Light,
            2 => Self::Standard,
            3 => Self::Fine,
            4 => Self::ExFine,
            _ => return None,
        })
    }
}

impl std::fmt::Display for ImageQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unknown => "--",
                Self::Light => "Light",
                Self::Standard => "Standard",
                Self::Fine => "Fine",
                Self::ExFine => "Extra Fine",
            }
        )
    }
}

/// Movie file format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MovieFileFormat {
    Avchd = 0,
    Mp4 = 1,
    XavcS4k = 2,
    XavcSHd = 3,
    XavcHs8k = 4,
    XavcHs4k = 5,
    XavcSL4k = 6,
    XavcSLHd = 7,
    XavcSI4k = 8,
    XavcSIHd = 9,
    XavcI = 10,
    XavcL = 11,
    XavcHsHd = 12,
    XavcSIDci4k = 13,
    XavcHIHq = 14,
    XavcHISq = 15,
    XavcHL = 16,
    XOcnXt = 17,
    XOcnSt = 18,
}

impl MovieFileFormat {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            0 => Self::Avchd,
            1 => Self::Mp4,
            2 => Self::XavcS4k,
            3 => Self::XavcSHd,
            4 => Self::XavcHs8k,
            5 => Self::XavcHs4k,
            6 => Self::XavcSL4k,
            7 => Self::XavcSLHd,
            8 => Self::XavcSI4k,
            9 => Self::XavcSIHd,
            10 => Self::XavcI,
            11 => Self::XavcL,
            12 => Self::XavcHsHd,
            13 => Self::XavcSIDci4k,
            14 => Self::XavcHIHq,
            15 => Self::XavcHISq,
            16 => Self::XavcHL,
            17 => Self::XOcnXt,
            18 => Self::XOcnSt,
            _ => return None,
        })
    }
}

impl std::fmt::Display for MovieFileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Avchd => "AVCHD",
                Self::Mp4 => "MP4",
                Self::XavcS4k => "XAVC S 4K",
                Self::XavcSHd => "XAVC S HD",
                Self::XavcHs8k => "XAVC HS 8K",
                Self::XavcHs4k => "XAVC HS 4K",
                Self::XavcSL4k => "XAVC S-L 4K",
                Self::XavcSLHd => "XAVC S-L HD",
                Self::XavcSI4k => "XAVC S-I 4K",
                Self::XavcSIHd => "XAVC S-I HD",
                Self::XavcI => "XAVC I",
                Self::XavcL => "XAVC L",
                Self::XavcHsHd => "XAVC HS HD",
                Self::XavcSIDci4k => "XAVC S-I DCI 4K",
                Self::XavcHIHq => "XAVC H-I HQ",
                Self::XavcHISq => "XAVC H-I SQ",
                Self::XavcHL => "XAVC H-L",
                Self::XOcnXt => "X-OCN XT",
                Self::XOcnSt => "X-OCN ST",
            }
        )
    }
}

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
        write!(
            f,
            "{}",
            match self {
                Self::Off => "Off",
                Self::On => "On",
            }
        )
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
        write!(
            f,
            "{}",
            match self {
                Self::Off => "Off",
                Self::On => "On",
            }
        )
    }
}

/// Format movie recording quality/bitrate setting to display string
pub fn format_movie_quality(value: u64) -> String {
    match value as u16 {
        0 => "--".to_string(),
        1 => "60p 50M".to_string(),
        2 => "30p 50M".to_string(),
        3 => "24p 50M".to_string(),
        4 => "50p 50M".to_string(),
        5 => "25p 50M".to_string(),
        6 => "60i 24M".to_string(),
        7 => "50i 24M FX".to_string(),
        8 => "60i 17M FH".to_string(),
        9 => "50i 17M FH".to_string(),
        10 => "60p 28M PS".to_string(),
        11 => "50p 28M PS".to_string(),
        12 => "24p 24M FX".to_string(),
        13 => "25p 24M FX".to_string(),
        14 => "24p 17M FH".to_string(),
        15 => "25p 17M FH".to_string(),
        16 => "120p 50M 720".to_string(),
        17 => "100p 50M 720".to_string(),
        18 => "1080 30p 16M".to_string(),
        19 => "1080 25p 16M".to_string(),
        20 => "720 30p 6M".to_string(),
        21 => "720 25p 6M".to_string(),
        22 => "1080 60p 28M".to_string(),
        23 => "1080 50p 28M".to_string(),
        24 => "60p 25M".to_string(),
        25 => "50p 25M".to_string(),
        26 => "30p 16M".to_string(),
        27 => "25p 16M".to_string(),
        28 => "120p 100M".to_string(),
        29 => "100p 100M".to_string(),
        30 => "120p 60M".to_string(),
        31 => "100p 60M".to_string(),
        32 => "30p 100M".to_string(),
        33 => "25p 100M".to_string(),
        34 => "24p 100M".to_string(),
        35 => "30p 60M".to_string(),
        36 => "25p 60M".to_string(),
        37 => "24p 60M".to_string(),
        38 => "600M 10bit".to_string(),
        39 => "500M 10bit".to_string(),
        40 => "400M 10bit".to_string(),
        41 => "300M 10bit".to_string(),
        42 => "280M 10bit".to_string(),
        43 => "250M 10bit".to_string(),
        44 => "240M 10bit".to_string(),
        45 => "222M 10bit".to_string(),
        46 => "200M 10bit".to_string(),
        47 => "200M 10bit 420".to_string(),
        48 => "200M 8bit".to_string(),
        49 => "185M 10bit".to_string(),
        50 => "150M 10bit 420".to_string(),
        51 => "150M 8bit".to_string(),
        52 => "140M 10bit".to_string(),
        53 => "111M 10bit".to_string(),
        54 => "100M 10bit".to_string(),
        55 => "100M 10bit 420".to_string(),
        56 => "100M 8bit".to_string(),
        57 => "93M 10bit".to_string(),
        58 => "89M 10bit".to_string(),
        59 => "75M 10bit 420".to_string(),
        60 => "60M 8bit".to_string(),
        61 => "50M 10bit".to_string(),
        62 => "50M 10bit 420".to_string(),
        63 => "50M 8bit".to_string(),
        64 => "45M 10bit 420".to_string(),
        65 => "30M 10bit 420".to_string(),
        66 => "25M 8bit".to_string(),
        67 => "16M 8bit".to_string(),
        68 => "520M 10bit".to_string(),
        69 => "260M 10bit".to_string(),
        _ => format!("{}M", value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_code_roundtrip() {
        let code = PropertyCode::IsoSensitivity;
        let raw = code.as_raw();
        let back = PropertyCode::from_raw(raw);
        assert_eq!(back, Some(code));
    }

    #[test]
    fn test_focus_mode_roundtrip() {
        let mode = FocusMode::AfContinuous;
        let raw = mode.as_raw();
        let back = FocusMode::from_raw(raw);
        assert_eq!(back, Some(mode));
    }

    #[test]
    fn test_white_balance_roundtrip() {
        let wb = WhiteBalance::Daylight;
        let raw = wb.as_raw();
        let back = WhiteBalance::from_raw(raw);
        assert_eq!(back, Some(wb));
    }

    #[test]
    fn test_exposure_program_roundtrip() {
        let mode = ExposureProgram::Manual;
        let raw = mode.as_raw();
        let back = ExposureProgram::from_raw(raw);
        assert_eq!(back, Some(mode));
    }

    #[test]
    fn test_drive_mode_roundtrip() {
        let mode = DriveMode::Single;
        let raw = mode.as_raw();
        let back = DriveMode::from_raw(raw);
        assert_eq!(back, Some(mode));
    }

    #[test]
    fn test_enable_flag_readable_writable() {
        assert!(EnableFlag::ReadWrite.is_readable());
        assert!(EnableFlag::ReadWrite.is_writable());
        assert!(EnableFlag::ReadOnly.is_readable());
        assert!(!EnableFlag::ReadOnly.is_writable());
        assert!(!EnableFlag::WriteOnly.is_readable());
        assert!(EnableFlag::WriteOnly.is_writable());
        assert!(!EnableFlag::NotSupported.is_readable());
        assert!(!EnableFlag::NotSupported.is_writable());
    }

    #[test]
    fn test_device_property_is_valid_value() {
        let prop = DeviceProperty {
            code: 0,
            data_type: DataType::UInt32,
            enable_flag: EnableFlag::ReadWrite,
            current_value: 100,
            current_string: None,
            possible_values: vec![100, 200, 400, 800],
        };
        assert!(prop.is_valid_value(100));
        assert!(prop.is_valid_value(400));
        assert!(!prop.is_valid_value(300));

        let prop_empty = DeviceProperty {
            code: 0,
            data_type: DataType::UInt32,
            enable_flag: EnableFlag::ReadWrite,
            current_value: 100,
            current_string: None,
            possible_values: vec![],
        };
        assert!(prop_empty.is_valid_value(999));
    }
}
