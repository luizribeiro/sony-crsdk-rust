//! Display/monitor property metadata (descriptions, display names).

use crsdk_sys::DevicePropertyCode;

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::LiveViewDisplayEffect => {
            "Shows exposure, white balance, and creative look effects in the viewfinder/LCD. Turn off for consistent preview brightness."
        }
        DevicePropertyCode::GridLineDisplay => {
            "Overlays composition guides on the screen. Rule of thirds, square grid, or diagonal lines help frame shots."
        }
        DevicePropertyCode::GridLineType => {
            "Choose grid pattern type: rule of thirds, square grid, diagonal lines, etc."
        }
        DevicePropertyCode::MonitorBrightnessType => {
            "Screen brightness setting. Manual sets a fixed level. Auto adjusts based on ambient light. Sunny Weather boosts for outdoors."
        }
        DevicePropertyCode::MonitorBrightnessManual => {
            "Manual brightness level for the LCD monitor."
        }
        DevicePropertyCode::LiveViewStatus => {
            "Current state of live view output. Shows if preview is active."
        }
        DevicePropertyCode::LiveViewImageQuality => {
            "Quality setting for live view stream. Higher quality uses more bandwidth."
        }
        DevicePropertyCode::LiveViewProtocol => {
            "Protocol used for live view streaming (USB, network, etc.)."
        }
        DevicePropertyCode::DispMode | DevicePropertyCode::DispModeSetting => {
            "Display mode configuration for screen layout and information overlays."
        }
        DevicePropertyCode::DispModeCandidate => {
            "Available display modes for the current shooting mode."
        }
        DevicePropertyCode::SelectFinder => {
            "Switch between LCD monitor and electronic viewfinder."
        }
        DevicePropertyCode::DisplayQualityFinder => {
            "Display quality setting for the electronic viewfinder."
        }
        DevicePropertyCode::MonitorLUTSetting => {
            "LUT applied to monitor output for color preview."
        }
        DevicePropertyCode::DisplayedMenuStatus => {
            "Current menu display state."
        }
        DevicePropertyCode::GammaDisplayAssist => {
            "Applies a preview LUT to show how log footage will look after color grading. Does not affect recorded video."
        }
        DevicePropertyCode::GammaDisplayAssistType => {
            "Type of display assist LUT to apply for preview."
        }
        DevicePropertyCode::PlaybackContentsGammaType => {
            "Gamma curve type of the content being played back."
        }
        DevicePropertyCode::DeSqueezeDisplayRatio => {
            "Stretches anamorphic footage horizontally for proper preview. Set to match your anamorphic lens squeeze factor (1.33x, 1.8x, 2x)."
        }
        DevicePropertyCode::GridLineDisplayPlayback => {
            "Shows grid line overlays during image and video playback for composition review."
        }
        DevicePropertyCode::HDMIResolutionStillPlay => {
            "HDMI output resolution when playing back still images. Match to your monitor for best quality."
        }
        DevicePropertyCode::LiveViewImageQualityByNumericalValue => {
            "Specific quality value for live view streaming. Higher values provide clearer preview but use more bandwidth."
        }
        DevicePropertyCode::OSDImageMode => {
            "On-screen display image mode. Controls overlay information shown on external monitors via HDMI/SDI."
        }
        DevicePropertyCode::TCUBDisplaySetting => {
            "Timecode and user bit display configuration. Controls how TC/UB information appears on screen and outputs."
        }
        DevicePropertyCode::StreamDisplayName => {
            "Display name shown for the stream on receiving devices."
        }
        DevicePropertyCode::MonitoringTransportProtocol => {
            "Network protocol for monitoring delivery (NDI, SRT, etc.)."
        }
        DevicePropertyCode::MonitoringDeliveryTypeSupportInfo => {
            "Supported delivery types for monitoring output."
        }
        DevicePropertyCode::MonitorLUTSetting1
        | DevicePropertyCode::MonitorLUTSetting2
        | DevicePropertyCode::MonitorLUTSetting3 => {
            "LUT applied to this monitoring output for color preview."
        }
        DevicePropertyCode::MonitorLUTSettingOutputDestAssign => {
            "Assigns LUT settings to monitoring output destinations."
        }
        DevicePropertyCode::MonitoringAvailableFormat => {
            "Video formats available for monitoring output."
        }
        DevicePropertyCode::MonitoringOutputDisplaySettingDestAssign => {
            "Assigns display settings to monitoring output destinations."
        }
        DevicePropertyCode::MonitoringOutputDisplayHDMI => {
            "Display settings for HDMI monitoring output."
        }
        DevicePropertyCode::MonitoringOutputDisplaySDI => {
            "Display settings for SDI monitoring output."
        }
        DevicePropertyCode::MonitoringOutputDisplaySetting1
        | DevicePropertyCode::MonitoringOutputDisplaySetting2 => {
            "Monitoring output display configuration preset."
        }
        DevicePropertyCode::MonitoringOutputFormat => {
            "Output format for monitoring signal (resolution, frame rate)."
        }
        DevicePropertyCode::MonitoringIsDelivering => {
            "Whether monitoring output is actively being delivered."
        }
        DevicePropertyCode::MonitoringDeliveringStatus => {
            "Current status of monitoring delivery to external devices."
        }
        DevicePropertyCode::MonitoringFormatSupportInformation => {
            "Detailed format support information for monitoring."
        }
        DevicePropertyCode::MonitoringSettingVersion => {
            "Version of the monitoring settings format."
        }
        DevicePropertyCode::FaceEyeFrameDisplay => {
            "Shows or hides the frame overlay around detected faces and eyes."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::LiveViewDisplayEffect => "LV Effect",
        DevicePropertyCode::LiveViewStatus => "LV Status",
        DevicePropertyCode::DispMode => "Display",
        DevicePropertyCode::DispModeSetting => "Display Setting",
        DevicePropertyCode::MonitorLUTSetting => "Monitor LUT",
        DevicePropertyCode::MonitorLUTSetting1 => "Monitor LUT 1",
        DevicePropertyCode::MonitorLUTSetting2 => "Monitor LUT 2",
        DevicePropertyCode::MonitorLUTSetting3 => "Monitor LUT 3",
        DevicePropertyCode::MonitorLUTSettingOutputDestAssign => "LUT Output Assign",
        DevicePropertyCode::GridLineDisplay => "Grid Lines",
        DevicePropertyCode::MonitorBrightnessType => "Monitor Bright.",
        DevicePropertyCode::MonitorBrightnessManual => "Monitor Bright. (M)",
        DevicePropertyCode::DisplayedMenuStatus => "Menu Status",
        DevicePropertyCode::SelectFinder => "Finder Select",
        DevicePropertyCode::DispModeCandidate => "Disp Mode Options",
        DevicePropertyCode::GammaDisplayAssist => "Gamma Disp Assist",
        DevicePropertyCode::GammaDisplayAssistType => "Gamma Assist Type",
        DevicePropertyCode::DisplayQualityFinder => "Finder Quality",
        DevicePropertyCode::PlaybackContentsGammaType => "Playback Gamma",
        DevicePropertyCode::MonitoringAvailableFormat => "Available Formats",
        DevicePropertyCode::MonitoringDeliveringStatus => "Delivery Status",
        DevicePropertyCode::MonitoringDeliveryTypeSupportInfo => "Delivery Type Info",
        DevicePropertyCode::MonitoringFormatSupportInformation => "Format Support Info",
        DevicePropertyCode::MonitoringIsDelivering => "Is Delivering",
        DevicePropertyCode::MonitoringOutputDisplayHDMI => "HDMI Display",
        DevicePropertyCode::MonitoringOutputDisplaySDI => "SDI Display",
        DevicePropertyCode::MonitoringOutputDisplaySetting1 => "Display Setting 1",
        DevicePropertyCode::MonitoringOutputDisplaySetting2 => "Display Setting 2",
        DevicePropertyCode::MonitoringOutputDisplaySettingDestAssign => "Display Dest Assign",
        DevicePropertyCode::MonitoringOutputFormat => "Output Format",
        DevicePropertyCode::MonitoringSettingVersion => "Setting Version",
        DevicePropertyCode::MonitoringTransportProtocol => "Transport Protocol",
        DevicePropertyCode::StreamDisplayName => "Stream Name",
        DevicePropertyCode::DeSqueezeDisplayRatio => "Desqueeze Display Ratio",
        DevicePropertyCode::FaceEyeFrameDisplay => "Face/Eye Frame Display",
        DevicePropertyCode::GridLineDisplayPlayback => "Grid Line Playback",
        DevicePropertyCode::TCUBDisplaySetting => "TC/UB Disp",
        _ => code.name(),
    }
}
