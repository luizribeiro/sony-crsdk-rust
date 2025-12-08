//! Image-related property value types.

use std::fmt;

use super::super::traits::PropertyValue;
use crate::types::ToCrsdk;

/// File type for still images (RAW vs JPEG)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FileType {
    /// No file type
    None = 0,
    /// JPEG only
    Jpeg = 1,
    /// RAW only
    Raw = 2,
    /// RAW + JPEG
    RawJpeg = 3,
    /// RAW + HEIF
    RawHeif = 4,
    /// HEIF only
    Heif = 5,
}

impl ToCrsdk<u64> for FileType {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for FileType {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u16 {
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

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::None => "--",
            Self::Jpeg => "JPEG",
            Self::Raw => "RAW",
            Self::RawJpeg => "RAW+JPEG",
            Self::RawHeif => "RAW+HEIF",
            Self::Heif => "HEIF",
        };
        write!(f, "{}", s)
    }
}

/// JPEG image quality level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ImageQuality {
    /// Unknown quality
    Unknown = 0,
    /// Light compression
    Light = 1,
    /// Standard quality
    Standard = 2,
    /// Fine quality
    Fine = 3,
    /// Extra fine quality
    ExFine = 4,
}

impl ToCrsdk<u64> for ImageQuality {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for ImageQuality {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u16 {
            0 => Self::Unknown,
            1 => Self::Light,
            2 => Self::Standard,
            3 => Self::Fine,
            4 => Self::ExFine,
            _ => return None,
        })
    }
}

impl fmt::Display for ImageQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Unknown => "--",
            Self::Light => "Light",
            Self::Standard => "Standard",
            Self::Fine => "Fine",
            Self::ExFine => "Extra Fine",
        };
        write!(f, "{}", s)
    }
}

/// Aspect ratio settings for still images
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AspectRatio {
    /// 3:2 aspect ratio (standard full-frame)
    Ratio3x2 = 1,
    /// 16:9 aspect ratio (widescreen)
    Ratio16x9 = 2,
    /// 4:3 aspect ratio
    Ratio4x3 = 3,
    /// 1:1 aspect ratio (square)
    Ratio1x1 = 4,
}

impl ToCrsdk<u64> for AspectRatio {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for AspectRatio {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u8 {
            1 => Self::Ratio3x2,
            2 => Self::Ratio16x9,
            3 => Self::Ratio4x3,
            4 => Self::Ratio1x1,
            _ => return None,
        })
    }
}

impl fmt::Display for AspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ratio3x2 => "3:2",
            Self::Ratio16x9 => "16:9",
            Self::Ratio4x3 => "4:3",
            Self::Ratio1x1 => "1:1",
        };
        write!(f, "{}", s)
    }
}

/// Image size settings for still images
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ImageSize {
    /// Large (full resolution)
    Large = 1,
    /// Medium resolution
    Medium = 2,
    /// Small resolution
    Small = 3,
    /// VGA resolution (640x480)
    Vga = 4,
}

impl ToCrsdk<u64> for ImageSize {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for ImageSize {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u8 {
            1 => Self::Large,
            2 => Self::Medium,
            3 => Self::Small,
            4 => Self::Vga,
            _ => return None,
        })
    }
}

impl fmt::Display for ImageSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Large => "L",
            Self::Medium => "M",
            Self::Small => "S",
            Self::Vga => "VGA",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_display() {
        assert_eq!(FileType::Jpeg.to_string(), "JPEG");
        assert_eq!(FileType::Raw.to_string(), "RAW");
        assert_eq!(FileType::RawJpeg.to_string(), "RAW+JPEG");
    }

    #[test]
    fn test_image_quality_display() {
        assert_eq!(ImageQuality::ExFine.to_string(), "Extra Fine");
        assert_eq!(ImageQuality::Standard.to_string(), "Standard");
    }

    #[test]
    fn test_aspect_ratio_display() {
        assert_eq!(AspectRatio::Ratio3x2.to_string(), "3:2");
        assert_eq!(AspectRatio::Ratio16x9.to_string(), "16:9");
    }

    #[test]
    fn test_image_size_display() {
        assert_eq!(ImageSize::Large.to_string(), "L");
        assert_eq!(ImageSize::Small.to_string(), "S");
    }
}
