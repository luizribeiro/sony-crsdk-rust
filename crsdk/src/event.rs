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
