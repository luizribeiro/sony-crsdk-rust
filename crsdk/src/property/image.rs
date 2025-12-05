//! Still image quality property types and metadata.

use super::PropertyValueType;
use crsdk_sys::DevicePropertyCode;

/// File type for still images (RAW vs JPEG)
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
        match self {
            Self::None => write!(f, "--"),
            Self::Jpeg => write!(f, "JPEG"),
            Self::Raw => write!(f, "RAW"),
            Self::RawJpeg => write!(f, "RAW+JPEG"),
            Self::RawHeif => write!(f, "RAW+HEIF"),
            Self::Heif => write!(f, "HEIF"),
        }
    }
}

/// JPEG image quality level
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
        match self {
            Self::Unknown => write!(f, "--"),
            Self::Light => write!(f, "Light"),
            Self::Standard => write!(f, "Standard"),
            Self::Fine => write!(f, "Fine"),
            Self::ExFine => write!(f, "Extra Fine"),
        }
    }
}

/// Aspect ratio settings for still images
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AspectRatio {
    Ratio3x2 = 1,
    Ratio16x9 = 2,
    Ratio4x3 = 3,
    Ratio1x1 = 4,
}

impl AspectRatio {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            1 => Self::Ratio3x2,
            2 => Self::Ratio16x9,
            3 => Self::Ratio4x3,
            4 => Self::Ratio1x1,
            _ => return None,
        })
    }
}

impl std::fmt::Display for AspectRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ratio3x2 => write!(f, "3:2"),
            Self::Ratio16x9 => write!(f, "16:9"),
            Self::Ratio4x3 => write!(f, "4:3"),
            Self::Ratio1x1 => write!(f, "1:1"),
        }
    }
}

/// Image size settings for still images
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ImageSize {
    Large = 1,
    Medium = 2,
    Small = 3,
    Vga = 4,
}

impl ImageSize {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
            1 => Self::Large,
            2 => Self::Medium,
            3 => Self::Small,
            4 => Self::Vga,
            _ => return None,
        })
    }
}

impl std::fmt::Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Large => write!(f, "L"),
            Self::Medium => write!(f, "M"),
            Self::Small => write!(f, "S"),
            Self::Vga => write!(f, "VGA"),
        }
    }
}

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::FileType => {
            "Choose between RAW (unprocessed sensor data for maximum editing flexibility), JPEG (processed and compressed), or RAW+JPEG (both files saved)."
        }
        DevicePropertyCode::StillImageQuality => {
            "JPEG compression level. Extra Fine has minimal compression (best quality, larger files). Standard has more compression (smaller files, some quality loss)."
        }
        DevicePropertyCode::ImageSize => {
            "Resolution of captured images. Larger sizes have more detail for printing/cropping. Smaller sizes save storage space."
        }
        DevicePropertyCode::AspectRatio => {
            "Image shape. 3:2 is standard for full-frame. 16:9 is widescreen. 1:1 is square. 4:3 matches micro four-thirds sensors."
        }
        DevicePropertyCode::RAWFileCompressionType => {
            "RAW file compression. Uncompressed preserves all data. Lossless compressed reduces size without quality loss. Compressed is smallest but may lose some data."
        }
        DevicePropertyCode::HighIsoNR => {
            "Reduces noise in high ISO images. Higher settings smooth noise but may soften fine detail. Applies to JPEGs."
        }
        DevicePropertyCode::LongExposureNR => {
            "Reduces hot pixels and noise in long exposures by taking a 'dark frame' subtraction. Doubles the capture time."
        }
        DevicePropertyCode::StillImageStoreDestination => {
            "Which memory card slot to save still images to. Slot 1, Slot 2, or both slots for simultaneous backup."
        }
        DevicePropertyCode::FileSettingsCameraId => {
            "Camera identifier embedded in file metadata. Useful for multi-camera shoots."
        }
        DevicePropertyCode::FileSettingsCameraPosition => {
            "Camera position identifier for multi-camera workflows."
        }
        DevicePropertyCode::FileSettingsReelNumber => {
            "Reel number for organizing footage. Auto-increments or can be set manually."
        }
        DevicePropertyCode::FileSettingsTitleNameSettings => {
            "Custom title name prefix for files. Helps identify footage from different shoots."
        }
        DevicePropertyCode::CompressionFileFormatStill => {
            "File format used for compressed still images."
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
        DevicePropertyCode::RemoteSaveImageSize => {
            "Image resolution when saving to a remote destination."
        }
        DevicePropertyCode::HLGStillImage => {
            "Enables HLG (Hybrid Log-Gamma) for still images. Provides HDR-compatible captures."
        }
        DevicePropertyCode::PictureEffect => {
            "Creative filters applied to images in-camera. Includes toy camera, posterization, etc."
        }
        DevicePropertyCode::SoftSkinEffect => {
            "Smooths skin tones in portraits. Higher settings provide more smoothing."
        }
        // Picture Profile properties route to Image category
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
        DevicePropertyCode::PictureProfileSaturation | DevicePropertyCode::PictureProfileColorPhase => {
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
        DevicePropertyCode::DisplayQualityFinder => {
            "Display quality setting for the electronic viewfinder."
        }
        DevicePropertyCode::LiveViewImageQuality => {
            "Quality setting for live view stream. Higher quality uses more bandwidth."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::FileType => "File Format",
        DevicePropertyCode::StillImageQuality => "JPEG Quality",
        DevicePropertyCode::ImageSize => "Size",
        DevicePropertyCode::AspectRatio => "Aspect",
        DevicePropertyCode::StillImageStoreDestination => "Save Destination",
        DevicePropertyCode::RAWFileCompressionType => "RAW Compression",
        DevicePropertyCode::CompressionFileFormatStill => "Compression Format",
        DevicePropertyCode::HighIsoNR => "High ISO Noise Reduction",
        DevicePropertyCode::LongExposureNR => "Long Exposure NR",
        DevicePropertyCode::HLGStillImage => "HLG Still Image",
        DevicePropertyCode::PictureEffect => "Picture Effect",
        DevicePropertyCode::SoftSkinEffect => "Soft Skin Effect",
        DevicePropertyCode::FileSettingsCameraId => "Camera ID",
        DevicePropertyCode::FileSettingsCameraPosition => "Camera Position",
        DevicePropertyCode::FileSettingsReelNumber => "Reel Number",
        DevicePropertyCode::FileSettingsTitleNameSettings => "Title Name",
        DevicePropertyCode::MediaSLOT1FileType => "Slot 1 File Type",
        DevicePropertyCode::MediaSLOT2FileType => "Slot 2 File Type",
        DevicePropertyCode::MediaSLOT1ImageSize => "Slot 1 Image Size",
        DevicePropertyCode::MediaSLOT2ImageSize => "Slot 2 Image Size",
        DevicePropertyCode::MediaSLOT1RAWFileCompressionType => "Slot 1 RAW Comp",
        DevicePropertyCode::MediaSLOT2RAWFileCompressionType => "Slot 2 RAW Comp",
        DevicePropertyCode::RemoteSaveImageSize => "Remote Save Size",
        // Picture Profile properties route to Image category
        DevicePropertyCode::PictureProfile => "Pict. Profile",
        DevicePropertyCode::PictureProfileGamma => "PP Gamma",
        DevicePropertyCode::PictureProfileColorMode => "PP Color Mode",
        DevicePropertyCode::PictureProfileSaturation => "PP Saturation",
        DevicePropertyCode::PictureProfileBlackLevel => "PP Black Level",
        DevicePropertyCode::PictureProfileBlackGammaLevel => "PP Black Gamma",
        DevicePropertyCode::PictureProfileBlackGammaRange => "PP BG Range",
        DevicePropertyCode::PictureProfileKneeMode => "PP Knee Mode",
        DevicePropertyCode::PictureProfileKneeAutoSetMaxPoint => "PP Knee Auto Max",
        DevicePropertyCode::PictureProfileKneeAutoSetSensitivity => "PP Knee Sens",
        DevicePropertyCode::PictureProfileKneeManualSetPoint => "PP Knee Point",
        DevicePropertyCode::PictureProfileKneeManualSetSlope => "PP Knee Slope",
        DevicePropertyCode::PictureProfileDetailLevel => "PP Detail Level",
        DevicePropertyCode::PictureProfileDetailAdjustMode => "PP Detail Mode",
        DevicePropertyCode::PictureProfileDetailAdjustVHBalance => "PP Detail V/H",
        DevicePropertyCode::PictureProfileDetailAdjustBWBalance => "PP Detail B/W",
        DevicePropertyCode::PictureProfileDetailAdjustLimit => "PP Detail Limit",
        DevicePropertyCode::PictureProfileDetailAdjustCrispening => "PP Crisp",
        DevicePropertyCode::PictureProfileDetailAdjustHiLightDetail => "PP HiLight",
        DevicePropertyCode::PictureProfileColorPhase => "PP Color Phase",
        DevicePropertyCode::PictureProfileColorDepthRed => "PP Red",
        DevicePropertyCode::PictureProfileColorDepthGreen => "PP Green",
        DevicePropertyCode::PictureProfileColorDepthBlue => "PP Blue",
        DevicePropertyCode::PictureProfileColorDepthCyan => "PP Cyan",
        DevicePropertyCode::PictureProfileColorDepthMagenta => "PP Magenta",
        DevicePropertyCode::PictureProfileColorDepthYellow => "PP Yellow",
        DevicePropertyCode::PictureProfileCopy => "PP Copy",
        DevicePropertyCode::PictureProfileResetEnableStatus => "PP Reset Avail",
        DevicePropertyCode::DisplayQualityFinder => "Finder Quality",
        DevicePropertyCode::LiveViewImageQuality => "LV Quality",
        _ => code.name(),
    }
}

pub fn value_type(code: DevicePropertyCode) -> Option<PropertyValueType> {
    use PropertyValueType as V;

    Some(match code {
        DevicePropertyCode::FileType
        | DevicePropertyCode::MediaSLOT1FileType
        | DevicePropertyCode::MediaSLOT2FileType
        | DevicePropertyCode::StillImageStoreDestination => V::FileType,
        DevicePropertyCode::StillImageQuality => V::ImageQuality,
        DevicePropertyCode::AspectRatio => V::AspectRatio,
        DevicePropertyCode::ImageSize
        | DevicePropertyCode::MediaSLOT1ImageSize
        | DevicePropertyCode::MediaSLOT2ImageSize
        | DevicePropertyCode::RemoteSaveImageSize => V::ImageSize,
        DevicePropertyCode::RAWFileCompressionType
        | DevicePropertyCode::MediaSLOT1RAWFileCompressionType
        | DevicePropertyCode::MediaSLOT2RAWFileCompressionType
        | DevicePropertyCode::CompressionFileFormatStill
        | DevicePropertyCode::FileSettingsCameraId
        | DevicePropertyCode::FileSettingsCameraPosition
        | DevicePropertyCode::FileSettingsReelNumber
        | DevicePropertyCode::FileSettingsTitleNameSettings
        | DevicePropertyCode::HighIsoNR
        | DevicePropertyCode::LongExposureNR
        | DevicePropertyCode::HLGStillImage
        | DevicePropertyCode::PictureEffect
        | DevicePropertyCode::SoftSkinEffect
        // Picture Profile properties route to Image category
        | DevicePropertyCode::PictureProfile
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
        | DevicePropertyCode::DisplayQualityFinder
        | DevicePropertyCode::LiveViewImageQuality => V::Integer,
        _ => return None,
    })
}
