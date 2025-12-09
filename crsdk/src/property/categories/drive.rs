//! Drive category: drive mode, bracketing, and timer properties.

use super::{Category, PropertyCategory, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

/// Drive mode, bracketing, and timer properties.
pub struct Drive;

impl Category for Drive {
    const CATEGORY: PropertyCategory = PropertyCategory::Drive;
    const NAME: &'static str = "Drive";
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::DriveMode,
            "Drive (Single/Cont)",
            "Controls how the camera captures. Single takes one shot. Continuous shoots while holding the shutter. Self-timer adds delay. Bracket varies exposure/focus across shots.",
            Some(V::DriveMode),
        ),
        PropertyDef::new(
            C::BracketOrder,
            "Bracket Seq",
            "Order of bracketed shots. 0→-→+ starts at metered, then under, then over. -→0→+ goes dark to bright.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::BulbTimerSetting,
            "Bulb Timer",
            "Timer for bulb exposures. Automatically closes shutter after set time.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ShootingSelfTimerStatus,
            "Self-Timer Status",
            "Current state of the self-timer countdown.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::RecordingSelfTimer,
            "Rec Self-Timer",
            "Self-timer delay before recording starts.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::RecordingSelfTimerCountTime,
            "Self-Timer Count",
            "Countdown duration for recording self-timer.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::RecordingSelfTimerContinuous,
            "Self-Timer Cont.",
            "Number of continuous shots with self-timer.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::RecordingSelfTimerStatus,
            "Self-Timer Status",
            "Current status of recording self-timer.",
            Some(V::Integer),
        ),
    ];
}

crate::register_category!(Drive);
