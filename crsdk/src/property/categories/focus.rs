//! Focus category: autofocus and manual focus properties.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

/// Autofocus and manual focus properties.
pub struct Focus;

impl Category for Focus {
    const NAME: &'static str = "Focus";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::FocusMode,
            "AF Mode",
            "AF-S (Single) locks focus once acquired—good for still subjects. AF-C (Continuous) tracks moving subjects. MF (Manual) gives you direct control via the lens ring.",
            Some(V::FocusMode),
        ),
        PropertyDef::new(
            C::FocusArea,
            "AF Area",
            "Where the camera looks for focus. Wide searches the whole frame. Zone uses a portion. Center uses the middle. Spot/Expand Spot use precise points for accuracy.",
            Some(V::FocusArea),
        ),
        PropertyDef::new(
            C::AFTrackingSensitivity,
            "AF Track Sens.",
            "How quickly AF reacts to subject distance changes. High sensitivity tracks fast-moving subjects but may be distracted. Low sensitivity is more stable but slower to adapt.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusMagnifierSetting,
            "Focus Magnifier",
            "Zooms in on the focus point for precise manual focus checking. Essential for critical focus in video and macro work.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::NearFar,
            "Near/Far",
            "Manual focus adjustment amount and direction. Controls how much to move focus nearer or farther.",
            Some(V::NearFarEnableStatus),
        ),
        PropertyDef::new(
            C::AFAreaPosition,
            "AF Area Pos",
            "Position of the AF area on screen. Controls where the camera focuses within the frame.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomAndFocusPositionSave,
            "Zoom/Focus Save",
            "Saves current zoom and focus position to a preset. For repeatable shot setups.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomAndFocusPositionLoad,
            "Zoom/Focus Load",
            "Recalls a saved zoom and focus position preset. Useful for repeatable shot setups.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusOperation,
            "Focus Op.",
            "Manual focus direction: Near, Stop, or Far.",
            Some(V::FocusOperation),
        ),
        PropertyDef::new(
            C::FocusModeSetting,
            "AF Mode Setting",
            "Choose between automatic or manual focus mode control. Automatic lets the camera manage focus mode. Manual gives you direct control.",
            Some(V::FocusMode),
        ),
        PropertyDef::new(
            C::AFTransitionSpeed,
            "AF Trans. Speed",
            "How smoothly focus changes between subjects in video. Slower transitions look more cinematic. Faster is better for action.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AFSubjShiftSens,
            "AF Subj. Shift",
            "Adjusts how quickly AF tracking responds when the subject moves laterally. Higher sensitivity tracks erratic movement but may lose lock more easily.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AFAssist,
            "AF Assist",
            "Emits a beam or pattern to help autofocus in dark conditions. Useful indoors but may be distracting for subjects.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::FollowFocusPositionSetting,
            "Follow Focus Pos",
            "Target focus position for follow focus systems. Used with external focus controllers.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusBracketShotNumber,
            "Focus Brkt Shots",
            "Number of shots in the focus bracket sequence. More shots mean smaller focus steps.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusBracketFocusRange,
            "Focus Brkt Range",
            "Focus distance range to cover during bracketing. Determines the depth of the focus stack.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusBracketOrder,
            "Focus Brkt Order",
            "Direction of focus movement through the bracket sequence. Start from near or far end of the focus range.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusBracketExposureLock1stImg,
            "Focus Brkt AE Lock",
            "When enabled, locks exposure from the first shot in a focus bracket sequence. Prevents exposure variation across the stack.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::FocusBracketIntervalUntilNextShot,
            "Focus Brkt Interval",
            "Time delay between each shot in focus bracketing. Allows flash recycling or vibration settling.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusPositionSetting,
            "Focus Pos",
            "Target focus position for manual or follow focus control. Sets where the lens should focus.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::PrioritySetInAFS,
            "Priority (AF-S)",
            "What takes priority in AF-S mode. AF waits for focus lock before shooting. Release fires immediately. Balanced tries to achieve focus but won't delay too long.",
            Some(V::PrioritySetInAF),
        ),
        PropertyDef::new(
            C::PrioritySetInAFC,
            "Priority (AF-C)",
            "What takes priority in AF-C mode. AF waits for focus lock before shooting. Release fires immediately. Balanced tries to achieve focus but won't delay too long.",
            Some(V::PrioritySetInAF),
        ),
        PropertyDef::new(
            C::FocusMagnificationTime,
            "Focus Mag. Time",
            "How long focus magnification stays on screen before automatically returning to normal view.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AFIlluminator,
            "AF Illum.",
            "Enables an infrared or visible light beam to help autofocus in dark conditions. Useful indoors but may be blocked by obstacles.",
            Some(V::OnOff),
        ),
        PropertyDef::new(
            C::AFWithShutter,
            "AF w/ Shutter",
            "When enabled, half-pressing the shutter activates autofocus. Disable to separate focus from shutter for back-button focus workflows.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::FocusBracketRecordingFolder,
            "Focus Brkt Folder",
            "Folder where focus bracket sequences are saved. Helps organize stacking sequences for post-processing.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusMap,
            "Depth Map",
            "Visual depth map showing focus distances across the frame. Helps visualize depth of field.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::InitialFocusMagnifier,
            "Init. Focus Mag.",
            "Initial magnification level when entering focus magnifier mode. Higher values show more detail.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AFInFocusMagnifier,
            "AF in Magnifier",
            "Enables autofocus while focus magnifier is active. Allows AF at magnified view for precision.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::AFTrackForSpeedChange,
            "AF Track Spd",
            "Controls how AF responds when a subject suddenly speeds up or slows down. Optimizes focus tracking for different movement patterns.",
            Some(V::AFTrackForSpeedChange),
        ),
        PropertyDef::new(
            C::AFFreeSizeAndPositionSetting,
            "AF Free Size/Pos",
            "Custom AF area size and position. Allows precise positioning of the focus area anywhere on screen.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomAndFocusPresetZoomOnlySet,
            "Zoom Only Preset",
            "Saves only zoom position (not focus) to a preset. Useful when focus should be manual.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AFAreaPositionAFC,
            "AF Area (AF-C)",
            "AF area position used in AF-C (continuous) mode. Can differ from AF-S position.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AFAreaPositionAFS,
            "AF Area (AF-S)",
            "AF area position used in AF-S (single) mode. Can differ from AF-C position.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AutoFocusHold,
            "AF Hold",
            "Temporarily pauses autofocus. Useful to prevent focus from changing during recomposition.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::PushAutoFocus,
            "Push AF",
            "Activates autofocus once. Similar to back-button focus—camera focuses then stops.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusSpeedDirectSync,
            "Focus Spd Sync",
            "Links focus speed directly to control input. Faster movements create faster focus changes.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusOperationWithInt16,
            "Focus Op. (16-bit)",
            "Focus adjustment command using 16-bit precision. Allows finer control over focus position changes.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::DefaultAFFreeSizeAndPositionSetting,
            "Default AF Size/Pos",
            "Default size and position for custom AF areas. Sets the initial AF area when creating new flexible spots.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusIndication,
            "Focus Lock",
            "Focus lock indicator. Shows when autofocus has achieved or lost focus on the subject.",
            Some(V::FocusIndicator),
        ),
        PropertyDef::new(
            C::FocusSpeedRange,
            "Focus Spd Range",
            "Limits the focus motor speed range. Useful for smooth, controlled focus pulls in video.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusTouchSpotStatus,
            "Touch Focus Status",
            "Shows if touch-to-focus is active at a specific screen position.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusTrackingStatus,
            "Focus Track Status",
            "Current state of focus tracking. Off means tracking is disabled. Focusing means actively searching. Tracking means actively following a subject.",
            Some(V::FocusTrackingStatus),
        ),
        PropertyDef::new(
            C::FollowFocusPositionCurrentValue,
            "Follow Focus Curr",
            "Current focus position in follow focus mode. Shows where the lens is currently focused.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusBracketShootingStatus,
            "Focus Brkt Status",
            "Current state of focus bracket shooting. Shows if a bracket sequence is in progress.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusPositionCurrentValue,
            "Focus Pos (Curr)",
            "Current focus position of the lens. Shows the actual focus distance or position value.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusDrivingStatus,
            "Focus Drive Status",
            "Current state of the focus motor. Shows if focus is moving, stopped, or actively adjusting.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ZoomAndFocusPresetDataVersion,
            "Zoom/Focus Ver.",
            "Version of the zoom/focus preset data format. For compatibility checking.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::FocusModeStatus,
            "AF Mode Status",
            "Current effective focus mode. May differ from setting if camera overrides in certain modes.",
            Some(V::FocusMode),
        ),
        PropertyDef::new(
            C::FocusOperationWithInt16EnableStatus,
            "Focus 16-bit Avail",
            "Indicates whether 16-bit precision focus control is available on this camera.",
            Some(V::Integer),
        ),
    ];
}

crate::register_category!(Focus);
