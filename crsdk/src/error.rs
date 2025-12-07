//! Error types for Sony Camera Remote SDK operations

use thiserror::Error;

/// Result type alias for SDK operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when interacting with the Sony Camera Remote SDK
#[derive(Debug, Error)]
pub enum Error {
    /// SDK initialization failed
    #[error("SDK initialization failed")]
    InitFailed,

    /// SDK operation failed with error code
    #[error("SDK error: 0x{0:X}")]
    SdkError(u32),

    /// Connection failed
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// Invalid parameter provided
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Camera not found
    #[error("Camera not found")]
    CameraNotFound,

    /// Timeout occurred
    #[error("Operation timed out")]
    Timeout,

    /// Memory allocation failed
    #[error("Memory allocation failed")]
    OutOfMemory,

    /// SSH authentication failed
    #[error("SSH authentication failed")]
    SshAuthFailed,

    /// Device disconnected
    #[error("Device disconnected unexpectedly")]
    Disconnected,

    /// Property is not writable
    #[error("Property is not writable")]
    PropertyNotWritable,

    /// Property is not supported
    #[error("Property is not supported")]
    PropertyNotSupported,

    /// Invalid property value
    #[error("Invalid property value")]
    InvalidPropertyValue,

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Address parse error
    #[error("Address parse error: {0}")]
    AddrParse(String),

    /// Other error with description
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Create error from SDK error code
    pub fn from_sdk_error(code: u32) -> Self {
        match code {
            0x0000 => Self::Other("Success (not an error)".to_string()),
            0x8200..=0x82FF => Self::ConnectionFailed(format!("Error code: 0x{:X}", code)),
            0x8300..=0x83FF => Self::OutOfMemory,
            _ => Self::SdkError(code),
        }
    }

    /// Check if SDK error code indicates success
    pub fn check_sdk_result(code: i32) -> Result<()> {
        if code == 0 {
            Ok(())
        } else {
            Err(Self::from_sdk_error(code as u32))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_from_sdk() {
        let err = Error::from_sdk_error(0x8200);
        assert!(matches!(err, Error::ConnectionFailed(_)));
    }
}
