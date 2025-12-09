//! Lens category: lens information and compensation properties.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

/// Lens information and compensation properties.
pub struct Lens;

impl Category for Lens {
    const NAME: &'static str = "Lens";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::ButtonAssignmentLensAssignable1,
            "Lens Btn Assign",
            "Function assigned to lens button 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::LensAssignableButton1,
            "Lens Btn 1",
            "Lens assignable button 1 setting.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::LensCompensationShading,
            "Shading Comp",
            "Corrects light falloff (vignetting) at image corners. Automatically adjusts based on lens profile.",
            Some(V::OnOff),
        ),
        PropertyDef::new(
            C::LensCompensationChromaticAberration,
            "Chrom. Aberr. Comp",
            "Corrects color fringing at high-contrast edges. Automatically adjusts based on lens profile.",
            Some(V::OnOff),
        ),
        PropertyDef::new(
            C::LensCompensationDistortion,
            "Distortion Comp",
            "Corrects barrel or pincushion distortion. Automatically adjusts based on lens profile.",
            Some(V::OnOff),
        ),
        PropertyDef::new(
            C::LensCompensationBreathing,
            "Breathing Comp",
            "Compensates for focal length shift when focusing. Keeps framing consistent during focus pulls.",
            Some(V::OnOff),
        ),
        PropertyDef::new(
            C::ReleaseWithoutLens,
            "No Lens Release",
            "Allow shutter release when no lens is attached. Useful for adapted lenses without electronic contacts.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::LensInformationEnableStatus,
            "Lens Info",
            "Whether lens information is available from the attached lens.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::LensVersionNumber,
            "Lens Firmware",
            "Firmware version of the attached lens.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::LensModelName,
            "Lens Model",
            "Model name of the attached lens.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::LensAssignableButtonIndicator1,
            "Lens Btn Ind",
            "Indicator state for lens assignable button.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::LensSerialNumber,
            "Lens Serial",
            "Serial number of the attached lens.",
            Some(V::Integer),
        ),
    ];
}

crate::register_category!(Lens);
