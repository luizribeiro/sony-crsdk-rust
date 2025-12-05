use std::collections::HashMap;

use crsdk::{
    format::{
        format_aperture, format_exposure_comp, format_iso_compact, format_shutter_speed,
        parse_aperture, parse_exposure_comp, parse_iso, parse_shutter_speed,
    },
    format_movie_quality, property_display_name, AspectRatio, AutoManual, DevicePropertyCode,
    DriveMode, ExposureCtrlType, ExposureProgram, FileType, FlashMode, FocusArea, FocusMode,
    FocusTrackingStatus, ImageQuality, ImageSize, IntervalRecShutterType, LiveViewDisplayEffect,
    MeteringMode, MovieFileFormat, OnOff, PrioritySetInAF, PropertyCategory, ShutterMode,
    ShutterModeStatus, SilentModeApertureDrive, SubjectRecognitionAF, Switch, WhiteBalance,
};

#[derive(Debug, Clone)]
pub struct Property {
    pub code: DevicePropertyCode,
    pub values: Vec<String>,
    pub current_index: usize,
    pub writable: bool,
}

impl Property {
    pub fn new(code: DevicePropertyCode) -> Self {
        Self {
            code,
            values: Vec::new(),
            current_index: 0,
            writable: false,
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
    properties: HashMap<DevicePropertyCode, Property>,
    pinned: Vec<DevicePropertyCode>,
    loaded: bool,
}

impl PropertyStore {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            pinned: Vec::new(),
            loaded: false,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn set_loaded(&mut self, loaded: bool) {
        self.loaded = loaded;
    }

    pub fn clear(&mut self) {
        self.properties.clear();
        self.pinned.clear();
        self.loaded = false;
    }

    pub fn get(&self, code: DevicePropertyCode) -> Option<&Property> {
        self.properties.get(&code)
    }

    pub fn get_mut(&mut self, code: DevicePropertyCode) -> Option<&mut Property> {
        self.properties.get_mut(&code)
    }

    pub fn pinned_properties(&self) -> Vec<&Property> {
        self.pinned
            .iter()
            .filter_map(|code| self.properties.get(code))
            .collect()
    }

    pub fn pinned_ids(&self) -> &[DevicePropertyCode] {
        &self.pinned
    }

    pub fn is_pinned(&self, code: DevicePropertyCode) -> bool {
        self.pinned.contains(&code)
    }

    pub fn toggle_pin(&mut self, code: DevicePropertyCode) {
        if let Some(pos) = self.pinned.iter().position(|&p| p == code) {
            self.pinned.remove(pos);
        } else {
            self.insert_pinned_sorted(code);
        }
    }

    fn insert_pinned_sorted(&mut self, code: DevicePropertyCode) {
        let category = code.category();
        let category_order = category_sort_order(category);

        let insert_pos = self
            .pinned
            .iter()
            .position(|&p| category_sort_order(p.category()) > category_order)
            .unwrap_or(self.pinned.len());

        self.pinned.insert(insert_pos, code);
    }

    pub fn properties_by_category(&self, category: PropertyCategory) -> Vec<&Property> {
        let mut props: Vec<_> = self
            .properties
            .values()
            .filter(|p| p.code.category() == category)
            .collect();
        props.sort_by_key(|p| p.code.name());
        props
    }

    pub fn all_properties_sorted(&self) -> Vec<&Property> {
        let mut props: Vec<_> = self.properties.values().collect();
        props.sort_by_key(|p| (category_sort_order(p.code.category()), p.code.name()));
        props
    }

    pub fn available_categories(&self) -> Vec<PropertyCategory> {
        let mut categories: Vec<PropertyCategory> = self
            .properties
            .values()
            .map(|p| p.code.category())
            .collect();
        categories.sort_by_key(|c| category_sort_order(*c));
        categories.dedup();
        categories
    }

    pub fn exposure_mode(&self) -> &str {
        self.get(DevicePropertyCode::ExposureProgramMode)
            .map(|p| p.current_value())
            .unwrap_or("M")
    }

    pub fn add_property(
        &mut self,
        code: DevicePropertyCode,
        current: &str,
        available: Vec<String>,
        writable: bool,
    ) {
        let mut prop = Property::new(code);
        prop.values = available;
        prop.set_value(current);
        prop.writable = writable;

        let is_new = !self.properties.contains_key(&code);
        self.properties.insert(code, prop);

        if is_new && is_default_pinned(code) {
            self.insert_pinned_sorted(code);
        }
    }

    pub fn update_property(
        &mut self,
        code: DevicePropertyCode,
        current: &str,
        available: Vec<String>,
        writable: bool,
    ) {
        if let Some(prop) = self.properties.get_mut(&code) {
            prop.values = available;
            prop.set_value(current);
            prop.writable = writable;
        } else {
            self.add_property(code, current, available, writable);
        }
    }

    pub fn update_value_only(
        &mut self,
        code: DevicePropertyCode,
        current: &str,
        available: Vec<String>,
    ) {
        if let Some(prop) = self.properties.get_mut(&code) {
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

fn is_default_pinned(code: DevicePropertyCode) -> bool {
    matches!(
        code,
        DevicePropertyCode::ShutterSpeed
            | DevicePropertyCode::FNumber
            | DevicePropertyCode::IsoSensitivity
            | DevicePropertyCode::ExposureBiasCompensation
            | DevicePropertyCode::FocusMode
            | DevicePropertyCode::FocusArea
            | DevicePropertyCode::WhiteBalance
            | DevicePropertyCode::DriveMode
    )
}

fn category_sort_order(cat: PropertyCategory) -> u8 {
    match cat {
        PropertyCategory::Exposure => 0,
        PropertyCategory::Focus => 1,
        PropertyCategory::WhiteBalance => 2,
        PropertyCategory::Image => 3,
        PropertyCategory::Movie => 4,
        PropertyCategory::Media => 5,
        PropertyCategory::Drive => 6,
        PropertyCategory::Metering => 7,
        PropertyCategory::Flash => 8,
        PropertyCategory::Zoom => 9,
        PropertyCategory::Lens => 10,
        PropertyCategory::Audio => 11,
        PropertyCategory::PictureProfile => 12,
        PropertyCategory::NDFilter => 13,
        PropertyCategory::Stabilization => 14,
        PropertyCategory::Display => 15,
        PropertyCategory::Power => 16,
        PropertyCategory::CustomButtons => 17,
        PropertyCategory::Silent => 18,
        PropertyCategory::Other => 19,
        _ => 20,
    }
}

pub fn format_sdk_value(code: DevicePropertyCode, raw: u64) -> String {
    match code {
        DevicePropertyCode::FNumber => format_aperture(raw),
        DevicePropertyCode::ShutterSpeed => format_shutter_speed(raw),
        DevicePropertyCode::IsoSensitivity => format_iso_compact(raw),
        DevicePropertyCode::ExposureBiasCompensation => format_exposure_comp(raw as i64),
        DevicePropertyCode::ExposureProgramMode => ExposureProgram::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        DevicePropertyCode::FocusMode => FocusMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        DevicePropertyCode::FocusArea => FocusArea::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        DevicePropertyCode::WhiteBalance => WhiteBalance::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        DevicePropertyCode::DriveMode => DriveMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        DevicePropertyCode::MeteringMode => MeteringMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        DevicePropertyCode::FlashMode => FlashMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        DevicePropertyCode::StillImageStoreDestination => FileType::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        DevicePropertyCode::StillImageQuality => ImageQuality::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        DevicePropertyCode::MovieFileFormat => MovieFileFormat::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        DevicePropertyCode::MovieRecordingSetting => format_movie_quality(raw),
        DevicePropertyCode::AutoSlowShutter
        | DevicePropertyCode::SilentMode
        | DevicePropertyCode::NDFilter
        | DevicePropertyCode::ShutterSetting
        | DevicePropertyCode::ShutterECSSetting
        | DevicePropertyCode::ShutterSlow
        | DevicePropertyCode::AFAssist
        | DevicePropertyCode::PreAF
        | DevicePropertyCode::AFWithShutter
        | DevicePropertyCode::SubjectRecognitionInAF
        | DevicePropertyCode::FaceEyeFrameDisplay => Switch::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        DevicePropertyCode::RedEyeReduction | DevicePropertyCode::AudioRecording => {
            OnOff::from_raw(raw)
                .map(|v| v.to_string())
                .unwrap_or_else(|| format!("{}", raw))
        }
        DevicePropertyCode::IrisModeSetting
        | DevicePropertyCode::ShutterModeSetting
        | DevicePropertyCode::GainControlSetting
        | DevicePropertyCode::NDFilterModeSetting
        | DevicePropertyCode::FocusModeSetting => AutoManual::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        DevicePropertyCode::AspectRatio => AspectRatio::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        DevicePropertyCode::ImageSize => ImageSize::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        DevicePropertyCode::LiveViewDisplayEffect => LiveViewDisplayEffect::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        DevicePropertyCode::ShutterModeStatus => ShutterModeStatus::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        DevicePropertyCode::ShutterMode => ShutterMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        DevicePropertyCode::ExposureCtrlType => ExposureCtrlType::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        DevicePropertyCode::IntervalRecShutterType => IntervalRecShutterType::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        DevicePropertyCode::SilentModeApertureDriveInAF => SilentModeApertureDrive::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        DevicePropertyCode::SubjectRecognitionAF => SubjectRecognitionAF::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        DevicePropertyCode::PrioritySetInAFS | DevicePropertyCode::PrioritySetInAFC => {
            PrioritySetInAF::from_raw(raw)
                .map(|v| v.to_string())
                .unwrap_or_else(|| format!("{}", raw))
        }
        DevicePropertyCode::FocusTrackingStatus => FocusTrackingStatus::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        _ => format!("{}", raw),
    }
}

pub fn parse_display_value(code: DevicePropertyCode, display: &str) -> Option<u64> {
    match code {
        DevicePropertyCode::FNumber => parse_aperture(display),
        DevicePropertyCode::ShutterSpeed => parse_shutter_speed(display),
        DevicePropertyCode::IsoSensitivity => parse_iso(display),
        DevicePropertyCode::ExposureBiasCompensation => {
            parse_exposure_comp(display).map(|v| v as u64)
        }
        _ => None,
    }
}

fn fuzzy_match_score(query: &str, name: &str) -> Option<i32> {
    if query.is_empty() {
        return Some(0);
    }

    let query_lower = query.to_lowercase();
    let name_lower = name.to_lowercase();

    if name_lower == query_lower {
        return Some(1000);
    }

    if name_lower.starts_with(&query_lower) {
        let length_bonus = (100i32).saturating_sub(name.len() as i32).max(0);
        return Some(500 + length_bonus);
    }

    if name_lower.contains(&query_lower) {
        let length_bonus = (100i32).saturating_sub(name.len() as i32).max(0);
        return Some(200 + length_bonus);
    }

    let mut query_chars = query_lower.chars().peekable();
    let mut score = 0;
    let mut prev_matched = false;

    for c in name_lower.chars() {
        if let Some(&qc) = query_chars.peek() {
            if c == qc {
                query_chars.next();
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

    if query_chars.peek().is_none() {
        Some(score)
    } else {
        None
    }
}

pub fn search_properties(store: &PropertyStore, query: &str) -> Vec<DevicePropertyCode> {
    let mut results: Vec<(DevicePropertyCode, i32)> = store
        .properties
        .keys()
        .filter_map(|&code| {
            let display_name = property_display_name(code);
            let category_name = code.category().to_string();
            let full_name = format!("{}: {}", category_name, display_name);

            let score = fuzzy_match_score(query, display_name)
                .or_else(|| fuzzy_match_score(query, &full_name));

            score.map(|s| (code, s))
        })
        .collect();

    results.sort_by(|a, b| b.1.cmp(&a.1));
    results.into_iter().map(|(code, _)| code).collect()
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
    fn test_property_store_add_and_get() {
        let mut store = PropertyStore::new();
        store.add_property(
            DevicePropertyCode::FNumber,
            "f/2.8",
            vec!["f/1.4".into(), "f/2.8".into(), "f/4.0".into()],
            true,
        );

        let prop = store.get(DevicePropertyCode::FNumber).unwrap();
        assert_eq!(prop.current_value(), "f/2.8");
        assert!(prop.writable);
    }

    #[test]
    fn test_category_sort_order() {
        assert!(
            category_sort_order(PropertyCategory::Exposure)
                < category_sort_order(PropertyCategory::Focus)
        );
        assert!(
            category_sort_order(PropertyCategory::Focus)
                < category_sort_order(PropertyCategory::Movie)
        );
    }
}
