use std::collections::HashMap;

use crsdk::{
    format::{
        format_aperture, format_exposure_comp, format_iso_compact, format_shutter_speed,
        parse_aperture, parse_exposure_comp, parse_iso, parse_shutter_speed,
    },
    format_movie_quality, DriveMode, ExposureProgram, FileType, FlashMode, FocusArea, FocusMode,
    ImageQuality, MeteringMode, MovieFileFormat, PropertyCode, WhiteBalance,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropertyId {
    // Exposure
    ExposureMode,
    ShutterSpeed,
    Aperture,
    Iso,
    ExposureComp,
    // Focus
    FocusMode,
    FocusArea,
    // Image
    WhiteBalance,
    DriveMode,
    // Additional properties for the full editor
    MeteringMode,
    FlashMode,
    FileType,
    ImageQuality,
    ImageSize,
    ColorSpace,
    // Movie
    MovieFormat,
    MovieQuality,
    RecordingFrameRate,
}

impl PropertyId {
    pub const ALL: &'static [Self] = &[
        Self::ExposureMode,
        Self::ShutterSpeed,
        Self::Aperture,
        Self::Iso,
        Self::ExposureComp,
        Self::FocusMode,
        Self::FocusArea,
        Self::WhiteBalance,
        Self::DriveMode,
        Self::MeteringMode,
        Self::FlashMode,
        Self::FileType,
        Self::ImageQuality,
        Self::ImageSize,
        Self::ColorSpace,
        Self::MovieFormat,
        Self::MovieQuality,
        Self::RecordingFrameRate,
    ];

    pub fn category(self) -> PropertyCategory {
        match self {
            Self::ExposureMode
            | Self::ShutterSpeed
            | Self::Aperture
            | Self::Iso
            | Self::ExposureComp => PropertyCategory::Exposure,
            Self::FocusMode | Self::FocusArea => PropertyCategory::Focus,
            Self::WhiteBalance | Self::DriveMode | Self::MeteringMode | Self::FlashMode => {
                PropertyCategory::Image
            }
            Self::FileType | Self::ImageQuality | Self::ImageSize | Self::ColorSpace => {
                PropertyCategory::Image
            }
            Self::MovieFormat | Self::MovieQuality | Self::RecordingFrameRate => {
                PropertyCategory::Movie
            }
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::ExposureMode => "Mode",
            Self::ShutterSpeed => "Shutter",
            Self::Aperture => "Aperture",
            Self::Iso => "ISO",
            Self::ExposureComp => "EV",
            Self::FocusMode => "Focus",
            Self::FocusArea => "Area",
            Self::WhiteBalance => "WB",
            Self::DriveMode => "Drive",
            Self::MeteringMode => "Metering",
            Self::FlashMode => "Flash",
            Self::FileType => "File Type",
            Self::ImageQuality => "Quality",
            Self::ImageSize => "Size",
            Self::ColorSpace => "Color",
            Self::MovieFormat => "Format",
            Self::MovieQuality => "Quality",
            Self::RecordingFrameRate => "Frame Rate",
        }
    }

    /// Convert to SDK property code
    pub fn to_sdk_code(self) -> Option<PropertyCode> {
        match self {
            Self::Aperture => Some(PropertyCode::FNumber),
            Self::ShutterSpeed => Some(PropertyCode::ShutterSpeed),
            Self::Iso => Some(PropertyCode::IsoSensitivity),
            Self::ExposureComp => Some(PropertyCode::ExposureBias),
            Self::ExposureMode => Some(PropertyCode::ExposureProgram),
            Self::FocusMode => Some(PropertyCode::FocusMode),
            Self::FocusArea => Some(PropertyCode::FocusArea),
            Self::WhiteBalance => Some(PropertyCode::WhiteBalance),
            Self::DriveMode => Some(PropertyCode::DriveMode),
            Self::MeteringMode => Some(PropertyCode::MeteringMode),
            Self::FlashMode => Some(PropertyCode::FlashMode),
            Self::FileType => Some(PropertyCode::FileType),
            Self::ImageQuality => Some(PropertyCode::ImageQuality),
            Self::ImageSize => Some(PropertyCode::ImageSize),
            Self::MovieFormat => Some(PropertyCode::MovieFormat),
            Self::MovieQuality => Some(PropertyCode::MovieRecordingSetting),
            Self::ColorSpace | Self::RecordingFrameRate => None,
        }
    }

    /// Convert from SDK property code
    pub fn from_sdk_code(code: PropertyCode) -> Option<Self> {
        match code {
            PropertyCode::FNumber => Some(Self::Aperture),
            PropertyCode::ShutterSpeed => Some(Self::ShutterSpeed),
            PropertyCode::IsoSensitivity => Some(Self::Iso),
            PropertyCode::ExposureBias => Some(Self::ExposureComp),
            PropertyCode::ExposureProgram => Some(Self::ExposureMode),
            PropertyCode::FocusMode => Some(Self::FocusMode),
            PropertyCode::FocusArea => Some(Self::FocusArea),
            PropertyCode::WhiteBalance => Some(Self::WhiteBalance),
            PropertyCode::DriveMode => Some(Self::DriveMode),
            PropertyCode::MeteringMode => Some(Self::MeteringMode),
            PropertyCode::FlashMode => Some(Self::FlashMode),
            PropertyCode::FileType => Some(Self::FileType),
            PropertyCode::ImageQuality => Some(Self::ImageQuality),
            PropertyCode::ImageSize => Some(Self::ImageSize),
            PropertyCode::MovieFormat => Some(Self::MovieFormat),
            PropertyCode::MovieRecordingSetting => Some(Self::MovieQuality),
            _ => None,
        }
    }

    /// Get all PropertyIds that map to SDK codes
    pub fn all_sdk_mapped() -> impl Iterator<Item = Self> {
        [
            Self::Aperture,
            Self::ShutterSpeed,
            Self::Iso,
            Self::ExposureComp,
            Self::ExposureMode,
            Self::FocusMode,
            Self::FocusArea,
            Self::WhiteBalance,
            Self::DriveMode,
            Self::MeteringMode,
            Self::FlashMode,
            Self::FileType,
            Self::ImageQuality,
            Self::ImageSize,
            Self::MovieFormat,
            Self::MovieQuality,
        ]
        .into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PropertyCategory {
    #[default]
    Exposure,
    Focus,
    Image,
    Movie,
}

impl PropertyCategory {
    pub const ALL: [PropertyCategory; 4] = [
        PropertyCategory::Exposure,
        PropertyCategory::Focus,
        PropertyCategory::Image,
        PropertyCategory::Movie,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Exposure => "Exposure",
            Self::Focus => "Focus",
            Self::Image => "Image",
            Self::Movie => "Movie",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    pub id: PropertyId,
    pub values: Vec<String>,
    pub current_index: usize,
    pub writable: bool,
}

impl Property {
    pub fn new(id: PropertyId, values: Vec<&str>, default_index: usize) -> Self {
        Self {
            id,
            values: values.into_iter().map(String::from).collect(),
            current_index: default_index,
            writable: true,
        }
    }

    pub fn current_value(&self) -> &str {
        self.values
            .get(self.current_index)
            .map(|s| s.as_str())
            .unwrap_or("--")
    }

    pub fn next(&mut self) -> usize {
        if self.writable && !self.values.is_empty() {
            self.current_index = (self.current_index + 1) % self.values.len();
        }
        self.current_index
    }

    pub fn prev(&mut self) -> usize {
        if self.writable && !self.values.is_empty() {
            self.current_index = self
                .current_index
                .checked_sub(1)
                .unwrap_or(self.values.len() - 1);
        }
        self.current_index
    }

    pub fn set_value(&mut self, value: &str) {
        if let Some(idx) = self.values.iter().position(|v| v == value) {
            self.current_index = idx;
        }
    }
}

#[derive(Debug, Clone)]
pub struct PropertyStore {
    properties: HashMap<PropertyId, Property>,
    pinned: Vec<PropertyId>,
    /// Whether properties have been loaded from the camera
    loaded: bool,
}

impl PropertyStore {
    pub fn new() -> Self {
        let mut store = Self {
            properties: HashMap::new(),
            pinned: Vec::new(),
            loaded: false,
        };
        store.init_defaults();
        store
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn set_loaded(&mut self, loaded: bool) {
        self.loaded = loaded;
    }

    fn init_defaults(&mut self) {
        // Exposure properties
        self.add(Property::new(
            PropertyId::ExposureMode,
            vec!["M", "A", "S", "P", "Auto"],
            0,
        ));
        self.add(Property::new(
            PropertyId::ShutterSpeed,
            vec![
                "1/8000", "1/4000", "1/2000", "1/1000", "1/500", "1/250", "1/200", "1/125", "1/60",
                "1/30", "1/15", "1/8", "1/4", "1\"", "2\"",
            ],
            6,
        ));
        self.add(Property::new(
            PropertyId::Aperture,
            vec![
                "f/1.4", "f/1.8", "f/2.0", "f/2.8", "f/4.0", "f/5.6", "f/8.0", "f/11", "f/16",
                "f/22",
            ],
            3,
        ));
        self.add(Property::new(
            PropertyId::Iso,
            vec![
                "100", "200", "400", "800", "1600", "3200", "6400", "12800", "25600",
            ],
            5,
        ));
        self.add(Property::new(
            PropertyId::ExposureComp,
            vec![
                "-3.0", "-2.0", "-1.0", "-0.7", "-0.3", "0.0", "+0.3", "+0.7", "+1.0", "+2.0",
                "+3.0",
            ],
            5,
        ));

        // Focus properties
        self.add(Property::new(
            PropertyId::FocusMode,
            vec!["AF-S", "AF-C", "DMF", "MF"],
            1,
        ));
        self.add(Property::new(
            PropertyId::FocusArea,
            vec!["Wide", "Zone", "Center", "Spot", "Tracking"],
            0,
        ));

        // Image properties
        self.add(Property::new(
            PropertyId::WhiteBalance,
            vec![
                "Auto",
                "Daylight",
                "Shade",
                "Cloudy",
                "Tungsten",
                "Fluorescent",
                "Flash",
            ],
            1,
        ));
        self.add(Property::new(
            PropertyId::DriveMode,
            vec!["Single", "Cont. Hi", "Cont. Lo", "Timer", "Bracket"],
            0,
        ));
        self.add(Property::new(
            PropertyId::MeteringMode,
            vec!["Multi", "Center", "Spot", "Average", "Highlight"],
            0,
        ));
        self.add(Property::new(
            PropertyId::FlashMode,
            vec!["Off", "Auto", "Fill", "Slow Sync", "Rear Sync"],
            0,
        ));
        self.add(Property::new(
            PropertyId::FileType,
            vec!["JPEG", "RAW", "RAW+JPEG", "RAW+HEIF", "HEIF"],
            0,
        ));
        self.add(Property::new(
            PropertyId::ImageQuality,
            vec!["Light", "Standard", "Fine", "Extra Fine"],
            2,
        ));
        self.add(Property::new(PropertyId::ImageSize, vec!["L", "M", "S"], 0));
        self.add(Property::new(
            PropertyId::ColorSpace,
            vec!["sRGB", "Adobe RGB"],
            0,
        ));

        // Movie properties
        self.add(Property::new(
            PropertyId::MovieFormat,
            vec!["XAVC S 4K", "XAVC S HD", "XAVC HS 4K", "XAVC HS 8K"],
            0,
        ));
        self.add(Property::new(
            PropertyId::MovieQuality,
            vec![
                "4K 60p",
                "4K 30p",
                "4K 24p",
                "1080 120p",
                "1080 60p",
                "1080 30p",
            ],
            0,
        ));
        self.add(Property::new(
            PropertyId::RecordingFrameRate,
            vec!["24p", "30p", "60p", "120p"],
            2,
        ));

        // Default pinned properties (the quick settings)
        self.pinned = vec![
            PropertyId::ShutterSpeed,
            PropertyId::Aperture,
            PropertyId::Iso,
            PropertyId::ExposureComp,
            PropertyId::FocusMode,
            PropertyId::FocusArea,
            PropertyId::WhiteBalance,
            PropertyId::DriveMode,
        ];
    }

    fn add(&mut self, property: Property) {
        self.properties.insert(property.id, property);
    }

    pub fn get(&self, id: PropertyId) -> Option<&Property> {
        self.properties.get(&id)
    }

    pub fn get_mut(&mut self, id: PropertyId) -> Option<&mut Property> {
        self.properties.get_mut(&id)
    }

    pub fn pinned_properties(&self) -> Vec<&Property> {
        self.pinned
            .iter()
            .filter_map(|id| self.properties.get(id))
            .collect()
    }

    pub fn pinned_ids(&self) -> &[PropertyId] {
        &self.pinned
    }

    pub fn is_pinned(&self, id: PropertyId) -> bool {
        self.pinned.contains(&id)
    }

    pub fn toggle_pin(&mut self, id: PropertyId) {
        if let Some(pos) = self.pinned.iter().position(|&p| p == id) {
            self.pinned.remove(pos);
        } else {
            self.insert_pinned_sorted(id);
        }
    }

    fn insert_pinned_sorted(&mut self, id: PropertyId) {
        let category = id.category();
        let category_order = category as u8;

        // Find the right position: after all properties of the same or earlier categories
        let insert_pos = self
            .pinned
            .iter()
            .position(|&p| (p.category() as u8) > category_order)
            .unwrap_or(self.pinned.len());

        self.pinned.insert(insert_pos, id);
    }

    pub fn properties_by_category(&self, category: PropertyCategory) -> Vec<&Property> {
        self.properties
            .values()
            .filter(|p| p.id.category() == category)
            .collect()
    }

    pub fn all_properties_sorted(&self) -> Vec<&Property> {
        let mut props: Vec<_> = self.properties.values().collect();
        props.sort_by_key(|p| (p.id.category() as u8, p.id.name()));
        props
    }

    pub fn exposure_mode(&self) -> &str {
        self.get(PropertyId::ExposureMode)
            .map(|p| p.current_value())
            .unwrap_or("M")
    }

    pub fn update_writable_for_mode(&mut self) {
        let mode = self.exposure_mode().to_string();

        let (shutter_writable, aperture_writable, iso_writable, ev_writable) = match mode.as_str() {
            "P" => (false, false, true, true),
            "A" => (false, true, true, true),
            "S" => (true, false, true, true),
            "M" => (true, true, true, false),
            "Auto" => (false, false, false, true),
            _ => (true, true, true, true),
        };

        if let Some(p) = self.get_mut(PropertyId::ShutterSpeed) {
            p.writable = shutter_writable;
        }
        if let Some(p) = self.get_mut(PropertyId::Aperture) {
            p.writable = aperture_writable;
        }
        if let Some(p) = self.get_mut(PropertyId::Iso) {
            p.writable = iso_writable;
        }
        if let Some(p) = self.get_mut(PropertyId::ExposureComp) {
            p.writable = ev_writable;
        }
    }

    /// Update a property from SDK data
    pub fn update_from_sdk(
        &mut self,
        id: PropertyId,
        current: u64,
        available: &[u64],
        writable: bool,
    ) {
        let current_str = format_sdk_value(id, current);
        let available_strs: Vec<String> =
            available.iter().map(|&v| format_sdk_value(id, v)).collect();

        if let Some(prop) = self.get_mut(id) {
            prop.values = available_strs;
            prop.set_value(&current_str);
            prop.writable = writable;
        }
    }

    /// Update a property with formatted strings directly
    pub fn update_from_sdk_formatted(
        &mut self,
        id: PropertyId,
        current: &str,
        available: Vec<String>,
        writable: bool,
    ) {
        if let Some(prop) = self.get_mut(id) {
            prop.values = available;
            prop.set_value(current);
            prop.writable = writable;
        }
    }

    /// Update only value and available options, preserving writable state.
    /// Used for PropertyChanged events where SDK may temporarily set writable=false
    /// during operations like autofocus but doesn't always restore it.
    pub fn update_from_sdk_value_only(
        &mut self,
        id: PropertyId,
        current: &str,
        available: Vec<String>,
    ) {
        if let Some(prop) = self.get_mut(id) {
            prop.values = available;
            prop.set_value(current);
        }
    }
}

impl Default for PropertyStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Format a raw SDK value to a display string based on property type
pub fn format_sdk_value(id: PropertyId, raw: u64) -> String {
    match id {
        PropertyId::Aperture => format_aperture(raw),
        PropertyId::ShutterSpeed => format_shutter_speed(raw),
        PropertyId::Iso => format_iso_compact(raw),
        PropertyId::ExposureComp => format_exposure_comp(raw as i64),
        PropertyId::ExposureMode => ExposureProgram::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("Unknown({})", raw)),
        PropertyId::FocusMode => FocusMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("Unknown({})", raw)),
        PropertyId::FocusArea => FocusArea::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("Unknown({})", raw)),
        PropertyId::WhiteBalance => WhiteBalance::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("Unknown({})", raw)),
        PropertyId::DriveMode => DriveMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("Unknown({})", raw)),
        PropertyId::MeteringMode => MeteringMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("Unknown({})", raw)),
        PropertyId::FlashMode => FlashMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("Unknown({})", raw)),
        PropertyId::FileType => FileType::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("Unknown({})", raw)),
        PropertyId::ImageQuality => ImageQuality::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("Unknown({})", raw)),
        PropertyId::MovieFormat => MovieFileFormat::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("Unknown({})", raw)),
        PropertyId::MovieQuality => format_movie_quality(raw),
        _ => format!("{}", raw),
    }
}

/// Parse a display string back to raw SDK value
pub fn parse_display_value(id: PropertyId, display: &str) -> Option<u64> {
    match id {
        PropertyId::Aperture => parse_aperture(display),
        PropertyId::ShutterSpeed => parse_shutter_speed(display),
        PropertyId::Iso => parse_iso(display),
        PropertyId::ExposureComp => parse_exposure_comp(display).map(|v| v as u64),
        _ => None,
    }
}

/// Fuzzy match score for a query against a property name
/// Returns Some(score) if matches, None if no match
/// Higher scores are better matches
fn fuzzy_match_score(query: &str, name: &str) -> Option<i32> {
    if query.is_empty() {
        return Some(0);
    }

    let query_lower = query.to_lowercase();
    let name_lower = name.to_lowercase();

    // Exact match gets highest score
    if name_lower == query_lower {
        return Some(1000);
    }

    // Prefix match gets high score (shorter names get bonus)
    if name_lower.starts_with(&query_lower) {
        let length_bonus = (100i32).saturating_sub(name.len() as i32).max(0);
        return Some(500 + length_bonus);
    }

    // Contains match (shorter names get bonus)
    if name_lower.contains(&query_lower) {
        let length_bonus = (100i32).saturating_sub(name.len() as i32).max(0);
        return Some(200 + length_bonus);
    }

    // Fuzzy match: all query chars must appear in order
    let mut query_chars = query_lower.chars().peekable();
    let mut score = 0;
    let mut prev_matched = false;

    for c in name_lower.chars() {
        if let Some(&qc) = query_chars.peek() {
            if c == qc {
                query_chars.next();
                // Bonus for consecutive matches
                if prev_matched {
                    score += 10;
                } else {
                    score += 5;
                }
                prev_matched = true;
            } else {
                prev_matched = false;
            }
        }
    }

    // All query chars must be consumed for a match
    if query_chars.peek().is_none() {
        Some(score)
    } else {
        None
    }
}

/// Search supported properties by fuzzy matching query against property names
pub fn search_properties(query: &str) -> Vec<PropertyId> {
    let mut results: Vec<(PropertyId, i32)> = PropertyId::ALL
        .iter()
        .filter_map(|&id| {
            let name = id.name();
            let category_name = id.category().name();
            let full_name = format!("{}: {}", category_name, name);

            let score =
                fuzzy_match_score(query, name).or_else(|| fuzzy_match_score(query, &full_name));

            score.map(|s| (id, s))
        })
        .collect();

    // Sort by score descending
    results.sort_by(|a, b| b.1.cmp(&a.1));

    results.into_iter().map(|(id, _)| id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match_exact() {
        assert_eq!(fuzzy_match_score("iso", "ISO"), Some(1000));
        assert_eq!(fuzzy_match_score("aperture", "Aperture"), Some(1000));
    }

    #[test]
    fn test_fuzzy_match_prefix() {
        let score = fuzzy_match_score("shut", "Shutter").unwrap();
        assert!(score > 500 && score < 1000);
    }

    #[test]
    fn test_fuzzy_match_contains() {
        let score = fuzzy_match_score("utter", "Shutter").unwrap();
        assert!(score > 200 && score < 500);
    }

    #[test]
    fn test_fuzzy_match_sequential_chars() {
        assert!(fuzzy_match_score("sr", "Shutter").is_some());
        assert!(fuzzy_match_score("ae", "Aperture").is_some());
    }

    #[test]
    fn test_fuzzy_match_no_match() {
        assert_eq!(fuzzy_match_score("xyz", "ISO"), None);
        assert_eq!(fuzzy_match_score("zba", "Aperture"), None);
    }

    #[test]
    fn test_fuzzy_match_empty_query() {
        assert_eq!(fuzzy_match_score("", "anything"), Some(0));
    }

    #[test]
    fn test_search_properties_empty_returns_all() {
        let results = search_properties("");
        assert_eq!(results.len(), PropertyId::ALL.len());
    }

    #[test]
    fn test_search_properties_exact_match_first() {
        let results = search_properties("ISO");
        assert!(!results.is_empty());
        assert_eq!(results[0], PropertyId::Iso);
    }

    #[test]
    fn test_search_properties_partial_match() {
        let results = search_properties("shut");
        assert!(!results.is_empty());
        assert_eq!(results[0], PropertyId::ShutterSpeed);
    }

    #[test]
    fn test_search_properties_case_insensitive() {
        let results1 = search_properties("ISO");
        let results2 = search_properties("iso");
        assert_eq!(results1[0], results2[0]);
    }

    #[test]
    fn test_all_properties_count() {
        assert_eq!(
            PropertyId::ALL.len(),
            18,
            "PropertyId::ALL out of sync with enum variants"
        );
    }
}
