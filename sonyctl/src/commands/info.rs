use crsdk::{property_display_name, EnableFlag, Result};
use crsdk_sys::DevicePropertyCode;

use super::format_value;

pub fn run(device: &crsdk::blocking::CameraDevice) -> Result<()> {
    println!("Camera Model: {:?}", device.model());

    let properties = device.get_all_properties()?;
    let writable = properties
        .iter()
        .filter(|p| p.enable_flag.is_writable())
        .count();
    let readonly = properties
        .iter()
        .filter(|p| matches!(p.enable_flag, EnableFlag::ReadOnly))
        .count();

    println!(
        "Properties:   {} total ({} writable, {} read-only)",
        properties.len(),
        writable,
        readonly
    );

    for code in [
        DevicePropertyCode::ExposureProgramMode,
        DevicePropertyCode::IsoSensitivity,
        DevicePropertyCode::FNumber,
        DevicePropertyCode::ShutterSpeed,
        DevicePropertyCode::FocusMode,
        DevicePropertyCode::WhiteBalance,
    ] {
        if let Ok(prop) = device.get_property(code) {
            println!(
                "  {}: {}",
                property_display_name(code),
                format_value(code, prop.current_value)
            );
        }
    }

    Ok(())
}
