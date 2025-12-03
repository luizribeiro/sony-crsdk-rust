//! Async camera device connection and control
//!
//! This module provides async wrappers around the blocking API using `block_in_place`.
//! For synchronous code, use `crsdk::blocking` instead.

use crate::blocking;
use crate::error::{Error, Result};
use crate::types::{CameraModel, ConnectionInfo, MacAddr};
use std::net::Ipv4Addr;

/// A connected camera device (async API)
///
/// This wraps the blocking `CameraDevice` for use with async runtimes.
/// All methods are auto-generated via the `asyncwrap` crate using `block_in_place`.
pub struct CameraDevice {
    /// The underlying blocking device (public for macro-generated code)
    pub(crate) inner: blocking::CameraDevice,
}

impl CameraDevice {
    /// Create a new builder for configuring camera connection
    pub fn builder() -> CameraDeviceBuilder {
        CameraDeviceBuilder::new()
    }

    /// Get the underlying blocking device
    pub fn into_inner(self) -> blocking::CameraDevice {
        self.inner
    }
}

/// Builder for configuring and connecting to a camera (async API)
#[derive(Default)]
pub struct CameraDeviceBuilder {
    info: ConnectionInfo,
}

impl CameraDeviceBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the camera's IP address
    pub fn ip_address(mut self, ip: Ipv4Addr) -> Self {
        self.info.ip_address = Some(ip);
        self
    }

    /// Set the camera's MAC address
    pub fn mac_address(mut self, mac: MacAddr) -> Self {
        self.info.mac_address = Some(mac);
        self
    }

    /// Set the camera model
    pub fn model(mut self, model: CameraModel) -> Self {
        self.info.model = Some(model);
        self
    }

    /// Enable or disable SSH tunnel
    pub fn ssh_enabled(mut self, enabled: bool) -> Self {
        self.info.ssh_enabled = enabled;
        self
    }

    /// Set SSH credentials (also enables SSH)
    pub fn ssh_credentials(mut self, user: impl Into<String>, password: impl Into<String>) -> Self {
        self.info.ssh_user = Some(user.into());
        self.info.ssh_password = Some(password.into());
        self.info.ssh_enabled = true;
        self
    }

    /// Set the SSH fingerprint for verification
    pub fn ssh_fingerprint(mut self, fingerprint: impl Into<String>) -> Self {
        self.info.ssh_fingerprint = Some(fingerprint.into());
        self
    }

    /// Fetch SSH fingerprint from camera for user confirmation
    pub async fn fetch_ssh_fingerprint(&mut self) -> Result<String> {
        let info = self.info.clone();

        let fingerprint = tokio::task::spawn_blocking(move || {
            let mut builder = blocking::CameraDeviceBuilder::new();

            if let Some(ip) = info.ip_address {
                builder = builder.ip_address(ip);
            }
            if let Some(mac) = info.mac_address {
                builder = builder.mac_address(mac);
            }
            if let Some(model) = info.model {
                builder = builder.model(model);
            }
            if info.ssh_enabled {
                builder = builder.ssh_enabled(true);
            }

            builder.fetch_ssh_fingerprint()
        })
        .await
        .map_err(|e| Error::Other(format!("Task join error: {}", e)))??;

        Ok(fingerprint)
    }

    /// Connect to the camera asynchronously
    pub async fn connect(self) -> Result<CameraDevice> {
        let info = self.info;

        let inner = tokio::task::spawn_blocking(move || {
            let mut builder = blocking::CameraDeviceBuilder::new();

            if let Some(ip) = info.ip_address {
                builder = builder.ip_address(ip);
            }
            if let Some(mac) = info.mac_address {
                builder = builder.mac_address(mac);
            }
            if let Some(model) = info.model {
                builder = builder.model(model);
            }
            if info.ssh_enabled {
                builder = builder.ssh_enabled(true);
            }
            if let Some(user) = &info.ssh_user {
                if let Some(pass) = &info.ssh_password {
                    builder = builder.ssh_credentials(user, pass);
                }
            }
            if let Some(fp) = info.ssh_fingerprint {
                builder = builder.ssh_fingerprint(fp);
            }

            // For SSH, we need to fetch fingerprint again since we can't reuse across threads
            if info.ssh_enabled && info.ssh_user.is_some() {
                builder.fetch_ssh_fingerprint()?;
            }

            builder.connect()
        })
        .await
        .map_err(|e| Error::Other(format!("Task join error: {}", e)))??;

        Ok(CameraDevice { inner })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_pattern() {
        let builder = CameraDevice::builder()
            .ip_address("192.168.1.100".parse().unwrap())
            .mac_address("10:32:2c:7d:c7:b3".parse().unwrap())
            .model(CameraModel::Fx3);

        assert!(builder.info.ip_address.is_some());
        assert!(builder.info.mac_address.is_some());
    }
}
