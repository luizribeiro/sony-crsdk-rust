//! Zoom category: optical and digital zoom properties.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

/// Zoom properties.
pub struct Zoom;

impl Category for Zoom {
    const NAME: &'static str = "Zoom";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::ZoomScale,
            "Zoom Level",
            "Current zoom magnification level.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomSetting,
            "Zoom Type",
            "Zoom behavior settings. Clear Image Zoom extends range with minimal quality loss. Digital Zoom further extends but with visible quality reduction.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomOperation,
            "Zoom Op",
            "Zoom control command. Positive values zoom in (telephoto), negative values zoom out (wide).",
            Some(V::ZoomOperation),
        ),
        PropertyDef::new(
            C::DigitalZoomScale,
            "Digital Zoom",
            "Digital zoom magnification beyond optical zoom range.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomDistance,
            "Focal Length",
            "Current focal length or zoom distance.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomDistanceUnitSetting,
            "Zoom Unit",
            "Unit for displaying zoom distance (mm, equivalent, etc.).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomPositionSetting,
            "Zoom Position",
            "Target zoom position setting.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomOperationStatus,
            "Zoom Status",
            "Current zoom operation state (idle, zooming in/out).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomBarInformation,
            "Zoom Bar",
            "Zoom bar display information.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomTypeStatus,
            "Zoom Type",
            "Current zoom type (optical, clear image, digital).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomSpeedRange,
            "Zoom Speed",
            "Range of available zoom speeds.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomDrivingStatus,
            "Zoom Drive",
            "Zoom motor driving status.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomPositionCurrentValue,
            "Zoom Pos (Curr)",
            "Current zoom position value.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomOperationWithInt16,
            "Zoom Op (16-bit)",
            "Zoom operation with 16-bit precision control.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::RemoconZoomSpeedType,
            "Remote Zoom Speed",
            "Zoom speed type for remote control.",
            Some(V::RemoconZoomSpeedType),
        ),
    ];
}

crate::register_category!(Zoom);
