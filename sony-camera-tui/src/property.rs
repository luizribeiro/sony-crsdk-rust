use std::collections::HashMap;

use crsdk::{
    format::{
        format_aperture, format_color_temp, format_exposure_comp, format_iso_compact,
        format_shutter_speed,
    },
    format_movie_quality, property_category, property_display_name, property_value_type,
    DevicePropertyCode, PropertyCategory, PropertyValueType,
};

/// How a property's values are constrained
#[derive(Debug, Clone, Default)]
pub enum PropertyKind {
    /// Discrete list of allowed values
    #[default]
    Discrete,
    /// Numeric range with min/max/step
    Range { min: i64, max: i64, step: i64 },
}

#[derive(Debug, Clone)]
pub struct Property {
    pub code: DevicePropertyCode,
    /// For discrete: formatted value strings. For range: may be empty or contain formatted current.
    pub values: Vec<String>,
    /// For discrete: index into values. For range: index in the range (0 = min).
    pub current_index: usize,
    pub writable: bool,
    /// The kind of constraint (discrete or range)
    pub kind: PropertyKind,
    /// Raw SDK value (useful for formatting)
    pub current_raw: u64,
}

impl Property {
    pub fn new(code: DevicePropertyCode) -> Self {
        Self {
            code,
            values: Vec::new(),
            current_index: 0,
            writable: false,
            kind: PropertyKind::Discrete,
            current_raw: 0,
        }
    }

    pub fn current_value(&self) -> &str {
        match &self.kind {
            PropertyKind::Discrete => self
                .values
                .get(self.current_index)
                .map(|s| s.as_str())
                .unwrap_or("--"),
            PropertyKind::Range { .. } => self.values.first().map(|s| s.as_str()).unwrap_or("--"),
        }
    }

    /// Check if this is a range property
    pub fn is_range(&self) -> bool {
        matches!(self.kind, PropertyKind::Range { .. })
    }

    /// Get range parameters if this is a range property
    pub fn range_params(&self) -> Option<(i64, i64, i64)> {
        match &self.kind {
            PropertyKind::Range { min, max, step } => Some((*min, *max, *step)),
            _ => None,
        }
    }

    /// Total number of possible values
    pub fn value_count(&self) -> usize {
        match &self.kind {
            PropertyKind::Discrete => self.values.len(),
            PropertyKind::Range { min, max, step } => {
                let step = if *step == 0 { 1 } else { *step };
                ((max - min) / step + 1) as usize
            }
        }
    }

    /// Get the raw value at a given index
    pub fn raw_value_at_index(&self, index: usize) -> Option<u64> {
        match &self.kind {
            PropertyKind::Discrete => {
                // For discrete, we don't store raw values, return None
                None
            }
            PropertyKind::Range { min, step, .. } => {
                let step = if *step == 0 { 1 } else { *step };
                Some((min + (index as i64) * step) as u64)
            }
        }
    }

    /// Move to next value, returns the new index
    pub fn next(&mut self) -> usize {
        self.advance(1)
    }

    /// Move to previous value, returns the new index
    pub fn prev(&mut self) -> usize {
        self.advance(-1)
    }

    /// Advance by a number of steps (positive or negative)
    pub fn advance(&mut self, steps: i64) -> usize {
        if !self.writable {
            return self.current_index;
        }

        let count = self.value_count();
        if count == 0 {
            return self.current_index;
        }

        let new_index = if steps >= 0 {
            let forward = steps as usize % count;
            (self.current_index + forward) % count
        } else {
            let backward = (-steps) as usize % count;
            (self.current_index + count - backward) % count
        };

        self.current_index = new_index;

        // Update current_raw for range properties
        if let PropertyKind::Range { min, step, .. } = &self.kind {
            let step = if *step == 0 { 1 } else { *step };
            self.current_raw = (min + (new_index as i64) * step) as u64;
        }

        self.current_index
    }

    /// Jump directly to a specific index (clamped to valid range)
    pub fn set_index(&mut self, index: usize) -> usize {
        let count = self.value_count();
        if count == 0 {
            return self.current_index;
        }

        self.current_index = index.min(count - 1);

        // Update current_raw for range properties
        if let PropertyKind::Range { min, step, .. } = &self.kind {
            let step = if *step == 0 { 1 } else { *step };
            self.current_raw = (min + (self.current_index as i64) * step) as u64;
        }

        self.current_index
    }

    /// Get progress as a ratio (0.0 to 1.0) for gauge display
    pub fn progress(&self) -> f64 {
        let count = self.value_count();
        if count <= 1 {
            return 0.0;
        }
        self.current_index as f64 / (count - 1) as f64
    }

    pub fn set_value(&mut self, value: &str) {
        if let Some(idx) = self.values.iter().position(|v| v == value) {
            self.current_index = idx;
        }
    }

    /// Set the current raw value (for range properties, computes the index)
    pub fn set_raw_value(&mut self, raw: u64) {
        self.current_raw = raw;
        if let PropertyKind::Range { min, step, .. } = &self.kind {
            let step = if *step == 0 { 1 } else { *step };
            let raw_signed = raw as i64;
            if raw_signed >= *min {
                self.current_index = ((raw_signed - min) / step) as usize;
            }
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

    pub fn get(&self, code: DevicePropertyCode) -> Option<&Property> {
        self.properties.get(&code)
    }

    pub fn get_mut(&mut self, code: DevicePropertyCode) -> Option<&mut Property> {
        self.properties.get_mut(&code)
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
        let category = property_category(code);
        let category_order = category_sort_order(category);

        let insert_pos = self
            .pinned
            .iter()
            .position(|&p| category_sort_order(property_category(p)) > category_order)
            .unwrap_or(self.pinned.len());

        self.pinned.insert(insert_pos, code);
    }

    pub fn properties_by_category(&self, category: PropertyCategory) -> Vec<&Property> {
        let mut props: Vec<_> = self
            .properties
            .values()
            .filter(|p| property_category(p.code) == category)
            .collect();
        props.sort_by_key(|p| p.code.name());
        props
    }

    pub fn available_categories(&self) -> Vec<PropertyCategory> {
        let mut categories: Vec<PropertyCategory> = self
            .properties
            .values()
            .map(|p| property_category(p.code))
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
        current_raw: u64,
        available: Vec<String>,
        writable: bool,
        kind: PropertyKind,
    ) {
        let mut prop = Property::new(code);
        prop.values = available;
        prop.writable = writable;
        prop.kind = kind;
        prop.current_raw = current_raw;

        // Set up the current index based on kind
        match &prop.kind {
            PropertyKind::Discrete => {
                prop.set_value(current);
            }
            PropertyKind::Range { min, step, .. } => {
                let step = if *step == 0 { 1 } else { *step };
                let raw_signed = current_raw as i64;
                if raw_signed >= *min {
                    prop.current_index = ((raw_signed - min) / step) as usize;
                }
                // Store the formatted current value
                prop.values = vec![current.to_string()];
            }
        }

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
        current_raw: u64,
        available: Vec<String>,
        writable: bool,
        kind: PropertyKind,
    ) {
        if let Some(prop) = self.properties.get_mut(&code) {
            prop.writable = writable;
            prop.kind = kind.clone();
            prop.current_raw = current_raw;

            match &prop.kind {
                PropertyKind::Discrete => {
                    prop.values = available;
                    prop.set_value(current);
                }
                PropertyKind::Range { min, step, .. } => {
                    let step = if *step == 0 { 1 } else { *step };
                    let raw_signed = current_raw as i64;
                    if raw_signed >= *min {
                        prop.current_index = ((raw_signed - min) / step) as usize;
                    }
                    // Store the formatted current value
                    prop.values = vec![current.to_string()];
                }
            }
        } else {
            self.add_property(code, current, current_raw, available, writable, kind);
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
    use PropertyValueType::*;

    match property_value_type(code) {
        // Formatted numeric values
        Aperture => format_aperture(raw),
        ShutterSpeed => format_shutter_speed(raw),
        Iso => format_iso_compact(raw),
        ExposureCompensation => format_exposure_comp(raw as i64),
        ColorTemperature => format_color_temp(raw),
        MovieQuality => format_movie_quality(raw),

        // Enum types
        ExposureProgram => crsdk::ExposureProgram::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        MeteringMode => crsdk::MeteringMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        FocusMode => crsdk::FocusMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        FocusArea => crsdk::FocusArea::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        SubjectRecognitionAF => crsdk::SubjectRecognitionAF::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PrioritySetInAF => crsdk::PrioritySetInAF::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        FocusTrackingStatus => crsdk::FocusTrackingStatus::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        WhiteBalance => crsdk::WhiteBalance::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        PrioritySetInAWB => crsdk::PrioritySetInAWB::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        DriveMode => crsdk::DriveMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        IntervalRecShutterType => crsdk::IntervalRecShutterType::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        FlashMode => crsdk::FlashMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        FileType => crsdk::FileType::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        ImageQuality => crsdk::ImageQuality::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        AspectRatio => crsdk::AspectRatio::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        ImageSize => crsdk::ImageSize::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        MovieFileFormat => crsdk::MovieFileFormat::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        ShutterModeStatus => crsdk::ShutterModeStatus::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        ShutterMode => crsdk::ShutterMode::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        ExposureCtrlType => crsdk::ExposureCtrlType::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        LiveViewDisplayEffect => crsdk::LiveViewDisplayEffect::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),
        SilentModeApertureDrive => crsdk::SilentModeApertureDrive::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("0x{:X}", raw)),

        // Generic toggle types
        OnOff => crsdk::OnOff::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        Switch => crsdk::Switch::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        AutoManual => crsdk::AutoManual::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),
        LockIndicator => crsdk::LockIndicator::from_raw(raw)
            .map(|v| v.to_string())
            .unwrap_or_else(|| format!("{}", raw)),

        // Raw value types
        Percentage => format!("{}%", raw),
        Integer => format!("{}", raw),
        Unknown => format!("{}", raw),
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
            let category_name = property_category(code).to_string();
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
    fn test_property_store_add_and_get_discrete() {
        let mut store = PropertyStore::new();
        store.add_property(
            DevicePropertyCode::FNumber,
            "f/2.8",
            280, // raw value
            vec!["f/1.4".into(), "f/2.8".into(), "f/4.0".into()],
            true,
            PropertyKind::Discrete,
        );

        let prop = store.get(DevicePropertyCode::FNumber).unwrap();
        assert_eq!(prop.current_value(), "f/2.8");
        assert!(prop.writable);
        assert!(!prop.is_range());
    }

    #[test]
    fn test_property_store_add_and_get_range() {
        let mut store = PropertyStore::new();
        store.add_property(
            DevicePropertyCode::AFTransitionSpeed,
            "3",
            3,
            vec!["3".into()],
            true,
            PropertyKind::Range {
                min: 1,
                max: 7,
                step: 1,
            },
        );

        let prop = store.get(DevicePropertyCode::AFTransitionSpeed).unwrap();
        assert_eq!(prop.current_value(), "3");
        assert!(prop.is_range());
        assert_eq!(prop.value_count(), 7); // 1 through 7
        assert_eq!(prop.current_index, 2); // value 3 is at index 2 (1, 2, 3)
        assert_eq!(prop.raw_value_at_index(0), Some(1));
        assert_eq!(prop.raw_value_at_index(6), Some(7));

        // Test progress
        let progress = prop.progress();
        assert!((progress - (2.0 / 6.0)).abs() < 0.01);
    }

    #[test]
    fn test_property_range_navigation() {
        let mut prop = Property::new(DevicePropertyCode::AFTransitionSpeed);
        prop.kind = PropertyKind::Range {
            min: 1,
            max: 7,
            step: 1,
        };
        prop.writable = true;
        prop.current_index = 3; // value = 4
        prop.current_raw = 4;

        // Test next
        prop.next();
        assert_eq!(prop.current_index, 4);
        assert_eq!(prop.current_raw, 5);

        // Test prev
        prop.prev();
        assert_eq!(prop.current_index, 3);
        assert_eq!(prop.current_raw, 4);

        // Test advance by multiple
        prop.advance(3);
        assert_eq!(prop.current_index, 6); // max
        assert_eq!(prop.current_raw, 7);

        // Test wrap around
        prop.advance(1);
        assert_eq!(prop.current_index, 0); // wrapped to min
        assert_eq!(prop.current_raw, 1);
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
