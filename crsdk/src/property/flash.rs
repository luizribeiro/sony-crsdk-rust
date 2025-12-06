//! Flash-related property types and metadata.

use super::PropertyValueType;
use crsdk_sys::DevicePropertyCode;

/// Flash mode settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FlashMode {
    /// Automatic flash (fires when needed)
    Auto = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_Auto,
    /// Flash disabled
    Off = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_Off,
    /// Fill flash (always fires)
    Fill = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_Fill,
    /// External flash sync
    ExternalSync = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_ExternalSync,
    /// Slow sync (long exposure with flash)
    SlowSync = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_SlowSync,
    /// Rear curtain sync (flash at end of exposure)
    RearSync = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_RearSync,
}

impl FlashMode {
    /// Converts the enum to its raw SDK value
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    /// Converts a raw SDK value to the enum
    pub fn from_raw(value: u64) -> Option<Self> {
        use crsdk_sys::SCRSDK::*;
        Some(match value as u16 {
            x if x == CrFlashMode_CrFlash_Auto => Self::Auto,
            x if x == CrFlashMode_CrFlash_Off => Self::Off,
            x if x == CrFlashMode_CrFlash_Fill => Self::Fill,
            x if x == CrFlashMode_CrFlash_ExternalSync => Self::ExternalSync,
            x if x == CrFlashMode_CrFlash_SlowSync => Self::SlowSync,
            x if x == CrFlashMode_CrFlash_RearSync => Self::RearSync,
            _ => return None,
        })
    }
}

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

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::FlashMode => "Flash",
        DevicePropertyCode::FlashCompensation => "Flash ±",
        DevicePropertyCode::WirelessFlash => "Wireless Flsh",
        DevicePropertyCode::RedEyeReduction => "Red-Eye Reduc",
        _ => code.name(),
    }
}

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
