//! Exposure-related property metadata (descriptions, display names, value types).

use super::super::PropertyValueType;
use crsdk_sys::DevicePropertyCode;

/// Returns a detailed description for an exposure-related property code
pub fn description(code: DevicePropertyCode) -> &'static str {
    match code {
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
        DevicePropertyCode::GainControlSetting => {
            "Gain control method for cinema cameras. Choose between ISO-based or dB-based gain adjustment."
        }
        DevicePropertyCode::GainBaseIsoSensitivity => {
            "Native/base ISO setting. Dual native ISO cameras have two optimal sensitivity levels with minimal noise."
        }
        DevicePropertyCode::GaindBValue => {
            "Gain level in decibels. Common in cinema workflows. 0dB is the base sensitivity, positive values amplify the signal."
        }
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
        DevicePropertyCode::IrisModeSetting => {
            "Aperture control mode. Auto lets the camera adjust. Manual gives full control for consistent exposure."
        }
        DevicePropertyCode::IrisDisplayUnit => {
            "How aperture values are displayed. F-stop (f/2.8) is standard. T-stop accounts for light transmission loss in the lens."
        }
        DevicePropertyCode::IrisCloseSetting => {
            "Allows closing the iris completely. Used for sensor protection or specific exposure effects."
        }
        DevicePropertyCode::ExposureStep => {
            "Granularity of exposure adjustments. 1/3 EV gives finer control, 1/2 EV gives larger steps."
        }
        DevicePropertyCode::BulbExposureTimeSetting => {
            "Sets the exposure time for bulb mode. Allows precise long exposures without holding the shutter button."
        }
        DevicePropertyCode::ExtendedShutterSpeed => {
            "Enables shutter speeds beyond the standard range. Allows very long exposures for night photography or creative effects."
        }
        DevicePropertyCode::HighIsoNR => {
            "Noise reduction applied at high ISO values. Reduces grain but may soften fine details."
        }
        DevicePropertyCode::LongExposureNR => {
            "Noise reduction for long exposures. Takes a dark frame to subtract hot pixels. Doubles exposure time."
        }
        DevicePropertyCode::IsoAutoMinShutterSpeedMode => {
            "How minimum shutter speed is determined in Auto ISO. Faster keeps shutter quick, Slower prioritizes low ISO."
        }
        DevicePropertyCode::IsoAutoMinShutterSpeedManual => {
            "Manual minimum shutter speed when using Auto ISO. Camera won't go slower than this value."
        }
        DevicePropertyCode::IsoAutoMinShutterSpeedPreset => {
            "Preset minimum shutter speed based on focal length. Helps prevent motion blur from camera shake."
        }
        DevicePropertyCode::IsoAutoRangeLimitMin => {
            "Minimum ISO when using Auto ISO. Keeps images clean in good light by preventing unnecessary sensitivity boost."
        }
        DevicePropertyCode::IsoAutoRangeLimitMax => {
            "Maximum ISO when using Auto ISO. Limits noise by capping how high sensitivity can go."
        }
        DevicePropertyCode::IsoCurrentSensitivity => {
            "Current effective ISO value. May differ from set ISO due to Auto ISO or exposure compensation."
        }
        DevicePropertyCode::ShutterSpeedValue => {
            "Numeric shutter speed value. Upper bits are numerator, lower bits are denominator of the fraction."
        }
        DevicePropertyCode::ShutterSpeedCurrentValue => {
            "Current effective shutter speed. May differ from set value in auto modes or with exposure compensation."
        }
        DevicePropertyCode::ShutterType => {
            "Mechanical vs electronic shutter. Electronic is silent and faster but may cause rolling shutter artifacts."
        }
        DevicePropertyCode::ShutterSelectMode => {
            "Chooses between shutter types. Auto selects based on conditions, Manual lets you force a specific type."
        }
        DevicePropertyCode::ShutterReleaseTimeLagControl => {
            "Controls shutter release delay. Standard mode optimizes image quality, Speed mode minimizes delay."
        }
        DevicePropertyCode::BaseISOSwitchEI => {
            "Switches between base ISO sensitivities on dual-ISO sensors. Each base has optimal dynamic range."
        }
        DevicePropertyCode::GainUnitSetting => {
            "Display gain as ISO values or decibels (dB). dB is common in video workflows."
        }
        DevicePropertyCode::GaindBCurrentValue => {
            "Current gain level in decibels. 0dB is base sensitivity."
        }
        DevicePropertyCode::FacePriorityInMultiMetering => {
            "Prioritizes detected faces when metering exposure. Ensures faces are properly exposed even in challenging lighting."
        }
        DevicePropertyCode::MeteredManualLevel => {
            "Exposure meter reading in manual mode. Shows how current settings compare to metered exposure."
        }
        DevicePropertyCode::IntervalRecShutterType => {
            "Shutter type for interval shooting. Auto selects automatically. Mechanical uses the physical shutter. Electronic is silent but may cause rolling shutter."
        }
        DevicePropertyCode::ShutterECSNumber => {
            "Extended Clear Scan number setting. Fine-tunes the ECS frequency for eliminating banding on specific displays."
        }
        DevicePropertyCode::ShutterECSNumberStep => {
            "Step size for ECS number adjustments. Smaller steps allow more precise tuning."
        }
        DevicePropertyCode::ShutterECSFrequency => {
            "Extended Clear Scan frequency. Match this to your display's refresh rate to eliminate banding."
        }
        DevicePropertyCode::GainBaseSensitivity => {
            "Base sensor sensitivity for cinema cameras. Determines the native ISO/gain starting point."
        }
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterHi => {
            "Frames per second for Hi continuous mode with electronic shutter."
        }
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterHiPlus => {
            "Frames per second for Hi+ continuous mode with electronic shutter. Fastest burst speed."
        }
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterMid => {
            "Frames per second for Mid continuous mode with electronic shutter."
        }
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterLo => {
            "Frames per second for Lo continuous mode with electronic shutter."
        }
        DevicePropertyCode::HighResolutionShutterSpeed => {
            "Fine-tuned shutter speed beyond standard increments. Used to eliminate banding from flickering artificial light sources like LED and fluorescent."
        }
        DevicePropertyCode::HighResolutionShutterSpeedAdjust => {
            "Fine-grained adjustment for high-resolution shutter speeds. Allows decimal values between standard stops to precisely match light source flicker frequency."
        }
        DevicePropertyCode::HighResolutionShutterSpeedAdjustInIntegralMultiples => {
            "Constrains high-resolution shutter speed adjustments to integral multiples. Provides structured fine-tuning in proportional steps."
        }
        DevicePropertyCode::HighResolutionShutterSpeedSetting => {
            "Master control for variable shutter speed functionality. When enabled, allows fine adjustments to counteract light source flickering."
        }
        DevicePropertyCode::ElectricFrontCurtainShutter => {
            "Uses electronic sensor control instead of mechanical movement for the front shutter curtain. Reduces shutter shock and noise."
        }
        DevicePropertyCode::ApertureDriveInAF => {
            "Controls aperture behavior during autofocus. Standard mode adjusts for focus speed, Focus Priority prioritizes tracking, Silent Priority minimizes noise."
        }
        DevicePropertyCode::SilentModeApertureDriveInAF => {
            "Aperture drive behavior when Silent Mode is active. Balances silent operation with focusing performance."
        }
        DevicePropertyCode::AmountOfDefocusSetting => {
            "Controls intensity of background defocus (bokeh). Adjusts blur applied to out-of-focus areas for artistic background separation."
        }
        DevicePropertyCode::ColorSpace => {
            "Selects color gamut for image encoding. sRGB for standard use and web, Adobe RGB for print with wider color reproduction."
        }
        DevicePropertyCode::EmbedLUTFile => {
            "Embeds a LUT file into video recordings for color grading reference. Ensures consistent color when imported into editing software."
        }
        DevicePropertyCode::EstimatePictureSize => {
            "Displays estimated file size before capture based on current settings. Helps monitor remaining card space."
        }
        DevicePropertyCode::SilentModeShutterWhenPowerOff => {
            "Controls shutter blade behavior when powering off in silent mode. Close keeps sensor protected from dust."
        }
        DevicePropertyCode::AntidustShutterWhenPowerOff => {
            "Closes shutter when powering off to protect sensor from dust. Keeps sensor clean during lens changes."
        }
        DevicePropertyCode::PushAutoIris => {
            "Temporarily engages auto iris while button is pressed. Useful for quick exposure checks."
        }
        _ => "",
    }
}

/// Returns a short display name for an exposure-related property code
pub fn display_name(code: DevicePropertyCode) -> &'static str {
    match code {
        DevicePropertyCode::FNumber => "Aperture (f-number)",
        DevicePropertyCode::ShutterSpeed => "Shutter",
        DevicePropertyCode::IsoSensitivity => "ISO",
        DevicePropertyCode::ExposureBiasCompensation => "Exposure Comp",
        DevicePropertyCode::ExposureProgramMode => "Exposure Mode",
        DevicePropertyCode::ExposureIndex => "Exposure Index (EI)",
        DevicePropertyCode::ExposureCtrlType => "Exposure Control Type",
        DevicePropertyCode::ExposureStep => "EV Step Size",
        DevicePropertyCode::AutoSlowShutter => "Slow Shutter Auto",
        DevicePropertyCode::ShutterModeSetting => "Shutter Control",
        DevicePropertyCode::ShutterModeStatus => "Shutter Mode",
        DevicePropertyCode::ShutterMode => "Shutter Unit",
        DevicePropertyCode::ShutterAngle => "Shutter Angle (°)",
        DevicePropertyCode::ShutterSetting => "Shutter",
        DevicePropertyCode::ShutterSlow => "Slow Shutter",
        DevicePropertyCode::ShutterSlowFrames => "Slow Shutter Frames",
        DevicePropertyCode::ShutterECSSetting => "ECS Mode",
        DevicePropertyCode::ShutterECSFrequency => "ECS Frequency",
        DevicePropertyCode::ShutterECSNumber => "ECS Number",
        DevicePropertyCode::ShutterECSNumberStep => "ECS Step",
        DevicePropertyCode::GainControlSetting => "Gain Control",
        DevicePropertyCode::GainBaseIsoSensitivity => "Base ISO",
        DevicePropertyCode::GainBaseSensitivity => "Base Sensitivity",
        DevicePropertyCode::GainUnitSetting => "Gain Unit",
        DevicePropertyCode::GaindBCurrentValue => "Gain (dB)",
        DevicePropertyCode::GaindBValue => "Gain Value (dB)",
        DevicePropertyCode::IrisModeSetting => "Iris Mode",
        DevicePropertyCode::IrisDisplayUnit => "Iris Unit (F/T)",
        DevicePropertyCode::IrisCloseSetting => "Iris Close Enable",
        DevicePropertyCode::BaseISOSwitchEI => "Base ISO (EI)",
        DevicePropertyCode::DRO => "D-Range Optimizer",
        DevicePropertyCode::MeteringMode => "Meter Mode",
        DevicePropertyCode::MeteredManualLevel => "Meter Level (M)",
        DevicePropertyCode::FacePriorityInMultiMetering => "Face Priority Metering",
        DevicePropertyCode::BulbExposureTimeSetting => "Bulb Exposure Time",
        DevicePropertyCode::ExtendedShutterSpeed => "Extended SS",
        DevicePropertyCode::HighIsoNR => "High ISO NR",
        DevicePropertyCode::LongExposureNR => "Long Exp NR",
        DevicePropertyCode::IsoAutoMinShutterSpeedMode => "ISO Auto Min SS Mode",
        DevicePropertyCode::IsoAutoMinShutterSpeedManual => "ISO Auto Min SS (Manual)",
        DevicePropertyCode::IsoAutoMinShutterSpeedPreset => "ISO Auto Min SS (Preset)",
        DevicePropertyCode::IsoAutoRangeLimitMin => "ISO Auto Range Min",
        DevicePropertyCode::IsoAutoRangeLimitMax => "ISO Auto Range Max",
        DevicePropertyCode::IsoCurrentSensitivity => "ISO (Current)",
        DevicePropertyCode::ShutterSpeedValue => "SS Value",
        DevicePropertyCode::ShutterSpeedCurrentValue => "SS (Current)",
        DevicePropertyCode::ShutterType => "Shutter (Mech/Elec)",
        DevicePropertyCode::ShutterSelectMode => "Shutter Selection",
        DevicePropertyCode::ShutterReleaseTimeLagControl => "Shutter Release Lag",
        DevicePropertyCode::IntervalRecShutterType => "Interval Shutter",
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterHi => "Cont. Hi FPS (E)",
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterHiPlus => "Cont. Hi+ FPS (E)",
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterMid => "Cont. Mid FPS (E)",
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterLo => "Cont. Lo FPS (E)",
        DevicePropertyCode::ApertureDriveInAF => "Aperture Drive in AF",
        DevicePropertyCode::SilentModeApertureDriveInAF => "Silent Aperture Drive",
        DevicePropertyCode::ElectricFrontCurtainShutter => "E-Front Curtain",
        DevicePropertyCode::HighResolutionShutterSpeed => "High Res Shutter Speed",
        DevicePropertyCode::HighResolutionShutterSpeedAdjust => "High Res Shutter Adjust",
        DevicePropertyCode::HighResolutionShutterSpeedAdjustInIntegralMultiples => {
            "High Res Shutter Multiples"
        }
        DevicePropertyCode::HighResolutionShutterSpeedSetting => "High Res Shutter Setting",
        DevicePropertyCode::PushAutoIris => "Push Iris",
        _ => code.name(),
    }
}

/// Returns the value type for an exposure-related property code
pub fn value_type(code: DevicePropertyCode) -> Option<PropertyValueType> {
    use PropertyValueType as V;

    Some(match code {
        DevicePropertyCode::FNumber => V::Aperture,
        DevicePropertyCode::ShutterSpeed | DevicePropertyCode::ShutterSpeedCurrentValue => {
            V::ShutterSpeed
        }
        DevicePropertyCode::IsoSensitivity => V::Iso,
        DevicePropertyCode::ExposureBiasCompensation => V::ExposureCompensation,
        DevicePropertyCode::ExposureProgramMode => V::ExposureProgram,
        DevicePropertyCode::MeteringMode => V::MeteringMode,
        DevicePropertyCode::ShutterModeStatus => V::ShutterModeStatus,
        DevicePropertyCode::ShutterMode => V::ShutterMode,
        DevicePropertyCode::ExposureCtrlType => V::ExposureCtrlType,
        DevicePropertyCode::AutoSlowShutter
        | DevicePropertyCode::ShutterSetting
        | DevicePropertyCode::ShutterECSSetting
        | DevicePropertyCode::ShutterSlow => V::Switch,
        DevicePropertyCode::IrisModeSetting
        | DevicePropertyCode::ShutterModeSetting
        | DevicePropertyCode::GainControlSetting => V::AutoManual,
        DevicePropertyCode::ShutterSpeedValue => V::ShutterSpeed,
        DevicePropertyCode::IsoCurrentSensitivity
        | DevicePropertyCode::IsoAutoRangeLimitMin
        | DevicePropertyCode::IsoAutoRangeLimitMax
        | DevicePropertyCode::ExposureIndex
        | DevicePropertyCode::GainBaseIsoSensitivity
        | DevicePropertyCode::BaseISOSwitchEI => V::Iso,
        DevicePropertyCode::ShutterAngle => V::Integer,
        DevicePropertyCode::ShutterSlowFrames
        | DevicePropertyCode::ShutterECSFrequency
        | DevicePropertyCode::ShutterECSNumber
        | DevicePropertyCode::ShutterECSNumberStep
        | DevicePropertyCode::BulbExposureTimeSetting
        | DevicePropertyCode::MeteredManualLevel
        | DevicePropertyCode::GaindBValue
        | DevicePropertyCode::ExposureStep => V::Integer,
        DevicePropertyCode::HighIsoNR
        | DevicePropertyCode::LongExposureNR
        | DevicePropertyCode::ExtendedShutterSpeed
        | DevicePropertyCode::FacePriorityInMultiMetering
        | DevicePropertyCode::DRO => V::Switch,
        DevicePropertyCode::IsoAutoMinShutterSpeedMode
        | DevicePropertyCode::ShutterSelectMode
        | DevicePropertyCode::ShutterType
        | DevicePropertyCode::ShutterReleaseTimeLagControl
        | DevicePropertyCode::IrisDisplayUnit
        | DevicePropertyCode::GainUnitSetting
        | DevicePropertyCode::GainBaseSensitivity
        | DevicePropertyCode::IrisCloseSetting => V::Integer,
        DevicePropertyCode::IsoAutoMinShutterSpeedManual
        | DevicePropertyCode::IsoAutoMinShutterSpeedPreset => V::ShutterSpeed,
        DevicePropertyCode::IntervalRecShutterType => V::IntervalRecShutterType,
        DevicePropertyCode::GaindBCurrentValue => V::Integer,
        DevicePropertyCode::ContinuousShootingSpeedInElectricShutterHi
        | DevicePropertyCode::ContinuousShootingSpeedInElectricShutterHiPlus
        | DevicePropertyCode::ContinuousShootingSpeedInElectricShutterMid
        | DevicePropertyCode::ContinuousShootingSpeedInElectricShutterLo => V::Integer,
        _ => return None,
    })
}
