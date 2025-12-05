//! Flash-related property types and metadata.

use crsdk_sys::DevicePropertyCode;

/// Flash mode settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FlashMode {
    Auto = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_Auto as u16,
    Off = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_Off as u16,
    Fill = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_Fill as u16,
    ExternalSync = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_ExternalSync as u16,
    SlowSync = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_SlowSync as u16,
    RearSync = crsdk_sys::SCRSDK::CrFlashMode_CrFlash_RearSync as u16,
}

impl FlashMode {
    pub fn as_raw(self) -> u64 {
        self as u64
    }

    pub fn from_raw(value: u64) -> Option<Self> {
        use crsdk_sys::SCRSDK::*;
        Some(match value as u16 {
            CrFlashMode_CrFlash_Auto => Self::Auto,
            CrFlashMode_CrFlash_Off => Self::Off,
            CrFlashMode_CrFlash_Fill => Self::Fill,
            CrFlashMode_CrFlash_ExternalSync => Self::ExternalSync,
            CrFlashMode_CrFlash_SlowSync => Self::SlowSync,
            CrFlashMode_CrFlash_RearSync => Self::RearSync,
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
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::FlashMode => "Flash Mode",
        DevicePropertyCode::FlashCompensation => "Flash Compensation",
        DevicePropertyCode::WirelessFlash => "Wireless Flash",
        DevicePropertyCode::RedEyeReduction => "Red Eye Reduction",
        _ => code.name(),
    }
}
