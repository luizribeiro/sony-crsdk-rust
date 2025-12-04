use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::{
    ManualConnectionState, Modal, PropertySearchState, SshCredentialsState, SshFingerprintState,
};
use crsdk::CameraModel;

pub fn render(frame: &mut Frame, modal: &Modal) {
    match modal {
        Modal::SshCredentials(state) => render_ssh_modal(frame, state),
        Modal::SshFingerprintConfirm(state) => render_fingerprint_modal(frame, state),
        Modal::ManualConnection(state) => render_manual_modal(frame, state),
        Modal::PropertySearch(state) => render_property_search_modal(frame, state),
        Modal::Confirmation { message } => render_confirmation_modal(frame, message),
        Modal::Error { message } => render_error_modal(frame, message),
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .split(area);

    Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .split(vertical[0])[0]
}

fn render_modal_frame(
    frame: &mut Frame,
    width: u16,
    height: u16,
    title: &str,
    color: Color,
) -> Rect {
    let area = centered_rect(width, height, frame.area());
    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color));

    frame.render_widget(block.clone(), area);
    block.inner(area)
}

fn render_ssh_modal(frame: &mut Frame, state: &SshCredentialsState) {
    let inner = render_modal_frame(frame, 50, 11, " SSH Authentication ", Color::Cyan);
    let layout = Layout::vertical([
        Constraint::Length(2), // Camera info
        Constraint::Length(2), // Username
        Constraint::Length(2), // Password
        Constraint::Length(2), // Remember checkbox
        Constraint::Length(2), // Buttons
    ])
    .split(inner);

    // Camera info
    let camera_info = Line::from(vec![
        Span::styled("  Camera: ", Style::default().fg(Color::DarkGray)),
        Span::styled(&state.camera_name, Style::default().fg(Color::White)),
        Span::styled(" (", Style::default().fg(Color::DarkGray)),
        Span::styled(&state.camera_address, Style::default().fg(Color::DarkGray)),
        Span::styled(")", Style::default().fg(Color::DarkGray)),
    ]);
    frame.render_widget(Paragraph::new(camera_info), layout[0]);

    // Username field
    render_input_field(
        frame,
        layout[1],
        "Username",
        &state.username,
        state.focused_field == 0,
        false,
    );

    // Password field
    render_input_field(
        frame,
        layout[2],
        "Password",
        &state.password,
        state.focused_field == 1,
        true,
    );

    // Remember checkbox
    let checkbox = if state.remember { "◉" } else { "○" };
    let checkbox_style = if state.focused_field == 2 {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let remember_line = Line::from(vec![
        Span::raw("  "),
        Span::styled(checkbox, checkbox_style),
        Span::styled(
            " Remember for this session",
            Style::default().fg(Color::DarkGray),
        ),
    ]);
    frame.render_widget(Paragraph::new(remember_line), layout[3]);

    // Buttons
    let buttons = Line::from(vec![
        Span::raw("  "),
        Span::styled("Enter", Style::default().fg(Color::Cyan)),
        Span::styled(" Connect    ", Style::default().fg(Color::DarkGray)),
        Span::styled("Esc", Style::default().fg(Color::Cyan)),
        Span::styled(" Cancel", Style::default().fg(Color::DarkGray)),
    ]);
    frame.render_widget(Paragraph::new(buttons), layout[4]);
}

fn render_fingerprint_modal(frame: &mut Frame, state: &SshFingerprintState) {
    let inner = render_modal_frame(
        frame,
        60,
        12,
        " SSH Fingerprint Verification ",
        Color::Yellow,
    );
    let layout = Layout::vertical([
        Constraint::Length(2), // Camera info
        Constraint::Length(1), // Spacer
        Constraint::Length(1), // Label
        Constraint::Min(4),    // Fingerprint
        Constraint::Length(1), // Spacer
        Constraint::Length(2), // Buttons
    ])
    .split(inner);

    // Camera info
    let camera_info = Line::from(vec![
        Span::styled("  Camera: ", Style::default().fg(Color::DarkGray)),
        Span::styled(state.ip.to_string(), Style::default().fg(Color::White)),
    ]);
    frame.render_widget(Paragraph::new(camera_info), layout[0]);

    // Label
    let label = Line::from(vec![Span::styled(
        "  Do you trust this SSH fingerprint?",
        Style::default().fg(Color::Yellow),
    )]);
    frame.render_widget(Paragraph::new(label), layout[2]);

    // Fingerprint (with word wrap)
    let fingerprint_text = format!("  {}", state.fingerprint);
    let fingerprint_paragraph = Paragraph::new(fingerprint_text)
        .style(Style::default().fg(Color::Cyan))
        .wrap(Wrap { trim: false });
    frame.render_widget(fingerprint_paragraph, layout[3]);

    // Buttons
    let buttons = Line::from(vec![
        Span::raw("  "),
        Span::styled("Enter", Style::default().fg(Color::Green)),
        Span::styled(" Trust & Connect    ", Style::default().fg(Color::DarkGray)),
        Span::styled("Esc", Style::default().fg(Color::Red)),
        Span::styled(" Cancel", Style::default().fg(Color::DarkGray)),
    ]);
    frame.render_widget(Paragraph::new(buttons), layout[5]);
}

fn render_manual_modal(frame: &mut Frame, state: &ManualConnectionState) {
    let inner = render_modal_frame(frame, 50, 12, " Manual Connection ", Color::Cyan);
    let layout = Layout::vertical([
        Constraint::Length(2), // IP Address
        Constraint::Length(2), // MAC Address
        Constraint::Length(2), // Model
        Constraint::Length(2), // SSH checkbox
        Constraint::Length(2), // Buttons
    ])
    .split(inner);

    // IP Address field
    render_input_field(
        frame,
        layout[0],
        "IP Address",
        &state.ip_address,
        state.focused_field == 0,
        false,
    );

    // MAC Address field
    render_input_field(
        frame,
        layout[1],
        "MAC Address",
        &state.mac_address,
        state.focused_field == 1,
        false,
    );

    // Model selection
    let selected_model = CameraModel::ALL
        .get(state.model_index)
        .unwrap_or(&CameraModel::Fx3);
    let model_style = if state.focused_field == 2 {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::White)
    };
    let model_line = Line::from(vec![
        Span::styled("  Model      ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "◀ ",
            Style::default().fg(if state.focused_field == 2 {
                Color::Cyan
            } else {
                Color::DarkGray
            }),
        ),
        Span::styled(selected_model.to_string(), model_style),
        Span::styled(
            " ▶",
            Style::default().fg(if state.focused_field == 2 {
                Color::Cyan
            } else {
                Color::DarkGray
            }),
        ),
    ]);
    frame.render_widget(Paragraph::new(model_line), layout[2]);

    // SSH checkbox
    let checkbox = if state.ssh_enabled { "◉" } else { "○" };
    let checkbox_style = if state.focused_field == 3 {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let ssh_line = Line::from(vec![
        Span::raw("  "),
        Span::styled(checkbox, checkbox_style),
        Span::styled(" Enable SSH", Style::default().fg(Color::DarkGray)),
    ]);
    frame.render_widget(Paragraph::new(ssh_line), layout[3]);

    // Buttons
    let buttons = Line::from(vec![
        Span::raw("  "),
        Span::styled("Enter", Style::default().fg(Color::Cyan)),
        Span::styled(" Connect    ", Style::default().fg(Color::DarkGray)),
        Span::styled("Esc", Style::default().fg(Color::Cyan)),
        Span::styled(" Cancel", Style::default().fg(Color::DarkGray)),
    ]);
    frame.render_widget(Paragraph::new(buttons), layout[4]);
}

fn render_confirmation_modal(frame: &mut Frame, message: &str) {
    let inner = render_modal_frame(frame, 40, 7, " Confirm ", Color::Yellow);
    let layout = Layout::vertical([
        Constraint::Min(2),    // Message
        Constraint::Length(2), // Buttons
    ])
    .split(inner);

    let message_paragraph = Paragraph::new(format!("\n  {}", message));
    frame.render_widget(message_paragraph, layout[0]);

    let buttons = Line::from(vec![
        Span::raw("  "),
        Span::styled("Enter", Style::default().fg(Color::Green)),
        Span::styled(" Confirm    ", Style::default().fg(Color::DarkGray)),
        Span::styled("Esc", Style::default().fg(Color::Red)),
        Span::styled(" Cancel", Style::default().fg(Color::DarkGray)),
    ]);
    frame.render_widget(Paragraph::new(buttons), layout[1]);
}

fn render_error_modal(frame: &mut Frame, message: &str) {
    let inner = render_modal_frame(frame, 50, 7, " Error ", Color::Red);
    let layout = Layout::vertical([
        Constraint::Min(2),    // Message
        Constraint::Length(2), // Button
    ])
    .split(inner);

    let message_paragraph =
        Paragraph::new(format!("\n  {}", message)).style(Style::default().fg(Color::Red));
    frame.render_widget(message_paragraph, layout[0]);

    let buttons = Line::from(vec![
        Span::raw("  "),
        Span::styled("Enter/Esc", Style::default().fg(Color::Cyan)),
        Span::styled(" Close", Style::default().fg(Color::DarkGray)),
    ]);
    frame.render_widget(Paragraph::new(buttons), layout[1]);
}

fn render_input_field(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    value: &str,
    focused: bool,
    masked: bool,
) {
    let display_value = if masked {
        "•".repeat(value.len())
    } else {
        value.to_string()
    };

    let cursor = if focused { "▎" } else { "" };

    let value_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::White)
    };

    let line = Line::from(vec![
        Span::styled(
            format!("  {:12}", label),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(display_value, value_style),
        Span::styled(cursor, Style::default().fg(Color::Cyan)),
    ]);

    frame.render_widget(Paragraph::new(line), area);
}

fn render_property_search_modal(frame: &mut Frame, state: &PropertySearchState) {
    let inner = render_modal_frame(frame, 60, 16, " Search Properties ", Color::Cyan);

    let layout = Layout::vertical([
        Constraint::Length(2), // Search input
        Constraint::Min(1),    // Results
        Constraint::Length(1), // Shortcuts
    ])
    .split(inner);

    // Search input
    let search_line = Line::from(vec![
        Span::styled("  / ", Style::default().fg(Color::DarkGray)),
        Span::styled(&state.query, Style::default().fg(Color::Cyan)),
        Span::styled("▎", Style::default().fg(Color::Cyan)),
    ]);
    frame.render_widget(Paragraph::new(search_line), layout[0]);

    // Results
    let results_area = layout[1];
    let visible_count = results_area.height as usize;

    if state.results.is_empty() {
        let no_results = Paragraph::new(Line::from(vec![Span::styled(
            "    No matching properties",
            Style::default().fg(Color::DarkGray),
        )]));
        frame.render_widget(no_results, results_area);
    } else {
        // Calculate scroll offset to keep selection visible
        let scroll_offset = if state.selected_index >= visible_count {
            state.selected_index - visible_count + 1
        } else {
            0
        };

        let visible_results: Vec<Line> = state
            .results
            .iter()
            .enumerate()
            .skip(scroll_offset)
            .take(visible_count)
            .map(|(i, &code)| {
                let is_selected = i == state.selected_index;
                let prefix = if is_selected { "▸ " } else { "  " };
                let style = if is_selected {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default().fg(Color::White)
                };

                // Truncate category to 14 chars max
                let category_str = code.category().to_string();
                let category_display = if category_str.len() > 14 {
                    format!("{}…", &category_str[..13])
                } else {
                    category_str
                };

                Line::from(vec![
                    Span::styled(format!("  {}", prefix), style),
                    Span::styled(
                        format!("{:14}", category_display),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(" │ ", Style::default().fg(Color::Rgb(60, 60, 60))),
                    Span::styled(code.name(), style),
                ])
            })
            .collect();

        let results_paragraph = Paragraph::new(visible_results);
        frame.render_widget(results_paragraph, results_area);

        // Show scroll indicator if needed
        if state.results.len() > visible_count {
            let indicator = format!(" {}/{} ", state.selected_index + 1, state.results.len());
            let indicator_area = Rect {
                x: results_area.x + results_area.width - indicator.len() as u16 - 1,
                y: results_area.y,
                width: indicator.len() as u16,
                height: 1,
            };
            frame.render_widget(
                Paragraph::new(indicator).style(Style::default().fg(Color::DarkGray)),
                indicator_area,
            );
        }
    }

    // Shortcuts
    let shortcuts = Line::from(vec![
        Span::styled("  ↑↓ ", Style::default().fg(Color::Cyan)),
        Span::styled("Select", Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled("Enter ", Style::default().fg(Color::Cyan)),
        Span::styled("Go", Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled("Esc ", Style::default().fg(Color::Cyan)),
        Span::styled("Cancel", Style::default().fg(Color::DarkGray)),
    ]);
    frame.render_widget(Paragraph::new(shortcuts), layout[2]);
}
