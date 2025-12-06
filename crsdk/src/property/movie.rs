//! Movie/video recording property types and metadata.

use super::PropertyValueType;
use crsdk_sys::DevicePropertyCode;

/// Movie file format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MovieFileFormat {
    Avchd = 0,
    Mp4 = 1,
    XavcS4k = 2,
    XavcSHd = 3,
    XavcHs8k = 4,
    XavcHs4k = 5,
    XavcSL4k = 6,
    XavcSLHd = 7,
    XavcSI4k = 8,
    XavcSIHd = 9,
    XavcI = 10,
    XavcL = 11,
    XavcHsHd = 12,
    XavcSIDci4k = 13,
    XavcHIHq = 14,
    XavcHISq = 15,
    XavcHL = 16,
    XOcnXt = 17,
    XOcnSt = 18,
}

impl MovieFileFormat {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        Some(match value as u8 {
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
            _ => return None,
        })
    }
}

impl std::fmt::Display for MovieFileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

/// Format movie recording quality/bitrate setting to display string
pub fn format_movie_quality(value: u64) -> String {
    match value as u16 {
        0 => "--".to_string(),
        1 => "60p 50M".to_string(),
        2 => "30p 50M".to_string(),
        3 => "24p 50M".to_string(),
        4 => "50p 50M".to_string(),
        5 => "25p 50M".to_string(),
        6 => "60i 24M".to_string(),
        7 => "50i 24M FX".to_string(),
        8 => "60i 17M FH".to_string(),
        9 => "50i 17M FH".to_string(),
        10 => "60p 28M PS".to_string(),
        11 => "50p 28M PS".to_string(),
        12 => "24p 24M FX".to_string(),
        13 => "25p 24M FX".to_string(),
        14 => "24p 17M FH".to_string(),
        15 => "25p 17M FH".to_string(),
        16 => "120p 50M 720".to_string(),
        17 => "100p 50M 720".to_string(),
        18 => "1080 30p 16M".to_string(),
        19 => "1080 25p 16M".to_string(),
        20 => "720 30p 6M".to_string(),
        21 => "720 25p 6M".to_string(),
        22 => "1080 60p 28M".to_string(),
        23 => "1080 50p 28M".to_string(),
        24 => "60p 25M".to_string(),
        25 => "50p 25M".to_string(),
        26 => "30p 16M".to_string(),
        27 => "25p 16M".to_string(),
        28 => "120p 100M".to_string(),
        29 => "100p 100M".to_string(),
        30 => "120p 60M".to_string(),
        31 => "100p 60M".to_string(),
        32 => "30p 100M".to_string(),
        33 => "25p 100M".to_string(),
        34 => "24p 100M".to_string(),
        35 => "30p 60M".to_string(),
        36 => "25p 60M".to_string(),
        37 => "24p 60M".to_string(),
        38 => "600M 10bit".to_string(),
        39 => "500M 10bit".to_string(),
        40 => "400M 10bit".to_string(),
        41 => "300M 10bit".to_string(),
        42 => "280M 10bit".to_string(),
        43 => "250M 10bit".to_string(),
        44 => "240M 10bit".to_string(),
        45 => "222M 10bit".to_string(),
        46 => "200M 10bit".to_string(),
        47 => "200M 10bit 420".to_string(),
        48 => "200M 8bit".to_string(),
        49 => "185M 10bit".to_string(),
        50 => "150M 10bit 420".to_string(),
        51 => "150M 8bit".to_string(),
        52 => "140M 10bit".to_string(),
        53 => "111M 10bit".to_string(),
        54 => "100M 10bit".to_string(),
        55 => "100M 10bit 420".to_string(),
        56 => "100M 8bit".to_string(),
        57 => "93M 10bit".to_string(),
        58 => "89M 10bit".to_string(),
        59 => "75M 10bit 420".to_string(),
        60 => "60M 8bit".to_string(),
        61 => "50M 10bit".to_string(),
        62 => "50M 10bit 420".to_string(),
        63 => "50M 8bit".to_string(),
        64 => "45M 10bit 420".to_string(),
        65 => "30M 10bit 420".to_string(),
        66 => "25M 8bit".to_string(),
        67 => "16M 8bit".to_string(),
        68 => "520M 10bit".to_string(),
        69 => "260M 10bit".to_string(),
        _ => format!("{}M", value),
    }
}

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::AudioRecording => {
            "Enables or disables audio recording with video. Turn off when using external audio recorders."
        }
        DevicePropertyCode::MovieFileFormat => {
            "Container format for video. XAVC S/HS/I offer high quality with various compression options. MP4 is widely compatible."
        }
        DevicePropertyCode::MovieRecordingSetting => {
            "Video resolution and bitrate combination. Higher settings (4K, high bitrate) have more detail but larger files."
        }
        DevicePropertyCode::MovieRecordingFrameRateSetting => {
            "Frames per second. 24fps is cinematic. 30fps is standard video. 60fps is smooth for action. 120fps+ enables slow motion."
        }
        DevicePropertyCode::RecordingState => {
            "Current recording status. Shows whether the camera is actively recording video."
        }
        DevicePropertyCode::TimeCodeFormat => {
            "Timecode standard. Drop Frame (DF) compensates for 29.97fps timing. Non-Drop Frame (NDF) counts frames directly."
        }
        DevicePropertyCode::ProxyRecordingSetting => {
            "Records a smaller, lower-quality copy alongside the main video. Useful for faster editing workflows."
        }
        DevicePropertyCode::SQRecordingSetting => {
            "Slow & Quick motion settings. Records at different frame rates for slow motion or time-lapse effects in-camera."
        }
        DevicePropertyCode::LogShootingMode => {
            "Enables log gamma curves (S-Log2, S-Log3) for maximum dynamic range. Requires color grading in post-production."
        }
        DevicePropertyCode::RecordingSelfTimer => {
            "Delay before the shutter fires. Useful for self-portraits or to avoid camera shake from pressing the button."
        }
        DevicePropertyCode::RecordingSelfTimerContinuous => {
            "Number of shots to take after the self-timer countdown. Useful for taking multiple self-portraits in succession."
        }
        DevicePropertyCode::RecordingSelfTimerCountTime => {
            "Duration of the self-timer countdown in seconds before the shutter fires."
        }
        DevicePropertyCode::RecordingSelfTimerStatus => {
            "Current state of the self-timer countdown. Shows whether the timer is active and counting down."
        }
        DevicePropertyCode::MovieRecordingResolutionForMain => {
            "Video resolution for the main recording. Higher resolutions have more detail but larger file sizes."
        }
        DevicePropertyCode::MovieRecordingResolutionForProxy => {
            "Video resolution for proxy recordings. Lower resolution copies for faster editing."
        }
        DevicePropertyCode::MovieRecordingFrameRateProxySetting => {
            "Frame rate setting for proxy video recordings."
        }
        DevicePropertyCode::MovieProxyFileFormat => {
            "File format for proxy video recordings. Typically a smaller, more edit-friendly format."
        }
        DevicePropertyCode::RecordingMedia => {
            "Which memory card slot is used for recording."
        }
        DevicePropertyCode::MovieRecordingMedia => {
            "Memory card slot for movie recordings. Can differ from still image storage."
        }
        DevicePropertyCode::RecorderProxyStatus => {
            "Current status of the proxy video recorder. Shows if proxy recording is active."
        }
        DevicePropertyCode::TimeCodeMake => {
            "How timecode is generated. Preset uses manual settings. Free Run continues counting even when not recording."
        }
        DevicePropertyCode::TimeCodePreset => {
            "Starting timecode value. Set to match other cameras or continue from previous recordings."
        }
        DevicePropertyCode::TimeCodeRun => {
            "Timecode behavior. Rec Run only advances during recording. Free Run advances continuously."
        }
        DevicePropertyCode::MovieRecordingFileNumber => {
            "Current file number for movie recordings. Auto-increments with each new recording."
        }
        DevicePropertyCode::MoviePlayingState => {
            "Current playback status. Shows if video is playing, paused, or stopped."
        }
        DevicePropertyCode::MoviePlayingSpeed => {
            "Current playback speed. Can be slowed down or sped up for review."
        }
        DevicePropertyCode::MediaSLOT1RecordingAvailableType
        | DevicePropertyCode::MediaSLOT2RecordingAvailableType
        | DevicePropertyCode::MediaSLOT3RecordingAvailableType => {
            "Types of recordings supported by the card in this slot (photo, video, etc.)."
        }
        DevicePropertyCode::MovieFTPAutoTransferTarget => {
            "Target files for automatic FTP transfer of movie recordings."
        }
        DevicePropertyCode::MovieFTPTransferTarget => {
            "Target files for manual FTP transfer of movie recordings."
        }
        DevicePropertyCode::MovieHDMIOutput4KSetting => {
            "Controls 4K output over HDMI during movie recording. Enables external monitors or recorders to receive 4K signal."
        }
        DevicePropertyCode::MovieHDMIOutputAudioCH => {
            "Number of audio channels output over HDMI. Typically 2 for stereo or more for professional multi-channel setups."
        }
        DevicePropertyCode::MovieHDMIOutputColorGamutForRAWOut => {
            "Color gamut used when outputting RAW video over HDMI. Affects color space for external recorders."
        }
        DevicePropertyCode::MovieHDMIOutputRAW => {
            "Enables RAW video output over HDMI for external recording with maximum quality and flexibility."
        }
        DevicePropertyCode::MovieHDMIOutputRawSetting => {
            "Configuration for RAW HDMI output including bit depth and compression settings."
        }
        DevicePropertyCode::MovieHDMIOutputRecControl => {
            "Allows camera to control recording on external HDMI recorders. Syncs start/stop between devices."
        }
        DevicePropertyCode::MovieHDMIOutputRecMedia => {
            "Specifies which media slot is used when recording via HDMI output."
        }
        DevicePropertyCode::MovieHDMIOutputResolution => {
            "Resolution of the HDMI video output. Can be set to match recording or downscaled for monitoring."
        }
        DevicePropertyCode::MovieHDMIOutputTimeCode => {
            "Embeds timecode in HDMI output signal. Essential for syncing with external recorders in multi-camera setups."
        }
        DevicePropertyCode::MovieImageStabilizationLevel => {
            "Intensity of in-body image stabilization for video. Higher values reduce shake but may cause warping artifacts."
        }
        DevicePropertyCode::MovieImageStabilizationSteadyShot => {
            "SteadyShot stabilization mode for movie recording. Active mode provides stronger stabilization with slight crop."
        }
        DevicePropertyCode::MovieIntervalRecCountDownIntervalTime => {
            "Countdown time displayed before interval recording starts. Gives time to prepare for first frame capture."
        }
        DevicePropertyCode::MovieIntervalRecFrameRateSetting => {
            "Frame rate for interval recording playback. Determines how fast the time-lapse plays back."
        }
        DevicePropertyCode::MovieIntervalRecFrames => {
            "Total number of frames to capture in interval recording. Combined with interval determines total recording time."
        }
        DevicePropertyCode::MovieIntervalRecIntervalTime => {
            "Time between each frame capture in interval recording. Longer intervals compress more time into the final video."
        }
        DevicePropertyCode::MovieIntervalRecRecordingDuration => {
            "Total duration of the resulting interval recording video at the playback frame rate."
        }
        DevicePropertyCode::MovieIntervalRecRecordingSetting => {
            "Overall settings for interval recording mode including quality and format options."
        }
        DevicePropertyCode::MovieForwardButton => {
            "Fast forward button control for movie playback. Skips ahead in recorded video."
        }
        DevicePropertyCode::MovieNextButton => {
            "Next button control for movie playback. Jumps to the next recorded clip."
        }
        DevicePropertyCode::MoviePlayButton => {
            "Play button control for movie playback. Starts playing the current video clip."
        }
        DevicePropertyCode::MoviePlayPauseButton => {
            "Play/Pause toggle for movie playback. Alternates between playing and paused states."
        }
        DevicePropertyCode::MoviePlayStopButton => {
            "Stop button control for movie playback. Stops playback and returns to beginning."
        }
        DevicePropertyCode::MoviePrevButton => {
            "Previous button control for movie playback. Jumps to the previous recorded clip."
        }
        DevicePropertyCode::MovieRewindButton => {
            "Rewind button control for movie playback. Plays video in reverse."
        }
        DevicePropertyCode::MovieQualityFullAutoMode => {
            "Allows camera to automatically select movie quality settings in full auto mode."
        }
        DevicePropertyCode::MovieRecButtonToggleEnableStatus => {
            "Indicates whether the movie record button toggle mode is available."
        }
        DevicePropertyCode::MovieRecReviewButton => {
            "Button to review the most recently recorded movie clip."
        }
        DevicePropertyCode::MovieRecReviewPlayingState => {
            "Current state of movie review playback. Shows if review is playing, paused, or stopped."
        }
        DevicePropertyCode::MovieRecordingFrameRateRTSPSetting => {
            "Frame rate for RTSP streaming output. Affects bandwidth and stream quality."
        }
        DevicePropertyCode::MovieRecordingResolutionForRAW => {
            "Resolution when recording RAW video internally. Higher resolutions require faster media."
        }
        DevicePropertyCode::MovieRecordingResolutionForRTSP => {
            "Resolution for RTSP streaming output. Lower resolutions reduce bandwidth requirements."
        }
        DevicePropertyCode::MovieShootingMode => {
            "Movie-specific shooting mode. Determines exposure behavior and available settings for video recording."
        }
        DevicePropertyCode::MovieShootingModeColorGamut => {
            "Color gamut used in movie shooting mode. Options include standard, wide gamut, and cinema color spaces."
        }
        DevicePropertyCode::MovieShootingModeTargetDisplay => {
            "Target display type for movie color settings. Optimizes output for specific monitor types."
        }
        DevicePropertyCode::SQFrameRate => {
            "Playback frame rate for Slow & Quick motion. Determines how fast or slow the footage plays."
        }
        DevicePropertyCode::SQModeSetting => {
            "Slow & Quick mode settings. Enables high frame rate capture for slow motion effects."
        }
        DevicePropertyCode::SQRecordingFrameRateSetting => {
            "Recording frame rate for S&Q mode. Higher rates enable more dramatic slow motion."
        }
        DevicePropertyCode::RecorderClipName => {
            "Naming format for recorded movie clips. Customizes how files are named on the memory card."
        }
        DevicePropertyCode::RecorderControlMainSetting => {
            "Main recorder control settings. Configures primary recording behavior and triggers."
        }
        DevicePropertyCode::RecorderControlProxySetting => {
            "Proxy recorder control settings. Configures secondary low-res recording for editing."
        }
        DevicePropertyCode::RecorderExtRawStatus => {
            "Status of external RAW recorder. Shows if external recorder is connected and recording."
        }
        DevicePropertyCode::RecorderSaveDestination => {
            "Destination for recorded files. Selects memory card slot or external device."
        }
        DevicePropertyCode::RecorderStartMain => {
            "Triggers main recorder to start or stop recording."
        }
        DevicePropertyCode::RecorderStartProxy => {
            "Triggers proxy recorder to start or stop recording."
        }
        DevicePropertyCode::RecordingFileNumber => {
            "Current file number for recordings. Auto-increments with each new file."
        }
        DevicePropertyCode::RecordingFolderFormat => {
            "Folder naming format for recorded files. Organizes media by date or custom scheme."
        }
        DevicePropertyCode::RecordingSettingFileName => {
            "File naming settings for recordings. Customizes how files are named."
        }
        DevicePropertyCode::DifferentSetForSQMovie => {
            "Enables different settings for Slow & Quick movie recording. Allows separate quality settings from normal video."
        }
        DevicePropertyCode::PictureCacheRecSetting => {
            "Picture cache recording settings. Captures frames before the record button is pressed for pre-roll."
        }
        DevicePropertyCode::PictureCacheRecSizeAndTime => {
            "Size and duration of the picture cache buffer. Determines how many seconds of pre-roll are captured."
        }
        DevicePropertyCode::PlaybackContentsRecordingDateTime => {
            "Date and time the currently playing content was recorded."
        }
        DevicePropertyCode::PlaybackContentsRecordingFileFormat => {
            "File format of the currently playing content (XAVC, AVCHD, etc.)."
        }
        DevicePropertyCode::PlaybackContentsRecordingFrameRate => {
            "Frame rate at which the currently playing content was recorded."
        }
        DevicePropertyCode::PlaybackContentsRecordingResolution => {
            "Resolution of the currently playing content (4K, HD, etc.)."
        }
        DevicePropertyCode::PlaybackMedia => {
            "Memory card slot or media source for playback. Selects which card to browse."
        }
        DevicePropertyCode::PlaySetOfMultiMedia => {
            "Configuration for playing content from multiple media sources or cards."
        }
        DevicePropertyCode::SimulRecSettingMovieRecButton => {
            "Simultaneous recording settings when pressing the movie record button. Records to multiple destinations."
        }
        DevicePropertyCode::ValidRecordingVideoFormat => {
            "Lists video formats supported by the current camera and card configuration."
        }
        DevicePropertyCode::VideoRecordingFormatBitrateSetting => {
            "Bitrate setting for the selected video recording format. Higher bitrates provide better quality."
        }
        DevicePropertyCode::VideoRecordingFormatQuality => {
            "Quality level for video recording. Affects detail and file size."
        }
        DevicePropertyCode::VideoStreamMovieRecPermission => {
            "Allows or restricts movie recording during video streaming. May be limited by bandwidth or heat."
        }
        DevicePropertyCode::EframingRecordingImageCrop => {
            "Image crop settings for electronic framing recording."
        }
        DevicePropertyCode::TimeCodePresetResetEnableStatus => {
            "Whether timecode preset can be reset to default."
        }
        DevicePropertyCode::UserBitTimeRec => {
            "User bit time recording mode for video metadata."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::AudioRecording => "Audio Rec",
        DevicePropertyCode::MovieFileFormat => "Movie Format",
        DevicePropertyCode::MovieRecordingSetting => "Movie Quality",
        DevicePropertyCode::MovieRecordingFrameRateSetting => "Movie Frame Rate",
        DevicePropertyCode::MovieRecordingResolutionForMain => "Movie Resolution",
        DevicePropertyCode::MovieRecordingResolutionForProxy => "Proxy Resolution",
        DevicePropertyCode::MovieShootingMode => "Movie Mode",
        DevicePropertyCode::MovieShootingModeColorGamut => "Movie Color Gamut",
        DevicePropertyCode::RecordingState => "Rec State",
        DevicePropertyCode::RecordingMedia => "Rec Media",
        DevicePropertyCode::ProxyRecordingSetting => "Proxy Recording",
        DevicePropertyCode::RecordingSettingFileName => "Rec File Name",
        DevicePropertyCode::MovieRecordingMedia => "Movie Rec Media",
        DevicePropertyCode::TimeCodeFormat => "TC Format",
        DevicePropertyCode::TimeCodeMake => "TC Make",
        DevicePropertyCode::TimeCodePreset => "TC Preset",
        DevicePropertyCode::TimeCodePresetResetEnableStatus => "TC Reset Status",
        DevicePropertyCode::TimeCodeRun => "TC Run",
        DevicePropertyCode::UserBitTimeRec => "UB Time",
        DevicePropertyCode::RecorderMainStatus => "Main Rec Status",
        DevicePropertyCode::RecorderProxyStatus => "Proxy Rec Status",
        DevicePropertyCode::RecorderStartMain => "Start Rec",
        DevicePropertyCode::RecorderStartProxy => "Start Proxy Rec",
        DevicePropertyCode::SQModeSetting => "S&Q Mode",
        DevicePropertyCode::SQRecordingSetting => "S&Q Recording",
        DevicePropertyCode::SQRecordingFrameRateSetting => "S&Q Frame Rate",
        DevicePropertyCode::SQFrameRate => "S&Q Playback Rate",
        DevicePropertyCode::LogShootingMode => "Log Mode",
        DevicePropertyCode::LogShootingModeColorGamut => "Log Color Gamut",
        DevicePropertyCode::RecordingSelfTimer => "Self-Timer",
        DevicePropertyCode::RecordingSelfTimerContinuous => "Self-Timer Cont.",
        DevicePropertyCode::RecordingSelfTimerCountTime => "Self-Timer Time",
        DevicePropertyCode::RecordingSelfTimerStatus => "Self-Timer State",
        DevicePropertyCode::MovieRecordingFrameRateProxySetting => "Proxy Frame Rate",
        DevicePropertyCode::MovieProxyFileFormat => "Proxy Format",
        DevicePropertyCode::MovieRecordingFileNumber => "Movie File #",
        DevicePropertyCode::MoviePlayingState => "Playback State",
        DevicePropertyCode::MoviePlayingSpeed => "Playback Speed",
        DevicePropertyCode::MediaSLOT1RecordingAvailableType => "Slot 1 Rec Type",
        DevicePropertyCode::MediaSLOT2RecordingAvailableType => "Slot 2 Rec Type",
        DevicePropertyCode::MediaSLOT3RecordingAvailableType => "Slot 3 Rec Type",
        DevicePropertyCode::MovieFTPAutoTransferTarget => "Movie FTP Auto",
        DevicePropertyCode::MovieFTPTransferTarget => "Movie FTP Target",
        DevicePropertyCode::MovieForwardButton => "Forward Button",
        DevicePropertyCode::MovieHDMIOutput4KSetting => "HDMI 4K Output",
        DevicePropertyCode::MovieHDMIOutputAudioCH => "HDMI Audio Channels",
        DevicePropertyCode::MovieHDMIOutputColorGamutForRAWOut => "HDMI RAW Color Gamut",
        DevicePropertyCode::MovieHDMIOutputRAW => "HDMI RAW Output",
        DevicePropertyCode::MovieHDMIOutputRawSetting => "HDMI RAW Setting",
        DevicePropertyCode::MovieHDMIOutputRecControl => "HDMI Rec Control",
        DevicePropertyCode::MovieHDMIOutputRecMedia => "HDMI Rec Media",
        DevicePropertyCode::MovieHDMIOutputResolution => "HDMI Resolution",
        DevicePropertyCode::MovieHDMIOutputTimeCode => "HDMI Timecode",
        DevicePropertyCode::MovieImageStabilizationLevel => "Movie Stabilization Level",
        DevicePropertyCode::MovieImageStabilizationSteadyShot => "Movie SteadyShot",
        DevicePropertyCode::MovieIntervalRecCountDownIntervalTime => "Interval Countdown",
        DevicePropertyCode::MovieIntervalRecFrameRateSetting => "Interval Frame Rate",
        DevicePropertyCode::MovieIntervalRecFrames => "Interval Frames",
        DevicePropertyCode::MovieIntervalRecIntervalTime => "Interval Time",
        DevicePropertyCode::MovieIntervalRecRecordingDuration => "Interval Duration",
        DevicePropertyCode::MovieIntervalRecRecordingSetting => "Interval Rec Setting",
        DevicePropertyCode::MovieNextButton => "Next Button",
        DevicePropertyCode::MoviePlayButton => "Play Button",
        DevicePropertyCode::MoviePlayPauseButton => "Play/Pause Button",
        DevicePropertyCode::MoviePlayStopButton => "Stop Button",
        DevicePropertyCode::MoviePrevButton => "Previous Button",
        DevicePropertyCode::MovieQualityFullAutoMode => "Full Auto Quality",
        DevicePropertyCode::MovieRecButtonToggleEnableStatus => "Rec Button Toggle",
        DevicePropertyCode::MovieRecReviewButton => "Rec Review Button",
        DevicePropertyCode::MovieRecReviewPlayingState => "Rec Review State",
        DevicePropertyCode::MovieRecordingFrameRateRTSPSetting => "RTSP Frame Rate",
        DevicePropertyCode::MovieRecordingResolutionForRAW => "RAW Resolution",
        DevicePropertyCode::MovieRecordingResolutionForRTSP => "RTSP Resolution",
        DevicePropertyCode::MovieRewindButton => "Rewind Button",
        DevicePropertyCode::MovieShootingModeTargetDisplay => "Target Display Mode",
        DevicePropertyCode::VideoStreamMovieRecPermission => "Stream Rec Permission",
        DevicePropertyCode::EframingRecordingImageCrop => "E-Framing Image Crop",
        DevicePropertyCode::PictureCacheRecSetting => "Cache Rec",
        DevicePropertyCode::PictureCacheRecSizeAndTime => "Cache Size/Time",
        DevicePropertyCode::DifferentSetForSQMovie => "S&Q Movie Set",
        DevicePropertyCode::PlaybackContentsRecordingDateTime => "Rec DateTime",
        DevicePropertyCode::PlaybackContentsRecordingFileFormat => "Rec File Format",
        DevicePropertyCode::PlaybackContentsRecordingFrameRate => "Rec FPS",
        DevicePropertyCode::PlaybackContentsRecordingResolution => "Rec Res",
        DevicePropertyCode::RecorderControlProxySetting => "Proxy Ctrl",
        DevicePropertyCode::RecordingFileNumber => "File #",
        DevicePropertyCode::RecordingFolderFormat => "Folder Fmt",
        DevicePropertyCode::SimulRecSettingMovieRecButton => "Simul Rec Btn",
        DevicePropertyCode::ValidRecordingVideoFormat => "Valid Rec Fmt",
        DevicePropertyCode::VideoRecordingFormatBitrateSetting => "Bitrate Set",
        DevicePropertyCode::VideoRecordingFormatQuality => "Quality Set",
        _ => code.name(),
    }
}

pub fn value_type(code: DevicePropertyCode) -> Option<PropertyValueType> {
    use PropertyValueType as V;

    Some(match code {
        DevicePropertyCode::AudioRecording => V::OnOff,
        DevicePropertyCode::MovieRecordingSetting => V::MovieQuality,
        DevicePropertyCode::MovieFileFormat | DevicePropertyCode::MovieProxyFileFormat => {
            V::MovieFileFormat
        }
        DevicePropertyCode::RecordingSelfTimer
        | DevicePropertyCode::RecordingSelfTimerContinuous
        | DevicePropertyCode::RecordingSelfTimerCountTime
        | DevicePropertyCode::RecordingSelfTimerStatus
        | DevicePropertyCode::MovieRecordingResolutionForMain
        | DevicePropertyCode::MovieRecordingResolutionForProxy
        | DevicePropertyCode::MovieRecordingFrameRateProxySetting
        | DevicePropertyCode::RecordingMedia
        | DevicePropertyCode::MovieRecordingMedia
        | DevicePropertyCode::RecordingState
        | DevicePropertyCode::RecorderMainStatus
        | DevicePropertyCode::RecorderProxyStatus
        | DevicePropertyCode::TimeCodeFormat
        | DevicePropertyCode::TimeCodeMake
        | DevicePropertyCode::TimeCodePreset
        | DevicePropertyCode::TimeCodeRun
        | DevicePropertyCode::MovieRecordingFileNumber
        | DevicePropertyCode::MoviePlayingState
        | DevicePropertyCode::MoviePlayingSpeed
        | DevicePropertyCode::MediaSLOT1RecordingAvailableType
        | DevicePropertyCode::MediaSLOT2RecordingAvailableType
        | DevicePropertyCode::MediaSLOT3RecordingAvailableType
        | DevicePropertyCode::MovieRecordingFrameRateSetting => V::Integer,
        _ => return None,
    })
}
