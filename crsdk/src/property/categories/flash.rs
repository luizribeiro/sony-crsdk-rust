//! Flash category: flash and wireless flash properties.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

pub struct Flash;

impl Category for Flash {
    const NAME: &'static str = "Flash";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::FlashCompensation,
            "Flash Comp",
            "Adjusts flash output relative to the camera's calculated exposure. Positive values make flash brighter, negative values make it dimmer.",
            Some(V::ExposureCompensation),
        ),
        PropertyDef::new(
            C::FlashMode,
            "Flash Firing Mode",
            "Flash firing mode. Fill fires every shot. Slow Sync uses slow shutter for ambient light. Rear Sync fires at end of exposure for motion trails.",
            Some(V::FlashMode),
        ),
        PropertyDef::new(
            C::WirelessFlash,
            "Wireless Flash Ctrl",
            "Enables wireless flash control. Allows triggering off-camera flashes optically or via radio.",
            Some(V::OnOff),
        ),
    ];
}
