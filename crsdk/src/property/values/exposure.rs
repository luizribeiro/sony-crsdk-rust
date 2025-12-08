//! Exposure-related property value types.

use std::fmt;

use super::super::traits::PropertyValue;
use crate::types::ToCrsdk;

/// Aperture (f-number) value.
///
/// The SDK represents aperture as an integer where the value is multiplied by 100.
/// For example: 280 → f/2.8, 140 → f/1.4, 1100 → f/11
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Aperture(u64);

impl Aperture {
    /// Get the f-number as a floating point value.
    pub fn f_number(&self) -> f64 {
        self.0 as f64 / 100.0
    }
}

impl ToCrsdk<u64> for Aperture {
    fn to_crsdk(&self) -> u64 {
        self.0
    }
}

impl PropertyValue for Aperture {
    fn from_raw(raw: u64) -> Option<Self> {
        if raw == 0 {
            None
        } else {
            Some(Aperture(raw))
        }
    }
}

impl fmt::Display for Aperture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fnum = self.f_number();
        if fnum >= 10.0 {
            write!(f, "f/{:.0}", fnum)
        } else {
            write!(f, "f/{:.1}", fnum)
        }
    }
}

/// Shutter speed value.
///
/// The SDK uses a special encoding: upper 16 bits = numerator, lower 16 bits = denominator.
/// For speeds >= 1 second, denominator is 0 or 1.
///
/// Examples:
/// - 0x0001_00C8 (1/200) → "1/200"
/// - 0x0002_0001 (2 seconds) → "2\""
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShutterSpeed(u64);

impl ShutterSpeed {
    /// Get the numerator of the shutter speed fraction.
    pub fn numerator(&self) -> u32 {
        ((self.0 >> 16) & 0xFFFF) as u32
    }

    /// Get the denominator of the shutter speed fraction.
    pub fn denominator(&self) -> u32 {
        (self.0 & 0xFFFF) as u32
    }

    /// Check if this represents a "bulb" or long exposure (>= 1 second).
    pub fn is_seconds(&self) -> bool {
        let denom = self.denominator();
        denom == 0 || denom == 1
    }
}

impl ToCrsdk<u64> for ShutterSpeed {
    fn to_crsdk(&self) -> u64 {
        self.0
    }
}

impl PropertyValue for ShutterSpeed {
    fn from_raw(raw: u64) -> Option<Self> {
        if raw == 0 {
            None
        } else {
            Some(ShutterSpeed(raw))
        }
    }
}

impl fmt::Display for ShutterSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let numerator = self.numerator();
        let denominator = self.denominator();

        if denominator == 0 || denominator == 1 {
            if numerator == 0 {
                write!(f, "--")
            } else {
                write!(f, "{}\"", numerator)
            }
        } else if numerator == 1 {
            write!(f, "1/{}", denominator)
        } else {
            write!(f, "{}/{}", numerator, denominator)
        }
    }
}

/// ISO sensitivity value.
///
/// The SDK represents ISO directly as an integer (e.g., 100, 800, 3200).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Iso(u64);

impl Iso {
    /// Get the numeric ISO value.
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl ToCrsdk<u64> for Iso {
    fn to_crsdk(&self) -> u64 {
        self.0
    }
}

impl PropertyValue for Iso {
    fn from_raw(raw: u64) -> Option<Self> {
        if raw == 0 {
            None
        } else {
            Some(Iso(raw))
        }
    }
}

impl fmt::Display for Iso {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ISO {}", self.0)
    }
}

/// Exposure compensation value.
///
/// The SDK represents exposure compensation in 1/3 EV steps as integers.
/// For example: 0 → 0.0, 3 → +1.0, -3 → -1.0, 1 → +0.3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExposureComp(i64);

impl ExposureComp {
    /// Get the EV value as a floating point number.
    pub fn ev(&self) -> f64 {
        self.0 as f64 / 3.0
    }

    /// Get the raw step value (in 1/3 EV increments).
    pub fn steps(&self) -> i64 {
        self.0
    }
}

impl ToCrsdk<u64> for ExposureComp {
    fn to_crsdk(&self) -> u64 {
        self.0 as u64
    }
}

impl PropertyValue for ExposureComp {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(ExposureComp(raw as i64))
    }
}

impl fmt::Display for ExposureComp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ev = self.ev();
        if ev == 0.0 {
            write!(f, "0.0")
        } else if ev > 0.0 {
            write!(f, "+{:.1}", ev)
        } else {
            write!(f, "{:.1}", ev)
        }
    }
}

/// Metered manual exposure level.
///
/// This is a signed value representing the exposure meter reading in manual mode.
/// The value is typically in 1/100 EV steps, with range -8000 to +8000.
/// Negative values indicate underexposure, positive values indicate overexposure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeterLevel(i64);

impl MeterLevel {
    /// Get the raw meter level value.
    pub fn value(&self) -> i64 {
        self.0
    }

    /// Get the meter level in EV (value / 100).
    pub fn ev(&self) -> f64 {
        self.0 as f64 / 100.0
    }
}

impl ToCrsdk<u64> for MeterLevel {
    fn to_crsdk(&self) -> u64 {
        self.0 as u64
    }
}

impl PropertyValue for MeterLevel {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(MeterLevel(raw as i64))
    }
}

impl fmt::Display for MeterLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Exposure program mode (shooting mode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
#[non_exhaustive]
pub enum ExposureProgram {
    /// Manual exposure (M)
    Manual = 1,
    /// Program auto (P)
    ProgramAuto = 2,
    /// Aperture priority (A)
    AperturePriority = 3,
    /// Shutter priority (S)
    ShutterPriority = 4,
    /// Program creative
    ProgramCreative = 5,
    /// Program action
    ProgramAction = 6,
    /// Portrait scene
    Portrait = 7,
    /// Full auto
    Auto = 32768,
    /// Auto+ (intelligent auto)
    AutoPlus = 32769,
    /// Program with aperture bias
    PA = 32770,
    /// Program with shutter bias
    PS = 32771,
    /// Sports/action scene
    SportsAction = 32785,
    /// Sunset scene
    Sunset = 32786,
    /// Night scene
    Night = 32787,
    /// Landscape scene
    Landscape = 32788,
    /// Macro scene
    Macro = 32789,
    /// Handheld twilight
    HandheldTwilight = 32790,
    /// Night portrait
    NightPortrait = 32791,
    /// Anti motion blur
    AntiMotionBlur = 32792,
    /// Pet photography
    Pet = 32793,
    /// Food/gourmet
    Gourmet = 32794,
    /// Fireworks
    Fireworks = 32795,
    /// High sensitivity
    HighSensitivity = 32796,
    /// Memory recall
    MemoryRecall = 32800,
    /// Continuous AE (8 shots)
    ContinuousPriorityAE8 = 32816,
    /// Continuous AE (10 shots)
    ContinuousPriorityAE10 = 32817,
    /// Continuous AE (12 shots)
    ContinuousPriorityAE12 = 32818,
    /// 3D sweep panorama
    SweepPanorama3D = 32832,
    /// Sweep panorama
    SweepPanorama = 32833,
    /// Movie program auto
    MovieP = 32848,
    /// Movie aperture priority
    MovieA = 32849,
    /// Movie shutter priority
    MovieS = 32850,
    /// Movie manual
    MovieM = 32851,
    /// Movie auto
    MovieAuto = 32852,
    /// Movie flexible exposure
    MovieF = 32853,
    /// S&Q motion program auto
    MovieSQMotionP = 32854,
    /// S&Q motion aperture priority
    MovieSQMotionA = 32855,
    /// S&Q motion shutter priority
    MovieSQMotionS = 32856,
    /// S&Q motion manual
    MovieSQMotionM = 32857,
    /// S&Q motion auto
    MovieSQMotionAuto = 32858,
    /// S&Q motion flexible
    MovieSQMotionF = 32859,
    /// Flash off scene
    FlashOff = 32864,
    /// Picture effect mode
    PictureEffect = 32865,
    /// High frame rate program
    HiFrameRateP = 32880,
    /// High frame rate aperture
    HiFrameRateA = 32881,
    /// High frame rate shutter
    HiFrameRateS = 32882,
    /// High frame rate manual
    HiFrameRateM = 32883,
    /// S&Q program auto
    SQMotionP = 32896,
    /// S&Q aperture priority
    SQMotionA = 32897,
    /// S&Q shutter priority
    SQMotionS = 32898,
    /// S&Q manual
    SQMotionM = 32899,
    /// Generic movie mode
    Movie = 32900,
    /// Still image mode
    Still = 32901,
    /// Movie F mode
    MovieFMode = 32902,
    /// Interval rec flexible
    MovieIntervalRecF = 32903,
    /// Interval rec program
    MovieIntervalRecP = 32904,
    /// Interval rec aperture
    MovieIntervalRecA = 32905,
    /// Interval rec shutter
    MovieIntervalRecS = 32906,
    /// Interval rec manual
    MovieIntervalRecM = 32907,
    /// Interval rec auto
    MovieIntervalRecAuto = 32908,
}

impl ToCrsdk<u64> for ExposureProgram {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for ExposureProgram {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u32 {
            1 => Self::Manual,
            2 => Self::ProgramAuto,
            3 => Self::AperturePriority,
            4 => Self::ShutterPriority,
            5 => Self::ProgramCreative,
            6 => Self::ProgramAction,
            7 => Self::Portrait,
            32768 => Self::Auto,
            32769 => Self::AutoPlus,
            32770 => Self::PA,
            32771 => Self::PS,
            32785 => Self::SportsAction,
            32786 => Self::Sunset,
            32787 => Self::Night,
            32788 => Self::Landscape,
            32789 => Self::Macro,
            32790 => Self::HandheldTwilight,
            32791 => Self::NightPortrait,
            32792 => Self::AntiMotionBlur,
            32793 => Self::Pet,
            32794 => Self::Gourmet,
            32795 => Self::Fireworks,
            32796 => Self::HighSensitivity,
            32800 => Self::MemoryRecall,
            32816 => Self::ContinuousPriorityAE8,
            32817 => Self::ContinuousPriorityAE10,
            32818 => Self::ContinuousPriorityAE12,
            32832 => Self::SweepPanorama3D,
            32833 => Self::SweepPanorama,
            32848 => Self::MovieP,
            32849 => Self::MovieA,
            32850 => Self::MovieS,
            32851 => Self::MovieM,
            32852 => Self::MovieAuto,
            32853 => Self::MovieF,
            32854 => Self::MovieSQMotionP,
            32855 => Self::MovieSQMotionA,
            32856 => Self::MovieSQMotionS,
            32857 => Self::MovieSQMotionM,
            32858 => Self::MovieSQMotionAuto,
            32859 => Self::MovieSQMotionF,
            32864 => Self::FlashOff,
            32865 => Self::PictureEffect,
            32880 => Self::HiFrameRateP,
            32881 => Self::HiFrameRateA,
            32882 => Self::HiFrameRateS,
            32883 => Self::HiFrameRateM,
            32896 => Self::SQMotionP,
            32897 => Self::SQMotionA,
            32898 => Self::SQMotionS,
            32899 => Self::SQMotionM,
            32900 => Self::Movie,
            32901 => Self::Still,
            32902 => Self::MovieFMode,
            32903 => Self::MovieIntervalRecF,
            32904 => Self::MovieIntervalRecP,
            32905 => Self::MovieIntervalRecA,
            32906 => Self::MovieIntervalRecS,
            32907 => Self::MovieIntervalRecM,
            32908 => Self::MovieIntervalRecAuto,
            _ => return None,
        })
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
    /// Check if this is a manual exposure mode
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

/// Metering mode settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
#[non_exhaustive]
pub enum MeteringMode {
    /// Average metering
    Average = 1,
    /// Center-weighted average
    CenterWeightedAverage = 2,
    /// Multi-spot metering
    MultiSpot = 3,
    /// Center spot metering
    CenterSpot = 4,
    /// Multi-pattern metering
    Multi = 32769,
    /// Center-weighted metering
    CenterWeighted = 32770,
    /// Entire screen average
    EntireScreenAverage = 32771,
    /// Standard spot metering
    SpotStandard = 32772,
    /// Large spot metering
    SpotLarge = 32773,
    /// Highlight-weighted metering
    HighLightWeighted = 32774,
    /// Standard metering
    Standard = 32775,
    /// Backlight compensation
    Backlight = 32776,
    /// Spotlight metering
    Spotlight = 32777,
}

impl ToCrsdk<u64> for MeteringMode {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for MeteringMode {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u16 {
            1 => Self::Average,
            2 => Self::CenterWeightedAverage,
            3 => Self::MultiSpot,
            4 => Self::CenterSpot,
            32769 => Self::Multi,
            32770 => Self::CenterWeighted,
            32771 => Self::EntireScreenAverage,
            32772 => Self::SpotStandard,
            32773 => Self::SpotLarge,
            32774 => Self::HighLightWeighted,
            32775 => Self::Standard,
            32776 => Self::Backlight,
            32777 => Self::Spotlight,
            _ => return None,
        })
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

/// Shutter mode status for cinema cameras
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ShutterModeStatus {
    /// Shutter mode off
    Off = 1,
    /// Speed-based shutter (time fractions)
    Speed = 2,
    /// Angle-based shutter (degrees)
    Angle = 3,
    /// Extended Clear Scan
    Ecs = 4,
    /// Auto shutter mode
    Auto = 5,
}

impl ToCrsdk<u64> for ShutterModeStatus {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for ShutterModeStatus {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u8 {
            1 => Self::Off,
            2 => Self::Speed,
            3 => Self::Angle,
            4 => Self::Ecs,
            5 => Self::Auto,
            _ => return None,
        })
    }
}

impl fmt::Display for ShutterModeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Off"),
            Self::Speed => write!(f, "Speed"),
            Self::Angle => write!(f, "Angle"),
            Self::Ecs => write!(f, "ECS"),
            Self::Auto => write!(f, "Auto"),
        }
    }
}

/// Shutter mode selection (Speed vs Angle)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ShutterMode {
    /// Speed-based shutter (time fractions like 1/200)
    Speed = 1,
    /// Angle-based shutter (degrees like 180°)
    Angle = 2,
}

impl ToCrsdk<u64> for ShutterMode {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for ShutterMode {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u8 {
            1 => Self::Speed,
            2 => Self::Angle,
            _ => return None,
        })
    }
}

impl fmt::Display for ShutterMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Speed => write!(f, "Speed"),
            Self::Angle => write!(f, "Angle"),
        }
    }
}

/// Exposure control type (P/A/S/M vs Flexible Exposure)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExposureCtrlType {
    /// Traditional P/A/S/M exposure modes
    Pasm = 1,
    /// Flexible exposure with independent control
    FlexibleExposure = 2,
}

impl ToCrsdk<u64> for ExposureCtrlType {
    fn to_crsdk(&self) -> u64 {
        *self as u64
    }
}

impl PropertyValue for ExposureCtrlType {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(match raw as u8 {
            1 => Self::Pasm,
            2 => Self::FlexibleExposure,
            _ => return None,
        })
    }
}

impl fmt::Display for ExposureCtrlType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pasm => write!(f, "P/A/S/M"),
            Self::FlexibleExposure => write!(f, "Flexible Exp"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aperture_display() {
        assert_eq!(Aperture(140).to_string(), "f/1.4");
        assert_eq!(Aperture(280).to_string(), "f/2.8");
        assert_eq!(Aperture(400).to_string(), "f/4.0");
        assert_eq!(Aperture(560).to_string(), "f/5.6");
        assert_eq!(Aperture(800).to_string(), "f/8.0");
        assert_eq!(Aperture(1100).to_string(), "f/11");
        assert_eq!(Aperture(1600).to_string(), "f/16");
        assert_eq!(Aperture(2200).to_string(), "f/22");
    }

    #[test]
    fn test_aperture_from_raw() {
        assert!(Aperture::from_raw(0).is_none());
        assert_eq!(Aperture::from_raw(280).unwrap().to_raw(), 280);
    }

    #[test]
    fn test_shutter_speed_display() {
        // 1/200
        assert_eq!(ShutterSpeed((1u64 << 16) | 200).to_string(), "1/200");
        // 1/3200
        assert_eq!(ShutterSpeed((1u64 << 16) | 3200).to_string(), "1/3200");
        // 2 seconds
        assert_eq!(ShutterSpeed((2u64 << 16) | 1).to_string(), "2\"");
        // 1 second
        assert_eq!(ShutterSpeed((1u64 << 16) | 1).to_string(), "1\"");
    }

    #[test]
    fn test_shutter_speed_from_raw() {
        assert!(ShutterSpeed::from_raw(0).is_none());
        let ss = ShutterSpeed::from_raw((1u64 << 16) | 200).unwrap();
        assert_eq!(ss.numerator(), 1);
        assert_eq!(ss.denominator(), 200);
    }

    #[test]
    fn test_iso_display() {
        assert_eq!(Iso(100).to_string(), "ISO 100");
        assert_eq!(Iso(3200).to_string(), "ISO 3200");
        assert_eq!(Iso(12800).to_string(), "ISO 12800");
    }

    #[test]
    fn test_iso_from_raw() {
        assert!(Iso::from_raw(0).is_none());
        assert_eq!(Iso::from_raw(800).unwrap().value(), 800);
    }

    #[test]
    fn test_exposure_comp_display() {
        assert_eq!(ExposureComp(0).to_string(), "0.0");
        assert_eq!(ExposureComp(3).to_string(), "+1.0");
        assert_eq!(ExposureComp(-3).to_string(), "-1.0");
        assert_eq!(ExposureComp(1).to_string(), "+0.3");
        assert_eq!(ExposureComp(-1).to_string(), "-0.3");
    }

    #[test]
    fn test_exposure_comp_from_raw() {
        // Test signed interpretation
        let ec = ExposureComp::from_raw(u64::MAX - 2).unwrap(); // -3 as u64
        assert_eq!(ec.steps(), -3);
    }

    #[test]
    fn test_meter_level_display() {
        assert_eq!(MeterLevel(0).to_string(), "0");
        assert_eq!(MeterLevel(100).to_string(), "100");
        assert_eq!(MeterLevel(-8000).to_string(), "-8000");
    }

    #[test]
    fn test_meter_level_signed_from_raw() {
        // This is the bug fix test: raw value that represents -8000
        let raw = (-8000i64) as u64;
        let ml = MeterLevel::from_raw(raw).unwrap();
        assert_eq!(ml.value(), -8000);
        assert_eq!(ml.to_string(), "-8000");
    }
}
