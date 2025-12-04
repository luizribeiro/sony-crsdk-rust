//! Formatting functions for camera property values
//!
//! This module provides human-readable display formatting for raw SDK property values.

use std::fmt;

use crate::property::{
    DriveMode, ExposureProgram, FlashMode, FocusArea, FocusMode, MeteringMode, WhiteBalance,
};

/// Format aperture value from SDK raw format to human-readable string.
///
/// The SDK represents aperture as an integer where the value is multiplied by 100.
/// For example:
/// - 280 → "f/2.8"
/// - 140 → "f/1.4"
/// - 1100 → "f/11"
pub fn format_aperture(raw: u64) -> String {
    if raw == 0 {
        return "--".to_string();
    }
    let fnum = raw as f64 / 100.0;
    if fnum >= 10.0 {
        format!("f/{:.0}", fnum)
    } else {
        format!("f/{:.1}", fnum)
    }
}

/// Parse aperture from display string back to raw SDK value.
pub fn parse_aperture(display: &str) -> Option<u64> {
    let s = display.trim_start_matches("f/").trim_start_matches("F/");
    let fnum: f64 = s.parse().ok()?;
    Some((fnum * 100.0).round() as u64)
}

/// Format shutter speed value from SDK raw format to human-readable string.
///
/// The SDK uses a special encoding for shutter speeds. The upper 16 bits represent
/// the numerator and lower 16 bits represent the denominator of a fraction.
/// For speeds >= 1 second, only numerator is used.
///
/// Examples:
/// - 0x0001_0C80 (1/3200) → "1/3200"
/// - 0x0001_0064 (1/100) → "1/100"
/// - 0x0002_0000 (2 seconds) → "2\""
/// - 0x0001_0000 (1 second) → "1\""
pub fn format_shutter_speed(raw: u64) -> String {
    if raw == 0 {
        return "--".to_string();
    }

    let numerator = ((raw >> 16) & 0xFFFF) as u32;
    let denominator = (raw & 0xFFFF) as u32;

    if denominator == 0 || denominator == 1 {
        if numerator == 0 {
            "--".to_string()
        } else {
            format!("{}\"", numerator)
        }
    } else if numerator == 1 {
        format!("1/{}", denominator)
    } else {
        format!("{}/{}", numerator, denominator)
    }
}

/// Parse shutter speed from display string back to raw SDK value.
pub fn parse_shutter_speed(display: &str) -> Option<u64> {
    let s = display.trim();
    if s.ends_with('"') {
        let secs: u32 = s.trim_end_matches('"').parse().ok()?;
        Some(((secs as u64) << 16) | 1)
    } else if s.starts_with("1/") {
        let denom: u32 = s.trim_start_matches("1/").parse().ok()?;
        Some((1u64 << 16) | denom as u64)
    } else if s.contains('/') {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() == 2 {
            let num: u32 = parts[0].parse().ok()?;
            let denom: u32 = parts[1].parse().ok()?;
            Some(((num as u64) << 16) | denom as u64)
        } else {
            None
        }
    } else {
        None
    }
}

/// Format ISO value to human-readable string.
///
/// The SDK represents ISO directly as an integer.
/// Examples:
/// - 100 → "ISO 100"
/// - 3200 → "ISO 3200"
pub fn format_iso(raw: u64) -> String {
    if raw == 0 {
        return "--".to_string();
    }
    format!("ISO {}", raw)
}

/// Format ISO value without prefix for compact display.
pub fn format_iso_compact(raw: u64) -> String {
    if raw == 0 {
        return "--".to_string();
    }
    format!("{}", raw)
}

/// Parse ISO from display string back to raw SDK value.
pub fn parse_iso(display: &str) -> Option<u64> {
    let s = display.trim().trim_start_matches("ISO").trim();
    s.parse().ok()
}

/// Format exposure compensation value to human-readable string.
///
/// The SDK represents exposure compensation in 1/3 EV steps as integers.
/// For example:
/// - 0 → "0.0"
/// - 3 → "+1.0"
/// - -3 → "-1.0"
/// - 1 → "+0.3"
pub fn format_exposure_comp(raw: i64) -> String {
    let ev = raw as f64 / 3.0;
    if ev == 0.0 {
        "0.0".to_string()
    } else if ev > 0.0 {
        format!("+{:.1}", ev)
    } else {
        format!("{:.1}", ev)
    }
}

/// Parse exposure compensation from display string back to raw SDK value.
pub fn parse_exposure_comp(display: &str) -> Option<i64> {
    let s = display.trim().trim_start_matches('+');
    let ev: f64 = s.parse().ok()?;
    Some((ev * 3.0).round() as i64)
}

/// Format color temperature in Kelvin.
pub fn format_color_temp(raw: u64) -> String {
    if raw == 0 {
        return "--".to_string();
    }
    format!("{}K", raw)
}

impl fmt::Display for FocusMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Manual => "MF",
            Self::AfSingle => "AF-S",
            Self::AfContinuous => "AF-C",
            Self::AfAutomatic => "AF-A",
            Self::AfDeepLearning => "AF-D",
            Self::DirectManual => "DMF",
            Self::PresetFocus => "PF",
        };
        write!(f, "{}", s)
    }
}

impl FocusMode {
    /// Get the full descriptive name
    pub fn description(&self) -> &'static str {
        match self {
            Self::Manual => "Manual Focus",
            Self::AfSingle => "Single-shot AF",
            Self::AfContinuous => "Continuous AF",
            Self::AfAutomatic => "Automatic AF",
            Self::AfDeepLearning => "Deep Learning AF",
            Self::DirectManual => "Direct Manual Focus",
            Self::PresetFocus => "Preset Focus",
        }
    }
}

impl fmt::Display for WhiteBalance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Auto => "AWB",
            Self::Daylight => "Daylight",
            Self::Shade => "Shade",
            Self::Cloudy => "Cloudy",
            Self::Tungsten => "Tungsten",
            Self::Fluorescent => "Fluorescent",
            Self::FluorescentWarmWhite => "Fluor. Warm",
            Self::FluorescentCoolWhite => "Fluor. Cool",
            Self::FluorescentDayWhite => "Fluor. Day",
            Self::FluorescentDaylight => "Fluor. Daylight",
            Self::Flash => "Flash",
            Self::UnderwaterAuto => "Underwater",
            Self::ColorTemp => "Color Temp",
            Self::Custom1 => "Custom 1",
            Self::Custom2 => "Custom 2",
            Self::Custom3 => "Custom 3",
            Self::Custom => "Custom",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for ExposureProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Manual => "M",
            Self::ProgramAuto => "P",
            Self::AperturePriority => "A",
            Self::ShutterPriority => "S",
            Self::ProgramCreative => "P Creative",
            Self::ProgramAction => "P Action",
            Self::Portrait => "Portrait",
            Self::Auto => "Auto",
            Self::AutoPlus => "Auto+",
            Self::PA => "P(A)",
            Self::PS => "P(S)",
            Self::SportsAction => "Sports",
            Self::Sunset => "Sunset",
            Self::Night => "Night",
            Self::Landscape => "Landscape",
            Self::Macro => "Macro",
            Self::HandheldTwilight => "HH Twilight",
            Self::NightPortrait => "Night Portrait",
            Self::AntiMotionBlur => "Anti Blur",
            Self::Pet => "Pet",
            Self::Gourmet => "Gourmet",
            Self::Fireworks => "Fireworks",
            Self::HighSensitivity => "High ISO",
            Self::MemoryRecall => "MR",
            Self::ContinuousPriorityAE8 => "Cont. AE 8",
            Self::ContinuousPriorityAE10 => "Cont. AE 10",
            Self::ContinuousPriorityAE12 => "Cont. AE 12",
            Self::SweepPanorama3D => "3D Panorama",
            Self::SweepPanorama => "Panorama",
            Self::MovieP => "Movie P",
            Self::MovieA => "Movie A",
            Self::MovieS => "Movie S",
            Self::MovieM => "Movie M",
            Self::MovieAuto => "Movie Auto",
            Self::MovieF => "Movie F",
            Self::MovieSQMotionP => "S&Q P",
            Self::MovieSQMotionA => "S&Q A",
            Self::MovieSQMotionS => "S&Q S",
            Self::MovieSQMotionM => "S&Q M",
            Self::MovieSQMotionAuto => "S&Q Auto",
            Self::MovieSQMotionF => "S&Q F",
            Self::FlashOff => "Flash Off",
            Self::PictureEffect => "Picture Effect",
            Self::HiFrameRateP => "HFR P",
            Self::HiFrameRateA => "HFR A",
            Self::HiFrameRateS => "HFR S",
            Self::HiFrameRateM => "HFR M",
            Self::SQMotionP => "S&Q P",
            Self::SQMotionA => "S&Q A",
            Self::SQMotionS => "S&Q S",
            Self::SQMotionM => "S&Q M",
            Self::Movie => "Movie",
            Self::Still => "Still",
            Self::MovieFMode => "Movie F",
            Self::MovieIntervalRecF => "Interval F",
            Self::MovieIntervalRecP => "Interval P",
            Self::MovieIntervalRecA => "Interval A",
            Self::MovieIntervalRecS => "Interval S",
            Self::MovieIntervalRecM => "Interval M",
            Self::MovieIntervalRecAuto => "Interval Auto",
        };
        write!(f, "{}", s)
    }
}

impl ExposureProgram {
    /// Check if this is a manual exposure mode (user controls shutter and aperture)
    pub fn is_manual(&self) -> bool {
        matches!(
            self,
            Self::Manual
                | Self::MovieM
                | Self::SQMotionM
                | Self::MovieSQMotionM
                | Self::HiFrameRateM
                | Self::MovieIntervalRecM
        )
    }

    /// Check if this is an aperture priority mode
    pub fn is_aperture_priority(&self) -> bool {
        matches!(
            self,
            Self::AperturePriority
                | Self::MovieA
                | Self::SQMotionA
                | Self::MovieSQMotionA
                | Self::HiFrameRateA
                | Self::MovieIntervalRecA
        )
    }

    /// Check if this is a shutter priority mode
    pub fn is_shutter_priority(&self) -> bool {
        matches!(
            self,
            Self::ShutterPriority
                | Self::MovieS
                | Self::SQMotionS
                | Self::MovieSQMotionS
                | Self::HiFrameRateS
                | Self::MovieIntervalRecS
        )
    }

    /// Check if this is a program auto mode
    pub fn is_program_auto(&self) -> bool {
        matches!(
            self,
            Self::ProgramAuto
                | Self::MovieP
                | Self::SQMotionP
                | Self::MovieSQMotionP
                | Self::HiFrameRateP
                | Self::MovieIntervalRecP
                | Self::PA
                | Self::PS
                | Self::ProgramCreative
                | Self::ProgramAction
        )
    }

    /// Check if this is a fully automatic mode
    pub fn is_full_auto(&self) -> bool {
        matches!(
            self,
            Self::Auto
                | Self::AutoPlus
                | Self::MovieAuto
                | Self::MovieSQMotionAuto
                | Self::MovieIntervalRecAuto
        )
    }
}

impl fmt::Display for DriveMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Single => "Single",
            Self::ContinuousHi => "Cont. Hi",
            Self::ContinuousHiPlus => "Cont. Hi+",
            Self::ContinuousHiLive => "Cont. Hi Live",
            Self::ContinuousLo => "Cont. Lo",
            Self::Continuous => "Continuous",
            Self::ContinuousSpeedPriority => "Cont. Speed",
            Self::ContinuousMid => "Cont. Mid",
            Self::ContinuousMidLive => "Cont. Mid Live",
            Self::ContinuousLoLive => "Cont. Lo Live",
            Self::SingleBurstShootingLo => "Burst Lo",
            Self::SingleBurstShootingMid => "Burst Mid",
            Self::SingleBurstShootingHi => "Burst Hi",
            Self::FocusBracket => "Focus Bracket",
            Self::Timelapse => "Timelapse",
            Self::Timer2s => "Timer 2s",
            Self::Timer5s => "Timer 5s",
            Self::Timer10s => "Timer 10s",

            // Continuous brackets
            Self::ContinuousBracket03Ev3Pics => "C.BRK 0.3EV 3",
            Self::ContinuousBracket03Ev5Pics => "C.BRK 0.3EV 5",
            Self::ContinuousBracket03Ev9Pics => "C.BRK 0.3EV 9",
            Self::ContinuousBracket03Ev2PicsPlus => "C.BRK 0.3EV 2+",
            Self::ContinuousBracket03Ev2PicsMinus => "C.BRK 0.3EV 2-",
            Self::ContinuousBracket03Ev7Pics => "C.BRK 0.3EV 7",
            Self::ContinuousBracket05Ev3Pics => "C.BRK 0.5EV 3",
            Self::ContinuousBracket05Ev5Pics => "C.BRK 0.5EV 5",
            Self::ContinuousBracket05Ev9Pics => "C.BRK 0.5EV 9",
            Self::ContinuousBracket05Ev2PicsPlus => "C.BRK 0.5EV 2+",
            Self::ContinuousBracket05Ev2PicsMinus => "C.BRK 0.5EV 2-",
            Self::ContinuousBracket05Ev7Pics => "C.BRK 0.5EV 7",
            Self::ContinuousBracket07Ev3Pics => "C.BRK 0.7EV 3",
            Self::ContinuousBracket07Ev5Pics => "C.BRK 0.7EV 5",
            Self::ContinuousBracket07Ev9Pics => "C.BRK 0.7EV 9",
            Self::ContinuousBracket07Ev2PicsPlus => "C.BRK 0.7EV 2+",
            Self::ContinuousBracket07Ev2PicsMinus => "C.BRK 0.7EV 2-",
            Self::ContinuousBracket07Ev7Pics => "C.BRK 0.7EV 7",
            Self::ContinuousBracket10Ev3Pics => "C.BRK 1.0EV 3",
            Self::ContinuousBracket10Ev5Pics => "C.BRK 1.0EV 5",
            Self::ContinuousBracket10Ev9Pics => "C.BRK 1.0EV 9",
            Self::ContinuousBracket10Ev2PicsPlus => "C.BRK 1.0EV 2+",
            Self::ContinuousBracket10Ev2PicsMinus => "C.BRK 1.0EV 2-",
            Self::ContinuousBracket10Ev7Pics => "C.BRK 1.0EV 7",
            Self::ContinuousBracket13Ev2PicsPlus => "C.BRK 1.3EV 2+",
            Self::ContinuousBracket13Ev2PicsMinus => "C.BRK 1.3EV 2-",
            Self::ContinuousBracket13Ev3Pics => "C.BRK 1.3EV 3",
            Self::ContinuousBracket13Ev5Pics => "C.BRK 1.3EV 5",
            Self::ContinuousBracket13Ev7Pics => "C.BRK 1.3EV 7",
            Self::ContinuousBracket15Ev2PicsPlus => "C.BRK 1.5EV 2+",
            Self::ContinuousBracket15Ev2PicsMinus => "C.BRK 1.5EV 2-",
            Self::ContinuousBracket15Ev3Pics => "C.BRK 1.5EV 3",
            Self::ContinuousBracket15Ev5Pics => "C.BRK 1.5EV 5",
            Self::ContinuousBracket15Ev7Pics => "C.BRK 1.5EV 7",
            Self::ContinuousBracket17Ev2PicsPlus => "C.BRK 1.7EV 2+",
            Self::ContinuousBracket17Ev2PicsMinus => "C.BRK 1.7EV 2-",
            Self::ContinuousBracket17Ev3Pics => "C.BRK 1.7EV 3",
            Self::ContinuousBracket17Ev5Pics => "C.BRK 1.7EV 5",
            Self::ContinuousBracket17Ev7Pics => "C.BRK 1.7EV 7",
            Self::ContinuousBracket20Ev3Pics => "C.BRK 2.0EV 3",
            Self::ContinuousBracket20Ev5Pics => "C.BRK 2.0EV 5",
            Self::ContinuousBracket20Ev2PicsPlus => "C.BRK 2.0EV 2+",
            Self::ContinuousBracket20Ev2PicsMinus => "C.BRK 2.0EV 2-",
            Self::ContinuousBracket20Ev7Pics => "C.BRK 2.0EV 7",
            Self::ContinuousBracket23Ev2PicsPlus => "C.BRK 2.3EV 2+",
            Self::ContinuousBracket23Ev2PicsMinus => "C.BRK 2.3EV 2-",
            Self::ContinuousBracket23Ev3Pics => "C.BRK 2.3EV 3",
            Self::ContinuousBracket23Ev5Pics => "C.BRK 2.3EV 5",
            Self::ContinuousBracket25Ev2PicsPlus => "C.BRK 2.5EV 2+",
            Self::ContinuousBracket25Ev2PicsMinus => "C.BRK 2.5EV 2-",
            Self::ContinuousBracket25Ev3Pics => "C.BRK 2.5EV 3",
            Self::ContinuousBracket25Ev5Pics => "C.BRK 2.5EV 5",
            Self::ContinuousBracket27Ev2PicsPlus => "C.BRK 2.7EV 2+",
            Self::ContinuousBracket27Ev2PicsMinus => "C.BRK 2.7EV 2-",
            Self::ContinuousBracket27Ev3Pics => "C.BRK 2.7EV 3",
            Self::ContinuousBracket27Ev5Pics => "C.BRK 2.7EV 5",
            Self::ContinuousBracket30Ev3Pics => "C.BRK 3.0EV 3",
            Self::ContinuousBracket30Ev5Pics => "C.BRK 3.0EV 5",
            Self::ContinuousBracket30Ev2PicsPlus => "C.BRK 3.0EV 2+",
            Self::ContinuousBracket30Ev2PicsMinus => "C.BRK 3.0EV 2-",

            // Single brackets
            Self::SingleBracket03Ev3Pics => "S.BRK 0.3EV 3",
            Self::SingleBracket03Ev5Pics => "S.BRK 0.3EV 5",
            Self::SingleBracket03Ev9Pics => "S.BRK 0.3EV 9",
            Self::SingleBracket03Ev2PicsPlus => "S.BRK 0.3EV 2+",
            Self::SingleBracket03Ev2PicsMinus => "S.BRK 0.3EV 2-",
            Self::SingleBracket03Ev7Pics => "S.BRK 0.3EV 7",
            Self::SingleBracket05Ev3Pics => "S.BRK 0.5EV 3",
            Self::SingleBracket05Ev5Pics => "S.BRK 0.5EV 5",
            Self::SingleBracket05Ev9Pics => "S.BRK 0.5EV 9",
            Self::SingleBracket05Ev2PicsPlus => "S.BRK 0.5EV 2+",
            Self::SingleBracket05Ev2PicsMinus => "S.BRK 0.5EV 2-",
            Self::SingleBracket05Ev7Pics => "S.BRK 0.5EV 7",
            Self::SingleBracket07Ev3Pics => "S.BRK 0.7EV 3",
            Self::SingleBracket07Ev5Pics => "S.BRK 0.7EV 5",
            Self::SingleBracket07Ev9Pics => "S.BRK 0.7EV 9",
            Self::SingleBracket07Ev2PicsPlus => "S.BRK 0.7EV 2+",
            Self::SingleBracket07Ev2PicsMinus => "S.BRK 0.7EV 2-",
            Self::SingleBracket07Ev7Pics => "S.BRK 0.7EV 7",
            Self::SingleBracket10Ev3Pics => "S.BRK 1.0EV 3",
            Self::SingleBracket10Ev5Pics => "S.BRK 1.0EV 5",
            Self::SingleBracket10Ev9Pics => "S.BRK 1.0EV 9",
            Self::SingleBracket10Ev2PicsPlus => "S.BRK 1.0EV 2+",
            Self::SingleBracket10Ev2PicsMinus => "S.BRK 1.0EV 2-",
            Self::SingleBracket10Ev7Pics => "S.BRK 1.0EV 7",
            Self::SingleBracket13Ev2PicsPlus => "S.BRK 1.3EV 2+",
            Self::SingleBracket13Ev2PicsMinus => "S.BRK 1.3EV 2-",
            Self::SingleBracket13Ev3Pics => "S.BRK 1.3EV 3",
            Self::SingleBracket13Ev5Pics => "S.BRK 1.3EV 5",
            Self::SingleBracket13Ev7Pics => "S.BRK 1.3EV 7",
            Self::SingleBracket15Ev2PicsPlus => "S.BRK 1.5EV 2+",
            Self::SingleBracket15Ev2PicsMinus => "S.BRK 1.5EV 2-",
            Self::SingleBracket15Ev3Pics => "S.BRK 1.5EV 3",
            Self::SingleBracket15Ev5Pics => "S.BRK 1.5EV 5",
            Self::SingleBracket15Ev7Pics => "S.BRK 1.5EV 7",
            Self::SingleBracket17Ev2PicsPlus => "S.BRK 1.7EV 2+",
            Self::SingleBracket17Ev2PicsMinus => "S.BRK 1.7EV 2-",
            Self::SingleBracket17Ev3Pics => "S.BRK 1.7EV 3",
            Self::SingleBracket17Ev5Pics => "S.BRK 1.7EV 5",
            Self::SingleBracket17Ev7Pics => "S.BRK 1.7EV 7",
            Self::SingleBracket20Ev3Pics => "S.BRK 2.0EV 3",
            Self::SingleBracket20Ev5Pics => "S.BRK 2.0EV 5",
            Self::SingleBracket20Ev2PicsPlus => "S.BRK 2.0EV 2+",
            Self::SingleBracket20Ev2PicsMinus => "S.BRK 2.0EV 2-",
            Self::SingleBracket20Ev7Pics => "S.BRK 2.0EV 7",
            Self::SingleBracket23Ev2PicsPlus => "S.BRK 2.3EV 2+",
            Self::SingleBracket23Ev2PicsMinus => "S.BRK 2.3EV 2-",
            Self::SingleBracket23Ev3Pics => "S.BRK 2.3EV 3",
            Self::SingleBracket23Ev5Pics => "S.BRK 2.3EV 5",
            Self::SingleBracket25Ev2PicsPlus => "S.BRK 2.5EV 2+",
            Self::SingleBracket25Ev2PicsMinus => "S.BRK 2.5EV 2-",
            Self::SingleBracket25Ev3Pics => "S.BRK 2.5EV 3",
            Self::SingleBracket25Ev5Pics => "S.BRK 2.5EV 5",
            Self::SingleBracket27Ev2PicsPlus => "S.BRK 2.7EV 2+",
            Self::SingleBracket27Ev2PicsMinus => "S.BRK 2.7EV 2-",
            Self::SingleBracket27Ev3Pics => "S.BRK 2.7EV 3",
            Self::SingleBracket27Ev5Pics => "S.BRK 2.7EV 5",
            Self::SingleBracket30Ev3Pics => "S.BRK 3.0EV 3",
            Self::SingleBracket30Ev5Pics => "S.BRK 3.0EV 5",
            Self::SingleBracket30Ev2PicsPlus => "S.BRK 3.0EV 2+",
            Self::SingleBracket30Ev2PicsMinus => "S.BRK 3.0EV 2-",

            // Other modes
            Self::WbBracketLo => "WB BRK Lo",
            Self::WbBracketHi => "WB BRK Hi",
            Self::DroBracketLo => "DRO BRK Lo",
            Self::DroBracketHi => "DRO BRK Hi",
            Self::ContinuousTimer3Pics => "Cont. Timer 3",
            Self::ContinuousTimer5Pics => "Cont. Timer 5",
            Self::ContinuousTimer2s3Pics => "Timer 2s 3pics",
            Self::ContinuousTimer2s5Pics => "Timer 2s 5pics",
            Self::ContinuousTimer5s3Pics => "Timer 5s 3pics",
            Self::ContinuousTimer5s5Pics => "Timer 5s 5pics",
            Self::LpfBracket => "LPF Bracket",
            Self::RemoteCommander => "Remote",
            Self::MirrorUp => "Mirror Up",
            Self::SelfPortrait1 => "Self Portrait 1",
            Self::SelfPortrait2 => "Self Portrait 2",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for MeteringMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Average => "Average",
            Self::CenterWeightedAverage => "Center Avg",
            Self::MultiSpot => "Multi Spot",
            Self::CenterSpot => "Center Spot",
            Self::Multi => "Multi",
            Self::CenterWeighted => "Center",
            Self::EntireScreenAverage => "Screen Avg",
            Self::SpotStandard => "Spot",
            Self::SpotLarge => "Spot Large",
            Self::HighLightWeighted => "Highlight",
            Self::Standard => "Standard",
            Self::Backlight => "Backlight",
            Self::Spotlight => "Spotlight",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for FlashMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Auto => "Auto",
            Self::Off => "Off",
            Self::Fill => "Fill",
            Self::ExternalSync => "Ext. Sync",
            Self::SlowSync => "Slow Sync",
            Self::RearSync => "Rear Sync",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for FocusArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Unknown => "Unknown",
            Self::Wide => "Wide",
            Self::Zone => "Zone",
            Self::Center => "Center",
            Self::FlexibleSpotS => "Spot S",
            Self::FlexibleSpotM => "Spot M",
            Self::FlexibleSpotL => "Spot L",
            Self::ExpandFlexibleSpot => "Expand Spot",
            Self::FlexibleSpot => "Spot",
            Self::TrackingWide => "Track Wide",
            Self::TrackingZone => "Track Zone",
            Self::TrackingCenter => "Track Center",
            Self::TrackingFlexibleSpotS => "Track Spot S",
            Self::TrackingFlexibleSpotM => "Track Spot M",
            Self::TrackingFlexibleSpotL => "Track Spot L",
            Self::TrackingExpandFlexibleSpot => "Track Expand",
            Self::TrackingFlexibleSpot => "Track Spot",
            Self::FlexibleSpotXS => "Spot XS",
            Self::FlexibleSpotXL => "Spot XL",
            Self::FlexibleSpotFreeSize1 => "Spot Free 1",
            Self::FlexibleSpotFreeSize2 => "Spot Free 2",
            Self::FlexibleSpotFreeSize3 => "Spot Free 3",
            Self::TrackingFlexibleSpotXS => "Track Spot XS",
            Self::TrackingFlexibleSpotXL => "Track Spot XL",
            Self::TrackingFlexibleSpotFreeSize1 => "Track Free 1",
            Self::TrackingFlexibleSpotFreeSize2 => "Track Free 2",
            Self::TrackingFlexibleSpotFreeSize3 => "Track Free 3",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_aperture() {
        assert_eq!(format_aperture(140), "f/1.4");
        assert_eq!(format_aperture(280), "f/2.8");
        assert_eq!(format_aperture(400), "f/4.0");
        assert_eq!(format_aperture(560), "f/5.6");
        assert_eq!(format_aperture(800), "f/8.0");
        assert_eq!(format_aperture(1100), "f/11");
        assert_eq!(format_aperture(1600), "f/16");
        assert_eq!(format_aperture(2200), "f/22");
        assert_eq!(format_aperture(0), "--");
    }

    #[test]
    fn test_parse_aperture() {
        assert_eq!(parse_aperture("f/2.8"), Some(280));
        assert_eq!(parse_aperture("f/11"), Some(1100));
        assert_eq!(parse_aperture("F/1.4"), Some(140));
    }

    #[test]
    fn test_format_shutter_speed() {
        // 1/200 = numerator 1, denominator 200
        assert_eq!(format_shutter_speed((1u64 << 16) | 200), "1/200");
        // 1/3200
        assert_eq!(format_shutter_speed((1u64 << 16) | 3200), "1/3200");
        // 2 seconds
        assert_eq!(format_shutter_speed((2u64 << 16) | 1), "2\"");
        // 1 second
        assert_eq!(format_shutter_speed((1u64 << 16) | 1), "1\"");
        assert_eq!(format_shutter_speed(0), "--");
    }

    #[test]
    fn test_parse_shutter_speed() {
        assert_eq!(parse_shutter_speed("1/200"), Some((1u64 << 16) | 200));
        assert_eq!(parse_shutter_speed("2\""), Some((2u64 << 16) | 1));
    }

    #[test]
    fn test_format_iso() {
        assert_eq!(format_iso(100), "ISO 100");
        assert_eq!(format_iso(3200), "ISO 3200");
        assert_eq!(format_iso(0), "--");
    }

    #[test]
    fn test_format_iso_compact() {
        assert_eq!(format_iso_compact(100), "100");
        assert_eq!(format_iso_compact(3200), "3200");
    }

    #[test]
    fn test_parse_iso() {
        assert_eq!(parse_iso("ISO 100"), Some(100));
        assert_eq!(parse_iso("3200"), Some(3200));
    }

    #[test]
    fn test_format_exposure_comp() {
        assert_eq!(format_exposure_comp(0), "0.0");
        assert_eq!(format_exposure_comp(3), "+1.0");
        assert_eq!(format_exposure_comp(-3), "-1.0");
        assert_eq!(format_exposure_comp(1), "+0.3");
        assert_eq!(format_exposure_comp(-1), "-0.3");
    }

    #[test]
    fn test_parse_exposure_comp() {
        assert_eq!(parse_exposure_comp("0.0"), Some(0));
        assert_eq!(parse_exposure_comp("+1.0"), Some(3));
        assert_eq!(parse_exposure_comp("-1.0"), Some(-3));
    }

    #[test]
    fn test_focus_mode_display() {
        assert_eq!(FocusMode::Manual.to_string(), "MF");
        assert_eq!(FocusMode::AfSingle.to_string(), "AF-S");
        assert_eq!(FocusMode::AfContinuous.to_string(), "AF-C");
        assert_eq!(FocusMode::DirectManual.to_string(), "DMF");
    }

    #[test]
    fn test_white_balance_display() {
        assert_eq!(WhiteBalance::Auto.to_string(), "AWB");
        assert_eq!(WhiteBalance::Daylight.to_string(), "Daylight");
        assert_eq!(WhiteBalance::Shade.to_string(), "Shade");
    }

    #[test]
    fn test_exposure_program_display() {
        assert_eq!(ExposureProgram::Manual.to_string(), "M");
        assert_eq!(ExposureProgram::ProgramAuto.to_string(), "P");
        assert_eq!(ExposureProgram::AperturePriority.to_string(), "A");
        assert_eq!(ExposureProgram::ShutterPriority.to_string(), "S");
        assert_eq!(ExposureProgram::Auto.to_string(), "Auto");
    }

    #[test]
    fn test_exposure_program_helpers() {
        assert!(ExposureProgram::Manual.is_manual());
        assert!(!ExposureProgram::Manual.is_aperture_priority());
        assert!(ExposureProgram::AperturePriority.is_aperture_priority());
        assert!(ExposureProgram::ShutterPriority.is_shutter_priority());
        assert!(ExposureProgram::ProgramAuto.is_program_auto());
        assert!(ExposureProgram::Auto.is_full_auto());
    }

    #[test]
    fn test_drive_mode_display() {
        assert_eq!(DriveMode::Single.to_string(), "Single");
        assert_eq!(DriveMode::ContinuousHi.to_string(), "Cont. Hi");
        assert_eq!(DriveMode::Timer2s.to_string(), "Timer 2s");
    }

    #[test]
    fn test_metering_mode_display() {
        assert_eq!(MeteringMode::Multi.to_string(), "Multi");
        assert_eq!(MeteringMode::CenterWeighted.to_string(), "Center");
        assert_eq!(MeteringMode::SpotStandard.to_string(), "Spot");
    }

    #[test]
    fn test_flash_mode_display() {
        assert_eq!(FlashMode::Auto.to_string(), "Auto");
        assert_eq!(FlashMode::Off.to_string(), "Off");
        assert_eq!(FlashMode::SlowSync.to_string(), "Slow Sync");
    }

    #[test]
    fn test_focus_area_display() {
        assert_eq!(FocusArea::Wide.to_string(), "Wide");
        assert_eq!(FocusArea::Zone.to_string(), "Zone");
        assert_eq!(FocusArea::Center.to_string(), "Center");
        assert_eq!(FocusArea::TrackingWide.to_string(), "Track Wide");
    }
}
