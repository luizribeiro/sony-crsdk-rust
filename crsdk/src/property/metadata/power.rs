//! Power/battery property metadata (descriptions, display names).

use crsdk_sys::DevicePropertyCode;

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::BatteryRemain => {
            "Remaining battery capacity as a percentage. Monitor this to avoid running out during a shoot."
        }
        DevicePropertyCode::BatteryLevel => "Battery charge level indicator. Shows approximate remaining power.",
        DevicePropertyCode::BatteryRemainingInMinutes => "Estimated recording time remaining in minutes.",
        DevicePropertyCode::BatteryRemainingInVoltage => "Battery voltage reading for precise monitoring.",
        DevicePropertyCode::BatteryRemainDisplayUnit => "Unit for battery display (percentage or time).",
        DevicePropertyCode::SecondBatteryLevel => "Charge level of the second battery (grip or backup).",
        DevicePropertyCode::SecondBatteryRemain => "Remaining capacity of the second battery.",
        DevicePropertyCode::TotalBatteryLevel => "Combined charge level from all battery sources.",
        DevicePropertyCode::TotalBatteryRemain => "Total remaining capacity from all batteries.",
        DevicePropertyCode::DCVoltage => "DC power supply voltage when using external power.",
        DevicePropertyCode::PowerSource => "Current power source—internal battery, external battery grip, or AC adapter.",
        DevicePropertyCode::RecordablePowerSources => "Power sources that provide enough power for recording.",
        DevicePropertyCode::AutoPowerOffTemperature => {
            "Temperature threshold for automatic shutdown. Higher settings allow longer recording but risk overheating damage."
        }
        DevicePropertyCode::DeviceOverheatingState => {
            "Current thermal status. Warning levels indicate the camera may shut down soon to prevent damage."
        }
        DevicePropertyCode::FTPPowerSave => "Power saving mode when using FTP transfer.",
        DevicePropertyCode::CameraPowerStatus => "Current power state of the camera.",
        DevicePropertyCode::SilentModeShutterWhenPowerOff => {
            "Controls shutter blade behavior when powering off in silent mode. Close keeps sensor protected from dust."
        }
        DevicePropertyCode::AntidustShutterWhenPowerOff => {
            "Closes shutter when powering off to protect sensor from dust. Keeps sensor clean during lens changes."
        }
        DevicePropertyCode::USBPowerSupply => {
            "USB power supply settings for connected devices. Controls power delivery over USB."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::BatteryRemain => "Battery Remaining",
        DevicePropertyCode::BatteryLevel => "Batt Level",
        DevicePropertyCode::BatteryRemainingInMinutes => "Battery (Minutes)",
        DevicePropertyCode::BatteryRemainingInVoltage => "Battery Voltage",
        DevicePropertyCode::BatteryRemainDisplayUnit => "Battery Display Unit",
        DevicePropertyCode::SecondBatteryLevel => "Battery 2 Level",
        DevicePropertyCode::SecondBatteryRemain => "Battery 2 Remaining",
        DevicePropertyCode::TotalBatteryLevel => "Total Batt",
        DevicePropertyCode::TotalBatteryRemain => "Total Battery",
        DevicePropertyCode::PowerSource => "Pwr Source",
        DevicePropertyCode::AutoPowerOffTemperature => "Auto Power Off Temp",
        DevicePropertyCode::DeviceOverheatingState => "Overheating State",
        DevicePropertyCode::RecordablePowerSources => "Rec Power",
        DevicePropertyCode::USBPowerSupply => "USB Power Supply",
        DevicePropertyCode::DCVoltage => "DC Voltage",
        DevicePropertyCode::FTPPowerSave => "FTP Pwr Save",
        DevicePropertyCode::CameraPowerStatus => "Power Status",
        DevicePropertyCode::AntidustShutterWhenPowerOff => "Antidust at Power Off",
        DevicePropertyCode::SilentModeShutterWhenPowerOff => "Silent Power Off",
        _ => code.name(),
    }
}
