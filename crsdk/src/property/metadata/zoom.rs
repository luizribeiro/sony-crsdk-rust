//! Zoom property metadata (descriptions, display names).

use crsdk_sys::DevicePropertyCode;

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::ZoomScale => {
            "Current zoom magnification level. Shows how much the lens is zoomed in from its widest setting."
        }
        DevicePropertyCode::ZoomSetting => {
            "Zoom behavior settings. Controls whether digital zoom is enabled and how zoom speed responds to input."
        }
        DevicePropertyCode::DigitalZoomScale => {
            "Digital zoom magnification. Extends beyond optical zoom but reduces image quality as it crops and enlarges."
        }
        DevicePropertyCode::ZoomOperation => {
            "Controls zoom movement. Use to zoom in or out programmatically."
        }
        DevicePropertyCode::ZoomOperationWithInt16 => {
            "Zoom control with 16-bit precision. Allows finer control over zoom speed and position."
        }
        DevicePropertyCode::ZoomOperationStatus => {
            "Current zoom motor status. Shows if zoom is moving, stopped, or at a limit."
        }
        DevicePropertyCode::ZoomPositionSetting => {
            "Target zoom position. Set a specific zoom level for the lens to move to."
        }
        DevicePropertyCode::ZoomPositionCurrentValue => {
            "Current zoom lens position. Shows the exact position of the zoom element."
        }
        DevicePropertyCode::ZoomDrivingStatus => {
            "Zoom motor driving state. Indicates if zoom is actively moving."
        }
        DevicePropertyCode::ZoomSpeedRange => {
            "Available zoom speed range. Defines minimum and maximum zoom motor speeds."
        }
        DevicePropertyCode::ZoomDistance => {
            "Focal length distance. Shows the current effective focal length of the zoom lens."
        }
        DevicePropertyCode::ZoomDistanceUnitSetting => {
            "Unit for displaying zoom distance. Choose between millimeters or other units."
        }
        DevicePropertyCode::ZoomBarInformation => {
            "Visual zoom bar indicator data. Used to display zoom position on screen."
        }
        DevicePropertyCode::ZoomTypeStatus => {
            "Type of zoom being used. Indicates optical, digital, or combined zoom mode."
        }
        DevicePropertyCode::DigitalExtenderMagnificationSetting => {
            "Digital extender zoom factor. Crops and enlarges the image beyond optical zoom range."
        }
        DevicePropertyCode::TeleWideLeverValueCapability => {
            "Range of values supported by the tele/wide zoom lever."
        }
        DevicePropertyCode::RemoconZoomSpeedType => {
            "Zoom speed setting for remote control. Adjusts how fast the zoom responds to remote input."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::ZoomScale => "Zoom",
        DevicePropertyCode::ZoomOperation => "Zoom Op",
        DevicePropertyCode::ZoomOperationWithInt16 => "Zoom Op (16-bit)",
        DevicePropertyCode::ZoomOperationStatus => "Zoom Op Status",
        DevicePropertyCode::ZoomPositionSetting => "Zoom Position",
        DevicePropertyCode::ZoomPositionCurrentValue => "Zoom Pos (Current)",
        DevicePropertyCode::ZoomDrivingStatus => "Zoom Drive Status",
        DevicePropertyCode::ZoomBarInformation => "Zoom Bar Info",
        DevicePropertyCode::ZoomTypeStatus => "Zoom Type",
        DevicePropertyCode::DigitalZoomScale => "Digital Zoom",
        DevicePropertyCode::RemoconZoomSpeedType => "Remote Zoom Speed",
        DevicePropertyCode::ZoomSetting => "Zoom Set",
        DevicePropertyCode::ZoomSpeedRange => "Zoom Speed",
        DevicePropertyCode::ZoomDistance => "Zoom Dist",
        DevicePropertyCode::ZoomDistanceUnitSetting => "Zoom Distance Unit",
        _ => code.name(),
    }
}
