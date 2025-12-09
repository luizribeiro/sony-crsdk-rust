//! Metering category: light metering properties.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

/// Light metering properties.
pub struct Metering;

impl Category for Metering {
    const NAME: &'static str = "Metering";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::MeteringMode,
            "Meter Mode",
            "How the camera measures light. Multi uses the whole frame. Center-weighted prioritizes the middle. Spot measures only a small area, useful for backlit subjects.",
            Some(V::MeteringMode),
        ),
        PropertyDef::new(
            C::FacePriorityInMultiMetering,
            "Face Priority Metering",
            "Prioritizes detected faces when metering exposure. Ensures faces are properly exposed even in challenging lighting.",
            Some(V::Switch),
        ),
    ];
}

crate::register_category!(Metering);
