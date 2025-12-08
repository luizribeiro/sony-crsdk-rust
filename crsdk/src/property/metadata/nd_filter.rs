//! ND filter property metadata (descriptions, display names).

use crsdk_sys::DevicePropertyCode;

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::NDFilter => {
            "Built-in neutral density filter. Reduces light entering the lens, allowing wider apertures or slower shutters in bright conditions."
        }
        DevicePropertyCode::NDFilterMode => {
            "ND filter behavior. Auto engages as needed. Manual gives direct control. Variable ND allows smooth adjustment."
        }
        DevicePropertyCode::NDFilterModeSetting => {
            "Setting for ND filter mode. Auto, manual, or variable ND options."
        }
        DevicePropertyCode::NDFilterValue => {
            "Current ND filter value/strength. Higher values block more light."
        }
        DevicePropertyCode::NDFilterSwitchingSetting => {
            "Controls how the ND filter switches between states."
        }
        DevicePropertyCode::NDFilterPositionSetting => {
            "Position of the variable ND filter. Adjusts the amount of light reduction."
        }
        DevicePropertyCode::NDFilterOpticalDensityValue => {
            "Optical density of the ND filter. Measured in stops of light reduction."
        }
        DevicePropertyCode::NDFilterUnitSetting => {
            "Display unit for ND filter values (stops, optical density, etc.)."
        }
        DevicePropertyCode::NDFilterPresetSelect => {
            "Selects which ND filter preset to use (1, 2, or 3)."
        }
        DevicePropertyCode::NDFilterPreset1Value
        | DevicePropertyCode::NDFilterPreset2Value
        | DevicePropertyCode::NDFilterPreset3Value => {
            "Stored ND filter value for quick recall."
        }
        DevicePropertyCode::ManualInputForNDFilterValue => {
            "Manual entry of a specific ND filter value."
        }
        DevicePropertyCode::PushAutoNDFilter => {
            "Temporarily engages auto ND filter while button is pressed."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::NDFilter => "ND Filter",
        DevicePropertyCode::NDFilterMode => "ND Mode",
        DevicePropertyCode::NDFilterModeSetting => "ND Mode Setting",
        DevicePropertyCode::NDFilterValue => "ND Value",
        DevicePropertyCode::NDFilterSwitchingSetting => "ND Switching",
        DevicePropertyCode::NDFilterPositionSetting => "ND Position",
        DevicePropertyCode::NDFilterOpticalDensityValue => "ND Density",
        DevicePropertyCode::NDFilterUnitSetting => "ND Unit",
        DevicePropertyCode::NDFilterPresetSelect => "ND Preset",
        DevicePropertyCode::NDFilterPreset1Value => "ND Preset 1",
        DevicePropertyCode::NDFilterPreset2Value => "ND Preset 2",
        DevicePropertyCode::NDFilterPreset3Value => "ND Preset 3",
        DevicePropertyCode::ManualInputForNDFilterValue => "ND Manual Input",
        DevicePropertyCode::PushAutoNDFilter => "Push Auto ND",
        _ => code.name(),
    }
}
