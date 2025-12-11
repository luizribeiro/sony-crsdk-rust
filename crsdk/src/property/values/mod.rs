//! Property value types.
//!
//! This module contains all typed representations of camera property values.
//! Each type implements the [`PropertyValue`](super::PropertyValue) trait,
//! providing consistent conversion between raw SDK values and Rust types.
//!
//! Types are organized by domain:
//! - [`common`] - Generic toggles and simple types (Switch, OnOff, Integer, etc.)
//! - [`exposure`] - Exposure-related types (Aperture, ShutterSpeed, Iso, etc.)
//! - [`focus`] - Focus-related types (FocusMode, FocusArea, etc.)
//! - [`white_balance`] - White balance types
//! - [`drive`] - Drive mode types
//! - [`flash`] - Flash mode types
//! - [`image`] - Image format and quality types
//! - [`movie`] - Movie format and quality types
//! - [`power`] - Battery and power types

mod common;
mod drive;
mod exposure;
mod flash;
mod focus;
mod image;
mod media;
mod movie;
mod power;
mod white_balance;

pub use common::{
    AFTrackForSpeedChange, AFTrackingSensitivity, ApertureDriveInAF, AudioInputCHInputSelect,
    AudioSignals, AudioStreamBitDepth, AudioStreamChannel, AutoManual, AutoPowerOffTemperature,
    BatteryRemainDisplayUnit, CameraOperatingMode, ColorSpace, CompressionFileFormat, CreativeLook,
    CreativeLookResetEnableStatus, CustomWBSizeSetting, DRangeOptimizer, DeviceOverheatingState,
    DispMode, EframingProductionEffect, EframingType, FTPConnectionStatus, FaceFrameType,
    FocusFrameType, FocusOperation, FocusOperationWithInt16EnableStatus, FrameInfoType,
    FunctionOfTouchOperation, GainBaseSensitivity, GridLineType, HighIsoNR,
    ImageStabilizationLevelMovie, ImageStabilizationSteadyShotMovie, ImagerScanMode, Integer,
    IntervalRecMode, IntervalRecStatus, IrisDisplayUnit, IsoAutoMinShutterSpeedMode,
    IsoAutoMinShutterSpeedPreset, LensCompensationShading, LiveViewDisplayEffect, LiveViewProtocol,
    LockIndicator, MonitoringOutputFormat, NDFilterMode, NearFarEnableStatus, OnOff, Percentage,
    PictureEffect, PictureProfile, PictureProfileBlackGammaRange, PictureProfileColorMode,
    PictureProfileDetailAdjustMode, PictureProfileGamma, PictureProfileKneeAutoSetSensitivity,
    PictureProfileKneeMode, PictureProfileResetEnableStatus, PlaybackMedia, PowerSource,
    PriorityKeySettings, PropertyValueType, RAWFileCompressionType, RecordingMedia,
    RecordingMediaMovie, RemoconZoomSpeedType, RightLeftEyeSelect, SdkControlMode, SelectFinder,
    ShutterReleaseTimeLagControl, ShutterType, SilentModeApertureDrive, SoftSkinEffect,
    StillImageStoreDestination, StreamCipherType, StreamStatus,
    SubjectRecognitionAnimalBirdDetectionParts, SubjectRecognitionAnimalBirdPriority, Switch,
    TCUBDisplaySetting, TimeCodePresetResetEnableStatus, TimeShiftTriggerSetting, TouchOperation,
    VideoStreamCodec, WindNoiseReduction, ZoomDrivingStatus, ZoomOperation, ZoomTypeStatus,
    APSC_S35,
};
pub use drive::{DriveMode, IntervalRecShutterType};
pub use exposure::{
    Aperture, ExposureComp, ExposureCtrlType, ExposureProgram, GainUnitSetting, Iso, MeterLevel,
    MeteringMode, ShutterMode, ShutterModeStatus, ShutterSpeed,
};
pub use flash::FlashMode;
pub use focus::{
    FocusArea, FocusBracketOrder, FocusBracketShootingStatus, FocusDrivingStatus, FocusFrameState,
    FocusIndicator, FocusMode, FocusTouchSpotStatus, FocusTrackingStatus, PrioritySetInAF,
    PushAutoFocus, SubjectRecognitionAF, TrackingFrameType,
};
pub use image::{AspectRatio, FileType, ImageQuality, ImageSize};
pub use media::{LiveViewStatus, MediaSlotRecordingType, MediaSlotWritingState, SlotStatus};
pub use movie::{
    MovieFileFormat, MoviePlayingState, MovieQuality, MovieRecReviewPlayingState,
    MovieShootingMode, MovieShootingModeColorGamut, MovieShootingModeTargetDisplay,
    PlaybackContentsGammaType, RecorderSaveDestination, RecorderStatus, RecordingFolderFormat,
    RecordingState, TimeCodeFormat, TimeCodeMake, TimeCodeRun, VideoRecordingFormatQuality,
};
pub use power::{BatteryLevel, CameraPowerStatus};
pub use white_balance::{ColorTemperature, PrioritySetInAWB, WhiteBalance, WhiteBalanceSwitch};
