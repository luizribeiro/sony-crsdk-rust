//! Typed property value representation.
//!
//! This module provides [`TypedValue`], a unified enum that can represent
//! any camera property value with proper type information and Display formatting.

use std::fmt;

use crsdk_sys::DevicePropertyCode;

use super::values::{
    Aperture, AspectRatio, AudioInputCHInputSelect, AudioStreamChannel, AutoManual, BatteryLevel,
    CameraPowerStatus, ColorTemperature, CompressionFileFormat, EframingType, ExposureComp,
    ExposureCtrlType, ExposureProgram, FaceFrameType, FileType, FlashMode, FocusArea,
    FocusBracketOrder, FocusBracketShootingStatus, FocusDrivingStatus, FocusFrameState,
    FocusFrameType, FocusIndicator, FocusMode, FocusTouchSpotStatus, FocusTrackingStatus,
    FrameInfoType, GainUnitSetting, GridLineType, ImageQuality, ImageSize, Integer, Iso,
    LiveViewDisplayEffect, LiveViewProtocol, LockIndicator, MeterLevel, MeteringMode,
    MoviePlayingState, MovieQuality, MovieRecReviewPlayingState, MovieShootingMode,
    MovieShootingModeColorGamut, MovieShootingModeTargetDisplay, OnOff, Percentage, PictureProfile,
    PlaybackContentsGammaType, PrioritySetInAF, PrioritySetInAWB, PushAutoFocus,
    RecorderSaveDestination, RecordingFolderFormat, ShutterMode, ShutterModeStatus, ShutterSpeed,
    SilentModeApertureDrive, SubjectRecognitionAF, Switch, TrackingFrameType,
    VideoRecordingFormatQuality, VideoStreamCodec, WhiteBalance, WhiteBalanceSwitch,
    ZoomDrivingStatus, ZoomTypeStatus,
};
use super::{property_value_type, PropertyValueType};
use super::{
    AFTrackForSpeedChange, AFTrackingSensitivity, ApertureDriveInAF, AudioSignals,
    AudioStreamBitDepth, AutoPowerOffTemperature, BatteryRemainDisplayUnit, CameraOperatingMode,
    ColorSpace, CreativeLook, CreativeLookResetEnableStatus, CustomWBSizeSetting, DRangeOptimizer,
    DeviceOverheatingState, DispMode, DriveMode, EframingProductionEffect, FTPConnectionStatus,
    FocusOperation, FocusOperationWithInt16EnableStatus, FunctionOfTouchOperation,
    GainBaseSensitivity, HighIsoNR, ImageStabilizationLevelMovie,
    ImageStabilizationSteadyShotMovie, ImagerScanMode, IntervalRecMode, IntervalRecShutterType,
    IntervalRecStatus, IrisDisplayUnit, IsoAutoMinShutterSpeedMode, IsoAutoMinShutterSpeedPreset,
    LensCompensationShading, LiveViewStatus, MediaSlotRecordingType, MediaSlotWritingState,
    MonitoringOutputFormat, MovieFileFormat, NDFilterMode, NearFarEnableStatus, PictureEffect,
    PictureProfileBlackGammaRange, PictureProfileColorMode, PictureProfileDetailAdjustMode,
    PictureProfileGamma, PictureProfileKneeAutoSetSensitivity, PictureProfileKneeMode,
    PictureProfileResetEnableStatus, PlaybackMedia, PowerSource, PriorityKeySettings,
    RAWFileCompressionType, RecorderStatus, RecordingMedia, RecordingMediaMovie, RecordingState,
    RemoconZoomSpeedType, RightLeftEyeSelect, SdkControlMode, SelectFinder,
    ShutterReleaseTimeLagControl, ShutterType, SlotStatus, SoftSkinEffect,
    StillImageStoreDestination, StreamCipherType, StreamStatus,
    SubjectRecognitionAnimalBirdDetectionParts, SubjectRecognitionAnimalBirdPriority,
    TCUBDisplaySetting, TimeCodeFormat, TimeCodeMake, TimeCodePresetResetEnableStatus, TimeCodeRun,
    TimeShiftTriggerSetting, TouchOperation, WindNoiseReduction, ZoomOperation, APSC_S35,
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
    /// Gain unit setting (dB/ISO)
    GainUnitSetting(GainUnitSetting),
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
    /// Focus frame state
    FocusFrameState(FocusFrameState),
    /// Tracking frame type (Target AF/Non-Target AF)
    TrackingFrameType(TrackingFrameType),
    /// White balance setting (Auto/Daylight/Tungsten/etc.)
    WhiteBalance(WhiteBalance),
    /// Priority setting in AWB (Standard/Ambience/White)
    PrioritySetInAWB(PrioritySetInAWB),
    /// White balance switch (Preset/Memory A/Memory B)
    WhiteBalanceSwitch(WhiteBalanceSwitch),
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
    /// Movie shooting mode (Off/Cine EI/Custom/etc.)
    MovieShootingMode(MovieShootingMode),
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
    /// Monitoring output format
    MonitoringOutputFormat(MonitoringOutputFormat),
    /// Streaming status
    StreamStatus(StreamStatus),
    /// Stream encryption cipher type
    StreamCipherType(StreamCipherType),
    /// Video stream codec
    VideoStreamCodec(VideoStreamCodec),
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
    /// Auto power-off temperature threshold
    AutoPowerOffTemperature(AutoPowerOffTemperature),
    /// Camera power status
    CameraPowerStatus(CameraPowerStatus),
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
    /// Audio stream bit depth
    AudioStreamBitDepth(AudioStreamBitDepth),
    /// Audio stream channel count
    AudioStreamChannel(AudioStreamChannel),
    /// Audio input channel source selection
    AudioInputCHInputSelect(AudioInputCHInputSelect),
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
    /// Image stabilization for movie
    ImageStabilizationSteadyShotMovie(ImageStabilizationSteadyShotMovie),
    /// Lens shading compensation
    LensCompensationShading(LensCompensationShading),
    /// Custom white balance size
    CustomWBSizeSetting(CustomWBSizeSetting),
    /// Aperture drive in AF
    ApertureDriveInAF(ApertureDriveInAF),
    /// Recording media (still)
    RecordingMedia(RecordingMedia),
    /// Recording media (movie)
    RecordingMediaMovie(RecordingMediaMovie),
    /// E-framing production effect
    EframingProductionEffect(EframingProductionEffect),
    /// AF tracking responsiveness
    AFTrackForSpeedChange(AFTrackForSpeedChange),
    /// AF tracking sensitivity
    AFTrackingSensitivity(AFTrackingSensitivity),
    /// Timecode/userbit display
    TCUBDisplaySetting(TCUBDisplaySetting),
    /// Subject recognition animal/bird priority
    SubjectRecognitionAnimalBirdPriority(SubjectRecognitionAnimalBirdPriority),
    /// Subject recognition detection parts
    SubjectRecognitionAnimalBirdDetectionParts(SubjectRecognitionAnimalBirdDetectionParts),
    /// Camera operating mode
    CameraOperatingMode(CameraOperatingMode),
    /// Iris display unit
    IrisDisplayUnit(IrisDisplayUnit),
    /// Image stabilization level for movie
    ImageStabilizationLevelMovie(ImageStabilizationLevelMovie),
    /// Shutter release time lag control
    ShutterReleaseTimeLagControl(ShutterReleaseTimeLagControl),
    /// TimeShift trigger setting
    TimeShiftTriggerSetting(TimeShiftTriggerSetting),
    /// APS-C/S35 crop mode
    APSC_S35(APSC_S35),
    /// Right/Left eye select for AF
    RightLeftEyeSelect(RightLeftEyeSelect),
    /// Gain base sensitivity
    GainBaseSensitivity(GainBaseSensitivity),
    /// FTP connection status
    FTPConnectionStatus(FTPConnectionStatus),
    /// Zoom operation direction
    ZoomOperation(ZoomOperation),
    /// RAW file compression type
    RAWFileCompressionType(RAWFileCompressionType),
    /// Compression file format
    CompressionFileFormat(CompressionFileFormat),
    /// Zoom speed type
    RemoconZoomSpeedType(RemoconZoomSpeedType),
    /// Zoom type status (optical/smart/clear image/digital)
    ZoomTypeStatus(ZoomTypeStatus),
    /// Zoom motor driving status
    ZoomDrivingStatus(ZoomDrivingStatus),
    /// ND filter mode
    NDFilterMode(NDFilterMode),
    /// Creative Look style preset
    CreativeLook(CreativeLook),
    /// ISO auto minimum shutter speed preset
    IsoAutoMinShutterSpeedPreset(IsoAutoMinShutterSpeedPreset),
    /// ISO auto minimum shutter speed mode
    IsoAutoMinShutterSpeedMode(IsoAutoMinShutterSpeedMode),
    /// Picture effect filter
    PictureEffect(PictureEffect),
    /// Picture profile color mode
    PictureProfileColorMode(PictureProfileColorMode),
    /// Picture profile gamma curve
    PictureProfileGamma(PictureProfileGamma),
    /// Picture profile selection (PP1-PP11)
    PictureProfile(PictureProfile),
    /// Picture profile black gamma range
    PictureProfileBlackGammaRange(PictureProfileBlackGammaRange),
    /// Grid line overlay type
    GridLineType(GridLineType),
    /// Face frame type
    FaceFrameType(FaceFrameType),
    /// Focus frame type
    FocusFrameType(FocusFrameType),
    /// Frame info type
    FrameInfoType(FrameInfoType),
    /// Imager/sensor scan mode
    ImagerScanMode(ImagerScanMode),
    /// Auto-framing/E-framing type
    EframingType(EframingType),
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
    /// Live view protocol
    LiveViewProtocol(LiveViewProtocol),
    /// Silent mode aperture drive setting
    SilentModeApertureDrive(SilentModeApertureDrive),

    // Focus status types
    /// Focus motor driving status
    FocusDrivingStatus(FocusDrivingStatus),
    /// Focus bracket shooting status
    FocusBracketShootingStatus(FocusBracketShootingStatus),
    /// Focus touch spot status
    FocusTouchSpotStatus(FocusTouchSpotStatus),
    /// Focus bracket order
    FocusBracketOrder(FocusBracketOrder),
    /// Push auto focus action
    PushAutoFocus(PushAutoFocus),
    /// Focus operation 16-bit enable status
    FocusOperationWithInt16EnableStatus(FocusOperationWithInt16EnableStatus),

    // Movie format types
    /// Movie shooting mode color gamut
    MovieShootingModeColorGamut(MovieShootingModeColorGamut),
    /// Movie shooting mode target display
    MovieShootingModeTargetDisplay(MovieShootingModeTargetDisplay),
    /// Recorder save destination
    RecorderSaveDestination(RecorderSaveDestination),
    /// Video recording format quality
    VideoRecordingFormatQuality(VideoRecordingFormatQuality),
    /// Recording folder format
    RecordingFolderFormat(RecordingFolderFormat),
    /// Movie playing state
    MoviePlayingState(MoviePlayingState),
    /// Movie recording review playing state
    MovieRecReviewPlayingState(MovieRecReviewPlayingState),
    /// Playback contents gamma type
    PlaybackContentsGammaType(PlaybackContentsGammaType),

    // Picture profile control types
    /// Picture profile detail adjustment mode
    PictureProfileDetailAdjustMode(PictureProfileDetailAdjustMode),
    /// Picture profile knee mode
    PictureProfileKneeMode(PictureProfileKneeMode),
    /// Picture profile knee auto-set sensitivity
    PictureProfileKneeAutoSetSensitivity(PictureProfileKneeAutoSetSensitivity),
    /// Picture profile reset enable status
    PictureProfileResetEnableStatus(PictureProfileResetEnableStatus),
    /// Creative look reset enable status
    CreativeLookResetEnableStatus(CreativeLookResetEnableStatus),
    /// Timecode preset reset enable status
    TimeCodePresetResetEnableStatus(TimeCodePresetResetEnableStatus),

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
            PVT::FocusFrameState => FocusFrameState::from_raw(raw)
                .map(TypedValue::FocusFrameState)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FocusFrameType => FocusFrameType::from_raw(raw)
                .map(TypedValue::FocusFrameType)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::TrackingFrameType => TrackingFrameType::from_raw(raw)
                .map(TypedValue::TrackingFrameType)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::WhiteBalance => WhiteBalance::from_raw(raw)
                .map(TypedValue::WhiteBalance)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::PrioritySetInAWB => PrioritySetInAWB::from_raw(raw)
                .map(TypedValue::PrioritySetInAWB)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::WhiteBalanceSwitch => WhiteBalanceSwitch::from_raw(raw)
                .map(TypedValue::WhiteBalanceSwitch)
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
            PVT::MovieShootingMode => MovieShootingMode::from_raw(raw)
                .map(TypedValue::MovieShootingMode)
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
            PVT::MonitoringOutputFormat => MonitoringOutputFormat::from_raw(raw)
                .map(TypedValue::MonitoringOutputFormat)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::StreamStatus => StreamStatus::from_raw(raw)
                .map(TypedValue::StreamStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::StreamCipherType => StreamCipherType::from_raw(raw)
                .map(TypedValue::StreamCipherType)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::VideoStreamCodec => VideoStreamCodec::from_raw(raw)
                .map(TypedValue::VideoStreamCodec)
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
            PVT::AutoPowerOffTemperature => AutoPowerOffTemperature::from_raw(raw)
                .map(TypedValue::AutoPowerOffTemperature)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::CameraPowerStatus => CameraPowerStatus::from_raw(raw)
                .map(TypedValue::CameraPowerStatus)
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
            PVT::AudioStreamBitDepth => AudioStreamBitDepth::from_raw(raw)
                .map(TypedValue::AudioStreamBitDepth)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::AudioStreamChannel => AudioStreamChannel::from_raw(raw)
                .map(TypedValue::AudioStreamChannel)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::AudioInputCHInputSelect => AudioInputCHInputSelect::from_raw(raw)
                .map(TypedValue::AudioInputCHInputSelect)
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
            PVT::ImageStabilizationSteadyShotMovie => {
                ImageStabilizationSteadyShotMovie::from_raw(raw)
                    .map(TypedValue::ImageStabilizationSteadyShotMovie)
                    .unwrap_or(TypedValue::Unknown(raw))
            }
            PVT::LensCompensationShading => LensCompensationShading::from_raw(raw)
                .map(TypedValue::LensCompensationShading)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::CustomWBSizeSetting => CustomWBSizeSetting::from_raw(raw)
                .map(TypedValue::CustomWBSizeSetting)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ApertureDriveInAF => ApertureDriveInAF::from_raw(raw)
                .map(TypedValue::ApertureDriveInAF)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::RecordingMedia => RecordingMedia::from_raw(raw)
                .map(TypedValue::RecordingMedia)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::RecordingMediaMovie => RecordingMediaMovie::from_raw(raw)
                .map(TypedValue::RecordingMediaMovie)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::EframingProductionEffect => EframingProductionEffect::from_raw(raw)
                .map(TypedValue::EframingProductionEffect)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::AFTrackForSpeedChange => AFTrackForSpeedChange::from_raw(raw)
                .map(TypedValue::AFTrackForSpeedChange)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::AFTrackingSensitivity => AFTrackingSensitivity::from_raw(raw)
                .map(TypedValue::AFTrackingSensitivity)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::TCUBDisplaySetting => TCUBDisplaySetting::from_raw(raw)
                .map(TypedValue::TCUBDisplaySetting)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::SubjectRecognitionAnimalBirdPriority => {
                SubjectRecognitionAnimalBirdPriority::from_raw(raw)
                    .map(TypedValue::SubjectRecognitionAnimalBirdPriority)
                    .unwrap_or(TypedValue::Unknown(raw))
            }
            PVT::SubjectRecognitionAnimalBirdDetectionParts => {
                SubjectRecognitionAnimalBirdDetectionParts::from_raw(raw)
                    .map(TypedValue::SubjectRecognitionAnimalBirdDetectionParts)
                    .unwrap_or(TypedValue::Unknown(raw))
            }
            PVT::CameraOperatingMode => CameraOperatingMode::from_raw(raw)
                .map(TypedValue::CameraOperatingMode)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::IrisDisplayUnit => IrisDisplayUnit::from_raw(raw)
                .map(TypedValue::IrisDisplayUnit)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ImageStabilizationLevelMovie => ImageStabilizationLevelMovie::from_raw(raw)
                .map(TypedValue::ImageStabilizationLevelMovie)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ShutterReleaseTimeLagControl => ShutterReleaseTimeLagControl::from_raw(raw)
                .map(TypedValue::ShutterReleaseTimeLagControl)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::TimeShiftTriggerSetting => TimeShiftTriggerSetting::from_raw(raw)
                .map(TypedValue::TimeShiftTriggerSetting)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::APSC_S35 => APSC_S35::from_raw(raw)
                .map(TypedValue::APSC_S35)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::RightLeftEyeSelect => RightLeftEyeSelect::from_raw(raw)
                .map(TypedValue::RightLeftEyeSelect)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::GainBaseSensitivity => GainBaseSensitivity::from_raw(raw)
                .map(TypedValue::GainBaseSensitivity)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FTPConnectionStatus => FTPConnectionStatus::from_raw(raw)
                .map(TypedValue::FTPConnectionStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ZoomOperation => ZoomOperation::from_raw(raw)
                .map(TypedValue::ZoomOperation)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ZoomTypeStatus => ZoomTypeStatus::from_raw(raw)
                .map(TypedValue::ZoomTypeStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ZoomDrivingStatus => ZoomDrivingStatus::from_raw(raw)
                .map(TypedValue::ZoomDrivingStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::RAWFileCompressionType => RAWFileCompressionType::from_raw(raw)
                .map(TypedValue::RAWFileCompressionType)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::CompressionFileFormat => CompressionFileFormat::from_raw(raw)
                .map(TypedValue::CompressionFileFormat)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::RemoconZoomSpeedType => RemoconZoomSpeedType::from_raw(raw)
                .map(TypedValue::RemoconZoomSpeedType)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::NDFilterMode => NDFilterMode::from_raw(raw)
                .map(TypedValue::NDFilterMode)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::CreativeLook => CreativeLook::from_raw(raw)
                .map(TypedValue::CreativeLook)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::IsoAutoMinShutterSpeedPreset => IsoAutoMinShutterSpeedPreset::from_raw(raw)
                .map(TypedValue::IsoAutoMinShutterSpeedPreset)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::IsoAutoMinShutterSpeedMode => IsoAutoMinShutterSpeedMode::from_raw(raw)
                .map(TypedValue::IsoAutoMinShutterSpeedMode)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::PictureEffect => PictureEffect::from_raw(raw)
                .map(TypedValue::PictureEffect)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::PictureProfileColorMode => PictureProfileColorMode::from_raw(raw)
                .map(TypedValue::PictureProfileColorMode)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::PictureProfileGamma => PictureProfileGamma::from_raw(raw)
                .map(TypedValue::PictureProfileGamma)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::PictureProfile => PictureProfile::from_raw(raw)
                .map(TypedValue::PictureProfile)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::PictureProfileBlackGammaRange => PictureProfileBlackGammaRange::from_raw(raw)
                .map(TypedValue::PictureProfileBlackGammaRange)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::GridLineType => GridLineType::from_raw(raw)
                .map(TypedValue::GridLineType)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::ImagerScanMode => ImagerScanMode::from_raw(raw)
                .map(TypedValue::ImagerScanMode)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::EframingType => EframingType::from_raw(raw)
                .map(TypedValue::EframingType)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FaceFrameType => FaceFrameType::from_raw(raw)
                .map(TypedValue::FaceFrameType)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FrameInfoType => FrameInfoType::from_raw(raw)
                .map(TypedValue::FrameInfoType)
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
            PVT::GainUnitSetting => GainUnitSetting::from_raw(raw)
                .map(TypedValue::GainUnitSetting)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::LiveViewDisplayEffect => LiveViewDisplayEffect::from_raw(raw)
                .map(TypedValue::LiveViewDisplayEffect)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::LiveViewProtocol => LiveViewProtocol::from_raw(raw)
                .map(TypedValue::LiveViewProtocol)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::SilentModeApertureDrive => SilentModeApertureDrive::from_raw(raw)
                .map(TypedValue::SilentModeApertureDrive)
                .unwrap_or(TypedValue::Unknown(raw)),
            // Focus status types
            PVT::FocusDrivingStatus => FocusDrivingStatus::from_raw(raw)
                .map(TypedValue::FocusDrivingStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FocusBracketShootingStatus => FocusBracketShootingStatus::from_raw(raw)
                .map(TypedValue::FocusBracketShootingStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FocusTouchSpotStatus => FocusTouchSpotStatus::from_raw(raw)
                .map(TypedValue::FocusTouchSpotStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FocusBracketOrder => FocusBracketOrder::from_raw(raw)
                .map(TypedValue::FocusBracketOrder)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::PushAutoFocus => PushAutoFocus::from_raw(raw)
                .map(TypedValue::PushAutoFocus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::FocusOperationWithInt16EnableStatus => {
                FocusOperationWithInt16EnableStatus::from_raw(raw)
                    .map(TypedValue::FocusOperationWithInt16EnableStatus)
                    .unwrap_or(TypedValue::Unknown(raw))
            }
            // Movie format types
            PVT::MovieShootingModeColorGamut => MovieShootingModeColorGamut::from_raw(raw)
                .map(TypedValue::MovieShootingModeColorGamut)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::MovieShootingModeTargetDisplay => MovieShootingModeTargetDisplay::from_raw(raw)
                .map(TypedValue::MovieShootingModeTargetDisplay)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::RecorderSaveDestination => RecorderSaveDestination::from_raw(raw)
                .map(TypedValue::RecorderSaveDestination)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::VideoRecordingFormatQuality => VideoRecordingFormatQuality::from_raw(raw)
                .map(TypedValue::VideoRecordingFormatQuality)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::RecordingFolderFormat => RecordingFolderFormat::from_raw(raw)
                .map(TypedValue::RecordingFolderFormat)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::MoviePlayingState => MoviePlayingState::from_raw(raw)
                .map(TypedValue::MoviePlayingState)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::MovieRecReviewPlayingState => MovieRecReviewPlayingState::from_raw(raw)
                .map(TypedValue::MovieRecReviewPlayingState)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::PlaybackContentsGammaType => PlaybackContentsGammaType::from_raw(raw)
                .map(TypedValue::PlaybackContentsGammaType)
                .unwrap_or(TypedValue::Unknown(raw)),
            // Picture profile control types
            PVT::PictureProfileDetailAdjustMode => PictureProfileDetailAdjustMode::from_raw(raw)
                .map(TypedValue::PictureProfileDetailAdjustMode)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::PictureProfileKneeMode => PictureProfileKneeMode::from_raw(raw)
                .map(TypedValue::PictureProfileKneeMode)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::PictureProfileKneeAutoSetSensitivity => {
                PictureProfileKneeAutoSetSensitivity::from_raw(raw)
                    .map(TypedValue::PictureProfileKneeAutoSetSensitivity)
                    .unwrap_or(TypedValue::Unknown(raw))
            }
            PVT::PictureProfileResetEnableStatus => PictureProfileResetEnableStatus::from_raw(raw)
                .map(TypedValue::PictureProfileResetEnableStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::CreativeLookResetEnableStatus => CreativeLookResetEnableStatus::from_raw(raw)
                .map(TypedValue::CreativeLookResetEnableStatus)
                .unwrap_or(TypedValue::Unknown(raw)),
            PVT::TimeCodePresetResetEnableStatus => TimeCodePresetResetEnableStatus::from_raw(raw)
                .map(TypedValue::TimeCodePresetResetEnableStatus)
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
            TypedValue::GainUnitSetting(v) => v.to_raw(),
            TypedValue::FocusMode(v) => v.to_raw(),
            TypedValue::FocusArea(v) => v.to_raw(),
            TypedValue::SubjectRecognitionAF(v) => v.to_raw(),
            TypedValue::PrioritySetInAF(v) => v.to_raw(),
            TypedValue::FocusTrackingStatus(v) => v.to_raw(),
            TypedValue::FocusIndicator(v) => v.to_raw(),
            TypedValue::FocusFrameState(v) => v.to_raw(),
            TypedValue::FocusFrameType(v) => v.to_raw(),
            TypedValue::TrackingFrameType(v) => v.to_raw(),
            TypedValue::WhiteBalance(v) => v.to_raw(),
            TypedValue::PrioritySetInAWB(v) => v.to_raw(),
            TypedValue::WhiteBalanceSwitch(v) => v.to_raw(),
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
            TypedValue::MovieShootingMode(v) => v.to_raw(),
            TypedValue::RecordingState(v) => v.to_raw(),
            TypedValue::RecorderStatus(v) => v.to_raw(),
            TypedValue::TimeCodeFormat(v) => v.to_raw(),
            TypedValue::TimeCodeRun(v) => v.to_raw(),
            TypedValue::TimeCodeMake(v) => v.to_raw(),
            TypedValue::LiveViewStatus(v) => v.to_raw(),
            TypedValue::SlotStatus(v) => v.to_raw(),
            TypedValue::MediaSlotWritingState(v) => v.to_raw(),
            TypedValue::MediaSlotRecordingType(v) => v.to_raw(),
            TypedValue::MonitoringOutputFormat(v) => v.to_raw(),
            TypedValue::StreamStatus(v) => v.to_raw(),
            TypedValue::StreamCipherType(v) => v.to_raw(),
            TypedValue::VideoStreamCodec(v) => v.to_raw(),
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
            TypedValue::AutoPowerOffTemperature(v) => v.to_raw(),
            TypedValue::CameraPowerStatus(v) => v.to_raw(),
            TypedValue::FocusOperation(v) => v.to_raw(),
            TypedValue::ShutterType(v) => v.to_raw(),
            TypedValue::DeviceOverheatingState(v) => v.to_raw(),
            TypedValue::HighIsoNR(v) => v.to_raw(),
            TypedValue::AudioSignals(v) => v.to_raw(),
            TypedValue::AudioStreamBitDepth(v) => v.to_raw(),
            TypedValue::AudioStreamChannel(v) => v.to_raw(),
            TypedValue::AudioInputCHInputSelect(v) => v.to_raw(),
            TypedValue::FunctionOfTouchOperation(v) => v.to_raw(),
            TypedValue::SoftSkinEffect(v) => v.to_raw(),
            TypedValue::WindNoiseReduction(v) => v.to_raw(),
            TypedValue::SelectFinder(v) => v.to_raw(),
            TypedValue::DispMode(v) => v.to_raw(),
            TypedValue::ImageStabilizationSteadyShotMovie(v) => v.to_raw(),
            TypedValue::LensCompensationShading(v) => v.to_raw(),
            TypedValue::CustomWBSizeSetting(v) => v.to_raw(),
            TypedValue::ApertureDriveInAF(v) => v.to_raw(),
            TypedValue::RecordingMedia(v) => v.to_raw(),
            TypedValue::RecordingMediaMovie(v) => v.to_raw(),
            TypedValue::EframingProductionEffect(v) => v.to_raw(),
            TypedValue::AFTrackForSpeedChange(v) => v.to_raw(),
            TypedValue::AFTrackingSensitivity(v) => v.to_raw(),
            TypedValue::TCUBDisplaySetting(v) => v.to_raw(),
            TypedValue::SubjectRecognitionAnimalBirdPriority(v) => v.to_raw(),
            TypedValue::SubjectRecognitionAnimalBirdDetectionParts(v) => v.to_raw(),
            TypedValue::CameraOperatingMode(v) => v.to_raw(),
            TypedValue::IrisDisplayUnit(v) => v.to_raw(),
            TypedValue::ImageStabilizationLevelMovie(v) => v.to_raw(),
            TypedValue::ShutterReleaseTimeLagControl(v) => v.to_raw(),
            TypedValue::TimeShiftTriggerSetting(v) => v.to_raw(),
            TypedValue::APSC_S35(v) => v.to_raw(),
            TypedValue::RightLeftEyeSelect(v) => v.to_raw(),
            TypedValue::GainBaseSensitivity(v) => v.to_raw(),
            TypedValue::FTPConnectionStatus(v) => v.to_raw(),
            TypedValue::ZoomOperation(v) => v.to_raw(),
            TypedValue::ZoomTypeStatus(v) => v.to_raw(),
            TypedValue::ZoomDrivingStatus(v) => v.to_raw(),
            TypedValue::RAWFileCompressionType(v) => v.to_raw(),
            TypedValue::CompressionFileFormat(v) => v.to_raw(),
            TypedValue::RemoconZoomSpeedType(v) => v.to_raw(),
            TypedValue::NDFilterMode(v) => v.to_raw(),
            TypedValue::CreativeLook(v) => v.to_raw(),
            TypedValue::IsoAutoMinShutterSpeedPreset(v) => v.to_raw(),
            TypedValue::IsoAutoMinShutterSpeedMode(v) => v.to_raw(),
            TypedValue::PictureEffect(v) => v.to_raw(),
            TypedValue::PictureProfileColorMode(v) => v.to_raw(),
            TypedValue::PictureProfileGamma(v) => v.to_raw(),
            TypedValue::PictureProfile(v) => v.to_raw(),
            TypedValue::PictureProfileBlackGammaRange(v) => v.to_raw(),
            TypedValue::GridLineType(v) => v.to_raw(),
            TypedValue::FaceFrameType(v) => v.to_raw(),
            TypedValue::FrameInfoType(v) => v.to_raw(),
            TypedValue::ImagerScanMode(v) => v.to_raw(),
            TypedValue::EframingType(v) => v.to_raw(),
            TypedValue::BatteryLevel(v) => v.to_raw(),
            TypedValue::Switch(v) => v.to_raw(),
            TypedValue::OnOff(v) => v.to_raw(),
            TypedValue::AutoManual(v) => v.to_raw(),
            TypedValue::LockIndicator(v) => v.to_raw(),
            TypedValue::LiveViewDisplayEffect(v) => v.to_raw(),
            TypedValue::LiveViewProtocol(v) => v.to_raw(),
            TypedValue::SilentModeApertureDrive(v) => v.to_raw(),
            // Focus status types
            TypedValue::FocusDrivingStatus(v) => v.to_raw(),
            TypedValue::FocusBracketShootingStatus(v) => v.to_raw(),
            TypedValue::FocusTouchSpotStatus(v) => v.to_raw(),
            TypedValue::FocusBracketOrder(v) => v.to_raw(),
            TypedValue::PushAutoFocus(v) => v.to_raw(),
            TypedValue::FocusOperationWithInt16EnableStatus(v) => v.to_raw(),
            // Movie format types
            TypedValue::MovieShootingModeColorGamut(v) => v.to_raw(),
            TypedValue::MovieShootingModeTargetDisplay(v) => v.to_raw(),
            TypedValue::RecorderSaveDestination(v) => v.to_raw(),
            TypedValue::VideoRecordingFormatQuality(v) => v.to_raw(),
            TypedValue::RecordingFolderFormat(v) => v.to_raw(),
            TypedValue::MoviePlayingState(v) => v.to_raw(),
            TypedValue::MovieRecReviewPlayingState(v) => v.to_raw(),
            TypedValue::PlaybackContentsGammaType(v) => v.to_raw(),
            // Picture profile control types
            TypedValue::PictureProfileDetailAdjustMode(v) => v.to_raw(),
            TypedValue::PictureProfileKneeMode(v) => v.to_raw(),
            TypedValue::PictureProfileKneeAutoSetSensitivity(v) => v.to_raw(),
            TypedValue::PictureProfileResetEnableStatus(v) => v.to_raw(),
            TypedValue::CreativeLookResetEnableStatus(v) => v.to_raw(),
            TypedValue::TimeCodePresetResetEnableStatus(v) => v.to_raw(),
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
            TypedValue::GainUnitSetting(v) => write!(f, "{}", v),
            TypedValue::FocusMode(v) => write!(f, "{}", v),
            TypedValue::FocusArea(v) => write!(f, "{}", v),
            TypedValue::SubjectRecognitionAF(v) => write!(f, "{}", v),
            TypedValue::PrioritySetInAF(v) => write!(f, "{}", v),
            TypedValue::FocusTrackingStatus(v) => write!(f, "{}", v),
            TypedValue::FocusIndicator(v) => write!(f, "{}", v),
            TypedValue::FocusFrameState(v) => write!(f, "{}", v),
            TypedValue::FocusFrameType(v) => write!(f, "{}", v),
            TypedValue::TrackingFrameType(v) => write!(f, "{}", v),
            TypedValue::WhiteBalance(v) => write!(f, "{}", v),
            TypedValue::PrioritySetInAWB(v) => write!(f, "{}", v),
            TypedValue::WhiteBalanceSwitch(v) => write!(f, "{}", v),
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
            TypedValue::MovieShootingMode(v) => write!(f, "{}", v),
            TypedValue::RecordingState(v) => write!(f, "{}", v),
            TypedValue::RecorderStatus(v) => write!(f, "{}", v),
            TypedValue::TimeCodeFormat(v) => write!(f, "{}", v),
            TypedValue::TimeCodeRun(v) => write!(f, "{}", v),
            TypedValue::TimeCodeMake(v) => write!(f, "{}", v),
            TypedValue::LiveViewStatus(v) => write!(f, "{}", v),
            TypedValue::SlotStatus(v) => write!(f, "{}", v),
            TypedValue::MediaSlotWritingState(v) => write!(f, "{}", v),
            TypedValue::MediaSlotRecordingType(v) => write!(f, "{}", v),
            TypedValue::MonitoringOutputFormat(v) => write!(f, "{}", v),
            TypedValue::StreamStatus(v) => write!(f, "{}", v),
            TypedValue::StreamCipherType(v) => write!(f, "{}", v),
            TypedValue::VideoStreamCodec(v) => write!(f, "{}", v),
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
            TypedValue::AutoPowerOffTemperature(v) => write!(f, "{}", v),
            TypedValue::CameraPowerStatus(v) => write!(f, "{}", v),
            TypedValue::FocusOperation(v) => write!(f, "{}", v),
            TypedValue::ShutterType(v) => write!(f, "{}", v),
            TypedValue::DeviceOverheatingState(v) => write!(f, "{}", v),
            TypedValue::HighIsoNR(v) => write!(f, "{}", v),
            TypedValue::AudioSignals(v) => write!(f, "{}", v),
            TypedValue::AudioStreamBitDepth(v) => write!(f, "{}", v),
            TypedValue::AudioStreamChannel(v) => write!(f, "{}", v),
            TypedValue::AudioInputCHInputSelect(v) => write!(f, "{}", v),
            TypedValue::FunctionOfTouchOperation(v) => write!(f, "{}", v),
            TypedValue::SoftSkinEffect(v) => write!(f, "{}", v),
            TypedValue::WindNoiseReduction(v) => write!(f, "{}", v),
            TypedValue::SelectFinder(v) => write!(f, "{}", v),
            TypedValue::DispMode(v) => write!(f, "{}", v),
            TypedValue::ImageStabilizationSteadyShotMovie(v) => write!(f, "{}", v),
            TypedValue::LensCompensationShading(v) => write!(f, "{}", v),
            TypedValue::CustomWBSizeSetting(v) => write!(f, "{}", v),
            TypedValue::ApertureDriveInAF(v) => write!(f, "{}", v),
            TypedValue::RecordingMedia(v) => write!(f, "{}", v),
            TypedValue::RecordingMediaMovie(v) => write!(f, "{}", v),
            TypedValue::EframingProductionEffect(v) => write!(f, "{}", v),
            TypedValue::AFTrackForSpeedChange(v) => write!(f, "{}", v),
            TypedValue::AFTrackingSensitivity(v) => write!(f, "{}", v),
            TypedValue::TCUBDisplaySetting(v) => write!(f, "{}", v),
            TypedValue::SubjectRecognitionAnimalBirdPriority(v) => write!(f, "{}", v),
            TypedValue::SubjectRecognitionAnimalBirdDetectionParts(v) => write!(f, "{}", v),
            TypedValue::CameraOperatingMode(v) => write!(f, "{}", v),
            TypedValue::IrisDisplayUnit(v) => write!(f, "{}", v),
            TypedValue::ImageStabilizationLevelMovie(v) => write!(f, "{}", v),
            TypedValue::ShutterReleaseTimeLagControl(v) => write!(f, "{}", v),
            TypedValue::TimeShiftTriggerSetting(v) => write!(f, "{}", v),
            TypedValue::APSC_S35(v) => write!(f, "{}", v),
            TypedValue::RightLeftEyeSelect(v) => write!(f, "{}", v),
            TypedValue::GainBaseSensitivity(v) => write!(f, "{}", v),
            TypedValue::FTPConnectionStatus(v) => write!(f, "{}", v),
            TypedValue::ZoomOperation(v) => write!(f, "{}", v),
            TypedValue::ZoomTypeStatus(v) => write!(f, "{}", v),
            TypedValue::ZoomDrivingStatus(v) => write!(f, "{}", v),
            TypedValue::RAWFileCompressionType(v) => write!(f, "{}", v),
            TypedValue::CompressionFileFormat(v) => write!(f, "{}", v),
            TypedValue::RemoconZoomSpeedType(v) => write!(f, "{}", v),
            TypedValue::NDFilterMode(v) => write!(f, "{}", v),
            TypedValue::CreativeLook(v) => write!(f, "{}", v),
            TypedValue::IsoAutoMinShutterSpeedPreset(v) => write!(f, "{}", v),
            TypedValue::IsoAutoMinShutterSpeedMode(v) => write!(f, "{}", v),
            TypedValue::PictureEffect(v) => write!(f, "{}", v),
            TypedValue::PictureProfileColorMode(v) => write!(f, "{}", v),
            TypedValue::PictureProfileGamma(v) => write!(f, "{}", v),
            TypedValue::PictureProfile(v) => write!(f, "{}", v),
            TypedValue::PictureProfileBlackGammaRange(v) => write!(f, "{}", v),
            TypedValue::GridLineType(v) => write!(f, "{}", v),
            TypedValue::ImagerScanMode(v) => write!(f, "{}", v),
            TypedValue::EframingType(v) => write!(f, "{}", v),
            TypedValue::FaceFrameType(v) => write!(f, "{}", v),
            TypedValue::FrameInfoType(v) => write!(f, "{}", v),
            TypedValue::BatteryLevel(v) => write!(f, "{}", v),
            TypedValue::Switch(v) => write!(f, "{}", v),
            TypedValue::OnOff(v) => write!(f, "{}", v),
            TypedValue::AutoManual(v) => write!(f, "{}", v),
            TypedValue::LockIndicator(v) => write!(f, "{}", v),
            TypedValue::LiveViewDisplayEffect(v) => write!(f, "{}", v),
            TypedValue::LiveViewProtocol(v) => write!(f, "{}", v),
            TypedValue::SilentModeApertureDrive(v) => write!(f, "{}", v),
            // Focus status types
            TypedValue::FocusDrivingStatus(v) => write!(f, "{}", v),
            TypedValue::FocusBracketShootingStatus(v) => write!(f, "{}", v),
            TypedValue::FocusTouchSpotStatus(v) => write!(f, "{}", v),
            TypedValue::FocusBracketOrder(v) => write!(f, "{}", v),
            TypedValue::PushAutoFocus(v) => write!(f, "{}", v),
            TypedValue::FocusOperationWithInt16EnableStatus(v) => write!(f, "{}", v),
            // Movie format types
            TypedValue::MovieShootingModeColorGamut(v) => write!(f, "{}", v),
            TypedValue::MovieShootingModeTargetDisplay(v) => write!(f, "{}", v),
            TypedValue::RecorderSaveDestination(v) => write!(f, "{}", v),
            TypedValue::VideoRecordingFormatQuality(v) => write!(f, "{}", v),
            TypedValue::RecordingFolderFormat(v) => write!(f, "{}", v),
            TypedValue::MoviePlayingState(v) => write!(f, "{}", v),
            TypedValue::MovieRecReviewPlayingState(v) => write!(f, "{}", v),
            TypedValue::PlaybackContentsGammaType(v) => write!(f, "{}", v),
            // Picture profile control types
            TypedValue::PictureProfileDetailAdjustMode(v) => write!(f, "{}", v),
            TypedValue::PictureProfileKneeMode(v) => write!(f, "{}", v),
            TypedValue::PictureProfileKneeAutoSetSensitivity(v) => write!(f, "{}", v),
            TypedValue::PictureProfileResetEnableStatus(v) => write!(f, "{}", v),
            TypedValue::CreativeLookResetEnableStatus(v) => write!(f, "{}", v),
            TypedValue::TimeCodePresetResetEnableStatus(v) => write!(f, "{}", v),
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
