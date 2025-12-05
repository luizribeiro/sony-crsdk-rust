//! Descriptions for camera properties.
//!
//! This module provides helpful descriptions explaining what each property does.

use crsdk_sys::DevicePropertyCode;

/// Get a description of what a property does.
///
/// Returns a human-readable explanation of the property's purpose and effect.
pub fn property_description(code: DevicePropertyCode) -> &'static str {
    match code {
        // === Exposure ===
        DevicePropertyCode::FNumber => {
            "Controls the lens aperture opening. Lower values (f/1.4) create shallow depth of field with blurry backgrounds. Higher values (f/16) keep more of the scene in focus but require more light."
        }
        DevicePropertyCode::ShutterSpeed => {
            "How long the sensor is exposed to light. Fast speeds (1/1000s) freeze motion but need more light. Slow speeds (1/30s) allow more light but can cause motion blur."
        }
        DevicePropertyCode::IsoSensitivity => {
            "Sensor light sensitivity. Lower values (100) produce cleaner images in bright light. Higher values (6400+) work in low light but add noise/grain."
        }
        DevicePropertyCode::ExposureBiasCompensation => {
            "Adjusts overall exposure brightness. Positive values (+1, +2) brighten the image. Negative values (-1, -2) darken it. Useful when the camera's metering is fooled by very bright or dark scenes."
        }
        DevicePropertyCode::ExposureProgramMode => {
            "Determines how aperture and shutter speed are set. Manual (M) gives full control. Aperture Priority (A) lets you set aperture while camera picks shutter. Shutter Priority (S) is the opposite. Program (P) automates both."
        }
        DevicePropertyCode::ExposureIndex => {
            "Exposure Index (EI) for cinema cameras. Similar to ISO but specifically for log/cinema workflows where the actual sensor sensitivity remains fixed."
        }
        DevicePropertyCode::MeteringMode => {
            "How the camera measures light. Multi uses the whole frame. Center-weighted prioritizes the middle. Spot measures only a small area, useful for backlit subjects."
        }
        DevicePropertyCode::AutoSlowShutter => {
            "In auto modes, allows the camera to use slower shutter speeds in low light. Helps maintain lower ISO but may introduce motion blur."
        }
        DevicePropertyCode::DRO => {
            "Dynamic Range Optimizer. Automatically adjusts shadows and highlights to preserve detail in high-contrast scenes. Works on JPEGs only."
        }
        DevicePropertyCode::ExposureCtrlType => {
            "P/A/S/M uses traditional exposure modes. Flexible Exposure allows independent control of aperture, shutter, and ISO regardless of the selected mode."
        }
        DevicePropertyCode::ShutterSlow => {
            "Enables extended slow shutter speeds for long exposures. Useful for light trails, smooth water, or low-light photography with a tripod."
        }
        DevicePropertyCode::ShutterSlowFrames => {
            "Number of frames to accumulate when using slow shutter. Higher values create longer effective exposures for creative effects."
        }

        // === Focus ===
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

        // === White Balance ===
        DevicePropertyCode::WhiteBalance => {
            "Adjusts color temperature so whites appear neutral. Auto works in most cases. Presets (Daylight, Tungsten, etc.) match specific light sources. Custom/Kelvin allows precise control."
        }
        DevicePropertyCode::Colortemp => {
            "Specific color temperature in Kelvin. Lower values (3200K) are warmer/orange for tungsten light. Higher values (5600K+) are cooler/blue for daylight."
        }
        DevicePropertyCode::ColorTuningAB => {
            "Fine-tunes white balance on the amber-blue axis. Positive shifts toward amber (warm), negative toward blue (cool)."
        }
        DevicePropertyCode::ColorTuningGM => {
            "Fine-tunes white balance on the green-magenta axis. Adjusts for fluorescent lighting or creative color effects."
        }
        DevicePropertyCode::AWBL => {
            "Locks the current auto white balance setting. Useful when you want consistent color across multiple shots in changing light."
        }
        DevicePropertyCode::PrioritySetInAWB => {
            "Controls AWB color bias. Standard is neutral. Ambience preserves warm/cool feeling of the light source. White makes whites appear more neutral."
        }

        // === Image Quality ===
        DevicePropertyCode::FileType => {
            "Choose between RAW (unprocessed sensor data for maximum editing flexibility), JPEG (processed and compressed), or RAW+JPEG (both files saved)."
        }
        DevicePropertyCode::StillImageQuality => {
            "JPEG compression level. Extra Fine has minimal compression (best quality, larger files). Standard has more compression (smaller files, some quality loss)."
        }
        DevicePropertyCode::ImageSize => {
            "Resolution of captured images. Larger sizes have more detail for printing/cropping. Smaller sizes save storage space."
        }
        DevicePropertyCode::AspectRatio => {
            "Image shape. 3:2 is standard for full-frame. 16:9 is widescreen. 1:1 is square. 4:3 matches micro four-thirds sensors."
        }
        DevicePropertyCode::RAWFileCompressionType => {
            "RAW file compression. Uncompressed preserves all data. Lossless compressed reduces size without quality loss. Compressed is smallest but may lose some data."
        }
        DevicePropertyCode::HighIsoNR => {
            "Reduces noise in high ISO images. Higher settings smooth noise but may soften fine detail. Applies to JPEGs."
        }
        DevicePropertyCode::LongExposureNR => {
            "Reduces hot pixels and noise in long exposures by taking a 'dark frame' subtraction. Doubles the capture time."
        }

        // === Movie ===
        DevicePropertyCode::MovieFileFormat => {
            "Container format for video. XAVC S/HS/I offer high quality with various compression options. MP4 is widely compatible."
        }
        DevicePropertyCode::MovieRecordingSetting => {
            "Video resolution and bitrate combination. Higher settings (4K, high bitrate) have more detail but larger files."
        }
        DevicePropertyCode::MovieRecordingFrameRateSetting => {
            "Frames per second. 24fps is cinematic. 30fps is standard video. 60fps is smooth for action. 120fps+ enables slow motion."
        }
        DevicePropertyCode::RecordingState => {
            "Current recording status. Shows whether the camera is actively recording video."
        }
        DevicePropertyCode::TimeCodeFormat => {
            "Timecode standard. Drop Frame (DF) compensates for 29.97fps timing. Non-Drop Frame (NDF) counts frames directly."
        }
        DevicePropertyCode::ProxyRecordingSetting => {
            "Records a smaller, lower-quality copy alongside the main video. Useful for faster editing workflows."
        }
        DevicePropertyCode::SQRecordingSetting => {
            "Slow & Quick motion settings. Records at different frame rates for slow motion or time-lapse effects in-camera."
        }
        DevicePropertyCode::LogShootingMode => {
            "Enables log gamma curves (S-Log2, S-Log3) for maximum dynamic range. Requires color grading in post-production."
        }

        // === Drive Mode ===
        DevicePropertyCode::DriveMode => {
            "Single shot takes one photo per press. Continuous shoots multiple frames while held. Self-timer delays the shot. Bracket takes multiple exposures."
        }
        DevicePropertyCode::BracketOrder => {
            "Sequence of bracketed exposures. 0/−/+ starts with normal exposure. −/0/+ starts with underexposed."
        }
        DevicePropertyCode::RecordingSelfTimer => {
            "Delay before the shutter fires. Useful for self-portraits or to avoid camera shake from pressing the button."
        }
        DevicePropertyCode::IntervalRecMode => {
            "Captures images at set intervals for time-lapse photography. The camera can optionally compile them into a video."
        }
        DevicePropertyCode::IntervalRecShutterType => {
            "Shutter type for interval shooting. Auto selects automatically. Mechanical uses the physical shutter. Electronic is silent but may cause rolling shutter with moving subjects."
        }

        // === Media ===
        DevicePropertyCode::MediaSLOT1Status => {
            "Status of memory card in slot 1. Shows if a card is inserted, its capacity, and any errors."
        }
        DevicePropertyCode::MediaSLOT2Status => {
            "Status of memory card in slot 2. Shows if a card is inserted, its capacity, and any errors."
        }
        DevicePropertyCode::AutoSwitchMedia => {
            "Automatically switches to the other card slot when the current card fills up. Prevents missed shots."
        }
        DevicePropertyCode::SimulRecSetting => {
            "Records to both card slots simultaneously. Provides instant backup of every shot."
        }

        // === Power & Battery ===
        DevicePropertyCode::BatteryRemain => {
            "Remaining battery capacity as a percentage. Monitor this to avoid running out during a shoot."
        }
        DevicePropertyCode::BatteryLevel => {
            "Battery charge level indicator. Shows approximate remaining power."
        }
        DevicePropertyCode::PowerSource => {
            "Current power source—internal battery, external battery grip, or AC adapter."
        }
        DevicePropertyCode::AutoPowerOffTemperature => {
            "Temperature threshold for automatic shutdown. Higher settings allow longer recording but risk overheating damage."
        }
        DevicePropertyCode::DeviceOverheatingState => {
            "Current thermal status. Warning levels indicate the camera may shut down soon to prevent damage."
        }

        // === Zoom ===
        DevicePropertyCode::ZoomScale => {
            "Current zoom magnification level. Shows how much the lens is zoomed in from its widest setting."
        }
        DevicePropertyCode::ZoomSetting => {
            "Zoom behavior settings. Controls whether digital zoom is enabled and how zoom speed responds to input."
        }
        DevicePropertyCode::DigitalZoomScale => {
            "Digital zoom magnification. Extends beyond optical zoom but reduces image quality as it crops and enlarges."
        }

        // === Lens ===
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

        // === Flash ===
        DevicePropertyCode::FlashMode => {
            "Flash behavior. Auto fires when needed. Fill fires always. Slow Sync uses slow shutter with flash for ambient light. Rear Sync fires at end of exposure."
        }
        DevicePropertyCode::FlashCompensation => {
            "Adjusts flash power relative to camera's calculation. Positive values increase flash output, negative decrease it."
        }
        DevicePropertyCode::RedEyeReduction => {
            "Fires pre-flashes to constrict subjects' pupils, reducing the red-eye effect in portraits."
        }

        // === Audio ===
        DevicePropertyCode::AudioRecording => {
            "Enables or disables audio recording with video. Turn off when using external audio recorders."
        }
        DevicePropertyCode::AudioSignals => {
            "Camera beeps for focus confirmation and self-timer. Disable for quiet shooting environments."
        }
        DevicePropertyCode::WindNoiseReduct => {
            "Reduces low-frequency wind noise in the built-in microphone. May slightly affect audio quality."
        }

        // === Picture Profile ===
        DevicePropertyCode::PictureProfile => {
            "Preset color, gamma, and detail settings for video. PP1-PP10 are customizable. Off uses standard processing."
        }
        DevicePropertyCode::CreativeLook => {
            "Camera-designed color looks for stills and video. Each style (ST, PT, NT, etc.) has a distinct aesthetic."
        }
        DevicePropertyCode::GammaDisplayAssist => {
            "Shows a preview of how log footage will look after color grading. Helps expose correctly without flat-looking preview."
        }

        // === ND Filter ===
        DevicePropertyCode::NDFilter => {
            "Built-in neutral density filter. Reduces light entering the lens, allowing wider apertures or slower shutters in bright conditions."
        }
        DevicePropertyCode::NDFilterMode => {
            "ND filter behavior. Auto engages as needed. Manual gives direct control. Variable ND allows smooth adjustment."
        }

        // === Display ===
        DevicePropertyCode::LiveViewDisplayEffect => {
            "Shows exposure, white balance, and creative look effects in the viewfinder/LCD. Turn off for consistent preview brightness."
        }
        DevicePropertyCode::GridLineDisplay => {
            "Overlays composition guides on the screen. Rule of thirds, square grid, or diagonal lines help frame shots."
        }
        DevicePropertyCode::MonitorBrightnessType => {
            "Screen brightness setting. Manual sets a fixed level. Auto adjusts based on ambient light. Sunny Weather boosts for outdoors."
        }

        // === Stabilization ===
        DevicePropertyCode::ImageStabilizationSteadyShot => {
            "In-body image stabilization (IBIS). Compensates for camera shake, allowing slower shutter speeds when handheld."
        }
        DevicePropertyCode::MovieImageStabilizationSteadyShot => {
            "Stabilization mode for video. Active mode is more aggressive but slightly crops the image."
        }

        // === Silent Mode ===
        DevicePropertyCode::SilentMode => {
            "Disables all mechanical sounds and lights. Uses electronic shutter and turns off AF illuminator and flash. Essential for weddings, wildlife, and theaters."
        }
        DevicePropertyCode::SilentModeApertureDriveInAF => {
            "Controls aperture motor noise during AF in silent mode. Not Target ignores this setting. Standard balances speed and noise. Silent Priority minimizes noise but may slow AF."
        }

        // === Streaming ===
        DevicePropertyCode::StreamModeSetting => {
            "Configures the camera for live streaming output. Sets up video format and connection type for streaming platforms."
        }

        // === Network ===
        DevicePropertyCode::FTPFunction => {
            "Enables FTP file transfer. Automatically uploads images to a configured FTP server over WiFi."
        }
        DevicePropertyCode::WakeOnLAN => {
            "Allows the camera to be powered on remotely via network signal. Useful for remote camera setups."
        }

        // === System ===
        DevicePropertyCode::ModelName => {
            "Camera model identifier. Useful for confirming the connected device."
        }
        DevicePropertyCode::BodySerialNumber => {
            "Unique camera serial number. Important for registration, insurance, and tracking ownership."
        }
        DevicePropertyCode::SoftwareVersion => {
            "Current firmware version. Check for updates to get new features and bug fixes."
        }

        // === Gain (Cinema) ===
        DevicePropertyCode::GainControlSetting => {
            "Gain control method for cinema cameras. Choose between ISO-based or dB-based gain adjustment."
        }
        DevicePropertyCode::GainBaseIsoSensitivity => {
            "Native/base ISO setting. Dual native ISO cameras have two optimal sensitivity levels with minimal noise."
        }
        DevicePropertyCode::GaindBValue => {
            "Gain level in decibels. Common in cinema workflows. 0dB is the base sensitivity, positive values amplify the signal."
        }

        // === Shutter (Cinema) ===
        DevicePropertyCode::ShutterAngle => {
            "Shutter timing expressed as an angle (45°-360°). 180° is cinematic standard, giving natural motion blur at 24fps."
        }
        DevicePropertyCode::ShutterModeSetting => {
            "Auto lets the camera control shutter timing. Manual gives you direct control over shutter speed or angle."
        }
        DevicePropertyCode::ShutterModeStatus => {
            "Shows the current shutter mode: Off, Speed (time-based), Angle (degree-based), ECS (Extended Clear Scan), or Auto."
        }
        DevicePropertyCode::ShutterMode => {
            "Choose how shutter is measured: Speed uses time fractions (1/100s), Angle uses degrees (180°) for consistent motion blur across frame rates."
        }
        DevicePropertyCode::ShutterSetting => {
            "Enables or disables manual shutter control. When Off, the camera handles shutter automatically."
        }
        DevicePropertyCode::ShutterECSSetting => {
            "Extended Clear Scan reduces banding when filming monitors and LED screens by syncing shutter to the display's refresh rate."
        }

        // === Iris (Cinema) ===
        DevicePropertyCode::IrisModeSetting => {
            "Aperture control mode. Auto lets the camera adjust. Manual gives full control for consistent exposure."
        }

        // Default fallback
        _ => "No description available for this property.",
    }
}
