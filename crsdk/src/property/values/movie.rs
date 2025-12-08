//! Movie/video recording property value types.

use std::fmt;

use super::super::traits::PropertyValue;

/// Movie recording quality/bitrate setting.
///
/// The SDK uses numeric codes to represent different recording quality presets.
/// Each code maps to a specific combination of frame rate, bitrate, and bit depth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MovieQuality(u64);

impl MovieQuality {
    /// Get the raw quality code.
    pub fn code(&self) -> u64 {
        self.0
    }
}

impl PropertyValue for MovieQuality {
    fn from_raw(raw: u64) -> Option<Self> {
        Some(MovieQuality(raw))
    }

    fn to_raw(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for MovieQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.0 as u16;
        let s = match value {
            0 => "--",
            1 => "60p 50M",
            2 => "30p 50M",
            3 => "24p 50M",
            4 => "50p 50M",
            5 => "25p 50M",
            6 => "60i 24M",
            7 => "50i 24M FX",
            8 => "60i 17M FH",
            9 => "50i 17M FH",
            10 => "60p 28M PS",
            11 => "50p 28M PS",
            12 => "24p 24M FX",
            13 => "25p 24M FX",
            14 => "24p 17M FH",
            15 => "25p 17M FH",
            16 => "120p 50M 720",
            17 => "100p 50M 720",
            18 => "1080 30p 16M",
            19 => "1080 25p 16M",
            20 => "720 30p 6M",
            21 => "720 25p 6M",
            22 => "1080 60p 28M",
            23 => "1080 50p 28M",
            24 => "60p 25M",
            25 => "50p 25M",
            26 => "30p 16M",
            27 => "25p 16M",
            28 => "120p 100M",
            29 => "100p 100M",
            30 => "120p 60M",
            31 => "100p 60M",
            32 => "30p 100M",
            33 => "25p 100M",
            34 => "24p 100M",
            35 => "30p 60M",
            36 => "25p 60M",
            37 => "24p 60M",
            38 => "600M 10bit",
            39 => "500M 10bit",
            40 => "400M 10bit",
            41 => "300M 10bit",
            42 => "280M 10bit",
            43 => "250M 10bit",
            44 => "240M 10bit",
            45 => "222M 10bit",
            46 => "200M 10bit",
            47 => "200M 10bit 420",
            48 => "200M 8bit",
            49 => "185M 10bit",
            50 => "150M 10bit 420",
            51 => "150M 8bit",
            52 => "140M 10bit",
            53 => "111M 10bit",
            54 => "100M 10bit",
            55 => "100M 10bit 420",
            56 => "100M 8bit",
            57 => "93M 10bit",
            58 => "89M 10bit",
            59 => "75M 10bit 420",
            60 => "60M 8bit",
            61 => "50M 10bit",
            62 => "50M 10bit 420",
            63 => "50M 8bit",
            64 => "45M 10bit 420",
            65 => "30M 10bit 420",
            66 => "25M 8bit",
            67 => "16M 8bit",
            68 => "520M 10bit",
            69 => "260M 10bit",
            _ => return write!(f, "{}M", value),
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movie_quality_known_values() {
        assert_eq!(MovieQuality(0).to_string(), "--");
        assert_eq!(MovieQuality(38).to_string(), "600M 10bit");
        assert_eq!(MovieQuality(56).to_string(), "100M 8bit");
    }

    #[test]
    fn test_movie_quality_unknown_fallback() {
        assert_eq!(MovieQuality(999).to_string(), "999M");
    }

    #[test]
    fn test_movie_quality_round_trip() {
        let mq = MovieQuality::from_raw(38).unwrap();
        assert_eq!(mq.to_raw(), 38);
    }
}
