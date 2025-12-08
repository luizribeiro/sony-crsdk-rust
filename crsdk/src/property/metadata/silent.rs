//! Silent mode property metadata (descriptions, display names).

use crsdk_sys::DevicePropertyCode;

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::SilentMode => {
            "Disables all mechanical sounds and lights. Uses electronic shutter and turns off AF illuminator and flash. Essential for weddings, wildlife, and theaters."
        }
        DevicePropertyCode::SilentModeApertureDriveInAF => {
            "Controls aperture motor noise during AF in silent mode. Not Target ignores this setting. Standard balances speed and noise. Silent Priority minimizes noise but may slow AF."
        }
        DevicePropertyCode::SilentModeAutoPixelMapping => {
            "Controls automatic pixel mapping (hot pixel correction) behavior in silent mode. May be disabled to avoid mechanical noise."
        }
        DevicePropertyCode::SilentModeShutterWhenPowerOff => {
            "Controls shutter blade behavior when powering off in silent mode. Close keeps sensor protected from dust."
        }
        DevicePropertyCode::AntidustShutterWhenPowerOff => {
            "Closes shutter when powering off to protect sensor from dust. Keeps sensor clean during lens changes."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::SilentMode => "Silent",
        DevicePropertyCode::SilentModeApertureDriveInAF => "Silent: Aperture Drive",
        DevicePropertyCode::SilentModeShutterWhenPowerOff => "Silent: Shutter Power Off",
        DevicePropertyCode::SilentModeAutoPixelMapping => "Silent: Auto Pixel Mapping",
        _ => code.name(),
    }
}
