//! Stabilization property metadata (descriptions, display names).

use crsdk_sys::DevicePropertyCode;

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::ImageStabilizationSteadyShot => {
            "In-body image stabilization (IBIS). Compensates for camera shake, allowing slower shutter speeds when handheld."
        }
        DevicePropertyCode::MovieImageStabilizationSteadyShot => {
            "Stabilization mode for video. Active mode is more aggressive but slightly crops the image."
        }
        DevicePropertyCode::ImageStabilizationSteadyShotAdjust => {
            "Fine-tune IBIS correction strength for specific shooting conditions."
        }
        DevicePropertyCode::ImageStabilizationSteadyShotFocalLength => {
            "Manual focal length input for SteadyShot. Set when using adapted lenses that don't report focal length."
        }
        DevicePropertyCode::ImageStabilizationFramingStabilizer => {
            "Framing stabilizer maintains frame position during video. Reduces shifts between cuts."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::ImageStabilizationSteadyShot => "SteadyShot",
        DevicePropertyCode::MovieImageStabilizationSteadyShot => "Movie SteadyShot",
        DevicePropertyCode::MovieImageStabilizationLevel => "SteadyShot Level",
        DevicePropertyCode::ImageStabilizationSteadyShotAdjust => "SteadyShot Adjust",
        DevicePropertyCode::ImageStabilizationSteadyShotFocalLength => "SteadyShot Focal Length",
        DevicePropertyCode::ImageStabilizationFramingStabilizer => "Framing Stabilizer",
        _ => code.name(),
    }
}
