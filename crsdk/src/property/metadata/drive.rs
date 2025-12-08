//! Drive mode property metadata (descriptions, display names, value types).

use super::super::PropertyValueType;
use crsdk_sys::DevicePropertyCode;

/// Returns a human-readable description for drive-related property codes
pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::DriveMode => {
            "Single shot takes one photo per press. Continuous shoots multiple frames while held. Self-timer delays the shot. Bracket takes multiple exposures."
        }
        DevicePropertyCode::BracketOrder => {
            "Sequence of bracketed exposures. 0/−/+ starts with normal exposure. −/0/+ starts with underexposed."
        }
        DevicePropertyCode::BulbTimerSetting => {
            "Sets a fixed exposure time for bulb mode, allowing long exposures without holding the shutter button."
        }
        DevicePropertyCode::ShootingSelfTimerStatus => {
            "Indicates whether a self-timer shot is currently in progress or pending."
        }
        _ => "",
    }
}

/// Returns a short display name for drive-related property codes
pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::DriveMode => "Drive",
        DevicePropertyCode::BracketOrder => "Bracket Seq",
        DevicePropertyCode::BulbTimerSetting => "Bulb Timer",
        DevicePropertyCode::ShootingSelfTimerStatus => "Timer Active",
        _ => code.name(),
    }
}

/// Returns the property value type for drive-related property codes
pub fn value_type(code: DevicePropertyCode) -> Option<PropertyValueType> {
    use PropertyValueType as V;

    Some(match code {
        DevicePropertyCode::DriveMode => V::DriveMode,
        DevicePropertyCode::BracketOrder => V::Integer,
        DevicePropertyCode::BulbTimerSetting => V::Integer,
        DevicePropertyCode::ShootingSelfTimerStatus => V::Integer,
        _ => return None,
    })
}
