//! Property metadata: descriptions, display names, and value type mappings.
//!
//! This module contains human-readable metadata for camera properties:
//! - [`description`] - Detailed explanation of what the property does
//! - [`display_name`] - Short human-readable name for UI display
//! - [`value_type`] - Which value type this property uses

pub mod audio;
pub mod display;
pub mod drive;
pub mod exposure;
pub mod flash;
pub mod focus;
pub mod image;
pub mod lens;
pub mod media;
pub mod movie;
pub mod nd_filter;
pub mod other;
pub mod picture_profile;
pub mod power;
pub mod silent;
pub mod stabilization;
pub mod white_balance;
pub mod zoom;

use super::category::{property_category, PropertyCategory};
use super::values::PropertyValueType;
use crsdk_sys::DevicePropertyCode;

/// Get the value type for a property code.
///
/// This determines how to format and parse the raw SDK value for display.
/// Checks all subsystem modules for type mappings, with fallback to common types.
pub fn value_type(code: DevicePropertyCode) -> PropertyValueType {
    // Try subsystem-specific value types first
    if let Some(vt) = exposure::value_type(code) {
        return vt;
    }
    if let Some(vt) = focus::value_type(code) {
        return vt;
    }
    if let Some(vt) = white_balance::value_type(code) {
        return vt;
    }
    if let Some(vt) = drive::value_type(code) {
        return vt;
    }
    if let Some(vt) = flash::value_type(code) {
        return vt;
    }
    if let Some(vt) = image::value_type(code) {
        return vt;
    }
    if let Some(vt) = movie::value_type(code) {
        return vt;
    }

    // Fallback to common value types for categories without dedicated modules
    common_value_type(code)
}

fn common_value_type(code: DevicePropertyCode) -> PropertyValueType {
    match code {
        // Power
        DevicePropertyCode::BatteryRemain | DevicePropertyCode::BatteryLevel => {
            PropertyValueType::Percentage
        }

        // Display
        DevicePropertyCode::LiveViewDisplayEffect => PropertyValueType::LiveViewDisplayEffect,
        DevicePropertyCode::GridLineDisplay => PropertyValueType::Switch,
        DevicePropertyCode::GridLineType
        | DevicePropertyCode::LiveViewStatus
        | DevicePropertyCode::LiveViewImageQuality
        | DevicePropertyCode::LiveViewProtocol
        | DevicePropertyCode::DispMode
        | DevicePropertyCode::DispModeSetting
        | DevicePropertyCode::DispModeCandidate
        | DevicePropertyCode::MonitorLUTSetting
        | DevicePropertyCode::MonitorBrightnessType
        | DevicePropertyCode::MonitorBrightnessManual
        | DevicePropertyCode::SelectFinder
        | DevicePropertyCode::DisplayQualityFinder
        | DevicePropertyCode::DisplayedMenuStatus
        | DevicePropertyCode::GammaDisplayAssist
        | DevicePropertyCode::GammaDisplayAssistType
        | DevicePropertyCode::PlaybackContentsGammaType => PropertyValueType::Integer,

        // Stabilization
        DevicePropertyCode::ImageStabilizationSteadyShot
        | DevicePropertyCode::ImageStabilizationSteadyShotAdjust
        | DevicePropertyCode::ImageStabilizationSteadyShotFocalLength
        | DevicePropertyCode::ImageStabilizationFramingStabilizer => PropertyValueType::Integer,

        // Audio
        DevicePropertyCode::AudioRecording => PropertyValueType::OnOff,
        DevicePropertyCode::AudioSignals => PropertyValueType::OnOff,
        DevicePropertyCode::AudioSignalsStartEnd => PropertyValueType::OnOff,
        DevicePropertyCode::AudioSignalsVolume
        | DevicePropertyCode::AudioLevelDisplay
        | DevicePropertyCode::AudioInput1TypeSelect
        | DevicePropertyCode::AudioInput2TypeSelect
        | DevicePropertyCode::AudioInputCH1InputSelect
        | DevicePropertyCode::AudioInputCH2InputSelect
        | DevicePropertyCode::AudioInputCH3InputSelect
        | DevicePropertyCode::AudioInputCH4InputSelect
        | DevicePropertyCode::AudioInputCH1Level
        | DevicePropertyCode::AudioInputCH2Level
        | DevicePropertyCode::AudioInputCH3Level
        | DevicePropertyCode::AudioInputCH4Level
        | DevicePropertyCode::AudioInputMasterLevel
        | DevicePropertyCode::AudioInputCH1LevelControl
        | DevicePropertyCode::AudioInputCH2LevelControl
        | DevicePropertyCode::AudioInputCH3LevelControl
        | DevicePropertyCode::AudioInputCH4LevelControl
        | DevicePropertyCode::AudioStreamBitDepth
        | DevicePropertyCode::AudioStreamChannel
        | DevicePropertyCode::AudioStreamCodecType
        | DevicePropertyCode::AudioStreamSamplingFrequency => PropertyValueType::Integer,
        DevicePropertyCode::AudioInputCH1WindFilter
        | DevicePropertyCode::AudioInputCH2WindFilter
        | DevicePropertyCode::AudioInputCH3WindFilter
        | DevicePropertyCode::AudioInputCH4WindFilter
        | DevicePropertyCode::WindNoiseReduct => PropertyValueType::OnOff,

        // Silent
        DevicePropertyCode::SilentMode => PropertyValueType::Switch,
        DevicePropertyCode::SilentModeApertureDriveInAF => {
            PropertyValueType::SilentModeApertureDrive
        }

        // ND Filter
        DevicePropertyCode::NDFilter => PropertyValueType::Switch,
        DevicePropertyCode::NDFilterModeSetting => PropertyValueType::AutoManual,
        DevicePropertyCode::NDFilterMode
        | DevicePropertyCode::NDFilterValue
        | DevicePropertyCode::NDFilterSwitchingSetting
        | DevicePropertyCode::NDFilterPositionSetting
        | DevicePropertyCode::NDFilterOpticalDensityValue
        | DevicePropertyCode::NDFilterUnitSetting
        | DevicePropertyCode::NDFilterPresetSelect
        | DevicePropertyCode::NDFilterPreset1Value
        | DevicePropertyCode::NDFilterPreset2Value
        | DevicePropertyCode::NDFilterPreset3Value
        | DevicePropertyCode::ManualInputForNDFilterValue
        | DevicePropertyCode::PushAutoNDFilter => PropertyValueType::Integer,

        // Other/Misc
        DevicePropertyCode::AEL | DevicePropertyCode::FEL => PropertyValueType::LockIndicator,
        DevicePropertyCode::AutoReview => PropertyValueType::Switch,

        // Interval Recording
        DevicePropertyCode::IntervalRecMode => PropertyValueType::OnOff,
        DevicePropertyCode::IntervalRecNumberOfShots
        | DevicePropertyCode::IntervalRecShootingInterval
        | DevicePropertyCode::IntervalRecShootingStartTime
        | DevicePropertyCode::IntervalRecStatus => PropertyValueType::Integer,
        DevicePropertyCode::IntervalRecShootIntervalPriority => PropertyValueType::OnOff,
        DevicePropertyCode::IntervalRecAETrackingSensitivity => PropertyValueType::Integer,

        // Continuous Shooting
        DevicePropertyCode::ContinuousShootingSpotBoostStatus
        | DevicePropertyCode::ContinuousShootingSpotBoostFrameSpeed
        | DevicePropertyCode::ContinuousShootingSpotBoostEnableStatus => PropertyValueType::Integer,

        // Custom Buttons
        DevicePropertyCode::AssignableButton1
        | DevicePropertyCode::AssignableButton2
        | DevicePropertyCode::AssignableButton3
        | DevicePropertyCode::AssignableButton4
        | DevicePropertyCode::AssignableButton5
        | DevicePropertyCode::AssignableButton6
        | DevicePropertyCode::AssignableButton7
        | DevicePropertyCode::AssignableButton8
        | DevicePropertyCode::AssignableButton9
        | DevicePropertyCode::AssignableButton10
        | DevicePropertyCode::AssignableButton11
        | DevicePropertyCode::LensAssignableButton1
        | DevicePropertyCode::AssignableButtonIndicator1
        | DevicePropertyCode::AssignableButtonIndicator2
        | DevicePropertyCode::AssignableButtonIndicator3
        | DevicePropertyCode::AssignableButtonIndicator4
        | DevicePropertyCode::AssignableButtonIndicator5
        | DevicePropertyCode::AssignableButtonIndicator6
        | DevicePropertyCode::AssignableButtonIndicator7
        | DevicePropertyCode::AssignableButtonIndicator8
        | DevicePropertyCode::AssignableButtonIndicator9
        | DevicePropertyCode::AssignableButtonIndicator10
        | DevicePropertyCode::AssignableButtonIndicator11
        | DevicePropertyCode::ButtonAssignmentAssignable1
        | DevicePropertyCode::ButtonAssignmentAssignable2
        | DevicePropertyCode::ButtonAssignmentAssignable3
        | DevicePropertyCode::ButtonAssignmentAssignable4
        | DevicePropertyCode::ButtonAssignmentAssignable5
        | DevicePropertyCode::ButtonAssignmentAssignable6
        | DevicePropertyCode::ButtonAssignmentAssignable7
        | DevicePropertyCode::ButtonAssignmentAssignable8
        | DevicePropertyCode::ButtonAssignmentAssignable9
        | DevicePropertyCode::ButtonAssignmentAssignable10
        | DevicePropertyCode::ButtonAssignmentAssignable11
        | DevicePropertyCode::ButtonAssignmentLensAssignable1 => PropertyValueType::Integer,

        // Picture Profile
        DevicePropertyCode::PictureProfile
        | DevicePropertyCode::PictureProfileGamma
        | DevicePropertyCode::PictureProfileColorMode
        | DevicePropertyCode::PictureProfileSaturation
        | DevicePropertyCode::PictureProfileBlackLevel
        | DevicePropertyCode::PictureProfileBlackGammaLevel
        | DevicePropertyCode::PictureProfileBlackGammaRange
        | DevicePropertyCode::PictureProfileKneeMode
        | DevicePropertyCode::PictureProfileKneeAutoSetMaxPoint
        | DevicePropertyCode::PictureProfileKneeAutoSetSensitivity
        | DevicePropertyCode::PictureProfileKneeManualSetPoint
        | DevicePropertyCode::PictureProfileKneeManualSetSlope
        | DevicePropertyCode::PictureProfileDetailLevel
        | DevicePropertyCode::PictureProfileDetailAdjustMode
        | DevicePropertyCode::PictureProfileDetailAdjustVHBalance
        | DevicePropertyCode::PictureProfileDetailAdjustBWBalance
        | DevicePropertyCode::PictureProfileDetailAdjustLimit
        | DevicePropertyCode::PictureProfileDetailAdjustCrispening
        | DevicePropertyCode::PictureProfileDetailAdjustHiLightDetail
        | DevicePropertyCode::PictureProfileColorPhase
        | DevicePropertyCode::PictureProfileColorDepthRed
        | DevicePropertyCode::PictureProfileColorDepthGreen
        | DevicePropertyCode::PictureProfileColorDepthBlue
        | DevicePropertyCode::PictureProfileColorDepthCyan
        | DevicePropertyCode::PictureProfileColorDepthMagenta
        | DevicePropertyCode::PictureProfileColorDepthYellow
        | DevicePropertyCode::PictureProfileCopy
        | DevicePropertyCode::PictureProfileResetEnableStatus
        | DevicePropertyCode::CreativeLook
        | DevicePropertyCode::CreativeLookContrast
        | DevicePropertyCode::CreativeLookHighlights
        | DevicePropertyCode::CreativeLookShadows
        | DevicePropertyCode::CreativeLookSaturation
        | DevicePropertyCode::CreativeLookSharpness
        | DevicePropertyCode::CreativeLookSharpnessRange
        | DevicePropertyCode::CreativeLookClarity
        | DevicePropertyCode::CreativeLookFade
        | DevicePropertyCode::CreativeLookCustomLook
        | DevicePropertyCode::CreativeLookResetEnableStatus
        | DevicePropertyCode::BaseLookValue
        | DevicePropertyCode::BaseLookAppliedofPlayback
        | DevicePropertyCode::BaseLookNameofPlayback
        | DevicePropertyCode::BaseLookImportOperationEnableStatus => PropertyValueType::Integer,

        // Zoom
        DevicePropertyCode::ZoomScale
        | DevicePropertyCode::ZoomSetting
        | DevicePropertyCode::ZoomOperation
        | DevicePropertyCode::ZoomOperationWithInt16
        | DevicePropertyCode::ZoomOperationStatus
        | DevicePropertyCode::ZoomPositionSetting
        | DevicePropertyCode::ZoomPositionCurrentValue
        | DevicePropertyCode::ZoomDrivingStatus
        | DevicePropertyCode::ZoomSpeedRange
        | DevicePropertyCode::ZoomDistance
        | DevicePropertyCode::ZoomDistanceUnitSetting
        | DevicePropertyCode::ZoomBarInformation
        | DevicePropertyCode::ZoomTypeStatus
        | DevicePropertyCode::DigitalZoomScale
        | DevicePropertyCode::DigitalExtenderMagnificationSetting
        | DevicePropertyCode::TeleWideLeverValueCapability
        | DevicePropertyCode::RemoconZoomSpeedType => PropertyValueType::Integer,

        // Media
        DevicePropertyCode::MediaSLOT1Status
        | DevicePropertyCode::MediaSLOT2Status
        | DevicePropertyCode::MediaSLOT3Status
        | DevicePropertyCode::MediaSLOT1RemainingNumber
        | DevicePropertyCode::MediaSLOT2RemainingNumber
        | DevicePropertyCode::MediaSLOT1RemainingTime
        | DevicePropertyCode::MediaSLOT2RemainingTime
        | DevicePropertyCode::MediaSLOT3RemainingTime
        | DevicePropertyCode::MediaSLOT1FileType
        | DevicePropertyCode::MediaSLOT2FileType
        | DevicePropertyCode::MediaSLOT1ImageQuality
        | DevicePropertyCode::MediaSLOT2ImageQuality
        | DevicePropertyCode::MediaSLOT1ImageSize
        | DevicePropertyCode::MediaSLOT2ImageSize
        | DevicePropertyCode::MediaSLOT1RAWFileCompressionType
        | DevicePropertyCode::MediaSLOT2RAWFileCompressionType
        | DevicePropertyCode::MediaSLOT1QuickFormatEnableStatus
        | DevicePropertyCode::MediaSLOT2QuickFormatEnableStatus
        | DevicePropertyCode::MediaSLOT1FormatEnableStatus
        | DevicePropertyCode::MediaSLOT2FormatEnableStatus
        | DevicePropertyCode::MediaSLOT1WritingState
        | DevicePropertyCode::MediaSLOT2WritingState
        | DevicePropertyCode::MediaSLOT1Player
        | DevicePropertyCode::MediaSLOT2Player
        | DevicePropertyCode::MediaSLOT1RecordingAvailableType
        | DevicePropertyCode::MediaSLOT2RecordingAvailableType
        | DevicePropertyCode::MediaSLOT3RecordingAvailableType
        | DevicePropertyCode::MediaSLOT1ContentsInfoListEnableStatus
        | DevicePropertyCode::MediaSLOT2ContentsInfoListEnableStatus
        | DevicePropertyCode::MediaSLOT1ContentsInfoListUpdateTime
        | DevicePropertyCode::MediaSLOT2ContentsInfoListUpdateTime
        | DevicePropertyCode::MediaSLOT1ContentsInfoListRegenerateUpdateTime
        | DevicePropertyCode::MediaSLOT2ContentsInfoListRegenerateUpdateTime
        | DevicePropertyCode::MediaFormatProgressRate
        | DevicePropertyCode::AutoSwitchMedia
        | DevicePropertyCode::SimulRecSetting => PropertyValueType::Integer,

        // Power/Battery
        DevicePropertyCode::SecondBatteryLevel | DevicePropertyCode::TotalBatteryLevel => {
            PropertyValueType::Percentage
        }
        DevicePropertyCode::SecondBatteryRemain
        | DevicePropertyCode::TotalBatteryRemain
        | DevicePropertyCode::BatteryRemainingInMinutes
        | DevicePropertyCode::BatteryRemainDisplayUnit
        | DevicePropertyCode::BatteryRemainingInVoltage
        | DevicePropertyCode::CameraPowerStatus
        | DevicePropertyCode::PowerSource
        | DevicePropertyCode::RecordablePowerSources
        | DevicePropertyCode::DCVoltage => PropertyValueType::Integer,

        // Camera system
        DevicePropertyCode::CameraOperatingMode
        | DevicePropertyCode::CameraErrorCautionStatus
        | DevicePropertyCode::CameraSystemErrorInfo
        | DevicePropertyCode::CameraShakeStatus
        | DevicePropertyCode::CameraSettingReadOperationEnableStatus
        | DevicePropertyCode::CameraSettingSaveOperationEnableStatus
        | DevicePropertyCode::CameraSettingsResetEnableStatus
        | DevicePropertyCode::SdkControlMode
        | DevicePropertyCode::BodyKeyLock => PropertyValueType::Integer,

        // FTP
        DevicePropertyCode::FTPAutoTransfer
        | DevicePropertyCode::FTPAutoTransferTarget
        | DevicePropertyCode::FTPTransferTarget
        | DevicePropertyCode::FTPAutoTransferTargetStillImage
        | DevicePropertyCode::FTPTransferStillImageQualitySize
        | DevicePropertyCode::FTPConnectionStatus
        | DevicePropertyCode::FTPConnectionErrorInfo
        | DevicePropertyCode::FTPServerSettingOperationEnableStatus
        | DevicePropertyCode::FTPServerSettingVersion
        | DevicePropertyCode::FTPTransferSettingReadOperationEnableStatus
        | DevicePropertyCode::FTPTransferSettingSaveOperationEnableStatus
        | DevicePropertyCode::FTPTransferSettingSaveReadState
        | DevicePropertyCode::FTPJobListDataVersion
        | DevicePropertyCode::SelectFTPServer
        | DevicePropertyCode::SelectFTPServerID
        | DevicePropertyCode::ProtectImageInFTPTransfer
        | DevicePropertyCode::MovieFTPAutoTransferTarget
        | DevicePropertyCode::MovieFTPTransferTarget => PropertyValueType::Integer,

        // Subject Recognition
        DevicePropertyCode::SubjectRecognitionAF
        | DevicePropertyCode::SubjectRecognitionInAF
        | DevicePropertyCode::SubjectRecognitionAnimalBirdPriority
        | DevicePropertyCode::SubjectRecognitionAnimalBirdDetectionParts
        | DevicePropertyCode::SubjectRecognitionAnimalDetectionParts
        | DevicePropertyCode::SubjectRecognitionAnimalDetectionSensitivity
        | DevicePropertyCode::SubjectRecognitionAnimalTrackingSensitivity
        | DevicePropertyCode::SubjectRecognitionAnimalTrackingSubjectShiftRange
        | DevicePropertyCode::SubjectRecognitionBirdDetectionParts
        | DevicePropertyCode::SubjectRecognitionBirdDetectionSensitivity
        | DevicePropertyCode::SubjectRecognitionBirdTrackingSensitivity
        | DevicePropertyCode::SubjectRecognitionBirdTrackingSubjectShiftRange
        | DevicePropertyCode::SubjectRecognitionInsectDetectionSensitivity
        | DevicePropertyCode::SubjectRecognitionInsectTrackingSensitivity
        | DevicePropertyCode::SubjectRecognitionInsectTrackingSubjectShiftRange
        | DevicePropertyCode::SubjectRecognitionCarTrainDetectionSensitivity
        | DevicePropertyCode::SubjectRecognitionCarTrainTrackingSensitivity
        | DevicePropertyCode::SubjectRecognitionCarTrainTrackingSubjectShiftRange
        | DevicePropertyCode::SubjectRecognitionPlaneDetectionSensitivity
        | DevicePropertyCode::SubjectRecognitionPlaneTrackingSensitivity
        | DevicePropertyCode::SubjectRecognitionPlaneTrackingSubjectShiftRange
        | DevicePropertyCode::SubjectRecognitionPersonTrackingSubjectShiftRange
        | DevicePropertyCode::SubjectRecognitionPriorityOnRegisteredFace => {
            PropertyValueType::Integer
        }

        // Lens Compensation
        DevicePropertyCode::LensCompensationBreathing
        | DevicePropertyCode::LensCompensationChromaticAberration
        | DevicePropertyCode::LensCompensationDistortion
        | DevicePropertyCode::LensCompensationShading => PropertyValueType::OnOff,

        // Flicker
        DevicePropertyCode::FlickerLessShooting => PropertyValueType::OnOff,
        DevicePropertyCode::FlickerLessShootingStatus
        | DevicePropertyCode::FlickerScanEnableStatus
        | DevicePropertyCode::FlickerScanStatus => PropertyValueType::Integer,

        // E-framing
        DevicePropertyCode::EframingAutoFraming
        | DevicePropertyCode::EframingCommandVersion
        | DevicePropertyCode::EframingHDMICrop
        | DevicePropertyCode::EframingModeAuto
        | DevicePropertyCode::EframingProductionEffect
        | DevicePropertyCode::EframingRecordingImageCrop
        | DevicePropertyCode::EframingScaleAuto
        | DevicePropertyCode::EframingSpeedAuto
        | DevicePropertyCode::EframingSpeedPTZ
        | DevicePropertyCode::EframingTrackingStartMode
        | DevicePropertyCode::EframingType => PropertyValueType::Integer,

        // Cinematic Vlog
        DevicePropertyCode::CinematicVlogAFTransitionSpeed
        | DevicePropertyCode::CinematicVlogLook
        | DevicePropertyCode::CinematicVlogMood
        | DevicePropertyCode::CinematicVlogSetting => PropertyValueType::Integer,

        // Focal Distance
        DevicePropertyCode::FocalDistanceInFeet
        | DevicePropertyCode::FocalDistanceInMeter
        | DevicePropertyCode::FocalDistanceUnitSetting => PropertyValueType::Integer,

        // Depth of Field
        DevicePropertyCode::DepthOfFieldAdjustmentInterlockingMode
        | DevicePropertyCode::DepthOfFieldAdjustmentMode => PropertyValueType::Integer,

        // Content Transfer
        DevicePropertyCode::ContentsTransferCancelEnableStatus
        | DevicePropertyCode::ContentsTransferProgress
        | DevicePropertyCode::ContentsTransferStatus => PropertyValueType::Integer,

        // Monitoring
        DevicePropertyCode::MonitoringAvailableFormat
        | DevicePropertyCode::MonitoringDeliveringStatus
        | DevicePropertyCode::MonitoringDeliveryTypeSupportInfo
        | DevicePropertyCode::MonitoringFormatSupportInformation
        | DevicePropertyCode::MonitoringIsDelivering
        | DevicePropertyCode::MonitoringOutputDisplayHDMI
        | DevicePropertyCode::MonitoringOutputDisplaySDI
        | DevicePropertyCode::MonitoringOutputDisplaySetting1
        | DevicePropertyCode::MonitoringOutputDisplaySetting2
        | DevicePropertyCode::MonitoringOutputDisplaySettingDestAssign
        | DevicePropertyCode::MonitoringOutputFormat
        | DevicePropertyCode::MonitoringSettingVersion
        | DevicePropertyCode::MonitoringTransportProtocol => PropertyValueType::Integer,

        // Movie HDMI Output
        DevicePropertyCode::MovieHDMIOutput4KSetting
        | DevicePropertyCode::MovieHDMIOutputAudioCH
        | DevicePropertyCode::MovieHDMIOutputColorGamutForRAWOut
        | DevicePropertyCode::MovieHDMIOutputRAW
        | DevicePropertyCode::MovieHDMIOutputRawSetting
        | DevicePropertyCode::MovieHDMIOutputRecControl
        | DevicePropertyCode::MovieHDMIOutputRecMedia
        | DevicePropertyCode::MovieHDMIOutputResolution
        | DevicePropertyCode::MovieHDMIOutputTimeCode => PropertyValueType::Integer,

        // Movie Interval Recording
        DevicePropertyCode::MovieIntervalRecCountDownIntervalTime
        | DevicePropertyCode::MovieIntervalRecFrameRateSetting
        | DevicePropertyCode::MovieIntervalRecFrames
        | DevicePropertyCode::MovieIntervalRecIntervalTime
        | DevicePropertyCode::MovieIntervalRecRecordingDuration
        | DevicePropertyCode::MovieIntervalRecRecordingSetting => PropertyValueType::Integer,

        // Movie Image Stabilization
        DevicePropertyCode::MovieImageStabilizationLevel
        | DevicePropertyCode::MovieImageStabilizationSteadyShot => PropertyValueType::Integer,

        // Playback
        DevicePropertyCode::PlaySetOfMultiMedia
        | DevicePropertyCode::PlaybackContentsName
        | DevicePropertyCode::PlaybackContentsNumber
        | DevicePropertyCode::PlaybackContentsRecordingDateTime
        | DevicePropertyCode::PlaybackContentsRecordingFileFormat
        | DevicePropertyCode::PlaybackContentsRecordingFrameRate
        | DevicePropertyCode::PlaybackContentsRecordingResolution
        | DevicePropertyCode::PlaybackContentsTotalNumber
        | DevicePropertyCode::PlaybackMedia
        | DevicePropertyCode::PlaybackViewMode
        | DevicePropertyCode::PlaybackVolumeSettings => PropertyValueType::Integer,

        // Video Stream
        DevicePropertyCode::VideoStreamAdaptiveRateControl
        | DevicePropertyCode::VideoStreamBitRateCompressionMode
        | DevicePropertyCode::VideoStreamBitRateVBRMode
        | DevicePropertyCode::VideoStreamCodec
        | DevicePropertyCode::VideoStreamMaxBitRate
        | DevicePropertyCode::VideoStreamMovieRecPermission
        | DevicePropertyCode::VideoStreamResolution
        | DevicePropertyCode::VideoStreamResolutionMethod
        | DevicePropertyCode::VideoStreamSelect
        | DevicePropertyCode::VideoStreamSettingVersion => PropertyValueType::Integer,

        // Time Shift
        DevicePropertyCode::TimeShiftPreShootingTimeSetting
        | DevicePropertyCode::TimeShiftShooting
        | DevicePropertyCode::TimeShiftShootingStatus
        | DevicePropertyCode::TimeShiftTriggerSetting => PropertyValueType::Integer,

        // Pan/Tilt
        DevicePropertyCode::PanLimitMode
        | DevicePropertyCode::PanLimitRangeMaximum
        | DevicePropertyCode::PanLimitRangeMinimum
        | DevicePropertyCode::PanPositionCurrentValue
        | DevicePropertyCode::PanPositionStatus
        | DevicePropertyCode::PanTiltAccelerationRampCurve
        | DevicePropertyCode::TiltLimitMode
        | DevicePropertyCode::TiltLimitRangeMaximum
        | DevicePropertyCode::TiltLimitRangeMinimum
        | DevicePropertyCode::TiltPositionCurrentValue
        | DevicePropertyCode::TiltPositionStatus => PropertyValueType::Integer,

        // Picture Cache
        DevicePropertyCode::PictureCacheRecSetting
        | DevicePropertyCode::PictureCacheRecSizeAndTime => PropertyValueType::Integer,

        // Paint Look
        DevicePropertyCode::PaintLookAutoKnee
        | DevicePropertyCode::PaintLookBBlack
        | DevicePropertyCode::PaintLookDetailLevel
        | DevicePropertyCode::PaintLookDetailSetting
        | DevicePropertyCode::PaintLookKneePoint
        | DevicePropertyCode::PaintLookKneeSetting
        | DevicePropertyCode::PaintLookKneeSlope
        | DevicePropertyCode::PaintLookMasterBlack
        | DevicePropertyCode::PaintLookMultiMatrixAreaIndication
        | DevicePropertyCode::PaintLookMultiMatrixAxis
        | DevicePropertyCode::PaintLookMultiMatrixHue
        | DevicePropertyCode::PaintLookMultiMatrixSaturation
        | DevicePropertyCode::PaintLookMultiMatrixSetting
        | DevicePropertyCode::PaintLookRBlack
        | DevicePropertyCode::PaintLookUserMatrixBG
        | DevicePropertyCode::PaintLookUserMatrixBR
        | DevicePropertyCode::PaintLookUserMatrixGB
        | DevicePropertyCode::PaintLookUserMatrixGR
        | DevicePropertyCode::PaintLookUserMatrixLevel
        | DevicePropertyCode::PaintLookUserMatrixPhase
        | DevicePropertyCode::PaintLookUserMatrixRB
        | DevicePropertyCode::PaintLookUserMatrixRG
        | DevicePropertyCode::PaintLookUserMatrixSetting => PropertyValueType::Integer,

        // User Bit/User Base Look
        DevicePropertyCode::UserBitPreset
        | DevicePropertyCode::UserBitPresetResetEnableStatus
        | DevicePropertyCode::UserBitTimeRec
        | DevicePropertyCode::UserBaseLookAELevelOffset
        | DevicePropertyCode::UserBaseLookInput
        | DevicePropertyCode::UserBaseLookOutput => PropertyValueType::Integer,

        // Tally Lamp
        DevicePropertyCode::TallyLampControlGreen
        | DevicePropertyCode::TallyLampControlRed
        | DevicePropertyCode::TallyLampControlYellow => PropertyValueType::Integer,

        // Stream
        DevicePropertyCode::StreamButtonEnableStatus
        | DevicePropertyCode::StreamCipherType
        | DevicePropertyCode::StreamDisplayName
        | DevicePropertyCode::StreamLatency
        | DevicePropertyCode::StreamModeSetting
        | DevicePropertyCode::StreamSettingListOperationStatus
        | DevicePropertyCode::StreamStatus
        | DevicePropertyCode::StreamTTL
        | DevicePropertyCode::TargetStreamingDestinationSelect => PropertyValueType::Integer,

        // Scene File
        DevicePropertyCode::SceneFileCommandVersion
        | DevicePropertyCode::SceneFileDownloadOperationEnableStatus
        | DevicePropertyCode::SceneFileIndex
        | DevicePropertyCode::SceneFileIndexesAvailableForDownload
        | DevicePropertyCode::SceneFileUploadOperationEnableStatus => PropertyValueType::Integer,

        // Recorder
        DevicePropertyCode::RecorderClipName
        | DevicePropertyCode::RecorderControlMainSetting
        | DevicePropertyCode::RecorderControlProxySetting
        | DevicePropertyCode::RecorderExtRawStatus
        | DevicePropertyCode::RecorderSaveDestination
        | DevicePropertyCode::RecorderStartMain
        | DevicePropertyCode::RecorderStartProxy
        | DevicePropertyCode::RecordingFileNumber
        | DevicePropertyCode::RecordingFolderFormat
        | DevicePropertyCode::RecordingSettingFileName => PropertyValueType::Integer,

        // Movie misc
        DevicePropertyCode::MovieForwardButton
        | DevicePropertyCode::MovieNextButton
        | DevicePropertyCode::MoviePlayButton
        | DevicePropertyCode::MoviePlayPauseButton
        | DevicePropertyCode::MoviePlayStopButton
        | DevicePropertyCode::MoviePrevButton
        | DevicePropertyCode::MovieRewindButton
        | DevicePropertyCode::MovieQualityFullAutoMode
        | DevicePropertyCode::MovieRecButtonToggleEnableStatus
        | DevicePropertyCode::MovieRecReviewButton
        | DevicePropertyCode::MovieRecReviewPlayingState
        | DevicePropertyCode::MovieRecordingFrameRateRTSPSetting
        | DevicePropertyCode::MovieRecordingResolutionForRAW
        | DevicePropertyCode::MovieRecordingResolutionForRTSP
        | DevicePropertyCode::MovieShootingMode
        | DevicePropertyCode::MovieShootingModeColorGamut
        | DevicePropertyCode::MovieShootingModeTargetDisplay
        | DevicePropertyCode::LogShootingMode
        | DevicePropertyCode::LogShootingModeColorGamut => PropertyValueType::Integer,

        // S&Q Mode
        DevicePropertyCode::SQFrameRate
        | DevicePropertyCode::SQModeSetting
        | DevicePropertyCode::SQRecordingFrameRateSetting
        | DevicePropertyCode::SQRecordingSetting => PropertyValueType::Integer,

        // Various enables/misc
        DevicePropertyCode::APSCOrFullSwitchingEnableStatus
        | DevicePropertyCode::APSCOrFullSwitchingSetting
        | DevicePropertyCode::APSCS35
        | DevicePropertyCode::AmountOfDefocusSetting
        | DevicePropertyCode::AntidustShutterWhenPowerOff
        | DevicePropertyCode::ApertureDriveInAF
        | DevicePropertyCode::AreaTimeZoneSettingVersion
        | DevicePropertyCode::AutoPowerOffTemperature
        | DevicePropertyCode::AutoRecognitionTargetCandidates
        | DevicePropertyCode::AutoRecognitionTargetSetting
        | DevicePropertyCode::BodySerialNumber
        | DevicePropertyCode::CallSetting
        | DevicePropertyCode::CameraButtonFunction
        | DevicePropertyCode::CameraButtonFunctionMulti
        | DevicePropertyCode::CameraButtonFunctionStatus
        | DevicePropertyCode::CameraDialFunction
        | DevicePropertyCode::CameraEframing
        | DevicePropertyCode::CameraLeverFunction
        | DevicePropertyCode::CameraSettingSaveReadState
        | DevicePropertyCode::CancelMediaFormatEnableStatus
        | DevicePropertyCode::CancelRemoteTouchOperationEnableStatus
        | DevicePropertyCode::ColorSpace
        | DevicePropertyCode::ControlForHDMI
        | DevicePropertyCode::CreateNewFolder
        | DevicePropertyCode::CreateNewFolderEnableStatus
        | DevicePropertyCode::CurrentSceneFileEdited
        | DevicePropertyCode::CustomGridLineFileCommandVersion
        | DevicePropertyCode::DateTimeSettings
        | DevicePropertyCode::DeSqueezeDisplayRatio
        | DevicePropertyCode::DebugMode
        | DevicePropertyCode::DeleteContentOperationEnableStatusSLOT1
        | DevicePropertyCode::DeleteContentOperationEnableStatusSLOT2
        | DevicePropertyCode::DeleteUserBaseLook
        | DevicePropertyCode::DeviceOverheatingState
        | DevicePropertyCode::DifferentSetForSQMovie
        | DevicePropertyCode::ElectricFrontCurtainShutter
        | DevicePropertyCode::EmbedLUTFile
        | DevicePropertyCode::EnlargeScreenSetting
        | DevicePropertyCode::EstimatePictureSize
        | DevicePropertyCode::ExtendedInterfaceMode
        | DevicePropertyCode::FTPFunction
        | DevicePropertyCode::FTPPowerSave
        | DevicePropertyCode::FaceEyeDetectionAFStatus
        | DevicePropertyCode::FirmwareUpdateCommandVersion
        | DevicePropertyCode::ForcedFileNumberResetEnableStatus
        | DevicePropertyCode::FullTimeDMF
        | DevicePropertyCode::FunctionOfRemoteTouchOperation
        | DevicePropertyCode::FunctionOfTouchOperation
        | DevicePropertyCode::GetOnly
        | DevicePropertyCode::GridLineDisplayPlayback
        | DevicePropertyCode::HDMIResolutionStillPlay
        | DevicePropertyCode::HighResolutionShutterSpeed
        | DevicePropertyCode::HighResolutionShutterSpeedAdjust
        | DevicePropertyCode::HighResolutionShutterSpeedAdjustInIntegralMultiples
        | DevicePropertyCode::HighResolutionShutterSpeedSetting
        | DevicePropertyCode::HomeMenuSetting
        | DevicePropertyCode::IPSetupProtocolSetting
        | DevicePropertyCode::IRRemoteSetting
        | DevicePropertyCode::ImageIDNum
        | DevicePropertyCode::ImageIDNumSetting
        | DevicePropertyCode::ImageIDString
        | DevicePropertyCode::ImagerScanMode
        | DevicePropertyCode::LanguageSetting
        | DevicePropertyCode::LensAssignableButtonIndicator1
        | DevicePropertyCode::LensInformationEnableStatus
        | DevicePropertyCode::LensModelName
        | DevicePropertyCode::LensSerialNumber
        | DevicePropertyCode::LensVersionNumber
        | DevicePropertyCode::LiveViewArea
        | DevicePropertyCode::LiveViewImageQualityByNumericalValue
        | DevicePropertyCode::MaxVal
        | DevicePropertyCode::MaximumNumberOfBytes
        | DevicePropertyCode::MaximumSizeOfImageIDString
        | DevicePropertyCode::MicrophoneDirectivity
        | DevicePropertyCode::ModelName
        | DevicePropertyCode::MonitorLUTSetting1
        | DevicePropertyCode::MonitorLUTSetting2
        | DevicePropertyCode::MonitorLUTSetting3
        | DevicePropertyCode::MonitorLUTSettingOutputDestAssign
        | DevicePropertyCode::OSDImageMode
        | DevicePropertyCode::PixelMappingEnableStatus
        | DevicePropertyCode::PostViewTransferResourceStatus
        | DevicePropertyCode::PresetPTZFSlotNumber
        | DevicePropertyCode::PriorityKeySettings
        | DevicePropertyCode::ProductShowcaseSet
        | DevicePropertyCode::ProgramShiftStatus
        | DevicePropertyCode::ProxyRecordingSetting
        | DevicePropertyCode::PushAFModeSetting
        | DevicePropertyCode::PushAGC
        | DevicePropertyCode::PushAutoIris
        | DevicePropertyCode::RAWJPCSaveImage
        | DevicePropertyCode::RecognitionTarget
        | DevicePropertyCode::ReleaseWithoutCard
        | DevicePropertyCode::ReleaseWithoutLens
        | DevicePropertyCode::RemoteKeySLOTSelectButton
        | DevicePropertyCode::RemoteKeyThumbnailButton
        | DevicePropertyCode::RemoteTouchOperation
        | DevicePropertyCode::RemoteTouchOperationEnableStatus
        | DevicePropertyCode::RightLeftEyeSelect
        | DevicePropertyCode::S1
        | DevicePropertyCode::S2
        | DevicePropertyCode::SelectIPTCMetadata
        | DevicePropertyCode::SelectUserBaseLookToEdit
        | DevicePropertyCode::SelectUserBaseLookToSetInPPLUT
        | DevicePropertyCode::SensorCleaningEnableStatus
        | DevicePropertyCode::SetCopyright
        | DevicePropertyCode::SetPhotographer
        | DevicePropertyCode::SetPresetPTZFBinaryVersion
        | DevicePropertyCode::ShootingEnableSettingLicense
        | DevicePropertyCode::ShootingTimingPreNotificationMode
        | DevicePropertyCode::SilentModeAutoPixelMapping
        | DevicePropertyCode::SilentModeShutterWhenPowerOff
        | DevicePropertyCode::SimulRecSettingMovieRecButton
        | DevicePropertyCode::SnapshotInfo
        | DevicePropertyCode::SoftwareVersion
        | DevicePropertyCode::StillImageTransSize
        | DevicePropertyCode::SynchroterminalForcedOutput
        | DevicePropertyCode::SystemErrorCautionStatus
        | DevicePropertyCode::TCUBDisplaySetting
        | DevicePropertyCode::TNumber
        | DevicePropertyCode::TimeCodePresetResetEnableStatus
        | DevicePropertyCode::TopOfTheGroupShootingMarkSetting
        | DevicePropertyCode::TouchFunctionInMF
        | DevicePropertyCode::TouchOperation
        | DevicePropertyCode::TrackingOnAndAFOnEnableStatus
        | DevicePropertyCode::USBPowerSupply
        | DevicePropertyCode::UpdateBodyStatus
        | DevicePropertyCode::UploadDatasetVersion
        | DevicePropertyCode::ValidRecordingVideoFormat
        | DevicePropertyCode::VideoRecordingFormatBitrateSetting
        | DevicePropertyCode::VideoRecordingFormatQuality
        | DevicePropertyCode::WakeOnLAN
        | DevicePropertyCode::WriteCopyrightInfo => PropertyValueType::Integer,

        _ => PropertyValueType::Unknown,
    }
}

/// Get a description of what a property does.
pub fn description(code: DevicePropertyCode) -> &'static str {
    match property_category(code) {
        PropertyCategory::Exposure | PropertyCategory::Metering => exposure::description(code),
        PropertyCategory::Focus => focus::description(code),
        PropertyCategory::WhiteBalance => white_balance::description(code),
        PropertyCategory::Drive => drive::description(code),
        PropertyCategory::Flash => flash::description(code),
        PropertyCategory::Image => image::description(code),
        PropertyCategory::Movie => movie::description(code),
        PropertyCategory::Media => media::description(code),
        PropertyCategory::Power => power::description(code),
        PropertyCategory::Zoom => zoom::description(code),
        PropertyCategory::Lens => lens::description(code),
        PropertyCategory::Audio => audio::description(code),
        PropertyCategory::PictureProfile => picture_profile::description(code),
        PropertyCategory::NDFilter => nd_filter::description(code),
        PropertyCategory::Display => display::description(code),
        PropertyCategory::Stabilization => stabilization::description(code),
        PropertyCategory::Silent => silent::description(code),
        _ => other::description(code),
    }
}

/// Get a human-readable display name for a property code.
pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match property_category(code) {
        PropertyCategory::Exposure | PropertyCategory::Metering => exposure::display_name(code),
        PropertyCategory::Focus => focus::display_name(code),
        PropertyCategory::WhiteBalance => white_balance::display_name(code),
        PropertyCategory::Drive => drive::display_name(code),
        PropertyCategory::Flash => flash::display_name(code),
        PropertyCategory::Image => image::display_name(code),
        PropertyCategory::Movie => movie::display_name(code),
        PropertyCategory::Media => media::display_name(code),
        PropertyCategory::Power => power::display_name(code),
        PropertyCategory::Zoom => zoom::display_name(code),
        PropertyCategory::Lens => lens::display_name(code),
        PropertyCategory::Audio => audio::display_name(code),
        PropertyCategory::PictureProfile => picture_profile::display_name(code),
        PropertyCategory::NDFilter => nd_filter::display_name(code),
        PropertyCategory::Display => display::display_name(code),
        PropertyCategory::Stabilization => stabilization::display_name(code),
        PropertyCategory::Silent => silent::display_name(code),
        _ => other::display_name(code),
    }
}
