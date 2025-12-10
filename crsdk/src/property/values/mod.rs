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
    AFTrackForSpeedChange, ApertureDriveInAF, AudioSignals, AutoManual, BatteryRemainDisplayUnit,
    ColorSpace, CustomWBSizeSetting, DRangeOptimizer, DeviceOverheatingState, DispMode,
    FocusOperation, FunctionOfTouchOperation, HighIsoNR, ImageStabilizationSteadyShotMovie,
    Integer, IntervalRecMode, IntervalRecStatus, LensCompensationShading, LiveViewDisplayEffect,
    LockIndicator, NearFarEnableStatus, OnOff, Percentage, PlaybackMedia, PowerSource,
    PriorityKeySettings, PropertyValueType, RecordingMedia, RecordingMediaMovie, SdkControlMode,
    SelectFinder, ShutterType, SilentModeApertureDrive, SoftSkinEffect, StillImageStoreDestination,
    SubjectRecognitionAnimalBirdDetectionParts, SubjectRecognitionAnimalBirdPriority, Switch,
    TCUBDisplaySetting, TouchOperation, WindNoiseReduction,
};
pub use drive::{DriveMode, IntervalRecShutterType};
pub use exposure::{
    Aperture, ExposureComp, ExposureCtrlType, ExposureProgram, Iso, MeterLevel, MeteringMode,
    ShutterMode, ShutterModeStatus, ShutterSpeed,
};
pub use flash::FlashMode;
pub use focus::{
    FocusArea, FocusIndicator, FocusMode, FocusTrackingStatus, PrioritySetInAF,
    SubjectRecognitionAF,
};
pub use image::{AspectRatio, FileType, ImageQuality, ImageSize};
pub use media::{LiveViewStatus, MediaSlotRecordingType, MediaSlotWritingState, SlotStatus};
pub use movie::{
    MovieFileFormat, MovieQuality, RecorderStatus, RecordingState, TimeCodeFormat, TimeCodeMake,
    TimeCodeRun,
};
pub use power::BatteryLevel;
pub use white_balance::{ColorTemperature, PrioritySetInAWB, WhiteBalance};
