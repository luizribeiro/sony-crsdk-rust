//! Focus-related property metadata (descriptions, display names, value types).

use super::PropertyValueType;
use crsdk_sys::DevicePropertyCode;

/// Returns a detailed description for a focus-related property code
pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::FocusMode => {
            "AF-S (Single) locks focus once acquired—good for still subjects. AF-C (Continuous) tracks moving subjects. MF (Manual) gives you direct control via the lens ring."
        }
        DevicePropertyCode::FocusArea => {
            "Where the camera looks for focus. Wide searches the whole frame. Zone uses a portion. Center uses the middle. Spot/Expand Spot use precise points for accuracy."
        }
        DevicePropertyCode::AFTrackingSensitivity => {
            "How quickly AF reacts to subject distance changes. High sensitivity tracks fast-moving subjects but may be distracted. Low sensitivity is more stable but slower to adapt."
        }
        DevicePropertyCode::FocusMagnifierSetting => {
            "Zooms in on the focus point for precise manual focus checking. Essential for critical focus in video and macro work."
        }
        DevicePropertyCode::AFAssist => {
            "Emits a beam or pattern to help autofocus in dark conditions. Useful indoors but may be distracting for subjects."
        }
        DevicePropertyCode::AFIlluminator => {
            "Enables an infrared or visible light beam to help autofocus in dark conditions. Useful indoors but may be blocked by obstacles."
        }
        DevicePropertyCode::AFSubjShiftSens => {
            "Adjusts how quickly AF tracking responds when the subject moves laterally. Higher sensitivity tracks erratic movement but may lose lock more easily."
        }
        DevicePropertyCode::AFTrackForSpeedChange => {
            "Controls how AF responds when a subject suddenly speeds up or slows down. Optimizes focus tracking for different movement patterns."
        }
        DevicePropertyCode::PreAF => {
            "Camera continuously adjusts focus even before half-pressing the shutter. Faster initial focus but uses more battery."
        }
        DevicePropertyCode::SubjectRecognitionInAF => {
            "Enables AI-powered detection of eyes, faces, animals, birds, vehicles, etc. The camera automatically prioritizes these subjects for focus."
        }
        DevicePropertyCode::SubjectRecognitionAF => {
            "Controls how subject recognition affects autofocus. Off disables recognition. Only AF detects but doesn't prioritize. Priority AF both detects and prioritizes recognized subjects."
        }
        DevicePropertyCode::AFTransitionSpeed => {
            "How smoothly focus changes between subjects in video. Slower transitions look more cinematic. Faster is better for action."
        }
        DevicePropertyCode::PrioritySetInAFS => {
            "What takes priority in AF-S mode. AF waits for focus lock before shooting. Release fires immediately. Balanced tries to achieve focus but won't delay too long."
        }
        DevicePropertyCode::PrioritySetInAFC => {
            "What takes priority in AF-C mode. AF waits for focus lock before shooting. Release fires immediately. Balanced tries to achieve focus but won't delay too long."
        }
        DevicePropertyCode::FocusTrackingStatus => {
            "Current state of focus tracking. Off means tracking is disabled. Focusing means actively searching. Tracking means actively following a subject."
        }
        DevicePropertyCode::FocusModeSetting => {
            "Choose between automatic or manual focus mode control. Automatic lets the camera manage focus mode. Manual gives you direct control."
        }
        DevicePropertyCode::AFWithShutter => {
            "When enabled, half-pressing the shutter activates autofocus. Disable to separate focus from shutter for back-button focus workflows."
        }
        DevicePropertyCode::FaceEyeFrameDisplay => {
            "Shows or hides the frame overlay around detected faces and eyes. Useful visual feedback for subject tracking."
        }
        DevicePropertyCode::FaceEyeDetectionAFStatus => {
            "Shows current status of face/eye detection. Indicates whether faces or eyes are currently detected and being tracked."
        }
        DevicePropertyCode::PushAFModeSetting => {
            "Configures behavior of Push AF button. Can be set to focus once, focus hold, or other AF trigger modes."
        }
        DevicePropertyCode::LensCompensationBreathing => {
            "Compensates for focus breathing where focal length shifts during focusing. Keeps framing consistent when pulling focus in video."
        }
        DevicePropertyCode::FocusBracketExposureLock1stImg => {
            "When enabled, locks exposure from the first shot in a focus bracket sequence. Prevents exposure variation across the stack."
        }
        DevicePropertyCode::FocusBracketIntervalUntilNextShot => {
            "Time delay between each shot in focus bracketing. Allows flash recycling or vibration settling."
        }
        DevicePropertyCode::FocusBracketOrder => {
            "Direction of focus movement through the bracket sequence. Start from near or far end of the focus range."
        }
        DevicePropertyCode::FocusBracketRecordingFolder => {
            "Folder where focus bracket sequences are saved. Helps organize stacking sequences for post-processing."
        }
        DevicePropertyCode::FocusBracketShootingStatus => {
            "Current state of focus bracket shooting. Shows if a bracket sequence is in progress."
        }
        DevicePropertyCode::FocusDrivingStatus => {
            "Current state of the focus motor. Shows if focus is moving, stopped, or actively adjusting."
        }
        DevicePropertyCode::FocusIndication => {
            "Focus lock indicator. Shows when autofocus has achieved or lost focus on the subject."
        }
        DevicePropertyCode::FocusMagnificationTime => {
            "How long focus magnification stays on screen before automatically returning to normal view."
        }
        DevicePropertyCode::FocusMap => {
            "Visual depth map showing focus distances across the frame. Helps visualize depth of field."
        }
        DevicePropertyCode::FocusSpeedDirectSync => {
            "Links focus speed directly to control input. Faster movements create faster focus changes."
        }
        DevicePropertyCode::FocusSpeedRange => {
            "Limits the focus motor speed range. Useful for smooth, controlled focus pulls in video."
        }
        DevicePropertyCode::FocusTouchSpotStatus => {
            "Shows if touch-to-focus is active at a specific screen position."
        }
        DevicePropertyCode::FocusOperationWithInt16 => {
            "Focus adjustment command using 16-bit precision. Allows finer control over focus position changes."
        }
        DevicePropertyCode::FocusOperationWithInt16EnableStatus => {
            "Indicates whether 16-bit precision focus control is available on this camera."
        }
        DevicePropertyCode::FollowFocusPositionSetting => {
            "Target focus position for follow focus systems. Used with external focus controllers."
        }
        DevicePropertyCode::FollowFocusPositionCurrentValue => {
            "Current focus position in follow focus mode. Shows where the lens is currently focused."
        }
        DevicePropertyCode::InitialFocusMagnifier => {
            "Initial magnification level when entering focus magnifier mode. Higher values show more detail."
        }
        DevicePropertyCode::AFInFocusMagnifier => {
            "Enables autofocus while focus magnifier is active. Allows AF at magnified view for precision."
        }
        DevicePropertyCode::ZoomAndFocusPositionLoad => {
            "Recalls a saved zoom and focus position preset. Useful for repeatable shot setups."
        }
        DevicePropertyCode::ZoomAndFocusPositionSave => {
            "Saves current zoom and focus position to a preset. For repeatable shot setups."
        }
        DevicePropertyCode::ZoomAndFocusPresetDataVersion => {
            "Version of the zoom/focus preset data format. For compatibility checking."
        }
        DevicePropertyCode::ZoomAndFocusPresetZoomOnlySet => {
            "Saves only zoom position (not focus) to a preset. Useful when focus should be manual."
        }
        DevicePropertyCode::AutoFocusHold => {
            "Temporarily pauses autofocus. Useful to prevent focus from changing during recomposition."
        }
        DevicePropertyCode::PushAutoFocus => {
            "Activates autofocus once. Similar to back-button focus—camera focuses then stops."
        }
        DevicePropertyCode::FocusModeStatus => {
            "Current effective focus mode. May differ from setting if camera overrides in certain modes."
        }
        DevicePropertyCode::FocusOperation => {
            "Manual focus adjustment direction. Controls whether to focus nearer or farther."
        }
        DevicePropertyCode::FocusBracketFocusRange => {
            "Focus distance range to cover during bracketing. Determines the depth of the focus stack."
        }
        DevicePropertyCode::FocusBracketShotNumber => {
            "Number of shots in the focus bracket sequence. More shots mean smaller focus steps."
        }
        DevicePropertyCode::AFFreeSizeAndPositionSetting => {
            "Custom AF area size and position. Allows precise positioning of the focus area anywhere on screen."
        }
        DevicePropertyCode::AFAreaPositionAFS => {
            "AF area position used in AF-S (single) mode. Can differ from AF-C position."
        }
        DevicePropertyCode::AFAreaPositionAFC => {
            "AF area position used in AF-C (continuous) mode. Can differ from AF-S position."
        }
        DevicePropertyCode::AFAreaPosition => {
            "Position of the AF area on screen. Controls where the camera focuses within the frame."
        }
        DevicePropertyCode::FocusPositionSetting => {
            "Target focus position for manual or follow focus control. Sets where the lens should focus."
        }
        DevicePropertyCode::FocusPositionCurrentValue => {
            "Current focus position of the lens. Shows the actual focus distance or position value."
        }
        DevicePropertyCode::NearFar => {
            "Manual focus adjustment amount and direction. Controls how much to move focus nearer or farther."
        }
        DevicePropertyCode::DefaultAFFreeSizeAndPositionSetting => {
            "Default size and position for custom AF areas. Sets the initial AF area when creating new flexible spots."
        }
        DevicePropertyCode::AmountOfDefocusSetting => {
            "Amount of defocus/blur to apply. Controls background blur intensity for creative effects."
        }
        _ => "",
    }
}

/// Returns a short display name for a focus-related property code
pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::FocusMode => "AF Mode",
        DevicePropertyCode::FocusModeStatus => "AF Mode Status",
        DevicePropertyCode::FocusModeSetting => "AF Mode Setting",
        DevicePropertyCode::FocusArea => "AF Area",
        DevicePropertyCode::AFTrackingSensitivity => "AF Track Sens.",
        DevicePropertyCode::AFAreaPosition => "AF Area Pos",
        DevicePropertyCode::AFAreaPositionAFS => "AF Area (AF-S)",
        DevicePropertyCode::AFAreaPositionAFC => "AF Area (AF-C)",
        DevicePropertyCode::FocusMagnifierSetting => "Focus Magnifier",
        DevicePropertyCode::NearFar => "Near/Far",
        DevicePropertyCode::AFAssist => "AF Assist",
        DevicePropertyCode::AFIlluminator => "AF Illum.",
        DevicePropertyCode::AFWithShutter => "AF w/ Shutter",
        DevicePropertyCode::PreAF => "Pre-AF",
        DevicePropertyCode::FocusOperation => "Focus Op.",
        DevicePropertyCode::FocusOperationWithInt16 => "Focus Op. (16-bit)",
        DevicePropertyCode::FocusOperationWithInt16EnableStatus => "Focus 16-bit Avail",
        DevicePropertyCode::FocusDrivingStatus => "Focus Drive Status",
        DevicePropertyCode::FocusIndication => "Focus Lock",
        DevicePropertyCode::FocusPositionSetting => "Focus Pos",
        DevicePropertyCode::FocusPositionCurrentValue => "Focus Pos (Curr)",
        DevicePropertyCode::AFTransitionSpeed => "AF Trans. Speed",
        DevicePropertyCode::AFSubjShiftSens => "AF Subj. Shift",
        DevicePropertyCode::AFTrackForSpeedChange => "AF Track Spd",
        DevicePropertyCode::PrioritySetInAFS => "Priority (AF-S)",
        DevicePropertyCode::PrioritySetInAFC => "Priority (AF-C)",
        DevicePropertyCode::PushAutoFocus => "Push AF",
        DevicePropertyCode::AutoFocusHold => "AF Hold",
        DevicePropertyCode::FocusBracketShotNumber => "Focus Brkt Shots",
        DevicePropertyCode::FocusBracketFocusRange => "Focus Brkt Range",
        DevicePropertyCode::FocusBracketExposureLock1stImg => "Focus Brkt AE Lock",
        DevicePropertyCode::FocusBracketIntervalUntilNextShot => "Focus Brkt Interval",
        DevicePropertyCode::FocusBracketOrder => "Focus Brkt Order",
        DevicePropertyCode::FocusBracketRecordingFolder => "Focus Brkt Folder",
        DevicePropertyCode::FocusBracketShootingStatus => "Focus Brkt Status",
        DevicePropertyCode::SubjectRecognitionInAF => "Subject Recog.",
        DevicePropertyCode::SubjectRecognitionAF => "Subject Recog. AF",
        DevicePropertyCode::FaceEyeDetectionAFStatus => "Face/Eye Status",
        DevicePropertyCode::FaceEyeFrameDisplay => "Face/Eye Frame",
        DevicePropertyCode::FocusTrackingStatus => "Focus Track Status",
        DevicePropertyCode::FocusMagnificationTime => "Focus Mag. Time",
        DevicePropertyCode::FocusMap => "Depth Map",
        DevicePropertyCode::FocusSpeedDirectSync => "Focus Spd Sync",
        DevicePropertyCode::FocusSpeedRange => "Focus Spd Range",
        DevicePropertyCode::FocusTouchSpotStatus => "Touch Focus Status",
        DevicePropertyCode::FollowFocusPositionSetting => "Follow Focus Pos",
        DevicePropertyCode::FollowFocusPositionCurrentValue => "Follow Focus Curr",
        DevicePropertyCode::InitialFocusMagnifier => "Init. Focus Mag.",
        DevicePropertyCode::AFInFocusMagnifier => "AF in Magnifier",
        DevicePropertyCode::AFFreeSizeAndPositionSetting => "AF Free Size/Pos",
        DevicePropertyCode::ZoomAndFocusPositionLoad => "Zoom/Focus Load",
        DevicePropertyCode::ZoomAndFocusPositionSave => "Zoom/Focus Save",
        DevicePropertyCode::ZoomAndFocusPresetDataVersion => "Zoom/Focus Ver.",
        DevicePropertyCode::ZoomAndFocusPresetZoomOnlySet => "Zoom Only Preset",
        DevicePropertyCode::DefaultAFFreeSizeAndPositionSetting => "Default AF Size/Pos",
        DevicePropertyCode::AmountOfDefocusSetting => "Defocus Amount",
        _ => code.name(),
    }
}

/// Returns the value type for a focus-related property code
pub fn value_type(code: DevicePropertyCode) -> Option<PropertyValueType> {
    use PropertyValueType as V;

    Some(match code {
        DevicePropertyCode::FocusMode | DevicePropertyCode::FocusModeSetting => V::FocusMode,
        DevicePropertyCode::FocusArea => V::FocusArea,
        DevicePropertyCode::SubjectRecognitionAF | DevicePropertyCode::SubjectRecognitionInAF => {
            V::SubjectRecognitionAF
        }
        DevicePropertyCode::PrioritySetInAFS | DevicePropertyCode::PrioritySetInAFC => {
            V::PrioritySetInAF
        }
        DevicePropertyCode::FocusTrackingStatus => V::FocusTrackingStatus,
        DevicePropertyCode::AFAssist
        | DevicePropertyCode::PreAF
        | DevicePropertyCode::AFWithShutter
        | DevicePropertyCode::FaceEyeFrameDisplay
        | DevicePropertyCode::AFInFocusMagnifier
        | DevicePropertyCode::FocusBracketExposureLock1stImg => V::Switch,
        DevicePropertyCode::FocusModeStatus => V::FocusMode,
        DevicePropertyCode::FocusBracketIntervalUntilNextShot
        | DevicePropertyCode::FocusBracketOrder
        | DevicePropertyCode::FocusBracketShootingStatus
        | DevicePropertyCode::FocusDrivingStatus
        | DevicePropertyCode::FocusIndication
        | DevicePropertyCode::FocusMagnificationTime
        | DevicePropertyCode::FocusSpeedRange
        | DevicePropertyCode::FocusTouchSpotStatus
        | DevicePropertyCode::FocusPositionSetting
        | DevicePropertyCode::FocusPositionCurrentValue
        | DevicePropertyCode::FollowFocusPositionSetting
        | DevicePropertyCode::FollowFocusPositionCurrentValue
        | DevicePropertyCode::InitialFocusMagnifier
        | DevicePropertyCode::FocusBracketShotNumber
        | DevicePropertyCode::FocusBracketFocusRange
        | DevicePropertyCode::AFTransitionSpeed
        | DevicePropertyCode::AFTrackingSensitivity => V::Integer,
        DevicePropertyCode::FocusOperation
        | DevicePropertyCode::FocusOperationWithInt16
        | DevicePropertyCode::NearFar => V::Integer,
        DevicePropertyCode::FocusSpeedDirectSync
        | DevicePropertyCode::FocusOperationWithInt16EnableStatus
        | DevicePropertyCode::AutoFocusHold
        | DevicePropertyCode::PushAutoFocus => V::Integer,
        DevicePropertyCode::ZoomAndFocusPositionLoad
        | DevicePropertyCode::ZoomAndFocusPositionSave
        | DevicePropertyCode::ZoomAndFocusPresetDataVersion
        | DevicePropertyCode::ZoomAndFocusPresetZoomOnlySet
        | DevicePropertyCode::FocusMap
        | DevicePropertyCode::AFAreaPosition
        | DevicePropertyCode::AFAreaPositionAFS
        | DevicePropertyCode::AFAreaPositionAFC
        | DevicePropertyCode::AFFreeSizeAndPositionSetting
        | DevicePropertyCode::DefaultAFFreeSizeAndPositionSetting
        | DevicePropertyCode::FocusBracketRecordingFolder
        | DevicePropertyCode::FocusMagnifierSetting
        | DevicePropertyCode::FaceEyeDetectionAFStatus
        | DevicePropertyCode::AFSubjShiftSens
        | DevicePropertyCode::AFTrackForSpeedChange => V::Integer,
        DevicePropertyCode::AFIlluminator => V::OnOff,
        _ => return None,
    })
}
