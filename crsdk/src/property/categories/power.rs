//! Power category: battery and power properties.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

/// Battery and power properties.
pub struct Power;

impl Category for Power {
    const NAME: &'static str = "Power";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::AutoPowerOffTemperature,
            "Auto Off Temp",
            "Temperature threshold for automatic power-off to prevent overheating.",
            Some(V::AutoPowerOffTemperature),
        ),
        PropertyDef::new(
            C::BatteryRemainDisplayUnit,
            "Battery Display",
            "How battery remaining is displayed (percentage, time, etc.).",
            Some(V::BatteryRemainDisplayUnit),
        ),
        PropertyDef::new(
            C::PowerSource,
            "Power Type",
            "Current power source (DC, battery, or PoE).",
            Some(V::PowerSource),
        ),
        PropertyDef::new(
            C::SilentModeShutterWhenPowerOff,
            "Silent Shutter Off",
            "Shutter behavior when powering off in silent mode.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FTPPowerSave,
            "FTP Pwr Save",
            "Power saving mode for FTP operations.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::USBPowerSupply,
            "USB Power",
            "USB power supply settings for charging or powering the camera.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AntidustShutterWhenPowerOff,
            "Dust Protection",
            "Closes shutter when powering off to protect sensor from dust.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::BatteryRemain,
            "Battery %",
            "Current battery charge level as a percentage.",
            Some(V::Percentage),
        ),
        PropertyDef::new(
            C::BatteryLevel,
            "Battery Indicator",
            "Battery charge indicator level.",
            Some(V::Percentage),
        ),
        PropertyDef::new(
            C::BatteryRemainingInMinutes,
            "Battery (min)",
            "Estimated remaining battery life in minutes.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::BatteryRemainingInVoltage,
            "Battery (V)",
            "Battery remaining capacity measured in voltage.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::DCVoltage,
            "DC Voltage",
            "DC power supply voltage when using external power.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::DeviceOverheatingState,
            "Overheat Status",
            "Camera overheating status and warning level.",
            Some(V::DeviceOverheatingState),
        ),
        PropertyDef::new(
            C::SecondBatteryRemain,
            "Battery 2 %",
            "Second battery pack charge level (for dual-battery grips).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::SecondBatteryLevel,
            "Battery 2 Level",
            "Second battery indicator level.",
            Some(V::Percentage),
        ),
        PropertyDef::new(
            C::TotalBatteryRemain,
            "Total Battery %",
            "Combined remaining charge from all batteries.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::TotalBatteryLevel,
            "Total Battery",
            "Combined battery level indicator.",
            Some(V::Percentage),
        ),
        PropertyDef::new(
            C::CameraPowerStatus,
            "Power Status",
            "Current power state of the camera.",
            Some(V::CameraPowerStatus),
        ),
        PropertyDef::new(
            C::RecordablePowerSources,
            "Rec Power",
            "Power sources that allow recording.",
            Some(V::Integer),
        ),
    ];
}

crate::register_category!(Power);
