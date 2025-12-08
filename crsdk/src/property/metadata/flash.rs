//! Flash-related property metadata (descriptions, display names, value types).

use super::super::PropertyValueType;
use crsdk_sys::DevicePropertyCode;

/// Returns a detailed description for a flash-related property code
pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::FlashMode => {
            "Flash behavior. Auto fires when needed. Fill fires always. Slow Sync uses slow shutter with flash for ambient light. Rear Sync fires at end of exposure."
        }
        DevicePropertyCode::FlashCompensation => {
            "Adjusts flash power relative to camera's calculation. Positive values increase flash output, negative decrease it."
        }
        DevicePropertyCode::RedEyeReduction => {
            "Fires pre-flashes to constrict subjects' pupils, reducing the red-eye effect in portraits."
        }
        DevicePropertyCode::WirelessFlash => {
            "Controls wireless flash units. When enabled, triggers compatible wireless flash units remotely."
        }
        _ => "",
    }
}

/// Returns a short display name for a flash-related property code
pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::FlashMode => "Flash",
        DevicePropertyCode::FlashCompensation => "Flash ±",
        DevicePropertyCode::WirelessFlash => "Wireless Flsh",
        DevicePropertyCode::RedEyeReduction => "Red-Eye Reduc",
        _ => code.name(),
    }
}

/// Returns the value type for a flash-related property code
pub fn value_type(code: DevicePropertyCode) -> Option<PropertyValueType> {
    use PropertyValueType as V;

    Some(match code {
        DevicePropertyCode::FlashMode => V::FlashMode,
        DevicePropertyCode::FlashCompensation => V::ExposureCompensation,
        DevicePropertyCode::RedEyeReduction => V::OnOff,
        DevicePropertyCode::WirelessFlash => V::OnOff,
        _ => return None,
    })
}
