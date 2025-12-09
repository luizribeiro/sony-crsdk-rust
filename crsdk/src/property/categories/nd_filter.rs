//! ND filter category: neutral density filter properties.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

/// ND filter properties.
pub struct NdFilter;

impl Category for NdFilter {
    const NAME: &'static str = "ND Filter";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::NDFilter,
            "ND Filter",
            "Built-in ND filter on/off. Reduces light entering the lens without affecting color.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::NDFilterModeSetting,
            "ND Mode",
            "ND filter control mode. Auto adjusts automatically. Manual gives direct control.",
            Some(V::AutoManual),
        ),
        PropertyDef::new(
            C::NDFilterValue,
            "ND Value",
            "Current ND filter density value.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::NDFilterSwitchingSetting,
            "ND Switching",
            "ND filter switching behavior settings.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::NDFilterUnitSetting,
            "ND Unit",
            "Unit for displaying ND filter value (stops, density, etc.).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::NDFilterOpticalDensityValue,
            "ND Density",
            "ND filter optical density value.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::NDFilterMode,
            "ND Mode Status",
            "Current ND filter operating mode.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::NDFilterPresetSelect,
            "ND Preset",
            "Select ND filter preset.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::NDFilterPreset1Value,
            "ND Preset 1",
            "ND filter preset 1 value.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::NDFilterPreset2Value,
            "ND Preset 2",
            "ND filter preset 2 value.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::NDFilterPreset3Value,
            "ND Preset 3",
            "ND filter preset 3 value.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::NDFilterPositionSetting,
            "ND Position",
            "ND filter position setting.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ManualInputForNDFilterValue,
            "ND Manual",
            "Manual input for ND filter value.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PushAutoNDFilter,
            "Push Auto ND",
            "Temporarily engage auto ND filter.",
            Some(V::Integer),
        ),
    ];
}

crate::register_category!(NdFilter);
