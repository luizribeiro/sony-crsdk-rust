//! Still image quality property types and metadata.

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
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::FileType => "File Type",
        DevicePropertyCode::StillImageQuality => "JPEG Quality",
        DevicePropertyCode::ImageSize => "Image Size",
        DevicePropertyCode::AspectRatio => "Aspect Ratio",
        DevicePropertyCode::StillImageStoreDestination => "Save Destination",
        DevicePropertyCode::RAWFileCompressionType => "RAW Compression",
        DevicePropertyCode::CompressionFileFormatStill => "Compression Format",
        DevicePropertyCode::HighIsoNR => "High ISO Noise Reduction",
        DevicePropertyCode::LongExposureNR => "Long Exposure NR",
        DevicePropertyCode::HLGStillImage => "HLG Still Image",
        DevicePropertyCode::PictureEffect => "Picture Effect",
        DevicePropertyCode::SoftSkinEffect => "Soft Skin Effect",
        _ => code.name(),
    }
}
