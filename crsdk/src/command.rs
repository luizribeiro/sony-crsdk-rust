//! Camera command types for shooting operations

/// Command IDs for camera operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CommandId {
    /// Full shutter release (take photo)
    Release = crsdk_sys::SCRSDK::CrCommandId_CrCommandId_Release,
    /// Cancel ongoing shooting
    CancelShooting = crsdk_sys::SCRSDK::CrCommandId_CrCommandId_CancelShooting,
    /// Movie recording start/stop
    MovieRecord = crsdk_sys::SCRSDK::CrCommandId_CrCommandId_MovieRecord,
    /// S1 (half-press) and release combined
    S1AndRelease = crsdk_sys::SCRSDK::CrCommandId_CrCommandId_S1andRelease,
    /// Toggle movie recording button
    MovieRecButtonToggle = crsdk_sys::SCRSDK::CrCommandId_CrCommandId_MovieRecButtonToggle,
    /// Cancel focus position
    CancelFocusPosition = crsdk_sys::SCRSDK::CrCommandId_CrCommandId_CancelFocusPosition,
    /// Enable tracking and AF
    TrackingOnAndAfOn = crsdk_sys::SCRSDK::CrCommandId_CrCommandId_TrackingOnAndAFOn,
}

impl CommandId {
    /// Get the raw SDK value
    pub fn as_raw(self) -> u32 {
        self as u32
    }
}

/// Command parameter for button press/release state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum CommandParam {
    /// Button released (up)
    Up = crsdk_sys::SCRSDK::CrCommandParam_CrCommandParam_Up as u16,
    /// Button pressed (down)
    Down = crsdk_sys::SCRSDK::CrCommandParam_CrCommandParam_Down as u16,
}

impl CommandParam {
    /// Get the raw SDK value
    pub fn as_raw(self) -> u32 {
        self as u32
    }
}

/// Lock indicator for S1 (half-press) state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum LockIndicator {
    /// Unknown state
    Unknown = crsdk_sys::SCRSDK::CrLockIndicator_CrLockIndicator_Unknown as u16,
    /// Unlocked (released)
    Unlocked = crsdk_sys::SCRSDK::CrLockIndicator_CrLockIndicator_Unlocked as u16,
    /// Locked (pressed)
    Locked = crsdk_sys::SCRSDK::CrLockIndicator_CrLockIndicator_Locked as u16,
}

impl LockIndicator {
    /// Get the raw SDK value
    pub fn as_raw(self) -> u64 {
        self as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_id_values() {
        assert_eq!(CommandId::Release.as_raw(), 0);
        assert_eq!(CommandId::MovieRecord.as_raw(), 1);
    }

    #[test]
    fn test_command_param_values() {
        assert_eq!(CommandParam::Up.as_raw(), 0);
        assert_eq!(CommandParam::Down.as_raw(), 1);
    }
}
