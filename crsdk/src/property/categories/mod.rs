//! Category-based property organization.
//!
//! Each category module defines a `PROPERTIES` const array containing all metadata
//! for the properties it owns. This is the single source of truth for:
//! - Which property codes belong to that category
//! - Human-readable descriptions and display names
//! - Value type mappings for formatting/parsing
//!
//! The [`define_categories!`] macro validates at compile time that:
//! - No property code appears in multiple categories
//! - All property codes are explicitly categorized
//! - All properties have descriptions and display names

mod registry;

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

use super::values::PropertyValueType;

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
    /// All properties belonging to this category with their metadata.
    const PROPERTIES: &'static [PropertyDef];
}

/// Semantic categories for camera properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PropertyCategory {
    /// Audio recording and monitoring properties
    Audio,
    /// Custom button assignments
    CustomButtons,
    /// Display, monitor, and viewfinder properties
    Display,
    /// Drive mode, bracketing, and timer properties
    Drive,
    /// Exposure, ISO, shutter, and aperture properties
    Exposure,
    /// Flash and wireless flash properties
    Flash,
    /// Autofocus and manual focus properties
    Focus,
    /// Image quality, format, and storage properties
    Image,
    /// Lens information and compensation properties
    Lens,
    /// Memory card and media properties
    Media,
    /// Light metering properties
    Metering,
    /// Movie recording and video properties
    Movie,
    /// ND filter properties
    NDFilter,
    /// Uncategorized properties
    Other,
    /// Picture profile and color grading properties
    PictureProfile,
    /// Battery and power properties
    Power,
    /// Silent/quiet shooting properties
    Silent,
    /// Image stabilization properties
    Stabilization,
    /// White balance and color temperature properties
    WhiteBalance,
    /// Zoom properties
    Zoom,
}

impl core::fmt::Display for PropertyCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// Use the macro to define the registry and generate lookup functions
registry::define_categories! {
    Audio => audio::Audio,
    CustomButtons => custom_buttons::CustomButtons,
    Display => display::Display,
    Drive => drive::Drive,
    Exposure => exposure::Exposure,
    Flash => flash::Flash,
    Focus => focus::Focus,
    Image => image::Image,
    Lens => lens::Lens,
    Media => media::Media,
    Metering => metering::Metering,
    Movie => movie::Movie,
    NDFilter => nd_filter::NdFilter,
    Other => other::Other,
    PictureProfile => picture_profile::PictureProfile,
    Power => power::Power,
    Silent => silent::Silent,
    Stabilization => stabilization::Stabilization,
    WhiteBalance => white_balance::WhiteBalance,
    Zoom => zoom::Zoom,
}
