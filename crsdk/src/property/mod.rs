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
        PropertyCategory::Media => metadata::media::description(code),
        PropertyCategory::Power => metadata::power::description(code),
        PropertyCategory::Zoom => metadata::zoom::description(code),
        PropertyCategory::Lens => metadata::lens::description(code),
        PropertyCategory::Audio => metadata::audio::description(code),
        PropertyCategory::PictureProfile => metadata::picture_profile::description(code),
        PropertyCategory::NDFilter => metadata::nd_filter::description(code),
        PropertyCategory::Display => metadata::display::description(code),
        PropertyCategory::Stabilization => metadata::stabilization::description(code),
        PropertyCategory::Silent => metadata::silent::description(code),
        _ => metadata::other::description(code),
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
        PropertyCategory::Media => metadata::media::display_name(code),
        PropertyCategory::Power => metadata::power::display_name(code),
        PropertyCategory::Zoom => metadata::zoom::display_name(code),
        PropertyCategory::Lens => metadata::lens::display_name(code),
        PropertyCategory::Audio => metadata::audio::display_name(code),
        PropertyCategory::PictureProfile => metadata::picture_profile::display_name(code),
        PropertyCategory::NDFilter => metadata::nd_filter::display_name(code),
        PropertyCategory::Display => metadata::display::display_name(code),
        PropertyCategory::Stabilization => metadata::stabilization::display_name(code),
        PropertyCategory::Silent => metadata::silent::display_name(code),
        _ => metadata::other::display_name(code),
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
