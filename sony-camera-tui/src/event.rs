use std::time::Duration;

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers};
use futures::StreamExt;
use tokio::time::{interval, Interval};

use crate::action::Action;
use crate::app::{App, PropertyEditorFocus, Screen};

pub struct EventHandler {
    events: EventStream,
    tick_interval: Interval,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            events: EventStream::new(),
            tick_interval: interval(Duration::from_secs(1)),
        }
    }

    pub async fn next(&mut self, app: &App) -> Option<Action> {
        // Get debounce timeout if there's a pending property change
        let debounce_timeout = app.debounce_timeout();

        tokio::select! {
            biased;

            Some(Ok(event)) = self.events.next() => {
                Self::map_terminal_event(event, app)
            }

            // Fire debounce timeout if pending property exists and timeout elapsed
            _ = async {
                if let Some(timeout) = debounce_timeout {
                    tokio::time::sleep(timeout).await;
                } else {
                    std::future::pending::<()>().await;
                }
            } => {
                Some(Action::FlushPendingProperty)
            }

            _ = self.tick_interval.tick() => {
                Some(Action::Tick)
            }
        }
    }

    fn map_terminal_event(event: Event, app: &App) -> Option<Action> {
        match event {
            Event::Key(key) => Self::map_key_event(key, app),
            Event::Resize(_, _) => None,
            _ => None,
        }
    }

    fn map_key_event(key: KeyEvent, app: &App) -> Option<Action> {
        // Global shortcuts
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return Some(Action::Quit);
        }

        // Help is visible - any key closes it
        if app.help_visible {
            return Some(Action::HideHelp);
        }

        // Modal is open - handle modal keys
        if app.modal.is_some() {
            return Self::map_modal_key(key);
        }

        // Screen-specific shortcuts
        match app.screen {
            Screen::Discovery => Self::map_discovery_key(key),
            Screen::Dashboard => Self::map_dashboard_key(key),
            Screen::PropertyEditor => Self::map_property_editor_key(key, app.property_editor.focus),
            Screen::EventsExpanded => Self::map_events_key(key),
        }
    }

    fn map_modal_key(key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Esc => Some(Action::ModalClose),
            KeyCode::Enter => Some(Action::ModalConfirm),
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    Some(Action::ModalPrevField)
                } else {
                    Some(Action::ModalNextField)
                }
            }
            KeyCode::Char(' ') => Some(Action::ModalToggleCheckbox),
            KeyCode::Char(c) => Some(Action::ModalInputChar(c)),
            KeyCode::Backspace => Some(Action::ModalInputBackspace),
            KeyCode::Delete => Some(Action::ModalInputDelete),
            KeyCode::Left => Some(Action::ModalInputLeft),
            KeyCode::Right => Some(Action::ModalInputRight),
            KeyCode::Down => Some(Action::ModalSelectNext),
            KeyCode::Up => Some(Action::ModalSelectPrev),
            _ => None,
        }
    }

    fn map_discovery_key(key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('q') => Some(Action::Quit),
            KeyCode::Char('?') => Some(Action::ShowHelp),
            KeyCode::Char('j') | KeyCode::Down => Some(Action::SelectNextCamera),
            KeyCode::Char('k') | KeyCode::Up => Some(Action::SelectPrevCamera),
            KeyCode::Enter => Some(Action::ConnectToSelected),
            KeyCode::Char('r') => Some(Action::StartScan),
            KeyCode::Char('m') => Some(Action::ShowManualConnect),
            _ => None,
        }
    }

    fn map_dashboard_key(key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('q') => Some(Action::Quit),
            KeyCode::Char('?') => Some(Action::ShowHelp),
            // Property navigation
            KeyCode::Char('j') | KeyCode::Down => Some(Action::SelectNextDashboardProperty),
            KeyCode::Char('k') | KeyCode::Up => Some(Action::SelectPrevDashboardProperty),
            // Adjust property values
            KeyCode::Char('h') | KeyCode::Left => Some(Action::AdjustPropertyDown),
            KeyCode::Char('l') | KeyCode::Right => Some(Action::AdjustPropertyUp),
            // Open property in editor
            KeyCode::Char('o') | KeyCode::Enter => Some(Action::OpenPropertyInEditor),
            // Shooting
            KeyCode::Char(' ') => Some(Action::Capture),
            KeyCode::Char('f') => Some(Action::HalfPressShutter),
            KeyCode::Char('c') => Some(Action::Capture),
            KeyCode::Char('v') => Some(Action::StartRecording),
            KeyCode::Char('s') => Some(Action::StopRecording),
            // Navigation
            KeyCode::Char('p') => Some(Action::ShowPropertyEditor),
            KeyCode::Char('e') => Some(Action::ShowEventsExpanded),
            KeyCode::Char('/') => Some(Action::ShowPropertySearch),
            KeyCode::Char('d') | KeyCode::Esc => Some(Action::Disconnect),
            _ => None,
        }
    }

    fn map_property_editor_key(key: KeyEvent, focus: PropertyEditorFocus) -> Option<Action> {
        let shift = key.modifiers.contains(KeyModifiers::SHIFT);

        match key.code {
            KeyCode::Char('q') => Some(Action::Quit),
            KeyCode::Char('?') => Some(Action::ShowHelp),
            KeyCode::Esc => Some(Action::Back),
            KeyCode::Char('j') | KeyCode::Down => Some(Action::PropertyEditorNext),
            KeyCode::Char('k') | KeyCode::Up => Some(Action::PropertyEditorPrev),
            KeyCode::Tab => Some(Action::PropertyEditorNextCategory),
            KeyCode::BackTab => Some(Action::PropertyEditorPrevCategory),
            // Value navigation with shift for fast mode
            KeyCode::Char('h') => {
                if shift {
                    Some(Action::PropertyEditorValuePrevFast)
                } else {
                    Some(Action::PropertyEditorValuePrev)
                }
            }
            KeyCode::Char('H') => Some(Action::PropertyEditorValuePrevFast),
            KeyCode::Char('l') => {
                if shift {
                    Some(Action::PropertyEditorValueNextFast)
                } else {
                    Some(Action::PropertyEditorValueNext)
                }
            }
            KeyCode::Char('L') => Some(Action::PropertyEditorValueNextFast),
            KeyCode::Left => {
                if shift {
                    Some(Action::PropertyEditorValuePrevFast)
                } else {
                    Some(Action::PropertyEditorValuePrev)
                }
            }
            KeyCode::Right => {
                if shift {
                    Some(Action::PropertyEditorValueNextFast)
                } else {
                    Some(Action::PropertyEditorValueNext)
                }
            }
            // Jump to min/max
            KeyCode::Char('g') => Some(Action::PropertyEditorValueToMin),
            KeyCode::Char('G') => Some(Action::PropertyEditorValueToMax),
            KeyCode::Home => Some(Action::PropertyEditorValueToMin),
            KeyCode::End => Some(Action::PropertyEditorValueToMax),
            // Direct value input for ranges
            KeyCode::Char('e') => Some(Action::PropertyEditorEditValue),
            KeyCode::Char('*') => Some(Action::TogglePropertyPin),
            KeyCode::Char('i') => Some(Action::TogglePropertyInfo),
            KeyCode::Char('o') | KeyCode::Enter => match focus {
                PropertyEditorFocus::Properties => Some(Action::PropertyEditorOpenValues),
                PropertyEditorFocus::Values => Some(Action::PropertyEditorApplyValue),
                _ => None,
            },
            KeyCode::Char('/') => Some(Action::ShowPropertySearch),
            _ => None,
        }
    }

    fn map_events_key(key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('q') => Some(Action::Quit),
            KeyCode::Char('?') => Some(Action::ShowHelp),
            KeyCode::Esc => Some(Action::Back),
            KeyCode::Char('j') | KeyCode::Down => Some(Action::ScrollEventsDown),
            KeyCode::Char('k') | KeyCode::Up => Some(Action::ScrollEventsUp),
            KeyCode::Char('g') | KeyCode::Home => Some(Action::ScrollEventsToTop),
            KeyCode::Char('G') | KeyCode::End => Some(Action::ScrollEventsToBottom),
            KeyCode::Char('c') => Some(Action::ClearEvents),
            _ => None,
        }
    }
}
