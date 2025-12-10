//! Display category: monitor, viewfinder, and display properties.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

/// Display, monitor, and viewfinder properties.
pub struct Display;

impl Category for Display {
    const NAME: &'static str = "Display";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::LiveViewDisplayEffect,
            "LV Display Effect",
            "Shows exposure, white balance, and creative style effects in live view. Turn off for accurate composition in challenging lighting.",
            Some(V::LiveViewDisplayEffect),
        ),
        PropertyDef::new(
            C::DispModeSetting,
            "Display Mode Setting",
            "Display information mode setting.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::DispMode,
            "Display Mode",
            "Current display mode.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::SelectFinder,
            "Finder/Monitor",
            "Choose between viewfinder and monitor display.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MonitorLUTSetting,
            "Monitor LUT",
            "LUT applied to monitor output for color grading preview.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::MonitoringOutputDisplayHDMI,
            "HDMI Monitor",
            "HDMI monitoring output display settings.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::MonitoringOutputDisplaySDI,
            "SDI Monitor",
            "SDI monitoring output display settings.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::MonitoringOutputDisplaySetting1,
            "Monitor Setting 1",
            "Monitoring output display setting slot 1.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MonitoringOutputDisplaySetting2,
            "Monitor Setting 2",
            "Monitoring output display setting slot 2.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MonitoringOutputDisplaySettingDestAssign,
            "Monitor Assign",
            "Destination assignment for monitoring output settings.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MonitorLUTSetting1,
            "LUT 1",
            "LUT setting slot 1 for monitor output.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MonitorLUTSetting2,
            "LUT 2",
            "LUT setting slot 2 for monitor output.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MonitorLUTSetting3,
            "LUT 3",
            "LUT setting slot 3 for monitor output.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MonitorLUTSettingOutputDestAssign,
            "LUT Assign",
            "LUT output destination assignment.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::GridLineDisplay,
            "Grid Lines",
            "Show composition grid lines on screen.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::GridLineDisplayPlayback,
            "Grid (Playback)",
            "Show grid lines during playback.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::GridLineType,
            "Grid Type",
            "Type of grid pattern (rule of thirds, square, diagonal, etc.).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::DeSqueezeDisplayRatio,
            "De-Squeeze",
            "Aspect correction for anamorphic lenses. Shows unsqueezed preview.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::EnlargeScreenSetting,
            "Enlarge Screen",
            "Magnification setting for enlarged preview.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::LiveViewStatus,
            "LV Status",
            "Current live view status.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::LiveViewProtocol,
            "LV Protocol",
            "Protocol used for live view streaming.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::LiveViewArea,
            "LV Area",
            "Area shown in live view (full or cropped).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::TCUBDisplaySetting,
            "TC/UB Display",
            "Timecode/User Bit display settings.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MonitorBrightnessType,
            "Monitor Brightness",
            "Monitor brightness adjustment mode.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::MonitorBrightnessManual,
            "Brightness Level",
            "Manual monitor brightness level.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::DisplayQualityFinder,
            "Finder Quality",
            "Display quality setting for viewfinder.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::GammaDisplayAssist,
            "Gamma Assist",
            "Gamma display assist for log footage preview.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::GammaDisplayAssistType,
            "Gamma Assist Type",
            "Type of gamma display assistance.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::DispModeCandidate,
            "Disp Mode Options",
            "Available display mode options.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::DisplayedMenuStatus,
            "Menu Status",
            "Status of displayed menu.",
            Some(V::Integer),
        ),
    ];
}

crate::register_category!(Display);
