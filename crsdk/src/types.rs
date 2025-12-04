//! Type definitions for camera connections

use crate::error::{Error, Result};
use std::fmt;
use std::net::Ipv4Addr;
use std::str::FromStr;

/// MAC address (6 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddr(pub [u8; 6]);

impl MacAddr {
    /// Create a new MAC address from bytes
    pub fn new(bytes: [u8; 6]) -> Self {
        MacAddr(bytes)
    }

    /// Get MAC address as byte array
    pub fn as_bytes(&self) -> &[u8; 6] {
        &self.0
    }
}

impl FromStr for MacAddr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 6 {
            return Err(Error::AddrParse(
                "MAC address must have 6 octets separated by colons".to_string(),
            ));
        }

        let mut bytes = [0u8; 6];
        for (i, part) in parts.iter().enumerate() {
            bytes[i] = u8::from_str_radix(part, 16)
                .map_err(|_| Error::AddrParse(format!("Invalid hex byte: {}", part)))?;
        }

        Ok(MacAddr(bytes))
    }
}

impl fmt::Display for MacAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

/// Camera model identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum CameraModel {
    /// Sony FX3
    Fx3,
    /// Sony FX6
    Fx6,
    /// Sony FX30
    Fx30,
    /// Sony α1
    Alpha1,
    /// Sony α7 IV
    Alpha7Iv,
    /// Sony α7R V
    Alpha7Rv,
    /// Sony α7S III
    Alpha7Siii,
    /// Sony α9 II
    Alpha9Ii,
    /// Sony α9 III
    Alpha9Iii,
}

impl fmt::Display for CameraModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CameraModel::Fx3 => write!(f, "Sony FX3"),
            CameraModel::Fx6 => write!(f, "Sony FX6"),
            CameraModel::Fx30 => write!(f, "Sony FX30"),
            CameraModel::Alpha1 => write!(f, "Sony α1"),
            CameraModel::Alpha7Iv => write!(f, "Sony α7 IV"),
            CameraModel::Alpha7Rv => write!(f, "Sony α7R V"),
            CameraModel::Alpha7Siii => write!(f, "Sony α7S III"),
            CameraModel::Alpha9Ii => write!(f, "Sony α9 II"),
            CameraModel::Alpha9Iii => write!(f, "Sony α9 III"),
        }
    }
}

impl CameraModel {
    /// All available camera models
    pub const ALL: &'static [CameraModel] = &[
        CameraModel::Fx3,
        CameraModel::Fx6,
        CameraModel::Fx30,
        CameraModel::Alpha1,
        CameraModel::Alpha7Iv,
        CameraModel::Alpha7Rv,
        CameraModel::Alpha7Siii,
        CameraModel::Alpha9Ii,
        CameraModel::Alpha9Iii,
    ];

    /// Get SDK enum value for this model
    pub(crate) fn to_sdk_value(self) -> u32 {
        use crsdk_sys::SCRSDK::*;
        match self {
            CameraModel::Fx3 => CrCameraDeviceModelList_CrCameraDeviceModel_ILME_FX3,
            CameraModel::Fx6 => CrCameraDeviceModelList_CrCameraDeviceModel_ILME_FX6,
            CameraModel::Fx30 => CrCameraDeviceModelList_CrCameraDeviceModel_ILME_FX30,
            CameraModel::Alpha1 => CrCameraDeviceModelList_CrCameraDeviceModel_ILCE_1,
            CameraModel::Alpha7Iv => CrCameraDeviceModelList_CrCameraDeviceModel_ILCE_7M4,
            CameraModel::Alpha7Rv => CrCameraDeviceModelList_CrCameraDeviceModel_ILCE_7RM5,
            CameraModel::Alpha7Siii => CrCameraDeviceModelList_CrCameraDeviceModel_ILCE_7SM3,
            CameraModel::Alpha9Ii => CrCameraDeviceModelList_CrCameraDeviceModel_ILCE_9M2,
            CameraModel::Alpha9Iii => CrCameraDeviceModelList_CrCameraDeviceModel_ILCE_9M3,
        }
    }
}

/// Connection type for a discovered camera
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    /// Ethernet/WiFi network connection
    Network,
    /// USB connection
    Usb,
    /// Unknown connection type
    Unknown,
}

impl fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionType::Network => write!(f, "Network"),
            ConnectionType::Usb => write!(f, "USB"),
            ConnectionType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// A camera discovered through network/USB enumeration
#[derive(Debug, Clone)]
pub struct DiscoveredCamera {
    /// Camera model name (e.g., "ILME-FX3")
    pub model: String,
    /// Device name
    pub name: String,
    /// Connection type
    pub connection_type: ConnectionType,
    /// IP address (for network connections)
    pub ip_address: Option<Ipv4Addr>,
    /// MAC address (for network connections)
    pub mac_address: Option<MacAddr>,
    /// Whether SSH is supported
    pub ssh_supported: bool,
    /// USB product ID (for USB connections)
    pub usb_pid: Option<i16>,
}

impl DiscoveredCamera {
    /// Check if this is a network camera
    pub fn is_network(&self) -> bool {
        self.connection_type == ConnectionType::Network
    }

    /// Check if this is a USB camera
    pub fn is_usb(&self) -> bool {
        self.connection_type == ConnectionType::Usb
    }
}

impl fmt::Display for DiscoveredCamera {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.connection_type {
            ConnectionType::Network => {
                if let Some(ip) = &self.ip_address {
                    write!(f, "{} ({}) at {}", self.model, self.connection_type, ip)
                } else {
                    write!(f, "{} ({})", self.model, self.connection_type)
                }
            }
            ConnectionType::Usb => {
                if let Some(pid) = self.usb_pid {
                    write!(
                        f,
                        "{} ({}, PID: {:04x})",
                        self.model, self.connection_type, pid
                    )
                } else {
                    write!(f, "{} ({})", self.model, self.connection_type)
                }
            }
            ConnectionType::Unknown => write!(f, "{}", self.model),
        }
    }
}

/// Connection information for a camera
#[derive(Debug, Clone, Default)]
pub struct ConnectionInfo {
    /// IP address (for network connections)
    pub ip_address: Option<Ipv4Addr>,
    /// MAC address (for network connections)
    pub mac_address: Option<MacAddr>,
    /// Camera model
    pub model: Option<CameraModel>,
    /// Enable SSH tunnel
    pub ssh_enabled: bool,
    /// SSH username
    pub ssh_user: Option<String>,
    /// SSH password
    pub ssh_password: Option<String>,
    /// SSH fingerprint (fetched from camera, must be confirmed by user)
    pub ssh_fingerprint: Option<String>,
}

/// Convert Ipv4Addr to SDK's 32-bit little-endian format
pub(crate) fn ip_to_sdk_format(ip: Ipv4Addr) -> u32 {
    let octets = ip.octets();
    u32::from_le_bytes(octets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_addr_parse() {
        let mac = "10:32:2c:7d:c7:b3".parse::<MacAddr>().unwrap();
        assert_eq!(mac.0, [0x10, 0x32, 0x2c, 0x7d, 0xc7, 0xb3]);
        assert_eq!(mac.to_string(), "10:32:2C:7D:C7:B3");
    }

    #[test]
    fn test_ip_to_sdk_format() {
        let ip = "192.168.1.100".parse::<Ipv4Addr>().unwrap();
        let sdk_format = ip_to_sdk_format(ip);
        // Little-endian: 100.1.168.192
        assert_eq!(sdk_format, 0x64_01_a8_c0);
    }

    #[test]
    fn test_connection_type_display() {
        assert_eq!(ConnectionType::Network.to_string(), "Network");
        assert_eq!(ConnectionType::Usb.to_string(), "USB");
        assert_eq!(ConnectionType::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn test_discovered_camera_is_network() {
        let camera = DiscoveredCamera {
            model: "ILME-FX3".to_string(),
            name: "ILME-FX3".to_string(),
            connection_type: ConnectionType::Network,
            ip_address: Some("192.168.1.100".parse().unwrap()),
            mac_address: Some("10:32:2c:7d:c7:b3".parse().unwrap()),
            ssh_supported: true,
            usb_pid: None,
        };

        assert!(camera.is_network());
        assert!(!camera.is_usb());
    }

    #[test]
    fn test_discovered_camera_is_usb() {
        let camera = DiscoveredCamera {
            model: "ILME-FX3".to_string(),
            name: "ILME-FX3".to_string(),
            connection_type: ConnectionType::Usb,
            ip_address: None,
            mac_address: None,
            ssh_supported: false,
            usb_pid: Some(0x0c06),
        };

        assert!(!camera.is_network());
        assert!(camera.is_usb());
    }

    #[test]
    fn test_discovered_camera_display_network() {
        let camera = DiscoveredCamera {
            model: "ILME-FX3".to_string(),
            name: "ILME-FX3".to_string(),
            connection_type: ConnectionType::Network,
            ip_address: Some("10.1.3.105".parse().unwrap()),
            mac_address: Some("10:32:2c:7d:c7:b3".parse().unwrap()),
            ssh_supported: true,
            usb_pid: None,
        };

        assert_eq!(camera.to_string(), "ILME-FX3 (Network) at 10.1.3.105");
    }

    #[test]
    fn test_discovered_camera_display_usb() {
        let camera = DiscoveredCamera {
            model: "ILCE-7M4".to_string(),
            name: "ILCE-7M4".to_string(),
            connection_type: ConnectionType::Usb,
            ip_address: None,
            mac_address: None,
            ssh_supported: false,
            usb_pid: Some(0x0c06),
        };

        assert_eq!(camera.to_string(), "ILCE-7M4 (USB, PID: 0c06)");
    }
}
