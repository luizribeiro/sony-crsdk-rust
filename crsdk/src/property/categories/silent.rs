//! Silent category: quiet shooting properties.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

pub struct Silent;

impl Category for Silent {
    const NAME: &'static str = "Silent";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::SilentMode,
            "Silent Shoot",
            "Silent shooting mode. Disables mechanical shutter and most sounds for unobtrusive shooting. Uses electronic shutter which may cause rolling shutter artifacts.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::SilentModeAutoPixelMapping,
            "Silent Pixel Map",
            "Automatic pixel mapping behavior in silent mode.",
            Some(V::Integer),
        ),
    ];
}
