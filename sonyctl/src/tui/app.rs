use std::collections::VecDeque;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::time::{Duration, Instant};

use super::action::Action;
use super::camera_service::{
    CameraCommand, CameraServiceHandle, CameraUpdate, DiscoveredCameraInfo, MediaSlotStatus,
    SlotInfo,
};
use super::property::PropertyStore;
use crsdk::{
    property_category, property_display_name, CameraModel, DevicePropertyCode, MacAddr,
    PropertyCategoryId,
};

const PROPERTY_DEBOUNCE_MS: u64 = 400;
const IN_FLIGHT_TIMEOUT_MS: u64 = 2000;
const MAX_EVENT_LOG_SIZE: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Discovery,
    Dashboard,
    PropertyEditor,
    EventsExpanded,
}

#[derive(Debug, Clone)]
pub enum Modal {
    SshCredentials(SshCredentialsState),
    SshFingerprintConfirm(SshFingerprintState),
    ManualConnection(ManualConnectionState),
    PropertySearch(PropertySearchState),
    RangeValueInput(RangeValueInputState),
    Error { message: String },
}

#[derive(Debug, Clone)]
pub struct SshFingerprintState {
    pub fingerprint: String,
    pub ip: Ipv4Addr,
    pub mac: MacAddr,
    pub ssh_user: String,
    pub ssh_pass: String,
}

#[derive(Debug, Clone, Default)]
pub struct SshCredentialsState {
    pub camera_name: String,
    pub camera_address: String,
    pub camera_mac: String,
    pub username: String,
    pub password: String,
    pub focused_field: usize,
    pub remember: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ManualConnectionState {
    pub ip_address: String,
    pub mac_address: String,
    pub model_index: usize,
    pub ssh_enabled: bool,
    pub focused_field: usize,
}

#[derive(Debug, Clone, Default)]
pub struct PropertySearchState {
    pub query: String,
    pub results: Vec<DevicePropertyCode>,
    pub selected_index: usize,
}

#[derive(Debug, Clone)]
pub struct RangeValueInputState {
    pub property_code: DevicePropertyCode,
    pub property_name: String,
    pub input: String,
    pub min: i64,
    pub max: i64,
    pub step: i64,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DiscoveredCamera {
    pub model: String,
    pub address: String,
    pub mac_address: String,
    pub connection_type: String,
    pub ssh_supported: bool,
}

impl From<DiscoveredCameraInfo> for DiscoveredCamera {
    fn from(info: DiscoveredCameraInfo) -> Self {
        Self {
            model: info.model,
            address: info.address,
            mac_address: info.mac_address,
            connection_type: info.connection_type.to_string(),
            ssh_supported: info.ssh_supported,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CameraEvent {
    pub timestamp: String,
    pub event_type: String,
    pub details: String,
}

#[derive(Debug, Clone, Default)]
pub struct DiscoveryState {
    pub cameras: Vec<DiscoveredCamera>,
    pub selected_index: usize,
    pub is_scanning: bool,
}

#[derive(Debug, Clone)]
pub struct MediaSlotInfo {
    pub media_type: String,
    pub free_space: String,
}

#[derive(Debug, Clone)]
pub struct CameraInfo {
    pub lens: String,
    pub focal_length: String,
    pub battery: u8,
    pub temperature: Option<String>,
    pub slot1: Option<MediaSlotInfo>,
    pub slot2: Option<MediaSlotInfo>,
    pub slot3: Option<MediaSlotInfo>,
    pub recording_format: String,
    pub image_format: String,
}

impl Default for CameraInfo {
    fn default() -> Self {
        Self {
            lens: "--".to_string(),
            focal_length: "--".to_string(),
            battery: 0,
            temperature: None,
            slot1: None,
            slot2: None,
            slot3: None,
            recording_format: "--".to_string(),
            image_format: "--".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DashboardState {
    pub selected_property: usize,
    pub is_recording: bool,
    pub recording_seconds: u64,
    pub session_seconds: u64,
    pub camera_info: CameraInfo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PropertyEditorFocus {
    Categories,
    #[default]
    Properties,
    Values,
}

#[derive(Debug, Clone, Default)]
pub struct PropertyEditorState {
    pub category_index: usize,
    pub property_index: usize,
    pub focus: PropertyEditorFocus,
    pub value_preview_index: usize,
    pub show_info: bool,
}

impl PropertyEditorState {
    pub fn current_category(&self, available: &[PropertyCategoryId]) -> PropertyCategoryId {
        available
            .get(self.category_index)
            .copied()
            .unwrap_or(PropertyCategoryId("Exposure"))
    }
}

#[derive(Debug, Clone, Default)]
pub struct EventsLogState {
    pub events: VecDeque<CameraEvent>,
    pub scroll_offset: usize,
}

#[derive(Debug, Clone)]
pub struct ConnectedCamera {
    pub model: String,
    pub address: String,
}

pub struct App {
    pub screen: Screen,
    pub modal: Option<Modal>,
    pub help_visible: bool,

    pub discovery: DiscoveryState,
    pub dashboard: DashboardState,
    pub property_editor: PropertyEditorState,
    pub events_log: EventsLogState,
    pub properties: PropertyStore,

    pub connected_camera: Option<ConnectedCamera>,
    pub should_quit: bool,

    camera_service: CameraServiceHandle,

    /// Whether to automatically trust SSH fingerprints (--trust flag)
    trust_ssh_fingerprint: bool,

    /// Pending property change for debouncing (property_code, value_index, timestamp)
    pending_property: Option<(DevicePropertyCode, usize, Instant)>,
    /// Property currently being sent to SDK, waiting for confirmation (with timestamp for timeout)
    in_flight_property: Option<(DevicePropertyCode, Instant)>,
}

impl App {
    pub fn new(camera_service: CameraServiceHandle, trust_ssh_fingerprint: bool) -> Self {
        Self {
            screen: Screen::Discovery,
            modal: None,
            help_visible: false,
            discovery: DiscoveryState::default(),
            dashboard: DashboardState::default(),
            property_editor: PropertyEditorState::default(),
            events_log: EventsLogState::default(),
            properties: PropertyStore::new(),
            connected_camera: None,
            should_quit: false,
            camera_service,
            trust_ssh_fingerprint,
            pending_property: None,
            in_flight_property: None,
        }
    }

    /// Get access to the camera service for sending commands
    pub fn camera_service_cmd(&self) -> &CameraServiceHandle {
        &self.camera_service
    }

    /// Poll for camera service updates (call from main loop)
    pub fn poll_camera_updates(&mut self) {
        while let Some(update) = self.camera_service.try_recv() {
            self.handle_camera_update(update);
        }
    }

    /// Get the duration until the pending property should be flushed, if any
    pub fn debounce_timeout(&self) -> Option<Duration> {
        self.pending_property.map(|(_, _, timestamp)| {
            let elapsed = timestamp.elapsed();
            let debounce = Duration::from_millis(PROPERTY_DEBOUNCE_MS);
            debounce.saturating_sub(elapsed)
        })
    }

    /// Flush any pending property change to the camera (call when debounce timeout fires)
    pub async fn flush_pending_property(&mut self) {
        if let Some((code, value_index, timestamp)) = self.pending_property.take() {
            if timestamp.elapsed() >= Duration::from_millis(PROPERTY_DEBOUNCE_MS) {
                tracing::debug!("Setting in_flight_property to {:?}", code);
                self.in_flight_property = Some((code, Instant::now()));
                let _ = self
                    .camera_service
                    .send(CameraCommand::SetProperty { code, value_index })
                    .await;
            }
        }
    }

    /// Queue a property change with debouncing
    fn queue_property_change(&mut self, code: DevicePropertyCode, value_index: usize) {
        self.pending_property = Some((code, value_index, Instant::now()));
    }

    /// Check if a specific property has a pending change (debouncing)
    pub fn has_pending_change(&self, code: DevicePropertyCode) -> bool {
        self.pending_property
            .map(|(pending_code, _, _)| pending_code == code)
            .unwrap_or(false)
    }

    /// Check if a specific property is in-flight (sent to SDK, waiting for response)
    /// Returns false if the in-flight state has timed out (2 seconds)
    pub fn is_in_flight(&self, code: DevicePropertyCode) -> bool {
        match self.in_flight_property {
            Some((in_flight_code, timestamp)) => {
                in_flight_code == code
                    && timestamp.elapsed() < Duration::from_millis(IN_FLIGHT_TIMEOUT_MS)
            }
            None => false,
        }
    }

    fn handle_camera_update(&mut self, update: CameraUpdate) {
        match update {
            CameraUpdate::Connected { model, address } => {
                self.connected_camera = Some(ConnectedCamera { model, address });
                self.discovery.is_scanning = false;
                self.screen = Screen::Dashboard;
                self.log_event("Connected", "Camera connected");
            }
            CameraUpdate::Disconnected { error } => {
                self.connected_camera = None;
                self.properties.set_loaded(false);
                self.screen = Screen::Discovery;
                if let Some(err) = error {
                    self.log_event("Disconnected", &err);
                } else {
                    self.log_event("Disconnected", "Normal disconnect");
                }
            }
            CameraUpdate::PropertiesLoaded => {
                self.properties.set_loaded(true);
                self.log_event("Properties", "Loaded from camera");

                // Derive format info from property store now that properties are loaded
                // FileType shows RAW/JPEG/RAW+JPEG, ImageQuality shows JPEG compression level
                if let Some(file_type) = self
                    .properties
                    .get(DevicePropertyCode::StillImageStoreDestination)
                {
                    let ft = file_type.current_value();
                    // Append JPEG quality if relevant (contains JPEG)
                    if ft.contains("JPEG") {
                        if let Some(quality) =
                            self.properties.get(DevicePropertyCode::StillImageQuality)
                        {
                            self.dashboard.camera_info.image_format =
                                format!("{} {}", ft, quality.current_value());
                        } else {
                            self.dashboard.camera_info.image_format = ft.to_string();
                        }
                    } else {
                        self.dashboard.camera_info.image_format = ft.to_string();
                    }
                }
                if let Some(prop) = self.properties.get(DevicePropertyCode::MovieFileFormat) {
                    let format = prop.current_value().to_string();
                    if let Some(quality) = self
                        .properties
                        .get(DevicePropertyCode::MovieRecordingSetting)
                    {
                        self.dashboard.camera_info.recording_format =
                            format!("{} {}", format, quality.current_value());
                    } else {
                        self.dashboard.camera_info.recording_format = format;
                    }
                }
            }
            CameraUpdate::PropertyChanged {
                code,
                value,
                raw_value,
                available,
                writable,
                kind,
            } => {
                // Clear in-flight state if this property was waiting for confirmation
                if let Some((in_flight_code, _)) = self.in_flight_property {
                    if in_flight_code == code {
                        tracing::debug!("Clearing in_flight_property for {:?}", code);
                        self.in_flight_property = None;
                    }
                }
                self.properties
                    .update_property(code, &value, raw_value, available, writable, kind);
            }
            CameraUpdate::Error { message } => {
                self.log_event("Error", &message);
                self.modal = Some(Modal::Error { message });
            }
            CameraUpdate::DiscoveryResult { cameras } => {
                self.discovery.cameras = cameras.into_iter().map(DiscoveredCamera::from).collect();
                self.discovery.is_scanning = false;
            }
            CameraUpdate::DiscoveryStarted => {
                self.discovery.is_scanning = true;
                self.discovery.cameras.clear();
            }
            CameraUpdate::SdkEvent {
                event_type,
                details,
            } => {
                self.log_event(&event_type, &details);
            }
            CameraUpdate::CaptureComplete { success } => {
                if success {
                    self.log_event("Capture", "Photo captured");
                }
            }
            CameraUpdate::RecordingStateChanged { is_recording } => {
                self.dashboard.is_recording = is_recording;
                if is_recording {
                    self.dashboard.recording_seconds = 0;
                }
            }
            CameraUpdate::SshFingerprintFetched {
                fingerprint,
                ip,
                mac,
                ssh_user,
                ssh_pass,
            } => {
                if self.trust_ssh_fingerprint {
                    tracing::info!("Auto-trusting SSH fingerprint (--trust flag)");
                    let _ = self.camera_service.cmd_tx.try_send(
                        CameraCommand::ConnectWithFingerprint {
                            ip,
                            mac,
                            ssh_user,
                            ssh_pass,
                            fingerprint,
                        },
                    );
                } else {
                    self.modal = Some(Modal::SshFingerprintConfirm(SshFingerprintState {
                        fingerprint,
                        ip,
                        mac,
                        ssh_user,
                        ssh_pass,
                    }));
                }
            }
            CameraUpdate::CameraInfoUpdate {
                battery_percent,
                lens_model,
                focal_length_mm,
                overheating_state,
                slot1,
                slot2,
                slot3,
            } => {
                if let Some(battery) = battery_percent {
                    self.dashboard.camera_info.battery = battery;
                }

                if let Some(lens) = lens_model {
                    self.dashboard.camera_info.lens = lens;
                }

                // Update focal length (value is in 0.001mm units, so divide by 1000)
                if let Some(fl) = focal_length_mm {
                    let mm = fl / 1000;
                    self.dashboard.camera_info.focal_length = format!("{}mm", mm);
                }

                // Update temperature/overheating state
                self.dashboard.camera_info.temperature = overheating_state.and_then(|state| {
                    match state {
                        0 => None, // Normal - don't show
                        1 => Some("Pre-Overheat".to_string()),
                        2 => Some("OVERHEATING".to_string()),
                        _ => None,
                    }
                });

                // Update slots - None means camera doesn't have that slot
                self.dashboard.camera_info.slot1 = slot1.as_ref().map(slot_info_to_media_slot);
                self.dashboard.camera_info.slot2 = slot2.as_ref().map(slot_info_to_media_slot);
                self.dashboard.camera_info.slot3 = slot3.as_ref().map(slot_info_to_media_slot);
            }
        }
    }

    fn log_event(&mut self, event_type: &str, details: &str) {
        self.events_log.events.push_back(CameraEvent {
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            event_type: event_type.to_string(),
            details: details.to_string(),
        });
        while self.events_log.events.len() > MAX_EVENT_LOG_SIZE {
            self.events_log.events.pop_front();
        }
    }

    pub fn selected_pinned_property_id(&self) -> Option<DevicePropertyCode> {
        self.properties
            .pinned_ids()
            .get(self.dashboard.selected_property)
            .copied()
    }

    pub async fn update(&mut self, action: Action) {
        if self.help_visible {
            self.help_visible = false;
            return;
        }

        if let Some(modal) = self.modal.clone() {
            self.handle_modal_action(action, modal).await;
            return;
        }

        match action {
            Action::Quit => self.should_quit = true,
            Action::ShowHelp => self.help_visible = true,
            Action::HideHelp => self.help_visible = false,
            Action::Tick => self.handle_tick(),
            Action::FlushPendingProperty => self.flush_pending_property().await,
            Action::Back => self.handle_back().await,
            _ => self.handle_screen_action(action).await,
        }
    }

    fn handle_tick(&mut self) {
        if self.connected_camera.is_some() {
            self.dashboard.session_seconds += 1;
            if self.dashboard.is_recording {
                self.dashboard.recording_seconds += 1;
            }
        }
    }

    async fn handle_back(&mut self) {
        match self.screen {
            Screen::Discovery => {}
            Screen::Dashboard => {
                let _ = self.camera_service.send(CameraCommand::Disconnect).await;
                self.screen = Screen::Discovery;
                self.connected_camera = None;
            }
            Screen::PropertyEditor => {
                if self.property_editor.focus == PropertyEditorFocus::Values {
                    self.property_editor.focus = PropertyEditorFocus::Properties;
                } else {
                    self.screen = Screen::Dashboard;
                }
            }
            Screen::EventsExpanded => {
                self.screen = Screen::Dashboard;
            }
        }
    }

    async fn handle_screen_action(&mut self, action: Action) {
        match self.screen {
            Screen::Discovery => self.handle_discovery_action(action).await,
            Screen::Dashboard => self.handle_dashboard_action(action).await,
            Screen::PropertyEditor => self.handle_property_editor_action(action).await,
            Screen::EventsExpanded => self.handle_events_action(action),
        }
    }

    async fn handle_discovery_action(&mut self, action: Action) {
        match action {
            Action::SelectNextCamera => {
                if !self.discovery.cameras.is_empty() {
                    self.discovery.selected_index =
                        (self.discovery.selected_index + 1) % self.discovery.cameras.len();
                }
            }
            Action::SelectPrevCamera => {
                if !self.discovery.cameras.is_empty() {
                    self.discovery.selected_index = self
                        .discovery
                        .selected_index
                        .checked_sub(1)
                        .unwrap_or(self.discovery.cameras.len() - 1);
                }
            }
            Action::ConnectToSelected => {
                if let Some(camera) = self.discovery.cameras.get(self.discovery.selected_index) {
                    if camera.ssh_supported {
                        self.modal = Some(Modal::SshCredentials(SshCredentialsState {
                            camera_name: camera.model.clone(),
                            camera_address: camera.address.clone(),
                            camera_mac: camera.mac_address.clone(),
                            ..Default::default()
                        }));
                    } else {
                        self.connect_to_camera(camera.clone()).await;
                    }
                }
            }
            Action::StartScan => {
                let _ = self.camera_service.send(CameraCommand::Discover).await;
            }
            Action::ShowManualConnect => {
                self.modal = Some(Modal::ManualConnection(ManualConnectionState::default()));
            }
            _ => {}
        }
    }

    async fn handle_dashboard_action(&mut self, action: Action) {
        match action {
            Action::SelectNextDashboardProperty => {
                let count = self.properties.pinned_ids().len();
                if count > 0 {
                    self.dashboard.selected_property =
                        (self.dashboard.selected_property + 1) % count;
                }
            }
            Action::SelectPrevDashboardProperty => {
                let count = self.properties.pinned_ids().len();
                if count > 0 {
                    self.dashboard.selected_property = self
                        .dashboard
                        .selected_property
                        .checked_sub(1)
                        .unwrap_or(count - 1);
                }
            }
            Action::AdjustPropertyUp => {
                if let Some(code) = self.selected_pinned_property_id() {
                    if !self.is_in_flight(code) {
                        if let Some(prop) = self.properties.get_mut(code) {
                            let new_index = prop.next();
                            self.queue_property_change(code, new_index);
                        }
                    }
                }
            }
            Action::AdjustPropertyDown => {
                if let Some(code) = self.selected_pinned_property_id() {
                    if !self.is_in_flight(code) {
                        if let Some(prop) = self.properties.get_mut(code) {
                            let new_index = prop.prev();
                            self.queue_property_change(code, new_index);
                        }
                    }
                }
            }
            Action::OpenPropertyInEditor => {
                if let Some(code) = self.selected_pinned_property_id() {
                    self.jump_to_property_in_editor(code);
                }
            }
            Action::Capture => {
                let _ = self.camera_service.send(CameraCommand::Capture).await;
            }
            Action::StartRecording => {
                let _ = self
                    .camera_service
                    .send(CameraCommand::StartRecording)
                    .await;
            }
            Action::StopRecording => {
                let _ = self.camera_service.send(CameraCommand::StopRecording).await;
            }
            Action::ShowPropertyEditor => {
                self.screen = Screen::PropertyEditor;
            }
            Action::ShowEventsExpanded => {
                self.screen = Screen::EventsExpanded;
            }
            Action::ShowPropertySearch => {
                let results = super::property::search_properties(&self.properties, "");
                self.modal = Some(Modal::PropertySearch(PropertySearchState {
                    query: String::new(),
                    results,
                    selected_index: 0,
                }));
            }
            Action::Disconnect => {
                let _ = self.camera_service.send(CameraCommand::Disconnect).await;
            }
            Action::HalfPressShutter => {
                let _ = self
                    .camera_service
                    .send(CameraCommand::HalfPressShutter)
                    .await;
            }
            _ => {}
        }
    }

    async fn handle_property_editor_action(&mut self, action: Action) {
        let available_categories = self.properties.available_categories();
        let category_count = available_categories.len().max(1);

        match action {
            Action::PropertyEditorNext => match self.property_editor.focus {
                PropertyEditorFocus::Categories => {
                    self.property_editor.category_index =
                        (self.property_editor.category_index + 1) % category_count;
                    self.property_editor.property_index = 0;
                }
                PropertyEditorFocus::Properties => {
                    let props = self.current_category_properties();
                    if !props.is_empty() {
                        self.property_editor.property_index =
                            (self.property_editor.property_index + 1) % props.len();
                    }
                }
                PropertyEditorFocus::Values => {
                    if let Some(code) = self.selected_property_id_in_editor() {
                        if let Some(prop) = self.properties.get(code) {
                            if !prop.values.is_empty() {
                                self.property_editor.value_preview_index =
                                    (self.property_editor.value_preview_index + 1)
                                        % prop.values.len();
                            }
                        }
                    }
                }
            },
            Action::PropertyEditorPrev => match self.property_editor.focus {
                PropertyEditorFocus::Categories => {
                    self.property_editor.category_index = self
                        .property_editor
                        .category_index
                        .checked_sub(1)
                        .unwrap_or(category_count - 1);
                    self.property_editor.property_index = 0;
                }
                PropertyEditorFocus::Properties => {
                    let props = self.current_category_properties();
                    if !props.is_empty() {
                        self.property_editor.property_index = self
                            .property_editor
                            .property_index
                            .checked_sub(1)
                            .unwrap_or(props.len() - 1);
                    }
                }
                PropertyEditorFocus::Values => {
                    if let Some(code) = self.selected_property_id_in_editor() {
                        if let Some(prop) = self.properties.get(code) {
                            if !prop.values.is_empty() {
                                self.property_editor.value_preview_index = self
                                    .property_editor
                                    .value_preview_index
                                    .checked_sub(1)
                                    .unwrap_or(prop.values.len() - 1);
                            }
                        }
                    }
                }
            },
            Action::PropertyEditorNextCategory => {
                self.change_property_category(1);
            }
            Action::PropertyEditorPrevCategory => {
                self.change_property_category(-1);
            }
            Action::PropertyEditorValueNext => {
                self.adjust_property_value(1);
            }
            Action::PropertyEditorValuePrev => {
                self.adjust_property_value(-1);
            }
            Action::PropertyEditorValueNextFast => {
                self.adjust_property_value(10);
            }
            Action::PropertyEditorValuePrevFast => {
                self.adjust_property_value(-10);
            }
            Action::PropertyEditorValueToMin => {
                self.set_property_to_extreme(false);
            }
            Action::PropertyEditorValueToMax => {
                self.set_property_to_extreme(true);
            }
            Action::PropertyEditorEditValue => {
                self.open_range_input_modal();
            }
            Action::PropertyEditorOpenValues => {
                if self.property_editor.focus == PropertyEditorFocus::Properties {
                    if let Some(code) = self.selected_property_id_in_editor() {
                        if let Some(prop) = self.properties.get(code) {
                            if prop.writable && !prop.values.is_empty() {
                                self.property_editor.value_preview_index = prop.current_index;
                                self.property_editor.focus = PropertyEditorFocus::Values;
                            }
                        }
                    }
                }
            }
            Action::PropertyEditorApplyValue => {
                if self.property_editor.focus == PropertyEditorFocus::Values {
                    if let Some(code) = self.selected_property_id_in_editor() {
                        if !self.is_in_flight(code) {
                            let new_index = self.property_editor.value_preview_index;
                            if let Some(prop) = self.properties.get_mut(code) {
                                prop.current_index = new_index;
                                self.queue_property_change(code, new_index);
                            }
                        }
                    }
                    self.property_editor.focus = PropertyEditorFocus::Properties;
                }
            }
            Action::TogglePropertyPin => {
                if self.property_editor.focus == PropertyEditorFocus::Properties {
                    if let Some(code) = self.selected_property_id_in_editor() {
                        self.properties.toggle_pin(code);
                    }
                }
            }
            Action::TogglePropertyInfo => {
                self.property_editor.show_info = !self.property_editor.show_info;
            }
            Action::ShowPropertySearch => {
                let results = super::property::search_properties(&self.properties, "");
                self.modal = Some(Modal::PropertySearch(PropertySearchState {
                    query: String::new(),
                    results,
                    selected_index: 0,
                }));
            }
            _ => {}
        }
    }

    fn change_property_category(&mut self, delta: isize) {
        let categories = self.properties.available_categories();
        let len = categories.len().max(1);
        self.property_editor.category_index = if delta > 0 {
            (self.property_editor.category_index + 1) % len
        } else {
            self.property_editor
                .category_index
                .checked_sub(1)
                .unwrap_or(len - 1)
        };
        self.property_editor.property_index = 0;
        self.property_editor.focus = PropertyEditorFocus::Properties;
    }

    fn current_category_properties(&self) -> Vec<DevicePropertyCode> {
        let categories = self.properties.available_categories();
        let category = self.property_editor.current_category(&categories);
        self.properties
            .properties_by_category(category)
            .iter()
            .map(|p| p.code)
            .collect()
    }

    fn selected_property_id_in_editor(&self) -> Option<DevicePropertyCode> {
        let props = self.current_category_properties();
        props.get(self.property_editor.property_index).copied()
    }

    fn jump_to_property_in_editor(&mut self, code: DevicePropertyCode) {
        let category = property_category(code);
        let categories = self.properties.available_categories();
        if let Some(cat_idx) = categories.iter().position(|&c| c == category) {
            self.property_editor.category_index = cat_idx;
        }

        let props = self.properties.properties_by_category(category);
        if let Some(prop_idx) = props.iter().position(|p| p.code == code) {
            self.property_editor.property_index = prop_idx;
        }

        self.property_editor.focus = PropertyEditorFocus::Properties;
        self.screen = Screen::PropertyEditor;
    }

    fn adjust_property_value(&mut self, steps: i64) {
        if self.property_editor.focus != PropertyEditorFocus::Properties {
            return;
        }
        let Some(code) = self.selected_property_id_in_editor() else {
            return;
        };
        if self.is_in_flight(code) {
            return;
        }
        let Some(prop) = self.properties.get_mut(code) else {
            return;
        };

        let new_index = prop.advance(steps);
        self.queue_property_change(code, new_index);
    }

    fn set_property_to_extreme(&mut self, to_max: bool) {
        if self.property_editor.focus != PropertyEditorFocus::Properties {
            return;
        }
        let Some(code) = self.selected_property_id_in_editor() else {
            return;
        };
        if self.is_in_flight(code) {
            return;
        }
        let Some(prop) = self.properties.get_mut(code) else {
            return;
        };

        let new_index = if to_max {
            prop.set_index(prop.value_count().saturating_sub(1))
        } else {
            prop.set_index(0)
        };
        self.queue_property_change(code, new_index);
    }

    fn open_range_input_modal(&mut self) {
        if self.property_editor.focus != PropertyEditorFocus::Properties {
            return;
        }
        let Some(code) = self.selected_property_id_in_editor() else {
            return;
        };
        let Some(prop) = self.properties.get(code) else {
            return;
        };

        if let Some((min, max, step)) = prop.range_params() {
            self.modal = Some(Modal::RangeValueInput(RangeValueInputState {
                property_code: code,
                property_name: property_display_name(code).to_string(),
                input: prop.current_raw.to_string(),
                min,
                max,
                step,
                error: None,
            }));
        }
    }

    fn apply_range_input_value(&mut self, state: RangeValueInputState) {
        let value: i64 = match state.input.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                if let Some(Modal::RangeValueInput(ref mut modal_state)) = self.modal {
                    modal_state.error = Some("Invalid number".to_string());
                }
                return;
            }
        };

        if value < state.min || value > state.max {
            if let Some(Modal::RangeValueInput(ref mut modal_state)) = self.modal {
                modal_state.error = Some(format!("Must be {} to {}", state.min, state.max));
            }
            return;
        }

        let step = if state.step == 0 { 1 } else { state.step };
        if (value - state.min) % step != 0 {
            if let Some(Modal::RangeValueInput(ref mut modal_state)) = self.modal {
                modal_state.error =
                    Some(format!("Must be a multiple of {} from {}", step, state.min));
            }
            return;
        }

        let new_index = ((value - state.min) / step) as usize;
        let code = state.property_code;

        if let Some(prop) = self.properties.get_mut(code) {
            prop.set_index(new_index);
            self.queue_property_change(code, new_index);
        }

        self.modal = None;
    }

    fn handle_events_action(&mut self, action: Action) {
        match action {
            Action::ScrollEventsUp => {
                self.events_log.scroll_offset = self.events_log.scroll_offset.saturating_sub(1);
            }
            Action::ScrollEventsDown => {
                let max_offset = self.events_log.events.len().saturating_sub(1);
                self.events_log.scroll_offset = (self.events_log.scroll_offset + 1).min(max_offset);
            }
            Action::ScrollEventsToTop => {
                self.events_log.scroll_offset = 0;
            }
            Action::ScrollEventsToBottom => {
                self.events_log.scroll_offset = self.events_log.events.len().saturating_sub(1);
            }
            Action::ClearEvents => {
                self.events_log.events.clear();
                self.events_log.scroll_offset = 0;
            }
            _ => {}
        }
    }

    async fn handle_modal_action(&mut self, action: Action, modal: Modal) {
        match action {
            Action::ModalClose => {
                self.modal = None;
            }
            Action::ModalConfirm => match modal {
                Modal::SshCredentials(state) => {
                    self.modal = None;
                    self.fetch_ssh_fingerprint(
                        &state.camera_address,
                        &state.camera_mac,
                        state.username,
                        state.password,
                    )
                    .await;
                }
                Modal::SshFingerprintConfirm(state) => {
                    self.modal = None;
                    self.dashboard.session_seconds = 0;
                    let _ = self
                        .camera_service
                        .send(CameraCommand::ConnectWithFingerprint {
                            ip: state.ip,
                            mac: state.mac,
                            ssh_user: state.ssh_user,
                            ssh_pass: state.ssh_pass,
                            fingerprint: state.fingerprint,
                        })
                        .await;
                }
                Modal::ManualConnection(state) => {
                    let camera = DiscoveredCamera {
                        model: "Manual".to_string(),
                        address: state.ip_address,
                        mac_address: state.mac_address,
                        connection_type: "Network".to_string(),
                        ssh_supported: state.ssh_enabled,
                    };
                    self.modal = None;
                    self.connect_to_camera(camera).await;
                }
                Modal::PropertySearch(state) => {
                    if let Some(&id) = state.results.get(state.selected_index) {
                        self.modal = None;
                        self.jump_to_property_in_editor(id);
                    }
                }
                Modal::RangeValueInput(state) => {
                    self.apply_range_input_value(state);
                }
                _ => {
                    self.modal = None;
                }
            },
            Action::ModalNextField => {
                if let Some(Modal::SshCredentials(ref mut state)) = self.modal {
                    state.focused_field = (state.focused_field + 1) % 3;
                } else if let Some(Modal::ManualConnection(ref mut state)) = self.modal {
                    state.focused_field = (state.focused_field + 1) % 4;
                }
            }
            Action::ModalPrevField => {
                if let Some(Modal::SshCredentials(ref mut state)) = self.modal {
                    state.focused_field = state.focused_field.checked_sub(1).unwrap_or(2);
                } else if let Some(Modal::ManualConnection(ref mut state)) = self.modal {
                    state.focused_field = state.focused_field.checked_sub(1).unwrap_or(3);
                }
            }
            Action::ModalToggleCheckbox => {
                if let Some(Modal::SshCredentials(ref mut state)) = self.modal {
                    state.remember = !state.remember;
                } else if let Some(Modal::ManualConnection(ref mut state)) = self.modal {
                    state.ssh_enabled = !state.ssh_enabled;
                }
            }
            Action::ModalInputChar(c) => {
                self.modal_input_char(c);
            }
            Action::ModalInputBackspace => {
                self.modal_input_backspace();
            }
            Action::ModalInputLeft => {
                if let Some(Modal::ManualConnection(ref mut state)) = self.modal {
                    if state.focused_field == 2 {
                        state.model_index = state
                            .model_index
                            .checked_sub(1)
                            .unwrap_or(CameraModel::ALL.len() - 1);
                    }
                }
            }
            Action::ModalInputRight => {
                if let Some(Modal::ManualConnection(ref mut state)) = self.modal {
                    if state.focused_field == 2 {
                        state.model_index = (state.model_index + 1) % CameraModel::ALL.len();
                    }
                }
            }
            Action::ModalSelectNext => {
                if let Some(Modal::PropertySearch(ref mut state)) = self.modal {
                    if !state.results.is_empty() {
                        state.selected_index = (state.selected_index + 1) % state.results.len();
                    }
                }
            }
            Action::ModalSelectPrev => {
                if let Some(Modal::PropertySearch(ref mut state)) = self.modal {
                    if !state.results.is_empty() {
                        state.selected_index = state
                            .selected_index
                            .checked_sub(1)
                            .unwrap_or(state.results.len() - 1);
                    }
                }
            }
            _ => {}
        }
    }

    fn with_focused_modal_field<F>(&mut self, f: F)
    where
        F: FnOnce(&mut String),
    {
        match &mut self.modal {
            Some(Modal::SshCredentials(state)) => match state.focused_field {
                0 => f(&mut state.username),
                1 => f(&mut state.password),
                _ => {}
            },
            Some(Modal::ManualConnection(state)) => match state.focused_field {
                0 => f(&mut state.ip_address),
                1 => f(&mut state.mac_address),
                _ => {}
            },
            _ => {}
        }
    }

    fn modal_input_char(&mut self, c: char) {
        if let Some(Modal::PropertySearch(ref mut state)) = self.modal {
            state.query.push(c);
            state.results = super::property::search_properties(&self.properties, &state.query);
            state.selected_index = 0;
        } else if let Some(Modal::RangeValueInput(ref mut state)) = self.modal {
            if c.is_ascii_digit() || (c == '-' && state.input.is_empty()) {
                state.input.push(c);
                state.error = None;
            }
        } else {
            self.with_focused_modal_field(|text| text.push(c));
        }
    }

    fn modal_input_backspace(&mut self) {
        if let Some(Modal::PropertySearch(ref mut state)) = self.modal {
            state.query.pop();
            state.results = super::property::search_properties(&self.properties, &state.query);
            state.selected_index = 0;
        } else if let Some(Modal::RangeValueInput(ref mut state)) = self.modal {
            state.input.pop();
            state.error = None;
        } else {
            self.with_focused_modal_field(|text| {
                text.pop();
            });
        }
    }

    async fn connect_to_camera(&mut self, camera: DiscoveredCamera) {
        let ip = match Ipv4Addr::from_str(&camera.address) {
            Ok(ip) => ip,
            Err(_) => {
                self.modal = Some(Modal::Error {
                    message: format!("Invalid IP address: {}", camera.address),
                });
                return;
            }
        };

        let mac = match MacAddr::from_str(&camera.mac_address) {
            Ok(mac) => mac,
            Err(_) => {
                self.modal = Some(Modal::Error {
                    message: format!("Invalid MAC address: {}", camera.mac_address),
                });
                return;
            }
        };

        self.dashboard.session_seconds = 0;
        let _ = self
            .camera_service
            .send(CameraCommand::Connect { ip, mac })
            .await;
    }

    async fn fetch_ssh_fingerprint(
        &mut self,
        address: &str,
        mac_address: &str,
        username: String,
        password: String,
    ) {
        let ip = match Ipv4Addr::from_str(address) {
            Ok(ip) => ip,
            Err(_) => {
                self.modal = Some(Modal::Error {
                    message: format!("Invalid IP address: {}", address),
                });
                return;
            }
        };

        let mac = match MacAddr::from_str(mac_address) {
            Ok(mac) => mac,
            Err(_) => {
                self.modal = Some(Modal::Error {
                    message: format!("Invalid MAC address: {}", mac_address),
                });
                return;
            }
        };

        self.log_event("SSH", "Fetching fingerprint...");
        let _ = self
            .camera_service
            .send(CameraCommand::FetchSshFingerprint {
                ip,
                mac,
                ssh_user: username,
                ssh_pass: password,
            })
            .await;
    }
}

fn slot_info_to_media_slot(info: &SlotInfo) -> MediaSlotInfo {
    match info.status {
        MediaSlotStatus::Ok => MediaSlotInfo {
            media_type: String::new(),
            free_space: format_remaining_info(info.remaining_photos, info.remaining_time_sec),
        },
        MediaSlotStatus::NoCard => MediaSlotInfo {
            media_type: String::new(),
            free_space: "No card".to_string(),
        },
        MediaSlotStatus::Error => MediaSlotInfo {
            media_type: "Error".to_string(),
            free_space: "Card Error".to_string(),
        },
    }
}

fn format_remaining_info(photos: Option<u32>, time_sec: Option<u32>) -> String {
    let time_str = time_sec.and_then(|t| {
        if t >= 3600 {
            let hours = t / 3600;
            let mins = (t % 3600) / 60;
            Some(format!("{}h{}m left", hours, mins))
        } else if t > 0 {
            Some(format!("{}m left", t / 60))
        } else {
            None
        }
    });

    match (photos, time_str) {
        (Some(p), Some(t)) if p > 0 => format!("{} photos / {}", p, t),
        (Some(p), _) if p > 0 => format!("{} photos", p),
        (_, Some(t)) => t,
        _ => "Ready".to_string(),
    }
}
