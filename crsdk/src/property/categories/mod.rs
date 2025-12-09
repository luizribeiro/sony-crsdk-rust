//! Category-based property organization.
//!
//! Each category module defines a struct implementing [`Category`] and registers itself
//! using the [`register_category!`] macro. Categories are automatically discovered at
//! link time via distributed slices.

pub mod audio;
pub mod custom_buttons;
pub mod display;
pub mod drive;
pub mod exposure;
pub mod flash;
pub mod focus;
pub mod image;
pub mod lens;
pub mod media;
pub mod metering;
pub mod movie;
pub mod nd_filter;
pub mod other;
pub mod picture_profile;
pub mod power;
pub mod silent;
pub mod stabilization;
pub mod white_balance;
pub mod zoom;

use crsdk_sys::DevicePropertyCode;
use linkme::distributed_slice;

use super::values::PropertyValueType;

/// Distributed slice collecting all registered categories.
#[distributed_slice]
pub static CATEGORIES: [CategoryRegistration];

/// Identifier for a property category.
///
/// This is a newtype wrapper around a static string, derived from each category's NAME.
/// Use the category struct's `ID` constant to get the identifier, e.g., `Exposure::ID`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PropertyCategoryId(pub &'static str);

impl PropertyCategoryId {
    /// Get the category name as a string.
    pub const fn name(self) -> &'static str {
        self.0
    }
}

impl core::fmt::Display for PropertyCategoryId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Registration entry for a category, collected at link time.
pub struct CategoryRegistration {
    /// The category identifier.
    pub id: PropertyCategoryId,
    /// All properties belonging to this category.
    pub properties: &'static [PropertyDef],
}

/// Definition of a single property's metadata.
#[derive(Debug, Clone, Copy)]
pub struct PropertyDef {
    /// The property code this definition is for.
    pub code: DevicePropertyCode,
    /// Short UI-friendly display name.
    pub name: &'static str,
    /// Detailed description of what the property does.
    pub description: &'static str,
    /// Value type for formatting/parsing. None means Unknown.
    pub value_type: Option<PropertyValueType>,
}

impl PropertyDef {
    /// Create a new property definition.
    pub const fn new(
        code: DevicePropertyCode,
        name: &'static str,
        description: &'static str,
        value_type: Option<PropertyValueType>,
    ) -> Self {
        Self {
            code,
            name,
            description,
            value_type,
        }
    }

    /// Create a property definition with Integer value type.
    pub const fn integer(
        code: DevicePropertyCode,
        name: &'static str,
        description: &'static str,
    ) -> Self {
        Self::new(code, name, description, Some(PropertyValueType::Integer))
    }
}

/// Trait implemented by each category module.
pub trait Category {
    /// Human-readable name for this category.
    const NAME: &'static str;
    /// Category identifier, derived from NAME.
    const ID: PropertyCategoryId = PropertyCategoryId(Self::NAME);
    /// All properties belonging to this category with their metadata.
    const PROPERTIES: &'static [PropertyDef];
}

/// Macro to register a category in the distributed slice.
#[macro_export]
macro_rules! register_category {
    ($ty:ty) => {
        #[::linkme::distributed_slice($crate::property::categories::CATEGORIES)]
        static _CATEGORY_REGISTRATION: $crate::property::categories::CategoryRegistration =
            $crate::property::categories::CategoryRegistration {
                id: <$ty as $crate::property::categories::Category>::ID,
                properties: <$ty as $crate::property::categories::Category>::PROPERTIES,
            };
    };
}

pub use register_category;

/// Find a property definition by code across all registered categories.
fn find_property(code: DevicePropertyCode) -> Option<(&'static PropertyDef, PropertyCategoryId)> {
    for reg in CATEGORIES {
        for prop in reg.properties {
            if prop.code == code {
                return Some((prop, reg.id));
            }
        }
    }
    None
}

/// Get the category for a property code.
pub fn property_category(code: DevicePropertyCode) -> PropertyCategoryId {
    find_property(code)
        .map(|(_, cat)| cat)
        .unwrap_or(other::Other::ID)
}

/// Get a description of what a property does.
pub fn description(code: DevicePropertyCode) -> &'static str {
    find_property(code)
        .map(|(prop, _)| prop.description)
        .unwrap_or("")
}

/// Get a human-readable display name for a property code.
pub fn display_name(code: DevicePropertyCode) -> &'static str {
    find_property(code)
        .map(|(prop, _)| prop.name)
        .unwrap_or_else(|| code.name())
}

/// Get the value type for a property code.
pub fn value_type(code: DevicePropertyCode) -> PropertyValueType {
    find_property(code)
        .and_then(|(prop, _)| prop.value_type)
        .unwrap_or(PropertyValueType::Unknown)
}

/// Get all registered category IDs.
pub fn all_categories() -> impl Iterator<Item = PropertyCategoryId> {
    CATEGORIES.iter().map(|reg| reg.id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_no_duplicate_property_codes() {
        let mut seen: HashSet<DevicePropertyCode> = HashSet::new();
        let mut duplicates: Vec<(DevicePropertyCode, &str, &str)> = Vec::new();

        for reg in CATEGORIES {
            for prop in reg.properties {
                if seen.contains(&prop.code) {
                    for other_reg in CATEGORIES {
                        if other_reg.properties.iter().any(|p| p.code == prop.code)
                            && other_reg.id != reg.id
                        {
                            duplicates.push((prop.code, other_reg.id.name(), reg.id.name()));
                            break;
                        }
                    }
                }
                seen.insert(prop.code);
            }
        }

        assert!(
            duplicates.is_empty(),
            "Property codes appear in multiple categories:\n{}",
            duplicates
                .iter()
                .map(|(code, cat1, cat2)| format!("  {:?} in both {} and {}", code, cat1, cat2))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    #[test]
    fn test_all_properties_have_category() {
        let mut categorized: HashSet<DevicePropertyCode> = HashSet::new();
        for reg in CATEGORIES {
            for prop in reg.properties {
                categorized.insert(prop.code);
            }
        }

        let uncategorized: Vec<_> = DevicePropertyCode::ALL
            .iter()
            .filter(|code| !categorized.contains(code))
            .collect();

        assert!(
            uncategorized.is_empty(),
            "Property codes not in any category:\n{:?}",
            uncategorized
        );
    }

    #[test]
    fn test_all_properties_have_descriptions() {
        let missing: Vec<_> = DevicePropertyCode::ALL
            .iter()
            .filter(|code| description(**code).is_empty())
            .collect();

        assert!(
            missing.is_empty(),
            "Properties missing descriptions:\n{:?}",
            missing
        );
    }

    #[test]
    fn test_all_properties_have_display_names() {
        let missing: Vec<_> = DevicePropertyCode::ALL
            .iter()
            .filter(|code| display_name(**code) == code.name())
            .collect();

        assert!(
            missing.is_empty(),
            "Properties missing custom display names:\n{:?}",
            missing
        );
    }

    #[test]
    fn test_all_properties_have_value_types() {
        let missing: Vec<_> = DevicePropertyCode::ALL
            .iter()
            .filter(|code| {
                **code != DevicePropertyCode::Undefined
                    && value_type(**code) == PropertyValueType::Unknown
            })
            .collect();

        assert!(
            missing.is_empty(),
            "Properties missing value types:\n{:?}",
            missing
        );
    }
}
