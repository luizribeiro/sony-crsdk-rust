//! Camera events received from the SDK callbacks
//!
//! Events are delivered asynchronously when the camera state changes.
//! Use `CameraDevice::events()` to receive them.

use crate::property::PropertyCode;

/// Events received from the camera via SDK callbacks
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum CameraEvent {
    /// Connection established with the camera
    Connected {
        /// SDK protocol version
        version: u32,
    },

    /// Connection to camera lost
    Disconnected {
        /// Error code (0 = normal disconnect)
        error: u32,
    },

    /// One or more camera properties changed
    ///
    /// Call `camera.get_property()` to read the new values.
    PropertyChanged {
        /// Property codes that changed
        codes: Vec<PropertyCode>,
    },

    /// Live view properties changed
    LiveViewPropertyChanged {
        /// Property codes that changed
        codes: Vec<u32>,
    },

    /// File download completed
    DownloadComplete {
        /// Filename of the downloaded file
        filename: String,
    },

    /// Content transfer notification
    ContentsTransfer {
        /// Transfer notification type
        notify: u32,
        /// Content handle
        handle: u64,
        /// Optional filename
        filename: Option<String>,
    },

    /// Warning from the camera
    Warning {
        /// Warning code
        code: u32,
        /// Additional parameters (if available)
        params: Option<(i32, i32, i32)>,
    },

    /// Error from the camera
    Error {
        /// Error code
        code: u32,
    },

    /// Remote transfer progress update
    RemoteTransferProgress {
        /// Notification type
        notify: u32,
        /// Progress percentage (0-100)
        percent: u32,
        /// Filename being transferred
        filename: Option<String>,
    },

    /// Remote transfer data received (for in-memory transfers)
    RemoteTransferData {
        /// Notification type
        notify: u32,
        /// Progress percentage
        percent: u32,
        /// Data chunk
        data: Vec<u8>,
    },

    /// Contents list changed on camera storage
    ContentsListChanged {
        /// Notification type
        notify: u32,
        /// Slot number
        slot: u32,
        /// Number of items added
        added: u32,
    },

    /// Firmware update progress
    FirmwareUpdateProgress {
        /// Notification type
        notify: u32,
    },
}

/// Get a human-readable name for a warning code
pub fn warning_code_name(code: u32) -> &'static str {
    match code {
        // Standard warnings (0x0002xxxx)
        0x00020000 => "Unknown",
        0x00020001 => "Reconnected",
        0x00020002 => "Reconnecting",
        0x00020003 => "Storage Full",
        0x00020004 => "SetFileName Failed",
        0x00020005 => "GetImage Failed",
        0x00020007 => "Network Error",
        0x00020008 => "Network Recovered",
        0x00020009 => "Format Failed",
        0x0002000A => "Format Invalid",
        0x0002000B => "Format Complete",
        0x00020010 => "Frame Not Updated",
        0x00020012 => "Already Connected",

        // Extended warnings (0x0006xxxx)
        0x00060000 => "Ext Unknown",
        0x00060001 => "AF Status",
        0x00060002 => "Operation Results",
        0x00060003 => "Operation Invalid",
        0x00060004 => "PTZF Result",
        0x00060005 => "Preset PTZF Clear",
        0x00060006 => "Preset PTZF Set",
        0x00060007 => "Preset PTZF Event",

        _ => "Unknown Warning",
    }
}

/// Get a human-readable description for warning parameters
pub fn warning_param_description(code: u32, p1: i32) -> Option<&'static str> {
    match code {
        // AF Status (0x00060001)
        0x00060001 => match p1 {
            0x01 => Some("Unlocked"),
            0x02 => Some("Focused (AF-S)"),
            0x03 => Some("Not Focused (AF-S)"),
            0x05 => Some("Tracking Subject (AF-C)"),
            0x06 => Some("Focused (AF-C)"),
            0x07 => Some("Not Focused (AF-C)"),
            0x08 => Some("Unpaused"),
            0x09 => Some("Paused"),
            _ => None,
        },
        // Operation Results (0x00060002)
        0x00060002 => match p1 {
            0 => Some("Invalid"),
            1 => Some("OK"),
            2 => Some("NG"),
            3 => Some("Invalid Parameter"),
            4 => Some("Camera Status Error"),
            5 => Some("Canceled"),
            _ => None,
        },
        _ => None,
    }
}

impl std::fmt::Display for CameraEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CameraEvent::Connected { version } => {
                write!(f, "Connected (protocol v{})", version)
            }
            CameraEvent::Disconnected { error } => {
                if *error == 0 {
                    write!(f, "Disconnected")
                } else {
                    write!(f, "Disconnected (error: 0x{:08X})", error)
                }
            }
            CameraEvent::PropertyChanged { codes } => {
                write!(f, "PropertyChanged ({} properties)", codes.len())
            }
            CameraEvent::LiveViewPropertyChanged { codes } => {
                write!(f, "LiveViewPropertyChanged ({} properties)", codes.len())
            }
            CameraEvent::DownloadComplete { filename } => {
                write!(f, "DownloadComplete: {}", filename)
            }
            CameraEvent::ContentsTransfer { notify, .. } => {
                write!(f, "ContentsTransfer (notify: {})", notify)
            }
            CameraEvent::Warning { code, .. } => {
                write!(f, "Warning: 0x{:08X}", code)
            }
            CameraEvent::Error { code } => {
                write!(f, "Error: 0x{:08X}", code)
            }
            CameraEvent::RemoteTransferProgress { percent, .. } => {
                write!(f, "RemoteTransferProgress: {}%", percent)
            }
            CameraEvent::RemoteTransferData { percent, data, .. } => {
                write!(f, "RemoteTransferData: {}% ({} bytes)", percent, data.len())
            }
            CameraEvent::ContentsListChanged { added, .. } => {
                write!(f, "ContentsListChanged: {} items added", added)
            }
            CameraEvent::FirmwareUpdateProgress { notify } => {
                write!(f, "FirmwareUpdateProgress (notify: {})", notify)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_display() {
        let event = CameraEvent::Connected { version: 1 };
        assert_eq!(event.to_string(), "Connected (protocol v1)");

        let event = CameraEvent::Disconnected { error: 0 };
        assert_eq!(event.to_string(), "Disconnected");

        let event = CameraEvent::Disconnected { error: 0x8200 };
        assert_eq!(event.to_string(), "Disconnected (error: 0x00008200)");

        let event = CameraEvent::PropertyChanged { codes: vec![] };
        assert_eq!(event.to_string(), "PropertyChanged (0 properties)");
    }

    #[test]
    fn test_event_clone() {
        let event = CameraEvent::RemoteTransferData {
            notify: 1,
            percent: 50,
            data: vec![1, 2, 3, 4],
        };
        let cloned = event.clone();
        if let CameraEvent::RemoteTransferData { data, .. } = cloned {
            assert_eq!(data, vec![1, 2, 3, 4]);
        } else {
            panic!("Clone failed");
        }
    }
}
