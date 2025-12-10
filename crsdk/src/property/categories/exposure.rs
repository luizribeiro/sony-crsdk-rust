//! Exposure category: aperture, shutter, ISO, and metering properties.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

/// Exposure, ISO, shutter, and aperture properties.
pub struct Exposure;

impl Category for Exposure {
    const NAME: &'static str = "Exposure";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::FNumber,
            "Aperture (f-number)",
            "Controls the lens aperture opening. Lower values (f/1.4) create shallow depth of field with blurry backgrounds. Higher values (f/16) keep more of the scene in focus but require more light.",
            Some(V::Aperture),
        ),
        PropertyDef::new(
            C::ExposureBiasCompensation,
            "Exposure Comp",
            "Adjusts overall exposure brightness. Positive values (+1, +2) brighten the image. Negative values (-1, -2) darken it. Useful when the camera's metering is fooled by very bright or dark scenes.",
            Some(V::ExposureCompensation),
        ),
        PropertyDef::new(
            C::ShutterSpeed,
            "Shutter",
            "How long the sensor is exposed to light. Fast speeds (1/1000s) freeze motion but need more light. Slow speeds (1/30s) allow more light but can cause motion blur.",
            Some(V::ShutterSpeed),
        ),
        PropertyDef::new(
            C::IsoSensitivity,
            "ISO",
            "Sensor light sensitivity. Lower values (100) produce cleaner images in bright light. Higher values (6400+) work in low light but add noise/grain.",
            Some(V::Iso),
        ),
        PropertyDef::new(
            C::ExposureProgramMode,
            "Exposure Mode",
            "Determines how aperture and shutter speed are set. Manual (M) gives full control. Aperture Priority (A) lets you set aperture while camera picks shutter. Shutter Priority (S) is the opposite. Program (P) automates both.",
            Some(V::ExposureProgram),
        ),
        PropertyDef::new(
            C::IrisModeSetting,
            "Iris Mode",
            "Aperture control mode. Auto lets the camera adjust. Manual gives full control for consistent exposure.",
            Some(V::AutoManual),
        ),
        PropertyDef::new(
            C::ShutterModeSetting,
            "Shutter Control",
            "Auto lets the camera control shutter timing. Manual gives you direct control over shutter speed or angle.",
            Some(V::AutoManual),
        ),
        PropertyDef::new(
            C::GainControlSetting,
            "Gain Control",
            "Gain control method for cinema cameras. Choose between ISO-based or dB-based gain adjustment.",
            Some(V::AutoManual),
        ),
        PropertyDef::new(
            C::GainBaseIsoSensitivity,
            "Base ISO",
            "Native/base ISO setting. Dual native ISO cameras have two optimal sensitivity levels with minimal noise.",
            Some(V::Iso),
        ),
        PropertyDef::new(
            C::GainBaseSensitivity,
            "Base Sensitivity",
            "Base sensor sensitivity for cinema cameras. Determines the native ISO/gain starting point.",
            Some(V::GainBaseSensitivity),
        ),
        PropertyDef::new(
            C::ExposureIndex,
            "Exposure Index (EI)",
            "Exposure Index (EI) for cinema cameras. Similar to ISO but specifically for log/cinema workflows where the actual sensor sensitivity remains fixed.",
            Some(V::Iso),
        ),
        PropertyDef::new(
            C::ExposureCtrlType,
            "Exposure Control Type",
            "P/A/S/M uses traditional exposure modes. Flexible Exposure allows independent control of aperture, shutter, and ISO regardless of the selected mode.",
            Some(V::ExposureCtrlType),
        ),
        PropertyDef::new(
            C::ShutterModeStatus,
            "Shutter Mode",
            "Shows the current shutter mode: Off, Speed (time-based), Angle (degree-based), ECS (Extended Clear Scan), or Auto.",
            Some(V::ShutterModeStatus),
        ),
        PropertyDef::new(
            C::ShutterSlow,
            "Slow Shutter",
            "Enables extended slow shutter speeds for long exposures. Useful for light trails, smooth water, or low-light photography with a tripod.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::ShutterSlowFrames,
            "Slow Shutter Frames",
            "Number of frames to accumulate when using slow shutter. Higher values create longer effective exposures for creative effects.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ShutterECSSetting,
            "ECS Mode",
            "Extended Clear Scan reduces banding when filming monitors and LED screens by syncing shutter to the display's refresh rate.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::ShutterECSNumber,
            "ECS Number",
            "Extended Clear Scan number setting. Fine-tunes the ECS frequency for eliminating banding on specific displays.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ShutterECSNumberStep,
            "ECS Step",
            "Step size for ECS number adjustments. Smaller steps allow more precise tuning.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ShutterECSFrequency,
            "ECS Frequency",
            "Extended Clear Scan frequency. Match this to your display's refresh rate to eliminate banding.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ShutterAngle,
            "Shutter Angle (°)",
            "Shutter timing expressed as an angle (45°-360°). 180° is cinematic standard, giving natural motion blur at 24fps.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ShutterSetting,
            "Shutter",
            "Enables or disables manual shutter control. When Off, the camera handles shutter automatically.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::ShutterMode,
            "Shutter Unit",
            "Choose how shutter is measured: Speed uses time fractions (1/100s), Angle uses degrees (180°) for consistent motion blur across frame rates.",
            Some(V::ShutterMode),
        ),
        PropertyDef::new(
            C::ShutterSpeedValue,
            "SS Value",
            "Numeric shutter speed value. Upper bits are numerator, lower bits are denominator of the fraction.",
            Some(V::ShutterSpeed),
        ),
        PropertyDef::new(
            C::GainUnitSetting,
            "Gain Unit",
            "Display gain as ISO values or decibels (dB). dB is common in video workflows.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::GaindBValue,
            "Gain Value (dB)",
            "Gain level in decibels. Common in cinema workflows. 0dB is the base sensitivity, positive values amplify the signal.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::SilentModeApertureDriveInAF,
            "Silent Aperture Drive",
            "Aperture drive behavior when Silent Mode is active. Balances silent operation with focusing performance.",
            Some(V::SilentModeApertureDrive),
        ),
        PropertyDef::new(
            C::ShutterType,
            "Shutter (Mech/Elec)",
            "Mechanical vs electronic shutter. Electronic is silent and faster but may cause rolling shutter artifacts.",
            Some(V::ShutterType),
        ),
        PropertyDef::new(
            C::HighResolutionShutterSpeedSetting,
            "High Res Shutter Setting",
            "Master control for variable shutter speed functionality. When enabled, allows fine adjustments to counteract light source flickering.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::BaseISOSwitchEI,
            "Base ISO (EI)",
            "Switches between base ISO sensitivities on dual-ISO sensors. Each base has optimal dynamic range.",
            Some(V::Iso),
        ),
        PropertyDef::new(
            C::LongExposureNR,
            "Long Exp NR",
            "Noise reduction for long exposures. Takes a dark frame to subtract hot pixels. Doubles exposure time.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::HighIsoNR,
            "High ISO NR",
            "Noise reduction applied at high ISO values. Reduces grain but may soften fine details.",
            Some(V::HighIsoNR),
        ),
        PropertyDef::new(
            C::IntervalRecShutterType,
            "Interval Shutter",
            "Shutter type for interval shooting. Auto selects automatically. Mechanical uses the physical shutter. Electronic is silent but may cause rolling shutter.",
            Some(V::IntervalRecShutterType),
        ),
        PropertyDef::new(
            C::ElectricFrontCurtainShutter,
            "E-Front Curtain",
            "Uses electronic sensor control instead of mechanical movement for the front shutter curtain. Reduces shutter shock and noise.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::BulbExposureTimeSetting,
            "Bulb Exposure Time",
            "Sets the exposure time for bulb mode. Allows precise long exposures without holding the shutter button.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AutoSlowShutter,
            "Slow Shutter Auto",
            "In auto modes, allows the camera to use slower shutter speeds in low light. Helps maintain lower ISO but may introduce motion blur.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::IsoAutoMinShutterSpeedMode,
            "ISO Auto Min SS Mode",
            "How minimum shutter speed is determined in Auto ISO. Faster keeps shutter quick, Slower prioritizes low ISO.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::IsoAutoMinShutterSpeedManual,
            "ISO Auto Min SS (Manual)",
            "Manual minimum shutter speed when using Auto ISO. Camera won't go slower than this value.",
            Some(V::ShutterSpeed),
        ),
        PropertyDef::new(
            C::IsoAutoMinShutterSpeedPreset,
            "ISO Auto Min SS (Preset)",
            "Preset minimum shutter speed based on focal length. Helps prevent motion blur from camera shake.",
            Some(V::ShutterSpeed),
        ),
        PropertyDef::new(
            C::IrisDisplayUnit,
            "Iris Unit (F/T)",
            "How aperture values are displayed. F-stop (f/2.8) is standard. T-stop accounts for light transmission loss in the lens.",
            Some(V::IrisDisplayUnit),
        ),
        PropertyDef::new(
            C::ExtendedShutterSpeed,
            "Extended SS",
            "Enables shutter speeds beyond the standard range. Allows very long exposures for night photography or creative effects.",
            Some(V::Switch),
        ),
        PropertyDef::new(
            C::ShutterReleaseTimeLagControl,
            "Shutter Release Lag",
            "Controls shutter release delay. Standard mode optimizes image quality, Speed mode minimizes delay.",
            Some(V::ShutterReleaseTimeLagControl),
        ),
        PropertyDef::new(
            C::ContinuousShootingSpeedInElectricShutterHiPlus,
            "Cont. Hi+ FPS (E)",
            "Frames per second for Hi+ continuous mode with electronic shutter. Fastest burst speed.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ContinuousShootingSpeedInElectricShutterHi,
            "Cont. Hi FPS (E)",
            "Frames per second for Hi continuous mode with electronic shutter.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ContinuousShootingSpeedInElectricShutterMid,
            "Cont. Mid FPS (E)",
            "Frames per second for Mid continuous mode with electronic shutter.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ContinuousShootingSpeedInElectricShutterLo,
            "Cont. Lo FPS (E)",
            "Frames per second for Lo continuous mode with electronic shutter.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::IsoAutoRangeLimitMin,
            "ISO Auto Range Min",
            "Minimum ISO when using Auto ISO. Keeps images clean in good light by preventing unnecessary sensitivity boost.",
            Some(V::Iso),
        ),
        PropertyDef::new(
            C::IsoAutoRangeLimitMax,
            "ISO Auto Range Max",
            "Maximum ISO when using Auto ISO. Limits noise by capping how high sensitivity can go.",
            Some(V::Iso),
        ),
        PropertyDef::new(
            C::ApertureDriveInAF,
            "Aperture Drive in AF",
            "Controls aperture behavior during autofocus. Standard mode adjusts for focus speed, Focus Priority prioritizes tracking, Silent Priority minimizes noise.",
            Some(V::ApertureDriveInAF),
        ),
        PropertyDef::new(
            C::PushAutoIris,
            "Push Iris",
            "Temporarily engages auto iris while button is pressed. Useful for quick exposure checks.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ExposureStep,
            "EV Step Size",
            "Granularity of exposure adjustments. 1/3 EV gives finer control, 1/2 EV gives larger steps.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ShutterSelectMode,
            "Shutter Selection",
            "Chooses between shutter types. Auto selects based on conditions, Manual lets you force a specific type.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::IrisCloseSetting,
            "Iris Close Enable",
            "Allows closing the iris completely. Used for sensor protection or specific exposure effects.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::HighResolutionShutterSpeedAdjust,
            "High Res Shutter Adjust",
            "Fine-grained adjustment for high-resolution shutter speeds. Allows decimal values between standard stops to precisely match light source flicker frequency.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::HighResolutionShutterSpeedAdjustInIntegralMultiples,
            "High Res Shutter Multiples",
            "Constrains high-resolution shutter speed adjustments to integral multiples. Provides structured fine-tuning in proportional steps.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::IsoCurrentSensitivity,
            "ISO (Current)",
            "Current effective ISO value. May differ from set ISO due to Auto ISO or exposure compensation.",
            Some(V::Iso),
        ),
        PropertyDef::new(
            C::ShutterSpeedCurrentValue,
            "SS (Current)",
            "Current effective shutter speed. May differ from set value in auto modes or with exposure compensation.",
            Some(V::ShutterSpeed),
        ),
        PropertyDef::new(
            C::GaindBCurrentValue,
            "Gain (dB)",
            "Current gain level in decibels. 0dB is base sensitivity.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::HighResolutionShutterSpeed,
            "High Res Shutter Speed",
            "Fine-tuned shutter speed beyond standard increments. Used to eliminate banding from flickering artificial light sources like LED and fluorescent.",
            Some(V::Integer),
        ),
    ];
}

crate::register_category!(Exposure);
