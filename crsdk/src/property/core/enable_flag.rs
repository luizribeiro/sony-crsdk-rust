//! Property enable/writable status.

use crate::types::FromCrsdk;

/// Property enable/writable status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnableFlag {
    /// Property is not supported by this camera
    NotSupported,
    /// Property is disabled
    Disabled,
    /// Property is readable and writable
    ReadWrite,
    /// Property is read-only (display only)
    ReadOnly,
    /// Property is write-only
    WriteOnly,
}

impl FromCrsdk<i16> for EnableFlag {
    fn from_crsdk(value: i16) -> crate::error::Result<Self> {
        use crsdk_sys::SCRSDK::*;
        Ok(match value {
            x if x == CrPropertyEnableFlag_CrEnableValue_NotSupported => Self::NotSupported,
            x if x == CrPropertyEnableFlag_CrEnableValue_False => Self::Disabled,
            x if x == CrPropertyEnableFlag_CrEnableValue_True => Self::ReadWrite,
            x if x == CrPropertyEnableFlag_CrEnableValue_DisplayOnly => Self::ReadOnly,
            x if x == CrPropertyEnableFlag_CrEnableValue_SetOnly => Self::WriteOnly,
            _ => Self::NotSupported,
        })
    }
}

impl EnableFlag {
    /// Check if the property is readable
    pub fn is_readable(self) -> bool {
        matches!(self, Self::ReadWrite | Self::ReadOnly)
    }

    /// Check if the property is writable
    pub fn is_writable(self) -> bool {
        matches!(self, Self::ReadWrite | Self::WriteOnly)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enable_flag_readable_writable() {
        assert!(EnableFlag::ReadWrite.is_readable());
        assert!(EnableFlag::ReadWrite.is_writable());
        assert!(EnableFlag::ReadOnly.is_readable());
        assert!(!EnableFlag::ReadOnly.is_writable());
        assert!(!EnableFlag::WriteOnly.is_readable());
        assert!(EnableFlag::WriteOnly.is_writable());
        assert!(!EnableFlag::NotSupported.is_readable());
        assert!(!EnableFlag::NotSupported.is_writable());
    }
}
