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
    /// Focus frame state
    FocusFrameState,
    /// Tracking frame type
    TrackingFrameType,
    /// White balance preset
    WhiteBalance,
    /// AWB priority setting
    PrioritySetInAWB,
    /// White balance switch (preset/memory)
    WhiteBalanceSwitch,
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
    /// Movie shooting mode (Off, Cine EI, Custom, etc.)
    MovieShootingMode,
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
    /// Auto power-off temperature threshold
    AutoPowerOffTemperature,
    /// Camera power status
    CameraPowerStatus,
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
    /// Audio stream bit depth
    AudioStreamBitDepth,
    /// Audio stream channel count
    AudioStreamChannel,
    /// Audio input channel source selection
    AudioInputCHInputSelect,
    /// Touch operation function
    FunctionOfTouchOperation,
    /// Soft skin effect level
    SoftSkinEffect,
    /// Wind noise reduction setting
    WindNoiseReduction,
    /// Finder/Monitor selection
    SelectFinder,
    /// Display mode
    DispMode,
    /// Image stabilization for movie
    ImageStabilizationSteadyShotMovie,
    /// Lens shading compensation
    LensCompensationShading,
    /// Custom white balance capture size
    CustomWBSizeSetting,
    /// Aperture drive behavior in AF
    ApertureDriveInAF,
    /// Recording media selection (still)
    RecordingMedia,
    /// Recording media selection (movie)
    RecordingMediaMovie,
    /// E-framing production effect
    EframingProductionEffect,
    /// AF tracking responsiveness
    AFTrackForSpeedChange,
    /// AF tracking sensitivity
    AFTrackingSensitivity,
    /// Timecode/userbit display setting
    TCUBDisplaySetting,
    /// Subject recognition animal/bird priority
    SubjectRecognitionAnimalBirdPriority,
    /// Subject recognition detection parts
    SubjectRecognitionAnimalBirdDetectionParts,
    /// Camera operating mode
    CameraOperatingMode,
    /// Iris display unit
    IrisDisplayUnit,
    /// Image stabilization level for movie
    ImageStabilizationLevelMovie,
    /// Shutter release time lag control
    ShutterReleaseTimeLagControl,
    /// TimeShift trigger setting
    TimeShiftTriggerSetting,
    /// APS-C/S35 crop mode
    APSC_S35,
    /// Shutter mode status
    ShutterModeStatus,
    /// Shutter mode
    ShutterMode,
    /// Exposure control type
    ExposureCtrlType,
    /// Gain unit setting (dB vs ISO)
    GainUnitSetting,
    /// Live view display effect
    LiveViewDisplayEffect,
    /// Live view protocol
    LiveViewProtocol,
    /// Silent mode aperture drive
    SilentModeApertureDrive,
    /// Right/Left eye select for AF
    RightLeftEyeSelect,
    /// Gain base sensitivity
    GainBaseSensitivity,
    /// FTP connection status
    FTPConnectionStatus,
    /// Zoom operation direction
    ZoomOperation,
    /// RAW file compression type
    RAWFileCompressionType,
    /// Compression file format
    CompressionFileFormat,
    /// Zoom speed type
    RemoconZoomSpeedType,
    /// Zoom type status (optical/smart/clear image/digital)
    ZoomTypeStatus,
    /// Zoom motor driving status
    ZoomDrivingStatus,
    /// ND filter mode
    NDFilterMode,
    /// Creative Look style
    CreativeLook,
    /// ISO auto minimum shutter speed preset
    IsoAutoMinShutterSpeedPreset,
    /// ISO auto minimum shutter speed mode
    IsoAutoMinShutterSpeedMode,
    /// Picture effect filter
    PictureEffect,
    /// Picture profile gamma curve
    PictureProfileGamma,
    /// Picture profile color mode
    PictureProfileColorMode,
    /// Picture profile selection (PP1-PP11)
    PictureProfile,
    /// Picture profile black gamma range
    PictureProfileBlackGammaRange,
    /// Grid line overlay type
    GridLineType,
    /// Streaming status
    StreamStatus,
    /// Stream encryption cipher type
    StreamCipherType,
    /// Imager/sensor scan mode
    ImagerScanMode,
    /// Auto-framing/E-framing type
    EframingType,
    /// Video stream codec
    VideoStreamCodec,
    /// Monitoring output format
    MonitoringOutputFormat,
    /// Face frame type
    FaceFrameType,
    /// Focus frame type
    FocusFrameType,
    /// Frame info type
    FrameInfoType,

    // Focus status types
    /// Focus motor driving status
    FocusDrivingStatus,
    /// Focus bracket shooting status
    FocusBracketShootingStatus,
    /// Focus touch spot status
    FocusTouchSpotStatus,
    /// Focus bracket order
    FocusBracketOrder,
    /// Push auto focus action
    PushAutoFocus,
    /// Focus operation 16-bit enable status
    FocusOperationWithInt16EnableStatus,

    // Movie format types
    /// Movie shooting mode color gamut
    MovieShootingModeColorGamut,
    /// Movie shooting mode target display
    MovieShootingModeTargetDisplay,
    /// Recorder save destination
    RecorderSaveDestination,
    /// Video recording format quality
    VideoRecordingFormatQuality,
    /// Recording folder format
    RecordingFolderFormat,
    /// Movie playing state
    MoviePlayingState,
    /// Movie recording review playing state
    MovieRecReviewPlayingState,
    /// Playback contents gamma type
    PlaybackContentsGammaType,

    // Picture profile control types
    /// Picture profile detail adjustment mode
    PictureProfileDetailAdjustMode,
    /// Picture profile knee mode
    PictureProfileKneeMode,
    /// Picture profile knee auto-set sensitivity
    PictureProfileKneeAutoSetSensitivity,
    /// Picture profile reset enable status
    PictureProfileResetEnableStatus,
    /// Creative look reset enable status
    CreativeLookResetEnableStatus,
    /// Timecode preset reset enable status
    TimeCodePresetResetEnableStatus,

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

/// Live view protocol setting (None=0, Main=1, Alt=2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum LiveViewProtocol {
    /// No protocol
    None = 0,
    /// Main protocol
    Main = 1,
    /// Alternate protocol
    Alt = 2,
}

impl ToCrsdk<u64> for LiveViewProtocol {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for LiveViewProtocol {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u32 {
            0 => Self::None,
            1 => Self::Main,
            2 => Self::Alt,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for LiveViewProtocol {}

impl fmt::Display for LiveViewProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Main => write!(f, "Main"),
            Self::Alt => write!(f, "Alt"),
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

/// Auto power-off temperature threshold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AutoPowerOffTemperature {
    /// Standard temperature threshold
    Standard = 0x01,
    /// High temperature threshold
    High = 0x02,
}

impl ToCrsdk<u64> for AutoPowerOffTemperature {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for AutoPowerOffTemperature {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Standard,
            0x02 => Self::High,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for AutoPowerOffTemperature {}

impl fmt::Display for AutoPowerOffTemperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "Standard"),
            Self::High => write!(f, "High"),
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

/// Audio stream bit depth setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AudioStreamBitDepth {
    /// 16-bit audio
    Bit16 = 0x01,
    /// 24-bit audio
    Bit24 = 0x02,
    /// 32-bit audio
    Bit32 = 0x03,
}

impl ToCrsdk<u64> for AudioStreamBitDepth {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for AudioStreamBitDepth {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Bit16,
            0x02 => Self::Bit24,
            0x03 => Self::Bit32,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for AudioStreamBitDepth {}

impl fmt::Display for AudioStreamBitDepth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bit16 => write!(f, "16-bit"),
            Self::Bit24 => write!(f, "24-bit"),
            Self::Bit32 => write!(f, "32-bit"),
        }
    }
}

/// Audio stream channel count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AudioStreamChannel {
    /// 1 channel (mono)
    Ch1 = 0x01,
    /// 2 channels (stereo)
    Ch2 = 0x02,
    /// 4 channels
    Ch4 = 0x04,
}

impl ToCrsdk<u64> for AudioStreamChannel {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for AudioStreamChannel {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Ch1,
            0x02 => Self::Ch2,
            0x04 => Self::Ch4,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for AudioStreamChannel {}

impl fmt::Display for AudioStreamChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ch1 => write!(f, "1 Channel"),
            Self::Ch2 => write!(f, "2 Channels"),
            Self::Ch4 => write!(f, "4 Channels"),
        }
    }
}

/// Audio input channel source selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AudioInputCHInputSelect {
    /// Input off
    Off = 0x01,
    /// Input 1
    INPUT1 = 0x02,
    /// Input 2
    INPUT2 = 0x03,
    /// Internal microphone
    InternalMIC = 0x04,
    /// Shoe channel 1
    ShoeCH1 = 0x05,
    /// Shoe channel 2
    ShoeCH2 = 0x06,
    /// Stereo microphone jack left
    StereoMicJackL = 0x07,
    /// Stereo microphone jack right
    StereoMicJackR = 0x08,
}

impl ToCrsdk<u64> for AudioInputCHInputSelect {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for AudioInputCHInputSelect {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Off,
            0x02 => Self::INPUT1,
            0x03 => Self::INPUT2,
            0x04 => Self::InternalMIC,
            0x05 => Self::ShoeCH1,
            0x06 => Self::ShoeCH2,
            0x07 => Self::StereoMicJackL,
            0x08 => Self::StereoMicJackR,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for AudioInputCHInputSelect {}

impl fmt::Display for AudioInputCHInputSelect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::INPUT1 => write!(f, "Input 1"),
            Self::INPUT2 => write!(f, "Input 2"),
            Self::InternalMIC => write!(f, "Internal Mic"),
            Self::ShoeCH1 => write!(f, "Shoe CH1"),
            Self::ShoeCH2 => write!(f, "Shoe CH2"),
            Self::StereoMicJackL => write!(f, "Stereo Mic Jack L"),
            Self::StereoMicJackR => write!(f, "Stereo Mic Jack R"),
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

/// Soft skin effect level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SoftSkinEffect {
    /// Effect off
    Off = 0x01,
    /// Low effect
    Low = 0x02,
    /// Medium effect
    Mid = 0x03,
    /// High effect
    High = 0x04,
}

impl ToCrsdk<u64> for SoftSkinEffect {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for SoftSkinEffect {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Off,
            0x02 => Self::Low,
            0x03 => Self::Mid,
            0x04 => Self::High,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for SoftSkinEffect {}

impl fmt::Display for SoftSkinEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::Low => write!(f, "Low"),
            Self::Mid => write!(f, "Mid"),
            Self::High => write!(f, "High"),
        }
    }
}

/// Wind noise reduction setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WindNoiseReduction {
    /// Wind noise reduction off
    Off = 0x01,
    /// Wind noise reduction on
    On = 0x02,
    /// Auto wind noise reduction
    Auto = 0x03,
}

impl ToCrsdk<u64> for WindNoiseReduction {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for WindNoiseReduction {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Off,
            0x02 => Self::On,
            0x03 => Self::Auto,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for WindNoiseReduction {}

impl fmt::Display for WindNoiseReduction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::On => write!(f, "On"),
            Self::Auto => write!(f, "Auto"),
        }
    }
}

/// Finder/Monitor selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SelectFinder {
    /// Auto select between finder and monitor
    Auto = 0x01,
    /// Force viewfinder (manual)
    ViewFinder = 0x02,
    /// Force monitor (manual)
    Monitor = 0x03,
    /// Auto select variant 2
    Auto2 = 0x04,
}

impl ToCrsdk<u64> for SelectFinder {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for SelectFinder {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Auto,
            0x02 => Self::ViewFinder,
            0x03 => Self::Monitor,
            0x04 => Self::Auto2,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for SelectFinder {}

impl fmt::Display for SelectFinder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::ViewFinder => write!(f, "Viewfinder"),
            Self::Monitor => write!(f, "Monitor"),
            Self::Auto2 => write!(f, "Auto 2"),
        }
    }
}

/// Display mode setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DispMode {
    /// Graphic display
    GraphicDisplay = 0x01,
    /// Display all info
    DisplayAllInfo = 0x02,
    /// No display info
    NoDispInfo = 0x03,
    /// Histogram display
    Histogram = 0x04,
    /// Level display
    Level = 0x05,
    /// For viewfinder
    ForViewFinder = 0x06,
    /// Monitor off
    MonitorOff = 0x07,
}

impl ToCrsdk<u64> for DispMode {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for DispMode {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::GraphicDisplay,
            0x02 => Self::DisplayAllInfo,
            0x03 => Self::NoDispInfo,
            0x04 => Self::Histogram,
            0x05 => Self::Level,
            0x06 => Self::ForViewFinder,
            0x07 => Self::MonitorOff,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for DispMode {}

impl fmt::Display for DispMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GraphicDisplay => write!(f, "Graphic"),
            Self::DisplayAllInfo => write!(f, "All Info"),
            Self::NoDispInfo => write!(f, "No Info"),
            Self::Histogram => write!(f, "Histogram"),
            Self::Level => write!(f, "Level"),
            Self::ForViewFinder => write!(f, "Viewfinder"),
            Self::MonitorOff => write!(f, "Off"),
        }
    }
}

/// Image stabilization mode for movie recording.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ImageStabilizationSteadyShotMovie {
    /// Stabilization off
    Off = 0x01,
    /// Standard stabilization
    Standard = 0x02,
    /// Active stabilization (stronger)
    Active = 0x03,
    /// Dynamic active (strongest, crops more)
    DynamicActive = 0x04,
}

impl ToCrsdk<u64> for ImageStabilizationSteadyShotMovie {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for ImageStabilizationSteadyShotMovie {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Off,
            0x02 => Self::Standard,
            0x03 => Self::Active,
            0x04 => Self::DynamicActive,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for ImageStabilizationSteadyShotMovie {}

impl fmt::Display for ImageStabilizationSteadyShotMovie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::Standard => write!(f, "Standard"),
            Self::Active => write!(f, "Active"),
            Self::DynamicActive => write!(f, "Dynamic"),
        }
    }
}

/// Lens shading compensation setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LensCompensationShading {
    /// Compensation off
    Off = 0x01,
    /// Auto compensation
    Auto = 0x02,
    /// Low compensation
    Low = 0x03,
}

impl ToCrsdk<u64> for LensCompensationShading {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for LensCompensationShading {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Off,
            0x02 => Self::Auto,
            0x03 => Self::Low,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for LensCompensationShading {}

impl fmt::Display for LensCompensationShading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::Auto => write!(f, "Auto"),
            Self::Low => write!(f, "Low"),
        }
    }
}

/// Custom white balance capture size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CustomWBSizeSetting {
    /// Small capture area
    Small = 0x01,
    /// Medium capture area
    Medium = 0x02,
    /// Large capture area
    Large = 0x03,
}

impl ToCrsdk<u64> for CustomWBSizeSetting {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for CustomWBSizeSetting {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Small,
            0x02 => Self::Medium,
            0x03 => Self::Large,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for CustomWBSizeSetting {}

impl fmt::Display for CustomWBSizeSetting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Small => write!(f, "S"),
            Self::Medium => write!(f, "M"),
            Self::Large => write!(f, "L"),
        }
    }
}

/// Aperture drive behavior during AF.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ApertureDriveInAF {
    /// Standard aperture drive
    Standard = 0x01,
    /// Prioritize focus speed
    FocusPriority = 0x02,
    /// Prioritize silent operation
    SilentPriority = 0x03,
}

impl ToCrsdk<u64> for ApertureDriveInAF {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for ApertureDriveInAF {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Standard,
            0x02 => Self::FocusPriority,
            0x03 => Self::SilentPriority,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for ApertureDriveInAF {}

impl fmt::Display for ApertureDriveInAF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "Standard"),
            Self::FocusPriority => write!(f, "Focus"),
            Self::SilentPriority => write!(f, "Silent"),
        }
    }
}

/// Recording media selection for still images.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum RecordingMedia {
    /// Record to slot 1
    Slot1 = 0x0001,
    /// Record to slot 2
    Slot2 = 0x0002,
    /// Record to both slots simultaneously
    Simultaneous = 0x0101,
    /// Sort recording between slots
    Sort = 0x0102,
}

impl ToCrsdk<u64> for RecordingMedia {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for RecordingMedia {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0001 => Self::Slot1,
            0x0002 => Self::Slot2,
            0x0101 => Self::Simultaneous,
            0x0102 => Self::Sort,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for RecordingMedia {}

impl fmt::Display for RecordingMedia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Slot1 => write!(f, "Slot 1"),
            Self::Slot2 => write!(f, "Slot 2"),
            Self::Simultaneous => write!(f, "Both"),
            Self::Sort => write!(f, "Sort"),
        }
    }
}

/// Recording media selection for movies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum RecordingMediaMovie {
    /// Record to slot 1
    Slot1 = 0x0001,
    /// Record to slot 2
    Slot2 = 0x0002,
    /// Record to both slots simultaneously
    Simultaneous = 0x0101,
}

impl ToCrsdk<u64> for RecordingMediaMovie {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for RecordingMediaMovie {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0001 => Self::Slot1,
            0x0002 => Self::Slot2,
            0x0101 => Self::Simultaneous,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for RecordingMediaMovie {}

impl fmt::Display for RecordingMediaMovie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Slot1 => write!(f, "Slot 1"),
            Self::Slot2 => write!(f, "Slot 2"),
            Self::Simultaneous => write!(f, "Both"),
        }
    }
}

/// E-framing production effect setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum EframingProductionEffect {
    /// Effect disabled
    Off = 0x0001,
    /// Interval zoom in/out effect with 15 second duration
    IntervalZoomInOut15sec = 0x0002,
    /// Interval zoom in/out effect with 30 second duration
    IntervalZoomInOut30sec = 0x0003,
}

impl ToCrsdk<u64> for EframingProductionEffect {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for EframingProductionEffect {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0001 => Self::Off,
            0x0002 => Self::IntervalZoomInOut15sec,
            0x0003 => Self::IntervalZoomInOut30sec,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for EframingProductionEffect {}

impl fmt::Display for EframingProductionEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::IntervalZoomInOut15sec => write!(f, "Interval Zoom 15s"),
            Self::IntervalZoomInOut30sec => write!(f, "Interval Zoom 30s"),
        }
    }
}

/// AF tracking responsiveness setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AFTrackForSpeedChange {
    /// Stable tracking (less responsive to speed changes)
    Stable = 0x01,
    /// Standard tracking
    Standard = 0x02,
    /// Responsive tracking (quick to adapt)
    Responsive = 0x03,
}

impl ToCrsdk<u64> for AFTrackForSpeedChange {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for AFTrackForSpeedChange {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Stable,
            0x02 => Self::Standard,
            0x03 => Self::Responsive,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for AFTrackForSpeedChange {}

impl fmt::Display for AFTrackForSpeedChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stable => write!(f, "Stable"),
            Self::Standard => write!(f, "Standard"),
            Self::Responsive => write!(f, "Responsive"),
        }
    }
}

/// AF tracking sensitivity setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AFTrackingSensitivity {
    /// Level 1 (Locked On) - most stable, least responsive
    Level1 = 0x01,
    /// Level 2
    Level2 = 0x02,
    /// Level 3 (Standard)
    Level3 = 0x03,
    /// Level 4
    Level4 = 0x04,
    /// Level 5 (Responsive) - least stable, most responsive
    Level5 = 0x05,
}

impl ToCrsdk<u64> for AFTrackingSensitivity {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for AFTrackingSensitivity {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Level1,
            0x02 => Self::Level2,
            0x03 => Self::Level3,
            0x04 => Self::Level4,
            0x05 => Self::Level5,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for AFTrackingSensitivity {}

impl fmt::Display for AFTrackingSensitivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Level1 => write!(f, "Level 1 (Locked On)"),
            Self::Level2 => write!(f, "Level 2"),
            Self::Level3 => write!(f, "Level 3 (Standard)"),
            Self::Level4 => write!(f, "Level 4"),
            Self::Level5 => write!(f, "Level 5 (Responsive)"),
        }
    }
}

/// Timecode/Userbit display setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TCUBDisplaySetting {
    /// Show counter
    Counter = 0x01,
    /// Show timecode
    TimeCode = 0x02,
    /// Show userbit
    UserBit = 0x03,
    /// Show duration
    Duration = 0x04,
}

impl ToCrsdk<u64> for TCUBDisplaySetting {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for TCUBDisplaySetting {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Counter,
            0x02 => Self::TimeCode,
            0x03 => Self::UserBit,
            0x04 => Self::Duration,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for TCUBDisplaySetting {}

impl fmt::Display for TCUBDisplaySetting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Counter => write!(f, "Counter"),
            Self::TimeCode => write!(f, "TC"),
            Self::UserBit => write!(f, "UB"),
            Self::Duration => write!(f, "Duration"),
        }
    }
}

/// Subject recognition animal/bird priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SubjectRecognitionAnimalBirdPriority {
    /// Auto priority selection
    Auto = 0x01,
    /// Prioritize animals
    AnimalPriority = 0x02,
    /// Prioritize birds
    BirdPriority = 0x03,
}

impl ToCrsdk<u64> for SubjectRecognitionAnimalBirdPriority {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for SubjectRecognitionAnimalBirdPriority {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Auto,
            0x02 => Self::AnimalPriority,
            0x03 => Self::BirdPriority,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for SubjectRecognitionAnimalBirdPriority {}

impl fmt::Display for SubjectRecognitionAnimalBirdPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::AnimalPriority => write!(f, "Animal"),
            Self::BirdPriority => write!(f, "Bird"),
        }
    }
}

/// Subject recognition detection parts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SubjectRecognitionAnimalBirdDetectionParts {
    /// Follow individual setting
    FollowIndividual = 0xFF,
    /// Detect eye, head, and body
    EyeHeadBody = 0x01,
    /// Detect eye and head only
    EyeHead = 0x02,
    /// Detect eye only
    Eye = 0x03,
}

impl ToCrsdk<u64> for SubjectRecognitionAnimalBirdDetectionParts {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for SubjectRecognitionAnimalBirdDetectionParts {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0xFF => Self::FollowIndividual,
            0x01 => Self::EyeHeadBody,
            0x02 => Self::EyeHead,
            0x03 => Self::Eye,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for SubjectRecognitionAnimalBirdDetectionParts {}

impl fmt::Display for SubjectRecognitionAnimalBirdDetectionParts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FollowIndividual => write!(f, "Individual"),
            Self::EyeHeadBody => write!(f, "Eye+Head+Body"),
            Self::EyeHead => write!(f, "Eye+Head"),
            Self::Eye => write!(f, "Eye Only"),
        }
    }
}

/// Camera operating mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CameraOperatingMode {
    /// Recording mode
    Record = 0x01,
    /// Playback mode
    Playback = 0x02,
}

impl ToCrsdk<u64> for CameraOperatingMode {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for CameraOperatingMode {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Record,
            0x02 => Self::Playback,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for CameraOperatingMode {}

impl fmt::Display for CameraOperatingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Record => write!(f, "Record"),
            Self::Playback => write!(f, "Playback"),
        }
    }
}

/// Iris display unit setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IrisDisplayUnit {
    /// Auto display unit
    Auto = 0x01,
    /// F-number lock
    FLock = 0x02,
    /// T-number lock
    TLock = 0x03,
}

impl ToCrsdk<u64> for IrisDisplayUnit {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for IrisDisplayUnit {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Auto,
            0x02 => Self::FLock,
            0x03 => Self::TLock,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for IrisDisplayUnit {}

impl fmt::Display for IrisDisplayUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::FLock => write!(f, "F-Lock"),
            Self::TLock => write!(f, "T-Lock"),
        }
    }
}

/// Image stabilization level for movie recording.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ImageStabilizationLevelMovie {
    /// Stabilization off
    Off = 0x01,
    /// Low stabilization
    Low = 0x02,
    /// High stabilization
    High = 0x03,
}

impl ToCrsdk<u64> for ImageStabilizationLevelMovie {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for ImageStabilizationLevelMovie {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Off,
            0x02 => Self::Low,
            0x03 => Self::High,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for ImageStabilizationLevelMovie {}

impl fmt::Display for ImageStabilizationLevelMovie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::Low => write!(f, "Low"),
            Self::High => write!(f, "High"),
        }
    }
}

/// Shutter release time lag control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ShutterReleaseTimeLagControl {
    /// Lag control off
    Off = 0x01,
    /// On with stability priority
    OnStability = 0x02,
    /// On with fastest response
    OnFastest = 0x03,
}

impl ToCrsdk<u64> for ShutterReleaseTimeLagControl {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for ShutterReleaseTimeLagControl {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Off,
            0x02 => Self::OnStability,
            0x03 => Self::OnFastest,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for ShutterReleaseTimeLagControl {}

impl fmt::Display for ShutterReleaseTimeLagControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::OnStability => write!(f, "Stability"),
            Self::OnFastest => write!(f, "Fastest"),
        }
    }
}

/// TimeShift trigger setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TimeShiftTriggerSetting {
    /// S1 and AF trigger
    S1AndAF = 0x01,
    /// S1 only trigger
    S1 = 0x02,
    /// AF only trigger
    AF = 0x03,
}

impl ToCrsdk<u64> for TimeShiftTriggerSetting {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for TimeShiftTriggerSetting {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::S1AndAF,
            0x02 => Self::S1,
            0x03 => Self::AF,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for TimeShiftTriggerSetting {}

impl fmt::Display for TimeShiftTriggerSetting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::S1AndAF => write!(f, "S1+AF"),
            Self::S1 => write!(f, "S1"),
            Self::AF => write!(f, "AF"),
        }
    }
}

/// APS-C/Super35 crop mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum APSC_S35 {
    /// Crop off (full frame)
    Off = 0x01,
    /// Crop on
    On = 0x02,
    /// Auto crop
    Auto = 0x03,
}

impl ToCrsdk<u64> for APSC_S35 {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for APSC_S35 {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Off,
            0x02 => Self::On,
            0x03 => Self::Auto,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for APSC_S35 {}

impl fmt::Display for APSC_S35 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::On => write!(f, "On"),
            Self::Auto => write!(f, "Auto"),
        }
    }
}

/// Right/Left eye select for AF.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RightLeftEyeSelect {
    /// Auto eye selection
    Auto = 0x01,
    /// Focus on right eye
    RightEye = 0x02,
    /// Focus on left eye
    LeftEye = 0x03,
}

impl ToCrsdk<u64> for RightLeftEyeSelect {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for RightLeftEyeSelect {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Auto,
            0x02 => Self::RightEye,
            0x03 => Self::LeftEye,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for RightLeftEyeSelect {}

impl fmt::Display for RightLeftEyeSelect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::RightEye => write!(f, "Right"),
            Self::LeftEye => write!(f, "Left"),
        }
    }
}

/// Gain base sensitivity setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GainBaseSensitivity {
    /// High sensitivity mode
    High = 0x01,
    /// Low sensitivity mode
    Low = 0x02,
}

impl ToCrsdk<u64> for GainBaseSensitivity {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for GainBaseSensitivity {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::High,
            0x02 => Self::Low,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for GainBaseSensitivity {}

impl fmt::Display for GainBaseSensitivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::High => write!(f, "High"),
            Self::Low => write!(f, "Low"),
        }
    }
}

/// FTP connection status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FTPConnectionStatus {
    /// Currently connecting
    Connecting = 0x01,
    /// Successfully connected
    Connected = 0x02,
    /// Connected but server error
    ConnectedServerError = 0x03,
    /// Connection error
    ConnectionError = 0x04,
}

impl ToCrsdk<u64> for FTPConnectionStatus {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for FTPConnectionStatus {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Connecting,
            0x02 => Self::Connected,
            0x03 => Self::ConnectedServerError,
            0x04 => Self::ConnectionError,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for FTPConnectionStatus {}

impl fmt::Display for FTPConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connecting => write!(f, "Connecting"),
            Self::Connected => write!(f, "Connected"),
            Self::ConnectedServerError => write!(f, "Server Error"),
            Self::ConnectionError => write!(f, "Error"),
        }
    }
}

/// Zoom operation direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum ZoomOperation {
    /// Zoom toward wide angle
    Wide = -1,
    /// Stop zoom movement
    Stop = 0,
    /// Zoom toward telephoto
    Tele = 1,
}

impl ToCrsdk<u64> for ZoomOperation {
    fn to_crsdk(&self) -> u64 {
        (*self as i8) as u64
    }
}

impl FromCrsdk<u64> for ZoomOperation {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as i8 {
            -1 => Self::Wide,
            0 => Self::Stop,
            1 => Self::Tele,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for ZoomOperation {}

impl fmt::Display for ZoomOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wide => write!(f, "Wide"),
            Self::Stop => write!(f, "Stop"),
            Self::Tele => write!(f, "Tele"),
        }
    }
}

/// RAW file compression type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum RAWFileCompressionType {
    /// Uncompressed RAW
    Uncompressed = 0x0000,
    /// Compressed RAW
    Compressed = 0x0001,
    /// Lossless compression
    Lossless = 0x0002,
    /// Lossless compression (Small)
    LosslessS = 0x0003,
    /// Lossless compression (Medium)
    LosslessM = 0x0004,
    /// Lossless compression (Large)
    LosslessL = 0x0005,
}

impl ToCrsdk<u64> for RAWFileCompressionType {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for RAWFileCompressionType {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0000 => Self::Uncompressed,
            0x0001 => Self::Compressed,
            0x0002 => Self::Lossless,
            0x0003 => Self::LosslessS,
            0x0004 => Self::LosslessM,
            0x0005 => Self::LosslessL,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for RAWFileCompressionType {}

impl fmt::Display for RAWFileCompressionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uncompressed => write!(f, "Uncompressed"),
            Self::Compressed => write!(f, "Compressed"),
            Self::Lossless => write!(f, "Lossless"),
            Self::LosslessS => write!(f, "Lossless S"),
            Self::LosslessM => write!(f, "Lossless M"),
            Self::LosslessL => write!(f, "Lossless L"),
        }
    }
}

/// Compression file format for still images.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CompressionFileFormat {
    /// JPEG compression
    Jpeg = 0x01,
    /// HEIF 4:2:2 compression
    Heif422 = 0x02,
    /// HEIF 4:2:0 compression
    Heif420 = 0x03,
}

impl ToCrsdk<u64> for CompressionFileFormat {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for CompressionFileFormat {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Jpeg,
            0x02 => Self::Heif422,
            0x03 => Self::Heif420,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for CompressionFileFormat {}

impl fmt::Display for CompressionFileFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Jpeg => write!(f, "JPEG"),
            Self::Heif422 => write!(f, "HEIF 4:2:2"),
            Self::Heif420 => write!(f, "HEIF 4:2:0"),
        }
    }
}

/// Remocon zoom speed type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RemoconZoomSpeedType {
    /// Invalid/not available
    Invalid = 0x00,
    /// Variable speed zoom
    Variable = 0x01,
    /// Fixed speed zoom
    Fixed = 0x02,
}

impl ToCrsdk<u64> for RemoconZoomSpeedType {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for RemoconZoomSpeedType {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x00 => Self::Invalid,
            0x01 => Self::Variable,
            0x02 => Self::Fixed,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for RemoconZoomSpeedType {}

impl fmt::Display for RemoconZoomSpeedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid => write!(f, "N/A"),
            Self::Variable => write!(f, "Variable"),
            Self::Fixed => write!(f, "Fixed"),
        }
    }
}

/// Zoom type status (current active zoom mode).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ZoomTypeStatus {
    /// Optical zoom
    Optical = 0x01,
    /// Smart zoom (digital zoom with minimal quality loss)
    Smart = 0x02,
    /// Clear Image Zoom (advanced digital zoom)
    ClearImage = 0x03,
    /// Digital zoom (standard digital zoom)
    Digital = 0x04,
}

impl ToCrsdk<u64> for ZoomTypeStatus {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for ZoomTypeStatus {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Optical,
            0x02 => Self::Smart,
            0x03 => Self::ClearImage,
            0x04 => Self::Digital,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for ZoomTypeStatus {}

impl fmt::Display for ZoomTypeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Optical => write!(f, "Optical Zoom"),
            Self::Smart => write!(f, "Smart Zoom"),
            Self::ClearImage => write!(f, "Clear Image Zoom"),
            Self::Digital => write!(f, "Digital Zoom"),
        }
    }
}

/// Zoom motor driving status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ZoomDrivingStatus {
    /// Zoom motor not driving
    NotDriving = 0x01,
    /// Zoom motor is driving
    Driving = 0x02,
}

impl ToCrsdk<u64> for ZoomDrivingStatus {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for ZoomDrivingStatus {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::NotDriving,
            0x02 => Self::Driving,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for ZoomDrivingStatus {}

impl fmt::Display for ZoomDrivingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotDriving => write!(f, "Stopped"),
            Self::Driving => write!(f, "Driving"),
        }
    }
}

/// ND filter mode setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NDFilterMode {
    /// Auto ND filter
    Auto = 0x01,
    /// Preset ND value
    Preset = 0x02,
    /// Preset with clear option
    PresetClear = 0x03,
    /// Variable ND
    Variable = 0x04,
    /// Variable with clear option
    VariableClear = 0x05,
    /// Step ND
    Step = 0x06,
    /// Step with clear option
    StepClear = 0x07,
}

impl ToCrsdk<u64> for NDFilterMode {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for NDFilterMode {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Auto,
            0x02 => Self::Preset,
            0x03 => Self::PresetClear,
            0x04 => Self::Variable,
            0x05 => Self::VariableClear,
            0x06 => Self::Step,
            0x07 => Self::StepClear,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for NDFilterMode {}

impl fmt::Display for NDFilterMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::Preset => write!(f, "Preset"),
            Self::PresetClear => write!(f, "Preset+Clear"),
            Self::Variable => write!(f, "Variable"),
            Self::VariableClear => write!(f, "Variable+Clear"),
            Self::Step => write!(f, "Step"),
            Self::StepClear => write!(f, "Step+Clear"),
        }
    }
}

/// Creative Look style preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum CreativeLook {
    /// Standard (ST)
    Standard = 0x0001,
    /// Portrait (PT)
    Portrait = 0x0002,
    /// Neutral (NT)
    Neutral = 0x0003,
    /// Vivid (VV)
    Vivid = 0x0004,
    /// Vivid 2 (VV2)
    Vivid2 = 0x0005,
    /// Film (FL)
    Film = 0x0006,
    /// Instant (IN)
    Instant = 0x0007,
    /// Soft Highkey (SH)
    SoftHighkey = 0x0008,
    /// Black & White (BW)
    BlackWhite = 0x0009,
    /// Sepia (SE)
    Sepia = 0x000A,
    /// Film 2
    Film2 = 0x000B,
    /// Film 3
    Film3 = 0x000C,
}

impl ToCrsdk<u64> for CreativeLook {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for CreativeLook {
    fn from_crsdk(raw: u64) -> Result<Self> {
        // Handle both raw values and custom look offset values
        let value = (raw & 0xFF) as u16;
        Ok(match value {
            0x01 => Self::Standard,
            0x02 => Self::Portrait,
            0x03 => Self::Neutral,
            0x04 => Self::Vivid,
            0x05 => Self::Vivid2,
            0x06 => Self::Film,
            0x07 => Self::Instant,
            0x08 => Self::SoftHighkey,
            0x09 => Self::BlackWhite,
            0x0A => Self::Sepia,
            0x0B => Self::Film2,
            0x0C => Self::Film3,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for CreativeLook {}

impl fmt::Display for CreativeLook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "ST"),
            Self::Portrait => write!(f, "PT"),
            Self::Neutral => write!(f, "NT"),
            Self::Vivid => write!(f, "VV"),
            Self::Vivid2 => write!(f, "VV2"),
            Self::Film => write!(f, "FL"),
            Self::Instant => write!(f, "IN"),
            Self::SoftHighkey => write!(f, "SH"),
            Self::BlackWhite => write!(f, "BW"),
            Self::Sepia => write!(f, "SE"),
            Self::Film2 => write!(f, "FL2"),
            Self::Film3 => write!(f, "FL3"),
        }
    }
}

/// ISO auto minimum shutter speed preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IsoAutoMinShutterSpeedPreset {
    /// Slower than standard
    Slower = 0x01,
    /// Slow
    Slow = 0x02,
    /// Standard speed
    Standard = 0x03,
    /// Fast
    Fast = 0x04,
    /// Faster than standard
    Faster = 0x05,
}

impl ToCrsdk<u64> for IsoAutoMinShutterSpeedPreset {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for IsoAutoMinShutterSpeedPreset {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Slower,
            0x02 => Self::Slow,
            0x03 => Self::Standard,
            0x04 => Self::Fast,
            0x05 => Self::Faster,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for IsoAutoMinShutterSpeedPreset {}

impl fmt::Display for IsoAutoMinShutterSpeedPreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Slower => write!(f, "Slower"),
            Self::Slow => write!(f, "Slow"),
            Self::Standard => write!(f, "Standard"),
            Self::Fast => write!(f, "Fast"),
            Self::Faster => write!(f, "Faster"),
        }
    }
}

/// ISO auto minimum shutter speed mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IsoAutoMinShutterSpeedMode {
    /// Use preset speed values
    Preset = 0x01,
    /// Manual speed setting
    Manual = 0x02,
}

impl ToCrsdk<u64> for IsoAutoMinShutterSpeedMode {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for IsoAutoMinShutterSpeedMode {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Preset,
            0x02 => Self::Manual,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for IsoAutoMinShutterSpeedMode {}

impl fmt::Display for IsoAutoMinShutterSpeedMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Preset => write!(f, "Preset"),
            Self::Manual => write!(f, "Manual"),
        }
    }
}

/// Picture effect creative filter setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PictureEffect {
    /// No picture effect
    Off = 0x00000000,
    /// Toy camera effect (normal)
    ToyCameraNormal = 0x00000001,
    /// Toy camera effect (cool tone)
    ToyCameraCool = 0x00000002,
    /// Toy camera effect (warm tone)
    ToyCameraWarm = 0x00000003,
    /// Toy camera effect (green tone)
    ToyCameraGreen = 0x00000004,
    /// Toy camera effect (magenta tone)
    ToyCameraMagenta = 0x00000005,
    /// Pop color effect
    Pop = 0x00000100,
    /// Posterization (black & white)
    PosterizationBW = 0x00000200,
    /// Posterization (color)
    PosterizationColor = 0x00000201,
    /// Retro photo effect
    Retro = 0x00000300,
    /// Soft high-key effect
    SoftHighkey = 0x00000400,
    /// Partial color (red only)
    PartColorRed = 0x00000500,
    /// Partial color (green only)
    PartColorGreen = 0x00000501,
    /// Partial color (blue only)
    PartColorBlue = 0x00000502,
    /// Partial color (yellow only)
    PartColorYellow = 0x00000503,
    /// High contrast monochrome
    HighContrastMonochrome = 0x00000600,
    /// Soft focus (low)
    SoftFocusLow = 0x00000700,
    /// Soft focus (medium)
    SoftFocusMid = 0x00000701,
    /// Soft focus (high)
    SoftFocusHigh = 0x00000702,
    /// HDR painting (low)
    HDRPaintingLow = 0x00000800,
    /// HDR painting (medium)
    HDRPaintingMid = 0x00000801,
    /// HDR painting (high)
    HDRPaintingHigh = 0x00000802,
    /// Rich tone monochrome
    RichToneMonochrome = 0x00000901,
    /// Miniature (auto)
    MiniatureAuto = 0x00000A00,
    /// Miniature (top)
    MiniatureTop = 0x00000A01,
    /// Miniature (middle horizontal)
    MiniatureMidHorizontal = 0x00000A02,
    /// Miniature (bottom)
    MiniatureBottom = 0x00000A03,
    /// Miniature (left)
    MiniatureLeft = 0x00000A04,
    /// Miniature (middle vertical)
    MiniatureMidVertical = 0x00000A05,
    /// Miniature (right)
    MiniatureRight = 0x00000A06,
    /// Watercolor effect
    MiniatureWaterColor = 0x00000B00,
    /// Illustration (low)
    MiniatureIllustrationLow = 0x00000C00,
    /// Illustration (medium)
    MiniatureIllustrationMid = 0x00000C01,
    /// Illustration (high)
    MiniatureIllustrationHigh = 0x00000C02,
}

impl ToCrsdk<u64> for PictureEffect {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for PictureEffect {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u32 {
            0x00000000 => Self::Off,
            0x00000001 => Self::ToyCameraNormal,
            0x00000002 => Self::ToyCameraCool,
            0x00000003 => Self::ToyCameraWarm,
            0x00000004 => Self::ToyCameraGreen,
            0x00000005 => Self::ToyCameraMagenta,
            0x00000100 => Self::Pop,
            0x00000200 => Self::PosterizationBW,
            0x00000201 => Self::PosterizationColor,
            0x00000300 => Self::Retro,
            0x00000400 => Self::SoftHighkey,
            0x00000500 => Self::PartColorRed,
            0x00000501 => Self::PartColorGreen,
            0x00000502 => Self::PartColorBlue,
            0x00000503 => Self::PartColorYellow,
            0x00000600 => Self::HighContrastMonochrome,
            0x00000700 => Self::SoftFocusLow,
            0x00000701 => Self::SoftFocusMid,
            0x00000702 => Self::SoftFocusHigh,
            0x00000800 => Self::HDRPaintingLow,
            0x00000801 => Self::HDRPaintingMid,
            0x00000802 => Self::HDRPaintingHigh,
            0x00000901 => Self::RichToneMonochrome,
            0x00000A00 => Self::MiniatureAuto,
            0x00000A01 => Self::MiniatureTop,
            0x00000A02 => Self::MiniatureMidHorizontal,
            0x00000A03 => Self::MiniatureBottom,
            0x00000A04 => Self::MiniatureLeft,
            0x00000A05 => Self::MiniatureMidVertical,
            0x00000A06 => Self::MiniatureRight,
            0x00000B00 => Self::MiniatureWaterColor,
            0x00000C00 => Self::MiniatureIllustrationLow,
            0x00000C01 => Self::MiniatureIllustrationMid,
            0x00000C02 => Self::MiniatureIllustrationHigh,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for PictureEffect {}

impl fmt::Display for PictureEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::ToyCameraNormal => write!(f, "Toy Camera"),
            Self::ToyCameraCool => write!(f, "Toy Camera (Cool)"),
            Self::ToyCameraWarm => write!(f, "Toy Camera (Warm)"),
            Self::ToyCameraGreen => write!(f, "Toy Camera (Green)"),
            Self::ToyCameraMagenta => write!(f, "Toy Camera (Magenta)"),
            Self::Pop => write!(f, "Pop Color"),
            Self::PosterizationBW => write!(f, "Posterization B&W"),
            Self::PosterizationColor => write!(f, "Posterization Color"),
            Self::Retro => write!(f, "Retro Photo"),
            Self::SoftHighkey => write!(f, "Soft High-key"),
            Self::PartColorRed => write!(f, "Partial Color (Red)"),
            Self::PartColorGreen => write!(f, "Partial Color (Green)"),
            Self::PartColorBlue => write!(f, "Partial Color (Blue)"),
            Self::PartColorYellow => write!(f, "Partial Color (Yellow)"),
            Self::HighContrastMonochrome => write!(f, "High Contrast Mono"),
            Self::SoftFocusLow => write!(f, "Soft Focus (Low)"),
            Self::SoftFocusMid => write!(f, "Soft Focus (Mid)"),
            Self::SoftFocusHigh => write!(f, "Soft Focus (High)"),
            Self::HDRPaintingLow => write!(f, "HDR Painting (Low)"),
            Self::HDRPaintingMid => write!(f, "HDR Painting (Mid)"),
            Self::HDRPaintingHigh => write!(f, "HDR Painting (High)"),
            Self::RichToneMonochrome => write!(f, "Rich-tone Mono"),
            Self::MiniatureAuto => write!(f, "Miniature (Auto)"),
            Self::MiniatureTop => write!(f, "Miniature (Top)"),
            Self::MiniatureMidHorizontal => write!(f, "Miniature (Mid H)"),
            Self::MiniatureBottom => write!(f, "Miniature (Bottom)"),
            Self::MiniatureLeft => write!(f, "Miniature (Left)"),
            Self::MiniatureMidVertical => write!(f, "Miniature (Mid V)"),
            Self::MiniatureRight => write!(f, "Miniature (Right)"),
            Self::MiniatureWaterColor => write!(f, "Watercolor"),
            Self::MiniatureIllustrationLow => write!(f, "Illustration (Low)"),
            Self::MiniatureIllustrationMid => write!(f, "Illustration (Mid)"),
            Self::MiniatureIllustrationHigh => write!(f, "Illustration (High)"),
        }
    }
}

/// Picture profile gamma curve selection.
///
/// Different gamma curves optimize for different workflows: broadcast (ITU709),
/// cinema (Cine1-4), log capture (S-Log2/3), or HDR (HLG).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum PictureProfileGamma {
    /// Movie gamma curve
    Movie = 0x0001,
    /// Still gamma curve
    Still = 0x0002,
    /// S-Cinetone gamma
    SCinetone = 0x0003,
    /// Cine1 gamma curve
    Cine1 = 0x0101,
    /// Cine2 gamma curve
    Cine2 = 0x0102,
    /// Cine3 gamma curve
    Cine3 = 0x0103,
    /// Cine4 gamma curve
    Cine4 = 0x0104,
    /// ITU709 standard gamma
    ITU709 = 0x0201,
    /// ITU709 (800%) extended range
    ITU709_800 = 0x0202,
    /// S-Log2 logarithmic gamma
    SLog2 = 0x0302,
    /// S-Log3 logarithmic gamma
    SLog3 = 0x0303,
    /// HLG (Hybrid Log-Gamma)
    HLG = 0x0401,
    /// HLG1 variant
    HLG1 = 0x0402,
    /// HLG2 variant
    HLG2 = 0x0403,
    /// HLG3 variant
    HLG3 = 0x0404,
}

impl ToCrsdk<u64> for PictureProfileGamma {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for PictureProfileGamma {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0001 => Self::Movie,
            0x0002 => Self::Still,
            0x0003 => Self::SCinetone,
            0x0101 => Self::Cine1,
            0x0102 => Self::Cine2,
            0x0103 => Self::Cine3,
            0x0104 => Self::Cine4,
            0x0201 => Self::ITU709,
            0x0202 => Self::ITU709_800,
            0x0302 => Self::SLog2,
            0x0303 => Self::SLog3,
            0x0401 => Self::HLG,
            0x0402 => Self::HLG1,
            0x0403 => Self::HLG2,
            0x0404 => Self::HLG3,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for PictureProfileGamma {}

impl fmt::Display for PictureProfileGamma {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Movie => write!(f, "Movie"),
            Self::Still => write!(f, "Still"),
            Self::SCinetone => write!(f, "S-Cinetone"),
            Self::Cine1 => write!(f, "Cine1"),
            Self::Cine2 => write!(f, "Cine2"),
            Self::Cine3 => write!(f, "Cine3"),
            Self::Cine4 => write!(f, "Cine4"),
            Self::ITU709 => write!(f, "ITU709"),
            Self::ITU709_800 => write!(f, "ITU709 (800%)"),
            Self::SLog2 => write!(f, "S-Log2"),
            Self::SLog3 => write!(f, "S-Log3"),
            Self::HLG => write!(f, "HLG"),
            Self::HLG1 => write!(f, "HLG1"),
            Self::HLG2 => write!(f, "HLG2"),
            Self::HLG3 => write!(f, "HLG3"),
        }
    }
}

/// Picture profile color mode setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum PictureProfileColorMode {
    /// Movie color mode
    Movie = 0x0001,
    /// Still color mode
    Still = 0x0002,
    /// S-Cinetone color mode
    SCinetone = 0x0003,
    /// Cinema color mode
    Cinema = 0x0004,
    /// Pro color mode
    Pro = 0x0005,
    /// ITU709 Matrix color mode
    ITU709Matrix = 0x0006,
    /// Black & White mode
    BlackWhite = 0x0007,
    /// S-Gamut3.Cine color mode
    SGamut3Cine = 0x0008,
    /// S-Gamut3 color mode
    SGamut3 = 0x0009,
    /// BT.2020 color mode
    BT2020 = 0x000A,
    /// 709 color mode
    Color709 = 0x000B,
    /// S-Gamut color mode
    SGamut = 0x000C,
    /// 709 tone color mode
    Color709Tone = 0x000D,
}

impl ToCrsdk<u64> for PictureProfileColorMode {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for PictureProfileColorMode {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0001 => Self::Movie,
            0x0002 => Self::Still,
            0x0003 => Self::SCinetone,
            0x0004 => Self::Cinema,
            0x0005 => Self::Pro,
            0x0006 => Self::ITU709Matrix,
            0x0007 => Self::BlackWhite,
            0x0008 => Self::SGamut3Cine,
            0x0009 => Self::SGamut3,
            0x000A => Self::BT2020,
            0x000B => Self::Color709,
            0x000C => Self::SGamut,
            0x000D => Self::Color709Tone,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for PictureProfileColorMode {}

impl fmt::Display for PictureProfileColorMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Movie => write!(f, "Movie"),
            Self::Still => write!(f, "Still"),
            Self::SCinetone => write!(f, "S-Cinetone"),
            Self::Cinema => write!(f, "Cinema"),
            Self::Pro => write!(f, "Pro"),
            Self::ITU709Matrix => write!(f, "ITU709 Matrix"),
            Self::BlackWhite => write!(f, "Black&White"),
            Self::SGamut3Cine => write!(f, "S-Gamut3.Cine"),
            Self::SGamut3 => write!(f, "S-Gamut3"),
            Self::BT2020 => write!(f, "BT.2020"),
            Self::Color709 => write!(f, "709"),
            Self::SGamut => write!(f, "S-Gamut"),
            Self::Color709Tone => write!(f, "709tone"),
        }
    }
}

/// Picture profile selection (PP1-PP11).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PictureProfile {
    /// Picture profile off
    Off = 0x00,
    /// Picture profile 1
    PP1 = 0x01,
    /// Picture profile 2
    PP2 = 0x02,
    /// Picture profile 3
    PP3 = 0x03,
    /// Picture profile 4
    PP4 = 0x04,
    /// Picture profile 5
    PP5 = 0x05,
    /// Picture profile 6
    PP6 = 0x06,
    /// Picture profile 7
    PP7 = 0x07,
    /// Picture profile 8
    PP8 = 0x08,
    /// Picture profile 9
    PP9 = 0x09,
    /// Picture profile 10
    PP10 = 0x0A,
    /// Picture profile 11
    PP11 = 0x0B,
}

impl ToCrsdk<u64> for PictureProfile {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for PictureProfile {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x00 => Self::Off,
            0x01 => Self::PP1,
            0x02 => Self::PP2,
            0x03 => Self::PP3,
            0x04 => Self::PP4,
            0x05 => Self::PP5,
            0x06 => Self::PP6,
            0x07 => Self::PP7,
            0x08 => Self::PP8,
            0x09 => Self::PP9,
            0x0A => Self::PP10,
            0x0B => Self::PP11,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for PictureProfile {}

impl fmt::Display for PictureProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::PP1 => write!(f, "PP1"),
            Self::PP2 => write!(f, "PP2"),
            Self::PP3 => write!(f, "PP3"),
            Self::PP4 => write!(f, "PP4"),
            Self::PP5 => write!(f, "PP5"),
            Self::PP6 => write!(f, "PP6"),
            Self::PP7 => write!(f, "PP7"),
            Self::PP8 => write!(f, "PP8"),
            Self::PP9 => write!(f, "PP9"),
            Self::PP10 => write!(f, "PP10"),
            Self::PP11 => write!(f, "PP11"),
        }
    }
}

/// Picture profile black gamma range setting.
///
/// Defines the range of shadow tones affected by black gamma adjustment.
/// Controls how wide the tonal range is for shadow detail manipulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PictureProfileBlackGammaRange {
    /// Wide shadow tone range
    Wide = 0x01,
    /// Middle shadow tone range
    Middle = 0x02,
    /// Narrow shadow tone range
    Narrow = 0x03,
}

impl ToCrsdk<u64> for PictureProfileBlackGammaRange {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for PictureProfileBlackGammaRange {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Wide,
            0x02 => Self::Middle,
            0x03 => Self::Narrow,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for PictureProfileBlackGammaRange {}

impl fmt::Display for PictureProfileBlackGammaRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wide => write!(f, "Wide"),
            Self::Middle => write!(f, "Middle"),
            Self::Narrow => write!(f, "Narrow"),
        }
    }
}

/// Picture profile detail adjustment mode.
///
/// Controls whether detail (sharpening) is adjusted automatically or manually.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PictureProfileDetailAdjustMode {
    /// Automatic detail adjustment
    Auto = 0x01,
    /// Manual detail adjustment
    Manual = 0x02,
}

impl ToCrsdk<u64> for PictureProfileDetailAdjustMode {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for PictureProfileDetailAdjustMode {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Auto,
            0x02 => Self::Manual,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for PictureProfileDetailAdjustMode {}

impl fmt::Display for PictureProfileDetailAdjustMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::Manual => write!(f, "Manual"),
        }
    }
}

/// Picture profile knee mode.
///
/// Controls highlight compression mode. Auto adjusts dynamically,
/// Manual gives direct control over knee point and slope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum PictureProfileKneeMode {
    /// Automatic knee adjustment
    Auto = 0x01,
    /// Manual knee adjustment
    Manual = 0x02,
}

impl ToCrsdk<u64> for PictureProfileKneeMode {
    fn to_crsdk(&self) -> u64 {
        *self as i8 as u64
    }
}

impl FromCrsdk<u64> for PictureProfileKneeMode {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as i8 {
            0x01 => Self::Auto,
            0x02 => Self::Manual,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for PictureProfileKneeMode {}

impl fmt::Display for PictureProfileKneeMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::Manual => write!(f, "Manual"),
        }
    }
}

/// Picture profile knee auto-set sensitivity.
///
/// Controls how aggressively auto knee compression responds to highlights.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PictureProfileKneeAutoSetSensitivity {
    /// Low sensitivity (less aggressive compression)
    Low = 0x01,
    /// Medium sensitivity
    Mid = 0x02,
    /// High sensitivity (more aggressive compression)
    High = 0x03,
}

impl ToCrsdk<u64> for PictureProfileKneeAutoSetSensitivity {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for PictureProfileKneeAutoSetSensitivity {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Low,
            0x02 => Self::Mid,
            0x03 => Self::High,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for PictureProfileKneeAutoSetSensitivity {}

impl fmt::Display for PictureProfileKneeAutoSetSensitivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "Low"),
            Self::Mid => write!(f, "Mid"),
            Self::High => write!(f, "High"),
        }
    }
}

/// Picture profile reset enable status.
///
/// Indicates whether the picture profile can be reset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PictureProfileResetEnableStatus {
    /// Reset is disabled
    Disable = 0x00,
    /// Reset is enabled
    Enable = 0x01,
}

impl ToCrsdk<u64> for PictureProfileResetEnableStatus {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for PictureProfileResetEnableStatus {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x00 => Self::Disable,
            0x01 => Self::Enable,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for PictureProfileResetEnableStatus {}

impl fmt::Display for PictureProfileResetEnableStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disable => write!(f, "Disabled"),
            Self::Enable => write!(f, "Enabled"),
        }
    }
}

/// Creative look reset enable status.
///
/// Indicates whether the creative look can be reset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CreativeLookResetEnableStatus {
    /// Reset is disabled
    Disable = 0x00,
    /// Reset is enabled
    Enable = 0x01,
}

impl ToCrsdk<u64> for CreativeLookResetEnableStatus {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for CreativeLookResetEnableStatus {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x00 => Self::Disable,
            0x01 => Self::Enable,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for CreativeLookResetEnableStatus {}

impl fmt::Display for CreativeLookResetEnableStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disable => write!(f, "Disabled"),
            Self::Enable => write!(f, "Enabled"),
        }
    }
}

/// Timecode preset reset enable status.
///
/// Indicates whether the timecode preset can be reset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TimeCodePresetResetEnableStatus {
    /// Reset is disabled
    Disable = 0x00,
    /// Reset is enabled
    Enable = 0x01,
}

impl ToCrsdk<u64> for TimeCodePresetResetEnableStatus {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for TimeCodePresetResetEnableStatus {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x00 => Self::Disable,
            0x01 => Self::Enable,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for TimeCodePresetResetEnableStatus {}

impl fmt::Display for TimeCodePresetResetEnableStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disable => write!(f, "Disabled"),
            Self::Enable => write!(f, "Enabled"),
        }
    }
}

/// Focus operation with 16-bit precision enable status.
///
/// Indicates whether 16-bit precision focus control is available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FocusOperationWithInt16EnableStatus {
    /// 16-bit focus operation is disabled
    Disable = 0x00,
    /// 16-bit focus operation is enabled
    Enable = 0x01,
}

impl ToCrsdk<u64> for FocusOperationWithInt16EnableStatus {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for FocusOperationWithInt16EnableStatus {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x00 => Self::Disable,
            0x01 => Self::Enable,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for FocusOperationWithInt16EnableStatus {}

impl fmt::Display for FocusOperationWithInt16EnableStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disable => write!(f, "Disabled"),
            Self::Enable => write!(f, "Enabled"),
        }
    }
}

/// Grid line overlay type for composition assistance.
///
/// Provides various grid patterns overlaid on the camera display
/// to help with composition and framing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GridLineType {
    /// Rule of thirds grid (3x3)
    RuleOf3rds = 0x01,
    /// Square grid
    Square = 0x02,
    /// Diagonal and square grid
    DiagAndSquare = 0x03,
    /// Golden ratio grid
    GoldenRatio = 0x04,
    /// Custom grid pattern 1
    Custom1 = 0x11,
    /// Custom grid pattern 2
    Custom2 = 0x12,
    /// Custom grid pattern 3
    Custom3 = 0x13,
    /// Custom grid pattern 4
    Custom4 = 0x14,
}

impl ToCrsdk<u64> for GridLineType {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for GridLineType {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::RuleOf3rds,
            0x02 => Self::Square,
            0x03 => Self::DiagAndSquare,
            0x04 => Self::GoldenRatio,
            0x11 => Self::Custom1,
            0x12 => Self::Custom2,
            0x13 => Self::Custom3,
            0x14 => Self::Custom4,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for GridLineType {}

impl fmt::Display for GridLineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::RuleOf3rds => "Rule of 3rds",
            Self::Square => "Square Grid",
            Self::DiagAndSquare => "Diag+Square",
            Self::GoldenRatio => "Golden Ratio",
            Self::Custom1 => "Custom 1",
            Self::Custom2 => "Custom 2",
            Self::Custom3 => "Custom 3",
            Self::Custom4 => "Custom 4",
        };
        write!(f, "{}", s)
    }
}

/// Streaming status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StreamStatus {
    /// Stream is inactive
    Inactive = 0x01,
    /// Stream is idle
    Idle = 0x02,
    /// Stream is actively streaming
    Streaming = 0x03,
    /// Stream is in error state
    Error = 0x04,
    /// Stream is ready
    Ready = 0x05,
}

impl ToCrsdk<u64> for StreamStatus {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for StreamStatus {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Inactive,
            0x02 => Self::Idle,
            0x03 => Self::Streaming,
            0x04 => Self::Error,
            0x05 => Self::Ready,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for StreamStatus {}

impl fmt::Display for StreamStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inactive => write!(f, "Inactive"),
            Self::Idle => write!(f, "Idle"),
            Self::Streaming => write!(f, "Streaming"),
            Self::Error => write!(f, "Error"),
            Self::Ready => write!(f, "Ready"),
        }
    }
}

/// Stream encryption cipher type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StreamCipherType {
    /// No encryption
    None = 0x01,
    /// AES-128 encryption
    AES128 = 0x02,
    /// AES-192 encryption
    AES192 = 0x03,
    /// AES-256 encryption
    AES256 = 0x04,
}

impl ToCrsdk<u64> for StreamCipherType {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for StreamCipherType {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::None,
            0x02 => Self::AES128,
            0x03 => Self::AES192,
            0x04 => Self::AES256,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for StreamCipherType {}

impl fmt::Display for StreamCipherType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::AES128 => write!(f, "AES-128"),
            Self::AES192 => write!(f, "AES-192"),
            Self::AES256 => write!(f, "AES-256"),
        }
    }
}

/// Imager/sensor scan mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ImagerScanMode {
    /// Auto scan mode selection
    Auto = 0x01,
    /// Full frame scan mode
    FullFrame = 0x02,
    /// Super 35mm scan mode
    Super35mm = 0x03,
}

impl ToCrsdk<u64> for ImagerScanMode {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for ImagerScanMode {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::Auto,
            0x02 => Self::FullFrame,
            0x03 => Self::Super35mm,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for ImagerScanMode {}

impl fmt::Display for ImagerScanMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::FullFrame => write!(f, "Full Frame"),
            Self::Super35mm => write!(f, "Super 35mm"),
        }
    }
}

/// Auto-framing/E-framing type for PTZ control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EframingType {
    /// E-framing disabled
    None = 0x01,
    /// Auto framing mode
    Auto = 0x02,
    /// Single framing mode
    Single = 0x03,
    /// Pan-tilt-zoom control mode
    PTZ = 0x05,
    /// Hold current position
    HoldCurrentPosition = 0x08,
    /// Force zoom out
    ForceZoomOut = 0x09,
}

impl ToCrsdk<u64> for EframingType {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for EframingType {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x01 => Self::None,
            0x02 => Self::Auto,
            0x03 => Self::Single,
            0x05 => Self::PTZ,
            0x08 => Self::HoldCurrentPosition,
            0x09 => Self::ForceZoomOut,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for EframingType {}

impl fmt::Display for EframingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Auto => write!(f, "Auto"),
            Self::Single => write!(f, "Single"),
            Self::PTZ => write!(f, "PTZ"),
            Self::HoldCurrentPosition => write!(f, "Hold Position"),
            Self::ForceZoomOut => write!(f, "Force Zoom Out"),
        }
    }
}

/// Video stream codec selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum VideoStreamCodec {
    /// Streaming disabled
    Off = 0x0001,
    /// Motion JPEG codec
    MotionJpeg = 0x0002,
    /// H.264 codec
    H264 = 0x0003,
    /// H.265 codec
    H265 = 0x0004,
}

impl ToCrsdk<u64> for VideoStreamCodec {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for VideoStreamCodec {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0001 => Self::Off,
            0x0002 => Self::MotionJpeg,
            0x0003 => Self::H264,
            0x0004 => Self::H265,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for VideoStreamCodec {}

impl fmt::Display for VideoStreamCodec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::MotionJpeg => write!(f, "Motion JPEG"),
            Self::H264 => write!(f, "H.264"),
            Self::H265 => write!(f, "H.265"),
        }
    }
}

/// Monitoring output format selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MonitoringOutputFormat {
    /// No output
    None = 0x00,
    /// 720x480p at 59.94Hz or 720x576p at 50Hz
    P720x480_576 = 0x01,
    /// 1920x1080i
    I1080 = 0x02,
    /// 1920x1080i PsF
    I1080PsF = 0x03,
    /// 1920x1080p
    P1080 = 0x04,
    /// 1920x1080p Level A
    P1080LevelA = 0x05,
    /// 1920x1080p Level B
    P1080LevelB = 0x06,
    /// 3840x2160p (4K UHD)
    P3840x2160 = 0x07,
    /// 4096x2160p (4K DCI)
    P4096x2160 = 0x08,
    /// 1280x720p
    P720 = 0x09,
}

impl ToCrsdk<u64> for MonitoringOutputFormat {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for MonitoringOutputFormat {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u8 {
            0x00 => Self::None,
            0x01 => Self::P720x480_576,
            0x02 => Self::I1080,
            0x03 => Self::I1080PsF,
            0x04 => Self::P1080,
            0x05 => Self::P1080LevelA,
            0x06 => Self::P1080LevelB,
            0x07 => Self::P3840x2160,
            0x08 => Self::P4096x2160,
            0x09 => Self::P720,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for MonitoringOutputFormat {}

impl fmt::Display for MonitoringOutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::P720x480_576 => write!(f, "720x480p/720x576p"),
            Self::I1080 => write!(f, "1080i"),
            Self::I1080PsF => write!(f, "1080i PsF"),
            Self::P1080 => write!(f, "1080p"),
            Self::P1080LevelA => write!(f, "1080p Level A"),
            Self::P1080LevelB => write!(f, "1080p Level B"),
            Self::P3840x2160 => write!(f, "4K (3840x2160)"),
            Self::P4096x2160 => write!(f, "4K (4096x2160)"),
            Self::P720 => write!(f, "720p"),
        }
    }
}

/// Face detection frame type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FaceFrameType {
    /// Unknown face frame type
    Unknown = 0x0000,
    /// Detected face
    DetectedFace = 0x0001,
    /// AF target face
    AFTargetFace = 0x0002,
    /// Personal recognition face
    PersonalRecognition = 0x0003,
    /// Smile detection face
    SmileDetection = 0x0004,
    /// Selected face
    SelectedFace = 0x0005,
    /// AF target selection face
    AFTargetSelection = 0x0006,
    /// Smile detection select face
    SmileDetectionSelect = 0x0007,
}

impl ToCrsdk<u64> for FaceFrameType {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for FaceFrameType {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0000 => Self::Unknown,
            0x0001 => Self::DetectedFace,
            0x0002 => Self::AFTargetFace,
            0x0003 => Self::PersonalRecognition,
            0x0004 => Self::SmileDetection,
            0x0005 => Self::SelectedFace,
            0x0006 => Self::AFTargetSelection,
            0x0007 => Self::SmileDetectionSelect,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for FaceFrameType {}

impl fmt::Display for FaceFrameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::DetectedFace => write!(f, "Detected Face"),
            Self::AFTargetFace => write!(f, "AF Target Face"),
            Self::PersonalRecognition => write!(f, "Personal Recognition"),
            Self::SmileDetection => write!(f, "Smile Detection"),
            Self::SelectedFace => write!(f, "Selected Face"),
            Self::AFTargetSelection => write!(f, "AF Target Selection"),
            Self::SmileDetectionSelect => write!(f, "Smile Detection Select"),
        }
    }
}

/// Frame information type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FrameInfoType {
    /// Unknown frame info type
    Unknown = 0x0000,
    /// Focus frame information
    FocusFrameInfo = 0x0001,
    /// Magnifier position
    MagnifierPosition = 0x0002,
    /// Face frame information
    FaceFrameInfo = 0x0004,
    /// Tracking frame information
    TrackingFrameInfo = 0x0005,
    /// Framing frame information
    FramingFrameInfo = 0x0006,
    /// Level information
    Level = 0x0007,
}

impl ToCrsdk<u64> for FrameInfoType {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for FrameInfoType {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0000 => Self::Unknown,
            0x0001 => Self::FocusFrameInfo,
            0x0002 => Self::MagnifierPosition,
            0x0004 => Self::FaceFrameInfo,
            0x0005 => Self::TrackingFrameInfo,
            0x0006 => Self::FramingFrameInfo,
            0x0007 => Self::Level,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for FrameInfoType {}

impl fmt::Display for FrameInfoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::FocusFrameInfo => write!(f, "Focus Frame Info"),
            Self::MagnifierPosition => write!(f, "Magnifier Position"),
            Self::FaceFrameInfo => write!(f, "Face Frame Info"),
            Self::TrackingFrameInfo => write!(f, "Tracking Frame Info"),
            Self::FramingFrameInfo => write!(f, "Framing Frame Info"),
            Self::Level => write!(f, "Level"),
        }
    }
}

/// Focus frame type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FocusFrameType {
    /// Unknown focus frame type
    Unknown = 0x0000,
    /// Phase detection AF sensor
    PhaseDetectionAFSensor = 0x0001,
    /// Phase detection image sensor
    PhaseDetectionImageSensor = 0x0002,
    /// Wide focus frame
    Wide = 0x0003,
    /// Zone focus frame
    Zone = 0x0004,
    /// Central emphasis focus frame
    CentralEmphasis = 0x0005,
    /// Contrast flexible main focus frame
    ContrastFlexibleMain = 0x0006,
    /// Contrast flexible assist focus frame
    ContrastFlexibleAssist = 0x0007,
    /// Contrast focus frame
    Contrast = 0x0008,
    /// Frame somewhere (arbitrary position)
    FrameSomewhere = 0x000F,
}

impl ToCrsdk<u64> for FocusFrameType {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl FromCrsdk<u64> for FocusFrameType {
    fn from_crsdk(raw: u64) -> Result<Self> {
        Ok(match raw as u16 {
            0x0000 => Self::Unknown,
            0x0001 => Self::PhaseDetectionAFSensor,
            0x0002 => Self::PhaseDetectionImageSensor,
            0x0003 => Self::Wide,
            0x0004 => Self::Zone,
            0x0005 => Self::CentralEmphasis,
            0x0006 => Self::ContrastFlexibleMain,
            0x0007 => Self::ContrastFlexibleAssist,
            0x0008 => Self::Contrast,
            0x000F => Self::FrameSomewhere,
            _ => return Err(Error::InvalidPropertyValue),
        })
    }
}

impl PropertyValue for FocusFrameType {}

impl fmt::Display for FocusFrameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::PhaseDetectionAFSensor => write!(f, "Phase Detection AF Sensor"),
            Self::PhaseDetectionImageSensor => write!(f, "Phase Detection Image Sensor"),
            Self::Wide => write!(f, "Wide"),
            Self::Zone => write!(f, "Zone"),
            Self::CentralEmphasis => write!(f, "Central Emphasis"),
            Self::ContrastFlexibleMain => write!(f, "Contrast Flexible Main"),
            Self::ContrastFlexibleAssist => write!(f, "Contrast Flexible Assist"),
            Self::Contrast => write!(f, "Contrast"),
            Self::FrameSomewhere => write!(f, "Frame Somewhere"),
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
