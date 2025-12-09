use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::tui::app::DiscoveryState;

pub fn render(frame: &mut Frame, state: &DiscoveryState) {
    let area = frame.area();

    let layout = Layout::vertical([
        Constraint::Length(1), // Title
        Constraint::Min(10),   // Camera list
        Constraint::Length(3), // Status/scanning
        Constraint::Length(1), // Shortcuts
    ])
    .split(area);

    render_title(frame, layout[0]);
    render_camera_list(frame, layout[1], state);
    render_status(frame, layout[2], state);
    render_shortcuts(frame, layout[3]);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new(Line::from(vec![
        Span::styled(
            " sonyctl ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("— Discovery", Style::default().fg(Color::DarkGray)),
    ]));
    frame.render_widget(title, area);
}

fn render_camera_list(frame: &mut Frame, area: Rect, state: &DiscoveryState) {
    let block = Block::default()
        .title(" Discovered Cameras ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    if state.cameras.is_empty() {
        let empty_message = if state.is_scanning {
            "Scanning for cameras..."
        } else {
            "No cameras found. Press [r] to rescan or [m] for manual connection."
        };

        let paragraph = Paragraph::new(format!("\n  {}", empty_message))
            .style(Style::default().fg(Color::DarkGray))
            .block(block);

        frame.render_widget(paragraph, area);
        return;
    }

    let items: Vec<ListItem> = state
        .cameras
        .iter()
        .map(|cam| {
            let ssh_indicator = if cam.ssh_supported {
                Span::styled(" SSH", Style::default().fg(Color::Green))
            } else {
                Span::raw("    ")
            };

            let conn_type_color = match cam.connection_type.as_str() {
                "USB" => Color::Magenta,
                _ => Color::Blue,
            };

            let line = Line::from(vec![
                Span::raw("  "),
                Span::styled(&cam.model, Style::default().fg(Color::White)),
                Span::raw("  "),
                Span::styled(
                    format!("{:18}", cam.address),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    format!("{:8}", cam.connection_type),
                    Style::default().fg(conn_type_color),
                ),
                ssh_indicator,
            ]);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(40, 80, 120))
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▸ ");

    let mut list_state = ListState::default().with_selected(Some(state.selected_index));
    frame.render_stateful_widget(list, area, &mut list_state);
}

fn render_status(frame: &mut Frame, area: Rect, state: &DiscoveryState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let status_text = if state.is_scanning {
        Line::from(vec![
            Span::styled(" ◐ ", Style::default().fg(Color::Yellow)),
            Span::raw("Scanning for cameras..."),
        ])
    } else {
        Line::from(vec![
            Span::styled(" ● ", Style::default().fg(Color::Green)),
            Span::raw(format!("{} camera(s) found", state.cameras.len())),
        ])
    };

    let paragraph = Paragraph::new(status_text).block(block);
    frame.render_widget(paragraph, area);
}

fn render_shortcuts(frame: &mut Frame, area: Rect) {
    let shortcuts = Line::from(vec![
        Span::styled(" Enter ", Style::default().fg(Color::Cyan)),
        Span::styled("Connect", Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled(" r ", Style::default().fg(Color::Cyan)),
        Span::styled("Rescan", Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled(" m ", Style::default().fg(Color::Cyan)),
        Span::styled("Manual", Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled(" ? ", Style::default().fg(Color::Cyan)),
        Span::styled("Help", Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled(" q ", Style::default().fg(Color::Cyan)),
        Span::styled("Quit", Style::default().fg(Color::DarkGray)),
    ]);

    frame.render_widget(Paragraph::new(shortcuts), area);
}
