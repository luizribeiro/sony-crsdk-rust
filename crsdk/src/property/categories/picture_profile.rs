//! Picture profile category: color grading and creative look properties.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

pub struct PictureProfile;

impl Category for PictureProfile {
    const NAME: &'static str = "Picture Profile";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::PictureEffect,
            "Effect Filter",
            "Creative in-camera filter effects.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfile,
            "PP Select",
            "Picture profile selection. Profiles define gamma curve, color mode, and other parameters for different looks.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileBlackLevel,
            "PP Black Level",
            "Black level adjustment. Negative values deepen blacks, positive values lift shadows.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileGamma,
            "PP Gamma",
            "Gamma curve selection. Different curves optimize for broadcast, cinema, or log capture.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileBlackGammaRange,
            "PP Black Gamma Range",
            "Range of shadow tones affected by black gamma adjustment.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileBlackGammaLevel,
            "PP Black Gamma Level",
            "Intensity of black gamma adjustment.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileKneeMode,
            "PP Knee Mode",
            "Highlight compression mode. Auto adjusts dynamically, Manual gives direct control.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileKneeAutoSetMaxPoint,
            "PP Knee Auto Max",
            "Maximum point for auto knee compression.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileKneeAutoSetSensitivity,
            "PP Knee Auto Sens",
            "Sensitivity of auto knee detection.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileKneeManualSetPoint,
            "PP Knee Point",
            "Manual knee point threshold.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileKneeManualSetSlope,
            "PP Knee Slope",
            "Compression rate above the manual knee point.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileColorMode,
            "PP Color Mode",
            "Color processing mode. Different modes optimize for different display targets.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileSaturation,
            "PP Saturation",
            "Overall color intensity adjustment.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileColorPhase,
            "PP Color Phase",
            "Overall color hue shift.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileColorDepthRed,
            "PP Red",
            "Red color saturation adjustment.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileColorDepthGreen,
            "PP Green",
            "Green color saturation adjustment.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileColorDepthBlue,
            "PP Blue",
            "Blue color saturation adjustment.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileColorDepthCyan,
            "PP Cyan",
            "Cyan color saturation adjustment.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileColorDepthMagenta,
            "PP Magenta",
            "Magenta color saturation adjustment.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileColorDepthYellow,
            "PP Yellow",
            "Yellow color saturation adjustment.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileDetailLevel,
            "PP Detail",
            "Sharpening intensity.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileDetailAdjustMode,
            "PP Detail Mode",
            "Detail adjustment mode.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileDetailAdjustVHBalance,
            "PP Detail V/H",
            "Vertical/horizontal detail balance.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileDetailAdjustBWBalance,
            "PP Detail B/W",
            "Black/white edge detail balance.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileDetailAdjustLimit,
            "PP Detail Limit",
            "Maximum detail enhancement limit.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileDetailAdjustCrispening,
            "PP Crisp",
            "Noise reduction before detail enhancement.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileDetailAdjustHiLightDetail,
            "PP HiLight Detail",
            "Detail in highlight areas.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileCopy,
            "PP Copy",
            "Copy picture profile to another slot.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CreativeLook,
            "CL Select",
            "Creative Look preset selection.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CreativeLookContrast,
            "CL Contrast",
            "Contrast adjustment for creative looks.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CreativeLookHighlights,
            "CL Highlights",
            "Highlight tone adjustment.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CreativeLookShadows,
            "CL Shadows",
            "Shadow tone adjustment.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CreativeLookFade,
            "CL Fade",
            "Fade/film look intensity.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CreativeLookSaturation,
            "CL Saturation",
            "Color saturation for creative looks.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CreativeLookSharpness,
            "CL Sharpness",
            "Sharpness for creative looks.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CreativeLookSharpnessRange,
            "CL Sharp Range",
            "Sharpness effect range.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CreativeLookClarity,
            "CL Clarity",
            "Clarity/local contrast adjustment.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CreativeLookCustomLook,
            "CL Custom",
            "Custom creative look slot.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::LogShootingModeColorGamut,
            "Log Gamut",
            "Color gamut for log shooting mode.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PictureProfileResetEnableStatus,
            "PP Reset Status",
            "Whether picture profile can be reset.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::CreativeLookResetEnableStatus,
            "CL Reset Status",
            "Whether creative look can be reset.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PlaybackContentsGammaType,
            "Playback Gamma",
            "Gamma type of playback content.",
            Some(V::Integer),
        ),
    ];
}
