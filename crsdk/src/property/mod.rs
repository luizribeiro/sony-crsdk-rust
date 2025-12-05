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
        DevicePropertyCode::MediaSLOT1FileType => "Slot 1 File Type",
        DevicePropertyCode::MediaSLOT2FileType => "Slot 2 File Type",
        DevicePropertyCode::MediaSLOT1ImageQuality => "Slot 1 Image Quality",
        DevicePropertyCode::MediaSLOT2ImageQuality => "Slot 2 Image Quality",
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
        _ => "",
    }
}

fn zoom_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::ZoomScale => "Zoom Scale",
        DevicePropertyCode::ZoomSetting => "Zoom Setting",
        DevicePropertyCode::ZoomOperation => "Zoom Operation",
        DevicePropertyCode::ZoomPositionSetting => "Zoom Position",
        DevicePropertyCode::ZoomPositionCurrentValue => "Zoom Position (Current)",
        DevicePropertyCode::ZoomDrivingStatus => "Zoom Status",
        DevicePropertyCode::ZoomSpeedRange => "Zoom Speed Range",
        DevicePropertyCode::ZoomDistance => "Zoom Distance",
        DevicePropertyCode::ZoomDistanceUnitSetting => "Zoom Distance Unit",
        DevicePropertyCode::DigitalZoomScale => "Digital Zoom Scale",
        DevicePropertyCode::DigitalExtenderMagnificationSetting => "Digital Extender",
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
        DevicePropertyCode::WindNoiseReduct => {
            "Reduces low-frequency wind noise in the built-in microphone. May slightly affect audio quality."
        }
        _ => "",
    }
}

fn audio_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::AudioRecording => "Audio Recording",
        DevicePropertyCode::AudioSignals => "Audio Signals",
        DevicePropertyCode::AudioSignalsVolume => "Audio Volume",
        DevicePropertyCode::AudioLevelDisplay => "Audio Level Display",
        DevicePropertyCode::WindNoiseReduct => "Wind Noise Reduction",
        DevicePropertyCode::AudioInputCH1Level => "Audio CH1 Level",
        DevicePropertyCode::AudioInputCH2Level => "Audio CH2 Level",
        DevicePropertyCode::AudioInputCH3Level => "Audio CH3 Level",
        DevicePropertyCode::AudioInputCH4Level => "Audio CH4 Level",
        DevicePropertyCode::AudioInputMasterLevel => "Audio Master Level",
        _ => code.name(),
    }
}

fn picture_profile_description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::PictureProfile => {
            "Preset color, gamma, and detail settings for video. PP1-PP10 are customizable. Off uses standard processing."
        }
        DevicePropertyCode::CreativeLook => {
            "Camera-designed color looks for stills and video. Each style (ST, PT, NT, etc.) has a distinct aesthetic."
        }
        DevicePropertyCode::GammaDisplayAssist => {
            "Shows a preview of how log footage will look after color grading. Helps expose correctly without flat-looking preview."
        }
        _ => "",
    }
}

fn picture_profile_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::PictureProfile => "Picture Profile",
        DevicePropertyCode::PictureProfileGamma => "PP Gamma",
        DevicePropertyCode::PictureProfileColorMode => "PP Color Mode",
        DevicePropertyCode::PictureProfileSaturation => "PP Saturation",
        DevicePropertyCode::PictureProfileBlackLevel => "PP Black Level",
        DevicePropertyCode::PictureProfileBlackGammaLevel => "PP Black Gamma",
        DevicePropertyCode::PictureProfileBlackGammaRange => "PP Black Gamma Range",
        DevicePropertyCode::PictureProfileKneeMode => "PP Knee Mode",
        DevicePropertyCode::PictureProfileDetailLevel => "PP Detail Level",
        DevicePropertyCode::CreativeLook => "Creative Look",
        DevicePropertyCode::CreativeLookContrast => "Creative Look: Contrast",
        DevicePropertyCode::CreativeLookHighlights => "Creative Look: Highlights",
        DevicePropertyCode::CreativeLookShadows => "Creative Look: Shadows",
        DevicePropertyCode::CreativeLookSaturation => "Creative Look: Saturation",
        DevicePropertyCode::CreativeLookSharpness => "Creative Look: Sharpness",
        DevicePropertyCode::CreativeLookSharpnessRange => "Creative Look: Sharpness Range",
        DevicePropertyCode::CreativeLookClarity => "Creative Look: Clarity",
        DevicePropertyCode::CreativeLookFade => "Creative Look: Fade",
        DevicePropertyCode::GammaDisplayAssist => "Gamma Display Assist",
        DevicePropertyCode::GammaDisplayAssistType => "Gamma Assist Type",
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
        _ => "",
    }
}

fn other_display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::StreamStatus => "Stream Status",
        DevicePropertyCode::StreamModeSetting => "Stream Mode",
        DevicePropertyCode::StreamDisplayName => "Stream Display Name",
        DevicePropertyCode::StreamLatency => "Stream Latency",
        DevicePropertyCode::FTPFunction => "FTP Function",
        DevicePropertyCode::FTPAutoTransfer => "FTP Auto Transfer",
        DevicePropertyCode::FTPConnectionStatus => "FTP Connection Status",
        DevicePropertyCode::WakeOnLAN => "Wake on LAN",
        DevicePropertyCode::IPSetupProtocolSetting => "IP Setup Protocol",
        DevicePropertyCode::AssignableButton1 => "Assignable Button 1",
        DevicePropertyCode::AssignableButton2 => "Assignable Button 2",
        DevicePropertyCode::AssignableButton3 => "Assignable Button 3",
        DevicePropertyCode::AssignableButton4 => "Assignable Button 4",
        DevicePropertyCode::AssignableButton5 => "Assignable Button 5",
        DevicePropertyCode::LensAssignableButton1 => "Lens Assignable Button",
        DevicePropertyCode::FunctionOfTouchOperation => "Touch Function",
        DevicePropertyCode::TouchFunctionInMF => "Touch Function (MF)",
        DevicePropertyCode::ModelName => "Camera Model",
        DevicePropertyCode::BodySerialNumber => "Serial Number",
        DevicePropertyCode::SoftwareVersion => "Firmware Version",
        DevicePropertyCode::DateTimeSettings => "Date/Time",
        DevicePropertyCode::LanguageSetting => "Language",
        DevicePropertyCode::CameraOperatingMode => "Operating Mode",
        DevicePropertyCode::CameraSettingSaveReadState => "Settings State",
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
