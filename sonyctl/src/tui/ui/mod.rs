mod dashboard;
mod discovery;
mod events;
mod header;
mod help;
mod modals;
mod properties;

use ratatui::Frame;

use super::app::{App, Screen};

pub fn render(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::Discovery => discovery::render(frame, &app.discovery),
        Screen::Dashboard => dashboard::render(frame, app, &app.connected_camera),
        Screen::PropertyEditor => properties::render(frame, app, &app.connected_camera),
        Screen::EventsExpanded => events::render(frame, app, &app.connected_camera),
    }

    if let Some(ref modal) = app.modal {
        modals::render(frame, modal);
    }

    if app.help_visible {
        help::render(frame, app.screen);
    }
}
