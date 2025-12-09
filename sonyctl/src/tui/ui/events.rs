use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::tui::app::{App, ConnectedCamera};

use super::header::{self, HeaderState};

pub fn render(frame: &mut Frame, app: &App, camera: &Option<ConnectedCamera>) {
    let area = frame.area();

    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(10),
        Constraint::Length(1),
    ])
    .split(area);

    let exposure_mode = app.properties.exposure_mode();
    let header_state = HeaderState {
        camera,
        exposure_mode: Some(exposure_mode),
        is_recording: app.dashboard.is_recording,
        recording_seconds: if app.dashboard.is_recording {
            Some(app.dashboard.recording_seconds)
        } else {
            None
        },
    };
    header::render(frame, layout[0], &header_state);
    render_events_list(frame, layout[1], app);
    render_shortcuts(frame, layout[2]);
}

fn render_events_list(frame: &mut Frame, area: Rect, app: &App) {
    let state = &app.events_log;

    let block = Block::default()
        .title(format!(" Events Log — {} events ", state.events.len()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 60)));

    if state.events.is_empty() {
        let paragraph = Paragraph::new("\n  No events recorded")
            .style(Style::default().fg(Color::DarkGray))
            .block(block);
        frame.render_widget(paragraph, area);
        return;
    }

    let items: Vec<ListItem> = state
        .events
        .iter()
        .enumerate()
        .map(|(i, event)| {
            let is_selected = i == state.scroll_offset;

            let type_color = match event.event_type.as_str() {
                "Error" => Color::Red,
                "Warning" => Color::Yellow,
                "Connected" => Color::Green,
                "Disconnected" => Color::Red,
                "Capture" => Color::Cyan,
                "PropertyChanged" => Color::Blue,
                _ => Color::DarkGray,
            };

            let prefix = if is_selected { "▸ " } else { "  " };
            let prefix_style = if is_selected {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![
                Span::styled(prefix, prefix_style),
                Span::styled(
                    format!("{} ", event.timestamp),
                    Style::default().fg(Color::Rgb(80, 80, 80)),
                ),
                Span::styled(
                    format!("{:18}", event.event_type),
                    Style::default().fg(type_color),
                ),
                Span::styled(&event.details, Style::default().fg(Color::DarkGray)),
            ]))
        })
        .collect();

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn render_shortcuts(frame: &mut Frame, area: Rect) {
    let shortcuts = Line::from(vec![
        Span::styled(" ↑↓ ", Style::default().fg(Color::Cyan)),
        Span::styled("Scroll", Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled(" g/G ", Style::default().fg(Color::Cyan)),
        Span::styled("Top/Bottom", Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled(" c ", Style::default().fg(Color::Cyan)),
        Span::styled("Clear", Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled(" Esc ", Style::default().fg(Color::Cyan)),
        Span::styled("Back", Style::default().fg(Color::DarkGray)),
    ]);

    frame.render_widget(Paragraph::new(shortcuts), area);
}
