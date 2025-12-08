//! Lens property metadata (descriptions, display names).

use crsdk_sys::DevicePropertyCode;

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::LensModelName => {
            "The currently attached lens model. Useful for metadata and ensuring correct lens-specific corrections."
        }
        DevicePropertyCode::LensCompensationShading => {
            "Corrects vignetting (corner darkening) caused by the lens. Automatic correction based on lens data."
        }
        DevicePropertyCode::LensCompensationChromaticAberration => {
            "Corrects color fringing at high-contrast edges. Reduces purple/green outlines caused by lens optics."
        }
        DevicePropertyCode::LensCompensationDistortion => {
            "Corrects barrel or pincushion distortion. Makes straight lines appear straight, especially with wide-angle lenses."
        }
        DevicePropertyCode::LensAssignableButton1 => {
            "Custom button on the lens that can be assigned to frequently used functions."
        }
        DevicePropertyCode::ButtonAssignmentLensAssignable1 => {
            "The function currently assigned to the lens assignable button."
        }
        DevicePropertyCode::LensSerialNumber => {
            "Serial number of the attached lens. Useful for metadata and tracking equipment."
        }
        DevicePropertyCode::LensVersionNumber => {
            "Firmware version of the attached lens. Check for updates to fix bugs or add features."
        }
        DevicePropertyCode::LensAssignableButtonIndicator1 => {
            "Status indicator showing the current function assigned to lens button 1."
        }
        DevicePropertyCode::LensInformationEnableStatus => {
            "Whether lens information (model, focal length, etc.) is available from the attached lens."
        }
        DevicePropertyCode::ReleaseWithoutLens => {
            "Allows shutter release without a lens attached. Enable for adapted lenses without electronic contacts."
        }
        DevicePropertyCode::LensCompensationBreathing => {
            "Compensates for focus breathing where focal length shifts during focusing. Keeps framing consistent when pulling focus in video."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::LensModelName => "Lens Model",
        DevicePropertyCode::LensCompensationShading => "Lens Comp: Shading",
        DevicePropertyCode::LensCompensationChromaticAberration => "Lens Comp: Chromatic",
        DevicePropertyCode::LensCompensationDistortion => "Lens Comp: Distortion",
        DevicePropertyCode::LensCompensationBreathing => "Lens Comp: Breathing",
        DevicePropertyCode::FocalDistanceInMeter => "Focal Distance (m)",
        DevicePropertyCode::FocalDistanceInFeet => "Focal Distance (ft)",
        DevicePropertyCode::FocalDistanceUnitSetting => "Focal Distance Unit",
        DevicePropertyCode::LensAssignableButton1 => "Lens Btn",
        DevicePropertyCode::ButtonAssignmentLensAssignable1 => "Assign Lens Btn",
        DevicePropertyCode::LensAssignableButtonIndicator1 => "Lens Button 1",
        DevicePropertyCode::LensInformationEnableStatus => "Lens Info Status",
        DevicePropertyCode::LensSerialNumber => "Lens Serial #",
        DevicePropertyCode::LensVersionNumber => "Lens Version",
        DevicePropertyCode::ReleaseWithoutLens => "Lensless Release",
        _ => code.name(),
    }
}
