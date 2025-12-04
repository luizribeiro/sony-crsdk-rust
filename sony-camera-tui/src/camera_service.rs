//! Camera service layer for SDK communication
//!
//! This module provides a background service that manages SDK communication
//! via bidirectional channels, keeping the UI responsive.

use std::net::Ipv4Addr;

use tokio::sync::mpsc;

use crsdk::{
    warning_code_name, warning_param_description, CameraDevice, CameraEvent as SdkEvent,
    DeviceProperty, MacAddr, PropertyCode,
};

use crate::property::{format_sdk_value, PropertyId};

/// Messages from CameraService to App
#[derive(Debug, Clone)]
pub enum CameraUpdate {
    /// Camera connected successfully
    Connected { model: String, address: String },
    /// Camera disconnected
    Disconnected { error: Option<String> },
    /// A property value changed
    PropertyChanged {
        id: PropertyId,
        value: String,
        available: Vec<String>,
        writable: bool,
    },
    /// An error occurred
    Error { message: String },
    /// Discovery results are available
    DiscoveryResult { cameras: Vec<DiscoveredCameraInfo> },
    /// Discovery started
    DiscoveryStarted,
    /// SDK event for logging
    SdkEvent { event_type: String, details: String },
    /// Capture completed
    CaptureComplete { success: bool },
    /// Recording state changed
    RecordingStateChanged { is_recording: bool },
    /// Properties have been loaded from the camera
    PropertiesLoaded,
    /// Camera info updated (battery, lens, media slots)
    /// For slots: None means camera doesn't have that slot, Some(status) means slot exists
    CameraInfoUpdate {
        battery_percent: Option<u8>,
        lens_model: Option<String>,
        focal_length_mm: Option<u32>,
        overheating_state: Option<u8>,
        slot1: Option<SlotInfo>,
        slot2: Option<SlotInfo>,
        slot3: Option<SlotInfo>,
    },
    /// SSH fingerprint fetched, waiting for user confirmation
    SshFingerprintFetched {
        fingerprint: String,
        ip: Ipv4Addr,
        mac: MacAddr,
        ssh_user: String,
        ssh_pass: String,
    },
}

/// Discovered camera info for the UI
#[derive(Debug, Clone)]
pub struct DiscoveredCameraInfo {
    pub model: String,
    pub address: String,
    pub mac_address: String,
    pub connection_type: ConnectionType,
    pub ssh_supported: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    Network,
    Usb,
}

impl std::fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network => write!(f, "Network"),
            Self::Usb => write!(f, "USB"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaSlotStatus {
    Ok,
    NoCard,
    Error,
}

#[derive(Debug, Clone)]
pub struct SlotInfo {
    pub status: MediaSlotStatus,
    pub remaining_photos: Option<u32>,
    pub remaining_time_sec: Option<u32>,
}

/// Commands from App to CameraService
#[derive(Debug)]
pub enum CameraCommand {
    /// Discover available cameras
    Discover,
    /// Fetch SSH fingerprint (first step for SSH connection)
    FetchSshFingerprint {
        ip: Ipv4Addr,
        mac: MacAddr,
        ssh_user: String,
        ssh_pass: String,
    },
    /// Connect to a camera with SSH fingerprint (after user confirms)
    ConnectWithFingerprint {
        ip: Ipv4Addr,
        mac: MacAddr,
        ssh_user: String,
        ssh_pass: String,
        fingerprint: String,
    },
    /// Connect to a camera (non-SSH)
    Connect { ip: Ipv4Addr, mac: MacAddr },
    /// Disconnect from current camera
    Disconnect,
    /// Set a property value by index in the available values
    SetProperty { id: PropertyId, value_index: usize },
    /// Set a property by raw value
    SetPropertyRaw { code: PropertyCode, value: u64 },
    /// Capture a photo
    Capture,
    /// Start video recording
    StartRecording,
    /// Stop video recording
    StopRecording,
    /// Half-press shutter (autofocus)
    HalfPressShutter,
    /// Release shutter button
    ReleaseShutter,
    /// Sync all properties (fetch current values)
    SyncProperties,
}

/// Handle to communicate with the camera service
pub struct CameraServiceHandle {
    pub cmd_tx: mpsc::Sender<CameraCommand>,
    pub update_rx: mpsc::Receiver<CameraUpdate>,
}

impl CameraServiceHandle {
    /// Send a command to the camera service
    pub async fn send(
        &self,
        cmd: CameraCommand,
    ) -> Result<(), mpsc::error::SendError<CameraCommand>> {
        self.cmd_tx.send(cmd).await
    }

    /// Try to receive an update without blocking
    pub fn try_recv(&mut self) -> Option<CameraUpdate> {
        self.update_rx.try_recv().ok()
    }
}

/// The camera service that runs as a background task
pub struct CameraService {
    cmd_rx: mpsc::Receiver<CameraCommand>,
    update_tx: mpsc::Sender<CameraUpdate>,
    device: Option<CameraDevice>,
    event_rx: Option<mpsc::UnboundedReceiver<SdkEvent>>,
    cached_properties: std::collections::HashMap<PropertyCode, DeviceProperty>,
    /// Whether AF (half-press) is currently engaged
    af_engaged: bool,
    /// When to auto-release AF (following SDK example pattern of fixed delay)
    af_release_at: Option<tokio::time::Instant>,
}

impl CameraService {
    /// Spawn the camera service as a background task
    pub fn spawn() -> CameraServiceHandle {
        let (cmd_tx, cmd_rx) = mpsc::channel(32);
        let (update_tx, update_rx) = mpsc::channel(64);

        let service = Self {
            cmd_rx,
            update_tx,
            device: None,
            event_rx: None,
            cached_properties: std::collections::HashMap::new(),
            af_engaged: false,
            af_release_at: None,
        };

        tokio::spawn(service.run());

        CameraServiceHandle { cmd_tx, update_rx }
    }

    async fn run(mut self) {
        loop {
            // Check if we need to auto-release AF
            let af_timeout = if let Some(release_at) = self.af_release_at {
                let now = tokio::time::Instant::now();
                if release_at <= now {
                    Some(tokio::time::Duration::ZERO)
                } else {
                    Some(release_at - now)
                }
            } else {
                None
            };

            tokio::select! {
                Some(cmd) = self.cmd_rx.recv() => {
                    self.handle_command(cmd).await;
                }
                Some(event) = recv_event(&mut self.event_rx) => {
                    self.handle_device_event(event).await;
                }
                _ = async {
                    if let Some(timeout) = af_timeout {
                        tokio::time::sleep(timeout).await;
                    } else {
                        std::future::pending::<()>().await;
                    }
                } => {
                    // AF timeout - auto-release shutter
                    self.handle_af_timeout().await;
                }
            }
        }
    }
}

async fn recv_event(rx: &mut Option<mpsc::UnboundedReceiver<SdkEvent>>) -> Option<SdkEvent> {
    match rx {
        Some(receiver) => receiver.recv().await,
        None => std::future::pending().await,
    }
}

impl CameraService {
    async fn send_update(&self, update: CameraUpdate) {
        if let Err(e) = self.update_tx.send(update).await {
            tracing::debug!("UI channel closed, update not sent: {}", e);
        }
    }

    async fn handle_command(&mut self, cmd: CameraCommand) {
        match cmd {
            CameraCommand::Discover => {
                self.handle_discover().await;
            }
            CameraCommand::FetchSshFingerprint {
                ip,
                mac,
                ssh_user,
                ssh_pass,
            } => {
                self.handle_fetch_ssh_fingerprint(ip, mac, ssh_user, ssh_pass)
                    .await;
            }
            CameraCommand::ConnectWithFingerprint {
                ip,
                mac,
                ssh_user,
                ssh_pass,
                fingerprint,
            } => {
                self.handle_connect_with_fingerprint(ip, mac, ssh_user, ssh_pass, fingerprint)
                    .await;
            }
            CameraCommand::Connect { ip, mac } => {
                self.handle_connect(ip, mac).await;
            }
            CameraCommand::Disconnect => {
                self.handle_disconnect().await;
            }
            CameraCommand::SetProperty { id, value_index } => {
                self.handle_set_property(id, value_index).await;
            }
            CameraCommand::SetPropertyRaw { code, value } => {
                self.handle_set_property_raw(code, value).await;
            }
            CameraCommand::Capture => {
                self.handle_capture().await;
            }
            CameraCommand::StartRecording => {
                self.handle_start_recording().await;
            }
            CameraCommand::StopRecording => {
                self.handle_stop_recording().await;
            }
            CameraCommand::HalfPressShutter => {
                self.handle_half_press().await;
            }
            CameraCommand::ReleaseShutter => {
                self.handle_release_shutter().await;
            }
            CameraCommand::SyncProperties => {
                self.sync_all_properties().await;
            }
        }
    }

    async fn handle_discover(&mut self) {
        self.send_update(CameraUpdate::DiscoveryStarted).await;

        match crsdk::discover_cameras(5).await {
            Ok(cameras) => {
                let infos: Vec<DiscoveredCameraInfo> = cameras
                    .into_iter()
                    .map(|c| DiscoveredCameraInfo {
                        model: c.model.clone(),
                        address: c
                            .ip_address
                            .map(|ip| ip.to_string())
                            .unwrap_or_else(|| "USB".to_string()),
                        mac_address: c.mac_address.map(|m| m.to_string()).unwrap_or_default(),
                        connection_type: if c.ip_address.is_some() {
                            ConnectionType::Network
                        } else {
                            ConnectionType::Usb
                        },
                        ssh_supported: c.ssh_supported,
                    })
                    .collect();

                self.send_update(CameraUpdate::DiscoveryResult { cameras: infos })
                    .await;
            }
            Err(e) => {
                self.send_update(CameraUpdate::Error {
                    message: format!("Discovery failed: {}", e),
                })
                .await;
            }
        }
    }

    async fn handle_fetch_ssh_fingerprint(
        &mut self,
        ip: Ipv4Addr,
        mac: MacAddr,
        ssh_user: String,
        ssh_pass: String,
    ) {
        tracing::info!("Fetching SSH fingerprint for {}...", ip);

        let mut builder = CameraDevice::builder()
            .ip_address(ip)
            .mac_address(mac)
            .ssh_enabled(true);

        match builder.fetch_ssh_fingerprint().await {
            Ok(fingerprint) => {
                tracing::info!("Got SSH fingerprint: {}", fingerprint);
                self.send_update(CameraUpdate::SshFingerprintFetched {
                    fingerprint,
                    ip,
                    mac,
                    ssh_user,
                    ssh_pass,
                })
                .await;
            }
            Err(e) => {
                tracing::error!("Failed to fetch SSH fingerprint: {}", e);
                self.send_update(CameraUpdate::Error {
                    message: format!("Failed to fetch SSH fingerprint: {}", e),
                })
                .await;
            }
        }
    }

    async fn handle_connect_with_fingerprint(
        &mut self,
        ip: Ipv4Addr,
        mac: MacAddr,
        ssh_user: String,
        ssh_pass: String,
        fingerprint: String,
    ) {
        tracing::info!("Connecting to {} with SSH (user={})...", ip, ssh_user);

        let builder = CameraDevice::builder()
            .ip_address(ip)
            .mac_address(mac)
            .ssh_enabled(true)
            .ssh_credentials(&ssh_user, &ssh_pass)
            .ssh_fingerprint(fingerprint);

        match builder.connect().await {
            Ok(mut device) => {
                let model = device.model().await.to_string();
                let address = ip.to_string();
                tracing::info!("Connected to {} ({})", model, address);

                self.event_rx = device.take_event_receiver();
                self.device = Some(device);

                self.send_update(CameraUpdate::Connected {
                    model: model.clone(),
                    address,
                })
                .await;

                // Wait for camera to initialize and send property updates
                tracing::info!("Waiting for camera to initialize...");
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                tracing::info!("Syncing properties...");
                self.sync_all_properties().await;
                tracing::info!("Property sync complete");
            }
            Err(e) => {
                tracing::error!("Connection failed: {}", e);
                self.send_update(CameraUpdate::Error {
                    message: format!("Connection failed: {}", e),
                })
                .await;
            }
        }
    }

    async fn handle_connect(&mut self, ip: Ipv4Addr, mac: MacAddr) {
        tracing::info!("Connecting to {} (no SSH)...", ip);

        let builder = CameraDevice::builder().ip_address(ip).mac_address(mac);

        match builder.connect().await {
            Ok(mut device) => {
                let model = device.model().await.to_string();
                let address = ip.to_string();
                tracing::info!("Connected to {} ({})", model, address);

                self.event_rx = device.take_event_receiver();
                self.device = Some(device);

                self.send_update(CameraUpdate::Connected {
                    model: model.clone(),
                    address,
                })
                .await;

                // Wait for camera to initialize and send property updates
                tracing::info!("Waiting for camera to initialize...");
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                tracing::info!("Syncing properties...");
                self.sync_all_properties().await;
                tracing::info!("Property sync complete");
            }
            Err(e) => {
                tracing::error!("Connection failed: {}", e);
                self.send_update(CameraUpdate::Error {
                    message: format!("Connection failed: {}", e),
                })
                .await;
            }
        }
    }

    async fn handle_disconnect(&mut self) {
        self.device = None;
        self.event_rx = None;
        self.cached_properties.clear();
        self.send_update(CameraUpdate::Disconnected { error: None })
            .await;
    }

    async fn sync_all_properties(&mut self) {
        let Some(ref device) = self.device else {
            return;
        };

        // First, dump ALL properties with debug info to understand SDK values
        match device.get_all_properties_debug().await {
            Ok(all_props) => {
                tracing::info!("Camera exposes {} properties:", all_props.len());
                for (prop, debug_info) in &all_props {
                    tracing::info!(
                        "  - code=0x{:08X} value={} writable={} {}",
                        prop.code,
                        prop.current_value,
                        prop.enable_flag.is_writable(),
                        debug_info
                    );
                }
            }
            Err(e) => {
                tracing::error!("Failed to get all properties: {}", e);
            }
        }

        // Now try to sync the specific properties we care about
        for property_id in PropertyId::all_sdk_mapped() {
            let Some(code) = property_id.to_sdk_code() else {
                continue;
            };

            tracing::debug!(
                "Looking for {:?} (code=0x{:08X})",
                property_id,
                code.as_raw()
            );

            match device.get_property(code).await {
                Ok(prop) => {
                    let current = format_sdk_value(property_id, prop.current_value);
                    let available: Vec<String> = prop
                        .possible_values
                        .iter()
                        .map(|&v| format_sdk_value(property_id, v))
                        .collect();
                    let writable = prop.enable_flag.is_writable();

                    tracing::debug!(
                        "Property {:?}: raw={} formatted='{}' possible_values={:?} available={:?} writable={}",
                        property_id,
                        prop.current_value,
                        current,
                        prop.possible_values,
                        available,
                        writable
                    );

                    self.cached_properties.insert(code, prop);

                    self.send_update(CameraUpdate::PropertyChanged {
                        id: property_id,
                        value: current,
                        available,
                        writable,
                    })
                    .await;
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to read property {:?} (0x{:08X}): {}",
                        property_id,
                        code.as_raw(),
                        e
                    );
                }
            }
        }

        self.send_update(CameraUpdate::PropertiesLoaded).await;

        self.sync_camera_info().await;
    }

    async fn sync_camera_info(&mut self) {
        let Some(ref device) = self.device else {
            return;
        };

        let mut battery_percent: Option<u8> = None;
        let mut lens_model: Option<String> = None;
        let mut focal_length_mm: Option<u32> = None;
        let mut overheating_state: Option<u8> = None;

        // Battery
        if let Ok(prop) = device.get_property(PropertyCode::BatteryRemain).await {
            battery_percent = Some(prop.current_value as u8);
            tracing::debug!("Battery: {}%", prop.current_value);
        }

        // Lens model - uses string property API
        if let Ok(prop) = device.get_property(PropertyCode::LensModelName).await {
            tracing::debug!(
                "Lens model property: data_type={:?} current_value={} current_string={:?}",
                prop.data_type,
                prop.current_value,
                prop.current_string
            );
            lens_model = prop.current_string;
        }

        // Focal length / zoom distance (in mm)
        if let Ok(prop) = device.get_property(PropertyCode::ZoomDistance).await {
            if prop.current_value > 0 && prop.current_value != 0xFFFFFFFF {
                focal_length_mm = Some(prop.current_value as u32);
                tracing::debug!("Zoom distance: {}mm", prop.current_value);
            }
        }

        // Overheating state (0=normal, 1=pre-overheating, 2=overheating)
        if let Ok(prop) = device
            .get_property(PropertyCode::DeviceOverheatingState)
            .await
        {
            overheating_state = Some(prop.current_value as u8);
            tracing::debug!("Overheating state: {}", prop.current_value);
        }

        // Media slot 1 - if status read fails, camera doesn't have this slot
        let slot1 = if let Ok(prop) = device.get_property(PropertyCode::MediaSlot1Status).await {
            let status = parse_slot_status(prop.current_value);
            tracing::debug!("Slot 1 status: {:?}", status);

            let remaining_photos = device
                .get_property(PropertyCode::MediaSlot1RemainingPhotos)
                .await
                .ok()
                .map(|p| p.current_value as u32);
            let remaining_time_sec = device
                .get_property(PropertyCode::MediaSlot1RemainingTime)
                .await
                .ok()
                .map(|p| p.current_value as u32);

            tracing::debug!(
                "Slot 1: photos={:?}, time={:?}s",
                remaining_photos,
                remaining_time_sec
            );
            Some(SlotInfo {
                status,
                remaining_photos,
                remaining_time_sec,
            })
        } else {
            None
        };

        // Media slot 2
        let slot2 = if let Ok(prop) = device.get_property(PropertyCode::MediaSlot2Status).await {
            let status = parse_slot_status(prop.current_value);
            tracing::debug!("Slot 2 status: {:?}", status);

            let remaining_photos = device
                .get_property(PropertyCode::MediaSlot2RemainingPhotos)
                .await
                .ok()
                .map(|p| p.current_value as u32);
            let remaining_time_sec = device
                .get_property(PropertyCode::MediaSlot2RemainingTime)
                .await
                .ok()
                .map(|p| p.current_value as u32);

            tracing::debug!(
                "Slot 2: photos={:?}, time={:?}s",
                remaining_photos,
                remaining_time_sec
            );
            Some(SlotInfo {
                status,
                remaining_photos,
                remaining_time_sec,
            })
        } else {
            None
        };

        // Media slot 3 (optional - some cameras have 3 slots)
        let slot3 = if let Ok(prop) = device.get_property(PropertyCode::MediaSlot3Status).await {
            let status = parse_slot_status(prop.current_value);
            tracing::debug!("Slot 3 status: {:?}", status);

            let remaining_time_sec = device
                .get_property(PropertyCode::MediaSlot3RemainingTime)
                .await
                .ok()
                .map(|p| p.current_value as u32);

            tracing::debug!("Slot 3: time={:?}s", remaining_time_sec);
            Some(SlotInfo {
                status,
                remaining_photos: None,
                remaining_time_sec,
            })
        } else {
            None
        };

        self.send_update(CameraUpdate::CameraInfoUpdate {
            battery_percent,
            lens_model,
            focal_length_mm,
            overheating_state,
            slot1,
            slot2,
            slot3,
        })
        .await;
    }

    async fn handle_set_property(&mut self, id: PropertyId, value_index: usize) {
        let Some(code) = id.to_sdk_code() else {
            self.send_update(CameraUpdate::Error {
                message: format!("Property {:?} has no SDK mapping", id),
            })
            .await;
            return;
        };

        let Some(cached) = self.cached_properties.get(&code) else {
            self.send_update(CameraUpdate::Error {
                message: format!("Property {:?} not in cache", id),
            })
            .await;
            return;
        };

        let Some(&value) = cached.possible_values.get(value_index) else {
            self.send_update(CameraUpdate::Error {
                message: format!("Invalid value index {} for {:?}", value_index, id),
            })
            .await;
            return;
        };

        self.handle_set_property_raw(code, value).await;
    }

    async fn handle_set_property_raw(&mut self, code: PropertyCode, value: u64) {
        let Some(ref device) = self.device else {
            self.send_update(CameraUpdate::Error {
                message: "Not connected".to_string(),
            })
            .await;
            return;
        };

        if let Err(e) = device.set_property(code, value).await {
            self.send_update(CameraUpdate::Error {
                message: format!("Failed to set property: {}", e),
            })
            .await;
        }
    }

    async fn handle_capture(&mut self) {
        let Some(ref device) = self.device else {
            self.send_update(CameraUpdate::Error {
                message: "Not connected".to_string(),
            })
            .await;
            return;
        };

        match device.capture().await {
            Ok(()) => {
                self.send_update(CameraUpdate::CaptureComplete { success: true })
                    .await;
                self.send_update(CameraUpdate::SdkEvent {
                    event_type: "Capture".to_string(),
                    details: "Photo captured".to_string(),
                })
                .await;
            }
            Err(e) => {
                self.send_update(CameraUpdate::CaptureComplete { success: false })
                    .await;
                self.send_update(CameraUpdate::Error {
                    message: format!("Capture failed: {}", e),
                })
                .await;
            }
        }
    }

    async fn handle_start_recording(&mut self) {
        let Some(ref device) = self.device else {
            self.send_update(CameraUpdate::Error {
                message: "Not connected".to_string(),
            })
            .await;
            return;
        };

        match device.start_recording().await {
            Ok(()) => {
                self.send_update(CameraUpdate::RecordingStateChanged { is_recording: true })
                    .await;
                self.send_update(CameraUpdate::SdkEvent {
                    event_type: "Recording".to_string(),
                    details: "Recording started".to_string(),
                })
                .await;
            }
            Err(e) => {
                self.send_update(CameraUpdate::Error {
                    message: format!("Start recording failed: {}", e),
                })
                .await;
            }
        }
    }

    async fn handle_stop_recording(&mut self) {
        let Some(ref device) = self.device else {
            self.send_update(CameraUpdate::Error {
                message: "Not connected".to_string(),
            })
            .await;
            return;
        };

        match device.stop_recording().await {
            Ok(()) => {
                self.send_update(CameraUpdate::RecordingStateChanged {
                    is_recording: false,
                })
                .await;
                self.send_update(CameraUpdate::SdkEvent {
                    event_type: "Recording".to_string(),
                    details: "Recording stopped".to_string(),
                })
                .await;
            }
            Err(e) => {
                self.send_update(CameraUpdate::Error {
                    message: format!("Stop recording failed: {}", e),
                })
                .await;
            }
        }
    }

    async fn handle_half_press(&mut self) {
        let Some(ref device) = self.device else {
            tracing::warn!("Half-press: no device connected");
            return;
        };

        tracing::info!("Half-press shutter (autofocus) starting...");

        // Following the SDK example pattern: send half-press, wait 1 second, then release.
        // The SDK example uses fixed delays rather than waiting for AF status events,
        // because those events aren't always reliably sent (especially in AF-C mode).
        match device.half_press_shutter().await {
            Ok(()) => {
                tracing::info!("Half-press shutter sent successfully");
                self.af_engaged = true;
                // Schedule auto-release after 1 second (like SDK example)
                self.af_release_at =
                    Some(tokio::time::Instant::now() + tokio::time::Duration::from_secs(1));
            }
            Err(e) => {
                tracing::error!("Half-press shutter failed: {}", e);
                self.send_update(CameraUpdate::Error {
                    message: format!("Autofocus failed: {}", e),
                })
                .await;
            }
        }
    }

    async fn handle_af_timeout(&mut self) {
        tracing::info!("AF timeout - auto-releasing shutter");
        self.af_release_at = None;
        self.af_engaged = false;

        if let Some(ref device) = self.device {
            if let Err(e) = device.release_shutter().await {
                tracing::debug!("Auto-release shutter failed: {}", e);
            }
        }
    }

    async fn handle_release_shutter(&mut self) {
        let Some(ref device) = self.device else {
            return;
        };

        if let Err(e) = device.release_shutter().await {
            tracing::debug!("Release shutter failed: {}", e);
        }
    }

    async fn handle_device_event(&mut self, event: SdkEvent) {
        match event {
            SdkEvent::Connected { version } => {
                self.send_update(CameraUpdate::SdkEvent {
                    event_type: "Connected".to_string(),
                    details: format!("Protocol v{}", version),
                })
                .await;
            }
            SdkEvent::Disconnected { error } => {
                let error_msg = if error == 0 {
                    None
                } else {
                    Some(format!("Error code: 0x{:08X}", error))
                };
                self.device = None;
                self.event_rx = None;
                self.cached_properties.clear();
                self.send_update(CameraUpdate::Disconnected { error: error_msg })
                    .await;
            }
            SdkEvent::PropertyChanged { codes } => {
                for code in codes {
                    if let Some(property_id) = PropertyId::from_sdk_code(code) {
                        if let Some(ref device) = self.device {
                            if let Ok(prop) = device.get_property(code).await {
                                let current = format_sdk_value(property_id, prop.current_value);
                                let available: Vec<String> = prop
                                    .possible_values
                                    .iter()
                                    .map(|&v| format_sdk_value(property_id, v))
                                    .collect();
                                let writable = prop.enable_flag.is_writable();

                                self.cached_properties.insert(code, prop);

                                self.send_update(CameraUpdate::PropertyChanged {
                                    id: property_id,
                                    value: current,
                                    available,
                                    writable,
                                })
                                .await;
                            }
                        }
                    }
                }
            }
            SdkEvent::Warning { code, params } => {
                let warning_name = warning_code_name(code);
                let details = if let Some((p1, p2, p3)) = params {
                    let param_desc = warning_param_description(code, p1);

                    // Handle AF Status events (0x00060001)
                    if code == 0x00060001 {
                        // AF Status codes:
                        // 0x02 = Focused (AF-S), 0x03 = Not Focused (AF-S)
                        // 0x06 = Focused (AF-C), 0x07 = Not Focused (AF-C)
                        // 0x01 = Unlocked
                        let is_focused = p1 == 0x02 || p1 == 0x06;
                        let is_not_focused = p1 == 0x03 || p1 == 0x07;
                        let is_unlocked = p1 == 0x01;

                        if self.af_engaged && (is_focused || is_not_focused) {
                            // Release the shutter now that focus result is known
                            if let Some(ref device) = self.device {
                                tracing::info!("AF complete (status={}), releasing shutter", p1);
                                let _ = device.release_shutter().await;
                                self.af_engaged = false;
                                self.af_release_at = None; // Cancel auto-release timer
                            }
                        }

                        // Resync properties when unlocked - always resync regardless of
                        // af_engaged state since interrupted AFs may not have cleaned up
                        if is_unlocked {
                            tracing::info!("AF unlocked, resyncing properties");
                            self.af_engaged = false;
                            self.af_release_at = None; // Cancel auto-release timer
                            self.sync_all_properties().await;
                        }
                    }

                    match param_desc {
                        Some(desc) => format!("{}: {}", warning_name, desc),
                        None => format!(
                            "{} (0x{:08X}) p1={} p2={} p3={}",
                            warning_name, code, p1, p2, p3
                        ),
                    }
                } else {
                    format!("{} (0x{:08X})", warning_name, code)
                };
                self.send_update(CameraUpdate::SdkEvent {
                    event_type: "Warning".to_string(),
                    details,
                })
                .await;
            }
            SdkEvent::Error { code } => {
                self.send_update(CameraUpdate::SdkEvent {
                    event_type: "Error".to_string(),
                    details: format!("Code: 0x{:08X}", code),
                })
                .await;
            }
            SdkEvent::DownloadComplete { filename } => {
                self.send_update(CameraUpdate::SdkEvent {
                    event_type: "Download".to_string(),
                    details: filename,
                })
                .await;
            }
            _ => {}
        }
    }
}

fn parse_slot_status(value: u64) -> MediaSlotStatus {
    // CrSlotStatus enum values from SDK
    const CR_SLOT_STATUS_OK: u64 = 0x0000;
    const CR_SLOT_STATUS_NO_CARD: u64 = 0x0001;
    // Other status codes (0x0002-0x0008) indicate various error states

    match value {
        CR_SLOT_STATUS_OK => MediaSlotStatus::Ok,
        CR_SLOT_STATUS_NO_CARD => MediaSlotStatus::NoCard,
        _ => MediaSlotStatus::Error,
    }
}
