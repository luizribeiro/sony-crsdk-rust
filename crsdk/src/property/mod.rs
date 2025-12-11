//! Camera property types, values, and metadata.
//!
//! This module provides:
//! - Core types for working with camera properties ([`DeviceProperty`], [`DataType`], [`EnableFlag`])
//! - Value enums for specific property types (organized by subsystem)
//! - Display names and descriptions for properties
//! - The [`PropertyValue`] trait for type-safe value conversion
//!
//! # Architecture
//!
//! Properties are organized into semantic categories (Exposure, Focus, WhiteBalance, etc.).
//! Each category module in `categories/` is the single source of truth for:
//! - Which property codes belong to that category (`CODES` array)
//! - Human-readable descriptions (`description()`)
//! - UI-friendly display names (`display_name()`)
//! - Value type mappings for formatting (`value_type()`)
//!
//! Tests validate at runtime that:
//! - No property code appears in multiple categories
//! - All property codes are explicitly categorized

pub mod categories;
mod core;
mod traits;
mod typed_value;
pub mod values;

// Re-export core infrastructure types
pub(crate) use core::{device_property_from_sdk, device_property_from_sdk_debug};
pub use core::{DataType, DeviceProperty, EnableFlag, ValueConstraint};

// Re-export core trait and typed value
pub use traits::PropertyValue;
pub use typed_value::TypedValue;

// Re-export category types from new categories module
pub use categories::{
    all_categories, description as property_description, display_name as property_display_name,
    property_category, value_type as property_value_type, Category, PropertyCategoryId,
};

// Re-export all value types from values/
pub use values::{
    AspectRatio, AutoManual, FileType, FlashMode, FocusArea, FocusBracketOrder,
    FocusBracketShootingStatus, FocusDrivingStatus, FocusFrameState, FocusIndicator, FocusMode,
    FocusTouchSpotStatus, FocusTrackingStatus, ImageQuality, ImageSize, LiveViewDisplayEffect,
    LiveViewProtocol, LockIndicator, MeteringMode, OnOff, PrioritySetInAF, PrioritySetInAWB,
    PropertyValueType, PushAutoFocus, ShutterMode, ShutterModeStatus, SilentModeApertureDrive,
    SubjectRecognitionAF, Switch, TrackingFrameType, WhiteBalance, WhiteBalanceSwitch,
};
pub use values::{ExposureCtrlType, ExposureProgram, GainUnitSetting};

// Re-export drive and movie types from values/
pub use values::{
    DriveMode, IntervalRecShutterType, MovieFileFormat, MoviePlayingState, MovieQuality,
    MovieRecReviewPlayingState, MovieShootingModeColorGamut, MovieShootingModeTargetDisplay,
    PlaybackContentsGammaType, RecorderSaveDestination, RecorderStatus, RecordingFolderFormat,
    RecordingState, TimeCodeFormat, TimeCodeMake, TimeCodeRun, VideoRecordingFormatQuality,
};

// Re-export media types from values/
pub use values::{LiveViewStatus, MediaSlotRecordingType, MediaSlotWritingState, SlotStatus};

// Re-export common/other types from values/
pub use values::{
    AFTrackForSpeedChange, AFTrackingSensitivity, ApertureDriveInAF, AudioInputCHInputSelect,
    AudioSignals, AudioStreamBitDepth, AudioStreamChannel, AutoPowerOffTemperature,
    BatteryRemainDisplayUnit, CameraOperatingMode, CameraPowerStatus, ColorSpace,
    CompressionFileFormat, CreativeLook, CreativeLookResetEnableStatus, CustomWBSizeSetting,
    DRangeOptimizer, DeviceOverheatingState, DispMode, EframingProductionEffect, EframingType,
    FTPConnectionStatus, FaceFrameType, FocusFrameType, FocusOperation,
    FocusOperationWithInt16EnableStatus, FrameInfoType, FunctionOfTouchOperation,
    GainBaseSensitivity, GridLineType, HighIsoNR, ImageStabilizationLevelMovie,
    ImageStabilizationSteadyShotMovie, ImagerScanMode, IntervalRecMode, IntervalRecStatus,
    IrisDisplayUnit, IsoAutoMinShutterSpeedMode, IsoAutoMinShutterSpeedPreset,
    LensCompensationShading, MonitoringOutputFormat, NDFilterMode, NearFarEnableStatus,
    PictureEffect, PictureProfile, PictureProfileBlackGammaRange, PictureProfileColorMode,
    PictureProfileDetailAdjustMode, PictureProfileGamma, PictureProfileKneeAutoSetSensitivity,
    PictureProfileKneeMode, PictureProfileResetEnableStatus, PlaybackMedia, PowerSource,
    PriorityKeySettings, RAWFileCompressionType, RecordingMedia, RecordingMediaMovie,
    RemoconZoomSpeedType, RightLeftEyeSelect, SdkControlMode, SelectFinder,
    ShutterReleaseTimeLagControl, ShutterType, SoftSkinEffect, StillImageStoreDestination,
    StreamCipherType, StreamStatus, SubjectRecognitionAnimalBirdDetectionParts,
    SubjectRecognitionAnimalBirdPriority, TCUBDisplaySetting, TimeCodePresetResetEnableStatus,
    TimeShiftTriggerSetting, TouchOperation, VideoStreamCodec, WindNoiseReduction,
    ZoomDrivingStatus, ZoomOperation, ZoomTypeStatus, APSC_S35,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crsdk_sys::DevicePropertyCode;

    #[test]
    fn test_all_properties_have_custom_display_names() {
        let mut missing = Vec::new();

        for code in DevicePropertyCode::ALL {
            let display = property_display_name(*code);
            if display == code.name() {
                missing.push(*code);
            }
        }

        assert!(
            missing.is_empty(),
            "Properties missing display names: {:?}",
            missing
        );
    }

    #[test]
    fn test_all_properties_have_descriptions() {
        let mut missing = Vec::new();

        for code in DevicePropertyCode::ALL {
            if property_description(*code).is_empty() {
                missing.push(*code);
            }
        }

        assert!(
            missing.is_empty(),
            "Properties missing descriptions: {:?}",
            missing
        );
    }

    #[test]
    fn test_all_properties_have_value_types() {
        let mut missing = Vec::new();

        for code in DevicePropertyCode::ALL {
            // Undefined is expected to have Unknown type
            if *code != DevicePropertyCode::Undefined
                && property_value_type(*code) == PropertyValueType::Unknown
            {
                missing.push(*code);
            }
        }

        assert!(
            missing.is_empty(),
            "Properties missing value types: {:?}",
            missing
        );
    }

    #[test]
    fn test_all_properties_have_valid_categories() {
        for code in DevicePropertyCode::ALL {
            let category = property_category(*code);
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
        assert_eq!(
            property_value_type(DevicePropertyCode::ShutterAngle),
            ShutterAngle
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
        for code in DevicePropertyCode::ALL {
            let _ = property_value_type(*code);
        }
    }
}
