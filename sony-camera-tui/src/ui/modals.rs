use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::{ManualConnectionState, Modal, SshCredentialsState, SshFingerprintState};
use crsdk::CameraModel;

pub fn render(frame: &mut Frame, modal: &Modal) {
    match modal {
        Modal::SshCredentials(state) => render_ssh_modal(frame, state),
        Modal::SshFingerprintConfirm(state) => render_fingerprint_modal(frame, state),
        Modal::ManualConnection(state) => render_manual_modal(frame, state),
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

fn render_ssh_modal(frame: &mut Frame, state: &SshCredentialsState) {
    let area = centered_rect(50, 11, frame.area());

    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" SSH Authentication ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    frame.render_widget(block.clone(), area);

    let inner = block.inner(area);
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
    let area = centered_rect(60, 12, frame.area());

    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" SSH Fingerprint Verification ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    frame.render_widget(block.clone(), area);

    let inner = block.inner(area);
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
    let area = centered_rect(50, 12, frame.area());

    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Manual Connection ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    frame.render_widget(block.clone(), area);

    let inner = block.inner(area);
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
    let area = centered_rect(40, 7, frame.area());

    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Confirm ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    frame.render_widget(block.clone(), area);

    let inner = block.inner(area);
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
    let area = centered_rect(50, 7, frame.area());

    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Error ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    frame.render_widget(block.clone(), area);

    let inner = block.inner(area);
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
