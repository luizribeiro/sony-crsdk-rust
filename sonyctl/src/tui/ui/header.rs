use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::tui::app::ConnectedCamera;

pub struct HeaderState<'a> {
    pub camera: &'a Option<ConnectedCamera>,
    pub exposure_mode: Option<&'a str>,
    pub is_recording: bool,
    pub recording_seconds: Option<u64>,
    pub is_connecting: bool,
}

pub fn render(frame: &mut Frame, area: Rect, state: &HeaderState) {
    let line = if let Some(cam) = state.camera {
        let mut spans = vec![Span::styled(" ", Style::default())];

        if let Some(mode) = state.exposure_mode {
            let bg_color = match mode {
                "M" => Color::Green,
                "A" => Color::Cyan,
                "S" => Color::Yellow,
                "P" => Color::Magenta,
                "Auto" => Color::Blue,
                _ => Color::White,
            };
            spans.push(Span::styled(
                format!(" {} ", mode),
                Style::default()
                    .fg(Color::Rgb(0, 0, 0))
                    .bg(bg_color)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::raw(" "));
        }

        spans.push(Span::styled(
            &cam.model,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            format!("  {}  ", cam.address),
            Style::default().fg(Color::DarkGray),
        ));
        spans.push(Span::styled("●", Style::default().fg(Color::Green)));
        spans.push(Span::styled(
            " Connected",
            Style::default().fg(Color::DarkGray),
        ));

        if state.is_recording {
            spans.push(Span::styled(" ● REC", Style::default().fg(Color::Red)));
            if let Some(secs) = state.recording_seconds {
                spans.push(Span::styled(
                    format!(" {}", format_duration(secs)),
                    Style::default().fg(Color::Red),
                ));
            }
        }

        Line::from(spans)
    } else if state.is_connecting {
        Line::from(vec![
            Span::styled(
                " sonyctl ",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("— ", Style::default().fg(Color::DarkGray)),
            Span::styled("Connecting...", Style::default().fg(Color::Yellow)),
        ])
    } else {
        Line::from(vec![
            Span::styled(
                " sonyctl ",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("— Not connected", Style::default().fg(Color::DarkGray)),
        ])
    };

    frame.render_widget(Paragraph::new(line), area);
}

fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}
