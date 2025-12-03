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
    /// Still image quality
    ImageQuality = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_StillImageQuality,
    /// Movie file format
    MovieFormat = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_Movie_File_Format,
    /// Movie recording setting
    MovieRecordingSetting =
        crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_Movie_Recording_Setting,
    /// Zoom scale
    ZoomScale = crsdk_sys::SCRSDK::CrDevicePropertyCode_CrDeviceProperty_Zoom_Scale,
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
            CrDevicePropertyCode_CrDeviceProperty_Movie_File_Format => Self::MovieFormat,
            CrDevicePropertyCode_CrDeviceProperty_Movie_Recording_Setting => {
                Self::MovieRecordingSetting
            }
            CrDevicePropertyCode_CrDeviceProperty_Zoom_Scale => Self::ZoomScale,
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

// TODO: Add ExposureProgram enum
// Map all CrExposureProgram_* constants from crsdk_sys::SCRSDK
// Including: M, P, A, S, Auto, Movie modes, Scene modes, S&Q motion, etc.
// Reference: CrExposureProgram_CrExposure_M_Manual, CrExposureProgram_CrExposure_P_Auto, etc.

// TODO: Add DriveMode enum
// Map all CrDriveMode_* constants from crsdk_sys::SCRSDK
// Including: Single, Continuous (Hi/Mid/Lo), Timer, Bracket variations, Burst, etc.
// Reference: CrDriveMode_CrDrive_Single, CrDriveMode_CrDrive_Continuous_Hi, etc.

// TODO: Add MeteringMode enum
// Map all CrMeteringMode_* constants from crsdk_sys::SCRSDK

// TODO: Add FlashMode enum
// Map all CrFlashMode_* constants from crsdk_sys::SCRSDK

// TODO: Add FocusArea enum
// Map all CrFocusArea_* constants from crsdk_sys::SCRSDK

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
        match value {
            CrDataType_CrDataType_UInt8 => Self::UInt8,
            CrDataType_CrDataType_UInt16 => Self::UInt16,
            CrDataType_CrDataType_UInt32 => Self::UInt32,
            CrDataType_CrDataType_UInt64 => Self::UInt64,
            CrDataType_CrDataType_Int8 => Self::Int8,
            CrDataType_CrDataType_Int16 => Self::Int16,
            CrDataType_CrDataType_Int32 => Self::Int32,
            CrDataType_CrDataType_Int64 => Self::Int64,
            CrDataType_CrDataType_STR => Self::String,
            other => Self::Unknown(other),
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

/// Convert SDK CrDeviceProperty to our DeviceProperty
pub(crate) unsafe fn device_property_from_sdk(
    prop: &crsdk_sys::SCRSDK::CrDeviceProperty,
) -> DeviceProperty {
    let data_type = DataType::from_sdk(prop.valueType);
    DeviceProperty {
        code: prop.code,
        data_type,
        enable_flag: EnableFlag::from_sdk(prop.enableFlag),
        current_value: prop.currentValue,
        possible_values: parse_possible_values(data_type, prop.values, prop.valuesSize),
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
            possible_values: vec![],
        };
        assert!(prop_empty.is_valid_value(999));
    }
}
