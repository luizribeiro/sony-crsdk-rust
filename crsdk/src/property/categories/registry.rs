//! Registry macro for category definitions.
//!
//! This module provides the `define_categories!` macro which:
//! - Maps `PropertyCategory` variants to their implementing modules
//! - Generates `property_category()` function
//! - Generates `description()`, `display_name()`, `value_type()` functions
//! - Provides compile-time validation via tests

/// Defines the category registry and generates lookup functions.
macro_rules! define_categories {
    (
        $($variant:ident => $module:ty),+ $(,)?
    ) => {
        /// Find a property definition by code across all categories.
        fn find_property(code: DevicePropertyCode) -> Option<(&'static PropertyDef, PropertyCategory)> {
            $(
                for prop in <$module as Category>::PROPERTIES {
                    if prop.code == code {
                        return Some((prop, PropertyCategory::$variant));
                    }
                }
            )+
            None
        }

        /// Get the category for a property code.
        pub fn property_category(code: DevicePropertyCode) -> PropertyCategory {
            find_property(code).map(|(_, cat)| cat).unwrap_or(PropertyCategory::Other)
        }

        /// Get a description of what a property does.
        pub fn description(code: DevicePropertyCode) -> &'static str {
            find_property(code).map(|(prop, _)| prop.description).unwrap_or("")
        }

        /// Get a human-readable display name for a property code.
        pub fn display_name(code: DevicePropertyCode) -> &'static str {
            find_property(code).map(|(prop, _)| prop.name).unwrap_or_else(|| code.name())
        }

        /// Get the value type for a property code.
        pub fn value_type(code: DevicePropertyCode) -> PropertyValueType {
            find_property(code)
                .and_then(|(prop, _)| prop.value_type)
                .unwrap_or(PropertyValueType::Unknown)
        }

        /// All registered category modules for validation.
        #[allow(dead_code)]
        const REGISTERED_CATEGORIES: &[(&str, &[PropertyDef])] = &[
            $(
                (stringify!($variant), <$module as Category>::PROPERTIES),
            )+
        ];

        #[cfg(test)]
        mod registry_tests {
            use super::*;
            use std::collections::HashSet;

            #[test]
            fn test_no_duplicate_property_codes() {
                let mut seen: HashSet<DevicePropertyCode> = HashSet::new();
                let mut duplicates: Vec<(DevicePropertyCode, &str, &str)> = Vec::new();

                for (cat_name, props) in REGISTERED_CATEGORIES {
                    for prop in *props {
                        if seen.contains(&prop.code) {
                            // Find which category already has this code
                            for (other_cat, other_props) in REGISTERED_CATEGORIES {
                                if other_props.iter().any(|p| p.code == prop.code) && other_cat != cat_name {
                                    duplicates.push((prop.code, other_cat, cat_name));
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
                let mut uncategorized = Vec::new();

                'outer: for code in DevicePropertyCode::ALL {
                    $(
                        if <$module as Category>::PROPERTIES.iter().any(|p| p.code == *code) {
                            continue 'outer;
                        }
                    )+
                    uncategorized.push(*code);
                }

                assert!(
                    uncategorized.is_empty(),
                    "Property codes not in any category:\n{:?}",
                    uncategorized
                );
            }

            #[test]
            fn test_all_properties_have_descriptions() {
                let mut missing = Vec::new();

                for code in DevicePropertyCode::ALL {
                    if description(*code).is_empty() {
                        missing.push(*code);
                    }
                }

                assert!(
                    missing.is_empty(),
                    "Properties missing descriptions:\n{:?}",
                    missing
                );
            }

            #[test]
            fn test_all_properties_have_display_names() {
                let mut missing = Vec::new();

                for code in DevicePropertyCode::ALL {
                    let name = display_name(*code);
                    if name == code.name() {
                        missing.push(*code);
                    }
                }

                assert!(
                    missing.is_empty(),
                    "Properties missing custom display names:\n{:?}",
                    missing
                );
            }

            #[test]
            fn test_all_properties_have_value_types() {
                let mut missing = Vec::new();

                for code in DevicePropertyCode::ALL {
                    if *code != DevicePropertyCode::Undefined
                        && value_type(*code) == PropertyValueType::Unknown
                    {
                        missing.push(*code);
                    }
                }

                assert!(
                    missing.is_empty(),
                    "Properties missing value types:\n{:?}",
                    missing
                );
            }
        }
    };
}

pub(super) use define_categories;
