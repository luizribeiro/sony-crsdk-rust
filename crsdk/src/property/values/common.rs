//! Common property value types used across multiple domains.

use std::fmt;

use super::super::traits::PropertyValue;
use crate::error::{Error, Result};
use crate::types::{FromCrsdk, ToCrsdk};

/// How to interpret and format a property's raw value.
///
/// This enum defines the semantic type of a property value, allowing type-safe
/// formatting and parsing. Use [`property_value_type`](super::super::property_value_type)
/// to get the type for a property code.
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
    /// Focus indicator (lock state)
    FocusIndicator,
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
    /// Movie recording state
    RecordingState,
    /// Recorder status (main or proxy)
    RecorderStatus,
    /// Timecode format (DF/NDF)
    TimeCodeFormat,
    /// Timecode run mode
    TimeCodeRun,
    /// Timecode make mode
    TimeCodeMake,
    /// Live view status
    LiveViewStatus,
    /// Memory card slot status
    SlotStatus,
    /// Media slot writing state
    MediaSlotWritingState,
    /// Media slot recording type
    MediaSlotRecordingType,
    /// SDK control mode
    SdkControlMode,
    /// Dynamic range optimizer
    DRangeOptimizer,
    /// Still image store destination
    StillImageStoreDestination,
    /// Near/far focus control status
    NearFarEnableStatus,
    /// Interval recording mode
    IntervalRecMode,
    /// Interval recording status
    IntervalRecStatus,
    /// Priority key settings
    PriorityKeySettings,
    /// Color space (sRGB/Adobe RGB)
    ColorSpace,
    /// Playback media slot
    PlaybackMedia,
    /// Touch operation mode
    TouchOperation,
    /// Power source type
    PowerSource,
    /// Battery remaining display unit
    BatteryRemainDisplayUnit,
    /// Focus operation direction
    FocusOperation,
    /// Shutter type (auto/mechanical/electronic)
    ShutterType,
    /// Device overheating state
    DeviceOverheatingState,
    /// High ISO noise reduction
    HighIsoNR,
    /// Audio signals (beep) setting
    AudioSignals,
    /// Touch operation function
    FunctionOfTouchOperation,
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

/// A generic integer property value.
///
/// Used for properties that represent raw numeric values without special formatting.
/// The value is interpreted as signed to handle properties like focus position offsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Integer(i64);

impl Integer {
    /// Get the integer value.
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl ToCrsdk<u64> for Integer {
    fn to_crsdk(&self) -> u64 {
        self.0 as u64
    }
}

impl FromCrsdk<u64> for Integer {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(Integer(raw as i64))
    }
}

impl PropertyValue for Integer {}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A percentage value (0-100 or similar ranges).
///
/// Used for properties like battery remaining, volume levels, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Percentage(u64);

impl Percentage {
    /// Get the percentage value.
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl ToCrsdk<u64> for Percentage {
    fn to_crsdk(&self) -> u64 {
        self.0
    }
}

impl FromCrsdk<u64> for Percentage {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(Percentage(raw))
    }
}

impl PropertyValue for Percentage {}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

/// A generic on/off switch (Off=1, On=2).
///
/// Used for properties like AutoSlowShutter, SilentMode, and NDFilter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Switch {
    /// Switch is off
    Off = 1,
    /// Switch is on
    On = 2,
}

impl ToCrsdk<u64> for Switch {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for Switch {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            1 => Self::Off,
            2 => Self::On,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for Switch {}

impl fmt::Display for Switch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::On => write!(f, "On"),
        }
    }
}

/// A generic on/off toggle (Off=0, On=1).
///
/// Used for properties like RedEyeReduction and AudioRecording.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OnOff {
    /// Setting is off
    Off = 0,
    /// Setting is on
    On = 1,
}

impl ToCrsdk<u64> for OnOff {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for OnOff {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0 => Self::Off,
            1 => Self::On,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for OnOff {}

impl fmt::Display for OnOff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::On => write!(f, "On"),
        }
    }
}

/// Automatic/Manual mode setting (Automatic=1, Manual=2).
///
/// Used for IrisModeSetting, ShutterModeSetting, GainControlSetting, and NDFilterModeSetting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AutoManual {
    /// Automatic mode
    Automatic = 1,
    /// Manual mode
    Manual = 2,
}

impl ToCrsdk<u64> for AutoManual {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for AutoManual {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            1 => Self::Automatic,
            2 => Self::Manual,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for AutoManual {}

impl fmt::Display for AutoManual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Automatic => write!(f, "Auto"),
            Self::Manual => write!(f, "Manual"),
        }
    }
}

/// Lock indicator (Unknown=0, Unlocked=1, Locked=2).
///
/// Used for white balance lock and similar features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum LockIndicator {
    /// Lock status is unknown
    Unknown = 0,
    /// Setting is unlocked
    Unlocked = 1,
    /// Setting is locked
    Locked = 2,
}

impl ToCrsdk<u64> for LockIndicator {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for LockIndicator {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0 => Self::Unknown,
            1 => Self::Unlocked,
            2 => Self::Locked,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for LockIndicator {}

impl fmt::Display for LockIndicator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Unlocked => write!(f, "Unlocked"),
            Self::Locked => write!(f, "Locked"),
        }
    }
}

/// Live view display effect setting (Unknown=0, On=1, Off=2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LiveViewDisplayEffect {
    /// Status unknown
    Unknown = 0,
    /// Effect is enabled
    On = 1,
    /// Effect is disabled
    Off = 2,
}

impl ToCrsdk<u64> for LiveViewDisplayEffect {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for LiveViewDisplayEffect {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0 => Self::Unknown,
            1 => Self::On,
            2 => Self::Off,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for LiveViewDisplayEffect {}

impl fmt::Display for LiveViewDisplayEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::On => write!(f, "On"),
            Self::Off => write!(f, "Off"),
        }
    }
}

/// Aperture drive behavior in silent mode during AF.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SilentModeApertureDrive {
    /// Aperture drive not affected by silent mode
    NotTarget = 1,
    /// Standard aperture drive behavior
    Standard = 2,
    /// Prioritize silent operation
    SilentPriority = 3,
}

impl ToCrsdk<u64> for SilentModeApertureDrive {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for SilentModeApertureDrive {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            1 => Self::NotTarget,
            2 => Self::Standard,
            3 => Self::SilentPriority,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for SilentModeApertureDrive {}

impl fmt::Display for SilentModeApertureDrive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotTarget => write!(f, "Not Target"),
            Self::Standard => write!(f, "Standard"),
            Self::SilentPriority => write!(f, "Silent Priority"),
        }
    }
}

/// SDK control mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SdkControlMode {
    /// Remote control mode
    Remote = 0x00000000,
    /// Contents transfer mode
    ContentsTransfer = 0x00000001,
    /// Remote transfer mode
    RemoteTransfer = 0x00000002,
}

impl ToCrsdk<u64> for SdkControlMode {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for SdkControlMode {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u32 {
            0x00000000 => Self::Remote,
            0x00000001 => Self::ContentsTransfer,
            0x00000002 => Self::RemoteTransfer,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for SdkControlMode {}

impl fmt::Display for SdkControlMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Remote => write!(f, "Remote"),
            Self::ContentsTransfer => write!(f, "Transfer"),
            Self::RemoteTransfer => write!(f, "Remote Transfer"),
        }
    }
}

/// Dynamic range optimizer setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum DRangeOptimizer {
    /// DRO off
    Off = 0x0000,
    /// DRO on (auto level)
    On = 0x0001,
    /// DRO+ base
    Plus = 0x0010,
    /// DRO+ manual level 1
    PlusManual1 = 0x0011,
    /// DRO+ manual level 2
    PlusManual2 = 0x0012,
    /// DRO+ manual level 3
    PlusManual3 = 0x0013,
    /// DRO+ manual level 4
    PlusManual4 = 0x0014,
    /// DRO+ manual level 5
    PlusManual5 = 0x0015,
    /// DRO auto
    Auto = 0x0020,
    /// HDR auto
    HdrAuto = 0x0030,
    /// HDR 1.0 EV
    Hdr10Ev = 0x0031,
    /// HDR 2.0 EV
    Hdr20Ev = 0x0032,
    /// HDR 3.0 EV
    Hdr30Ev = 0x0033,
    /// HDR 4.0 EV
    Hdr40Ev = 0x0034,
    /// HDR 5.0 EV
    Hdr50Ev = 0x0035,
    /// HDR 6.0 EV
    Hdr60Ev = 0x0036,
}

impl ToCrsdk<u64> for DRangeOptimizer {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for DRangeOptimizer {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0000 => Self::Off,
            0x0001 => Self::On,
            0x0010 => Self::Plus,
            0x0011 => Self::PlusManual1,
            0x0012 => Self::PlusManual2,
            0x0013 => Self::PlusManual3,
            0x0014 => Self::PlusManual4,
            0x0015 => Self::PlusManual5,
            0x0020 => Self::Auto,
            0x0030 => Self::HdrAuto,
            0x0031 => Self::Hdr10Ev,
            0x0032 => Self::Hdr20Ev,
            0x0033 => Self::Hdr30Ev,
            0x0034 => Self::Hdr40Ev,
            0x0035 => Self::Hdr50Ev,
            0x0036 => Self::Hdr60Ev,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for DRangeOptimizer {}

impl fmt::Display for DRangeOptimizer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::On => write!(f, "On"),
            Self::Plus => write!(f, "DRO+"),
            Self::PlusManual1 => write!(f, "DRO+ Lv1"),
            Self::PlusManual2 => write!(f, "DRO+ Lv2"),
            Self::PlusManual3 => write!(f, "DRO+ Lv3"),
            Self::PlusManual4 => write!(f, "DRO+ Lv4"),
            Self::PlusManual5 => write!(f, "DRO+ Lv5"),
            Self::Auto => write!(f, "Auto"),
            Self::HdrAuto => write!(f, "HDR Auto"),
            Self::Hdr10Ev => write!(f, "HDR 1.0EV"),
            Self::Hdr20Ev => write!(f, "HDR 2.0EV"),
            Self::Hdr30Ev => write!(f, "HDR 3.0EV"),
            Self::Hdr40Ev => write!(f, "HDR 4.0EV"),
            Self::Hdr50Ev => write!(f, "HDR 5.0EV"),
            Self::Hdr60Ev => write!(f, "HDR 6.0EV"),
        }
    }
}

/// Still image store destination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum StillImageStoreDestination {
    /// Save to host PC only
    HostPC = 0x0001,
    /// Save to memory card only
    MemoryCard = 0x0002,
    /// Save to both host PC and memory card
    Both = 0x0003,
}

impl ToCrsdk<u64> for StillImageStoreDestination {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for StillImageStoreDestination {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0001 => Self::HostPC,
            0x0002 => Self::MemoryCard,
            0x0003 => Self::Both,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for StillImageStoreDestination {}

impl fmt::Display for StillImageStoreDestination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HostPC => write!(f, "PC"),
            Self::MemoryCard => write!(f, "Card"),
            Self::Both => write!(f, "PC+Card"),
        }
    }
}

/// Near/far focus control enable status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum NearFarEnableStatus {
    /// Near/far control disabled
    Disabled = 0x0000,
    /// Near/far control enabled
    Enabled = 0x0001,
}

impl ToCrsdk<u64> for NearFarEnableStatus {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for NearFarEnableStatus {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0000 => Self::Disabled,
            0x0001 => Self::Enabled,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for NearFarEnableStatus {}

impl fmt::Display for NearFarEnableStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disabled => write!(f, "Disabled"),
            Self::Enabled => write!(f, "Enabled"),
        }
    }
}

/// Interval recording mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum IntervalRecMode {
    /// Interval recording off
    Off = 0x0001,
    /// Interval recording on
    On = 0x0002,
}

impl ToCrsdk<u64> for IntervalRecMode {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for IntervalRecMode {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0001 => Self::Off,
            0x0002 => Self::On,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for IntervalRecMode {}

impl fmt::Display for IntervalRecMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::On => write!(f, "On"),
        }
    }
}

/// Interval recording status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum IntervalRecStatus {
    /// Waiting for interval shooting to start
    WaitingStart = 0x0001,
    /// Interval shooting in progress
    IntervalShooting = 0x0002,
}

impl ToCrsdk<u64> for IntervalRecStatus {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for IntervalRecStatus {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0001 => Self::WaitingStart,
            0x0002 => Self::IntervalShooting,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for IntervalRecStatus {}

impl fmt::Display for IntervalRecStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WaitingStart => write!(f, "Waiting"),
            Self::IntervalShooting => write!(f, "Shooting"),
        }
    }
}

/// Priority key settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum PriorityKeySettings {
    /// Camera position has priority
    CameraPosition = 0x0001,
    /// PC Remote has priority
    PcRemote = 0x0002,
}

impl ToCrsdk<u64> for PriorityKeySettings {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for PriorityKeySettings {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0001 => Self::CameraPosition,
            0x0002 => Self::PcRemote,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for PriorityKeySettings {}

impl fmt::Display for PriorityKeySettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CameraPosition => write!(f, "Camera"),
            Self::PcRemote => write!(f, "PC Remote"),
        }
    }
}

/// Color space setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ColorSpace {
    /// sRGB color space (web/most monitors)
    Srgb = 0x01,
    /// Adobe RGB color space (print/wide gamut)
    AdobeRgb = 0x02,
}

impl ToCrsdk<u64> for ColorSpace {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for ColorSpace {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Srgb,
            0x02 => Self::AdobeRgb,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for ColorSpace {}

impl fmt::Display for ColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Srgb => write!(f, "sRGB"),
            Self::AdobeRgb => write!(f, "Adobe RGB"),
        }
    }
}

/// Playback media slot selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlaybackMedia {
    /// Playback from slot 1
    Slot1 = 0x01,
    /// Playback from slot 2
    Slot2 = 0x02,
}

impl ToCrsdk<u64> for PlaybackMedia {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for PlaybackMedia {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Slot1,
            0x02 => Self::Slot2,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for PlaybackMedia {}

impl fmt::Display for PlaybackMedia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Slot1 => write!(f, "Slot 1"),
            Self::Slot2 => write!(f, "Slot 2"),
        }
    }
}

/// Touch operation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TouchOperation {
    /// Touch disabled
    Off = 0x01,
    /// Touch enabled
    On = 0x02,
    /// Touch enabled only during playback
    PlaybackOnly = 0x03,
}

impl ToCrsdk<u64> for TouchOperation {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for TouchOperation {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Off,
            0x02 => Self::On,
            0x03 => Self::PlaybackOnly,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for TouchOperation {}

impl fmt::Display for TouchOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::On => write!(f, "On"),
            Self::PlaybackOnly => write!(f, "Playback Only"),
        }
    }
}

/// Power source type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PowerSource {
    /// DC power (external adapter)
    Dc = 0x01,
    /// Battery power
    Battery = 0x02,
    /// Power over Ethernet
    PoE = 0x03,
}

impl ToCrsdk<u64> for PowerSource {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for PowerSource {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Dc,
            0x02 => Self::Battery,
            0x03 => Self::PoE,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for PowerSource {}

impl fmt::Display for PowerSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dc => write!(f, "DC"),
            Self::Battery => write!(f, "Battery"),
            Self::PoE => write!(f, "PoE"),
        }
    }
}

/// Battery remaining display unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BatteryRemainDisplayUnit {
    /// Display as minutes remaining
    Minute = 0x01,
    /// Display as percentage
    Percent = 0x02,
    /// Display as voltage
    Voltage = 0x03,
}

impl ToCrsdk<u64> for BatteryRemainDisplayUnit {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for BatteryRemainDisplayUnit {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Minute,
            0x02 => Self::Percent,
            0x03 => Self::Voltage,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for BatteryRemainDisplayUnit {}

impl fmt::Display for BatteryRemainDisplayUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minute => write!(f, "Minutes"),
            Self::Percent => write!(f, "%"),
            Self::Voltage => write!(f, "Voltage"),
        }
    }
}

/// Focus operation direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum FocusOperation {
    /// Focus toward wide/near
    Wide = -1,
    /// Stop focus movement
    Stop = 0,
    /// Focus toward tele/far
    Tele = 1,
}

impl ToCrsdk<u64> for FocusOperation {
    fn to_crsdk(&self) -> u64 {
        (*self as i8) as u64
    }
}

impl FromCrsdk<u64> for FocusOperation {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as i8 {
            -1 => Self::Wide,
            0 => Self::Stop,
            1 => Self::Tele,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for FocusOperation {}

impl fmt::Display for FocusOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wide => write!(f, "Near"),
            Self::Stop => write!(f, "Stop"),
            Self::Tele => write!(f, "Far"),
        }
    }
}

/// Shutter type setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ShutterType {
    /// Auto shutter selection
    Auto = 0x01,
    /// Mechanical shutter
    Mechanical = 0x02,
    /// Electronic shutter
    Electronic = 0x03,
}

impl ToCrsdk<u64> for ShutterType {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for ShutterType {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Auto,
            0x02 => Self::Mechanical,
            0x03 => Self::Electronic,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for ShutterType {}

impl fmt::Display for ShutterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::Mechanical => write!(f, "Mechanical"),
            Self::Electronic => write!(f, "Electronic"),
        }
    }
}

/// Device overheating state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeviceOverheatingState {
    /// Normal temperature
    NotOverheating = 0x00,
    /// Camera is warming up
    PreOverheating = 0x01,
    /// Camera is overheated
    Overheating = 0x02,
}

impl ToCrsdk<u64> for DeviceOverheatingState {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for DeviceOverheatingState {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x00 => Self::NotOverheating,
            0x01 => Self::PreOverheating,
            0x02 => Self::Overheating,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for DeviceOverheatingState {}

impl fmt::Display for DeviceOverheatingState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotOverheating => write!(f, "Normal"),
            Self::PreOverheating => write!(f, "Warning"),
            Self::Overheating => write!(f, "Overheated"),
        }
    }
}

/// High ISO noise reduction setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HighIsoNR {
    /// Noise reduction off
    Off = 0x01,
    /// Low noise reduction
    Low = 0x02,
    /// Normal noise reduction
    Normal = 0x03,
    /// High noise reduction
    High = 0x04,
}

impl ToCrsdk<u64> for HighIsoNR {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for HighIsoNR {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Off,
            0x02 => Self::Low,
            0x03 => Self::Normal,
            0x04 => Self::High,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for HighIsoNR {}

impl fmt::Display for HighIsoNR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::Low => write!(f, "Low"),
            Self::Normal => write!(f, "Normal"),
            Self::High => write!(f, "High"),
        }
    }
}

/// Audio signals (beep) setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AudioSignals {
    /// Audio signals off
    Off = 0x01,
    /// Audio signals on
    On = 0x02,
    /// Only shutter sound
    OnShutterOnly = 0x03,
    /// All sounds except shutter
    OnWithoutShutter = 0x04,
}

impl ToCrsdk<u64> for AudioSignals {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for AudioSignals {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Off,
            0x02 => Self::On,
            0x03 => Self::OnShutterOnly,
            0x04 => Self::OnWithoutShutter,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for AudioSignals {}

impl fmt::Display for AudioSignals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::On => write!(f, "On"),
            Self::OnShutterOnly => write!(f, "Shutter Only"),
            Self::OnWithoutShutter => write!(f, "No Shutter"),
        }
    }
}

/// Touch operation function setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FunctionOfTouchOperation {
    /// Touch function off
    Off = 0x01,
    /// Touch to trigger shutter
    Shutter = 0x02,
    /// Touch to focus
    Focus = 0x03,
    /// Touch to track subject
    Tracking = 0x04,
    /// Touch for auto exposure
    AE = 0x05,
    /// Shutter + AE enabled
    ShutterAndAEOn = 0x06,
    /// Shutter only, AE disabled
    ShutterAndAEOff = 0x07,
    /// Focus + AE enabled
    FocusAndAEOn = 0x08,
    /// Focus only, AE disabled
    FocusAndAEOff = 0x09,
    /// Tracking + AE enabled
    TrackingAndAEOn = 0x0A,
    /// Tracking only, AE disabled
    TrackingAndAEOff = 0x0B,
}

impl ToCrsdk<u64> for FunctionOfTouchOperation {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for FunctionOfTouchOperation {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Off,
            0x02 => Self::Shutter,
            0x03 => Self::Focus,
            0x04 => Self::Tracking,
            0x05 => Self::AE,
            0x06 => Self::ShutterAndAEOn,
            0x07 => Self::ShutterAndAEOff,
            0x08 => Self::FocusAndAEOn,
            0x09 => Self::FocusAndAEOff,
            0x0A => Self::TrackingAndAEOn,
            0x0B => Self::TrackingAndAEOff,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for FunctionOfTouchOperation {}

impl fmt::Display for FunctionOfTouchOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::Shutter => write!(f, "Shutter"),
            Self::Focus => write!(f, "Focus"),
            Self::Tracking => write!(f, "Tracking"),
            Self::AE => write!(f, "AE"),
            Self::ShutterAndAEOn => write!(f, "Shutter+AE"),
            Self::ShutterAndAEOff => write!(f, "Shutter Only"),
            Self::FocusAndAEOn => write!(f, "Focus+AE"),
            Self::FocusAndAEOff => write!(f, "Focus Only"),
            Self::TrackingAndAEOn => write!(f, "Track+AE"),
            Self::TrackingAndAEOff => write!(f, "Track Only"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_display() {
        assert_eq!(Integer(42).to_string(), "42");
        assert_eq!(Integer(-100).to_string(), "-100");
        assert_eq!(Integer(0).to_string(), "0");
    }

    #[test]
    fn test_integer_signed_from_raw() {
        let raw = (-50i64) as u64;
        let int = Integer::from_raw(raw).unwrap();
        assert_eq!(int.value(), -50);
    }

    #[test]
    fn test_percentage_display() {
        assert_eq!(Percentage(100).to_string(), "100%");
        assert_eq!(Percentage(50).to_string(), "50%");
        assert_eq!(Percentage(0).to_string(), "0%");
    }

    #[test]
    fn test_switch_round_trip() {
        assert_eq!(Switch::from_raw(1).unwrap(), Switch::Off);
        assert_eq!(Switch::from_raw(2).unwrap(), Switch::On);
        assert_eq!(Switch::Off.to_raw(), 1);
        assert_eq!(Switch::On.to_raw(), 2);
    }

    #[test]
    fn test_onoff_round_trip() {
        assert_eq!(OnOff::from_raw(0).unwrap(), OnOff::Off);
        assert_eq!(OnOff::from_raw(1).unwrap(), OnOff::On);
        assert_eq!(OnOff::Off.to_raw(), 0);
        assert_eq!(OnOff::On.to_raw(), 1);
    }

    #[test]
    fn test_auto_manual_display() {
        assert_eq!(AutoManual::Automatic.to_string(), "Auto");
        assert_eq!(AutoManual::Manual.to_string(), "Manual");
    }

    #[test]
    fn test_lock_indicator_display() {
        assert_eq!(LockIndicator::Locked.to_string(), "Locked");
        assert_eq!(LockIndicator::Unlocked.to_string(), "Unlocked");
        assert_eq!(LockIndicator::Unknown.to_string(), "Unknown");
    }
}
