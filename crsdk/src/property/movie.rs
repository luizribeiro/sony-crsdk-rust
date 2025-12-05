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
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::MovieFileFormat => "Movie Format",
        DevicePropertyCode::MovieRecordingSetting => "Movie Quality",
        DevicePropertyCode::MovieRecordingFrameRateSetting => "Movie Frame Rate",
        DevicePropertyCode::MovieRecordingResolutionForMain => "Movie Resolution",
        DevicePropertyCode::MovieRecordingResolutionForProxy => "Proxy Resolution",
        DevicePropertyCode::MovieShootingMode => "Movie Shooting Mode",
        DevicePropertyCode::MovieShootingModeColorGamut => "Movie Color Gamut",
        DevicePropertyCode::RecordingState => "Rec State",
        DevicePropertyCode::RecordingMedia => "Rec Media",
        DevicePropertyCode::ProxyRecordingSetting => "Proxy Recording",
        DevicePropertyCode::RecordingSettingFileName => "Rec File Name",
        DevicePropertyCode::MovieRecordingMedia => "Movie Rec Media",
        DevicePropertyCode::TimeCodeFormat => "TC Format",
        DevicePropertyCode::TimeCodeMake => "TC Make",
        DevicePropertyCode::TimeCodePreset => "TC Preset",
        DevicePropertyCode::TimeCodeRun => "TC Run",
        DevicePropertyCode::RecorderMainStatus => "Main Rec Status",
        DevicePropertyCode::RecorderProxyStatus => "Proxy Rec Status",
        DevicePropertyCode::RecorderStartMain => "Start Rec",
        DevicePropertyCode::RecorderStartProxy => "Start Proxy Rec",
        DevicePropertyCode::SQModeSetting => "S&Q Mode",
        DevicePropertyCode::SQRecordingSetting => "S&Q Recording",
        DevicePropertyCode::SQRecordingFrameRateSetting => "S&Q Frame Rate",
        DevicePropertyCode::SQFrameRate => "S&Q Playback Rate",
        DevicePropertyCode::LogShootingMode => "Log Shooting Mode",
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
        _ => code.name(),
    }
}

pub fn value_type(code: DevicePropertyCode) -> Option<PropertyValueType> {
    use PropertyValueType as V;

    Some(match code {
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
        | DevicePropertyCode::MoviePlayingSpeed => V::Integer,
        _ => return None,
    })
}
