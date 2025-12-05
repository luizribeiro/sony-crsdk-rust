//! Camera property types, values, and metadata.
//!
//! This module provides:
//! - Core types for working with camera properties ([`DeviceProperty`], [`DataType`], [`EnableFlag`])
//! - Value enums for specific property types (organized by subsystem)
//! - Display names and descriptions for properties

mod common;
mod drive;
mod exposure;
mod flash;
mod focus;
mod image;
mod movie;
#[cfg(test)]
mod todo;
mod white_balance;

// Re-export all value enums
pub use common::{AutoManual, LiveViewDisplayEffect, OnOff, SilentModeApertureDrive, Switch};
pub use drive::{DriveMode, IntervalRecShutterType};
pub use exposure::{
    ExposureCtrlType, ExposureProgram, MeteringMode, ShutterMode, ShutterModeStatus,
};
pub use flash::FlashMode;
pub use focus::{FocusArea, FocusMode, FocusTrackingStatus, PrioritySetInAF, SubjectRecognitionAF};
pub use image::{AspectRatio, FileType, ImageQuality, ImageSize};
pub use movie::{format_movie_quality, MovieFileFormat};
pub use white_balance::{LockIndicator, PrioritySetInAWB, WhiteBalance};

use crsdk_sys::{DevicePropertyCode, PropertyCategory};

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

        const ARRAY_BIT: u32 = 0x2000;
        const RANGE_BIT: u32 = 0x4000;

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
    if let Some(vt) = exposure::value_type(code) {
        return vt;
    }
    if let Some(vt) = focus::value_type(code) {
        return vt;
    }
    if let Some(vt) = white_balance::value_type(code) {
        return vt;
    }
    if let Some(vt) = drive::value_type(code) {
        return vt;
    }
    if let Some(vt) = flash::value_type(code) {
        return vt;
    }
    if let Some(vt) = image::value_type(code) {
        return vt;
    }
    if let Some(vt) = movie::value_type(code) {
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
        | DevicePropertyCode::GammaDisplayAssist
        | DevicePropertyCode::GammaDisplayAssistType
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

    let possible_values = if prop.getSetValuesSize > 0 && !prop.getSetValues.is_null() {
        parse_possible_values(data_type, prop.getSetValues, prop.getSetValuesSize)
    } else {
        parse_possible_values(data_type, prop.values, prop.valuesSize)
    };

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

/// Get a description of what a property does.
pub fn property_description(code: DevicePropertyCode) -> &'static str {
    match code.category() {
        PropertyCategory::Exposure | PropertyCategory::Metering => exposure::description(code),
        PropertyCategory::Focus => focus::description(code),
        PropertyCategory::WhiteBalance => white_balance::description(code),
        PropertyCategory::Drive => drive::description(code),
        PropertyCategory::Flash => flash::description(code),
        PropertyCategory::Image => image::description(code),
        PropertyCategory::Movie => movie::description(code),
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
    match code.category() {
        PropertyCategory::Exposure | PropertyCategory::Metering => exposure::display_name(code),
        PropertyCategory::Focus => focus::display_name(code),
        PropertyCategory::WhiteBalance => white_balance::display_name(code),
        PropertyCategory::Drive => drive::display_name(code),
        PropertyCategory::Flash => flash::display_name(code),
        PropertyCategory::Image => image::display_name(code),
        PropertyCategory::Movie => movie::display_name(code),
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
        DevicePropertyCode::MediaSLOT1ImageSize => {
            "Image resolution for files saved to slot 1."
        }
        DevicePropertyCode::MediaSLOT2ImageSize => {
            "Image resolution for files saved to slot 2."
        }
        DevicePropertyCode::MediaSLOT1RAWFileCompressionType => {
            "RAW file compression type for slot 1."
        }
        DevicePropertyCode::MediaSLOT2RAWFileCompressionType => {
            "RAW file compression type for slot 2."
        }
        _ => "",
    }
}

fn media_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::MediaSLOT1Status => "Slot 1 Status",
        DevicePropertyCode::MediaSLOT2Status => "Slot 2 Status",
        DevicePropertyCode::MediaSLOT3Status => "Slot 3 Status",
        DevicePropertyCode::MediaSLOT1RemainingNumber => "Slot 1 Remaining",
        DevicePropertyCode::MediaSLOT2RemainingNumber => "Slot 2 Remaining",
        DevicePropertyCode::MediaSLOT1RemainingTime => "Slot 1 Time Left",
        DevicePropertyCode::MediaSLOT2RemainingTime => "Slot 2 Time Left",
        DevicePropertyCode::MediaSLOT3RemainingTime => "Slot 3 Time Left",
        DevicePropertyCode::MediaSLOT1FileType => "Slot 1 Format",
        DevicePropertyCode::MediaSLOT2FileType => "Slot 2 Format",
        DevicePropertyCode::MediaSLOT1ImageQuality => "Slot 1 Quality",
        DevicePropertyCode::MediaSLOT2ImageQuality => "Slot 2 Quality",
        DevicePropertyCode::MediaSLOT1ImageSize => "Slot 1 Size",
        DevicePropertyCode::MediaSLOT2ImageSize => "Slot 2 Size",
        DevicePropertyCode::MediaSLOT1RAWFileCompressionType => "Slot 1 RAW Comp",
        DevicePropertyCode::MediaSLOT2RAWFileCompressionType => "Slot 2 RAW Comp",
        DevicePropertyCode::MediaSLOT1QuickFormatEnableStatus => "Slot 1 Quick Format",
        DevicePropertyCode::MediaSLOT2QuickFormatEnableStatus => "Slot 2 Quick Format",
        DevicePropertyCode::AutoSwitchMedia => "Auto Switch Media",
        DevicePropertyCode::SimulRecSetting => "Simultaneous Recording",
        _ => code.name(),
    }
}

fn power_description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::BatteryRemain => {
            "Remaining battery capacity as a percentage. Monitor this to avoid running out during a shoot."
        }
        DevicePropertyCode::BatteryLevel => "Battery charge level indicator. Shows approximate remaining power.",
        DevicePropertyCode::PowerSource => "Current power source—internal battery, external battery grip, or AC adapter.",
        DevicePropertyCode::AutoPowerOffTemperature => {
            "Temperature threshold for automatic shutdown. Higher settings allow longer recording but risk overheating damage."
        }
        DevicePropertyCode::DeviceOverheatingState => {
            "Current thermal status. Warning levels indicate the camera may shut down soon to prevent damage."
        }
        _ => "",
    }
}

fn power_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::BatteryRemain => "Battery Remaining",
        DevicePropertyCode::BatteryLevel => "Battery Level",
        DevicePropertyCode::BatteryRemainingInMinutes => "Battery (Minutes)",
        DevicePropertyCode::BatteryRemainingInVoltage => "Battery Voltage",
        DevicePropertyCode::BatteryRemainDisplayUnit => "Battery Display Unit",
        DevicePropertyCode::PowerSource => "Power Source",
        DevicePropertyCode::AutoPowerOffTemperature => "Auto Power Off Temp",
        DevicePropertyCode::DeviceOverheatingState => "Overheating State",
        DevicePropertyCode::RecordablePowerSources => "Recordable Power Sources",
        DevicePropertyCode::USBPowerSupply => "USB Power Supply",
        DevicePropertyCode::DCVoltage => "DC Voltage",
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
        DevicePropertyCode::ZoomSetting => "Zoom Mode",
        DevicePropertyCode::ZoomOperation => "Zoom Op",
        DevicePropertyCode::ZoomOperationWithInt16 => "Zoom Op (16-bit)",
        DevicePropertyCode::ZoomOperationStatus => "Zoom Op Status",
        DevicePropertyCode::ZoomPositionSetting => "Zoom Position",
        DevicePropertyCode::ZoomPositionCurrentValue => "Zoom Pos (Current)",
        DevicePropertyCode::ZoomDrivingStatus => "Zoom Drive Status",
        DevicePropertyCode::ZoomSpeedRange => "Zoom Speed",
        DevicePropertyCode::ZoomDistance => "Zoom Dist",
        DevicePropertyCode::ZoomDistanceUnitSetting => "Zoom Dist Unit",
        DevicePropertyCode::ZoomBarInformation => "Zoom Bar Info",
        DevicePropertyCode::ZoomTypeStatus => "Zoom Type",
        DevicePropertyCode::DigitalZoomScale => "Digital Zoom",
        DevicePropertyCode::RemoconZoomSpeedType => "Remote Zoom Speed",
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
        DevicePropertyCode::WindNoiseReduct => {
            "Reduces low-frequency wind noise in the built-in microphone. May slightly affect audio quality."
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
        DevicePropertyCode::DigitalExtenderMagnificationSetting => {
            "Digital extender zoom factor. Crops and enlarges the image beyond optical zoom range."
        }
        _ => "",
    }
}

fn nd_filter_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::NDFilter => "ND Filter",
        DevicePropertyCode::NDFilterMode => "ND Filter Mode",
        DevicePropertyCode::NDFilterModeSetting => "ND Mode Setting",
        DevicePropertyCode::NDFilterValue => "ND Filter Value",
        DevicePropertyCode::NDFilterSwitchingSetting => "ND Switching",
        DevicePropertyCode::WindNoiseReduct => "Wind Noise Reduct.",
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
        DevicePropertyCode::DigitalExtenderMagnificationSetting => "Digital Extender",
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
        DevicePropertyCode::MonitorBrightnessType => {
            "Screen brightness setting. Manual sets a fixed level. Auto adjusts based on ambient light. Sunny Weather boosts for outdoors."
        }
        _ => "",
    }
}

fn display_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::LiveViewDisplayEffect => "Live View Display Effect",
        DevicePropertyCode::LiveViewStatus => "Live View Status",
        DevicePropertyCode::LiveViewImageQuality => "Live View Quality",
        DevicePropertyCode::LiveViewProtocol => "Live View Protocol",
        DevicePropertyCode::DispMode => "Display Mode",
        DevicePropertyCode::DispModeSetting => "Display Mode Setting",
        DevicePropertyCode::MonitorLUTSetting => "Monitor LUT",
        DevicePropertyCode::GridLineDisplay => "Grid Line Display",
        DevicePropertyCode::GridLineType => "Grid Line Type",
        DevicePropertyCode::MonitorBrightnessType => "Monitor Brightness",
        DevicePropertyCode::MonitorBrightnessManual => "Monitor Brightness (Manual)",
        DevicePropertyCode::SelectFinder => "Select Finder",
        DevicePropertyCode::DisplayQualityFinder => "Finder Display Quality",
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
        _ => "",
    }
}

fn silent_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::SilentMode => "Silent Mode",
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
        _ => "",
    }
}

fn other_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::DRO => "D-Range Optimizer",
        DevicePropertyCode::MeteredManualLevel => "Meter Level (M)",
        DevicePropertyCode::StreamStatus => "Stream Status",
        DevicePropertyCode::StreamModeSetting => "Stream Mode",
        DevicePropertyCode::StreamDisplayName => "Stream Display Name",
        DevicePropertyCode::StreamLatency => "Stream Latency",
        DevicePropertyCode::FTPFunction => "FTP Function",
        DevicePropertyCode::FTPAutoTransfer => "FTP Auto Transfer",
        DevicePropertyCode::FTPConnectionStatus => "FTP Connection Status",
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
        DevicePropertyCode::FunctionOfTouchOperation => "Touch Function",
        DevicePropertyCode::TouchFunctionInMF => "Touch Fn (MF)",
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
            let category = code.category();
            // Ensure category() doesn't panic and returns a valid category
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
