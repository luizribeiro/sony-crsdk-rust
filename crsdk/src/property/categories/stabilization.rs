//! Stabilization category: image stabilization properties.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

/// Image stabilization properties.
pub struct Stabilization;

impl Category for Stabilization {
    const NAME: &'static str = "Stabilization";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::ImageStabilizationSteadyShot,
            "SteadyShot",
            "In-body image stabilization. Compensates for camera shake to allow slower shutter speeds. Different modes optimize for stills vs video.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::ImageStabilizationSteadyShotAdjust,
            "SteadyShot Adjust",
            "Fine-tune stabilization for specific focal lengths when using adapted lenses without electronic contacts.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ImageStabilizationSteadyShotFocalLength,
            "SS Focal Length",
            "Focal length used for stabilization calculations.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ImageStabilizationFramingStabilizer,
            "Framing Stabilizer",
            "Framing stabilizer for maintaining consistent framing during video.",
            Some(V::Integer),
        ),
    ];
}

crate::register_category!(Stabilization);
