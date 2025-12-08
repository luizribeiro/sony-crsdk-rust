//! Picture profile property metadata (descriptions, display names).

use crsdk_sys::DevicePropertyCode;

pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::PictureProfile => {
            "Preset color, gamma, and detail settings for video. PP1-PP10 are customizable. Off uses standard processing."
        }
        DevicePropertyCode::PictureProfileGamma
        | DevicePropertyCode::PictureProfileBlackGammaLevel
        | DevicePropertyCode::PictureProfileBlackGammaRange => {
            "Gamma curve settings. Controls how brightness values are mapped for different contrast and dynamic range."
        }
        DevicePropertyCode::PictureProfileColorMode => {
            "Color processing mode. Different modes optimize for various shooting scenarios."
        }
        DevicePropertyCode::PictureProfileSaturation
        | DevicePropertyCode::PictureProfileColorPhase => {
            "Color saturation and hue adjustments for the picture profile."
        }
        DevicePropertyCode::PictureProfileBlackLevel => {
            "Adjusts the level of black in the image. Affects shadow detail and contrast."
        }
        DevicePropertyCode::PictureProfileKneeMode
        | DevicePropertyCode::PictureProfileKneeAutoSetMaxPoint
        | DevicePropertyCode::PictureProfileKneeAutoSetSensitivity
        | DevicePropertyCode::PictureProfileKneeManualSetPoint
        | DevicePropertyCode::PictureProfileKneeManualSetSlope => {
            "Knee settings control highlight compression. Prevents clipping in bright areas."
        }
        DevicePropertyCode::PictureProfileDetailLevel
        | DevicePropertyCode::PictureProfileDetailAdjustMode
        | DevicePropertyCode::PictureProfileDetailAdjustVHBalance
        | DevicePropertyCode::PictureProfileDetailAdjustBWBalance
        | DevicePropertyCode::PictureProfileDetailAdjustLimit
        | DevicePropertyCode::PictureProfileDetailAdjustCrispening
        | DevicePropertyCode::PictureProfileDetailAdjustHiLightDetail => {
            "Detail/sharpening settings. Controls edge enhancement and texture rendering."
        }
        DevicePropertyCode::PictureProfileColorDepthRed
        | DevicePropertyCode::PictureProfileColorDepthGreen
        | DevicePropertyCode::PictureProfileColorDepthBlue
        | DevicePropertyCode::PictureProfileColorDepthCyan
        | DevicePropertyCode::PictureProfileColorDepthMagenta
        | DevicePropertyCode::PictureProfileColorDepthYellow => {
            "Per-color saturation adjustments. Fine-tune individual color channels."
        }
        DevicePropertyCode::PictureProfileCopy => {
            "Copies picture profile settings to another profile slot."
        }
        DevicePropertyCode::PictureProfileResetEnableStatus => {
            "Indicates whether the picture profile can be reset to defaults."
        }
        DevicePropertyCode::CreativeLook => {
            "Camera-designed color looks for stills and video. Each style (ST, PT, NT, etc.) has a distinct aesthetic."
        }
        DevicePropertyCode::CreativeLookContrast
        | DevicePropertyCode::CreativeLookHighlights
        | DevicePropertyCode::CreativeLookShadows
        | DevicePropertyCode::CreativeLookSaturation
        | DevicePropertyCode::CreativeLookSharpness
        | DevicePropertyCode::CreativeLookSharpnessRange
        | DevicePropertyCode::CreativeLookClarity
        | DevicePropertyCode::CreativeLookFade => {
            "Fine-tuning parameter for the creative look. Adjusts the overall style characteristics."
        }
        DevicePropertyCode::CreativeLookCustomLook => {
            "Custom creative look that can be loaded from a file."
        }
        DevicePropertyCode::CreativeLookResetEnableStatus => {
            "Indicates whether the creative look can be reset to defaults."
        }
        DevicePropertyCode::GammaDisplayAssist => {
            "Shows a preview of how log footage will look after color grading. Helps expose correctly without flat-looking preview."
        }
        DevicePropertyCode::GammaDisplayAssistType => {
            "Type of gamma assist preview. Different LUT presets for various color workflows."
        }
        DevicePropertyCode::BaseLookValue | DevicePropertyCode::BaseLookAppliedofPlayback => {
            "Base look setting that defines the fundamental color characteristics before other adjustments."
        }
        DevicePropertyCode::BaseLookNameofPlayback => {
            "Name of the base look applied during playback."
        }
        DevicePropertyCode::BaseLookImportOperationEnableStatus => {
            "Indicates whether base look import is available."
        }
        DevicePropertyCode::PictureEffect => {
            "Creative filters applied to images in-camera. Includes toy camera, posterization, etc."
        }
        DevicePropertyCode::LogShootingModeColorGamut => {
            "Color gamut for log shooting. S-Gamut3 provides maximum color range for grading."
        }
        DevicePropertyCode::ColorSpace => {
            "Color space for images. sRGB for web/print. AdobeRGB for wider gamut professional workflows."
        }
        DevicePropertyCode::EstimatePictureSize => {
            "Estimated file size for the current image quality settings."
        }
        _ => "",
    }
}

pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::PictureProfile => "Pict. Profile",
        DevicePropertyCode::PictureProfileGamma => "PP Gamma",
        DevicePropertyCode::PictureProfileColorMode => "PP Color Mode",
        DevicePropertyCode::PictureProfileSaturation => "PP Saturation",
        DevicePropertyCode::PictureProfileBlackLevel => "PP Black Level",
        DevicePropertyCode::PictureProfileBlackGammaLevel => "PP Black Gamma",
        DevicePropertyCode::PictureProfileBlackGammaRange => "PP BG Range",
        DevicePropertyCode::PictureProfileKneeMode => "PP Knee Mode",
        DevicePropertyCode::PictureProfileKneeAutoSetMaxPoint => "PP Knee Auto Max",
        DevicePropertyCode::PictureProfileKneeAutoSetSensitivity => "PP Knee Auto Sens",
        DevicePropertyCode::PictureProfileKneeManualSetPoint => "PP Knee Point",
        DevicePropertyCode::PictureProfileKneeManualSetSlope => "PP Knee Slope",
        DevicePropertyCode::PictureProfileDetailLevel => "PP Detail Level",
        DevicePropertyCode::PictureProfileDetailAdjustMode => "PP Detail Mode",
        DevicePropertyCode::PictureProfileDetailAdjustVHBalance => "PP Detail V/H",
        DevicePropertyCode::PictureProfileDetailAdjustBWBalance => "PP Detail B/W",
        DevicePropertyCode::PictureProfileDetailAdjustLimit => "PP Detail Limit",
        DevicePropertyCode::PictureProfileDetailAdjustCrispening => "PP Crisp",
        DevicePropertyCode::PictureProfileDetailAdjustHiLightDetail => "PP HiLight Detail",
        DevicePropertyCode::PictureProfileColorPhase => "PP Color Phase",
        DevicePropertyCode::PictureProfileColorDepthRed => "PP Depth: Red",
        DevicePropertyCode::PictureProfileColorDepthGreen => "PP Depth: Green",
        DevicePropertyCode::PictureProfileColorDepthBlue => "PP Depth: Blue",
        DevicePropertyCode::PictureProfileColorDepthCyan => "PP Depth: Cyan",
        DevicePropertyCode::PictureProfileColorDepthMagenta => "PP Depth: Magenta",
        DevicePropertyCode::PictureProfileColorDepthYellow => "PP Depth: Yellow",
        DevicePropertyCode::PictureProfileCopy => "PP Copy",
        DevicePropertyCode::PictureProfileResetEnableStatus => "PP Reset Avail",
        DevicePropertyCode::CreativeLook => "Creat. Look",
        DevicePropertyCode::CreativeLookContrast => "CL: Contrast",
        DevicePropertyCode::CreativeLookHighlights => "CL: Highlights",
        DevicePropertyCode::CreativeLookShadows => "CL: Shadows",
        DevicePropertyCode::CreativeLookSaturation => "CL: Saturation",
        DevicePropertyCode::CreativeLookSharpness => "CL: Sharpness",
        DevicePropertyCode::CreativeLookSharpnessRange => "CL: Sharp Range",
        DevicePropertyCode::CreativeLookClarity => "CL: Clarity",
        DevicePropertyCode::CreativeLookFade => "CL: Fade",
        DevicePropertyCode::CreativeLookCustomLook => "CL: Custom",
        DevicePropertyCode::CreativeLookResetEnableStatus => "CL Reset Avail",
        DevicePropertyCode::GammaDisplayAssist => "Gamma Assist",
        DevicePropertyCode::GammaDisplayAssistType => "Gamma Assist Type",
        DevicePropertyCode::BaseLookValue => "Base Look",
        DevicePropertyCode::BaseLookAppliedofPlayback => "Base Look (Play)",
        DevicePropertyCode::BaseLookNameofPlayback => "Base Look Name",
        DevicePropertyCode::BaseLookImportOperationEnableStatus => "Base Look Import",
        DevicePropertyCode::PictureEffect => "Pict. Effect",
        DevicePropertyCode::ColorSpace => "Clr Space",
        DevicePropertyCode::EstimatePictureSize => "Est. Picture Size",
        DevicePropertyCode::LogShootingModeColorGamut => "Log Gamut",
        _ => code.name(),
    }
}
