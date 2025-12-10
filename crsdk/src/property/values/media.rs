//! Media/storage-related property value types.

use std::fmt;

use super::super::traits::PropertyValue;
use crate::error::{Error, Result};
use crate::types::{FromCrsdk, ToCrsdk};

/// Live view status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum LiveViewStatus {
    /// Live view is not supported on this device
    NotSupported = 0x0000,
    /// Live view is supported but currently disabled
    Disabled = 0x0001,
    /// Live view is active and available
    Enabled = 0x0002,
}

impl ToCrsdk<u64> for LiveViewStatus {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for LiveViewStatus {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0000 => Self::NotSupported,
            0x0001 => Self::Disabled,
            0x0002 => Self::Enabled,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for LiveViewStatus {}

impl fmt::Display for LiveViewStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotSupported => write!(f, "Not Supported"),
            Self::Disabled => write!(f, "Disabled"),
            Self::Enabled => write!(f, "Enabled"),
        }
    }
}

/// Memory card slot status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum SlotStatus {
    /// Card is OK and ready
    Ok = 0x0000,
    /// No card inserted
    NoCard = 0x0001,
    /// Card error
    CardError = 0x0002,
    /// Card is being recognized or is locked
    RecognizingOrLocked = 0x0003,
    /// Database error on card
    DbError = 0x0004,
    /// Card is being recognized
    Recognizing = 0x0005,
    /// Card locked and database error
    LockedAndDbError = 0x0006,
    /// Database error that requires format
    DbErrorNeedFormat = 0x0007,
    /// Card is read-only
    ReadOnly = 0x0008,
}

impl ToCrsdk<u64> for SlotStatus {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for SlotStatus {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0000 => Self::Ok,
            0x0001 => Self::NoCard,
            0x0002 => Self::CardError,
            0x0003 => Self::RecognizingOrLocked,
            0x0004 => Self::DbError,
            0x0005 => Self::Recognizing,
            0x0006 => Self::LockedAndDbError,
            0x0007 => Self::DbErrorNeedFormat,
            0x0008 => Self::ReadOnly,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for SlotStatus {}

impl fmt::Display for SlotStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok => write!(f, "OK"),
            Self::NoCard => write!(f, "No Card"),
            Self::CardError => write!(f, "Error"),
            Self::RecognizingOrLocked => write!(f, "Locked"),
            Self::DbError => write!(f, "DB Error"),
            Self::Recognizing => write!(f, "Reading..."),
            Self::LockedAndDbError => write!(f, "Locked/Error"),
            Self::DbErrorNeedFormat => write!(f, "Need Format"),
            Self::ReadOnly => write!(f, "Read Only"),
        }
    }
}

/// Media slot writing state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MediaSlotWritingState {
    /// Not currently writing
    NotWriting = 0x01,
    /// Contents being written
    Writing = 0x02,
}

impl ToCrsdk<u64> for MediaSlotWritingState {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for MediaSlotWritingState {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::NotWriting,
            0x02 => Self::Writing,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for MediaSlotWritingState {}

impl fmt::Display for MediaSlotWritingState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotWriting => write!(f, "Idle"),
            Self::Writing => write!(f, "Writing"),
        }
    }
}

/// Media slot recording availability type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MediaSlotRecordingType {
    /// No recording available
    None = 0x00,
    /// Main recording only
    Main = 0x01,
    /// Proxy recording only
    Proxy = 0x02,
    /// Both main and proxy recording
    MainAndProxy = 0x03,
}

impl ToCrsdk<u64> for MediaSlotRecordingType {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for MediaSlotRecordingType {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x00 => Self::None,
            0x01 => Self::Main,
            0x02 => Self::Proxy,
            0x03 => Self::MainAndProxy,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for MediaSlotRecordingType {}

impl fmt::Display for MediaSlotRecordingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Main => write!(f, "Main"),
            Self::Proxy => write!(f, "Proxy"),
            Self::MainAndProxy => write!(f, "Main+Proxy"),
        }
    }
}
