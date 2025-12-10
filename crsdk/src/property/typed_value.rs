//! Typed property value representation.
//!
//! This module provides [`TypedValue`], a unified enum that can represent
//! any camera property value with proper type information and Display formatting.

use std::fmt;

use crsdk_sys::DevicePropertyCode;

use super::values::{
    Aperture, AspectRatio, AutoManual, BatteryLevel, ColorTemperature, ExposureComp,
    ExposureCtrlType, ExposureProgram, FileType, FlashMode, FocusArea, FocusIndicator, FocusMode,
    FocusTrackingStatus, ImageQuality, ImageSize, Integer, Iso, LiveViewDisplayEffect,
    LockIndicator, MeterLevel, MeteringMode, MovieQuality, OnOff, Percentage, PrioritySetInAF,
    PrioritySetInAWB, ShutterMode, ShutterModeStatus, ShutterSpeed, SilentModeApertureDrive,
    SubjectRecognitionAF, Switch, WhiteBalance,
};
use super::{property_value_type, PropertyValueType};
use super::{
    AudioSignals, BatteryRemainDisplayUnit, ColorSpace, DRangeOptimizer, DeviceOverheatingState,
    DispMode, DriveMode, FocusOperation, FunctionOfTouchOperation, HighIsoNR, IntervalRecMode,
    IntervalRecShutterType, IntervalRecStatus, LiveViewStatus, MediaSlotRecordingType,
    MediaSlotWritingState, MovieFileFormat, NearFarEnableStatus, PlaybackMedia, PowerSource,
    PriorityKeySettings, RecorderStatus, RecordingState, SdkControlMode, SelectFinder, ShutterType,
    SlotStatus, SoftSkinEffect, StillImageStoreDestination, TimeCodeFormat, TimeCodeMake,
    TimeCodeRun, TouchOperation, WindNoiseReduction,
};
use crate::property::traits::PropertyValue;

/// A typed property value with Display formatting.
///
/// This enum represents any camera property value in a type-safe manner.
/// Use [`TypedValue::from_raw`] to convert raw SDK values into properly
/// typed and formatted values.
///
/// # Example
///
/// ```ignore
/// use crsdk::property::TypedValue;
/// use crsdk_sys::DevicePropertyCode;
///
/// let value = TypedValue::from_raw(DevicePropertyCode::Aperture, 280);
/// assert_eq!(value.to_string(), "f/2.8");
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum TypedValue {
    /// Aperture value (f-number)
    Aperture(Aperture),
    /// Shutter speed value
    ShutterSpeed(ShutterSpeed),
    /// ISO sensitivity value
    Iso(Iso),
    /// Exposure compensation value
    ExposureComp(ExposureComp),
    /// Exposure meter level (signed)
    MeterLevel(MeterLevel),
    /// Exposure program mode (P/A/S/M/Auto/etc.)
    ExposureProgram(ExposureProgram),
    /// Metering mode (Multi/Center/Spot/etc.)
    MeteringMode(MeteringMode),
    /// Shutter mode status (Off/Speed/Angle/ECS/Auto)
    ShutterModeStatus(ShutterModeStatus),
    /// Shutter mode (Speed/Angle)
    ShutterMode(ShutterMode),
    /// Exposure control type (P/A/S/M vs Flexible)
    ExposureCtrlType(ExposureCtrlType),
    /// Focus mode (AF-S/AF-C/MF/etc.)
    FocusMode(FocusMode),
    /// Focus area (Wide/Zone/Center/Spot/etc.)
    FocusArea(FocusArea),
    /// Subject recognition in AF (Human/Animal/Bird/etc.)
    SubjectRecognitionAF(SubjectRecognitionAF),
    /// Priority setting in AF (AF/Release/Balanced)
    PrioritySetInAF(PrioritySetInAF),
    /// Focus tracking status (Off/Focusing/Tracking)
    FocusTrackingStatus(FocusTrackingStatus),
    /// Focus indicator (lock state)
    FocusIndicator(FocusIndicator),
    /// White balance setting (Auto/Daylight/Tungsten/etc.)
    WhiteBalance(WhiteBalance),
    /// Priority setting in AWB (Standard/Ambience/White)
    PrioritySetInAWB(PrioritySetInAWB),
    /// Color temperature in Kelvin
    ColorTemperature(ColorTemperature),
    /// Drive mode (Single/Continuous/Bracket/Timer/etc.)
    DriveMode(DriveMode),
    /// Interval recording shutter type
    IntervalRecShutterType(IntervalRecShutterType),
    /// Flash mode (Auto/Off/Fill/Sync/etc.)
    FlashMode(FlashMode),
    /// File type (JPEG/RAW/RAW+JPEG/HEIF)
    FileType(FileType),
    /// JPEG image quality (Standard/Fine/ExFine)
    ImageQuality(ImageQuality),
    /// Aspect ratio (3:2/16:9/4:3/1:1)
    AspectRatio(AspectRatio),
    /// Image size (L/M/S/VGA)
    ImageSize(ImageSize),
    /// Movie file format (XAVC/AVCHD/etc.)
    MovieFileFormat(MovieFileFormat),
    /// Movie quality setting
    MovieQuality(MovieQuality),
    /// Movie recording state
    RecordingState(RecordingState),
    /// Recorder status (main or proxy)
    RecorderStatus(RecorderStatus),
    /// Timecode format (DF/NDF)
    TimeCodeFormat(TimeCodeFormat),
    /// Timecode run mode
    TimeCodeRun(TimeCodeRun),
    /// Timecode make mode
    TimeCodeMake(TimeCodeMake),
    /// Live view status
    LiveViewStatus(LiveViewStatus),
    /// Memory card slot status
    SlotStatus(SlotStatus),
    /// Media slot writing state
    MediaSlotWritingState(MediaSlotWritingState),
    /// Media slot recording type
    MediaSlotRecordingType(MediaSlotRecordingType),
    /// SDK control mode (Remote/Transfer)
    SdkControlMode(SdkControlMode),
    /// Dynamic range optimizer
    DRangeOptimizer(DRangeOptimizer),
    /// Still image store destination
    StillImageStoreDestination(StillImageStoreDestination),
    /// Near/far focus enable status
    NearFarEnableStatus(NearFarEnableStatus),
    /// Interval recording mode
    IntervalRecMode(IntervalRecMode),
    /// Interval recording status
    IntervalRecStatus(IntervalRecStatus),
    /// Priority key settings
    PriorityKeySettings(PriorityKeySettings),
    /// Color space (sRGB/Adobe RGB)
    ColorSpace(ColorSpace),
    /// Playback media slot
    PlaybackMedia(PlaybackMedia),
    /// Touch operation mode
    TouchOperation(TouchOperation),
    /// Power source type
    PowerSource(PowerSource),
    /// Battery remaining display unit
    BatteryRemainDisplayUnit(BatteryRemainDisplayUnit),
    /// Focus operation direction
    FocusOperation(FocusOperation),
    /// Shutter type
    ShutterType(ShutterType),
    /// Device overheating state
    DeviceOverheatingState(DeviceOverheatingState),
    /// High ISO noise reduction
    HighIsoNR(HighIsoNR),
    /// Audio signals (beep) setting
    AudioSignals(AudioSignals),
    /// Touch operation function
    FunctionOfTouchOperation(FunctionOfTouchOperation),
    /// Soft skin effect level
    SoftSkinEffect(SoftSkinEffect),
    /// Wind noise reduction
    WindNoiseReduction(WindNoiseReduction),
    /// Finder/Monitor selection
    SelectFinder(SelectFinder),
    /// Display mode
    DispMode(DispMode),
    /// Battery level (packed status + percentage)
    BatteryLevel(BatteryLevel),
    /// Switch value (On/Off)
    Switch(Switch),
    /// On/Off value
    OnOff(OnOff),
    /// Auto/Manual mode
    AutoManual(AutoManual),
    /// Lock indicator (Locked/Unlocked)
    LockIndicator(LockIndicator),
    /// Live view display effect mode
    LiveViewDisplayEffect(LiveViewDisplayEffect),
    /// Silent mode aperture drive setting
    SilentModeApertureDrive(SilentModeApertureDrive),
    /// Percentage value
    Percentage(Percentage),
    /// Generic integer value
    Integer(Integer),
    /// Unknown or unrecognized value
    Unknown(u64),
}

impl TypedValue {
    /// Convert a raw SDK value to a typed value based on property code.
    ///
    /// This is the primary way to convert raw camera property values into
    /// properly typed and formatted values.
    pub fn from_raw(code: DevicePropertyCode, raw: u64) -> Self {
        use PropertyValueType as PVT;

        match property_value_type(code) {
            PVT::Aperture => Aperture::from_raw(raw)
                .map(TypedValue::Aperture)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ShutterSpeed => ShutterSpeed::from_raw(raw)
                .map(TypedValue::ShutterSpeed)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::Iso => Iso::from_raw(raw)
                .map(TypedValue::Iso)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ExposureCompensation => ExposureComp::from_raw(raw)
                .map(TypedValue::ExposureComp)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ColorTemperature => ColorTemperature::from_raw(raw)
                .map(TypedValue::ColorTemperature)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::MovieQuality => MovieQuality::from_raw(raw)
                .map(TypedValue::MovieQuality)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ExposureProgram => ExposureProgram::from_raw(raw)
                .map(TypedValue::ExposureProgram)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::MeteringMode => MeteringMode::from_raw(raw)
                .map(TypedValue::MeteringMode)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FocusMode => FocusMode::from_raw(raw)
                .map(TypedValue::FocusMode)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FocusArea => FocusArea::from_raw(raw)
                .map(TypedValue::FocusArea)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::SubjectRecognitionAF => SubjectRecognitionAF::from_raw(raw)
                .map(TypedValue::SubjectRecognitionAF)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::PrioritySetInAF => PrioritySetInAF::from_raw(raw)
                .map(TypedValue::PrioritySetInAF)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FocusTrackingStatus => FocusTrackingStatus::from_raw(raw)
                .map(TypedValue::FocusTrackingStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FocusIndicator => FocusIndicator::from_raw(raw)
                .map(TypedValue::FocusIndicator)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::WhiteBalance => WhiteBalance::from_raw(raw)
                .map(TypedValue::WhiteBalance)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::PrioritySetInAWB => PrioritySetInAWB::from_raw(raw)
                .map(TypedValue::PrioritySetInAWB)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::DriveMode => DriveMode::from_raw(raw)
                .map(TypedValue::DriveMode)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::IntervalRecShutterType => IntervalRecShutterType::from_raw(raw)
                .map(TypedValue::IntervalRecShutterType)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FlashMode => FlashMode::from_raw(raw)
                .map(TypedValue::FlashMode)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FileType => FileType::from_raw(raw)
                .map(TypedValue::FileType)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ImageQuality => ImageQuality::from_raw(raw)
                .map(TypedValue::ImageQuality)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::AspectRatio => AspectRatio::from_raw(raw)
                .map(TypedValue::AspectRatio)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ImageSize => ImageSize::from_raw(raw)
                .map(TypedValue::ImageSize)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::MovieFileFormat => MovieFileFormat::from_raw(raw)
                .map(TypedValue::MovieFileFormat)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::RecordingState => RecordingState::from_raw(raw)
                .map(TypedValue::RecordingState)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::RecorderStatus => RecorderStatus::from_raw(raw)
                .map(TypedValue::RecorderStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::TimeCodeFormat => TimeCodeFormat::from_raw(raw)
                .map(TypedValue::TimeCodeFormat)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::TimeCodeRun => TimeCodeRun::from_raw(raw)
                .map(TypedValue::TimeCodeRun)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::TimeCodeMake => TimeCodeMake::from_raw(raw)
                .map(TypedValue::TimeCodeMake)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::LiveViewStatus => LiveViewStatus::from_raw(raw)
                .map(TypedValue::LiveViewStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::SlotStatus => SlotStatus::from_raw(raw)
                .map(TypedValue::SlotStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::MediaSlotWritingState => MediaSlotWritingState::from_raw(raw)
                .map(TypedValue::MediaSlotWritingState)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::MediaSlotRecordingType => MediaSlotRecordingType::from_raw(raw)
                .map(TypedValue::MediaSlotRecordingType)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::SdkControlMode => SdkControlMode::from_raw(raw)
                .map(TypedValue::SdkControlMode)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::DRangeOptimizer => DRangeOptimizer::from_raw(raw)
                .map(TypedValue::DRangeOptimizer)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::StillImageStoreDestination => StillImageStoreDestination::from_raw(raw)
                .map(TypedValue::StillImageStoreDestination)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::NearFarEnableStatus => NearFarEnableStatus::from_raw(raw)
                .map(TypedValue::NearFarEnableStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::IntervalRecMode => IntervalRecMode::from_raw(raw)
                .map(TypedValue::IntervalRecMode)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::IntervalRecStatus => IntervalRecStatus::from_raw(raw)
                .map(TypedValue::IntervalRecStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::PriorityKeySettings => PriorityKeySettings::from_raw(raw)
                .map(TypedValue::PriorityKeySettings)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ColorSpace => ColorSpace::from_raw(raw)
                .map(TypedValue::ColorSpace)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::PlaybackMedia => PlaybackMedia::from_raw(raw)
                .map(TypedValue::PlaybackMedia)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::TouchOperation => TouchOperation::from_raw(raw)
                .map(TypedValue::TouchOperation)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::PowerSource => PowerSource::from_raw(raw)
                .map(TypedValue::PowerSource)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::BatteryRemainDisplayUnit => BatteryRemainDisplayUnit::from_raw(raw)
                .map(TypedValue::BatteryRemainDisplayUnit)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FocusOperation => FocusOperation::from_raw(raw)
                .map(TypedValue::FocusOperation)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ShutterType => ShutterType::from_raw(raw)
                .map(TypedValue::ShutterType)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::DeviceOverheatingState => DeviceOverheatingState::from_raw(raw)
                .map(TypedValue::DeviceOverheatingState)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::HighIsoNR => HighIsoNR::from_raw(raw)
                .map(TypedValue::HighIsoNR)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::AudioSignals => AudioSignals::from_raw(raw)
                .map(TypedValue::AudioSignals)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FunctionOfTouchOperation => FunctionOfTouchOperation::from_raw(raw)
                .map(TypedValue::FunctionOfTouchOperation)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::SoftSkinEffect => SoftSkinEffect::from_raw(raw)
                .map(TypedValue::SoftSkinEffect)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::WindNoiseReduction => WindNoiseReduction::from_raw(raw)
                .map(TypedValue::WindNoiseReduction)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::SelectFinder => SelectFinder::from_raw(raw)
                .map(TypedValue::SelectFinder)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::DispMode => DispMode::from_raw(raw)
                .map(TypedValue::DispMode)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ShutterModeStatus => ShutterModeStatus::from_raw(raw)
                .map(TypedValue::ShutterModeStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ShutterMode => ShutterMode::from_raw(raw)
                .map(TypedValue::ShutterMode)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ExposureCtrlType => ExposureCtrlType::from_raw(raw)
                .map(TypedValue::ExposureCtrlType)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::LiveViewDisplayEffect => LiveViewDisplayEffect::from_raw(raw)
                .map(TypedValue::LiveViewDisplayEffect)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::SilentModeApertureDrive => SilentModeApertureDrive::from_raw(raw)
                .map(TypedValue::SilentModeApertureDrive)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::OnOff => OnOff::from_raw(raw)
                .map(TypedValue::OnOff)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::Switch => Switch::from_raw(raw)
                .map(TypedValue::Switch)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::AutoManual => AutoManual::from_raw(raw)
                .map(TypedValue::AutoManual)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::LockIndicator => LockIndicator::from_raw(raw)
                .map(TypedValue::LockIndicator)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::Percentage => {
                // Special handling for battery level which is a packed value
                if matches!(
                    code,
                    DevicePropertyCode::BatteryLevel | DevicePropertyCode::BatteryRemain
                ) {
                    BatteryLevel::from_raw(raw)
                        .map(TypedValue::BatteryLevel)
                        .unwrap_or(TypedValue::Unknown(raw))
                } else {
                    Percentage::from_raw(raw)
                        .map(TypedValue::Percentage)
                        .unwrap_or(TypedValue::Unknown(raw))
                }
            }
            PVT::Integer => {
                // Special handling for MeterLevel which is signed
                if code == DevicePropertyCode::MeteredManualLevel {
                    MeterLevel::from_raw(raw)
                        .map(TypedValue::MeterLevel)
                        .unwrap_or(TypedValue::Unknown(raw))
                } else {
                    Integer::from_raw(raw)
                        .map(TypedValue::Integer)
                        .unwrap_or(TypedValue::Unknown(raw))
                }
            }
            PVT::Unknown => TypedValue::Unknown(raw),
        }
    }

    /// Get the raw u64 value.
    pub fn to_raw(&self) -> u64 {
        match self {
            TypedValue::Aperture(v) => v.to_raw(),
            TypedValue::ShutterSpeed(v) => v.to_raw(),
            TypedValue::Iso(v) => v.to_raw(),
            TypedValue::ExposureComp(v) => v.to_raw(),
            TypedValue::MeterLevel(v) => v.to_raw(),
            TypedValue::ExposureProgram(v) => v.to_raw(),
            TypedValue::MeteringMode(v) => v.to_raw(),
            TypedValue::ShutterModeStatus(v) => v.to_raw(),
            TypedValue::ShutterMode(v) => v.to_raw(),
            TypedValue::ExposureCtrlType(v) => v.to_raw(),
            TypedValue::FocusMode(v) => v.to_raw(),
            TypedValue::FocusArea(v) => v.to_raw(),
            TypedValue::SubjectRecognitionAF(v) => v.to_raw(),
            TypedValue::PrioritySetInAF(v) => v.to_raw(),
            TypedValue::FocusTrackingStatus(v) => v.to_raw(),
            TypedValue::FocusIndicator(v) => v.to_raw(),
            TypedValue::WhiteBalance(v) => v.to_raw(),
            TypedValue::PrioritySetInAWB(v) => v.to_raw(),
            TypedValue::ColorTemperature(v) => v.to_raw(),
            TypedValue::DriveMode(v) => v.to_raw(),
            TypedValue::IntervalRecShutterType(v) => v.to_raw(),
            TypedValue::FlashMode(v) => v.to_raw(),
            TypedValue::FileType(v) => v.to_raw(),
            TypedValue::ImageQuality(v) => v.to_raw(),
            TypedValue::AspectRatio(v) => v.to_raw(),
            TypedValue::ImageSize(v) => v.to_raw(),
            TypedValue::MovieFileFormat(v) => v.to_raw(),
            TypedValue::MovieQuality(v) => v.to_raw(),
            TypedValue::RecordingState(v) => v.to_raw(),
            TypedValue::RecorderStatus(v) => v.to_raw(),
            TypedValue::TimeCodeFormat(v) => v.to_raw(),
            TypedValue::TimeCodeRun(v) => v.to_raw(),
            TypedValue::TimeCodeMake(v) => v.to_raw(),
            TypedValue::LiveViewStatus(v) => v.to_raw(),
            TypedValue::SlotStatus(v) => v.to_raw(),
            TypedValue::MediaSlotWritingState(v) => v.to_raw(),
            TypedValue::MediaSlotRecordingType(v) => v.to_raw(),
            TypedValue::SdkControlMode(v) => v.to_raw(),
            TypedValue::DRangeOptimizer(v) => v.to_raw(),
            TypedValue::StillImageStoreDestination(v) => v.to_raw(),
            TypedValue::NearFarEnableStatus(v) => v.to_raw(),
            TypedValue::IntervalRecMode(v) => v.to_raw(),
            TypedValue::IntervalRecStatus(v) => v.to_raw(),
            TypedValue::PriorityKeySettings(v) => v.to_raw(),
            TypedValue::ColorSpace(v) => v.to_raw(),
            TypedValue::PlaybackMedia(v) => v.to_raw(),
            TypedValue::TouchOperation(v) => v.to_raw(),
            TypedValue::PowerSource(v) => v.to_raw(),
            TypedValue::BatteryRemainDisplayUnit(v) => v.to_raw(),
            TypedValue::FocusOperation(v) => v.to_raw(),
            TypedValue::ShutterType(v) => v.to_raw(),
            TypedValue::DeviceOverheatingState(v) => v.to_raw(),
            TypedValue::HighIsoNR(v) => v.to_raw(),
            TypedValue::AudioSignals(v) => v.to_raw(),
            TypedValue::FunctionOfTouchOperation(v) => v.to_raw(),
            TypedValue::SoftSkinEffect(v) => v.to_raw(),
            TypedValue::WindNoiseReduction(v) => v.to_raw(),
            TypedValue::SelectFinder(v) => v.to_raw(),
            TypedValue::DispMode(v) => v.to_raw(),
            TypedValue::BatteryLevel(v) => v.to_raw(),
            TypedValue::Switch(v) => v.to_raw(),
            TypedValue::OnOff(v) => v.to_raw(),
            TypedValue::AutoManual(v) => v.to_raw(),
            TypedValue::LockIndicator(v) => v.to_raw(),
            TypedValue::LiveViewDisplayEffect(v) => v.to_raw(),
            TypedValue::SilentModeApertureDrive(v) => v.to_raw(),
            TypedValue::Percentage(v) => v.to_raw(),
            TypedValue::Integer(v) => v.to_raw(),
            TypedValue::Unknown(v) => *v,
        }
    }
}

impl fmt::Display for TypedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypedValue::Aperture(v) => write!(f, "{}", v),
            TypedValue::ShutterSpeed(v) => write!(f, "{}", v),
            TypedValue::Iso(v) => write!(f, "{}", v),
            TypedValue::ExposureComp(v) => write!(f, "{}", v),
            TypedValue::MeterLevel(v) => write!(f, "{}", v),
            TypedValue::ExposureProgram(v) => write!(f, "{}", v),
            TypedValue::MeteringMode(v) => write!(f, "{}", v),
            TypedValue::ShutterModeStatus(v) => write!(f, "{}", v),
            TypedValue::ShutterMode(v) => write!(f, "{}", v),
            TypedValue::ExposureCtrlType(v) => write!(f, "{}", v),
            TypedValue::FocusMode(v) => write!(f, "{}", v),
            TypedValue::FocusArea(v) => write!(f, "{}", v),
            TypedValue::SubjectRecognitionAF(v) => write!(f, "{}", v),
            TypedValue::PrioritySetInAF(v) => write!(f, "{}", v),
            TypedValue::FocusTrackingStatus(v) => write!(f, "{}", v),
            TypedValue::FocusIndicator(v) => write!(f, "{}", v),
            TypedValue::WhiteBalance(v) => write!(f, "{}", v),
            TypedValue::PrioritySetInAWB(v) => write!(f, "{}", v),
            TypedValue::ColorTemperature(v) => write!(f, "{}", v),
            TypedValue::DriveMode(v) => write!(f, "{}", v),
            TypedValue::IntervalRecShutterType(v) => write!(f, "{}", v),
            TypedValue::FlashMode(v) => write!(f, "{}", v),
            TypedValue::FileType(v) => write!(f, "{}", v),
            TypedValue::ImageQuality(v) => write!(f, "{}", v),
            TypedValue::AspectRatio(v) => write!(f, "{}", v),
            TypedValue::ImageSize(v) => write!(f, "{}", v),
            TypedValue::MovieFileFormat(v) => write!(f, "{}", v),
            TypedValue::MovieQuality(v) => write!(f, "{}", v),
            TypedValue::RecordingState(v) => write!(f, "{}", v),
            TypedValue::RecorderStatus(v) => write!(f, "{}", v),
            TypedValue::TimeCodeFormat(v) => write!(f, "{}", v),
            TypedValue::TimeCodeRun(v) => write!(f, "{}", v),
            TypedValue::TimeCodeMake(v) => write!(f, "{}", v),
            TypedValue::LiveViewStatus(v) => write!(f, "{}", v),
            TypedValue::SlotStatus(v) => write!(f, "{}", v),
            TypedValue::MediaSlotWritingState(v) => write!(f, "{}", v),
            TypedValue::MediaSlotRecordingType(v) => write!(f, "{}", v),
            TypedValue::SdkControlMode(v) => write!(f, "{}", v),
            TypedValue::DRangeOptimizer(v) => write!(f, "{}", v),
            TypedValue::StillImageStoreDestination(v) => write!(f, "{}", v),
            TypedValue::NearFarEnableStatus(v) => write!(f, "{}", v),
            TypedValue::IntervalRecMode(v) => write!(f, "{}", v),
            TypedValue::IntervalRecStatus(v) => write!(f, "{}", v),
            TypedValue::PriorityKeySettings(v) => write!(f, "{}", v),
            TypedValue::ColorSpace(v) => write!(f, "{}", v),
            TypedValue::PlaybackMedia(v) => write!(f, "{}", v),
            TypedValue::TouchOperation(v) => write!(f, "{}", v),
            TypedValue::PowerSource(v) => write!(f, "{}", v),
            TypedValue::BatteryRemainDisplayUnit(v) => write!(f, "{}", v),
            TypedValue::FocusOperation(v) => write!(f, "{}", v),
            TypedValue::ShutterType(v) => write!(f, "{}", v),
            TypedValue::DeviceOverheatingState(v) => write!(f, "{}", v),
            TypedValue::HighIsoNR(v) => write!(f, "{}", v),
            TypedValue::AudioSignals(v) => write!(f, "{}", v),
            TypedValue::FunctionOfTouchOperation(v) => write!(f, "{}", v),
            TypedValue::SoftSkinEffect(v) => write!(f, "{}", v),
            TypedValue::WindNoiseReduction(v) => write!(f, "{}", v),
            TypedValue::SelectFinder(v) => write!(f, "{}", v),
            TypedValue::DispMode(v) => write!(f, "{}", v),
            TypedValue::BatteryLevel(v) => write!(f, "{}", v),
            TypedValue::Switch(v) => write!(f, "{}", v),
            TypedValue::OnOff(v) => write!(f, "{}", v),
            TypedValue::AutoManual(v) => write!(f, "{}", v),
            TypedValue::LockIndicator(v) => write!(f, "{}", v),
            TypedValue::LiveViewDisplayEffect(v) => write!(f, "{}", v),
            TypedValue::SilentModeApertureDrive(v) => write!(f, "{}", v),
            TypedValue::Percentage(v) => write!(f, "{}", v),
            TypedValue::Integer(v) => write!(f, "{}", v),
            TypedValue::Unknown(v) => write!(f, "0x{:X}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aperture_from_raw() {
        let value = TypedValue::from_raw(DevicePropertyCode::FNumber, 280);
        assert_eq!(value.to_string(), "f/2.8");
    }

    #[test]
    fn test_shutter_speed_from_raw() {
        // 1/200
        let raw = (1u64 << 16) | 200;
        let value = TypedValue::from_raw(DevicePropertyCode::ShutterSpeed, raw);
        assert_eq!(value.to_string(), "1/200");
    }

    #[test]
    fn test_iso_from_raw() {
        let value = TypedValue::from_raw(DevicePropertyCode::IsoSensitivity, 3200);
        assert_eq!(value.to_string(), "ISO 3200");
    }

    #[test]
    fn test_battery_level_packed_value() {
        // Raw value 65541 = 0x10005 should decode to level 5
        let value = TypedValue::from_raw(DevicePropertyCode::BatteryLevel, 65541);
        assert_eq!(value.to_string(), "5");
    }

    #[test]
    fn test_meter_level_signed() {
        // Raw value that represents -8000 when interpreted as signed
        let raw = (-8000i64) as u64;
        let value = TypedValue::from_raw(DevicePropertyCode::MeteredManualLevel, raw);
        assert_eq!(value.to_string(), "-8000");
    }

    #[test]
    fn test_unknown_value() {
        let value = TypedValue::Unknown(0xDEADBEEF);
        assert_eq!(value.to_string(), "0xDEADBEEF");
    }

    #[test]
    fn test_round_trip() {
        let original = 280u64;
        let value = TypedValue::from_raw(DevicePropertyCode::FNumber, original);
        assert_eq!(value.to_raw(), original);
    }
}
