//! Movie/video recording property value types.

use std::fmt;

use super::super::traits::PropertyValue;
use crate::error::{Error, Result};
use crate::types::{FromCrsdk, ToCrsdk};

/// Movie recording quality/bitrate setting.
///
/// The SDK uses numeric codes to represent different recording quality presets.
/// Each code maps to a specific combination of frame rate, bitrate, and bit depth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MovieQuality(u64);

impl MovieQuality {
    /// Get the raw quality code.
    pub fn code(&self) -> u64 {
        self.0
    }
}

impl ToCrsdk<u64> for MovieQuality {
    fn to_crsdk(&self) -> u64 {
        self.0
    }
}

impl FromCrsdk<u64> for MovieQuality {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(MovieQuality(raw))
    }
}

impl PropertyValue for MovieQuality {}

impl fmt::Display for MovieQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.0 as u16;
        let s = match value {
            0 => "--",
            1 => "60p 50M",
            2 => "30p 50M",
            3 => "24p 50M",
            4 => "50p 50M",
            5 => "25p 50M",
            6 => "60i 24M",
            7 => "50i 24M FX",
            8 => "60i 17M FH",
            9 => "50i 17M FH",
            10 => "60p 28M PS",
            11 => "50p 28M PS",
            12 => "24p 24M FX",
            13 => "25p 24M FX",
            14 => "24p 17M FH",
            15 => "25p 17M FH",
            16 => "120p 50M 720",
            17 => "100p 50M 720",
            18 => "1080 30p 16M",
            19 => "1080 25p 16M",
            20 => "720 30p 6M",
            21 => "720 25p 6M",
            22 => "1080 60p 28M",
            23 => "1080 50p 28M",
            24 => "60p 25M",
            25 => "50p 25M",
            26 => "30p 16M",
            27 => "25p 16M",
            28 => "120p 100M",
            29 => "100p 100M",
            30 => "120p 60M",
            31 => "100p 60M",
            32 => "30p 100M",
            33 => "25p 100M",
            34 => "24p 100M",
            35 => "30p 60M",
            36 => "25p 60M",
            37 => "24p 60M",
            38 => "600M 10bit",
            39 => "500M 10bit",
            40 => "400M 10bit",
            41 => "300M 10bit",
            42 => "280M 10bit",
            43 => "250M 10bit",
            44 => "240M 10bit",
            45 => "222M 10bit",
            46 => "200M 10bit",
            47 => "200M 10bit 420",
            48 => "200M 8bit",
            49 => "185M 10bit",
            50 => "150M 10bit 420",
            51 => "150M 8bit",
            52 => "140M 10bit",
            53 => "111M 10bit",
            54 => "100M 10bit",
            55 => "100M 10bit 420",
            56 => "100M 8bit",
            57 => "93M 10bit",
            58 => "89M 10bit",
            59 => "75M 10bit 420",
            60 => "60M 8bit",
            61 => "50M 10bit",
            62 => "50M 10bit 420",
            63 => "50M 8bit",
            64 => "45M 10bit 420",
            65 => "30M 10bit 420",
            66 => "25M 8bit",
            67 => "16M 8bit",
            68 => "520M 10bit",
            69 => "260M 10bit",
            _ => return write!(f, "{}M", value),
        };
        write!(f, "{}", s)
    }
}

/// Movie file format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MovieFileFormat {
    /// AVCHD format
    Avchd = 0,
    /// MP4 format
    Mp4 = 1,
    /// XAVC S 4K format
    XavcS4k = 2,
    /// XAVC S HD format
    XavcSHd = 3,
    /// XAVC HS 8K format
    XavcHs8k = 4,
    /// XAVC HS 4K format
    XavcHs4k = 5,
    /// XAVC S-L 4K format
    XavcSL4k = 6,
    /// XAVC S-L HD format
    XavcSLHd = 7,
    /// XAVC S-I 4K format
    XavcSI4k = 8,
    /// XAVC S-I HD format
    XavcSIHd = 9,
    /// XAVC I format
    XavcI = 10,
    /// XAVC L format
    XavcL = 11,
    /// XAVC HS HD format
    XavcHsHd = 12,
    /// XAVC S-I DCI 4K format
    XavcSIDci4k = 13,
    /// XAVC H-I HQ format
    XavcHIHq = 14,
    /// XAVC H-I SQ format
    XavcHISq = 15,
    /// XAVC H-L format
    XavcHL = 16,
    /// X-OCN XT format
    XOcnXt = 17,
    /// X-OCN ST format
    XOcnSt = 18,
}

impl ToCrsdk<u64> for MovieFileFormat {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for MovieFileFormat {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
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
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for MovieFileFormat {}

impl fmt::Display for MovieFileFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Avchd => write!(f, "AVCHD"),
            Self::Mp4 => write!(f, "MP4"),
            Self::XavcS4k => write!(f, "XAVC S 4K"),
            Self::XavcSHd => write!(f, "XAVC S HD"),
            Self::XavcHs8k => write!(f, "XAVC HS 8K"),
            Self::XavcHs4k => write!(f, "XAVC HS 4K"),
            Self::XavcSL4k => write!(f, "XAVC S-L 4K"),
            Self::XavcSLHd => write!(f, "XAVC S-L HD"),
            Self::XavcSI4k => write!(f, "XAVC S-I 4K"),
            Self::XavcSIHd => write!(f, "XAVC S-I HD"),
            Self::XavcI => write!(f, "XAVC I"),
            Self::XavcL => write!(f, "XAVC L"),
            Self::XavcHsHd => write!(f, "XAVC HS HD"),
            Self::XavcSIDci4k => write!(f, "XAVC S-I DCI 4K"),
            Self::XavcHIHq => write!(f, "XAVC H-I HQ"),
            Self::XavcHISq => write!(f, "XAVC H-I SQ"),
            Self::XavcHL => write!(f, "XAVC H-L"),
            Self::XOcnXt => write!(f, "X-OCN XT"),
            Self::XOcnSt => write!(f, "X-OCN ST"),
        }
    }
}

/// Recording state for movie recording.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum RecordingState {
    /// Not currently recording
    NotRecording = 0x0000,
    /// Actively recording
    Recording = 0x0001,
    /// Recording failed
    RecordingFailed = 0x0002,
    /// Interval recording - waiting for next recording
    IntervalWaiting = 0x0003,
}

impl ToCrsdk<u64> for RecordingState {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for RecordingState {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0000 => Self::NotRecording,
            0x0001 => Self::Recording,
            0x0002 => Self::RecordingFailed,
            0x0003 => Self::IntervalWaiting,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for RecordingState {}

impl fmt::Display for RecordingState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRecording => write!(f, "Stopped"),
            Self::Recording => write!(f, "Recording"),
            Self::RecordingFailed => write!(f, "Failed"),
            Self::IntervalWaiting => write!(f, "Waiting"),
        }
    }
}

/// Timecode format (drop frame vs non-drop frame).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TimeCodeFormat {
    /// Drop frame timecode (for NTSC 29.97fps)
    DropFrame = 0x01,
    /// Non-drop frame timecode
    NonDropFrame = 0x02,
}

impl ToCrsdk<u64> for TimeCodeFormat {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for TimeCodeFormat {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::DropFrame,
            0x02 => Self::NonDropFrame,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for TimeCodeFormat {}

impl fmt::Display for TimeCodeFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DropFrame => write!(f, "DF"),
            Self::NonDropFrame => write!(f, "NDF"),
        }
    }
}

/// Timecode run mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TimeCodeRun {
    /// Record run - timecode advances only during recording
    RecRun = 0x01,
    /// Free run - timecode advances continuously
    FreeRun = 0x02,
}

impl ToCrsdk<u64> for TimeCodeRun {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for TimeCodeRun {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::RecRun,
            0x02 => Self::FreeRun,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for TimeCodeRun {}

impl fmt::Display for TimeCodeRun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RecRun => write!(f, "Rec Run"),
            Self::FreeRun => write!(f, "Free Run"),
        }
    }
}

/// Timecode generation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TimeCodeMake {
    /// Use preset timecode value
    Preset = 0x01,
    /// Regenerate from previous recording
    Regenerate = 0x02,
}

impl ToCrsdk<u64> for TimeCodeMake {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for TimeCodeMake {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Preset,
            0x02 => Self::Regenerate,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for TimeCodeMake {}

impl fmt::Display for TimeCodeMake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Preset => write!(f, "Preset"),
            Self::Regenerate => write!(f, "Regen"),
        }
    }
}

/// Recorder status (main or proxy).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RecorderStatus {
    /// Recorder is idle
    Idle = 0x00,
    /// Ready to record
    Ready = 0x01,
    /// Preparing to record
    Preparing = 0x02,
    /// In standby mode
    Standby = 0x03,
    /// Actively recording
    Recording = 0x04,
    /// Stopping recording
    Stopping = 0x05,
}

impl ToCrsdk<u64> for RecorderStatus {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for RecorderStatus {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x00 => Self::Idle,
            0x01 => Self::Ready,
            0x02 => Self::Preparing,
            0x03 => Self::Standby,
            0x04 => Self::Recording,
            0x05 => Self::Stopping,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for RecorderStatus {}

impl fmt::Display for RecorderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::Ready => write!(f, "Ready"),
            Self::Preparing => write!(f, "Preparing"),
            Self::Standby => write!(f, "Standby"),
            Self::Recording => write!(f, "Recording"),
            Self::Stopping => write!(f, "Stopping"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movie_quality_known_values() {
        assert_eq!(MovieQuality(0).to_string(), "--");
        assert_eq!(MovieQuality(38).to_string(), "600M 10bit");
        assert_eq!(MovieQuality(56).to_string(), "100M 8bit");
    }

    #[test]
    fn test_movie_quality_unknown_fallback() {
        assert_eq!(MovieQuality(999).to_string(), "999M");
    }

    #[test]
    fn test_movie_quality_round_trip() {
        let mq = MovieQuality::from_raw(38).unwrap();
        assert_eq!(mq.to_raw(), 38);
    }
}
