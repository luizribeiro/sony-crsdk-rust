//! Miscellaneous property metadata (descriptions, display names).

use crsdk_sys::DevicePropertyCode;

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::DRO => {
            "Dynamic Range Optimizer. Automatically adjusts shadows and highlights to preserve detail in high-contrast scenes."
        }
        DevicePropertyCode::MeteredManualLevel => {
            "Exposure meter reading in manual mode. Shows how current settings compare to metered exposure."
        }
        DevicePropertyCode::StreamModeSetting => {
            "Configures the camera for live streaming output. Sets up video format and connection type for streaming platforms."
        }
        DevicePropertyCode::FTPFunction => {
            "Enables FTP file transfer. Automatically uploads images to a configured FTP server over WiFi."
        }
        DevicePropertyCode::WakeOnLAN => {
            "Allows the camera to be powered on remotely via network signal. Useful for remote camera setups."
        }
        DevicePropertyCode::ModelName => "Camera model identifier. Useful for confirming the connected device.",
        DevicePropertyCode::BodySerialNumber => {
            "Unique camera serial number. Important for registration, insurance, and tracking ownership."
        }
        DevicePropertyCode::SoftwareVersion => "Current firmware version. Check for updates to get new features and bug fixes.",
        DevicePropertyCode::IntervalRecMode => {
            "Captures images at set intervals for time-lapse photography. The camera can optionally compile them into a video."
        }
        DevicePropertyCode::IntervalRecNumberOfShots => {
            "Total number of shots to capture during interval shooting. Set to 0 or unlimited for continuous capture until stopped."
        }
        DevicePropertyCode::IntervalRecShootingInterval => {
            "Time between each shot in interval shooting. Shorter intervals capture faster motion; longer intervals for slow changes."
        }
        DevicePropertyCode::IntervalRecShootingStartTime => {
            "Delay before interval shooting begins. Allows time to leave the scene or wait for a specific moment."
        }
        DevicePropertyCode::IntervalRecStatus => {
            "Current state of interval recording. Shows if shooting is active, paused, or complete."
        }
        DevicePropertyCode::IntervalRecShootIntervalPriority => {
            "When enabled, prioritizes maintaining the interval timing even if it requires shorter exposures."
        }
        DevicePropertyCode::IntervalRecAETrackingSensitivity => {
            "How quickly exposure adjusts between interval shots. Low for gradual changes like sunsets. High for faster changes."
        }
        DevicePropertyCode::ContinuousShootingSpotBoostStatus => {
            "Shows if continuous shooting speed boost is active for the current focus area."
        }
        DevicePropertyCode::ContinuousShootingSpotBoostFrameSpeed => {
            "The boosted frame rate when spot boost is active. Higher speeds capture more frames per second."
        }
        DevicePropertyCode::ContinuousShootingSpotBoostEnableStatus => {
            "Indicates whether the spot boost feature can be enabled in the current shooting mode."
        }
        DevicePropertyCode::AspectRatio => {
            "Image shape. 3:2 is standard for full-frame. 16:9 is widescreen. 1:1 is square. 4:3 matches micro four-thirds sensors."
        }
        DevicePropertyCode::SoftSkinEffect => {
            "Smooths skin tones in portraits. Higher settings provide more smoothing."
        }
        DevicePropertyCode::RecorderMainStatus => {
            "Current status of the main video recorder. Shows if recording is active, paused, or stopped."
        }
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
        | DevicePropertyCode::LensAssignableButton1 => {
            "Custom button on the camera body that can be assigned to frequently used functions."
        }
        DevicePropertyCode::AssignableButtonIndicator1
        | DevicePropertyCode::AssignableButtonIndicator2
        | DevicePropertyCode::AssignableButtonIndicator3
        | DevicePropertyCode::AssignableButtonIndicator4
        | DevicePropertyCode::AssignableButtonIndicator5
        | DevicePropertyCode::AssignableButtonIndicator6
        | DevicePropertyCode::AssignableButtonIndicator7
        | DevicePropertyCode::AssignableButtonIndicator8
        | DevicePropertyCode::AssignableButtonIndicator9
        | DevicePropertyCode::AssignableButtonIndicator10
        | DevicePropertyCode::AssignableButtonIndicator11 => {
            "Shows the current state of the assignable button indicator (active/inactive)."
        }
        DevicePropertyCode::ButtonAssignmentAssignable1
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
        | DevicePropertyCode::ButtonAssignmentLensAssignable1 => {
            "The function currently assigned to this custom button."
        }
        // BaseLook properties route to Other
        DevicePropertyCode::BaseLookValue | DevicePropertyCode::BaseLookAppliedofPlayback => {
            "Base look setting that defines the fundamental color characteristics before other adjustments."
        }
        DevicePropertyCode::BaseLookNameofPlayback => {
            "Name of the base look applied during playback."
        }
        DevicePropertyCode::BaseLookImportOperationEnableStatus => {
            "Indicates whether base look import is available."
        }
        DevicePropertyCode::TeleWideLeverValueCapability => {
            "Range of values supported by the tele/wide zoom lever."
        }
        DevicePropertyCode::SimulRecSetting => {
            "Records to both card slots simultaneously. Provides instant backup of every shot."
        }
        DevicePropertyCode::GridLineType => {
            "Choose grid pattern type: rule of thirds, square grid, diagonal lines, etc."
        }
        DevicePropertyCode::LiveViewStatus => {
            "Current state of live view output. Shows if preview is active."
        }
        DevicePropertyCode::LiveViewProtocol => {
            "Protocol used for live view streaming (USB, network, etc.)."
        }
        // Camera system properties
        DevicePropertyCode::CameraPowerStatus => "Current power state of the camera.",
        DevicePropertyCode::CameraOperatingMode => "Current operating mode (photo, video, playback).",
        DevicePropertyCode::CameraErrorCautionStatus => "Error or warning status from the camera.",
        DevicePropertyCode::CameraSystemErrorInfo => "Detailed system error information.",
        DevicePropertyCode::CameraShakeStatus => "Camera shake detection status for blur warning.",
        DevicePropertyCode::CameraSettingReadOperationEnableStatus => {
            "Whether camera settings can be read from file."
        }
        DevicePropertyCode::CameraSettingSaveOperationEnableStatus => {
            "Whether camera settings can be saved to file."
        }
        DevicePropertyCode::CameraSettingsResetEnableStatus => {
            "Whether factory reset is available."
        }
        DevicePropertyCode::SdkControlMode => "SDK control mode for remote operation.",
        DevicePropertyCode::BodyKeyLock => "Locks physical buttons to prevent accidental changes.",
        DevicePropertyCode::DCVoltage => "DC power supply voltage when using external power.",
        // FTP and file transfer (non-Movie category)
        DevicePropertyCode::FTPConnectionStatus => "Current FTP server connection state.",
        DevicePropertyCode::FTPConnectionErrorInfo => "FTP connection error details.",
        DevicePropertyCode::FTPAutoTransfer => "Automatically transfers files to FTP server.",
        DevicePropertyCode::FTPAutoTransferTarget => "Which files to auto-transfer (all, selected).",
        DevicePropertyCode::FTPAutoTransferTargetStillImage => "Auto-transfer setting for still images.",
        DevicePropertyCode::FTPTransferTarget => "Target files for manual FTP transfer.",
        DevicePropertyCode::FTPTransferStillImageQualitySize => "Quality/size setting for FTP transfers.",
        DevicePropertyCode::FTPServerSettingOperationEnableStatus => "Whether FTP settings can be modified.",
        DevicePropertyCode::FTPServerSettingVersion => "FTP server settings version.",
        DevicePropertyCode::FTPTransferSettingReadOperationEnableStatus => {
            "Whether FTP transfer settings can be read."
        }
        DevicePropertyCode::FTPTransferSettingSaveOperationEnableStatus => {
            "Whether FTP transfer settings can be saved."
        }
        DevicePropertyCode::FTPTransferSettingSaveReadState => "Current FTP transfer settings state.",
        DevicePropertyCode::FTPJobListDataVersion => "Version of the FTP job list data.",
        DevicePropertyCode::SelectFTPServer => "Select which configured FTP server to use.",
        DevicePropertyCode::SelectFTPServerID => "ID of the selected FTP server.",
        DevicePropertyCode::ProtectImageInFTPTransfer => "Protect transferred images from deletion.",
        // Subject recognition (AI detection)
        DevicePropertyCode::SubjectRecognitionAF => "AI-powered subject detection for autofocus.",
        DevicePropertyCode::SubjectRecognitionInAF => "Subject recognition mode during AF.",
        DevicePropertyCode::SubjectRecognitionAnimalBirdPriority => {
            "Priority between animal and bird detection."
        }
        DevicePropertyCode::SubjectRecognitionAnimalBirdDetectionParts => {
            "Which body parts to detect on animals/birds (eye, body, head)."
        }
        DevicePropertyCode::SubjectRecognitionAnimalDetectionParts => "Body parts to detect on animals.",
        DevicePropertyCode::SubjectRecognitionAnimalDetectionSensitivity => {
            "Sensitivity for animal detection. Higher values detect smaller subjects."
        }
        DevicePropertyCode::SubjectRecognitionAnimalTrackingSensitivity => {
            "How aggressively to track detected animals."
        }
        DevicePropertyCode::SubjectRecognitionAnimalTrackingSubjectShiftRange => {
            "Range for tracking animal subjects across the frame."
        }
        DevicePropertyCode::SubjectRecognitionBirdDetectionParts => "Body parts to detect on birds.",
        DevicePropertyCode::SubjectRecognitionBirdDetectionSensitivity => {
            "Sensitivity for bird detection."
        }
        DevicePropertyCode::SubjectRecognitionBirdTrackingSensitivity => {
            "How aggressively to track detected birds."
        }
        DevicePropertyCode::SubjectRecognitionBirdTrackingSubjectShiftRange => {
            "Range for tracking bird subjects across the frame."
        }
        DevicePropertyCode::SubjectRecognitionInsectDetectionSensitivity => {
            "Sensitivity for insect detection."
        }
        DevicePropertyCode::SubjectRecognitionInsectTrackingSensitivity => {
            "How aggressively to track detected insects."
        }
        DevicePropertyCode::SubjectRecognitionInsectTrackingSubjectShiftRange => {
            "Range for tracking insect subjects across the frame."
        }
        DevicePropertyCode::SubjectRecognitionCarTrainDetectionSensitivity => {
            "Sensitivity for car/train detection."
        }
        DevicePropertyCode::SubjectRecognitionCarTrainTrackingSensitivity => {
            "How aggressively to track detected vehicles."
        }
        DevicePropertyCode::SubjectRecognitionCarTrainTrackingSubjectShiftRange => {
            "Range for tracking vehicle subjects across the frame."
        }
        DevicePropertyCode::SubjectRecognitionPlaneDetectionSensitivity => {
            "Sensitivity for airplane detection."
        }
        DevicePropertyCode::SubjectRecognitionPlaneTrackingSensitivity => {
            "How aggressively to track detected airplanes."
        }
        DevicePropertyCode::SubjectRecognitionPlaneTrackingSubjectShiftRange => {
            "Range for tracking airplane subjects across the frame."
        }
        DevicePropertyCode::SubjectRecognitionPersonTrackingSubjectShiftRange => {
            "Range for tracking person subjects across the frame."
        }
        DevicePropertyCode::SubjectRecognitionPriorityOnRegisteredFace => {
            "Prioritize registered faces over other subjects."
        }
        // APS-C and sensor mode
        DevicePropertyCode::APSCOrFullSwitchingEnableStatus => {
            "Whether switching between APS-C and full-frame crop is available."
        }
        DevicePropertyCode::APSCOrFullSwitchingSetting => {
            "Choose between full-frame or APS-C crop mode. APS-C extends reach but reduces resolution."
        }
        DevicePropertyCode::APSCS35 => {
            "APS-C/Super 35mm mode. Crops to APS-C size sensor area for extra reach or to use APS-C lenses."
        }
        // Auto review and preview
        DevicePropertyCode::AutoReview => {
            "Shows captured image briefly after shooting. Set duration or disable to maximize shooting speed."
        }
        // Cinematic Vlog settings
        DevicePropertyCode::CinematicVlogSetting => "Cinematic vlog video style preset.",
        DevicePropertyCode::CinematicVlogLook => "Visual look/style for cinematic vlog mode.",
        DevicePropertyCode::CinematicVlogMood => "Color mood preset for cinematic vlog.",
        DevicePropertyCode::CinematicVlogAFTransitionSpeed => {
            "How quickly focus transitions in cinematic vlog mode. Slower is more cinematic."
        }
        // Contents transfer
        DevicePropertyCode::ContentsTransferStatus => "Current state of file transfer operation.",
        DevicePropertyCode::ContentsTransferProgress => "Progress percentage of current file transfer.",
        DevicePropertyCode::ContentsTransferCancelEnableStatus => {
            "Whether the current file transfer can be cancelled."
        }
        // Grid and display
        DevicePropertyCode::CreateNewFolder => "Creates a new folder on the memory card.",
        DevicePropertyCode::CreateNewFolderEnableStatus => "Whether new folder creation is available.",
        DevicePropertyCode::CurrentSceneFileEdited => "Indicates if current scene file has unsaved changes.",
        DevicePropertyCode::CustomGridLineFileCommandVersion => "Version of custom grid line file format.",
        // Depth of field settings
        DevicePropertyCode::DepthOfFieldAdjustmentMode => {
            "Mode for adjusting depth of field preview during shooting."
        }
        DevicePropertyCode::DepthOfFieldAdjustmentInterlockingMode => {
            "Links depth of field preview to other camera settings."
        }
        // De-squeeze display
        DevicePropertyCode::DeSqueezeDisplayRatio => {
            "Display aspect correction for anamorphic lenses. Shows unsqueezed preview."
        }
        // Difference settings
        DevicePropertyCode::DifferentSetForSQMovie => {
            "Use different settings for S&Q (slow & quick) movie mode than regular video."
        }
        // Eframing (auto-framing) settings
        DevicePropertyCode::EframingAutoFraming => "Automatic subject framing using AI detection.",
        DevicePropertyCode::EframingModeAuto => "Automatic mode selection for eframing feature.",
        DevicePropertyCode::EframingProductionEffect => "Production effects applied during auto-framing.",
        DevicePropertyCode::EframingHDMICrop => "HDMI output crop settings for eframing.",
        DevicePropertyCode::EframingRecordingImageCrop => "Image crop settings for eframing recording.",
        DevicePropertyCode::EframingScaleAuto => "Automatic scaling for auto-framing.",
        DevicePropertyCode::EframingSpeedAuto => "Automatic speed adjustment for framing transitions.",
        DevicePropertyCode::EframingSpeedPTZ => "Pan-tilt-zoom speed for eframing movements.",
        DevicePropertyCode::EframingTrackingStartMode => "How auto-framing begins tracking subjects.",
        DevicePropertyCode::EframingType => "Type of auto-framing behavior.",
        DevicePropertyCode::EframingCommandVersion => "Eframing protocol version.",
        // Extended features
        DevicePropertyCode::ExtendedInterfaceMode => "Extended interface mode for advanced control.",
        DevicePropertyCode::EmbedLUTFile => "Embeds LUT file data in recorded video.",
        DevicePropertyCode::EnlargeScreenSetting => "Settings for enlarged screen view.",
        DevicePropertyCode::EstimatePictureSize => "Estimated file size for current image settings.",
        // Model and identification
        DevicePropertyCode::AreaTimeZoneSettingVersion => "Version of timezone settings format.",
        DevicePropertyCode::CallSetting => "Quick recall of saved camera settings.",
        // AEL and exposure lock
        DevicePropertyCode::AEL => "Auto Exposure Lock. Locks the current exposure settings.",
        // Auto recognition
        DevicePropertyCode::AutoRecognitionTargetCandidates => {
            "Available targets for automatic subject recognition."
        }
        DevicePropertyCode::AutoRecognitionTargetSetting => {
            "Selected target type for automatic subject recognition."
        }
        // Camera lever
        DevicePropertyCode::CameraEframing => "Camera electronic framing mode setting.",
        DevicePropertyCode::CameraLeverFunction => "Function assigned to the camera's lever control.",
        // Content operations
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT1 => {
            "Whether content deletion is available for slot 1."
        }
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT2 => {
            "Whether content deletion is available for slot 2."
        }
        DevicePropertyCode::DeleteUserBaseLook => "Delete a user-created base look preset.",
        // Control and interface
        DevicePropertyCode::ControlForHDMI => "HDMI control settings (HDMI-CEC).",
        DevicePropertyCode::DebugMode => "Debug mode for troubleshooting.",
        // Touch and remote
        DevicePropertyCode::CancelRemoteTouchOperationEnableStatus => {
            "Whether remote touch operations can be cancelled."
        }
        DevicePropertyCode::FunctionOfRemoteTouchOperation => "Function activated by remote touch.",
        // Flicker
        DevicePropertyCode::FlickerLessShooting => {
            "Reduces banding from flickering artificial light sources."
        }
        DevicePropertyCode::FlickerLessShootingStatus => "Current flicker reduction status.",
        DevicePropertyCode::FlickerScanEnableStatus => "Whether flicker scan is available.",
        DevicePropertyCode::FlickerScanStatus => "Current flicker scan detection status.",
        // Focal distance display
        DevicePropertyCode::FocalDistanceInFeet => "Shows focus distance in feet.",
        DevicePropertyCode::FocalDistanceInMeter => "Shows focus distance in meters.",
        DevicePropertyCode::FocalDistanceUnitSetting => "Unit for displaying focus distance.",
        // File numbering
        DevicePropertyCode::ForcedFileNumberResetEnableStatus => {
            "Whether file number reset is available."
        }
        // Focus assist
        DevicePropertyCode::FullTimeDMF => {
            "Direct Manual Focus always available even in AF mode."
        }
        // Get only (read-only flag)
        DevicePropertyCode::GetOnly => "Read-only property flag.",
        // Moved from NDFilter (was incorrectly categorized due to "nd" substring)
        DevicePropertyCode::WindNoiseReduct => {
            "Reduces low-frequency wind noise in the built-in microphone. May slightly affect audio quality."
        }
        DevicePropertyCode::DigitalExtenderMagnificationSetting => {
            "Digital extender zoom factor. Crops and enlarges the image beyond optical zoom range."
        }
        // Paint Look settings (professional video color control)
        DevicePropertyCode::PaintLookAutoKnee => {
            "Automatic knee point adjustment. Compresses highlights to prevent clipping."
        }
        DevicePropertyCode::PaintLookBBlack => "Blue channel black level adjustment.",
        DevicePropertyCode::PaintLookRBlack => "Red channel black level adjustment.",
        DevicePropertyCode::PaintLookMasterBlack => {
            "Overall black level adjustment. Affects all color channels equally."
        }
        DevicePropertyCode::PaintLookDetailLevel => {
            "Amount of detail/sharpening applied to the image."
        }
        DevicePropertyCode::PaintLookDetailSetting => "Detail enhancement settings and options.",
        DevicePropertyCode::PaintLookKneePoint => {
            "Highlight compression threshold. Point at which highlights begin to be compressed."
        }
        DevicePropertyCode::PaintLookKneeSetting => {
            "Knee compression mode and settings for highlight handling."
        }
        DevicePropertyCode::PaintLookKneeSlope => {
            "Rate of highlight compression above the knee point."
        }
        DevicePropertyCode::PaintLookMultiMatrixAreaIndication => {
            "Display indicator for multi-matrix color correction areas."
        }
        DevicePropertyCode::PaintLookMultiMatrixAxis => {
            "Color axis selection for multi-matrix adjustments."
        }
        DevicePropertyCode::PaintLookMultiMatrixHue => {
            "Hue adjustment for selected multi-matrix color axis."
        }
        DevicePropertyCode::PaintLookMultiMatrixSaturation => {
            "Saturation adjustment for selected multi-matrix color axis."
        }
        DevicePropertyCode::PaintLookMultiMatrixSetting => "Multi-matrix color correction settings.",
        DevicePropertyCode::PaintLookUserMatrixBG => "User matrix blue-green cross-coupling.",
        DevicePropertyCode::PaintLookUserMatrixBR => "User matrix blue-red cross-coupling.",
        DevicePropertyCode::PaintLookUserMatrixGB => "User matrix green-blue cross-coupling.",
        DevicePropertyCode::PaintLookUserMatrixGR => "User matrix green-red cross-coupling.",
        DevicePropertyCode::PaintLookUserMatrixRB => "User matrix red-blue cross-coupling.",
        DevicePropertyCode::PaintLookUserMatrixRG => "User matrix red-green cross-coupling.",
        DevicePropertyCode::PaintLookUserMatrixLevel => "Overall level of user matrix adjustment.",
        DevicePropertyCode::PaintLookUserMatrixPhase => "Phase adjustment for user color matrix.",
        DevicePropertyCode::PaintLookUserMatrixSetting => "User-defined color matrix settings.",
        // Pan/Tilt (PTZ camera control)
        DevicePropertyCode::PanLimitMode => "Mode for pan movement limits.",
        DevicePropertyCode::PanLimitRangeMaximum => "Maximum pan angle limit.",
        DevicePropertyCode::PanLimitRangeMinimum => "Minimum pan angle limit.",
        DevicePropertyCode::PanPositionCurrentValue => "Current pan position of the camera head.",
        DevicePropertyCode::PanPositionStatus => "Status of pan position control.",
        DevicePropertyCode::PanTiltAccelerationRampCurve => {
            "Acceleration curve for pan/tilt movements. Smoother curves for cinematic motion."
        }
        DevicePropertyCode::TiltLimitMode => "Mode for tilt movement limits.",
        DevicePropertyCode::TiltLimitRangeMaximum => "Maximum tilt angle limit.",
        DevicePropertyCode::TiltLimitRangeMinimum => "Minimum tilt angle limit.",
        DevicePropertyCode::TiltPositionCurrentValue => "Current tilt position of the camera head.",
        DevicePropertyCode::TiltPositionStatus => "Status of tilt position control.",
        // Monitoring and stream settings
        DevicePropertyCode::MonitoringAvailableFormat => {
            "Available video formats for external monitoring output."
        }
        DevicePropertyCode::MonitoringDeliveringStatus => "Current monitoring stream delivery status.",
        DevicePropertyCode::MonitoringDeliveryTypeSupportInfo => {
            "Supported monitoring delivery protocols and types."
        }
        DevicePropertyCode::MonitoringFormatSupportInformation => {
            "Supported formats for monitoring output."
        }
        DevicePropertyCode::MonitoringIsDelivering => "Whether monitoring stream is currently active.",
        DevicePropertyCode::MonitoringOutputDisplayHDMI => "HDMI monitoring output display settings.",
        DevicePropertyCode::MonitoringOutputDisplaySDI => "SDI monitoring output display settings.",
        DevicePropertyCode::MonitoringOutputDisplaySetting1 => "Primary monitoring display settings.",
        DevicePropertyCode::MonitoringOutputDisplaySetting2 => "Secondary monitoring display settings.",
        DevicePropertyCode::MonitoringOutputDisplaySettingDestAssign => {
            "Assignment of monitoring display settings to outputs."
        }
        DevicePropertyCode::MonitoringOutputFormat => "Video format for monitoring output.",
        DevicePropertyCode::MonitoringSettingVersion => "Version of monitoring settings format.",
        DevicePropertyCode::MonitoringTransportProtocol => "Network protocol for monitoring stream.",
        DevicePropertyCode::MonitorLUTSetting1 => "First LUT slot for monitoring output.",
        DevicePropertyCode::MonitorLUTSetting2 => "Second LUT slot for monitoring output.",
        DevicePropertyCode::MonitorLUTSetting3 => "Third LUT slot for monitoring output.",
        DevicePropertyCode::MonitorLUTSettingOutputDestAssign => {
            "LUT assignment to monitoring outputs."
        }
        // Stream settings
        DevicePropertyCode::StreamButtonEnableStatus => {
            "Whether stream start button is enabled."
        }
        DevicePropertyCode::StreamCipherType => "Encryption type for secure streaming.",
        DevicePropertyCode::StreamDisplayName => "Display name for the stream.",
        DevicePropertyCode::StreamLatency => "Stream latency mode (low, normal, etc.).",
        DevicePropertyCode::StreamSettingListOperationStatus => {
            "Status of stream settings list operations."
        }
        DevicePropertyCode::StreamStatus => "Current streaming status.",
        DevicePropertyCode::StreamTTL => "Time-to-live for stream packets.",
        DevicePropertyCode::TargetStreamingDestinationSelect => {
            "Select target destination for streaming."
        }
        // Playback properties
        DevicePropertyCode::PlaySetOfMultiMedia => "Select media set for multi-media playback.",
        DevicePropertyCode::PlaybackContentsName => "Name of currently playing content.",
        DevicePropertyCode::PlaybackContentsNumber => "Number/index of current playback content.",
        DevicePropertyCode::PlaybackContentsRecordingDateTime => {
            "Recording date and time of current playback content."
        }
        DevicePropertyCode::PlaybackContentsRecordingFileFormat => {
            "File format of current playback content."
        }
        DevicePropertyCode::PlaybackContentsRecordingFrameRate => {
            "Frame rate of current playback content."
        }
        DevicePropertyCode::PlaybackContentsRecordingResolution => {
            "Resolution of current playback content."
        }
        DevicePropertyCode::PlaybackContentsTotalNumber => "Total number of playback contents.",
        DevicePropertyCode::PlaybackMedia => "Media slot used for playback.",
        DevicePropertyCode::PlaybackViewMode => "View mode for playback (single, thumbnail, etc.).",
        DevicePropertyCode::PlaybackVolumeSettings => "Audio volume settings for playback.",
        DevicePropertyCode::GridLineDisplayPlayback => "Show grid lines during playback.",
        DevicePropertyCode::HDMIResolutionStillPlay => {
            "HDMI output resolution during still image playback."
        }
        // Time Shift recording
        DevicePropertyCode::TimeShiftPreShootingTimeSetting => {
            "Buffer time before trigger in time shift recording."
        }
        DevicePropertyCode::TimeShiftShooting => "Time shift recording mode setting.",
        DevicePropertyCode::TimeShiftShootingStatus => "Current time shift recording status.",
        DevicePropertyCode::TimeShiftTriggerSetting => "Trigger setting for time shift recording.",
        // Tally lamp
        DevicePropertyCode::TallyLampControlGreen => "Green tally lamp control.",
        DevicePropertyCode::TallyLampControlRed => "Red tally lamp control.",
        DevicePropertyCode::TallyLampControlYellow => "Yellow tally lamp control.",
        // User Bit settings (video metadata)
        DevicePropertyCode::UserBitPreset => "Preset value for user bit metadata in video.",
        DevicePropertyCode::UserBitPresetResetEnableStatus => {
            "Whether user bit reset is available."
        }
        DevicePropertyCode::UserBitTimeRec => "User bit time recording mode.",
        DevicePropertyCode::TCUBDisplaySetting => "Timecode/User Bit display settings.",
        DevicePropertyCode::TimeCodePresetResetEnableStatus => {
            "Whether timecode preset reset is available."
        }
        // User Base Look
        DevicePropertyCode::UserBaseLookAELevelOffset => {
            "Auto-exposure level offset for user base look."
        }
        DevicePropertyCode::UserBaseLookInput => "Input setting for user base look.",
        DevicePropertyCode::UserBaseLookOutput => "Output setting for user base look.",
        DevicePropertyCode::SelectUserBaseLookToEdit => "Select user base look to edit.",
        DevicePropertyCode::SelectUserBaseLookToSetInPPLUT => {
            "Select user base look to set in picture profile LUT."
        }
        // Video Stream settings
        DevicePropertyCode::VideoStreamAdaptiveRateControl => {
            "Adaptive bitrate control for video streaming."
        }
        DevicePropertyCode::VideoStreamBitRateCompressionMode => {
            "Bitrate compression mode (CBR, VBR, etc.)."
        }
        DevicePropertyCode::VideoStreamBitRateVBRMode => "VBR mode settings for video stream.",
        DevicePropertyCode::VideoStreamCodec => "Video codec used for streaming.",
        DevicePropertyCode::VideoStreamMaxBitRate => "Maximum bitrate for video streaming.",
        DevicePropertyCode::VideoStreamMovieRecPermission => {
            "Permission for movie recording during streaming."
        }
        DevicePropertyCode::VideoStreamResolution => "Resolution for video streaming.",
        DevicePropertyCode::VideoStreamResolutionMethod => {
            "Method for determining stream resolution."
        }
        DevicePropertyCode::VideoStreamSelect => "Select video stream configuration.",
        DevicePropertyCode::VideoStreamSettingVersion => "Version of video stream settings.",
        DevicePropertyCode::VideoRecordingFormatBitrateSetting => {
            "Bitrate setting for video recording format."
        }
        DevicePropertyCode::VideoRecordingFormatQuality => "Quality level for video recording.",
        DevicePropertyCode::ValidRecordingVideoFormat => {
            "List of valid video formats for recording."
        }
        // Scene file settings
        DevicePropertyCode::SceneFileCommandVersion => "Scene file command protocol version.",
        DevicePropertyCode::SceneFileDownloadOperationEnableStatus => {
            "Whether scene file download is available."
        }
        DevicePropertyCode::SceneFileIndex => "Index of current scene file.",
        DevicePropertyCode::SceneFileIndexesAvailableForDownload => {
            "Scene file indexes available for download."
        }
        DevicePropertyCode::SceneFileUploadOperationEnableStatus => {
            "Whether scene file upload is available."
        }
        // Miscellaneous camera settings
        DevicePropertyCode::AmountOfDefocusSetting => {
            "Amount of intentional defocus for creative effects."
        }
        DevicePropertyCode::AntidustShutterWhenPowerOff => {
            "Closes shutter when powering off to protect sensor from dust."
        }
        DevicePropertyCode::ApertureDriveInAF => "Aperture behavior during autofocus.",
        DevicePropertyCode::CameraButtonFunction => "Function assigned to camera button.",
        DevicePropertyCode::CameraButtonFunctionMulti => "Multi-function camera button settings.",
        DevicePropertyCode::CameraButtonFunctionStatus => "Current camera button function status.",
        DevicePropertyCode::CameraDialFunction => "Function assigned to camera dial.",
        DevicePropertyCode::CameraSettingSaveReadState => "State of camera setting save/read.",
        DevicePropertyCode::CancelMediaFormatEnableStatus => {
            "Whether media format can be cancelled."
        }
        DevicePropertyCode::ColorSpace => "Color space setting (sRGB, Adobe RGB, etc.).",
        DevicePropertyCode::DateTimeSettings => "Camera date and time settings.",
        DevicePropertyCode::ElectricFrontCurtainShutter => {
            "Uses electronic shutter for first curtain. Reduces shutter shock."
        }
        DevicePropertyCode::FEL => "Flash Exposure Lock. Locks flash output level.",
        DevicePropertyCode::FirmwareUpdateCommandVersion => "Firmware update protocol version.",
        DevicePropertyCode::FunctionOfTouchOperation => "Function assigned to touch operation.",
        DevicePropertyCode::HighResolutionShutterSpeed => "High-resolution shutter speed value.",
        DevicePropertyCode::HighResolutionShutterSpeedAdjust => {
            "Fine adjustment for high-resolution shutter speed."
        }
        DevicePropertyCode::HighResolutionShutterSpeedAdjustInIntegralMultiples => {
            "Shutter speed adjustment in integral multiples."
        }
        DevicePropertyCode::HighResolutionShutterSpeedSetting => {
            "Settings for high-resolution shutter speed mode."
        }
        DevicePropertyCode::HomeMenuSetting => "Home menu configuration.",
        DevicePropertyCode::IRRemoteSetting => "Infrared remote control settings.",
        DevicePropertyCode::IPSetupProtocolSetting => "IP address setup protocol (DHCP, static).",
        DevicePropertyCode::ImageIDNum => "Numeric image identifier.",
        DevicePropertyCode::ImageIDNumSetting => "Image ID number setting.",
        DevicePropertyCode::ImageIDString => "String image identifier.",
        DevicePropertyCode::ImagerScanMode => "Image sensor scanning mode.",
        DevicePropertyCode::LanguageSetting => "Camera menu language setting.",
        DevicePropertyCode::LensAssignableButtonIndicator1 => {
            "Indicator for lens assignable button."
        }
        DevicePropertyCode::LensCompensationBreathing => {
            "Compensates for focus breathing (focal length shift during focusing)."
        }
        DevicePropertyCode::LensInformationEnableStatus => {
            "Whether lens information is available."
        }
        DevicePropertyCode::LensSerialNumber => "Lens serial number.",
        DevicePropertyCode::LensVersionNumber => "Lens firmware version.",
        DevicePropertyCode::LiveViewArea => "Area of the frame shown in live view.",
        DevicePropertyCode::LiveViewImageQualityByNumericalValue => {
            "Live view image quality as numeric value."
        }
        DevicePropertyCode::MaxVal => "Maximum value property.",
        DevicePropertyCode::MaximumNumberOfBytes => "Maximum bytes for data transfer.",
        DevicePropertyCode::MaximumSizeOfImageIDString => "Maximum length of image ID string.",
        DevicePropertyCode::MicrophoneDirectivity => {
            "Microphone pickup pattern (omni, directional, etc.)."
        }
        DevicePropertyCode::OSDImageMode => "On-screen display image mode.",
        DevicePropertyCode::PictureCacheRecSetting => "Picture cache recording settings.",
        DevicePropertyCode::PictureCacheRecSizeAndTime => {
            "Size and time settings for picture cache recording."
        }
        DevicePropertyCode::PixelMappingEnableStatus => "Whether pixel mapping is available.",
        DevicePropertyCode::PostViewTransferResourceStatus => {
            "Status of post-view image transfer resources."
        }
        DevicePropertyCode::PresetPTZFSlotNumber => "PTZ preset slot number.",
        DevicePropertyCode::PriorityKeySettings => "Priority key configuration.",
        DevicePropertyCode::ProductShowcaseSet => "Product showcase mode settings.",
        DevicePropertyCode::ProgramShiftStatus => "Status of program shift in P mode.",
        DevicePropertyCode::PushAGC => "Push automatic gain control.",
        DevicePropertyCode::PushAutoIris => "Push automatic iris control.",
        DevicePropertyCode::RAWJPCSaveImage => "Save both RAW and JPEG together.",
        DevicePropertyCode::RecognitionTarget => "Target type for subject recognition.",
        DevicePropertyCode::RedEyeReduction => "Reduces red-eye effect in flash photos.",
        DevicePropertyCode::ReleaseWithoutCard => "Allow shutter release without memory card.",
        DevicePropertyCode::ReleaseWithoutLens => "Allow shutter release without lens attached.",
        DevicePropertyCode::RemoteKeySLOTSelectButton => "Remote key slot select button.",
        DevicePropertyCode::RemoteKeyThumbnailButton => "Remote key thumbnail button.",
        DevicePropertyCode::RemoteTouchOperation => "Remote touch operation mode.",
        DevicePropertyCode::RemoteTouchOperationEnableStatus => {
            "Whether remote touch operation is available."
        }
        DevicePropertyCode::RightLeftEyeSelect => "Select left or right eye for eye AF.",
        DevicePropertyCode::S1 => "S1 shutter button half-press action.",
        DevicePropertyCode::S2 => "S2 shutter button full-press action.",
        DevicePropertyCode::SensorCleaningEnableStatus => "Whether sensor cleaning is available.",
        DevicePropertyCode::SetCopyright => "Set copyright information in image metadata.",
        DevicePropertyCode::SetPhotographer => "Set photographer name in image metadata.",
        DevicePropertyCode::SetPresetPTZFBinaryVersion => "PTZ preset binary version.",
        DevicePropertyCode::ShootingEnableSettingLicense => "Shooting license settings.",
        DevicePropertyCode::ShootingTimingPreNotificationMode => {
            "Pre-notification mode for shooting timing."
        }
        DevicePropertyCode::SilentModeApertureDriveInAF => {
            "Silent aperture drive during autofocus."
        }
        DevicePropertyCode::SilentModeAutoPixelMapping => {
            "Silent mode for automatic pixel mapping."
        }
        DevicePropertyCode::SilentModeShutterWhenPowerOff => {
            "Silent shutter operation when powering off."
        }
        DevicePropertyCode::SimulRecSettingMovieRecButton => {
            "Movie record button behavior for simultaneous recording."
        }
        DevicePropertyCode::SnapshotInfo => "Snapshot information.",
        DevicePropertyCode::StillImageTransSize => "Transfer size for still images.",
        DevicePropertyCode::SynchroterminalForcedOutput => {
            "Forced output through sync terminal."
        }
        DevicePropertyCode::SystemErrorCautionStatus => "System error and caution status.",
        DevicePropertyCode::TNumber => "T-number (transmission-based f-number).",
        DevicePropertyCode::TopOfTheGroupShootingMarkSetting => {
            "Marks first shot of each burst group."
        }
        DevicePropertyCode::TouchFunctionInMF => "Touch function in manual focus mode.",
        DevicePropertyCode::TouchOperation => "Touch screen operation mode.",
        DevicePropertyCode::TrackingOnAndAFOnEnableStatus => {
            "Whether tracking and AF-On can be combined."
        }
        DevicePropertyCode::USBPowerSupply => "USB power supply settings.",
        DevicePropertyCode::Undefined => "Undefined property.",
        DevicePropertyCode::UpdateBodyStatus => "Camera body update status.",
        DevicePropertyCode::UploadDatasetVersion => "Upload dataset version.",
        DevicePropertyCode::WriteCopyrightInfo => "Write copyright information to files.",
        DevicePropertyCode::SelectIPTCMetadata => "Select IPTC metadata preset.",
        DevicePropertyCode::FaceEyeDetectionAFStatus => {
            "Shows current status of face/eye detection. Indicates whether faces or eyes are currently detected."
        }
        DevicePropertyCode::PreAF => {
            "Camera continuously adjusts focus before half-pressing shutter. Faster initial focus but uses more battery."
        }
        DevicePropertyCode::PushAFModeSetting => {
            "Configures behavior of Push AF button. Can be set to focus once, focus hold, or other modes."
        }
        DevicePropertyCode::LogShootingMode => {
            "Enables log gamma curves (S-Log2, S-Log3) for maximum dynamic range. Requires color grading."
        }
        DevicePropertyCode::SQModeSetting => {
            "Slow & Quick mode settings. Enables high frame rate capture for slow motion effects."
        }
        DevicePropertyCode::RecorderStartMain => "Starts recording on the main recorder slot.",
        DevicePropertyCode::RecorderClipName => "Current clip name for the recording.",
        DevicePropertyCode::RecorderControlMainSetting => {
            "Recording control settings for the main recorder slot."
        }
        DevicePropertyCode::RecorderSaveDestination => {
            "Destination for saving recorded footage. Selects memory card slot or external recorder."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::PushAGC => "Push Gain",
        DevicePropertyCode::DRO => "D-Range Optimizer",
        DevicePropertyCode::MeteredManualLevel => "Meter Level (M)",
        DevicePropertyCode::StreamStatus => "Strm Status",
        DevicePropertyCode::StreamModeSetting => "Stream Mode",
        DevicePropertyCode::StreamDisplayName => "Stream Name",
        DevicePropertyCode::StreamLatency => "Strm Latency",
        DevicePropertyCode::StreamButtonEnableStatus => "Stream Button Status",
        DevicePropertyCode::StreamCipherType => "Stream Cipher",
        DevicePropertyCode::StreamSettingListOperationStatus => "Stream Settings Status",
        DevicePropertyCode::StreamTTL => "Strm TTL",
        DevicePropertyCode::TargetStreamingDestinationSelect => "Stream Destination",
        DevicePropertyCode::VideoStreamAdaptiveRateControl => "Adaptive Rate Control",
        DevicePropertyCode::VideoStreamBitRateCompressionMode => "Bitrate Compression",
        DevicePropertyCode::VideoStreamBitRateVBRMode => "VBR Mode",
        DevicePropertyCode::VideoStreamCodec => "Video Codec",
        DevicePropertyCode::VideoStreamMaxBitRate => "Max Bitrate",
        DevicePropertyCode::VideoStreamResolution => "Stream Resolution",
        DevicePropertyCode::VideoStreamResolutionMethod => "Resolution Method",
        DevicePropertyCode::VideoStreamSelect => "Stream Select",
        DevicePropertyCode::VideoStreamSettingVersion => "Stream Settings Ver",
        DevicePropertyCode::FTPFunction => "FTP Func",
        DevicePropertyCode::FTPAutoTransfer => "FTP Auto Xfer",
        DevicePropertyCode::FTPConnectionStatus => "FTP Connection",
        DevicePropertyCode::WakeOnLAN => "Wake on LAN",
        DevicePropertyCode::IPSetupProtocolSetting => "IP Setup Protocol",
        DevicePropertyCode::AssignableButton1 => "Btn C1",
        DevicePropertyCode::AssignableButton2 => "Btn C2",
        DevicePropertyCode::AssignableButton3 => "Btn C3",
        DevicePropertyCode::AssignableButton4 => "Btn C4",
        DevicePropertyCode::AssignableButton5 => "Btn C5",
        DevicePropertyCode::AssignableButton6 => "Btn C6",
        DevicePropertyCode::AssignableButton7 => "Btn C7",
        DevicePropertyCode::AssignableButton8 => "Btn C8",
        DevicePropertyCode::AssignableButton9 => "Btn C9",
        DevicePropertyCode::AssignableButton10 => "Btn C10",
        DevicePropertyCode::AssignableButton11 => "Btn C11",
        DevicePropertyCode::AssignableButtonIndicator1 => "Btn Ind 1",
        DevicePropertyCode::AssignableButtonIndicator2 => "Btn Ind 2",
        DevicePropertyCode::AssignableButtonIndicator3 => "Btn Ind 3",
        DevicePropertyCode::AssignableButtonIndicator4 => "Btn Ind 4",
        DevicePropertyCode::AssignableButtonIndicator5 => "Btn Ind 5",
        DevicePropertyCode::AssignableButtonIndicator6 => "Btn Ind 6",
        DevicePropertyCode::AssignableButtonIndicator7 => "Btn Ind 7",
        DevicePropertyCode::AssignableButtonIndicator8 => "Btn Ind 8",
        DevicePropertyCode::AssignableButtonIndicator9 => "Btn Ind 9",
        DevicePropertyCode::AssignableButtonIndicator10 => "Btn Ind 10",
        DevicePropertyCode::AssignableButtonIndicator11 => "Btn Ind 11",
        DevicePropertyCode::ButtonAssignmentAssignable1 => "Assign C1",
        DevicePropertyCode::ButtonAssignmentAssignable2 => "Assign C2",
        DevicePropertyCode::ButtonAssignmentAssignable3 => "Assign C3",
        DevicePropertyCode::ButtonAssignmentAssignable4 => "Assign C4",
        DevicePropertyCode::ButtonAssignmentAssignable5 => "Assign C5",
        DevicePropertyCode::ButtonAssignmentAssignable6 => "Assign C6",
        DevicePropertyCode::ButtonAssignmentAssignable7 => "Assign C7",
        DevicePropertyCode::ButtonAssignmentAssignable8 => "Assign C8",
        DevicePropertyCode::ButtonAssignmentAssignable9 => "Assign C9",
        DevicePropertyCode::ButtonAssignmentAssignable10 => "Assign C10",
        DevicePropertyCode::ButtonAssignmentAssignable11 => "Assign C11",
        DevicePropertyCode::ButtonAssignmentLensAssignable1 => "Assign Lens",
        DevicePropertyCode::LensAssignableButton1 => "Lens Btn",
        DevicePropertyCode::CameraButtonFunction => "Button Function",
        DevicePropertyCode::CameraButtonFunctionMulti => "Multi Button Func",
        DevicePropertyCode::CameraButtonFunctionStatus => "Button Func Status",
        DevicePropertyCode::CameraDialFunction => "Dial Function",
        DevicePropertyCode::RemoteKeyThumbnailButton => "Thumbnail Button",
        DevicePropertyCode::FunctionOfTouchOperation => "Touch Function",
        DevicePropertyCode::TouchFunctionInMF => "Touch Fn (MF)",
        DevicePropertyCode::TouchOperation => "Touch Op",
        DevicePropertyCode::ModelName => "Camera Model",
        DevicePropertyCode::BodySerialNumber => "Serial Number",
        DevicePropertyCode::SoftwareVersion => "Firmware Version",
        DevicePropertyCode::DateTimeSettings => "Date/Time",
        DevicePropertyCode::LanguageSetting => "Language",
        DevicePropertyCode::CameraOperatingMode => "Operating Mode",
        DevicePropertyCode::CameraSettingSaveReadState => "Settings State",
        DevicePropertyCode::IntervalRecMode => "Interval Rec",
        DevicePropertyCode::IntervalRecNumberOfShots => "Interval #",
        DevicePropertyCode::IntervalRecShootingInterval => "Interval Time",
        DevicePropertyCode::IntervalRecShootingStartTime => "Interval Start",
        DevicePropertyCode::IntervalRecStatus => "Interval State",
        DevicePropertyCode::IntervalRecShootIntervalPriority => "Interval Priority",
        DevicePropertyCode::IntervalRecAETrackingSensitivity => "Interval AE Track",
        DevicePropertyCode::ContinuousShootingSpotBoostStatus => "Burst Boost",
        DevicePropertyCode::ContinuousShootingSpotBoostFrameSpeed => "Burst Boost FPS",
        DevicePropertyCode::ContinuousShootingSpotBoostEnableStatus => "Burst Boost Avail",
        DevicePropertyCode::AspectRatio => "Aspect",
        DevicePropertyCode::SoftSkinEffect => "Soft Skin",
        DevicePropertyCode::RecorderMainStatus => "Main Rec Status",
        // BaseLook properties route to Other
        DevicePropertyCode::BaseLookValue => "Base Look",
        DevicePropertyCode::BaseLookAppliedofPlayback => "Base Look (Play)",
        DevicePropertyCode::BaseLookNameofPlayback => "Base Look Name",
        DevicePropertyCode::BaseLookImportOperationEnableStatus => "Base Look Import",
        DevicePropertyCode::TeleWideLeverValueCapability => "Tele/Wide Lever",
        DevicePropertyCode::SimulRecSetting => "Simul. Rec",
        DevicePropertyCode::GridLineType => "Grid Type",
        DevicePropertyCode::LiveViewStatus => "LV Status",
        DevicePropertyCode::LiveViewProtocol => "LV Protocol",
        DevicePropertyCode::LogShootingMode => "Log Mode",
        DevicePropertyCode::LogShootingModeColorGamut => "Log Gamut",
        DevicePropertyCode::MaxVal => "Maximum Value",
        DevicePropertyCode::MaximumNumberOfBytes => "Maximum Bytes",
        DevicePropertyCode::MaximumSizeOfImageIDString => "Max ID Size",
        DevicePropertyCode::MeteringMode => "Meter Mode",
        DevicePropertyCode::MicrophoneDirectivity => "Mic Directivity",
        DevicePropertyCode::MovieShootingMode => "Movie Mode",
        DevicePropertyCode::OSDImageMode => "OSD Mode",
        // Camera system
        DevicePropertyCode::CameraPowerStatus => "Power Status",
        DevicePropertyCode::CameraErrorCautionStatus => "Error Status",
        DevicePropertyCode::CameraSystemErrorInfo => "System Error",
        DevicePropertyCode::CameraShakeStatus => "Shake Status",
        DevicePropertyCode::CameraSettingReadOperationEnableStatus => "Settings Read",
        DevicePropertyCode::CameraSettingSaveOperationEnableStatus => "Settings Save",
        DevicePropertyCode::CameraSettingsResetEnableStatus => "Reset Avail",
        DevicePropertyCode::SdkControlMode => "SDK Mode",
        DevicePropertyCode::BodyKeyLock => "Key Lock",
        DevicePropertyCode::SystemErrorCautionStatus => "System Error Status",
        DevicePropertyCode::UpdateBodyStatus => "Body Update",
        // FTP (non-Movie category FTP properties)
        DevicePropertyCode::FTPConnectionErrorInfo => "FTP Error",
        DevicePropertyCode::FTPAutoTransferTarget => "FTP Auto Target",
        DevicePropertyCode::FTPAutoTransferTargetStillImage => "FTP Auto (Still)",
        DevicePropertyCode::FTPTransferTarget => "FTP Target",
        DevicePropertyCode::FTPTransferStillImageQualitySize => "FTP Quality",
        DevicePropertyCode::FTPServerSettingOperationEnableStatus => "FTP Server Edit",
        DevicePropertyCode::FTPServerSettingVersion => "FTP Server Ver",
        DevicePropertyCode::FTPTransferSettingReadOperationEnableStatus => "FTP Read Avail",
        DevicePropertyCode::FTPTransferSettingSaveOperationEnableStatus => "FTP Save Avail",
        DevicePropertyCode::FTPTransferSettingSaveReadState => "FTP Settings",
        DevicePropertyCode::FTPJobListDataVersion => "FTP Job Ver",
        DevicePropertyCode::SelectFTPServer => "FTP Server",
        DevicePropertyCode::SelectFTPServerID => "FTP Server ID",
        DevicePropertyCode::ProtectImageInFTPTransfer => "FTP Protect",
        // Subject recognition
        DevicePropertyCode::SubjectRecognitionAF => "Subject Rec AF",
        DevicePropertyCode::SubjectRecognitionInAF => "Subject In AF",
        DevicePropertyCode::SubjectRecognitionAnimalBirdPriority => "Animal/Bird Pri",
        DevicePropertyCode::SubjectRecognitionAnimalBirdDetectionParts => "Animal/Bird Parts",
        DevicePropertyCode::SubjectRecognitionAnimalDetectionParts => "Animal Parts",
        DevicePropertyCode::SubjectRecognitionAnimalDetectionSensitivity => "Animal Sens",
        DevicePropertyCode::SubjectRecognitionAnimalTrackingSensitivity => "Animal Track",
        DevicePropertyCode::SubjectRecognitionAnimalTrackingSubjectShiftRange => "Animal Shift",
        DevicePropertyCode::SubjectRecognitionBirdDetectionParts => "Bird Parts",
        DevicePropertyCode::SubjectRecognitionBirdDetectionSensitivity => "Bird Sens",
        DevicePropertyCode::SubjectRecognitionBirdTrackingSensitivity => "Bird Track",
        DevicePropertyCode::SubjectRecognitionBirdTrackingSubjectShiftRange => "Bird Shift",
        DevicePropertyCode::SubjectRecognitionInsectDetectionSensitivity => "Insect Sens",
        DevicePropertyCode::SubjectRecognitionInsectTrackingSensitivity => "Insect Track",
        DevicePropertyCode::SubjectRecognitionInsectTrackingSubjectShiftRange => "Insect Shift",
        DevicePropertyCode::SubjectRecognitionCarTrainDetectionSensitivity => "Vehicle Sens",
        DevicePropertyCode::SubjectRecognitionCarTrainTrackingSensitivity => "Vehicle Track",
        DevicePropertyCode::SubjectRecognitionCarTrainTrackingSubjectShiftRange => "Vehicle Shift",
        DevicePropertyCode::SubjectRecognitionPlaneDetectionSensitivity => "Plane Sens",
        DevicePropertyCode::SubjectRecognitionPlaneTrackingSensitivity => "Plane Track",
        DevicePropertyCode::SubjectRecognitionPlaneTrackingSubjectShiftRange => "Plane Shift",
        DevicePropertyCode::SubjectRecognitionPersonTrackingSubjectShiftRange => "Person Shift",
        DevicePropertyCode::SubjectRecognitionPriorityOnRegisteredFace => "Reg Face Priority",
        // APS-C and sensor mode
        DevicePropertyCode::APSCOrFullSwitchingEnableStatus => "APS-C/Full Avail",
        DevicePropertyCode::APSCOrFullSwitchingSetting => "APS-C/Full Mode",
        DevicePropertyCode::APSCS35 => "APS-C/S35",
        // Auto review
        DevicePropertyCode::AutoReview => "Auto Rev",
        // Cinematic Vlog
        DevicePropertyCode::CinematicVlogSetting => "Cine Vlog",
        DevicePropertyCode::CinematicVlogLook => "Cine Vlog Look",
        DevicePropertyCode::CinematicVlogMood => "Cine Vlog Mood",
        DevicePropertyCode::CinematicVlogAFTransitionSpeed => "Cine Vlog AF",
        // Contents transfer
        DevicePropertyCode::ContentsTransferStatus => "Transfer Status",
        DevicePropertyCode::ContentsTransferProgress => "Transfer Progress",
        DevicePropertyCode::ContentsTransferCancelEnableStatus => "Transfer Cancel",
        // Folder and scene
        DevicePropertyCode::CreateNewFolder => "New Folder",
        DevicePropertyCode::CreateNewFolderEnableStatus => "New Folder Avail",
        DevicePropertyCode::CustomGridLineFileCommandVersion => "Grid Line Ver",
        // Depth of field
        DevicePropertyCode::DepthOfFieldAdjustmentMode => "DoF Adjust Mode",
        DevicePropertyCode::DepthOfFieldAdjustmentInterlockingMode => "DoF Interlock",
        // S&Q difference
        DevicePropertyCode::DifferentSetForSQMovie => "S&Q Movie Set",
        // Eframing
        DevicePropertyCode::EframingAutoFraming => "Auto Framing",
        DevicePropertyCode::EframingModeAuto => "E-Frame Auto",
        DevicePropertyCode::EframingProductionEffect => "E-Frame Effect",
        DevicePropertyCode::EframingHDMICrop => "E-Frame HDMI",
        DevicePropertyCode::EframingScaleAuto => "E-Frame Scale",
        DevicePropertyCode::EframingSpeedAuto => "E-Frame Speed",
        DevicePropertyCode::EframingSpeedPTZ => "E-Frame PTZ",
        DevicePropertyCode::EframingTrackingStartMode => "E-Frame Start",
        DevicePropertyCode::EframingType => "E-Frame Type",
        DevicePropertyCode::EframingCommandVersion => "E-Frame Ver",
        // Extended features
        DevicePropertyCode::ExtendedInterfaceMode => "Ext Interface",
        DevicePropertyCode::EnlargeScreenSetting => "Enlarge Screen",
        DevicePropertyCode::FaceEyeDetectionAFStatus => "Face/Eye AF Status",
        DevicePropertyCode::FirmwareUpdateCommandVersion => "Firmware Update Version",
        DevicePropertyCode::LiveViewArea => "LV Area",
        // Model and settings
        DevicePropertyCode::AreaTimeZoneSettingVersion => "Timezone Ver",
        DevicePropertyCode::CallSetting => "Recall Settings",
        DevicePropertyCode::HomeMenuSetting => "Home Menu",
        // AEL
        DevicePropertyCode::AEL => "AE Lock",
        // Auto recognition
        DevicePropertyCode::AutoRecognitionTargetCandidates => "Rec Target Options",
        DevicePropertyCode::AutoRecognitionTargetSetting => "Rec Target",
        // Camera lever
        DevicePropertyCode::CameraEframing => "Camera E-Frame",
        DevicePropertyCode::CameraLeverFunction => "Lever Function",
        // Content operations
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT1 => "Delete Slot 1",
        DevicePropertyCode::DeleteContentOperationEnableStatusSLOT2 => "Delete Slot 2",
        DevicePropertyCode::DeleteUserBaseLook => "Del User Look",
        // Control and interface
        DevicePropertyCode::ControlForHDMI => "HDMI Control",
        DevicePropertyCode::DebugMode => "Debug",
        // Touch and remote
        DevicePropertyCode::CancelRemoteTouchOperationEnableStatus => "Cancel Remote",
        DevicePropertyCode::FunctionOfRemoteTouchOperation => "Remote Touch Fn",
        DevicePropertyCode::RemoteTouchOperation => "Remote Touch Op",
        DevicePropertyCode::RemoteTouchOperationEnableStatus => "Remote Touch Status",
        DevicePropertyCode::IRRemoteSetting => "IR Remote Setting",
        // Flicker
        DevicePropertyCode::FlickerLessShooting => "Flicker Reduce",
        DevicePropertyCode::FlickerLessShootingStatus => "Flicker Status",
        DevicePropertyCode::FlickerScanEnableStatus => "Flicker Scan Avail",
        DevicePropertyCode::FlickerScanStatus => "Flicker Scan",
        // Focal distance
        DevicePropertyCode::FocalDistanceInFeet => "Focus Dist (ft)",
        DevicePropertyCode::FocalDistanceInMeter => "Focus Dist (m)",
        DevicePropertyCode::FocalDistanceUnitSetting => "Focus Dist Unit",
        // File numbering
        DevicePropertyCode::ForcedFileNumberResetEnableStatus => "Force File Reset",
        // Focus assist
        DevicePropertyCode::FullTimeDMF => "Full-time DMF",
        DevicePropertyCode::TrackingOnAndAFOnEnableStatus => "Tracking+AF Status",
        // Read-only flag
        DevicePropertyCode::GetOnly => "Read Only",
        // Moved from NDFilter (was incorrectly categorized due to "nd" substring)
        DevicePropertyCode::WindNoiseReduct => "Wind Noise Reduct.",
        DevicePropertyCode::DigitalExtenderMagnificationSetting => "Digital Extender",
        // PaintLook properties
        DevicePropertyCode::PaintLookAutoKnee => "Auto Knee",
        DevicePropertyCode::PaintLookBBlack => "B Black Level",
        DevicePropertyCode::PaintLookDetailLevel => "Detail Level",
        DevicePropertyCode::PaintLookDetailSetting => "Detail Setting",
        DevicePropertyCode::PaintLookKneePoint => "Knee Point",
        DevicePropertyCode::PaintLookKneeSetting => "Knee Setting",
        DevicePropertyCode::PaintLookKneeSlope => "Knee Slope",
        DevicePropertyCode::PaintLookMasterBlack => "Master Black",
        DevicePropertyCode::PaintLookMultiMatrixAreaIndication => "Matrix Area Indicator",
        DevicePropertyCode::PaintLookMultiMatrixAxis => "Matrix Axis",
        DevicePropertyCode::PaintLookMultiMatrixHue => "Matrix Hue",
        DevicePropertyCode::PaintLookMultiMatrixSaturation => "Matrix Saturation",
        DevicePropertyCode::PaintLookMultiMatrixSetting => "Matrix Setting",
        DevicePropertyCode::PaintLookRBlack => "R Black Level",
        DevicePropertyCode::PaintLookUserMatrixBG => "User Matrix B-G",
        DevicePropertyCode::PaintLookUserMatrixBR => "User Matrix B-R",
        DevicePropertyCode::PaintLookUserMatrixGB => "User Matrix G-B",
        DevicePropertyCode::PaintLookUserMatrixGR => "User Matrix G-R",
        DevicePropertyCode::PaintLookUserMatrixLevel => "User Matrix Level",
        DevicePropertyCode::PaintLookUserMatrixPhase => "User Matrix Phase",
        DevicePropertyCode::PaintLookUserMatrixRB => "User Matrix R-B",
        DevicePropertyCode::PaintLookUserMatrixRG => "User Matrix R-G",
        DevicePropertyCode::PaintLookUserMatrixSetting => "User Matrix Setting",
        DevicePropertyCode::PictureCacheRecSetting => "Cache Rec",
        DevicePropertyCode::PictureCacheRecSizeAndTime => "Cache Size/Time",
        DevicePropertyCode::PixelMappingEnableStatus => "Pixel Mapping Status",
        DevicePropertyCode::PostViewTransferResourceStatus => "Post View Transfer Status",
        DevicePropertyCode::PreAF => "Pre-AF",
        DevicePropertyCode::PresetPTZFSlotNumber => "PTZF Slot",
        DevicePropertyCode::PriorityKeySettings => "Priority Keys",
        DevicePropertyCode::ProductShowcaseSet => "Showcase Set",
        DevicePropertyCode::ProgramShiftStatus => "Prog Shift",
        DevicePropertyCode::PushAFModeSetting => "Push AF Mode",
        // Pan/Tilt properties
        DevicePropertyCode::PanLimitMode => "Pan Limit",
        DevicePropertyCode::PanLimitRangeMaximum => "Pan Limit Max",
        DevicePropertyCode::PanLimitRangeMinimum => "Pan Limit Min",
        DevicePropertyCode::PanPositionCurrentValue => "Pan Position",
        DevicePropertyCode::PanPositionStatus => "Pan Status",
        DevicePropertyCode::PanTiltAccelerationRampCurve => "PT Accel Curve",
        DevicePropertyCode::TiltLimitMode => "Tilt Limit",
        DevicePropertyCode::TiltLimitRangeMaximum => "Tilt Limit Max",
        DevicePropertyCode::TiltLimitRangeMinimum => "Tilt Limit Min",
        DevicePropertyCode::TiltPositionCurrentValue => "Tilt Position",
        DevicePropertyCode::TiltPositionStatus => "Tilt Status",
        // Tally lamp properties
        DevicePropertyCode::TallyLampControlGreen => "Tally Lamp (Green)",
        DevicePropertyCode::TallyLampControlRed => "Tally Lamp (Red)",
        DevicePropertyCode::TallyLampControlYellow => "Tally Lamp (Yellow)",
        // Timecode properties
        DevicePropertyCode::TCUBDisplaySetting => "TC/UB Display",
        DevicePropertyCode::TNumber => "T-Number",
        DevicePropertyCode::TopOfTheGroupShootingMarkSetting => "Group Shooting Mark",
        DevicePropertyCode::SynchroterminalForcedOutput => "Sync Terminal Output",
        // Playback properties
        DevicePropertyCode::PlaySetOfMultiMedia => "Multi-Media Set",
        DevicePropertyCode::PlaybackContentsName => "Content Name",
        DevicePropertyCode::PlaybackContentsNumber => "Content Number",
        DevicePropertyCode::PlaybackContentsRecordingDateTime => "Rec Date/Time",
        DevicePropertyCode::PlaybackContentsRecordingFileFormat => "Rec Format",
        DevicePropertyCode::PlaybackContentsRecordingFrameRate => "Rec Frame Rate",
        DevicePropertyCode::PlaybackContentsRecordingResolution => "Rec Resolution",
        DevicePropertyCode::PlaybackContentsTotalNumber => "Total Contents",
        DevicePropertyCode::PlaybackMedia => "Play Media",
        DevicePropertyCode::PlaybackViewMode => "Play View Mode",
        DevicePropertyCode::PlaybackVolumeSettings => "Playback Volume",
        // Recorder properties
        DevicePropertyCode::RAWJPCSaveImage => "RAW+JPG Save",
        DevicePropertyCode::RecognitionTarget => "Rec Target",
        DevicePropertyCode::RecordablePowerSources => "Rec Power",
        DevicePropertyCode::RecorderClipName => "Clip Name",
        DevicePropertyCode::RecorderControlMainSetting => "Main Rec Control",
        DevicePropertyCode::RecorderControlProxySetting => "Proxy Control",
        DevicePropertyCode::RecorderExtRawStatus => "Ext RAW",
        DevicePropertyCode::RecorderSaveDestination => "Save Destination",
        DevicePropertyCode::RecorderStartMain => "Start Main Rec",
        DevicePropertyCode::RecordingFileNumber => "File Number",
        DevicePropertyCode::RecordingFolderFormat => "Folder Format",
        // Scene file properties
        DevicePropertyCode::SceneFileCommandVersion => "Scene Ver",
        DevicePropertyCode::SceneFileDownloadOperationEnableStatus => "Scene Download",
        DevicePropertyCode::SceneFileIndex => "Scene Index",
        DevicePropertyCode::SceneFileIndexesAvailableForDownload => "Scene Downloads",
        DevicePropertyCode::SceneFileUploadOperationEnableStatus => "Scene Upload",
        DevicePropertyCode::SensorCleaningEnableStatus => "Sensor Cleaning Status",
        // Metadata properties
        DevicePropertyCode::SelectIPTCMetadata => "IPTC Metadata",
        DevicePropertyCode::SetCopyright => "Copyright",
        DevicePropertyCode::SetPhotographer => "Photographer",
        DevicePropertyCode::SetPresetPTZFBinaryVersion => "Preset PTZF Binary Version",
        DevicePropertyCode::ShootingEnableSettingLicense => "Shooting License Status",
        DevicePropertyCode::ShootingTimingPreNotificationMode => "Shooting Timing Notification",
        DevicePropertyCode::ShutterSpeed => "Shutter",
        DevicePropertyCode::SilentMode => "Silent",
        DevicePropertyCode::SimulRecSettingMovieRecButton => "Simul Rec Btn",
        DevicePropertyCode::SnapshotInfo => "Snapshot",
        DevicePropertyCode::StillImageTransSize => "Still Xfer Size",
        DevicePropertyCode::WriteCopyrightInfo => "Write Copyright",
        // User base look selection
        DevicePropertyCode::SelectUserBaseLookToEdit => "Edit Base Look",
        DevicePropertyCode::SelectUserBaseLookToSetInPPLUT => "Base Look for PP",
        DevicePropertyCode::UserBaseLookAELevelOffset => "Base Look AE Offset",
        DevicePropertyCode::UserBaseLookInput => "Base Look Input",
        DevicePropertyCode::UserBaseLookOutput => "Base Look Output",
        // User bit properties
        DevicePropertyCode::UserBitPreset => "UB Preset",
        DevicePropertyCode::UserBitPresetResetEnableStatus => "UB Reset Status",
        DevicePropertyCode::UserBitTimeRec => "UB Time Rec",
        DevicePropertyCode::UploadDatasetVersion => "Dataset Ver",
        // Time shift properties
        DevicePropertyCode::TimeShiftPreShootingTimeSetting => "Pre-Shoot Time",
        DevicePropertyCode::TimeShiftShooting => "Time Shift",
        DevicePropertyCode::TimeShiftShootingStatus => "Time Shift Status",
        DevicePropertyCode::TimeShiftTriggerSetting => "Time Shift Trigger",
        // Miscellaneous settings
        DevicePropertyCode::RedEyeReduction => "Red-Eye",
        DevicePropertyCode::ReleaseWithoutCard => "Release w/o Card",
        DevicePropertyCode::ReleaseWithoutLens => "No Lens Release",
        DevicePropertyCode::RightLeftEyeSelect => "Right/Left Eye Select",
        DevicePropertyCode::S1 => "S1 (Half Press)",
        DevicePropertyCode::S2 => "S2 (Full Press)",
        DevicePropertyCode::SQModeSetting => "S&Q Mode Setting",
        // Video recording properties
        DevicePropertyCode::ValidRecordingVideoFormat => "Valid Formats",
        DevicePropertyCode::VideoRecordingFormatBitrateSetting => "Rec Bitrate",
        DevicePropertyCode::VideoRecordingFormatQuality => "Rec Quality",
        // Zoom properties
        DevicePropertyCode::ZoomDistance => "Zoom Dist",
        DevicePropertyCode::ZoomSetting => "Zoom Set",
        DevicePropertyCode::ZoomSpeedRange => "Zoom Speed",
        // Other properties
        DevicePropertyCode::CancelMediaFormatEnableStatus => "Cancel Format",
        DevicePropertyCode::FEL => "Flash EV Lock",
        DevicePropertyCode::Undefined => "Unknown",
        _ => code.name(),
    }
}
