//! Camera property types, values, and metadata.
//!
//! This module provides:
//! - Core types for working with camera properties ([`DeviceProperty`], [`DataType`], [`EnableFlag`])
//! - Value enums for specific property types (organized by subsystem)
//! - Display names and descriptions for properties
//! - The [`PropertyValue`] trait for type-safe value conversion

mod category;
mod metadata;
#[cfg(test)]
mod todo;
mod traits;
mod typed_value;
pub mod values;

// Re-export core trait and typed value
pub use traits::PropertyValue;
pub use typed_value::TypedValue;

// Re-export category types
pub use category::{property_category, PropertyCategory};

// Re-export all value types from values/
pub use values::{
    AspectRatio, AutoManual, FileType, FlashMode, FocusArea, FocusMode, FocusTrackingStatus,
    ImageQuality, ImageSize, LiveViewDisplayEffect, LockIndicator, MeteringMode, OnOff,
    PrioritySetInAF, PrioritySetInAWB, ShutterMode, ShutterModeStatus, SilentModeApertureDrive,
    SubjectRecognitionAF, Switch, WhiteBalance,
};
pub use values::{ExposureCtrlType, ExposureProgram};

// Re-export drive and movie types from values/
pub use values::{DriveMode, IntervalRecShutterType, MovieFileFormat, MovieQuality};

use crsdk_sys::DevicePropertyCode;

/// SDK data type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// 8-bit unsigned integer
    UInt8,
    /// 16-bit unsigned integer
    UInt16,
    /// 32-bit unsigned integer
    UInt32,
    /// 64-bit unsigned integer
    UInt64,
    /// 8-bit signed integer
    Int8,
    /// 16-bit signed integer
    Int16,
    /// 32-bit signed integer
    Int32,
    /// 64-bit signed integer
    Int64,
    /// String value
    String,
    /// Unknown data type (raw SDK value)
    Unknown(u32),
}

impl DataType {
    pub(crate) fn from_sdk(value: u32) -> Self {
        use crsdk_sys::SCRSDK::*;

        const ARRAY_BIT: u32 = 0x2000;
        const RANGE_BIT: u32 = 0x4000;

        let base_type = value & !(ARRAY_BIT | RANGE_BIT);

        match base_type {
            x if x == CrDataType_CrDataType_UInt8 => Self::UInt8,
            x if x == CrDataType_CrDataType_UInt16 => Self::UInt16,
            x if x == CrDataType_CrDataType_UInt32 => Self::UInt32,
            x if x == CrDataType_CrDataType_UInt64 => Self::UInt64,
            x if x == CrDataType_CrDataType_Int8 => Self::Int8,
            x if x == CrDataType_CrDataType_Int16 => Self::Int16,
            x if x == CrDataType_CrDataType_Int32 => Self::Int32,
            x if x == CrDataType_CrDataType_Int64 => Self::Int64,
            x if x == CrDataType_CrDataType_STR => Self::String,
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
            x if x == CrPropertyEnableFlag_CrEnableValue_NotSupported => Self::NotSupported,
            x if x == CrPropertyEnableFlag_CrEnableValue_False => Self::Disabled,
            x if x == CrPropertyEnableFlag_CrEnableValue_True => Self::ReadWrite,
            x if x == CrPropertyEnableFlag_CrEnableValue_DisplayOnly => Self::ReadOnly,
            x if x == CrPropertyEnableFlag_CrEnableValue_SetOnly => Self::WriteOnly,
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

/// Constraint on what values a property can have.
///
/// The SDK provides value constraints in two forms:
/// - **Discrete**: A list of specific allowed values
/// - **Range**: A min/max/step triplet defining a numeric range
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ValueConstraint {
    /// No constraint information available
    #[default]
    None,
    /// Discrete list of allowed values
    Discrete(Vec<u64>),
    /// Numeric range with min, max, and step
    Range {
        /// Minimum allowed value
        min: i64,
        /// Maximum allowed value
        max: i64,
        /// Step/increment between valid values
        step: i64,
    },
}

impl ValueConstraint {
    /// Check if a value satisfies this constraint
    pub fn is_valid(&self, value: u64) -> bool {
        match self {
            Self::None => true,
            Self::Discrete(values) => values.is_empty() || values.contains(&value),
            Self::Range { min, max, step } => {
                let v = value as i64;
                if v < *min || v > *max {
                    return false;
                }
                if *step == 0 {
                    return true;
                }
                (v - min) % step == 0
            }
        }
    }

    /// Check if this constraint is empty (no values or no range)
    pub fn is_empty(&self) -> bool {
        match self {
            Self::None => true,
            Self::Discrete(values) => values.is_empty(),
            Self::Range { .. } => false,
        }
    }

    /// Get all valid values if this is a discrete constraint
    pub fn discrete_values(&self) -> Option<&[u64]> {
        match self {
            Self::Discrete(values) => Some(values),
            _ => None,
        }
    }

    /// Get range parameters if this is a range constraint
    pub fn range_params(&self) -> Option<(i64, i64, i64)> {
        match self {
            Self::Range { min, max, step } => Some((*min, *max, *step)),
            _ => None,
        }
    }

    /// Expand a range constraint into discrete values (for UI display)
    pub fn expand_range(&self) -> Option<Vec<i64>> {
        match self {
            Self::Range { min, max, step } => {
                if *step == 0 {
                    return Some(vec![*min, *max]);
                }
                let count = ((max - min) / step + 1) as usize;
                if count > 1000 {
                    return None;
                }
                Some((0..count as i64).map(|i| min + i * step).collect())
            }
            _ => None,
        }
    }
}

/// How to interpret and format a property's raw value.
///
/// This enum defines the semantic type of a property value, allowing type-safe
/// formatting and parsing. Use [`property_value_type`] to get the type for a property code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropertyValueType {
    // Formatted numeric values (special encoding)
    /// Aperture f-number (raw value / 100, e.g., 280 → f/2.8)
    Aperture,
    /// Shutter speed (upper 16 bits = numerator, lower 16 = denominator)
    ShutterSpeed,
    /// ISO sensitivity (raw value is ISO number)
    Iso,
    /// Exposure compensation (raw value in 1/3 EV steps)
    ExposureCompensation,
    /// Color temperature in Kelvin
    ColorTemperature,
    /// Movie recording quality/bitrate
    MovieQuality,

    // Enum value types
    /// Exposure program mode (P, A, S, M, etc.)
    ExposureProgram,
    /// Metering mode
    MeteringMode,
    /// Focus mode (AF-S, AF-C, MF, etc.)
    FocusMode,
    /// Focus area
    FocusArea,
    /// Subject recognition for AF
    SubjectRecognitionAF,
    /// AF priority setting
    PrioritySetInAF,
    /// Focus tracking status
    FocusTrackingStatus,
    /// White balance preset
    WhiteBalance,
    /// AWB priority setting
    PrioritySetInAWB,
    /// Drive mode (single, continuous, bracket, timer)
    DriveMode,
    /// Interval recording shutter type
    IntervalRecShutterType,
    /// Flash mode
    FlashMode,
    /// File type (slot assignment)
    FileType,
    /// Still image quality (RAW, JPEG, etc.)
    ImageQuality,
    /// Aspect ratio
    AspectRatio,
    /// Image size
    ImageSize,
    /// Movie file format (XAVC, etc.)
    MovieFileFormat,
    /// Shutter mode status
    ShutterModeStatus,
    /// Shutter mode
    ShutterMode,
    /// Exposure control type
    ExposureCtrlType,
    /// Live view display effect
    LiveViewDisplayEffect,
    /// Silent mode aperture drive
    SilentModeApertureDrive,

    // Generic toggle types
    /// On/Off toggle (0=Off, 1=On)
    OnOff,
    /// Switch toggle (1=Off, 2=On)
    Switch,
    /// Auto/Manual toggle
    AutoManual,
    /// Lock indicator (unlocked/locked)
    LockIndicator,

    // Raw value types
    /// Percentage (0-100)
    Percentage,
    /// Plain integer value
    Integer,
    /// Unknown/untyped value (display as raw hex)
    Unknown,
}

/// Get the value type for a property code.
///
/// This determines how to format and parse the raw SDK value for display.
/// Checks all subsystem modules for type mappings, with fallback to common types.
pub fn property_value_type(code: DevicePropertyCode) -> PropertyValueType {
    // Try subsystem-specific value types first
    if let Some(vt) = metadata::exposure::value_type(code) {
        return vt;
    }
    if let Some(vt) = metadata::focus::value_type(code) {
        return vt;
    }
    if let Some(vt) = metadata::white_balance::value_type(code) {
        return vt;
    }
    if let Some(vt) = metadata::drive::value_type(code) {
        return vt;
    }
    if let Some(vt) = metadata::flash::value_type(code) {
        return vt;
    }
    if let Some(vt) = metadata::image::value_type(code) {
        return vt;
    }
    if let Some(vt) = metadata::movie::value_type(code) {
        return vt;
    }

    // Fallback to common value types for categories without dedicated modules
    common_value_type(code)
}

fn common_value_type(code: DevicePropertyCode) -> PropertyValueType {
    match code {
        // Power
        DevicePropertyCode::BatteryRemain | DevicePropertyCode::BatteryLevel => {
            PropertyValueType::Percentage
        }

        // Display
        DevicePropertyCode::LiveViewDisplayEffect => PropertyValueType::LiveViewDisplayEffect,
        DevicePropertyCode::GridLineDisplay => PropertyValueType::Switch,
        DevicePropertyCode::GridLineType
        | DevicePropertyCode::LiveViewStatus
        | DevicePropertyCode::LiveViewImageQuality
        | DevicePropertyCode::LiveViewProtocol
        | DevicePropertyCode::DispMode
        | DevicePropertyCode::DispModeSetting
        | DevicePropertyCode::DispModeCandidate
        | DevicePropertyCode::MonitorLUTSetting
        | DevicePropertyCode::MonitorBrightnessType
        | DevicePropertyCode::MonitorBrightnessManual
        | DevicePropertyCode::SelectFinder
        | DevicePropertyCode::DisplayQualityFinder
        | DevicePropertyCode::DisplayedMenuStatus
        | DevicePropertyCode::GammaDisplayAssist
        | DevicePropertyCode::GammaDisplayAssistType
        | DevicePropertyCode::PlaybackContentsGammaType => PropertyValueType::Integer,

        // Stabilization
        DevicePropertyCode::ImageStabilizationSteadyShot
        | DevicePropertyCode::ImageStabilizationSteadyShotAdjust
        | DevicePropertyCode::ImageStabilizationSteadyShotFocalLength
        | DevicePropertyCode::ImageStabilizationFramingStabilizer => PropertyValueType::Integer,

        // Audio
        DevicePropertyCode::AudioRecording => PropertyValueType::OnOff,
        DevicePropertyCode::AudioSignals => PropertyValueType::OnOff,
        DevicePropertyCode::AudioSignalsStartEnd => PropertyValueType::OnOff,
        DevicePropertyCode::AudioSignalsVolume
        | DevicePropertyCode::AudioLevelDisplay
        | DevicePropertyCode::AudioInput1TypeSelect
        | DevicePropertyCode::AudioInput2TypeSelect
        | DevicePropertyCode::AudioInputCH1InputSelect
        | DevicePropertyCode::AudioInputCH2InputSelect
        | DevicePropertyCode::AudioInputCH3InputSelect
        | DevicePropertyCode::AudioInputCH4InputSelect
        | DevicePropertyCode::AudioInputCH1Level
        | DevicePropertyCode::AudioInputCH2Level
        | DevicePropertyCode::AudioInputCH3Level
        | DevicePropertyCode::AudioInputCH4Level
        | DevicePropertyCode::AudioInputMasterLevel
        | DevicePropertyCode::AudioInputCH1LevelControl
        | DevicePropertyCode::AudioInputCH2LevelControl
        | DevicePropertyCode::AudioInputCH3LevelControl
        | DevicePropertyCode::AudioInputCH4LevelControl
        | DevicePropertyCode::AudioStreamBitDepth
        | DevicePropertyCode::AudioStreamChannel
        | DevicePropertyCode::AudioStreamCodecType
        | DevicePropertyCode::AudioStreamSamplingFrequency => PropertyValueType::Integer,
        DevicePropertyCode::AudioInputCH1WindFilter
        | DevicePropertyCode::AudioInputCH2WindFilter
        | DevicePropertyCode::AudioInputCH3WindFilter
        | DevicePropertyCode::AudioInputCH4WindFilter
        | DevicePropertyCode::WindNoiseReduct => PropertyValueType::OnOff,

        // Silent
        DevicePropertyCode::SilentMode => PropertyValueType::Switch,
        DevicePropertyCode::SilentModeApertureDriveInAF => {
            PropertyValueType::SilentModeApertureDrive
        }

        // ND Filter
        DevicePropertyCode::NDFilter => PropertyValueType::Switch,
        DevicePropertyCode::NDFilterModeSetting => PropertyValueType::AutoManual,
        DevicePropertyCode::NDFilterMode
        | DevicePropertyCode::NDFilterValue
        | DevicePropertyCode::NDFilterSwitchingSetting
        | DevicePropertyCode::NDFilterPositionSetting
        | DevicePropertyCode::NDFilterOpticalDensityValue
        | DevicePropertyCode::NDFilterUnitSetting
        | DevicePropertyCode::NDFilterPresetSelect
        | DevicePropertyCode::NDFilterPreset1Value
        | DevicePropertyCode::NDFilterPreset2Value
        | DevicePropertyCode::NDFilterPreset3Value
        | DevicePropertyCode::ManualInputForNDFilterValue
        | DevicePropertyCode::PushAutoNDFilter => PropertyValueType::Integer,

        // Other/Misc
        DevicePropertyCode::AEL | DevicePropertyCode::FEL => PropertyValueType::LockIndicator,
        DevicePropertyCode::AutoReview => PropertyValueType::Switch,

        // Interval Recording
        DevicePropertyCode::IntervalRecMode => PropertyValueType::OnOff,
        DevicePropertyCode::IntervalRecNumberOfShots
        | DevicePropertyCode::IntervalRecShootingInterval
        | DevicePropertyCode::IntervalRecShootingStartTime
        | DevicePropertyCode::IntervalRecStatus => PropertyValueType::Integer,
        DevicePropertyCode::IntervalRecShootIntervalPriority => PropertyValueType::OnOff,
        DevicePropertyCode::IntervalRecAETrackingSensitivity => PropertyValueType::Integer,

        // Continuous Shooting
        DevicePropertyCode::ContinuousShootingSpotBoostStatus
        | DevicePropertyCode::ContinuousShootingSpotBoostFrameSpeed
        | DevicePropertyCode::ContinuousShootingSpotBoostEnableStatus => PropertyValueType::Integer,

        // Custom Buttons
        DevicePropertyCode::AssignableButton1
        | DevicePropertyCode::AssignableButton2
        | DevicePropertyCode::AssignableButton3
        | DevicePropertyCode::AssignableButton4
        | DevicePropertyCode::AssignableButton5
        | DevicePropertyCode::AssignableButton6
        | DevicePropertyCode::AssignableButton7
        | DevicePropertyCode::AssignableButton8
        | DevicePropertyCode::AssignableButton9
        | DevicePropertyCode::AssignableButton10
        | DevicePropertyCode::AssignableButton11
        | DevicePropertyCode::LensAssignableButton1
        | DevicePropertyCode::AssignableButtonIndicator1
        | DevicePropertyCode::AssignableButtonIndicator2
        | DevicePropertyCode::AssignableButtonIndicator3
        | DevicePropertyCode::AssignableButtonIndicator4
        | DevicePropertyCode::AssignableButtonIndicator5
        | DevicePropertyCode::AssignableButtonIndicator6
        | DevicePropertyCode::AssignableButtonIndicator7
        | DevicePropertyCode::AssignableButtonIndicator8
        | DevicePropertyCode::AssignableButtonIndicator9
        | DevicePropertyCode::AssignableButtonIndicator10
        | DevicePropertyCode::AssignableButtonIndicator11
        | DevicePropertyCode::ButtonAssignmentAssignable1
        | DevicePropertyCode::ButtonAssignmentAssignable2
        | DevicePropertyCode::ButtonAssignmentAssignable3
        | DevicePropertyCode::ButtonAssignmentAssignable4
        | DevicePropertyCode::ButtonAssignmentAssignable5
        | DevicePropertyCode::ButtonAssignmentAssignable6
        | DevicePropertyCode::ButtonAssignmentAssignable7
        | DevicePropertyCode::ButtonAssignmentAssignable8
        | DevicePropertyCode::ButtonAssignmentAssignable9
        | DevicePropertyCode::ButtonAssignmentAssignable10
        | DevicePropertyCode::ButtonAssignmentAssignable11
        | DevicePropertyCode::ButtonAssignmentLensAssignable1 => PropertyValueType::Integer,

        // Picture Profile
        DevicePropertyCode::PictureProfile
        | DevicePropertyCode::PictureProfileGamma
        | DevicePropertyCode::PictureProfileColorMode
        | DevicePropertyCode::PictureProfileSaturation
        | DevicePropertyCode::PictureProfileBlackLevel
        | DevicePropertyCode::PictureProfileBlackGammaLevel
        | DevicePropertyCode::PictureProfileBlackGammaRange
        | DevicePropertyCode::PictureProfileKneeMode
        | DevicePropertyCode::PictureProfileKneeAutoSetMaxPoint
        | DevicePropertyCode::PictureProfileKneeAutoSetSensitivity
        | DevicePropertyCode::PictureProfileKneeManualSetPoint
        | DevicePropertyCode::PictureProfileKneeManualSetSlope
        | DevicePropertyCode::PictureProfileDetailLevel
        | DevicePropertyCode::PictureProfileDetailAdjustMode
        | DevicePropertyCode::PictureProfileDetailAdjustVHBalance
        | DevicePropertyCode::PictureProfileDetailAdjustBWBalance
        | DevicePropertyCode::PictureProfileDetailAdjustLimit
        | DevicePropertyCode::PictureProfileDetailAdjustCrispening
        | DevicePropertyCode::PictureProfileDetailAdjustHiLightDetail
        | DevicePropertyCode::PictureProfileColorPhase
        | DevicePropertyCode::PictureProfileColorDepthRed
        | DevicePropertyCode::PictureProfileColorDepthGreen
        | DevicePropertyCode::PictureProfileColorDepthBlue
        | DevicePropertyCode::PictureProfileColorDepthCyan
        | DevicePropertyCode::PictureProfileColorDepthMagenta
        | DevicePropertyCode::PictureProfileColorDepthYellow
        | DevicePropertyCode::PictureProfileCopy
        | DevicePropertyCode::PictureProfileResetEnableStatus
        | DevicePropertyCode::CreativeLook
        | DevicePropertyCode::CreativeLookContrast
        | DevicePropertyCode::CreativeLookHighlights
        | DevicePropertyCode::CreativeLookShadows
        | DevicePropertyCode::CreativeLookSaturation
        | DevicePropertyCode::CreativeLookSharpness
        | DevicePropertyCode::CreativeLookSharpnessRange
        | DevicePropertyCode::CreativeLookClarity
        | DevicePropertyCode::CreativeLookFade
        | DevicePropertyCode::CreativeLookCustomLook
        | DevicePropertyCode::CreativeLookResetEnableStatus
        | DevicePropertyCode::BaseLookValue
        | DevicePropertyCode::BaseLookAppliedofPlayback
        | DevicePropertyCode::BaseLookNameofPlayback
        | DevicePropertyCode::BaseLookImportOperationEnableStatus => PropertyValueType::Integer,

        // Zoom
        DevicePropertyCode::ZoomScale
        | DevicePropertyCode::ZoomSetting
        | DevicePropertyCode::ZoomOperation
        | DevicePropertyCode::ZoomOperationWithInt16
        | DevicePropertyCode::ZoomOperationStatus
        | DevicePropertyCode::ZoomPositionSetting
        | DevicePropertyCode::ZoomPositionCurrentValue
        | DevicePropertyCode::ZoomDrivingStatus
        | DevicePropertyCode::ZoomSpeedRange
        | DevicePropertyCode::ZoomDistance
        | DevicePropertyCode::ZoomDistanceUnitSetting
        | DevicePropertyCode::ZoomBarInformation
        | DevicePropertyCode::ZoomTypeStatus
        | DevicePropertyCode::DigitalZoomScale
        | DevicePropertyCode::DigitalExtenderMagnificationSetting
        | DevicePropertyCode::TeleWideLeverValueCapability
        | DevicePropertyCode::RemoconZoomSpeedType => PropertyValueType::Integer,

        // Media
        DevicePropertyCode::MediaSLOT1Status
        | DevicePropertyCode::MediaSLOT2Status
        | DevicePropertyCode::MediaSLOT3Status
        | DevicePropertyCode::MediaSLOT1RemainingNumber
        | DevicePropertyCode::MediaSLOT2RemainingNumber
        | DevicePropertyCode::MediaSLOT1RemainingTime
        | DevicePropertyCode::MediaSLOT2RemainingTime
        | DevicePropertyCode::MediaSLOT3RemainingTime
        | DevicePropertyCode::MediaSLOT1FileType
        | DevicePropertyCode::MediaSLOT2FileType
        | DevicePropertyCode::MediaSLOT1ImageQuality
        | DevicePropertyCode::MediaSLOT2ImageQuality
        | DevicePropertyCode::MediaSLOT1ImageSize
        | DevicePropertyCode::MediaSLOT2ImageSize
        | DevicePropertyCode::MediaSLOT1RAWFileCompressionType
        | DevicePropertyCode::MediaSLOT2RAWFileCompressionType
        | DevicePropertyCode::MediaSLOT1QuickFormatEnableStatus
        | DevicePropertyCode::MediaSLOT2QuickFormatEnableStatus
        | DevicePropertyCode::MediaSLOT1FormatEnableStatus
        | DevicePropertyCode::MediaSLOT2FormatEnableStatus
        | DevicePropertyCode::MediaSLOT1WritingState
        | DevicePropertyCode::MediaSLOT2WritingState
        | DevicePropertyCode::MediaSLOT1Player
        | DevicePropertyCode::MediaSLOT2Player
        | DevicePropertyCode::MediaSLOT1RecordingAvailableType
        | DevicePropertyCode::MediaSLOT2RecordingAvailableType
        | DevicePropertyCode::MediaSLOT3RecordingAvailableType
        | DevicePropertyCode::MediaSLOT1ContentsInfoListEnableStatus
        | DevicePropertyCode::MediaSLOT2ContentsInfoListEnableStatus
        | DevicePropertyCode::MediaSLOT1ContentsInfoListUpdateTime
        | DevicePropertyCode::MediaSLOT2ContentsInfoListUpdateTime
        | DevicePropertyCode::MediaSLOT1ContentsInfoListRegenerateUpdateTime
        | DevicePropertyCode::MediaSLOT2ContentsInfoListRegenerateUpdateTime
        | DevicePropertyCode::MediaFormatProgressRate
        | DevicePropertyCode::AutoSwitchMedia
        | DevicePropertyCode::SimulRecSetting => PropertyValueType::Integer,

        // Power/Battery
        DevicePropertyCode::SecondBatteryLevel | DevicePropertyCode::TotalBatteryLevel => {
            PropertyValueType::Percentage
        }
        DevicePropertyCode::SecondBatteryRemain
        | DevicePropertyCode::TotalBatteryRemain
        | DevicePropertyCode::BatteryRemainingInMinutes
        | DevicePropertyCode::BatteryRemainDisplayUnit
        | DevicePropertyCode::BatteryRemainingInVoltage
        | DevicePropertyCode::CameraPowerStatus
        | DevicePropertyCode::PowerSource
        | DevicePropertyCode::RecordablePowerSources
        | DevicePropertyCode::DCVoltage => PropertyValueType::Integer,

        // Camera system
        DevicePropertyCode::CameraOperatingMode
        | DevicePropertyCode::CameraErrorCautionStatus
        | DevicePropertyCode::CameraSystemErrorInfo
        | DevicePropertyCode::CameraShakeStatus
        | DevicePropertyCode::CameraSettingReadOperationEnableStatus
        | DevicePropertyCode::CameraSettingSaveOperationEnableStatus
        | DevicePropertyCode::CameraSettingsResetEnableStatus
        | DevicePropertyCode::SdkControlMode
        | DevicePropertyCode::BodyKeyLock => PropertyValueType::Integer,

        // FTP
        DevicePropertyCode::FTPAutoTransfer
        | DevicePropertyCode::FTPAutoTransferTarget
        | DevicePropertyCode::FTPTransferTarget
        | DevicePropertyCode::FTPAutoTransferTargetStillImage
        | DevicePropertyCode::FTPTransferStillImageQualitySize
        | DevicePropertyCode::FTPConnectionStatus
        | DevicePropertyCode::FTPConnectionErrorInfo
        | DevicePropertyCode::FTPServerSettingOperationEnableStatus
        | DevicePropertyCode::FTPServerSettingVersion
        | DevicePropertyCode::FTPTransferSettingReadOperationEnableStatus
        | DevicePropertyCode::FTPTransferSettingSaveOperationEnableStatus
        | DevicePropertyCode::FTPTransferSettingSaveReadState
        | DevicePropertyCode::FTPJobListDataVersion
        | DevicePropertyCode::SelectFTPServer
        | DevicePropertyCode::SelectFTPServerID
        | DevicePropertyCode::ProtectImageInFTPTransfer
        | DevicePropertyCode::MovieFTPAutoTransferTarget
        | DevicePropertyCode::MovieFTPTransferTarget => PropertyValueType::Integer,

        // Subject Recognition
        DevicePropertyCode::SubjectRecognitionAF
        | DevicePropertyCode::SubjectRecognitionInAF
        | DevicePropertyCode::SubjectRecognitionAnimalBirdPriority
        | DevicePropertyCode::SubjectRecognitionAnimalBirdDetectionParts
        | DevicePropertyCode::SubjectRecognitionAnimalDetectionParts
        | DevicePropertyCode::SubjectRecognitionAnimalDetectionSensitivity
        | DevicePropertyCode::SubjectRecognitionAnimalTrackingSensitivity
        | DevicePropertyCode::SubjectRecognitionAnimalTrackingSubjectShiftRange
        | DevicePropertyCode::SubjectRecognitionBirdDetectionParts
        | DevicePropertyCode::SubjectRecognitionBirdDetectionSensitivity
        | DevicePropertyCode::SubjectRecognitionBirdTrackingSensitivity
        | DevicePropertyCode::SubjectRecognitionBirdTrackingSubjectShiftRange
        | DevicePropertyCode::SubjectRecognitionInsectDetectionSensitivity
        | DevicePropertyCode::SubjectRecognitionInsectTrackingSensitivity
        | DevicePropertyCode::SubjectRecognitionInsectTrackingSubjectShiftRange
        | DevicePropertyCode::SubjectRecognitionCarTrainDetectionSensitivity
        | DevicePropertyCode::SubjectRecognitionCarTrainTrackingSensitivity
        | DevicePropertyCode::SubjectRecognitionCarTrainTrackingSubjectShiftRange
        | DevicePropertyCode::SubjectRecognitionPlaneDetectionSensitivity
        | DevicePropertyCode::SubjectRecognitionPlaneTrackingSensitivity
        | DevicePropertyCode::SubjectRecognitionPlaneTrackingSubjectShiftRange
        | DevicePropertyCode::SubjectRecognitionPersonTrackingSubjectShiftRange
        | DevicePropertyCode::SubjectRecognitionPriorityOnRegisteredFace => {
            PropertyValueType::Integer
        }

        // Lens Compensation
        DevicePropertyCode::LensCompensationBreathing
        | DevicePropertyCode::LensCompensationChromaticAberration
        | DevicePropertyCode::LensCompensationDistortion
        | DevicePropertyCode::LensCompensationShading => PropertyValueType::OnOff,

        // Flicker
        DevicePropertyCode::FlickerLessShooting => PropertyValueType::OnOff,
        DevicePropertyCode::FlickerLessShootingStatus
        | DevicePropertyCode::FlickerScanEnableStatus
        | DevicePropertyCode::FlickerScanStatus => PropertyValueType::Integer,

        // E-framing
        DevicePropertyCode::EframingAutoFraming
        | DevicePropertyCode::EframingCommandVersion
        | DevicePropertyCode::EframingHDMICrop
        | DevicePropertyCode::EframingModeAuto
        | DevicePropertyCode::EframingProductionEffect
        | DevicePropertyCode::EframingRecordingImageCrop
        | DevicePropertyCode::EframingScaleAuto
        | DevicePropertyCode::EframingSpeedAuto
        | DevicePropertyCode::EframingSpeedPTZ
        | DevicePropertyCode::EframingTrackingStartMode
        | DevicePropertyCode::EframingType => PropertyValueType::Integer,

        // Cinematic Vlog
        DevicePropertyCode::CinematicVlogAFTransitionSpeed
        | DevicePropertyCode::CinematicVlogLook
        | DevicePropertyCode::CinematicVlogMood
        | DevicePropertyCode::CinematicVlogSetting => PropertyValueType::Integer,

        // Focal Distance
        DevicePropertyCode::FocalDistanceInFeet
        | DevicePropertyCode::FocalDistanceInMeter
        | DevicePropertyCode::FocalDistanceUnitSetting => PropertyValueType::Integer,

        // Depth of Field
        DevicePropertyCode::DepthOfFieldAdjustmentInterlockingMode
        | DevicePropertyCode::DepthOfFieldAdjustmentMode => PropertyValueType::Integer,

        // Content Transfer
        DevicePropertyCode::ContentsTransferCancelEnableStatus
        | DevicePropertyCode::ContentsTransferProgress
        | DevicePropertyCode::ContentsTransferStatus => PropertyValueType::Integer,

        // Monitoring
        DevicePropertyCode::MonitoringAvailableFormat
        | DevicePropertyCode::MonitoringDeliveringStatus
        | DevicePropertyCode::MonitoringDeliveryTypeSupportInfo
        | DevicePropertyCode::MonitoringFormatSupportInformation
        | DevicePropertyCode::MonitoringIsDelivering
        | DevicePropertyCode::MonitoringOutputDisplayHDMI
        | DevicePropertyCode::MonitoringOutputDisplaySDI
        | DevicePropertyCode::MonitoringOutputDisplaySetting1
        | DevicePropertyCode::MonitoringOutputDisplaySetting2
        | DevicePropertyCode::MonitoringOutputDisplaySettingDestAssign
        | DevicePropertyCode::MonitoringOutputFormat
        | DevicePropertyCode::MonitoringSettingVersion
        | DevicePropertyCode::MonitoringTransportProtocol => PropertyValueType::Integer,

        // Movie HDMI Output
        DevicePropertyCode::MovieHDMIOutput4KSetting
        | DevicePropertyCode::MovieHDMIOutputAudioCH
        | DevicePropertyCode::MovieHDMIOutputColorGamutForRAWOut
        | DevicePropertyCode::MovieHDMIOutputRAW
        | DevicePropertyCode::MovieHDMIOutputRawSetting
        | DevicePropertyCode::MovieHDMIOutputRecControl
        | DevicePropertyCode::MovieHDMIOutputRecMedia
        | DevicePropertyCode::MovieHDMIOutputResolution
        | DevicePropertyCode::MovieHDMIOutputTimeCode => PropertyValueType::Integer,

        // Movie Interval Recording
        DevicePropertyCode::MovieIntervalRecCountDownIntervalTime
        | DevicePropertyCode::MovieIntervalRecFrameRateSetting
        | DevicePropertyCode::MovieIntervalRecFrames
        | DevicePropertyCode::MovieIntervalRecIntervalTime
        | DevicePropertyCode::MovieIntervalRecRecordingDuration
        | DevicePropertyCode::MovieIntervalRecRecordingSetting => PropertyValueType::Integer,

        // Movie Image Stabilization
        DevicePropertyCode::MovieImageStabilizationLevel
        | DevicePropertyCode::MovieImageStabilizationSteadyShot => PropertyValueType::Integer,

        // Playback
        DevicePropertyCode::PlaySetOfMultiMedia
        | DevicePropertyCode::PlaybackContentsName
        | DevicePropertyCode::PlaybackContentsNumber
        | DevicePropertyCode::PlaybackContentsRecordingDateTime
        | DevicePropertyCode::PlaybackContentsRecordingFileFormat
        | DevicePropertyCode::PlaybackContentsRecordingFrameRate
        | DevicePropertyCode::PlaybackContentsRecordingResolution
        | DevicePropertyCode::PlaybackContentsTotalNumber
        | DevicePropertyCode::PlaybackMedia
        | DevicePropertyCode::PlaybackViewMode
        | DevicePropertyCode::PlaybackVolumeSettings => PropertyValueType::Integer,

        // Video Stream
        DevicePropertyCode::VideoStreamAdaptiveRateControl
        | DevicePropertyCode::VideoStreamBitRateCompressionMode
        | DevicePropertyCode::VideoStreamBitRateVBRMode
        | DevicePropertyCode::VideoStreamCodec
        | DevicePropertyCode::VideoStreamMaxBitRate
        | DevicePropertyCode::VideoStreamMovieRecPermission
        | DevicePropertyCode::VideoStreamResolution
        | DevicePropertyCode::VideoStreamResolutionMethod
        | DevicePropertyCode::VideoStreamSelect
        | DevicePropertyCode::VideoStreamSettingVersion => PropertyValueType::Integer,

        // Time Shift
        DevicePropertyCode::TimeShiftPreShootingTimeSetting
        | DevicePropertyCode::TimeShiftShooting
        | DevicePropertyCode::TimeShiftShootingStatus
        | DevicePropertyCode::TimeShiftTriggerSetting => PropertyValueType::Integer,

        // Pan/Tilt
        DevicePropertyCode::PanLimitMode
        | DevicePropertyCode::PanLimitRangeMaximum
        | DevicePropertyCode::PanLimitRangeMinimum
        | DevicePropertyCode::PanPositionCurrentValue
        | DevicePropertyCode::PanPositionStatus
        | DevicePropertyCode::PanTiltAccelerationRampCurve
        | DevicePropertyCode::TiltLimitMode
        | DevicePropertyCode::TiltLimitRangeMaximum
        | DevicePropertyCode::TiltLimitRangeMinimum
        | DevicePropertyCode::TiltPositionCurrentValue
        | DevicePropertyCode::TiltPositionStatus => PropertyValueType::Integer,

        // Picture Cache
        DevicePropertyCode::PictureCacheRecSetting
        | DevicePropertyCode::PictureCacheRecSizeAndTime => PropertyValueType::Integer,

        // Paint Look
        DevicePropertyCode::PaintLookAutoKnee
        | DevicePropertyCode::PaintLookBBlack
        | DevicePropertyCode::PaintLookDetailLevel
        | DevicePropertyCode::PaintLookDetailSetting
        | DevicePropertyCode::PaintLookKneePoint
        | DevicePropertyCode::PaintLookKneeSetting
        | DevicePropertyCode::PaintLookKneeSlope
        | DevicePropertyCode::PaintLookMasterBlack
        | DevicePropertyCode::PaintLookMultiMatrixAreaIndication
        | DevicePropertyCode::PaintLookMultiMatrixAxis
        | DevicePropertyCode::PaintLookMultiMatrixHue
        | DevicePropertyCode::PaintLookMultiMatrixSaturation
        | DevicePropertyCode::PaintLookMultiMatrixSetting
        | DevicePropertyCode::PaintLookRBlack
        | DevicePropertyCode::PaintLookUserMatrixBG
        | DevicePropertyCode::PaintLookUserMatrixBR
        | DevicePropertyCode::PaintLookUserMatrixGB
        | DevicePropertyCode::PaintLookUserMatrixGR
        | DevicePropertyCode::PaintLookUserMatrixLevel
        | DevicePropertyCode::PaintLookUserMatrixPhase
        | DevicePropertyCode::PaintLookUserMatrixRB
        | DevicePropertyCode::PaintLookUserMatrixRG
        | DevicePropertyCode::PaintLookUserMatrixSetting => PropertyValueType::Integer,

        // User Bit/User Base Look
        DevicePropertyCode::UserBitPreset
        | DevicePropertyCode::UserBitPresetResetEnableStatus
        | DevicePropertyCode::UserBitTimeRec
        | DevicePropertyCode::UserBaseLookAELevelOffset
        | DevicePropertyCode::UserBaseLookInput
        | DevicePropertyCode::UserBaseLookOutput => PropertyValueType::Integer,

        // Tally Lamp
        DevicePropertyCode::TallyLampControlGreen
        | DevicePropertyCode::TallyLampControlRed
        | DevicePropertyCode::TallyLampControlYellow => PropertyValueType::Integer,

        // Stream
        DevicePropertyCode::StreamButtonEnableStatus
        | DevicePropertyCode::StreamCipherType
        | DevicePropertyCode::StreamDisplayName
        | DevicePropertyCode::StreamLatency
        | DevicePropertyCode::StreamModeSetting
        | DevicePropertyCode::StreamSettingListOperationStatus
        | DevicePropertyCode::StreamStatus
        | DevicePropertyCode::StreamTTL
        | DevicePropertyCode::TargetStreamingDestinationSelect => PropertyValueType::Integer,

        // Scene File
        DevicePropertyCode::SceneFileCommandVersion
        | DevicePropertyCode::SceneFileDownloadOperationEnableStatus
        | DevicePropertyCode::SceneFileIndex
        | DevicePropertyCode::SceneFileIndexesAvailableForDownload
        | DevicePropertyCode::SceneFileUploadOperationEnableStatus => PropertyValueType::Integer,

        // Recorder
        DevicePropertyCode::RecorderClipName
        | DevicePropertyCode::RecorderControlMainSetting
        | DevicePropertyCode::RecorderControlProxySetting
        | DevicePropertyCode::RecorderExtRawStatus
        | DevicePropertyCode::RecorderSaveDestination
        | DevicePropertyCode::RecorderStartMain
        | DevicePropertyCode::RecorderStartProxy
        | DevicePropertyCode::RecordingFileNumber
        | DevicePropertyCode::RecordingFolderFormat
        | DevicePropertyCode::RecordingSettingFileName => PropertyValueType::Integer,

        // Movie misc
        DevicePropertyCode::MovieForwardButton
        | DevicePropertyCode::MovieNextButton
        | DevicePropertyCode::MoviePlayButton
        | DevicePropertyCode::MoviePlayPauseButton
        | DevicePropertyCode::MoviePlayStopButton
        | DevicePropertyCode::MoviePrevButton
        | DevicePropertyCode::MovieRewindButton
        | DevicePropertyCode::MovieQualityFullAutoMode
        | DevicePropertyCode::MovieRecButtonToggleEnableStatus
        | DevicePropertyCode::MovieRecReviewButton
        | DevicePropertyCode::MovieRecReviewPlayingState
        | DevicePropertyCode::MovieRecordingFrameRateRTSPSetting
        | DevicePropertyCode::MovieRecordingResolutionForRAW
        | DevicePropertyCode::MovieRecordingResolutionForRTSP
        | DevicePropertyCode::MovieShootingMode
        | DevicePropertyCode::MovieShootingModeColorGamut
        | DevicePropertyCode::MovieShootingModeTargetDisplay
        | DevicePropertyCode::LogShootingMode
        | DevicePropertyCode::LogShootingModeColorGamut => PropertyValueType::Integer,

        // S&Q Mode
        DevicePropertyCode::SQFrameRate
        | DevicePropertyCode::SQModeSetting
        | DevicePropertyCode::SQRecordingFrameRateSetting
        | DevicePropertyCode::SQRecordingSetting => PropertyValueType::Integer,

        // Various enables/misc
        DevicePropertyCode::APSCOrFullSwitchingEnableStatus
        | DevicePropertyCode::APSCOrFullSwitchingSetting
        | DevicePropertyCode::APSCS35
        | DevicePropertyCode::AmountOfDefocusSetting
        | DevicePropertyCode::AntidustShutterWhenPowerOff
        | DevicePropertyCode::ApertureDriveInAF
        | DevicePropertyCode::AreaTimeZoneSettingVersion
        | DevicePropertyCode::AutoPowerOffTemperature
        | DevicePropertyCode::AutoRecognitionTargetCandidates
        | DevicePropertyCode::AutoRecognitionTargetSetting
        | DevicePropertyCode::BodySerialNumber
        | DevicePropertyCode::CallSetting
        | DevicePropertyCode::CameraButtonFunction
        | DevicePropertyCode::CameraButtonFunctionMulti
        | DevicePropertyCode::CameraButtonFunctionStatus
        | DevicePropertyCode::CameraDialFunction
        | DevicePropertyCode::CameraEframing
        | DevicePropertyCode::CameraLeverFunction
        | DevicePropertyCode::CameraSettingSaveReadState
        | DevicePropertyCode::CancelMediaFormatEnableStatus
        | DevicePropertyCode::CancelRemoteTouchOperationEnableStatus
        | DevicePropertyCode::ColorSpace
        | DevicePropertyCode::ControlForHDMI
        | DevicePropertyCode::CreateNewFolder
        | DevicePropertyCode::CreateNewFolderEnableStatus
        | DevicePropertyCode::CurrentSceneFileEdited
        | DevicePropertyCode::CustomGridLineFileCommandVersion
        | DevicePropertyCode::DateTimeSettings
        | DevicePropertyCode::DeSqueezeDisplayRatio
        | DevicePropertyCode::DebugMode
        | DevicePropertyCode::DeleteContentOperationEnableStatusSLOT1
        | DevicePropertyCode::DeleteContentOperationEnableStatusSLOT2
        | DevicePropertyCode::DeleteUserBaseLook
        | DevicePropertyCode::DeviceOverheatingState
        | DevicePropertyCode::DifferentSetForSQMovie
        | DevicePropertyCode::ElectricFrontCurtainShutter
        | DevicePropertyCode::EmbedLUTFile
        | DevicePropertyCode::EnlargeScreenSetting
        | DevicePropertyCode::EstimatePictureSize
        | DevicePropertyCode::ExtendedInterfaceMode
        | DevicePropertyCode::FTPFunction
        | DevicePropertyCode::FTPPowerSave
        | DevicePropertyCode::FaceEyeDetectionAFStatus
        | DevicePropertyCode::FirmwareUpdateCommandVersion
        | DevicePropertyCode::ForcedFileNumberResetEnableStatus
        | DevicePropertyCode::FullTimeDMF
        | DevicePropertyCode::FunctionOfRemoteTouchOperation
        | DevicePropertyCode::FunctionOfTouchOperation
        | DevicePropertyCode::GetOnly
        | DevicePropertyCode::GridLineDisplayPlayback
        | DevicePropertyCode::HDMIResolutionStillPlay
        | DevicePropertyCode::HighResolutionShutterSpeed
        | DevicePropertyCode::HighResolutionShutterSpeedAdjust
        | DevicePropertyCode::HighResolutionShutterSpeedAdjustInIntegralMultiples
        | DevicePropertyCode::HighResolutionShutterSpeedSetting
        | DevicePropertyCode::HomeMenuSetting
        | DevicePropertyCode::IPSetupProtocolSetting
        | DevicePropertyCode::IRRemoteSetting
        | DevicePropertyCode::ImageIDNum
        | DevicePropertyCode::ImageIDNumSetting
        | DevicePropertyCode::ImageIDString
        | DevicePropertyCode::ImagerScanMode
        | DevicePropertyCode::LanguageSetting
        | DevicePropertyCode::LensAssignableButtonIndicator1
        | DevicePropertyCode::LensInformationEnableStatus
        | DevicePropertyCode::LensModelName
        | DevicePropertyCode::LensSerialNumber
        | DevicePropertyCode::LensVersionNumber
        | DevicePropertyCode::LiveViewArea
        | DevicePropertyCode::LiveViewImageQualityByNumericalValue
        | DevicePropertyCode::MaxVal
        | DevicePropertyCode::MaximumNumberOfBytes
        | DevicePropertyCode::MaximumSizeOfImageIDString
        | DevicePropertyCode::MicrophoneDirectivity
        | DevicePropertyCode::ModelName
        | DevicePropertyCode::MonitorLUTSetting1
        | DevicePropertyCode::MonitorLUTSetting2
        | DevicePropertyCode::MonitorLUTSetting3
        | DevicePropertyCode::MonitorLUTSettingOutputDestAssign
        | DevicePropertyCode::OSDImageMode
        | DevicePropertyCode::PixelMappingEnableStatus
        | DevicePropertyCode::PostViewTransferResourceStatus
        | DevicePropertyCode::PresetPTZFSlotNumber
        | DevicePropertyCode::PriorityKeySettings
        | DevicePropertyCode::ProductShowcaseSet
        | DevicePropertyCode::ProgramShiftStatus
        | DevicePropertyCode::ProxyRecordingSetting
        | DevicePropertyCode::PushAFModeSetting
        | DevicePropertyCode::PushAGC
        | DevicePropertyCode::PushAutoIris
        | DevicePropertyCode::RAWJPCSaveImage
        | DevicePropertyCode::RecognitionTarget
        | DevicePropertyCode::ReleaseWithoutCard
        | DevicePropertyCode::ReleaseWithoutLens
        | DevicePropertyCode::RemoteKeySLOTSelectButton
        | DevicePropertyCode::RemoteKeyThumbnailButton
        | DevicePropertyCode::RemoteTouchOperation
        | DevicePropertyCode::RemoteTouchOperationEnableStatus
        | DevicePropertyCode::RightLeftEyeSelect
        | DevicePropertyCode::S1
        | DevicePropertyCode::S2
        | DevicePropertyCode::SelectIPTCMetadata
        | DevicePropertyCode::SelectUserBaseLookToEdit
        | DevicePropertyCode::SelectUserBaseLookToSetInPPLUT
        | DevicePropertyCode::SensorCleaningEnableStatus
        | DevicePropertyCode::SetCopyright
        | DevicePropertyCode::SetPhotographer
        | DevicePropertyCode::SetPresetPTZFBinaryVersion
        | DevicePropertyCode::ShootingEnableSettingLicense
        | DevicePropertyCode::ShootingTimingPreNotificationMode
        | DevicePropertyCode::SilentModeAutoPixelMapping
        | DevicePropertyCode::SilentModeShutterWhenPowerOff
        | DevicePropertyCode::SimulRecSettingMovieRecButton
        | DevicePropertyCode::SnapshotInfo
        | DevicePropertyCode::SoftwareVersion
        | DevicePropertyCode::StillImageTransSize
        | DevicePropertyCode::SynchroterminalForcedOutput
        | DevicePropertyCode::SystemErrorCautionStatus
        | DevicePropertyCode::TCUBDisplaySetting
        | DevicePropertyCode::TNumber
        | DevicePropertyCode::TimeCodePresetResetEnableStatus
        | DevicePropertyCode::TopOfTheGroupShootingMarkSetting
        | DevicePropertyCode::TouchFunctionInMF
        | DevicePropertyCode::TouchOperation
        | DevicePropertyCode::TrackingOnAndAFOnEnableStatus
        | DevicePropertyCode::USBPowerSupply
        | DevicePropertyCode::UpdateBodyStatus
        | DevicePropertyCode::UploadDatasetVersion
        | DevicePropertyCode::ValidRecordingVideoFormat
        | DevicePropertyCode::VideoRecordingFormatBitrateSetting
        | DevicePropertyCode::VideoRecordingFormatQuality
        | DevicePropertyCode::WakeOnLAN
        | DevicePropertyCode::WriteCopyrightInfo => PropertyValueType::Integer,

        _ => PropertyValueType::Unknown,
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
    /// Constraint on what values this property can be set to
    pub constraint: ValueConstraint,
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
        self.constraint.is_valid(value)
    }

    /// Get discrete possible values (for backward compatibility and UI)
    ///
    /// Returns `Some(&[u64])` if this property has discrete values,
    /// `None` if it's a range or has no constraint.
    pub fn possible_values(&self) -> Option<&[u64]> {
        self.constraint.discrete_values()
    }

    /// Check if this property is range-constrained
    pub fn is_range(&self) -> bool {
        matches!(self.constraint, ValueConstraint::Range { .. })
    }

    /// Get range parameters if this is a range-constrained property
    ///
    /// Returns `Some((min, max, step))` if this property has a range constraint.
    pub fn range_params(&self) -> Option<(i64, i64, i64)> {
        self.constraint.range_params()
    }
}

const RANGE_BIT: u32 = 0x4000;

/// Parse raw values from SDK property data as u64 (for discrete values)
pub(crate) fn parse_raw_values(
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

/// Parse raw values as signed i64 (for range constraints with signed types)
fn parse_raw_values_signed(data_type: DataType, values_ptr: *mut u8, values_size: u32) -> Vec<i64> {
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
            let value: i64 = match data_type {
                DataType::UInt8 => *values_ptr.add(offset) as i64,
                DataType::Int8 => (*values_ptr.add(offset) as i8) as i64,
                DataType::UInt16 => {
                    u16::from_ne_bytes([*values_ptr.add(offset), *values_ptr.add(offset + 1)])
                        as i64
                }
                DataType::Int16 => {
                    i16::from_ne_bytes([*values_ptr.add(offset), *values_ptr.add(offset + 1)])
                        as i64
                }
                DataType::UInt32 => u32::from_ne_bytes([
                    *values_ptr.add(offset),
                    *values_ptr.add(offset + 1),
                    *values_ptr.add(offset + 2),
                    *values_ptr.add(offset + 3),
                ]) as i64,
                DataType::Int32 => i32::from_ne_bytes([
                    *values_ptr.add(offset),
                    *values_ptr.add(offset + 1),
                    *values_ptr.add(offset + 2),
                    *values_ptr.add(offset + 3),
                ]) as i64,
                DataType::UInt64 => u64::from_ne_bytes([
                    *values_ptr.add(offset),
                    *values_ptr.add(offset + 1),
                    *values_ptr.add(offset + 2),
                    *values_ptr.add(offset + 3),
                    *values_ptr.add(offset + 4),
                    *values_ptr.add(offset + 5),
                    *values_ptr.add(offset + 6),
                    *values_ptr.add(offset + 7),
                ]) as i64,
                DataType::Int64 => i64::from_ne_bytes([
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

/// Parse a ValueConstraint from SDK property data
pub(crate) fn parse_constraint(
    raw_value_type: u32,
    data_type: DataType,
    values_ptr: *mut u8,
    values_size: u32,
) -> ValueConstraint {
    if values_ptr.is_null() || values_size == 0 {
        return ValueConstraint::None;
    }

    let is_range = (raw_value_type & RANGE_BIT) != 0;

    if is_range {
        let signed_values = parse_raw_values_signed(data_type, values_ptr, values_size);
        if signed_values.len() >= 3 {
            return ValueConstraint::Range {
                min: signed_values[0],
                max: signed_values[1],
                step: signed_values[2],
            };
        } else if signed_values.len() == 2 {
            return ValueConstraint::Range {
                min: signed_values[0],
                max: signed_values[1],
                step: 1,
            };
        }
    }

    let values = parse_raw_values(data_type, values_ptr, values_size);
    if values.is_empty() {
        ValueConstraint::None
    } else {
        ValueConstraint::Discrete(values)
    }
}

/// Parse UTF-16 string from SDK's currentStr pointer
unsafe fn parse_current_string(str_ptr: *const u16) -> Option<String> {
    if str_ptr.is_null() {
        return None;
    }

    let len = unsafe { *str_ptr } as usize;
    if len == 0 || len > 1024 {
        return None;
    }

    let char_count = len.saturating_sub(1);
    if char_count == 0 {
        return None;
    }

    let slice = unsafe { std::slice::from_raw_parts(str_ptr.add(1), char_count) };
    String::from_utf16(slice).ok()
}

/// Convert SDK CrDeviceProperty to our DeviceProperty
pub(crate) unsafe fn device_property_from_sdk(
    prop: &crsdk_sys::SCRSDK::CrDeviceProperty,
) -> DeviceProperty {
    let data_type = DataType::from_sdk(prop.valueType);

    let constraint = if prop.getSetValuesSize > 0 && !prop.getSetValues.is_null() {
        parse_constraint(
            prop.valueType,
            data_type,
            prop.getSetValues,
            prop.getSetValuesSize,
        )
    } else {
        parse_constraint(prop.valueType, data_type, prop.values, prop.valuesSize)
    };

    let current_string = unsafe { parse_current_string(prop.currentStr) };

    DeviceProperty {
        code: prop.code,
        data_type,
        enable_flag: EnableFlag::from_sdk(prop.enableFlag),
        current_value: prop.currentValue,
        current_string,
        constraint,
    }
}

/// Convert SDK CrDeviceProperty to our DeviceProperty with debug info
pub(crate) unsafe fn device_property_from_sdk_debug(
    prop: &crsdk_sys::SCRSDK::CrDeviceProperty,
) -> (DeviceProperty, String) {
    let data_type = DataType::from_sdk(prop.valueType);

    let values_from_sdk = parse_raw_values(data_type, prop.values, prop.valuesSize);
    let get_set_values = parse_raw_values(data_type, prop.getSetValues, prop.getSetValuesSize);

    let is_range = (prop.valueType & RANGE_BIT) != 0;

    let constraint = if prop.getSetValuesSize > 0 && !prop.getSetValues.is_null() {
        parse_constraint(
            prop.valueType,
            data_type,
            prop.getSetValues,
            prop.getSetValuesSize,
        )
    } else {
        parse_constraint(prop.valueType, data_type, prop.values, prop.valuesSize)
    };

    let current_string = unsafe { parse_current_string(prop.currentStr) };

    let debug_info = format!(
        "dataType={:?}(raw={:#06x}) is_range={} valuesSize={} values_ptr={:?} getSetValuesSize={} getSetValues_ptr={:?} values={:?} getSetValues={:?} constraint={:?} currentStr={:?}",
        data_type,
        prop.valueType,
        is_range,
        prop.valuesSize,
        prop.values,
        prop.getSetValuesSize,
        prop.getSetValues,
        values_from_sdk,
        get_set_values,
        constraint,
        current_string,
    );

    let device_prop = DeviceProperty {
        code: prop.code,
        data_type,
        enable_flag: EnableFlag::from_sdk(prop.enableFlag),
        current_value: prop.currentValue,
        current_string,
        constraint,
    };

    (device_prop, debug_info)
}

/// Get a description of what a property does.
pub fn property_description(code: DevicePropertyCode) -> &'static str {
    match property_category(code) {
        PropertyCategory::Exposure | PropertyCategory::Metering => {
            metadata::exposure::description(code)
        }
        PropertyCategory::Focus => metadata::focus::description(code),
        PropertyCategory::WhiteBalance => metadata::white_balance::description(code),
        PropertyCategory::Drive => metadata::drive::description(code),
        PropertyCategory::Flash => metadata::flash::description(code),
        PropertyCategory::Image => metadata::image::description(code),
        PropertyCategory::Movie => metadata::movie::description(code),
        PropertyCategory::Media => media_description(code),
        PropertyCategory::Power => power_description(code),
        PropertyCategory::Zoom => zoom_description(code),
        PropertyCategory::Lens => lens_description(code),
        PropertyCategory::Audio => audio_description(code),
        PropertyCategory::PictureProfile => picture_profile_description(code),
        PropertyCategory::NDFilter => nd_filter_description(code),
        PropertyCategory::Display => display_description(code),
        PropertyCategory::Stabilization => stabilization_description(code),
        PropertyCategory::Silent => silent_description(code),
        _ => other_description(code),
    }
}

/// Get a human-readable display name for a property code.
pub fn property_display_name(code: DevicePropertyCode) -> &'static str {
    match property_category(code) {
        PropertyCategory::Exposure | PropertyCategory::Metering => {
            metadata::exposure::display_name(code)
        }
        PropertyCategory::Focus => metadata::focus::display_name(code),
        PropertyCategory::WhiteBalance => metadata::white_balance::display_name(code),
        PropertyCategory::Drive => metadata::drive::display_name(code),
        PropertyCategory::Flash => metadata::flash::display_name(code),
        PropertyCategory::Image => metadata::image::display_name(code),
        PropertyCategory::Movie => metadata::movie::display_name(code),
        PropertyCategory::Media => media_display_name(code),
        PropertyCategory::Power => power_display_name(code),
        PropertyCategory::Zoom => zoom_display_name(code),
        PropertyCategory::Lens => lens_display_name(code),
        PropertyCategory::Audio => audio_display_name(code),
        PropertyCategory::PictureProfile => picture_profile_display_name(code),
        PropertyCategory::NDFilter => nd_filter_display_name(code),
        PropertyCategory::Display => display_display_name(code),
        PropertyCategory::Stabilization => stabilization_display_name(code),
        PropertyCategory::Silent => silent_display_name(code),
        _ => other_display_name(code),
    }
}

// === Inline metadata for categories without their own modules ===

fn media_description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::MediaSLOT1Status => {
            "Status of memory card in slot 1. Shows if a card is inserted, its capacity, and any errors."
        }
        DevicePropertyCode::MediaSLOT2Status => {
            "Status of memory card in slot 2. Shows if a card is inserted, its capacity, and any errors."
        }
        DevicePropertyCode::MediaSLOT3Status => {
            "Status of memory card in slot 3. Shows if a card is inserted, its capacity, and any errors."
        }
        DevicePropertyCode::AutoSwitchMedia => {
            "Automatically switches to the other card slot when the current card fills up. Prevents missed shots."
        }
        DevicePropertyCode::SimulRecSetting => {
            "Records to both card slots simultaneously. Provides instant backup of every shot."
        }
        DevicePropertyCode::MediaSLOT1FileType => {
            "File type (RAW/JPEG/etc.) saved to memory card slot 1."
        }
        DevicePropertyCode::MediaSLOT2FileType => {
            "File type (RAW/JPEG/etc.) saved to memory card slot 2."
        }
        DevicePropertyCode::MediaSLOT1ImageSize | DevicePropertyCode::MediaSLOT2ImageSize => {
            "Image resolution for files saved to this slot."
        }
        DevicePropertyCode::MediaSLOT1ImageQuality | DevicePropertyCode::MediaSLOT2ImageQuality => {
            "JPEG compression quality for files saved to this slot."
        }
        DevicePropertyCode::MediaSLOT1RAWFileCompressionType
        | DevicePropertyCode::MediaSLOT2RAWFileCompressionType => {
            "RAW file compression type. Uncompressed preserves all data. Lossless/lossy compressed saves space."
        }
        DevicePropertyCode::MediaSLOT1RemainingNumber | DevicePropertyCode::MediaSLOT2RemainingNumber => {
            "Estimated number of photos remaining at current settings."
        }
        DevicePropertyCode::MediaSLOT1RemainingTime
        | DevicePropertyCode::MediaSLOT2RemainingTime
        | DevicePropertyCode::MediaSLOT3RemainingTime => {
            "Estimated recording time remaining at current video settings."
        }
        DevicePropertyCode::MediaSLOT1FormatEnableStatus
        | DevicePropertyCode::MediaSLOT2FormatEnableStatus => {
            "Whether the card in this slot can be formatted."
        }
        DevicePropertyCode::MediaSLOT1QuickFormatEnableStatus
        | DevicePropertyCode::MediaSLOT2QuickFormatEnableStatus => {
            "Whether quick format is available for the card in this slot."
        }
        DevicePropertyCode::MediaSLOT1WritingState | DevicePropertyCode::MediaSLOT2WritingState => {
            "Current write status of the card. Shows if data is being written."
        }
        DevicePropertyCode::MediaSLOT1Player | DevicePropertyCode::MediaSLOT2Player => {
            "Playback source selection for this card slot."
        }
        DevicePropertyCode::MediaSLOT1RecordingAvailableType
        | DevicePropertyCode::MediaSLOT2RecordingAvailableType
        | DevicePropertyCode::MediaSLOT3RecordingAvailableType => {
            "Types of recordings supported by the card in this slot (photo, video, etc.)."
        }
        DevicePropertyCode::MediaSLOT1ContentsInfoListEnableStatus
        | DevicePropertyCode::MediaSLOT2ContentsInfoListEnableStatus => {
            "Whether the content list for this slot is available for reading."
        }
        DevicePropertyCode::MediaSLOT1ContentsInfoListUpdateTime
        | DevicePropertyCode::MediaSLOT2ContentsInfoListUpdateTime => {
            "Last update timestamp of the content info list."
        }
        DevicePropertyCode::MediaSLOT1ContentsInfoListRegenerateUpdateTime
        | DevicePropertyCode::MediaSLOT2ContentsInfoListRegenerateUpdateTime => {
            "Timestamp when the content list was regenerated."
        }
        DevicePropertyCode::MediaFormatProgressRate => {
            "Progress percentage of current format operation."
        }
        DevicePropertyCode::PresetPTZFSlotNumber => {
            "Preset slot for Pan/Tilt/Zoom/Focus position recall."
        }
        DevicePropertyCode::PlaySetOfMultiMedia => {
            "Configuration for playing content from multiple media sources."
        }
        DevicePropertyCode::PlaybackMedia => {
            "Memory card slot or media source for playback. Selects which card to browse."
        }
        DevicePropertyCode::RemoteKeySLOTSelectButton => {
            "Remote control button to select memory card slot."
        }
        DevicePropertyCode::CancelMediaFormatEnableStatus => {
            "Whether media format operation can be cancelled."
        }
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT1 => {
            "Whether content deletion is available for slot 1."
        }
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT2 => {
            "Whether content deletion is available for slot 2."
        }
        _ => "",
    }
}

fn media_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::MediaSLOT1Status => "Slot 1 Status",
        DevicePropertyCode::MediaSLOT2Status => "Slot 2 Status",
        DevicePropertyCode::MediaSLOT3Status => "Slot 3 Status",
        DevicePropertyCode::MediaSLOT1RemainingNumber => "Slot 1 Remain #",
        DevicePropertyCode::MediaSLOT2RemainingNumber => "Slot 2 Remain #",
        DevicePropertyCode::MediaSLOT1RemainingTime => "Slot 1 Time",
        DevicePropertyCode::MediaSLOT2RemainingTime => "Slot 2 Time",
        DevicePropertyCode::MediaSLOT3RemainingTime => "Slot 3 Time",
        DevicePropertyCode::MediaSLOT1FileType => "Slot 1 File Type",
        DevicePropertyCode::MediaSLOT2FileType => "Slot 2 File Type",
        DevicePropertyCode::MediaSLOT1ImageQuality => "Slot 1 Quality",
        DevicePropertyCode::MediaSLOT2ImageQuality => "Slot 2 Quality",
        DevicePropertyCode::MediaSLOT1ImageSize => "Slot 1 Size",
        DevicePropertyCode::MediaSLOT2ImageSize => "Slot 2 Size",
        DevicePropertyCode::MediaSLOT1RAWFileCompressionType => "Slot 1 RAW Comp",
        DevicePropertyCode::MediaSLOT2RAWFileCompressionType => "Slot 2 RAW Comp",
        DevicePropertyCode::MediaSLOT1QuickFormatEnableStatus => "Slot 1 Quick Fmt",
        DevicePropertyCode::MediaSLOT2QuickFormatEnableStatus => "Slot 2 Quick Fmt",
        DevicePropertyCode::MediaSLOT1FormatEnableStatus => "Slot 1 Format",
        DevicePropertyCode::MediaSLOT2FormatEnableStatus => "Slot 2 Format",
        DevicePropertyCode::MediaSLOT1WritingState => "Slot 1 Writing",
        DevicePropertyCode::MediaSLOT2WritingState => "Slot 2 Writing",
        DevicePropertyCode::MediaSLOT1Player => "Slot 1 Player",
        DevicePropertyCode::MediaSLOT2Player => "Slot 2 Player",
        DevicePropertyCode::MediaSLOT1RecordingAvailableType => "Slot 1 Rec Type",
        DevicePropertyCode::MediaSLOT2RecordingAvailableType => "Slot 2 Rec Type",
        DevicePropertyCode::MediaSLOT3RecordingAvailableType => "Slot 3 Rec Type",
        DevicePropertyCode::MediaSLOT1ContentsInfoListEnableStatus => "Slot 1 Content",
        DevicePropertyCode::MediaSLOT2ContentsInfoListEnableStatus => "Slot 2 Content",
        DevicePropertyCode::MediaSLOT1ContentsInfoListUpdateTime => "Slot 1 Updated",
        DevicePropertyCode::MediaSLOT2ContentsInfoListUpdateTime => "Slot 2 Updated",
        DevicePropertyCode::MediaSLOT1ContentsInfoListRegenerateUpdateTime => "Slot 1 Regen",
        DevicePropertyCode::MediaSLOT2ContentsInfoListRegenerateUpdateTime => "Slot 2 Regen",
        DevicePropertyCode::MediaFormatProgressRate => "Format Progress",
        DevicePropertyCode::AutoSwitchMedia => "Auto Switch",
        DevicePropertyCode::SimulRecSetting => "Simul. Rec",
        DevicePropertyCode::RemoteKeySLOTSelectButton => "Slot Select Button",
        DevicePropertyCode::PlaySetOfMultiMedia => "Multi-Media Set",
        DevicePropertyCode::CancelMediaFormatEnableStatus => "Cancel Format",
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT1 => "Delete Slot 1",
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT2 => "Delete Slot 2",
        DevicePropertyCode::PlaybackMedia => "Play Media",
        DevicePropertyCode::PresetPTZFSlotNumber => "PTZF Slot #",
        _ => code.name(),
    }
}

fn power_description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::BatteryRemain => {
            "Remaining battery capacity as a percentage. Monitor this to avoid running out during a shoot."
        }
        DevicePropertyCode::BatteryLevel => "Battery charge level indicator. Shows approximate remaining power.",
        DevicePropertyCode::BatteryRemainingInMinutes => "Estimated recording time remaining in minutes.",
        DevicePropertyCode::BatteryRemainingInVoltage => "Battery voltage reading for precise monitoring.",
        DevicePropertyCode::BatteryRemainDisplayUnit => "Unit for battery display (percentage or time).",
        DevicePropertyCode::SecondBatteryLevel => "Charge level of the second battery (grip or backup).",
        DevicePropertyCode::SecondBatteryRemain => "Remaining capacity of the second battery.",
        DevicePropertyCode::TotalBatteryLevel => "Combined charge level from all battery sources.",
        DevicePropertyCode::TotalBatteryRemain => "Total remaining capacity from all batteries.",
        DevicePropertyCode::DCVoltage => "DC power supply voltage when using external power.",
        DevicePropertyCode::PowerSource => "Current power source—internal battery, external battery grip, or AC adapter.",
        DevicePropertyCode::RecordablePowerSources => "Power sources that provide enough power for recording.",
        DevicePropertyCode::AutoPowerOffTemperature => {
            "Temperature threshold for automatic shutdown. Higher settings allow longer recording but risk overheating damage."
        }
        DevicePropertyCode::DeviceOverheatingState => {
            "Current thermal status. Warning levels indicate the camera may shut down soon to prevent damage."
        }
        DevicePropertyCode::FTPPowerSave => "Power saving mode when using FTP transfer.",
        DevicePropertyCode::CameraPowerStatus => "Current power state of the camera.",
        DevicePropertyCode::SilentModeShutterWhenPowerOff => {
            "Controls shutter blade behavior when powering off in silent mode. Close keeps sensor protected from dust."
        }
        DevicePropertyCode::AntidustShutterWhenPowerOff => {
            "Closes shutter when powering off to protect sensor from dust. Keeps sensor clean during lens changes."
        }
        DevicePropertyCode::USBPowerSupply => {
            "USB power supply settings for connected devices. Controls power delivery over USB."
        }
        _ => "",
    }
}

fn power_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::BatteryRemain => "Battery Remaining",
        DevicePropertyCode::BatteryLevel => "Batt Level",
        DevicePropertyCode::BatteryRemainingInMinutes => "Battery (Minutes)",
        DevicePropertyCode::BatteryRemainingInVoltage => "Battery Voltage",
        DevicePropertyCode::BatteryRemainDisplayUnit => "Battery Display Unit",
        DevicePropertyCode::SecondBatteryLevel => "Battery 2 Level",
        DevicePropertyCode::SecondBatteryRemain => "Battery 2 Remaining",
        DevicePropertyCode::TotalBatteryLevel => "Total Batt",
        DevicePropertyCode::TotalBatteryRemain => "Total Battery",
        DevicePropertyCode::PowerSource => "Pwr Source",
        DevicePropertyCode::AutoPowerOffTemperature => "Auto Power Off Temp",
        DevicePropertyCode::DeviceOverheatingState => "Overheating State",
        DevicePropertyCode::RecordablePowerSources => "Rec Power",
        DevicePropertyCode::USBPowerSupply => "USB Power Supply",
        DevicePropertyCode::DCVoltage => "DC Voltage",
        DevicePropertyCode::FTPPowerSave => "FTP Pwr Save",
        DevicePropertyCode::CameraPowerStatus => "Power Status",
        DevicePropertyCode::AntidustShutterWhenPowerOff => "Antidust at Power Off",
        DevicePropertyCode::SilentModeShutterWhenPowerOff => "Silent Power Off",
        _ => code.name(),
    }
}

fn zoom_description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::ZoomScale => {
            "Current zoom magnification level. Shows how much the lens is zoomed in from its widest setting."
        }
        DevicePropertyCode::ZoomSetting => {
            "Zoom behavior settings. Controls whether digital zoom is enabled and how zoom speed responds to input."
        }
        DevicePropertyCode::DigitalZoomScale => {
            "Digital zoom magnification. Extends beyond optical zoom but reduces image quality as it crops and enlarges."
        }
        DevicePropertyCode::ZoomOperation => {
            "Controls zoom movement. Use to zoom in or out programmatically."
        }
        DevicePropertyCode::ZoomOperationWithInt16 => {
            "Zoom control with 16-bit precision. Allows finer control over zoom speed and position."
        }
        DevicePropertyCode::ZoomOperationStatus => {
            "Current zoom motor status. Shows if zoom is moving, stopped, or at a limit."
        }
        DevicePropertyCode::ZoomPositionSetting => {
            "Target zoom position. Set a specific zoom level for the lens to move to."
        }
        DevicePropertyCode::ZoomPositionCurrentValue => {
            "Current zoom lens position. Shows the exact position of the zoom element."
        }
        DevicePropertyCode::ZoomDrivingStatus => {
            "Zoom motor driving state. Indicates if zoom is actively moving."
        }
        DevicePropertyCode::ZoomSpeedRange => {
            "Available zoom speed range. Defines minimum and maximum zoom motor speeds."
        }
        DevicePropertyCode::ZoomDistance => {
            "Focal length distance. Shows the current effective focal length of the zoom lens."
        }
        DevicePropertyCode::ZoomDistanceUnitSetting => {
            "Unit for displaying zoom distance. Choose between millimeters or other units."
        }
        DevicePropertyCode::ZoomBarInformation => {
            "Visual zoom bar indicator data. Used to display zoom position on screen."
        }
        DevicePropertyCode::ZoomTypeStatus => {
            "Type of zoom being used. Indicates optical, digital, or combined zoom mode."
        }
        DevicePropertyCode::DigitalExtenderMagnificationSetting => {
            "Digital extender zoom factor. Crops and enlarges the image beyond optical zoom range."
        }
        DevicePropertyCode::TeleWideLeverValueCapability => {
            "Range of values supported by the tele/wide zoom lever."
        }
        DevicePropertyCode::RemoconZoomSpeedType => {
            "Zoom speed setting for remote control. Adjusts how fast the zoom responds to remote input."
        }
        _ => "",
    }
}

fn zoom_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::ZoomScale => "Zoom",
        DevicePropertyCode::ZoomOperation => "Zoom Op",
        DevicePropertyCode::ZoomOperationWithInt16 => "Zoom Op (16-bit)",
        DevicePropertyCode::ZoomOperationStatus => "Zoom Op Status",
        DevicePropertyCode::ZoomPositionSetting => "Zoom Position",
        DevicePropertyCode::ZoomPositionCurrentValue => "Zoom Pos (Current)",
        DevicePropertyCode::ZoomDrivingStatus => "Zoom Drive Status",
        DevicePropertyCode::ZoomBarInformation => "Zoom Bar Info",
        DevicePropertyCode::ZoomTypeStatus => "Zoom Type",
        DevicePropertyCode::DigitalZoomScale => "Digital Zoom",
        DevicePropertyCode::RemoconZoomSpeedType => "Remote Zoom Speed",
        DevicePropertyCode::ZoomSetting => "Zoom Set",
        DevicePropertyCode::ZoomSpeedRange => "Zoom Speed",
        DevicePropertyCode::ZoomDistance => "Zoom Dist",
        DevicePropertyCode::ZoomDistanceUnitSetting => "Zoom Distance Unit",
        _ => code.name(),
    }
}

fn lens_description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::LensModelName => {
            "The currently attached lens model. Useful for metadata and ensuring correct lens-specific corrections."
        }
        DevicePropertyCode::LensCompensationShading => {
            "Corrects vignetting (corner darkening) caused by the lens. Automatic correction based on lens data."
        }
        DevicePropertyCode::LensCompensationChromaticAberration => {
            "Corrects color fringing at high-contrast edges. Reduces purple/green outlines caused by lens optics."
        }
        DevicePropertyCode::LensCompensationDistortion => {
            "Corrects barrel or pincushion distortion. Makes straight lines appear straight, especially with wide-angle lenses."
        }
        DevicePropertyCode::LensAssignableButton1 => {
            "Custom button on the lens that can be assigned to frequently used functions."
        }
        DevicePropertyCode::ButtonAssignmentLensAssignable1 => {
            "The function currently assigned to the lens assignable button."
        }
        DevicePropertyCode::LensSerialNumber => {
            "Serial number of the attached lens. Useful for metadata and tracking equipment."
        }
        DevicePropertyCode::LensVersionNumber => {
            "Firmware version of the attached lens. Check for updates to fix bugs or add features."
        }
        DevicePropertyCode::LensAssignableButtonIndicator1 => {
            "Status indicator showing the current function assigned to lens button 1."
        }
        DevicePropertyCode::LensInformationEnableStatus => {
            "Whether lens information (model, focal length, etc.) is available from the attached lens."
        }
        DevicePropertyCode::ReleaseWithoutLens => {
            "Allows shutter release without a lens attached. Enable for adapted lenses without electronic contacts."
        }
        DevicePropertyCode::LensCompensationBreathing => {
            "Compensates for focus breathing where focal length shifts during focusing. Keeps framing consistent when pulling focus in video."
        }
        _ => "",
    }
}

fn lens_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::LensModelName => "Lens Model",
        DevicePropertyCode::LensCompensationShading => "Lens Comp: Shading",
        DevicePropertyCode::LensCompensationChromaticAberration => "Lens Comp: Chromatic",
        DevicePropertyCode::LensCompensationDistortion => "Lens Comp: Distortion",
        DevicePropertyCode::LensCompensationBreathing => "Lens Comp: Breathing",
        DevicePropertyCode::FocalDistanceInMeter => "Focal Distance (m)",
        DevicePropertyCode::FocalDistanceInFeet => "Focal Distance (ft)",
        DevicePropertyCode::FocalDistanceUnitSetting => "Focal Distance Unit",
        DevicePropertyCode::LensAssignableButton1 => "Lens Btn",
        DevicePropertyCode::ButtonAssignmentLensAssignable1 => "Assign Lens Btn",
        DevicePropertyCode::LensAssignableButtonIndicator1 => "Lens Button 1",
        DevicePropertyCode::LensInformationEnableStatus => "Lens Info Status",
        DevicePropertyCode::LensSerialNumber => "Lens Serial #",
        DevicePropertyCode::LensVersionNumber => "Lens Version",
        DevicePropertyCode::ReleaseWithoutLens => "Lensless Release",
        _ => code.name(),
    }
}

fn audio_description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::AudioRecording => {
            "Enables or disables audio recording with video. Turn off when using external audio recorders."
        }
        DevicePropertyCode::AudioSignals => {
            "Camera beeps for focus confirmation and self-timer. Disable for quiet shooting environments."
        }
        DevicePropertyCode::AudioSignalsStartEnd => {
            "Audio signals at the start and end of recording. Helps confirm recording status."
        }
        DevicePropertyCode::AudioSignalsVolume => {
            "Volume level for camera beeps and audio signals."
        }
        DevicePropertyCode::AudioLevelDisplay => {
            "Shows audio input levels on screen. Essential for monitoring recording quality."
        }
        DevicePropertyCode::WindNoiseReduct => {
            "Reduces low-frequency wind noise in the built-in microphone. May slightly affect audio quality."
        }
        DevicePropertyCode::AudioInput1TypeSelect | DevicePropertyCode::AudioInput2TypeSelect => {
            "Selects the type of audio input for this connector (XLR, line, etc.)."
        }
        DevicePropertyCode::AudioInputCH1InputSelect
        | DevicePropertyCode::AudioInputCH2InputSelect
        | DevicePropertyCode::AudioInputCH3InputSelect
        | DevicePropertyCode::AudioInputCH4InputSelect => {
            "Selects the audio source for this channel (internal mic, external, line in)."
        }
        DevicePropertyCode::AudioInputCH1Level
        | DevicePropertyCode::AudioInputCH2Level
        | DevicePropertyCode::AudioInputCH3Level
        | DevicePropertyCode::AudioInputCH4Level
        | DevicePropertyCode::AudioInputMasterLevel => {
            "Audio recording level for this channel. Adjust to avoid clipping or too-quiet recordings."
        }
        DevicePropertyCode::AudioInputCH1LevelControl
        | DevicePropertyCode::AudioInputCH2LevelControl
        | DevicePropertyCode::AudioInputCH3LevelControl
        | DevicePropertyCode::AudioInputCH4LevelControl => {
            "How audio level is controlled (auto, manual). Auto adjusts dynamically; manual gives precise control."
        }
        DevicePropertyCode::AudioInputCH1WindFilter
        | DevicePropertyCode::AudioInputCH2WindFilter
        | DevicePropertyCode::AudioInputCH3WindFilter
        | DevicePropertyCode::AudioInputCH4WindFilter => {
            "Wind noise filter for this audio channel. Reduces rumble from wind but may affect bass response."
        }
        DevicePropertyCode::AudioStreamBitDepth => {
            "Bit depth for audio streaming. Higher values capture more dynamic range."
        }
        DevicePropertyCode::AudioStreamChannel => "Number of audio channels for streaming (mono, stereo, etc.).",
        DevicePropertyCode::AudioStreamCodecType => "Audio codec used for streaming. Different codecs have different quality and bandwidth tradeoffs.",
        DevicePropertyCode::AudioStreamSamplingFrequency => {
            "Audio sample rate for streaming. Higher rates capture more high-frequency detail."
        }
        _ => "",
    }
}

fn audio_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::AudioRecording => "Audio Rec",
        DevicePropertyCode::AudioSignals => "Beep",
        DevicePropertyCode::AudioSignalsStartEnd => "Rec Start/End Beep",
        DevicePropertyCode::AudioSignalsVolume => "Beep Volume",
        DevicePropertyCode::AudioLevelDisplay => "Audio Meter",
        DevicePropertyCode::WindNoiseReduct => "Wind Noise Reduct.",
        DevicePropertyCode::AudioInput1TypeSelect => "Input 1 Type",
        DevicePropertyCode::AudioInput2TypeSelect => "Input 2 Type",
        DevicePropertyCode::AudioInputCH1InputSelect => "CH1 Input",
        DevicePropertyCode::AudioInputCH2InputSelect => "CH2 Input",
        DevicePropertyCode::AudioInputCH3InputSelect => "CH3 Input",
        DevicePropertyCode::AudioInputCH4InputSelect => "CH4 Input",
        DevicePropertyCode::AudioInputCH1Level => "CH1 Level",
        DevicePropertyCode::AudioInputCH2Level => "CH2 Level",
        DevicePropertyCode::AudioInputCH3Level => "CH3 Level",
        DevicePropertyCode::AudioInputCH4Level => "CH4 Level",
        DevicePropertyCode::AudioInputMasterLevel => "Master Level",
        DevicePropertyCode::AudioInputCH1LevelControl => "CH1 Level Ctrl",
        DevicePropertyCode::AudioInputCH2LevelControl => "CH2 Level Ctrl",
        DevicePropertyCode::AudioInputCH3LevelControl => "CH3 Level Ctrl",
        DevicePropertyCode::AudioInputCH4LevelControl => "CH4 Level Ctrl",
        DevicePropertyCode::AudioInputCH1WindFilter => "CH1 Wind Filter",
        DevicePropertyCode::AudioInputCH2WindFilter => "CH2 Wind Filter",
        DevicePropertyCode::AudioInputCH3WindFilter => "CH3 Wind Filter",
        DevicePropertyCode::AudioInputCH4WindFilter => "CH4 Wind Filter",
        DevicePropertyCode::AudioStreamBitDepth => "Stream Bit Depth",
        DevicePropertyCode::AudioStreamChannel => "Stream Channels",
        DevicePropertyCode::AudioStreamCodecType => "Stream Codec",
        DevicePropertyCode::AudioStreamSamplingFrequency => "Stream Sample Rate",
        _ => code.name(),
    }
}

fn picture_profile_description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::PictureProfile => {
            "Preset color, gamma, and detail settings for video. PP1-PP10 are customizable. Off uses standard processing."
        }
        DevicePropertyCode::PictureProfileGamma
        | DevicePropertyCode::PictureProfileBlackGammaLevel
        | DevicePropertyCode::PictureProfileBlackGammaRange => {
            "Gamma curve settings. Controls how brightness values are mapped for different contrast and dynamic range."
        }
        DevicePropertyCode::PictureProfileColorMode => {
            "Color processing mode. Different modes optimize for various shooting scenarios."
        }
        DevicePropertyCode::PictureProfileSaturation
        | DevicePropertyCode::PictureProfileColorPhase => {
            "Color saturation and hue adjustments for the picture profile."
        }
        DevicePropertyCode::PictureProfileBlackLevel => {
            "Adjusts the level of black in the image. Affects shadow detail and contrast."
        }
        DevicePropertyCode::PictureProfileKneeMode
        | DevicePropertyCode::PictureProfileKneeAutoSetMaxPoint
        | DevicePropertyCode::PictureProfileKneeAutoSetSensitivity
        | DevicePropertyCode::PictureProfileKneeManualSetPoint
        | DevicePropertyCode::PictureProfileKneeManualSetSlope => {
            "Knee settings control highlight compression. Prevents clipping in bright areas."
        }
        DevicePropertyCode::PictureProfileDetailLevel
        | DevicePropertyCode::PictureProfileDetailAdjustMode
        | DevicePropertyCode::PictureProfileDetailAdjustVHBalance
        | DevicePropertyCode::PictureProfileDetailAdjustBWBalance
        | DevicePropertyCode::PictureProfileDetailAdjustLimit
        | DevicePropertyCode::PictureProfileDetailAdjustCrispening
        | DevicePropertyCode::PictureProfileDetailAdjustHiLightDetail => {
            "Detail/sharpening settings. Controls edge enhancement and texture rendering."
        }
        DevicePropertyCode::PictureProfileColorDepthRed
        | DevicePropertyCode::PictureProfileColorDepthGreen
        | DevicePropertyCode::PictureProfileColorDepthBlue
        | DevicePropertyCode::PictureProfileColorDepthCyan
        | DevicePropertyCode::PictureProfileColorDepthMagenta
        | DevicePropertyCode::PictureProfileColorDepthYellow => {
            "Per-color saturation adjustments. Fine-tune individual color channels."
        }
        DevicePropertyCode::PictureProfileCopy => {
            "Copies picture profile settings to another profile slot."
        }
        DevicePropertyCode::PictureProfileResetEnableStatus => {
            "Indicates whether the picture profile can be reset to defaults."
        }
        DevicePropertyCode::CreativeLook => {
            "Camera-designed color looks for stills and video. Each style (ST, PT, NT, etc.) has a distinct aesthetic."
        }
        DevicePropertyCode::CreativeLookContrast
        | DevicePropertyCode::CreativeLookHighlights
        | DevicePropertyCode::CreativeLookShadows
        | DevicePropertyCode::CreativeLookSaturation
        | DevicePropertyCode::CreativeLookSharpness
        | DevicePropertyCode::CreativeLookSharpnessRange
        | DevicePropertyCode::CreativeLookClarity
        | DevicePropertyCode::CreativeLookFade => {
            "Fine-tuning parameter for the creative look. Adjusts the overall style characteristics."
        }
        DevicePropertyCode::CreativeLookCustomLook => {
            "Custom creative look that can be loaded from a file."
        }
        DevicePropertyCode::CreativeLookResetEnableStatus => {
            "Indicates whether the creative look can be reset to defaults."
        }
        DevicePropertyCode::GammaDisplayAssist => {
            "Shows a preview of how log footage will look after color grading. Helps expose correctly without flat-looking preview."
        }
        DevicePropertyCode::GammaDisplayAssistType => {
            "Type of gamma assist preview. Different LUT presets for various color workflows."
        }
        DevicePropertyCode::BaseLookValue | DevicePropertyCode::BaseLookAppliedofPlayback => {
            "Base look setting that defines the fundamental color characteristics before other adjustments."
        }
        DevicePropertyCode::BaseLookNameofPlayback => {
            "Name of the base look applied during playback."
        }
        DevicePropertyCode::BaseLookImportOperationEnableStatus => {
            "Indicates whether base look import is available."
        }
        DevicePropertyCode::PictureEffect => {
            "Creative filters applied to images in-camera. Includes toy camera, posterization, etc."
        }
        DevicePropertyCode::LogShootingModeColorGamut => {
            "Color gamut for log shooting. S-Gamut3 provides maximum color range for grading."
        }
        DevicePropertyCode::ColorSpace => {
            "Color space for images. sRGB for web/print. AdobeRGB for wider gamut professional workflows."
        }
        DevicePropertyCode::EstimatePictureSize => {
            "Estimated file size for the current image quality settings."
        }
        _ => "",
    }
}

fn picture_profile_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::PictureProfile => "Pict. Profile",
        DevicePropertyCode::PictureProfileGamma => "PP Gamma",
        DevicePropertyCode::PictureProfileColorMode => "PP Color Mode",
        DevicePropertyCode::PictureProfileSaturation => "PP Saturation",
        DevicePropertyCode::PictureProfileBlackLevel => "PP Black Level",
        DevicePropertyCode::PictureProfileBlackGammaLevel => "PP Black Gamma",
        DevicePropertyCode::PictureProfileBlackGammaRange => "PP BG Range",
        DevicePropertyCode::PictureProfileKneeMode => "PP Knee Mode",
        DevicePropertyCode::PictureProfileKneeAutoSetMaxPoint => "PP Knee Auto Max",
        DevicePropertyCode::PictureProfileKneeAutoSetSensitivity => "PP Knee Auto Sens",
        DevicePropertyCode::PictureProfileKneeManualSetPoint => "PP Knee Point",
        DevicePropertyCode::PictureProfileKneeManualSetSlope => "PP Knee Slope",
        DevicePropertyCode::PictureProfileDetailLevel => "PP Detail Level",
        DevicePropertyCode::PictureProfileDetailAdjustMode => "PP Detail Mode",
        DevicePropertyCode::PictureProfileDetailAdjustVHBalance => "PP Detail V/H",
        DevicePropertyCode::PictureProfileDetailAdjustBWBalance => "PP Detail B/W",
        DevicePropertyCode::PictureProfileDetailAdjustLimit => "PP Detail Limit",
        DevicePropertyCode::PictureProfileDetailAdjustCrispening => "PP Crisp",
        DevicePropertyCode::PictureProfileDetailAdjustHiLightDetail => "PP HiLight Detail",
        DevicePropertyCode::PictureProfileColorPhase => "PP Color Phase",
        DevicePropertyCode::PictureProfileColorDepthRed => "PP Depth: Red",
        DevicePropertyCode::PictureProfileColorDepthGreen => "PP Depth: Green",
        DevicePropertyCode::PictureProfileColorDepthBlue => "PP Depth: Blue",
        DevicePropertyCode::PictureProfileColorDepthCyan => "PP Depth: Cyan",
        DevicePropertyCode::PictureProfileColorDepthMagenta => "PP Depth: Magenta",
        DevicePropertyCode::PictureProfileColorDepthYellow => "PP Depth: Yellow",
        DevicePropertyCode::PictureProfileCopy => "PP Copy",
        DevicePropertyCode::PictureProfileResetEnableStatus => "PP Reset Avail",
        DevicePropertyCode::CreativeLook => "Creat. Look",
        DevicePropertyCode::CreativeLookContrast => "CL: Contrast",
        DevicePropertyCode::CreativeLookHighlights => "CL: Highlights",
        DevicePropertyCode::CreativeLookShadows => "CL: Shadows",
        DevicePropertyCode::CreativeLookSaturation => "CL: Saturation",
        DevicePropertyCode::CreativeLookSharpness => "CL: Sharpness",
        DevicePropertyCode::CreativeLookSharpnessRange => "CL: Sharp Range",
        DevicePropertyCode::CreativeLookClarity => "CL: Clarity",
        DevicePropertyCode::CreativeLookFade => "CL: Fade",
        DevicePropertyCode::CreativeLookCustomLook => "CL: Custom",
        DevicePropertyCode::CreativeLookResetEnableStatus => "CL Reset Avail",
        DevicePropertyCode::GammaDisplayAssist => "Gamma Assist",
        DevicePropertyCode::GammaDisplayAssistType => "Gamma Assist Type",
        DevicePropertyCode::BaseLookValue => "Base Look",
        DevicePropertyCode::BaseLookAppliedofPlayback => "Base Look (Play)",
        DevicePropertyCode::BaseLookNameofPlayback => "Base Look Name",
        DevicePropertyCode::BaseLookImportOperationEnableStatus => "Base Look Import",
        DevicePropertyCode::PictureEffect => "Pict. Effect",
        DevicePropertyCode::ColorSpace => "Clr Space",
        DevicePropertyCode::EstimatePictureSize => "Est. Picture Size",
        DevicePropertyCode::LogShootingModeColorGamut => "Log Gamut",
        _ => code.name(),
    }
}

fn nd_filter_description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::NDFilter => {
            "Built-in neutral density filter. Reduces light entering the lens, allowing wider apertures or slower shutters in bright conditions."
        }
        DevicePropertyCode::NDFilterMode => {
            "ND filter behavior. Auto engages as needed. Manual gives direct control. Variable ND allows smooth adjustment."
        }
        DevicePropertyCode::NDFilterModeSetting => {
            "Setting for ND filter mode. Auto, manual, or variable ND options."
        }
        DevicePropertyCode::NDFilterValue => {
            "Current ND filter value/strength. Higher values block more light."
        }
        DevicePropertyCode::NDFilterSwitchingSetting => {
            "Controls how the ND filter switches between states."
        }
        DevicePropertyCode::NDFilterPositionSetting => {
            "Position of the variable ND filter. Adjusts the amount of light reduction."
        }
        DevicePropertyCode::NDFilterOpticalDensityValue => {
            "Optical density of the ND filter. Measured in stops of light reduction."
        }
        DevicePropertyCode::NDFilterUnitSetting => {
            "Display unit for ND filter values (stops, optical density, etc.)."
        }
        DevicePropertyCode::NDFilterPresetSelect => {
            "Selects which ND filter preset to use (1, 2, or 3)."
        }
        DevicePropertyCode::NDFilterPreset1Value
        | DevicePropertyCode::NDFilterPreset2Value
        | DevicePropertyCode::NDFilterPreset3Value => {
            "Stored ND filter value for quick recall."
        }
        DevicePropertyCode::ManualInputForNDFilterValue => {
            "Manual entry of a specific ND filter value."
        }
        DevicePropertyCode::PushAutoNDFilter => {
            "Temporarily engages auto ND filter while button is pressed."
        }
        _ => "",
    }
}

fn nd_filter_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::NDFilter => "ND Filter",
        DevicePropertyCode::NDFilterMode => "ND Mode",
        DevicePropertyCode::NDFilterModeSetting => "ND Mode Setting",
        DevicePropertyCode::NDFilterValue => "ND Value",
        DevicePropertyCode::NDFilterSwitchingSetting => "ND Switching",
        DevicePropertyCode::NDFilterPositionSetting => "ND Position",
        DevicePropertyCode::NDFilterOpticalDensityValue => "ND Density",
        DevicePropertyCode::NDFilterUnitSetting => "ND Unit",
        DevicePropertyCode::NDFilterPresetSelect => "ND Preset",
        DevicePropertyCode::NDFilterPreset1Value => "ND Preset 1",
        DevicePropertyCode::NDFilterPreset2Value => "ND Preset 2",
        DevicePropertyCode::NDFilterPreset3Value => "ND Preset 3",
        DevicePropertyCode::ManualInputForNDFilterValue => "ND Manual Input",
        DevicePropertyCode::PushAutoNDFilter => "Push Auto ND",
        _ => code.name(),
    }
}

fn display_description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::LiveViewDisplayEffect => {
            "Shows exposure, white balance, and creative look effects in the viewfinder/LCD. Turn off for consistent preview brightness."
        }
        DevicePropertyCode::GridLineDisplay => {
            "Overlays composition guides on the screen. Rule of thirds, square grid, or diagonal lines help frame shots."
        }
        DevicePropertyCode::GridLineType => {
            "Choose grid pattern type: rule of thirds, square grid, diagonal lines, etc."
        }
        DevicePropertyCode::MonitorBrightnessType => {
            "Screen brightness setting. Manual sets a fixed level. Auto adjusts based on ambient light. Sunny Weather boosts for outdoors."
        }
        DevicePropertyCode::MonitorBrightnessManual => {
            "Manual brightness level for the LCD monitor."
        }
        DevicePropertyCode::LiveViewStatus => {
            "Current state of live view output. Shows if preview is active."
        }
        DevicePropertyCode::LiveViewImageQuality => {
            "Quality setting for live view stream. Higher quality uses more bandwidth."
        }
        DevicePropertyCode::LiveViewProtocol => {
            "Protocol used for live view streaming (USB, network, etc.)."
        }
        DevicePropertyCode::DispMode | DevicePropertyCode::DispModeSetting => {
            "Display mode configuration for screen layout and information overlays."
        }
        DevicePropertyCode::DispModeCandidate => {
            "Available display modes for the current shooting mode."
        }
        DevicePropertyCode::SelectFinder => {
            "Switch between LCD monitor and electronic viewfinder."
        }
        DevicePropertyCode::DisplayQualityFinder => {
            "Display quality setting for the electronic viewfinder."
        }
        DevicePropertyCode::MonitorLUTSetting => {
            "LUT applied to monitor output for color preview."
        }
        DevicePropertyCode::DisplayedMenuStatus => {
            "Current menu display state."
        }
        DevicePropertyCode::GammaDisplayAssist => {
            "Applies a preview LUT to show how log footage will look after color grading. Does not affect recorded video."
        }
        DevicePropertyCode::GammaDisplayAssistType => {
            "Type of display assist LUT to apply for preview."
        }
        DevicePropertyCode::PlaybackContentsGammaType => {
            "Gamma curve type of the content being played back."
        }
        DevicePropertyCode::DeSqueezeDisplayRatio => {
            "Stretches anamorphic footage horizontally for proper preview. Set to match your anamorphic lens squeeze factor (1.33x, 1.8x, 2x)."
        }
        DevicePropertyCode::GridLineDisplayPlayback => {
            "Shows grid line overlays during image and video playback for composition review."
        }
        DevicePropertyCode::HDMIResolutionStillPlay => {
            "HDMI output resolution when playing back still images. Match to your monitor for best quality."
        }
        DevicePropertyCode::LiveViewImageQualityByNumericalValue => {
            "Specific quality value for live view streaming. Higher values provide clearer preview but use more bandwidth."
        }
        DevicePropertyCode::OSDImageMode => {
            "On-screen display image mode. Controls overlay information shown on external monitors via HDMI/SDI."
        }
        DevicePropertyCode::TCUBDisplaySetting => {
            "Timecode and user bit display configuration. Controls how TC/UB information appears on screen and outputs."
        }
        DevicePropertyCode::StreamDisplayName => {
            "Display name shown for the stream on receiving devices."
        }
        DevicePropertyCode::MonitoringTransportProtocol => {
            "Network protocol for monitoring delivery (NDI, SRT, etc.)."
        }
        DevicePropertyCode::MonitoringDeliveryTypeSupportInfo => {
            "Supported delivery types for monitoring output."
        }
        DevicePropertyCode::MonitorLUTSetting1
        | DevicePropertyCode::MonitorLUTSetting2
        | DevicePropertyCode::MonitorLUTSetting3 => {
            "LUT applied to this monitoring output for color preview."
        }
        DevicePropertyCode::MonitorLUTSettingOutputDestAssign => {
            "Assigns LUT settings to monitoring output destinations."
        }
        DevicePropertyCode::MonitoringAvailableFormat => {
            "Video formats available for monitoring output."
        }
        DevicePropertyCode::MonitoringOutputDisplaySettingDestAssign => {
            "Assigns display settings to monitoring output destinations."
        }
        DevicePropertyCode::MonitoringOutputDisplayHDMI => {
            "Display settings for HDMI monitoring output."
        }
        DevicePropertyCode::MonitoringOutputDisplaySDI => {
            "Display settings for SDI monitoring output."
        }
        DevicePropertyCode::MonitoringOutputDisplaySetting1
        | DevicePropertyCode::MonitoringOutputDisplaySetting2 => {
            "Monitoring output display configuration preset."
        }
        DevicePropertyCode::MonitoringOutputFormat => {
            "Output format for monitoring signal (resolution, frame rate)."
        }
        DevicePropertyCode::MonitoringIsDelivering => {
            "Whether monitoring output is actively being delivered."
        }
        DevicePropertyCode::MonitoringDeliveringStatus => {
            "Current status of monitoring delivery to external devices."
        }
        DevicePropertyCode::MonitoringFormatSupportInformation => {
            "Detailed format support information for monitoring."
        }
        DevicePropertyCode::MonitoringSettingVersion => {
            "Version of the monitoring settings format."
        }
        DevicePropertyCode::FaceEyeFrameDisplay => {
            "Shows or hides the frame overlay around detected faces and eyes."
        }
        _ => "",
    }
}

fn display_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::LiveViewDisplayEffect => "LV Effect",
        DevicePropertyCode::LiveViewStatus => "LV Status",
        DevicePropertyCode::DispMode => "Display",
        DevicePropertyCode::DispModeSetting => "Display Setting",
        DevicePropertyCode::MonitorLUTSetting => "Monitor LUT",
        DevicePropertyCode::MonitorLUTSetting1 => "Monitor LUT 1",
        DevicePropertyCode::MonitorLUTSetting2 => "Monitor LUT 2",
        DevicePropertyCode::MonitorLUTSetting3 => "Monitor LUT 3",
        DevicePropertyCode::MonitorLUTSettingOutputDestAssign => "LUT Output Assign",
        DevicePropertyCode::GridLineDisplay => "Grid Lines",
        DevicePropertyCode::MonitorBrightnessType => "Monitor Bright.",
        DevicePropertyCode::MonitorBrightnessManual => "Monitor Bright. (M)",
        DevicePropertyCode::DisplayedMenuStatus => "Menu Status",
        DevicePropertyCode::SelectFinder => "Finder Select",
        DevicePropertyCode::DispModeCandidate => "Disp Mode Options",
        DevicePropertyCode::GammaDisplayAssist => "Gamma Disp Assist",
        DevicePropertyCode::GammaDisplayAssistType => "Gamma Assist Type",
        DevicePropertyCode::DisplayQualityFinder => "Finder Quality",
        DevicePropertyCode::PlaybackContentsGammaType => "Playback Gamma",
        DevicePropertyCode::MonitoringAvailableFormat => "Available Formats",
        DevicePropertyCode::MonitoringDeliveringStatus => "Delivery Status",
        DevicePropertyCode::MonitoringDeliveryTypeSupportInfo => "Delivery Type Info",
        DevicePropertyCode::MonitoringFormatSupportInformation => "Format Support Info",
        DevicePropertyCode::MonitoringIsDelivering => "Is Delivering",
        DevicePropertyCode::MonitoringOutputDisplayHDMI => "HDMI Display",
        DevicePropertyCode::MonitoringOutputDisplaySDI => "SDI Display",
        DevicePropertyCode::MonitoringOutputDisplaySetting1 => "Display Setting 1",
        DevicePropertyCode::MonitoringOutputDisplaySetting2 => "Display Setting 2",
        DevicePropertyCode::MonitoringOutputDisplaySettingDestAssign => "Display Dest Assign",
        DevicePropertyCode::MonitoringOutputFormat => "Output Format",
        DevicePropertyCode::MonitoringSettingVersion => "Setting Version",
        DevicePropertyCode::MonitoringTransportProtocol => "Transport Protocol",
        DevicePropertyCode::StreamDisplayName => "Stream Name",
        DevicePropertyCode::DeSqueezeDisplayRatio => "Desqueeze Display Ratio",
        DevicePropertyCode::FaceEyeFrameDisplay => "Face/Eye Frame Display",
        DevicePropertyCode::GridLineDisplayPlayback => "Grid Line Playback",
        DevicePropertyCode::TCUBDisplaySetting => "TC/UB Disp",
        _ => code.name(),
    }
}

fn stabilization_description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::ImageStabilizationSteadyShot => {
            "In-body image stabilization (IBIS). Compensates for camera shake, allowing slower shutter speeds when handheld."
        }
        DevicePropertyCode::MovieImageStabilizationSteadyShot => {
            "Stabilization mode for video. Active mode is more aggressive but slightly crops the image."
        }
        DevicePropertyCode::ImageStabilizationSteadyShotAdjust => {
            "Fine-tune IBIS correction strength for specific shooting conditions."
        }
        DevicePropertyCode::ImageStabilizationSteadyShotFocalLength => {
            "Manual focal length input for SteadyShot. Set when using adapted lenses that don't report focal length."
        }
        DevicePropertyCode::ImageStabilizationFramingStabilizer => {
            "Framing stabilizer maintains frame position during video. Reduces shifts between cuts."
        }
        _ => "",
    }
}

fn stabilization_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::ImageStabilizationSteadyShot => "SteadyShot",
        DevicePropertyCode::MovieImageStabilizationSteadyShot => "Movie SteadyShot",
        DevicePropertyCode::MovieImageStabilizationLevel => "SteadyShot Level",
        DevicePropertyCode::ImageStabilizationSteadyShotAdjust => "SteadyShot Adjust",
        DevicePropertyCode::ImageStabilizationSteadyShotFocalLength => "SteadyShot Focal Length",
        DevicePropertyCode::ImageStabilizationFramingStabilizer => "Framing Stabilizer",
        _ => code.name(),
    }
}

fn silent_description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::SilentMode => {
            "Disables all mechanical sounds and lights. Uses electronic shutter and turns off AF illuminator and flash. Essential for weddings, wildlife, and theaters."
        }
        DevicePropertyCode::SilentModeApertureDriveInAF => {
            "Controls aperture motor noise during AF in silent mode. Not Target ignores this setting. Standard balances speed and noise. Silent Priority minimizes noise but may slow AF."
        }
        DevicePropertyCode::SilentModeAutoPixelMapping => {
            "Controls automatic pixel mapping (hot pixel correction) behavior in silent mode. May be disabled to avoid mechanical noise."
        }
        DevicePropertyCode::SilentModeShutterWhenPowerOff => {
            "Controls shutter blade behavior when powering off in silent mode. Close keeps sensor protected from dust."
        }
        DevicePropertyCode::AntidustShutterWhenPowerOff => {
            "Closes shutter when powering off to protect sensor from dust. Keeps sensor clean during lens changes."
        }
        _ => "",
    }
}

fn silent_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::SilentMode => "Silent",
        DevicePropertyCode::SilentModeApertureDriveInAF => "Silent: Aperture Drive",
        DevicePropertyCode::SilentModeShutterWhenPowerOff => "Silent: Shutter Power Off",
        DevicePropertyCode::SilentModeAutoPixelMapping => "Silent: Auto Pixel Mapping",
        _ => code.name(),
    }
}

fn other_description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::DRO => {
            "Dynamic Range Optimizer. Automatically adjusts shadows and highlights to preserve detail in high-contrast scenes."
        }
        DevicePropertyCode::MeteredManualLevel => {
            "Exposure meter reading in manual mode. Shows how current settings compare to metered exposure."
        }
        DevicePropertyCode::StreamModeSetting => {
            "Configures the camera for live streaming output. Sets up video format and connection type for streaming platforms."
        }
        DevicePropertyCode::FTPFunction => {
            "Enables FTP file transfer. Automatically uploads images to a configured FTP server over WiFi."
        }
        DevicePropertyCode::WakeOnLAN => {
            "Allows the camera to be powered on remotely via network signal. Useful for remote camera setups."
        }
        DevicePropertyCode::ModelName => "Camera model identifier. Useful for confirming the connected device.",
        DevicePropertyCode::BodySerialNumber => {
            "Unique camera serial number. Important for registration, insurance, and tracking ownership."
        }
        DevicePropertyCode::SoftwareVersion => "Current firmware version. Check for updates to get new features and bug fixes.",
        DevicePropertyCode::IntervalRecMode => {
            "Captures images at set intervals for time-lapse photography. The camera can optionally compile them into a video."
        }
        DevicePropertyCode::IntervalRecNumberOfShots => {
            "Total number of shots to capture during interval shooting. Set to 0 or unlimited for continuous capture until stopped."
        }
        DevicePropertyCode::IntervalRecShootingInterval => {
            "Time between each shot in interval shooting. Shorter intervals capture faster motion; longer intervals for slow changes."
        }
        DevicePropertyCode::IntervalRecShootingStartTime => {
            "Delay before interval shooting begins. Allows time to leave the scene or wait for a specific moment."
        }
        DevicePropertyCode::IntervalRecStatus => {
            "Current state of interval recording. Shows if shooting is active, paused, or complete."
        }
        DevicePropertyCode::IntervalRecShootIntervalPriority => {
            "When enabled, prioritizes maintaining the interval timing even if it requires shorter exposures."
        }
        DevicePropertyCode::IntervalRecAETrackingSensitivity => {
            "How quickly exposure adjusts between interval shots. Low for gradual changes like sunsets. High for faster changes."
        }
        DevicePropertyCode::ContinuousShootingSpotBoostStatus => {
            "Shows if continuous shooting speed boost is active for the current focus area."
        }
        DevicePropertyCode::ContinuousShootingSpotBoostFrameSpeed => {
            "The boosted frame rate when spot boost is active. Higher speeds capture more frames per second."
        }
        DevicePropertyCode::ContinuousShootingSpotBoostEnableStatus => {
            "Indicates whether the spot boost feature can be enabled in the current shooting mode."
        }
        DevicePropertyCode::AspectRatio => {
            "Image shape. 3:2 is standard for full-frame. 16:9 is widescreen. 1:1 is square. 4:3 matches micro four-thirds sensors."
        }
        DevicePropertyCode::SoftSkinEffect => {
            "Smooths skin tones in portraits. Higher settings provide more smoothing."
        }
        DevicePropertyCode::RecorderMainStatus => {
            "Current status of the main video recorder. Shows if recording is active, paused, or stopped."
        }
        DevicePropertyCode::AssignableButton1
        | DevicePropertyCode::AssignableButton2
        | DevicePropertyCode::AssignableButton3
        | DevicePropertyCode::AssignableButton4
        | DevicePropertyCode::AssignableButton5
        | DevicePropertyCode::AssignableButton6
        | DevicePropertyCode::AssignableButton7
        | DevicePropertyCode::AssignableButton8
        | DevicePropertyCode::AssignableButton9
        | DevicePropertyCode::AssignableButton10
        | DevicePropertyCode::AssignableButton11
        | DevicePropertyCode::LensAssignableButton1 => {
            "Custom button on the camera body that can be assigned to frequently used functions."
        }
        DevicePropertyCode::AssignableButtonIndicator1
        | DevicePropertyCode::AssignableButtonIndicator2
        | DevicePropertyCode::AssignableButtonIndicator3
        | DevicePropertyCode::AssignableButtonIndicator4
        | DevicePropertyCode::AssignableButtonIndicator5
        | DevicePropertyCode::AssignableButtonIndicator6
        | DevicePropertyCode::AssignableButtonIndicator7
        | DevicePropertyCode::AssignableButtonIndicator8
        | DevicePropertyCode::AssignableButtonIndicator9
        | DevicePropertyCode::AssignableButtonIndicator10
        | DevicePropertyCode::AssignableButtonIndicator11 => {
            "Shows the current state of the assignable button indicator (active/inactive)."
        }
        DevicePropertyCode::ButtonAssignmentAssignable1
        | DevicePropertyCode::ButtonAssignmentAssignable2
        | DevicePropertyCode::ButtonAssignmentAssignable3
        | DevicePropertyCode::ButtonAssignmentAssignable4
        | DevicePropertyCode::ButtonAssignmentAssignable5
        | DevicePropertyCode::ButtonAssignmentAssignable6
        | DevicePropertyCode::ButtonAssignmentAssignable7
        | DevicePropertyCode::ButtonAssignmentAssignable8
        | DevicePropertyCode::ButtonAssignmentAssignable9
        | DevicePropertyCode::ButtonAssignmentAssignable10
        | DevicePropertyCode::ButtonAssignmentAssignable11
        | DevicePropertyCode::ButtonAssignmentLensAssignable1 => {
            "The function currently assigned to this custom button."
        }
        // BaseLook properties route to Other
        DevicePropertyCode::BaseLookValue | DevicePropertyCode::BaseLookAppliedofPlayback => {
            "Base look setting that defines the fundamental color characteristics before other adjustments."
        }
        DevicePropertyCode::BaseLookNameofPlayback => {
            "Name of the base look applied during playback."
        }
        DevicePropertyCode::BaseLookImportOperationEnableStatus => {
            "Indicates whether base look import is available."
        }
        DevicePropertyCode::TeleWideLeverValueCapability => {
            "Range of values supported by the tele/wide zoom lever."
        }
        DevicePropertyCode::SimulRecSetting => {
            "Records to both card slots simultaneously. Provides instant backup of every shot."
        }
        DevicePropertyCode::GridLineType => {
            "Choose grid pattern type: rule of thirds, square grid, diagonal lines, etc."
        }
        DevicePropertyCode::LiveViewStatus => {
            "Current state of live view output. Shows if preview is active."
        }
        DevicePropertyCode::LiveViewProtocol => {
            "Protocol used for live view streaming (USB, network, etc.)."
        }
        // Camera system properties
        DevicePropertyCode::CameraPowerStatus => "Current power state of the camera.",
        DevicePropertyCode::CameraOperatingMode => "Current operating mode (photo, video, playback).",
        DevicePropertyCode::CameraErrorCautionStatus => "Error or warning status from the camera.",
        DevicePropertyCode::CameraSystemErrorInfo => "Detailed system error information.",
        DevicePropertyCode::CameraShakeStatus => "Camera shake detection status for blur warning.",
        DevicePropertyCode::CameraSettingReadOperationEnableStatus => {
            "Whether camera settings can be read from file."
        }
        DevicePropertyCode::CameraSettingSaveOperationEnableStatus => {
            "Whether camera settings can be saved to file."
        }
        DevicePropertyCode::CameraSettingsResetEnableStatus => {
            "Whether factory reset is available."
        }
        DevicePropertyCode::SdkControlMode => "SDK control mode for remote operation.",
        DevicePropertyCode::BodyKeyLock => "Locks physical buttons to prevent accidental changes.",
        DevicePropertyCode::DCVoltage => "DC power supply voltage when using external power.",
        // FTP and file transfer (non-Movie category)
        DevicePropertyCode::FTPConnectionStatus => "Current FTP server connection state.",
        DevicePropertyCode::FTPConnectionErrorInfo => "FTP connection error details.",
        DevicePropertyCode::FTPAutoTransfer => "Automatically transfers files to FTP server.",
        DevicePropertyCode::FTPAutoTransferTarget => "Which files to auto-transfer (all, selected).",
        DevicePropertyCode::FTPAutoTransferTargetStillImage => "Auto-transfer setting for still images.",
        DevicePropertyCode::FTPTransferTarget => "Target files for manual FTP transfer.",
        DevicePropertyCode::FTPTransferStillImageQualitySize => "Quality/size setting for FTP transfers.",
        DevicePropertyCode::FTPServerSettingOperationEnableStatus => "Whether FTP settings can be modified.",
        DevicePropertyCode::FTPServerSettingVersion => "FTP server settings version.",
        DevicePropertyCode::FTPTransferSettingReadOperationEnableStatus => {
            "Whether FTP transfer settings can be read."
        }
        DevicePropertyCode::FTPTransferSettingSaveOperationEnableStatus => {
            "Whether FTP transfer settings can be saved."
        }
        DevicePropertyCode::FTPTransferSettingSaveReadState => "Current FTP transfer settings state.",
        DevicePropertyCode::FTPJobListDataVersion => "Version of the FTP job list data.",
        DevicePropertyCode::SelectFTPServer => "Select which configured FTP server to use.",
        DevicePropertyCode::SelectFTPServerID => "ID of the selected FTP server.",
        DevicePropertyCode::ProtectImageInFTPTransfer => "Protect transferred images from deletion.",
        // Subject recognition (AI detection)
        DevicePropertyCode::SubjectRecognitionAF => "AI-powered subject detection for autofocus.",
        DevicePropertyCode::SubjectRecognitionInAF => "Subject recognition mode during AF.",
        DevicePropertyCode::SubjectRecognitionAnimalBirdPriority => {
            "Priority between animal and bird detection."
        }
        DevicePropertyCode::SubjectRecognitionAnimalBirdDetectionParts => {
            "Which body parts to detect on animals/birds (eye, body, head)."
        }
        DevicePropertyCode::SubjectRecognitionAnimalDetectionParts => "Body parts to detect on animals.",
        DevicePropertyCode::SubjectRecognitionAnimalDetectionSensitivity => {
            "Sensitivity for animal detection. Higher values detect smaller subjects."
        }
        DevicePropertyCode::SubjectRecognitionAnimalTrackingSensitivity => {
            "How aggressively to track detected animals."
        }
        DevicePropertyCode::SubjectRecognitionAnimalTrackingSubjectShiftRange => {
            "Range for tracking animal subjects across the frame."
        }
        DevicePropertyCode::SubjectRecognitionBirdDetectionParts => "Body parts to detect on birds.",
        DevicePropertyCode::SubjectRecognitionBirdDetectionSensitivity => {
            "Sensitivity for bird detection."
        }
        DevicePropertyCode::SubjectRecognitionBirdTrackingSensitivity => {
            "How aggressively to track detected birds."
        }
        DevicePropertyCode::SubjectRecognitionBirdTrackingSubjectShiftRange => {
            "Range for tracking bird subjects across the frame."
        }
        DevicePropertyCode::SubjectRecognitionInsectDetectionSensitivity => {
            "Sensitivity for insect detection."
        }
        DevicePropertyCode::SubjectRecognitionInsectTrackingSensitivity => {
            "How aggressively to track detected insects."
        }
        DevicePropertyCode::SubjectRecognitionInsectTrackingSubjectShiftRange => {
            "Range for tracking insect subjects across the frame."
        }
        DevicePropertyCode::SubjectRecognitionCarTrainDetectionSensitivity => {
            "Sensitivity for car/train detection."
        }
        DevicePropertyCode::SubjectRecognitionCarTrainTrackingSensitivity => {
            "How aggressively to track detected vehicles."
        }
        DevicePropertyCode::SubjectRecognitionCarTrainTrackingSubjectShiftRange => {
            "Range for tracking vehicle subjects across the frame."
        }
        DevicePropertyCode::SubjectRecognitionPlaneDetectionSensitivity => {
            "Sensitivity for airplane detection."
        }
        DevicePropertyCode::SubjectRecognitionPlaneTrackingSensitivity => {
            "How aggressively to track detected airplanes."
        }
        DevicePropertyCode::SubjectRecognitionPlaneTrackingSubjectShiftRange => {
            "Range for tracking airplane subjects across the frame."
        }
        DevicePropertyCode::SubjectRecognitionPersonTrackingSubjectShiftRange => {
            "Range for tracking person subjects across the frame."
        }
        DevicePropertyCode::SubjectRecognitionPriorityOnRegisteredFace => {
            "Prioritize registered faces over other subjects."
        }
        // APS-C and sensor mode
        DevicePropertyCode::APSCOrFullSwitchingEnableStatus => {
            "Whether switching between APS-C and full-frame crop is available."
        }
        DevicePropertyCode::APSCOrFullSwitchingSetting => {
            "Choose between full-frame or APS-C crop mode. APS-C extends reach but reduces resolution."
        }
        DevicePropertyCode::APSCS35 => {
            "APS-C/Super 35mm mode. Crops to APS-C size sensor area for extra reach or to use APS-C lenses."
        }
        // Auto review and preview
        DevicePropertyCode::AutoReview => {
            "Shows captured image briefly after shooting. Set duration or disable to maximize shooting speed."
        }
        // Cinematic Vlog settings
        DevicePropertyCode::CinematicVlogSetting => "Cinematic vlog video style preset.",
        DevicePropertyCode::CinematicVlogLook => "Visual look/style for cinematic vlog mode.",
        DevicePropertyCode::CinematicVlogMood => "Color mood preset for cinematic vlog.",
        DevicePropertyCode::CinematicVlogAFTransitionSpeed => {
            "How quickly focus transitions in cinematic vlog mode. Slower is more cinematic."
        }
        // Contents transfer
        DevicePropertyCode::ContentsTransferStatus => "Current state of file transfer operation.",
        DevicePropertyCode::ContentsTransferProgress => "Progress percentage of current file transfer.",
        DevicePropertyCode::ContentsTransferCancelEnableStatus => {
            "Whether the current file transfer can be cancelled."
        }
        // Grid and display
        DevicePropertyCode::CreateNewFolder => "Creates a new folder on the memory card.",
        DevicePropertyCode::CreateNewFolderEnableStatus => "Whether new folder creation is available.",
        DevicePropertyCode::CurrentSceneFileEdited => "Indicates if current scene file has unsaved changes.",
        DevicePropertyCode::CustomGridLineFileCommandVersion => "Version of custom grid line file format.",
        // Depth of field settings
        DevicePropertyCode::DepthOfFieldAdjustmentMode => {
            "Mode for adjusting depth of field preview during shooting."
        }
        DevicePropertyCode::DepthOfFieldAdjustmentInterlockingMode => {
            "Links depth of field preview to other camera settings."
        }
        // De-squeeze display
        DevicePropertyCode::DeSqueezeDisplayRatio => {
            "Display aspect correction for anamorphic lenses. Shows unsqueezed preview."
        }
        // Difference settings
        DevicePropertyCode::DifferentSetForSQMovie => {
            "Use different settings for S&Q (slow & quick) movie mode than regular video."
        }
        // Eframing (auto-framing) settings
        DevicePropertyCode::EframingAutoFraming => "Automatic subject framing using AI detection.",
        DevicePropertyCode::EframingModeAuto => "Automatic mode selection for eframing feature.",
        DevicePropertyCode::EframingProductionEffect => "Production effects applied during auto-framing.",
        DevicePropertyCode::EframingHDMICrop => "HDMI output crop settings for eframing.",
        DevicePropertyCode::EframingRecordingImageCrop => "Image crop settings for eframing recording.",
        DevicePropertyCode::EframingScaleAuto => "Automatic scaling for auto-framing.",
        DevicePropertyCode::EframingSpeedAuto => "Automatic speed adjustment for framing transitions.",
        DevicePropertyCode::EframingSpeedPTZ => "Pan-tilt-zoom speed for eframing movements.",
        DevicePropertyCode::EframingTrackingStartMode => "How auto-framing begins tracking subjects.",
        DevicePropertyCode::EframingType => "Type of auto-framing behavior.",
        DevicePropertyCode::EframingCommandVersion => "Eframing protocol version.",
        // Extended features
        DevicePropertyCode::ExtendedInterfaceMode => "Extended interface mode for advanced control.",
        DevicePropertyCode::EmbedLUTFile => "Embeds LUT file data in recorded video.",
        DevicePropertyCode::EnlargeScreenSetting => "Settings for enlarged screen view.",
        DevicePropertyCode::EstimatePictureSize => "Estimated file size for current image settings.",
        // Model and identification
        DevicePropertyCode::AreaTimeZoneSettingVersion => "Version of timezone settings format.",
        DevicePropertyCode::CallSetting => "Quick recall of saved camera settings.",
        // AEL and exposure lock
        DevicePropertyCode::AEL => "Auto Exposure Lock. Locks the current exposure settings.",
        // Auto recognition
        DevicePropertyCode::AutoRecognitionTargetCandidates => {
            "Available targets for automatic subject recognition."
        }
        DevicePropertyCode::AutoRecognitionTargetSetting => {
            "Selected target type for automatic subject recognition."
        }
        // Camera lever
        DevicePropertyCode::CameraEframing => "Camera electronic framing mode setting.",
        DevicePropertyCode::CameraLeverFunction => "Function assigned to the camera's lever control.",
        // Content operations
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT1 => {
            "Whether content deletion is available for slot 1."
        }
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT2 => {
            "Whether content deletion is available for slot 2."
        }
        DevicePropertyCode::DeleteUserBaseLook => "Delete a user-created base look preset.",
        // Control and interface
        DevicePropertyCode::ControlForHDMI => "HDMI control settings (HDMI-CEC).",
        DevicePropertyCode::DebugMode => "Debug mode for troubleshooting.",
        // Touch and remote
        DevicePropertyCode::CancelRemoteTouchOperationEnableStatus => {
            "Whether remote touch operations can be cancelled."
        }
        DevicePropertyCode::FunctionOfRemoteTouchOperation => "Function activated by remote touch.",
        // Flicker
        DevicePropertyCode::FlickerLessShooting => {
            "Reduces banding from flickering artificial light sources."
        }
        DevicePropertyCode::FlickerLessShootingStatus => "Current flicker reduction status.",
        DevicePropertyCode::FlickerScanEnableStatus => "Whether flicker scan is available.",
        DevicePropertyCode::FlickerScanStatus => "Current flicker scan detection status.",
        // Focal distance display
        DevicePropertyCode::FocalDistanceInFeet => "Shows focus distance in feet.",
        DevicePropertyCode::FocalDistanceInMeter => "Shows focus distance in meters.",
        DevicePropertyCode::FocalDistanceUnitSetting => "Unit for displaying focus distance.",
        // File numbering
        DevicePropertyCode::ForcedFileNumberResetEnableStatus => {
            "Whether file number reset is available."
        }
        // Focus assist
        DevicePropertyCode::FullTimeDMF => {
            "Direct Manual Focus always available even in AF mode."
        }
        // Get only (read-only flag)
        DevicePropertyCode::GetOnly => "Read-only property flag.",
        // Moved from NDFilter (was incorrectly categorized due to "nd" substring)
        DevicePropertyCode::WindNoiseReduct => {
            "Reduces low-frequency wind noise in the built-in microphone. May slightly affect audio quality."
        }
        DevicePropertyCode::DigitalExtenderMagnificationSetting => {
            "Digital extender zoom factor. Crops and enlarges the image beyond optical zoom range."
        }
        // Paint Look settings (professional video color control)
        DevicePropertyCode::PaintLookAutoKnee => {
            "Automatic knee point adjustment. Compresses highlights to prevent clipping."
        }
        DevicePropertyCode::PaintLookBBlack => "Blue channel black level adjustment.",
        DevicePropertyCode::PaintLookRBlack => "Red channel black level adjustment.",
        DevicePropertyCode::PaintLookMasterBlack => {
            "Overall black level adjustment. Affects all color channels equally."
        }
        DevicePropertyCode::PaintLookDetailLevel => {
            "Amount of detail/sharpening applied to the image."
        }
        DevicePropertyCode::PaintLookDetailSetting => "Detail enhancement settings and options.",
        DevicePropertyCode::PaintLookKneePoint => {
            "Highlight compression threshold. Point at which highlights begin to be compressed."
        }
        DevicePropertyCode::PaintLookKneeSetting => {
            "Knee compression mode and settings for highlight handling."
        }
        DevicePropertyCode::PaintLookKneeSlope => {
            "Rate of highlight compression above the knee point."
        }
        DevicePropertyCode::PaintLookMultiMatrixAreaIndication => {
            "Display indicator for multi-matrix color correction areas."
        }
        DevicePropertyCode::PaintLookMultiMatrixAxis => {
            "Color axis selection for multi-matrix adjustments."
        }
        DevicePropertyCode::PaintLookMultiMatrixHue => {
            "Hue adjustment for selected multi-matrix color axis."
        }
        DevicePropertyCode::PaintLookMultiMatrixSaturation => {
            "Saturation adjustment for selected multi-matrix color axis."
        }
        DevicePropertyCode::PaintLookMultiMatrixSetting => "Multi-matrix color correction settings.",
        DevicePropertyCode::PaintLookUserMatrixBG => "User matrix blue-green cross-coupling.",
        DevicePropertyCode::PaintLookUserMatrixBR => "User matrix blue-red cross-coupling.",
        DevicePropertyCode::PaintLookUserMatrixGB => "User matrix green-blue cross-coupling.",
        DevicePropertyCode::PaintLookUserMatrixGR => "User matrix green-red cross-coupling.",
        DevicePropertyCode::PaintLookUserMatrixRB => "User matrix red-blue cross-coupling.",
        DevicePropertyCode::PaintLookUserMatrixRG => "User matrix red-green cross-coupling.",
        DevicePropertyCode::PaintLookUserMatrixLevel => "Overall level of user matrix adjustment.",
        DevicePropertyCode::PaintLookUserMatrixPhase => "Phase adjustment for user color matrix.",
        DevicePropertyCode::PaintLookUserMatrixSetting => "User-defined color matrix settings.",
        // Pan/Tilt (PTZ camera control)
        DevicePropertyCode::PanLimitMode => "Mode for pan movement limits.",
        DevicePropertyCode::PanLimitRangeMaximum => "Maximum pan angle limit.",
        DevicePropertyCode::PanLimitRangeMinimum => "Minimum pan angle limit.",
        DevicePropertyCode::PanPositionCurrentValue => "Current pan position of the camera head.",
        DevicePropertyCode::PanPositionStatus => "Status of pan position control.",
        DevicePropertyCode::PanTiltAccelerationRampCurve => {
            "Acceleration curve for pan/tilt movements. Smoother curves for cinematic motion."
        }
        DevicePropertyCode::TiltLimitMode => "Mode for tilt movement limits.",
        DevicePropertyCode::TiltLimitRangeMaximum => "Maximum tilt angle limit.",
        DevicePropertyCode::TiltLimitRangeMinimum => "Minimum tilt angle limit.",
        DevicePropertyCode::TiltPositionCurrentValue => "Current tilt position of the camera head.",
        DevicePropertyCode::TiltPositionStatus => "Status of tilt position control.",
        // Monitoring and stream settings
        DevicePropertyCode::MonitoringAvailableFormat => {
            "Available video formats for external monitoring output."
        }
        DevicePropertyCode::MonitoringDeliveringStatus => "Current monitoring stream delivery status.",
        DevicePropertyCode::MonitoringDeliveryTypeSupportInfo => {
            "Supported monitoring delivery protocols and types."
        }
        DevicePropertyCode::MonitoringFormatSupportInformation => {
            "Supported formats for monitoring output."
        }
        DevicePropertyCode::MonitoringIsDelivering => "Whether monitoring stream is currently active.",
        DevicePropertyCode::MonitoringOutputDisplayHDMI => "HDMI monitoring output display settings.",
        DevicePropertyCode::MonitoringOutputDisplaySDI => "SDI monitoring output display settings.",
        DevicePropertyCode::MonitoringOutputDisplaySetting1 => "Primary monitoring display settings.",
        DevicePropertyCode::MonitoringOutputDisplaySetting2 => "Secondary monitoring display settings.",
        DevicePropertyCode::MonitoringOutputDisplaySettingDestAssign => {
            "Assignment of monitoring display settings to outputs."
        }
        DevicePropertyCode::MonitoringOutputFormat => "Video format for monitoring output.",
        DevicePropertyCode::MonitoringSettingVersion => "Version of monitoring settings format.",
        DevicePropertyCode::MonitoringTransportProtocol => "Network protocol for monitoring stream.",
        DevicePropertyCode::MonitorLUTSetting1 => "First LUT slot for monitoring output.",
        DevicePropertyCode::MonitorLUTSetting2 => "Second LUT slot for monitoring output.",
        DevicePropertyCode::MonitorLUTSetting3 => "Third LUT slot for monitoring output.",
        DevicePropertyCode::MonitorLUTSettingOutputDestAssign => {
            "LUT assignment to monitoring outputs."
        }
        // Stream settings
        DevicePropertyCode::StreamButtonEnableStatus => {
            "Whether stream start button is enabled."
        }
        DevicePropertyCode::StreamCipherType => "Encryption type for secure streaming.",
        DevicePropertyCode::StreamDisplayName => "Display name for the stream.",
        DevicePropertyCode::StreamLatency => "Stream latency mode (low, normal, etc.).",
        DevicePropertyCode::StreamSettingListOperationStatus => {
            "Status of stream settings list operations."
        }
        DevicePropertyCode::StreamStatus => "Current streaming status.",
        DevicePropertyCode::StreamTTL => "Time-to-live for stream packets.",
        DevicePropertyCode::TargetStreamingDestinationSelect => {
            "Select target destination for streaming."
        }
        // Playback properties
        DevicePropertyCode::PlaySetOfMultiMedia => "Select media set for multi-media playback.",
        DevicePropertyCode::PlaybackContentsName => "Name of currently playing content.",
        DevicePropertyCode::PlaybackContentsNumber => "Number/index of current playback content.",
        DevicePropertyCode::PlaybackContentsRecordingDateTime => {
            "Recording date and time of current playback content."
        }
        DevicePropertyCode::PlaybackContentsRecordingFileFormat => {
            "File format of current playback content."
        }
        DevicePropertyCode::PlaybackContentsRecordingFrameRate => {
            "Frame rate of current playback content."
        }
        DevicePropertyCode::PlaybackContentsRecordingResolution => {
            "Resolution of current playback content."
        }
        DevicePropertyCode::PlaybackContentsTotalNumber => "Total number of playback contents.",
        DevicePropertyCode::PlaybackMedia => "Media slot used for playback.",
        DevicePropertyCode::PlaybackViewMode => "View mode for playback (single, thumbnail, etc.).",
        DevicePropertyCode::PlaybackVolumeSettings => "Audio volume settings for playback.",
        DevicePropertyCode::GridLineDisplayPlayback => "Show grid lines during playback.",
        DevicePropertyCode::HDMIResolutionStillPlay => {
            "HDMI output resolution during still image playback."
        }
        // Time Shift recording
        DevicePropertyCode::TimeShiftPreShootingTimeSetting => {
            "Buffer time before trigger in time shift recording."
        }
        DevicePropertyCode::TimeShiftShooting => "Time shift recording mode setting.",
        DevicePropertyCode::TimeShiftShootingStatus => "Current time shift recording status.",
        DevicePropertyCode::TimeShiftTriggerSetting => "Trigger setting for time shift recording.",
        // Tally lamp
        DevicePropertyCode::TallyLampControlGreen => "Green tally lamp control.",
        DevicePropertyCode::TallyLampControlRed => "Red tally lamp control.",
        DevicePropertyCode::TallyLampControlYellow => "Yellow tally lamp control.",
        // User Bit settings (video metadata)
        DevicePropertyCode::UserBitPreset => "Preset value for user bit metadata in video.",
        DevicePropertyCode::UserBitPresetResetEnableStatus => {
            "Whether user bit reset is available."
        }
        DevicePropertyCode::UserBitTimeRec => "User bit time recording mode.",
        DevicePropertyCode::TCUBDisplaySetting => "Timecode/User Bit display settings.",
        DevicePropertyCode::TimeCodePresetResetEnableStatus => {
            "Whether timecode preset reset is available."
        }
        // User Base Look
        DevicePropertyCode::UserBaseLookAELevelOffset => {
            "Auto-exposure level offset for user base look."
        }
        DevicePropertyCode::UserBaseLookInput => "Input setting for user base look.",
        DevicePropertyCode::UserBaseLookOutput => "Output setting for user base look.",
        DevicePropertyCode::SelectUserBaseLookToEdit => "Select user base look to edit.",
        DevicePropertyCode::SelectUserBaseLookToSetInPPLUT => {
            "Select user base look to set in picture profile LUT."
        }
        // Video Stream settings
        DevicePropertyCode::VideoStreamAdaptiveRateControl => {
            "Adaptive bitrate control for video streaming."
        }
        DevicePropertyCode::VideoStreamBitRateCompressionMode => {
            "Bitrate compression mode (CBR, VBR, etc.)."
        }
        DevicePropertyCode::VideoStreamBitRateVBRMode => "VBR mode settings for video stream.",
        DevicePropertyCode::VideoStreamCodec => "Video codec used for streaming.",
        DevicePropertyCode::VideoStreamMaxBitRate => "Maximum bitrate for video streaming.",
        DevicePropertyCode::VideoStreamMovieRecPermission => {
            "Permission for movie recording during streaming."
        }
        DevicePropertyCode::VideoStreamResolution => "Resolution for video streaming.",
        DevicePropertyCode::VideoStreamResolutionMethod => {
            "Method for determining stream resolution."
        }
        DevicePropertyCode::VideoStreamSelect => "Select video stream configuration.",
        DevicePropertyCode::VideoStreamSettingVersion => "Version of video stream settings.",
        DevicePropertyCode::VideoRecordingFormatBitrateSetting => {
            "Bitrate setting for video recording format."
        }
        DevicePropertyCode::VideoRecordingFormatQuality => "Quality level for video recording.",
        DevicePropertyCode::ValidRecordingVideoFormat => {
            "List of valid video formats for recording."
        }
        // Scene file settings
        DevicePropertyCode::SceneFileCommandVersion => "Scene file command protocol version.",
        DevicePropertyCode::SceneFileDownloadOperationEnableStatus => {
            "Whether scene file download is available."
        }
        DevicePropertyCode::SceneFileIndex => "Index of current scene file.",
        DevicePropertyCode::SceneFileIndexesAvailableForDownload => {
            "Scene file indexes available for download."
        }
        DevicePropertyCode::SceneFileUploadOperationEnableStatus => {
            "Whether scene file upload is available."
        }
        // Miscellaneous camera settings
        DevicePropertyCode::AmountOfDefocusSetting => {
            "Amount of intentional defocus for creative effects."
        }
        DevicePropertyCode::AntidustShutterWhenPowerOff => {
            "Closes shutter when powering off to protect sensor from dust."
        }
        DevicePropertyCode::ApertureDriveInAF => "Aperture behavior during autofocus.",
        DevicePropertyCode::CameraButtonFunction => "Function assigned to camera button.",
        DevicePropertyCode::CameraButtonFunctionMulti => "Multi-function camera button settings.",
        DevicePropertyCode::CameraButtonFunctionStatus => "Current camera button function status.",
        DevicePropertyCode::CameraDialFunction => "Function assigned to camera dial.",
        DevicePropertyCode::CameraSettingSaveReadState => "State of camera setting save/read.",
        DevicePropertyCode::CancelMediaFormatEnableStatus => {
            "Whether media format can be cancelled."
        }
        DevicePropertyCode::ColorSpace => "Color space setting (sRGB, Adobe RGB, etc.).",
        DevicePropertyCode::DateTimeSettings => "Camera date and time settings.",
        DevicePropertyCode::ElectricFrontCurtainShutter => {
            "Uses electronic shutter for first curtain. Reduces shutter shock."
        }
        DevicePropertyCode::FEL => "Flash Exposure Lock. Locks flash output level.",
        DevicePropertyCode::FirmwareUpdateCommandVersion => "Firmware update protocol version.",
        DevicePropertyCode::FunctionOfTouchOperation => "Function assigned to touch operation.",
        DevicePropertyCode::HighResolutionShutterSpeed => "High-resolution shutter speed value.",
        DevicePropertyCode::HighResolutionShutterSpeedAdjust => {
            "Fine adjustment for high-resolution shutter speed."
        }
        DevicePropertyCode::HighResolutionShutterSpeedAdjustInIntegralMultiples => {
            "Shutter speed adjustment in integral multiples."
        }
        DevicePropertyCode::HighResolutionShutterSpeedSetting => {
            "Settings for high-resolution shutter speed mode."
        }
        DevicePropertyCode::HomeMenuSetting => "Home menu configuration.",
        DevicePropertyCode::IRRemoteSetting => "Infrared remote control settings.",
        DevicePropertyCode::IPSetupProtocolSetting => "IP address setup protocol (DHCP, static).",
        DevicePropertyCode::ImageIDNum => "Numeric image identifier.",
        DevicePropertyCode::ImageIDNumSetting => "Image ID number setting.",
        DevicePropertyCode::ImageIDString => "String image identifier.",
        DevicePropertyCode::ImagerScanMode => "Image sensor scanning mode.",
        DevicePropertyCode::LanguageSetting => "Camera menu language setting.",
        DevicePropertyCode::LensAssignableButtonIndicator1 => {
            "Indicator for lens assignable button."
        }
        DevicePropertyCode::LensCompensationBreathing => {
            "Compensates for focus breathing (focal length shift during focusing)."
        }
        DevicePropertyCode::LensInformationEnableStatus => {
            "Whether lens information is available."
        }
        DevicePropertyCode::LensSerialNumber => "Lens serial number.",
        DevicePropertyCode::LensVersionNumber => "Lens firmware version.",
        DevicePropertyCode::LiveViewArea => "Area of the frame shown in live view.",
        DevicePropertyCode::LiveViewImageQualityByNumericalValue => {
            "Live view image quality as numeric value."
        }
        DevicePropertyCode::MaxVal => "Maximum value property.",
        DevicePropertyCode::MaximumNumberOfBytes => "Maximum bytes for data transfer.",
        DevicePropertyCode::MaximumSizeOfImageIDString => "Maximum length of image ID string.",
        DevicePropertyCode::MicrophoneDirectivity => {
            "Microphone pickup pattern (omni, directional, etc.)."
        }
        DevicePropertyCode::OSDImageMode => "On-screen display image mode.",
        DevicePropertyCode::PictureCacheRecSetting => "Picture cache recording settings.",
        DevicePropertyCode::PictureCacheRecSizeAndTime => {
            "Size and time settings for picture cache recording."
        }
        DevicePropertyCode::PixelMappingEnableStatus => "Whether pixel mapping is available.",
        DevicePropertyCode::PostViewTransferResourceStatus => {
            "Status of post-view image transfer resources."
        }
        DevicePropertyCode::PresetPTZFSlotNumber => "PTZ preset slot number.",
        DevicePropertyCode::PriorityKeySettings => "Priority key configuration.",
        DevicePropertyCode::ProductShowcaseSet => "Product showcase mode settings.",
        DevicePropertyCode::ProgramShiftStatus => "Status of program shift in P mode.",
        DevicePropertyCode::PushAGC => "Push automatic gain control.",
        DevicePropertyCode::PushAutoIris => "Push automatic iris control.",
        DevicePropertyCode::RAWJPCSaveImage => "Save both RAW and JPEG together.",
        DevicePropertyCode::RecognitionTarget => "Target type for subject recognition.",
        DevicePropertyCode::RedEyeReduction => "Reduces red-eye effect in flash photos.",
        DevicePropertyCode::ReleaseWithoutCard => "Allow shutter release without memory card.",
        DevicePropertyCode::ReleaseWithoutLens => "Allow shutter release without lens attached.",
        DevicePropertyCode::RemoteKeySLOTSelectButton => "Remote key slot select button.",
        DevicePropertyCode::RemoteKeyThumbnailButton => "Remote key thumbnail button.",
        DevicePropertyCode::RemoteTouchOperation => "Remote touch operation mode.",
        DevicePropertyCode::RemoteTouchOperationEnableStatus => {
            "Whether remote touch operation is available."
        }
        DevicePropertyCode::RightLeftEyeSelect => "Select left or right eye for eye AF.",
        DevicePropertyCode::S1 => "S1 shutter button half-press action.",
        DevicePropertyCode::S2 => "S2 shutter button full-press action.",
        DevicePropertyCode::SensorCleaningEnableStatus => "Whether sensor cleaning is available.",
        DevicePropertyCode::SetCopyright => "Set copyright information in image metadata.",
        DevicePropertyCode::SetPhotographer => "Set photographer name in image metadata.",
        DevicePropertyCode::SetPresetPTZFBinaryVersion => "PTZ preset binary version.",
        DevicePropertyCode::ShootingEnableSettingLicense => "Shooting license settings.",
        DevicePropertyCode::ShootingTimingPreNotificationMode => {
            "Pre-notification mode for shooting timing."
        }
        DevicePropertyCode::SilentModeApertureDriveInAF => {
            "Silent aperture drive during autofocus."
        }
        DevicePropertyCode::SilentModeAutoPixelMapping => {
            "Silent mode for automatic pixel mapping."
        }
        DevicePropertyCode::SilentModeShutterWhenPowerOff => {
            "Silent shutter operation when powering off."
        }
        DevicePropertyCode::SimulRecSettingMovieRecButton => {
            "Movie record button behavior for simultaneous recording."
        }
        DevicePropertyCode::SnapshotInfo => "Snapshot information.",
        DevicePropertyCode::StillImageTransSize => "Transfer size for still images.",
        DevicePropertyCode::SynchroterminalForcedOutput => {
            "Forced output through sync terminal."
        }
        DevicePropertyCode::SystemErrorCautionStatus => "System error and caution status.",
        DevicePropertyCode::TNumber => "T-number (transmission-based f-number).",
        DevicePropertyCode::TopOfTheGroupShootingMarkSetting => {
            "Marks first shot of each burst group."
        }
        DevicePropertyCode::TouchFunctionInMF => "Touch function in manual focus mode.",
        DevicePropertyCode::TouchOperation => "Touch screen operation mode.",
        DevicePropertyCode::TrackingOnAndAFOnEnableStatus => {
            "Whether tracking and AF-On can be combined."
        }
        DevicePropertyCode::USBPowerSupply => "USB power supply settings.",
        DevicePropertyCode::Undefined => "Undefined property.",
        DevicePropertyCode::UpdateBodyStatus => "Camera body update status.",
        DevicePropertyCode::UploadDatasetVersion => "Upload dataset version.",
        DevicePropertyCode::WriteCopyrightInfo => "Write copyright information to files.",
        DevicePropertyCode::SelectIPTCMetadata => "Select IPTC metadata preset.",
        DevicePropertyCode::FaceEyeDetectionAFStatus => {
            "Shows current status of face/eye detection. Indicates whether faces or eyes are currently detected."
        }
        DevicePropertyCode::PreAF => {
            "Camera continuously adjusts focus before half-pressing shutter. Faster initial focus but uses more battery."
        }
        DevicePropertyCode::PushAFModeSetting => {
            "Configures behavior of Push AF button. Can be set to focus once, focus hold, or other modes."
        }
        DevicePropertyCode::LogShootingMode => {
            "Enables log gamma curves (S-Log2, S-Log3) for maximum dynamic range. Requires color grading."
        }
        DevicePropertyCode::SQModeSetting => {
            "Slow & Quick mode settings. Enables high frame rate capture for slow motion effects."
        }
        DevicePropertyCode::RecorderStartMain => "Starts recording on the main recorder slot.",
        DevicePropertyCode::RecorderClipName => "Current clip name for the recording.",
        DevicePropertyCode::RecorderControlMainSetting => {
            "Recording control settings for the main recorder slot."
        }
        DevicePropertyCode::RecorderSaveDestination => {
            "Destination for saving recorded footage. Selects memory card slot or external recorder."
        }
        _ => "",
    }
}

fn other_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::PushAGC => "Push Gain",
        DevicePropertyCode::DRO => "D-Range Optimizer",
        DevicePropertyCode::MeteredManualLevel => "Meter Level (M)",
        DevicePropertyCode::StreamStatus => "Strm Status",
        DevicePropertyCode::StreamModeSetting => "Stream Mode",
        DevicePropertyCode::StreamDisplayName => "Stream Name",
        DevicePropertyCode::StreamLatency => "Strm Latency",
        DevicePropertyCode::StreamButtonEnableStatus => "Stream Button Status",
        DevicePropertyCode::StreamCipherType => "Stream Cipher",
        DevicePropertyCode::StreamSettingListOperationStatus => "Stream Settings Status",
        DevicePropertyCode::StreamTTL => "Strm TTL",
        DevicePropertyCode::TargetStreamingDestinationSelect => "Stream Destination",
        DevicePropertyCode::VideoStreamAdaptiveRateControl => "Adaptive Rate Control",
        DevicePropertyCode::VideoStreamBitRateCompressionMode => "Bitrate Compression",
        DevicePropertyCode::VideoStreamBitRateVBRMode => "VBR Mode",
        DevicePropertyCode::VideoStreamCodec => "Video Codec",
        DevicePropertyCode::VideoStreamMaxBitRate => "Max Bitrate",
        DevicePropertyCode::VideoStreamResolution => "Stream Resolution",
        DevicePropertyCode::VideoStreamResolutionMethod => "Resolution Method",
        DevicePropertyCode::VideoStreamSelect => "Stream Select",
        DevicePropertyCode::VideoStreamSettingVersion => "Stream Settings Ver",
        DevicePropertyCode::FTPFunction => "FTP Func",
        DevicePropertyCode::FTPAutoTransfer => "FTP Auto Xfer",
        DevicePropertyCode::FTPConnectionStatus => "FTP Connection",
        DevicePropertyCode::WakeOnLAN => "Wake on LAN",
        DevicePropertyCode::IPSetupProtocolSetting => "IP Setup Protocol",
        DevicePropertyCode::AssignableButton1 => "Btn C1",
        DevicePropertyCode::AssignableButton2 => "Btn C2",
        DevicePropertyCode::AssignableButton3 => "Btn C3",
        DevicePropertyCode::AssignableButton4 => "Btn C4",
        DevicePropertyCode::AssignableButton5 => "Btn C5",
        DevicePropertyCode::AssignableButton6 => "Btn C6",
        DevicePropertyCode::AssignableButton7 => "Btn C7",
        DevicePropertyCode::AssignableButton8 => "Btn C8",
        DevicePropertyCode::AssignableButton9 => "Btn C9",
        DevicePropertyCode::AssignableButton10 => "Btn C10",
        DevicePropertyCode::AssignableButton11 => "Btn C11",
        DevicePropertyCode::AssignableButtonIndicator1 => "Btn Ind 1",
        DevicePropertyCode::AssignableButtonIndicator2 => "Btn Ind 2",
        DevicePropertyCode::AssignableButtonIndicator3 => "Btn Ind 3",
        DevicePropertyCode::AssignableButtonIndicator4 => "Btn Ind 4",
        DevicePropertyCode::AssignableButtonIndicator5 => "Btn Ind 5",
        DevicePropertyCode::AssignableButtonIndicator6 => "Btn Ind 6",
        DevicePropertyCode::AssignableButtonIndicator7 => "Btn Ind 7",
        DevicePropertyCode::AssignableButtonIndicator8 => "Btn Ind 8",
        DevicePropertyCode::AssignableButtonIndicator9 => "Btn Ind 9",
        DevicePropertyCode::AssignableButtonIndicator10 => "Btn Ind 10",
        DevicePropertyCode::AssignableButtonIndicator11 => "Btn Ind 11",
        DevicePropertyCode::ButtonAssignmentAssignable1 => "Assign C1",
        DevicePropertyCode::ButtonAssignmentAssignable2 => "Assign C2",
        DevicePropertyCode::ButtonAssignmentAssignable3 => "Assign C3",
        DevicePropertyCode::ButtonAssignmentAssignable4 => "Assign C4",
        DevicePropertyCode::ButtonAssignmentAssignable5 => "Assign C5",
        DevicePropertyCode::ButtonAssignmentAssignable6 => "Assign C6",
        DevicePropertyCode::ButtonAssignmentAssignable7 => "Assign C7",
        DevicePropertyCode::ButtonAssignmentAssignable8 => "Assign C8",
        DevicePropertyCode::ButtonAssignmentAssignable9 => "Assign C9",
        DevicePropertyCode::ButtonAssignmentAssignable10 => "Assign C10",
        DevicePropertyCode::ButtonAssignmentAssignable11 => "Assign C11",
        DevicePropertyCode::ButtonAssignmentLensAssignable1 => "Assign Lens",
        DevicePropertyCode::LensAssignableButton1 => "Lens Btn",
        DevicePropertyCode::CameraButtonFunction => "Button Function",
        DevicePropertyCode::CameraButtonFunctionMulti => "Multi Button Func",
        DevicePropertyCode::CameraButtonFunctionStatus => "Button Func Status",
        DevicePropertyCode::CameraDialFunction => "Dial Function",
        DevicePropertyCode::RemoteKeyThumbnailButton => "Thumbnail Button",
        DevicePropertyCode::FunctionOfTouchOperation => "Touch Function",
        DevicePropertyCode::TouchFunctionInMF => "Touch Fn (MF)",
        DevicePropertyCode::TouchOperation => "Touch Op",
        DevicePropertyCode::ModelName => "Camera Model",
        DevicePropertyCode::BodySerialNumber => "Serial Number",
        DevicePropertyCode::SoftwareVersion => "Firmware Version",
        DevicePropertyCode::DateTimeSettings => "Date/Time",
        DevicePropertyCode::LanguageSetting => "Language",
        DevicePropertyCode::CameraOperatingMode => "Operating Mode",
        DevicePropertyCode::CameraSettingSaveReadState => "Settings State",
        DevicePropertyCode::IntervalRecMode => "Interval Rec",
        DevicePropertyCode::IntervalRecNumberOfShots => "Interval #",
        DevicePropertyCode::IntervalRecShootingInterval => "Interval Time",
        DevicePropertyCode::IntervalRecShootingStartTime => "Interval Start",
        DevicePropertyCode::IntervalRecStatus => "Interval State",
        DevicePropertyCode::IntervalRecShootIntervalPriority => "Interval Priority",
        DevicePropertyCode::IntervalRecAETrackingSensitivity => "Interval AE Track",
        DevicePropertyCode::ContinuousShootingSpotBoostStatus => "Burst Boost",
        DevicePropertyCode::ContinuousShootingSpotBoostFrameSpeed => "Burst Boost FPS",
        DevicePropertyCode::ContinuousShootingSpotBoostEnableStatus => "Burst Boost Avail",
        DevicePropertyCode::AspectRatio => "Aspect",
        DevicePropertyCode::SoftSkinEffect => "Soft Skin",
        DevicePropertyCode::RecorderMainStatus => "Main Rec Status",
        // BaseLook properties route to Other
        DevicePropertyCode::BaseLookValue => "Base Look",
        DevicePropertyCode::BaseLookAppliedofPlayback => "Base Look (Play)",
        DevicePropertyCode::BaseLookNameofPlayback => "Base Look Name",
        DevicePropertyCode::BaseLookImportOperationEnableStatus => "Base Look Import",
        DevicePropertyCode::TeleWideLeverValueCapability => "Tele/Wide Lever",
        DevicePropertyCode::SimulRecSetting => "Simul. Rec",
        DevicePropertyCode::GridLineType => "Grid Type",
        DevicePropertyCode::LiveViewStatus => "LV Status",
        DevicePropertyCode::LiveViewProtocol => "LV Protocol",
        DevicePropertyCode::LogShootingMode => "Log Mode",
        DevicePropertyCode::LogShootingModeColorGamut => "Log Gamut",
        DevicePropertyCode::MaxVal => "Maximum Value",
        DevicePropertyCode::MaximumNumberOfBytes => "Maximum Bytes",
        DevicePropertyCode::MaximumSizeOfImageIDString => "Max ID Size",
        DevicePropertyCode::MeteringMode => "Meter Mode",
        DevicePropertyCode::MicrophoneDirectivity => "Mic Directivity",
        DevicePropertyCode::MovieShootingMode => "Movie Mode",
        DevicePropertyCode::OSDImageMode => "OSD Mode",
        // Camera system
        DevicePropertyCode::CameraPowerStatus => "Power Status",
        DevicePropertyCode::CameraErrorCautionStatus => "Error Status",
        DevicePropertyCode::CameraSystemErrorInfo => "System Error",
        DevicePropertyCode::CameraShakeStatus => "Shake Status",
        DevicePropertyCode::CameraSettingReadOperationEnableStatus => "Settings Read",
        DevicePropertyCode::CameraSettingSaveOperationEnableStatus => "Settings Save",
        DevicePropertyCode::CameraSettingsResetEnableStatus => "Reset Avail",
        DevicePropertyCode::SdkControlMode => "SDK Mode",
        DevicePropertyCode::BodyKeyLock => "Key Lock",
        DevicePropertyCode::SystemErrorCautionStatus => "System Error Status",
        DevicePropertyCode::UpdateBodyStatus => "Body Update",
        // FTP (non-Movie category FTP properties)
        DevicePropertyCode::FTPConnectionErrorInfo => "FTP Error",
        DevicePropertyCode::FTPAutoTransferTarget => "FTP Auto Target",
        DevicePropertyCode::FTPAutoTransferTargetStillImage => "FTP Auto (Still)",
        DevicePropertyCode::FTPTransferTarget => "FTP Target",
        DevicePropertyCode::FTPTransferStillImageQualitySize => "FTP Quality",
        DevicePropertyCode::FTPServerSettingOperationEnableStatus => "FTP Server Edit",
        DevicePropertyCode::FTPServerSettingVersion => "FTP Server Ver",
        DevicePropertyCode::FTPTransferSettingReadOperationEnableStatus => "FTP Read Avail",
        DevicePropertyCode::FTPTransferSettingSaveOperationEnableStatus => "FTP Save Avail",
        DevicePropertyCode::FTPTransferSettingSaveReadState => "FTP Settings",
        DevicePropertyCode::FTPJobListDataVersion => "FTP Job Ver",
        DevicePropertyCode::SelectFTPServer => "FTP Server",
        DevicePropertyCode::SelectFTPServerID => "FTP Server ID",
        DevicePropertyCode::ProtectImageInFTPTransfer => "FTP Protect",
        // Subject recognition
        DevicePropertyCode::SubjectRecognitionAF => "Subject Rec AF",
        DevicePropertyCode::SubjectRecognitionInAF => "Subject In AF",
        DevicePropertyCode::SubjectRecognitionAnimalBirdPriority => "Animal/Bird Pri",
        DevicePropertyCode::SubjectRecognitionAnimalBirdDetectionParts => "Animal/Bird Parts",
        DevicePropertyCode::SubjectRecognitionAnimalDetectionParts => "Animal Parts",
        DevicePropertyCode::SubjectRecognitionAnimalDetectionSensitivity => "Animal Sens",
        DevicePropertyCode::SubjectRecognitionAnimalTrackingSensitivity => "Animal Track",
        DevicePropertyCode::SubjectRecognitionAnimalTrackingSubjectShiftRange => "Animal Shift",
        DevicePropertyCode::SubjectRecognitionBirdDetectionParts => "Bird Parts",
        DevicePropertyCode::SubjectRecognitionBirdDetectionSensitivity => "Bird Sens",
        DevicePropertyCode::SubjectRecognitionBirdTrackingSensitivity => "Bird Track",
        DevicePropertyCode::SubjectRecognitionBirdTrackingSubjectShiftRange => "Bird Shift",
        DevicePropertyCode::SubjectRecognitionInsectDetectionSensitivity => "Insect Sens",
        DevicePropertyCode::SubjectRecognitionInsectTrackingSensitivity => "Insect Track",
        DevicePropertyCode::SubjectRecognitionInsectTrackingSubjectShiftRange => "Insect Shift",
        DevicePropertyCode::SubjectRecognitionCarTrainDetectionSensitivity => "Vehicle Sens",
        DevicePropertyCode::SubjectRecognitionCarTrainTrackingSensitivity => "Vehicle Track",
        DevicePropertyCode::SubjectRecognitionCarTrainTrackingSubjectShiftRange => "Vehicle Shift",
        DevicePropertyCode::SubjectRecognitionPlaneDetectionSensitivity => "Plane Sens",
        DevicePropertyCode::SubjectRecognitionPlaneTrackingSensitivity => "Plane Track",
        DevicePropertyCode::SubjectRecognitionPlaneTrackingSubjectShiftRange => "Plane Shift",
        DevicePropertyCode::SubjectRecognitionPersonTrackingSubjectShiftRange => "Person Shift",
        DevicePropertyCode::SubjectRecognitionPriorityOnRegisteredFace => "Reg Face Priority",
        // APS-C and sensor mode
        DevicePropertyCode::APSCOrFullSwitchingEnableStatus => "APS-C/Full Avail",
        DevicePropertyCode::APSCOrFullSwitchingSetting => "APS-C/Full Mode",
        DevicePropertyCode::APSCS35 => "APS-C/S35",
        // Auto review
        DevicePropertyCode::AutoReview => "Auto Rev",
        // Cinematic Vlog
        DevicePropertyCode::CinematicVlogSetting => "Cine Vlog",
        DevicePropertyCode::CinematicVlogLook => "Cine Vlog Look",
        DevicePropertyCode::CinematicVlogMood => "Cine Vlog Mood",
        DevicePropertyCode::CinematicVlogAFTransitionSpeed => "Cine Vlog AF",
        // Contents transfer
        DevicePropertyCode::ContentsTransferStatus => "Transfer Status",
        DevicePropertyCode::ContentsTransferProgress => "Transfer Progress",
        DevicePropertyCode::ContentsTransferCancelEnableStatus => "Transfer Cancel",
        // Folder and scene
        DevicePropertyCode::CreateNewFolder => "New Folder",
        DevicePropertyCode::CreateNewFolderEnableStatus => "New Folder Avail",
        DevicePropertyCode::CustomGridLineFileCommandVersion => "Grid Line Ver",
        // Depth of field
        DevicePropertyCode::DepthOfFieldAdjustmentMode => "DoF Adjust Mode",
        DevicePropertyCode::DepthOfFieldAdjustmentInterlockingMode => "DoF Interlock",
        // S&Q difference
        DevicePropertyCode::DifferentSetForSQMovie => "S&Q Movie Set",
        // Eframing
        DevicePropertyCode::EframingAutoFraming => "Auto Framing",
        DevicePropertyCode::EframingModeAuto => "E-Frame Auto",
        DevicePropertyCode::EframingProductionEffect => "E-Frame Effect",
        DevicePropertyCode::EframingHDMICrop => "E-Frame HDMI",
        DevicePropertyCode::EframingScaleAuto => "E-Frame Scale",
        DevicePropertyCode::EframingSpeedAuto => "E-Frame Speed",
        DevicePropertyCode::EframingSpeedPTZ => "E-Frame PTZ",
        DevicePropertyCode::EframingTrackingStartMode => "E-Frame Start",
        DevicePropertyCode::EframingType => "E-Frame Type",
        DevicePropertyCode::EframingCommandVersion => "E-Frame Ver",
        // Extended features
        DevicePropertyCode::ExtendedInterfaceMode => "Ext Interface",
        DevicePropertyCode::EnlargeScreenSetting => "Enlarge Screen",
        DevicePropertyCode::FaceEyeDetectionAFStatus => "Face/Eye AF Status",
        DevicePropertyCode::FirmwareUpdateCommandVersion => "Firmware Update Version",
        DevicePropertyCode::LiveViewArea => "LV Area",
        // Model and settings
        DevicePropertyCode::AreaTimeZoneSettingVersion => "Timezone Ver",
        DevicePropertyCode::CallSetting => "Recall Settings",
        DevicePropertyCode::HomeMenuSetting => "Home Menu",
        // AEL
        DevicePropertyCode::AEL => "AE Lock",
        // Auto recognition
        DevicePropertyCode::AutoRecognitionTargetCandidates => "Rec Target Options",
        DevicePropertyCode::AutoRecognitionTargetSetting => "Rec Target",
        // Camera lever
        DevicePropertyCode::CameraEframing => "Camera E-Frame",
        DevicePropertyCode::CameraLeverFunction => "Lever Function",
        // Content operations
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT1 => "Delete Slot 1",
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT2 => "Delete Slot 2",
        DevicePropertyCode::DeleteUserBaseLook => "Del User Look",
        // Control and interface
        DevicePropertyCode::ControlForHDMI => "HDMI Control",
        DevicePropertyCode::DebugMode => "Debug",
        // Touch and remote
        DevicePropertyCode::CancelRemoteTouchOperationEnableStatus => "Cancel Remote",
        DevicePropertyCode::FunctionOfRemoteTouchOperation => "Remote Touch Fn",
        DevicePropertyCode::RemoteTouchOperation => "Remote Touch Op",
        DevicePropertyCode::RemoteTouchOperationEnableStatus => "Remote Touch Status",
        DevicePropertyCode::IRRemoteSetting => "IR Remote Setting",
        // Flicker
        DevicePropertyCode::FlickerLessShooting => "Flicker Reduce",
        DevicePropertyCode::FlickerLessShootingStatus => "Flicker Status",
        DevicePropertyCode::FlickerScanEnableStatus => "Flicker Scan Avail",
        DevicePropertyCode::FlickerScanStatus => "Flicker Scan",
        // Focal distance
        DevicePropertyCode::FocalDistanceInFeet => "Focus Dist (ft)",
        DevicePropertyCode::FocalDistanceInMeter => "Focus Dist (m)",
        DevicePropertyCode::FocalDistanceUnitSetting => "Focus Dist Unit",
        // File numbering
        DevicePropertyCode::ForcedFileNumberResetEnableStatus => "Force File Reset",
        // Focus assist
        DevicePropertyCode::FullTimeDMF => "Full-time DMF",
        DevicePropertyCode::TrackingOnAndAFOnEnableStatus => "Tracking+AF Status",
        // Read-only flag
        DevicePropertyCode::GetOnly => "Read Only",
        // Moved from NDFilter (was incorrectly categorized due to "nd" substring)
        DevicePropertyCode::WindNoiseReduct => "Wind Noise Reduct.",
        DevicePropertyCode::DigitalExtenderMagnificationSetting => "Digital Extender",
        // PaintLook properties
        DevicePropertyCode::PaintLookAutoKnee => "Auto Knee",
        DevicePropertyCode::PaintLookBBlack => "B Black Level",
        DevicePropertyCode::PaintLookDetailLevel => "Detail Level",
        DevicePropertyCode::PaintLookDetailSetting => "Detail Setting",
        DevicePropertyCode::PaintLookKneePoint => "Knee Point",
        DevicePropertyCode::PaintLookKneeSetting => "Knee Setting",
        DevicePropertyCode::PaintLookKneeSlope => "Knee Slope",
        DevicePropertyCode::PaintLookMasterBlack => "Master Black",
        DevicePropertyCode::PaintLookMultiMatrixAreaIndication => "Matrix Area Indicator",
        DevicePropertyCode::PaintLookMultiMatrixAxis => "Matrix Axis",
        DevicePropertyCode::PaintLookMultiMatrixHue => "Matrix Hue",
        DevicePropertyCode::PaintLookMultiMatrixSaturation => "Matrix Saturation",
        DevicePropertyCode::PaintLookMultiMatrixSetting => "Matrix Setting",
        DevicePropertyCode::PaintLookRBlack => "R Black Level",
        DevicePropertyCode::PaintLookUserMatrixBG => "User Matrix B-G",
        DevicePropertyCode::PaintLookUserMatrixBR => "User Matrix B-R",
        DevicePropertyCode::PaintLookUserMatrixGB => "User Matrix G-B",
        DevicePropertyCode::PaintLookUserMatrixGR => "User Matrix G-R",
        DevicePropertyCode::PaintLookUserMatrixLevel => "User Matrix Level",
        DevicePropertyCode::PaintLookUserMatrixPhase => "User Matrix Phase",
        DevicePropertyCode::PaintLookUserMatrixRB => "User Matrix R-B",
        DevicePropertyCode::PaintLookUserMatrixRG => "User Matrix R-G",
        DevicePropertyCode::PaintLookUserMatrixSetting => "User Matrix Setting",
        DevicePropertyCode::PictureCacheRecSetting => "Cache Rec",
        DevicePropertyCode::PictureCacheRecSizeAndTime => "Cache Size/Time",
        DevicePropertyCode::PixelMappingEnableStatus => "Pixel Mapping Status",
        DevicePropertyCode::PostViewTransferResourceStatus => "Post View Transfer Status",
        DevicePropertyCode::PreAF => "Pre-AF",
        DevicePropertyCode::PresetPTZFSlotNumber => "PTZF Slot",
        DevicePropertyCode::PriorityKeySettings => "Priority Keys",
        DevicePropertyCode::ProductShowcaseSet => "Showcase Set",
        DevicePropertyCode::ProgramShiftStatus => "Prog Shift",
        DevicePropertyCode::PushAFModeSetting => "Push AF Mode",
        // Pan/Tilt properties
        DevicePropertyCode::PanLimitMode => "Pan Limit",
        DevicePropertyCode::PanLimitRangeMaximum => "Pan Limit Max",
        DevicePropertyCode::PanLimitRangeMinimum => "Pan Limit Min",
        DevicePropertyCode::PanPositionCurrentValue => "Pan Position",
        DevicePropertyCode::PanPositionStatus => "Pan Status",
        DevicePropertyCode::PanTiltAccelerationRampCurve => "PT Accel Curve",
        DevicePropertyCode::TiltLimitMode => "Tilt Limit",
        DevicePropertyCode::TiltLimitRangeMaximum => "Tilt Limit Max",
        DevicePropertyCode::TiltLimitRangeMinimum => "Tilt Limit Min",
        DevicePropertyCode::TiltPositionCurrentValue => "Tilt Position",
        DevicePropertyCode::TiltPositionStatus => "Tilt Status",
        // Tally lamp properties
        DevicePropertyCode::TallyLampControlGreen => "Tally Lamp (Green)",
        DevicePropertyCode::TallyLampControlRed => "Tally Lamp (Red)",
        DevicePropertyCode::TallyLampControlYellow => "Tally Lamp (Yellow)",
        // Timecode properties
        DevicePropertyCode::TCUBDisplaySetting => "TC/UB Display",
        DevicePropertyCode::TNumber => "T-Number",
        DevicePropertyCode::TopOfTheGroupShootingMarkSetting => "Group Shooting Mark",
        DevicePropertyCode::SynchroterminalForcedOutput => "Sync Terminal Output",
        // Playback properties
        DevicePropertyCode::PlaySetOfMultiMedia => "Multi-Media Set",
        DevicePropertyCode::PlaybackContentsName => "Content Name",
        DevicePropertyCode::PlaybackContentsNumber => "Content Number",
        DevicePropertyCode::PlaybackContentsRecordingDateTime => "Rec Date/Time",
        DevicePropertyCode::PlaybackContentsRecordingFileFormat => "Rec Format",
        DevicePropertyCode::PlaybackContentsRecordingFrameRate => "Rec Frame Rate",
        DevicePropertyCode::PlaybackContentsRecordingResolution => "Rec Resolution",
        DevicePropertyCode::PlaybackContentsTotalNumber => "Total Contents",
        DevicePropertyCode::PlaybackMedia => "Play Media",
        DevicePropertyCode::PlaybackViewMode => "Play View Mode",
        DevicePropertyCode::PlaybackVolumeSettings => "Playback Volume",
        // Recorder properties
        DevicePropertyCode::RAWJPCSaveImage => "RAW+JPG Save",
        DevicePropertyCode::RecognitionTarget => "Rec Target",
        DevicePropertyCode::RecordablePowerSources => "Rec Power",
        DevicePropertyCode::RecorderClipName => "Clip Name",
        DevicePropertyCode::RecorderControlMainSetting => "Main Rec Control",
        DevicePropertyCode::RecorderControlProxySetting => "Proxy Control",
        DevicePropertyCode::RecorderExtRawStatus => "Ext RAW",
        DevicePropertyCode::RecorderSaveDestination => "Save Destination",
        DevicePropertyCode::RecorderStartMain => "Start Main Rec",
        DevicePropertyCode::RecordingFileNumber => "File Number",
        DevicePropertyCode::RecordingFolderFormat => "Folder Format",
        // Scene file properties
        DevicePropertyCode::SceneFileCommandVersion => "Scene Ver",
        DevicePropertyCode::SceneFileDownloadOperationEnableStatus => "Scene Download",
        DevicePropertyCode::SceneFileIndex => "Scene Index",
        DevicePropertyCode::SceneFileIndexesAvailableForDownload => "Scene Downloads",
        DevicePropertyCode::SceneFileUploadOperationEnableStatus => "Scene Upload",
        DevicePropertyCode::SensorCleaningEnableStatus => "Sensor Cleaning Status",
        // Metadata properties
        DevicePropertyCode::SelectIPTCMetadata => "IPTC Metadata",
        DevicePropertyCode::SetCopyright => "Copyright",
        DevicePropertyCode::SetPhotographer => "Photographer",
        DevicePropertyCode::SetPresetPTZFBinaryVersion => "Preset PTZF Binary Version",
        DevicePropertyCode::ShootingEnableSettingLicense => "Shooting License Status",
        DevicePropertyCode::ShootingTimingPreNotificationMode => "Shooting Timing Notification",
        DevicePropertyCode::ShutterSpeed => "Shutter",
        DevicePropertyCode::SilentMode => "Silent",
        DevicePropertyCode::SimulRecSettingMovieRecButton => "Simul Rec Btn",
        DevicePropertyCode::SnapshotInfo => "Snapshot",
        DevicePropertyCode::StillImageTransSize => "Still Xfer Size",
        DevicePropertyCode::WriteCopyrightInfo => "Write Copyright",
        // User base look selection
        DevicePropertyCode::SelectUserBaseLookToEdit => "Edit Base Look",
        DevicePropertyCode::SelectUserBaseLookToSetInPPLUT => "Base Look for PP",
        DevicePropertyCode::UserBaseLookAELevelOffset => "Base Look AE Offset",
        DevicePropertyCode::UserBaseLookInput => "Base Look Input",
        DevicePropertyCode::UserBaseLookOutput => "Base Look Output",
        // User bit properties
        DevicePropertyCode::UserBitPreset => "UB Preset",
        DevicePropertyCode::UserBitPresetResetEnableStatus => "UB Reset Status",
        DevicePropertyCode::UserBitTimeRec => "UB Time Rec",
        DevicePropertyCode::UploadDatasetVersion => "Dataset Ver",
        // Time shift properties
        DevicePropertyCode::TimeShiftPreShootingTimeSetting => "Pre-Shoot Time",
        DevicePropertyCode::TimeShiftShooting => "Time Shift",
        DevicePropertyCode::TimeShiftShootingStatus => "Time Shift Status",
        DevicePropertyCode::TimeShiftTriggerSetting => "Time Shift Trigger",
        // Miscellaneous settings
        DevicePropertyCode::RedEyeReduction => "Red-Eye",
        DevicePropertyCode::ReleaseWithoutCard => "Release w/o Card",
        DevicePropertyCode::ReleaseWithoutLens => "No Lens Release",
        DevicePropertyCode::RightLeftEyeSelect => "Right/Left Eye Select",
        DevicePropertyCode::S1 => "S1 (Half Press)",
        DevicePropertyCode::S2 => "S2 (Full Press)",
        DevicePropertyCode::SQModeSetting => "S&Q Mode Setting",
        // Video recording properties
        DevicePropertyCode::ValidRecordingVideoFormat => "Valid Formats",
        DevicePropertyCode::VideoRecordingFormatBitrateSetting => "Rec Bitrate",
        DevicePropertyCode::VideoRecordingFormatQuality => "Rec Quality",
        // Zoom properties
        DevicePropertyCode::ZoomDistance => "Zoom Dist",
        DevicePropertyCode::ZoomSetting => "Zoom Set",
        DevicePropertyCode::ZoomSpeedRange => "Zoom Speed",
        // Other properties
        DevicePropertyCode::CancelMediaFormatEnableStatus => "Cancel Format",
        DevicePropertyCode::FEL => "Flash EV Lock",
        DevicePropertyCode::Undefined => "Unknown",
        _ => code.name(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_device_property_is_valid_value_discrete() {
        let prop = DeviceProperty {
            code: 0,
            data_type: DataType::UInt32,
            enable_flag: EnableFlag::ReadWrite,
            current_value: 100,
            current_string: None,
            constraint: ValueConstraint::Discrete(vec![100, 200, 400, 800]),
        };
        assert!(prop.is_valid_value(100));
        assert!(prop.is_valid_value(400));
        assert!(!prop.is_valid_value(300));
        assert!(prop.possible_values().is_some());
        assert_eq!(prop.possible_values().unwrap(), &[100, 200, 400, 800]);
    }

    #[test]
    fn test_device_property_is_valid_value_none() {
        let prop_empty = DeviceProperty {
            code: 0,
            data_type: DataType::UInt32,
            enable_flag: EnableFlag::ReadWrite,
            current_value: 100,
            current_string: None,
            constraint: ValueConstraint::None,
        };
        assert!(prop_empty.is_valid_value(999));
        assert!(prop_empty.possible_values().is_none());
    }

    #[test]
    fn test_device_property_is_valid_value_range() {
        let prop_range = DeviceProperty {
            code: 0,
            data_type: DataType::UInt8,
            enable_flag: EnableFlag::ReadWrite,
            current_value: 3,
            current_string: None,
            constraint: ValueConstraint::Range {
                min: 1,
                max: 7,
                step: 1,
            },
        };
        assert!(prop_range.is_valid_value(1));
        assert!(prop_range.is_valid_value(4));
        assert!(prop_range.is_valid_value(7));
        assert!(!prop_range.is_valid_value(0));
        assert!(!prop_range.is_valid_value(8));
        assert!(prop_range.is_range());
        assert_eq!(prop_range.range_params(), Some((1, 7, 1)));
        assert!(prop_range.possible_values().is_none());
    }

    #[test]
    fn test_value_constraint_range_with_step() {
        let constraint = ValueConstraint::Range {
            min: 0,
            max: 10,
            step: 2,
        };
        assert!(constraint.is_valid(0));
        assert!(constraint.is_valid(2));
        assert!(constraint.is_valid(4));
        assert!(constraint.is_valid(10));
        assert!(!constraint.is_valid(1));
        assert!(!constraint.is_valid(3));
        assert!(!constraint.is_valid(11));

        let expanded = constraint.expand_range().unwrap();
        assert_eq!(expanded, vec![0, 2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_value_constraint_range_signed() {
        let constraint = ValueConstraint::Range {
            min: -10,
            max: 10,
            step: 5,
        };
        assert!(constraint.is_valid((-10_i64) as u64));
        assert!(constraint.is_valid((-5_i64) as u64));
        assert!(constraint.is_valid(0));
        assert!(constraint.is_valid(5));
        assert!(constraint.is_valid(10));

        let expanded = constraint.expand_range().unwrap();
        assert_eq!(expanded, vec![-10, -5, 0, 5, 10]);
    }

    use super::todo::{NEEDS_DESCRIPTION, NEEDS_DISPLAY_NAME, NEEDS_VALUE_TYPE};
    use std::collections::HashSet;

    #[test]
    fn test_all_properties_have_custom_display_names() {
        let expected: HashSet<_> = NEEDS_DISPLAY_NAME.iter().collect();
        let mut actual_missing = Vec::new();

        for code in DevicePropertyCode::ALL {
            let display = property_display_name(*code);
            let fallback = code.name();

            if display == fallback {
                actual_missing.push(*code);
            }
        }

        let actual: HashSet<_> = actual_missing.iter().collect();

        // Find properties that are missing but not in expected list (new regressions)
        let unexpected: Vec<_> = actual.difference(&expected).collect();
        assert!(
            unexpected.is_empty(),
            "New properties missing display names (add display name or add to NEEDS_DISPLAY_NAME in todo.rs): {:?}",
            unexpected
        );

        // Find properties in expected list that now have display names (need to remove from list)
        let fixed: Vec<_> = expected.difference(&actual).collect();
        assert!(
            fixed.is_empty(),
            "Properties now have display names - remove from NEEDS_DISPLAY_NAME in todo.rs: {:?}",
            fixed
        );
    }

    #[test]
    fn test_all_properties_have_descriptions() {
        let expected: HashSet<_> = NEEDS_DESCRIPTION.iter().collect();
        let mut actual_missing = Vec::new();

        for code in DevicePropertyCode::ALL {
            let desc = property_description(*code);
            if desc.is_empty() {
                actual_missing.push(*code);
            }
        }

        let actual: HashSet<_> = actual_missing.iter().collect();

        // Find properties that are missing but not in expected list (new regressions)
        let unexpected: Vec<_> = actual.difference(&expected).collect();
        assert!(
            unexpected.is_empty(),
            "New properties missing descriptions (add description or add to NEEDS_DESCRIPTION in todo.rs): {:?}",
            unexpected
        );

        // Find properties in expected list that now have descriptions (need to remove from list)
        let fixed: Vec<_> = expected.difference(&actual).collect();
        assert!(
            fixed.is_empty(),
            "Properties now have descriptions - remove from NEEDS_DESCRIPTION in todo.rs: {:?}",
            fixed
        );
    }

    #[test]
    fn test_all_properties_have_value_types() {
        let expected: HashSet<_> = NEEDS_VALUE_TYPE.iter().collect();
        let mut actual_missing = Vec::new();

        for code in DevicePropertyCode::ALL {
            if property_value_type(*code) == PropertyValueType::Unknown {
                actual_missing.push(*code);
            }
        }

        let actual: HashSet<_> = actual_missing.iter().collect();

        // Find properties that are missing but not in expected list (new regressions)
        let unexpected: Vec<_> = actual.difference(&expected).collect();
        assert!(
            unexpected.is_empty(),
            "New properties missing value types (add value type or add to NEEDS_VALUE_TYPE in todo.rs): {:?}",
            unexpected
        );

        // Find properties in expected list that now have value types (need to remove from list)
        let fixed: Vec<_> = expected.difference(&actual).collect();
        assert!(
            fixed.is_empty(),
            "Properties now have value types - remove from NEEDS_VALUE_TYPE in todo.rs: {:?}",
            fixed
        );
    }

    #[test]
    fn test_all_properties_have_valid_categories() {
        for code in DevicePropertyCode::ALL {
            let category = property_category(*code);
            // Ensure property_category() doesn't panic and returns a valid category
            let _ = format!("{:?}", category);
        }
    }

    #[test]
    fn test_property_value_type_mapping() {
        use PropertyValueType::*;

        // Formatted numeric values
        assert_eq!(property_value_type(DevicePropertyCode::FNumber), Aperture);
        assert_eq!(
            property_value_type(DevicePropertyCode::ShutterSpeed),
            ShutterSpeed
        );
        assert_eq!(property_value_type(DevicePropertyCode::IsoSensitivity), Iso);
        assert_eq!(
            property_value_type(DevicePropertyCode::ExposureBiasCompensation),
            ExposureCompensation
        );
        assert_eq!(
            property_value_type(DevicePropertyCode::Colortemp),
            ColorTemperature
        );

        // Enum types
        assert_eq!(
            property_value_type(DevicePropertyCode::ExposureProgramMode),
            ExposureProgram
        );
        assert_eq!(
            property_value_type(DevicePropertyCode::FocusMode),
            FocusMode
        );
        assert_eq!(
            property_value_type(DevicePropertyCode::WhiteBalance),
            WhiteBalance
        );
        assert_eq!(
            property_value_type(DevicePropertyCode::DriveMode),
            DriveMode
        );

        // Toggle types
        assert_eq!(
            property_value_type(DevicePropertyCode::AutoSlowShutter),
            Switch
        );
        assert_eq!(
            property_value_type(DevicePropertyCode::RedEyeReduction),
            OnOff
        );
        assert_eq!(
            property_value_type(DevicePropertyCode::IrisModeSetting),
            AutoManual
        );

        // Percentage
        assert_eq!(
            property_value_type(DevicePropertyCode::BatteryRemain),
            Percentage
        );

        // Unknown falls through
        assert_eq!(property_value_type(DevicePropertyCode::Undefined), Unknown);
    }

    #[test]
    fn test_all_properties_have_valid_value_types() {
        // Ensure property_value_type doesn't panic for any property
        for code in DevicePropertyCode::ALL {
            let _ = property_value_type(*code);
        }
    }
}
