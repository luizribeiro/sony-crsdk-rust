//! White balance category: color temperature and white balance properties.

use super::{Category, PropertyCategory, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

/// White balance and color temperature properties.
pub struct WhiteBalance;

impl Category for WhiteBalance {
    const CATEGORY: PropertyCategory = PropertyCategory::WhiteBalance;
    const NAME: &'static str = "White Balance";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::AWBL,
            "AWB Lock",
            "Auto White Balance Lock. Holds the current AWB setting to prevent changes.",
            Some(V::LockIndicator),
        ),
        PropertyDef::new(
            C::WhiteBalance,
            "WB Preset",
            "White balance preset selection. Auto adjusts automatically. Daylight, Shade, Cloudy, Tungsten, Fluorescent for specific lighting. Custom for manual measurement.",
            Some(V::WhiteBalance),
        ),
        PropertyDef::new(
            C::Colortemp,
            "Color Temp (K)",
            "Color temperature in Kelvin. Lower values (2500K) are warmer/orange. Higher values (10000K) are cooler/blue. 5500K is daylight neutral.",
            Some(V::ColorTemperature),
        ),
        PropertyDef::new(
            C::ColorTuningAB,
            "WB Tune A-B",
            "Fine-tune white balance on Amber-Blue axis. Positive shifts toward amber, negative toward blue.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ColorTuningGM,
            "WB Tune G-M",
            "Fine-tune white balance on Green-Magenta axis. Positive shifts toward green, negative toward magenta.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ColortempStep,
            "Color Temp Step",
            "Step size for color temperature adjustments. Smaller steps allow finer control.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::WhiteBalanceModeSetting,
            "WB Mode",
            "White balance control mode. Auto lets camera adjust. Manual gives direct control.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::WhiteBalanceTint,
            "WB Tint",
            "Tint adjustment on green-magenta axis. Complements color temperature setting.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::WhiteBalanceTintStep,
            "Tint Step",
            "Step size for tint adjustments.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AWB,
            "AWB Status",
            "Auto White Balance lock status. Shows if AWB is currently locked or adjusting.",
            Some(V::LockIndicator),
        ),
        PropertyDef::new(
            C::WhiteBalanceSwitch,
            "WB Switch",
            "Quick switch between white balance presets.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::WhiteBalancePresetColorTemperature,
            "WB Preset Temp",
            "Color temperature for the current white balance preset.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::WhiteBalanceRGain,
            "WB R Gain",
            "Red channel gain adjustment for manual white balance.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::WhiteBalanceBGain,
            "WB B Gain",
            "Blue channel gain adjustment for manual white balance.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::WhiteBalanceOffsetColorTempATW,
            "ATW Temp Offset",
            "Color temperature offset for Auto Tracking White balance.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::WhiteBalanceOffsetTintATW,
            "ATW Tint Offset",
            "Tint offset for Auto Tracking White balance.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::WhiteBalanceOffsetSetting,
            "WB Offset Setting",
            "White balance offset configuration.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::WhiteBalanceOffsetColorTemp,
            "WB Temp Offset",
            "Color temperature offset from the base white balance.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PrioritySetInAWB,
            "AWB Priority",
            "Priority setting for Auto White Balance. Ambience preserves scene color. White makes whites neutral.",
            Some(V::PrioritySetInAWB),
        ),
        PropertyDef::new(
            C::CustomWBSizeSetting,
            "Custom WB Size",
            "Size of the area used for custom white balance measurement.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CustomWBCaptureStandby,
            "Custom WB Standby",
            "Prepares camera for custom white balance capture.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CustomWBCaptureStandbyCancel,
            "Custom WB Cancel",
            "Cancels custom white balance capture standby.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CustomWBCapture,
            "Custom WB Measure",
            "Captures a custom white balance reference from the current scene.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CustomWBExecutionState,
            "Custom WB State",
            "Current state of custom white balance capture process.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CustomWBCapturableArea,
            "Custom WB Area",
            "Area available for custom white balance measurement.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CustomWBCaptureFrameSize,
            "Custom WB Frame",
            "Frame size for custom white balance capture.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CustomWBCaptureOperation,
            "Custom WB Op",
            "Custom white balance capture operation control.",
            Some(V::Integer),
        ),
    ];
}

crate::register_category!(WhiteBalance);
