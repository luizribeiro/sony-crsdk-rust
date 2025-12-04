use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::Screen;

pub fn render(frame: &mut Frame, screen: Screen) {
    let area = centered_rect(60, 18, frame.area());

    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Keyboard Shortcuts ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    frame.render_widget(block.clone(), area);

    let inner = block.inner(area);

    let content = match screen {
        Screen::Discovery => discovery_help(),
        Screen::Dashboard => dashboard_help(),
        Screen::PropertyEditor => property_editor_help(),
        Screen::EventsExpanded => events_help(),
    };

    let paragraph = Paragraph::new(content);
    frame.render_widget(paragraph, inner);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .split(area);

    Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .split(vertical[0])[0]
}

fn discovery_help() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        section("Navigation"),
        shortcut("↑↓  j/k", "Move selection"),
        shortcut("Enter", "Connect to camera"),
        Line::from(""),
        section("Actions"),
        shortcut("r", "Rescan for cameras"),
        shortcut("m", "Manual connection"),
        Line::from(""),
        section("General"),
        shortcut("?", "Toggle help"),
        shortcut("q", "Quit"),
        Line::from(""),
        Line::from(""),
        footer(),
    ]
}

fn dashboard_help() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        two_columns("Navigation", "Shooting"),
        two_col_shortcut("↑↓  j/k", "Select setting", "Space", "Capture"),
        two_col_shortcut("←→  h/l", "Adjust value", "f", "Focus"),
        two_col_shortcut("Tab", "Next panel", "v/s", "Record"),
        two_col_shortcut("1-5", "Go to panel", "", ""),
        Line::from(""),
        two_columns("Screens", "General"),
        two_col_shortcut("p", "Properties", "?", "Help"),
        two_col_shortcut("e", "Events log", "q", "Quit"),
        two_col_shortcut("d/Esc", "Disconnect", "", ""),
        Line::from(""),
        footer(),
    ]
}

fn property_editor_help() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        section("Navigation"),
        shortcut("←→  h/l", "Switch categories"),
        shortcut("↑↓  j/k", "Select property"),
        shortcut("Tab", "Toggle value list"),
        Line::from(""),
        section("Actions"),
        shortcut("Enter", "Apply selected value"),
        shortcut("Esc", "Back to dashboard"),
        Line::from(""),
        section("General"),
        shortcut("?", "Toggle help"),
        shortcut("q", "Quit"),
        Line::from(""),
        footer(),
    ]
}

fn events_help() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        section("Navigation"),
        shortcut("↑↓  j/k", "Scroll events"),
        shortcut("g", "Jump to top"),
        shortcut("G", "Jump to bottom"),
        Line::from(""),
        section("Actions"),
        shortcut("c", "Clear log"),
        shortcut("Esc", "Back to dashboard"),
        Line::from(""),
        section("General"),
        shortcut("?", "Toggle help"),
        shortcut("q", "Quit"),
        Line::from(""),
        footer(),
    ]
}

fn section(text: &str) -> Line<'static> {
    Line::from(vec![
        Span::raw("  "),
        Span::styled(
            text.to_string(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ])
}

fn shortcut(key: &str, desc: &str) -> Line<'static> {
    Line::from(vec![
        Span::raw("    "),
        Span::styled(format!("{:12}", key), Style::default().fg(Color::Cyan)),
        Span::styled(desc.to_string(), Style::default().fg(Color::DarkGray)),
    ])
}

fn two_columns(left: &str, right: &str) -> Line<'static> {
    Line::from(vec![
        Span::raw("  "),
        Span::styled(
            format!("{:24}", left),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            right.to_string(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ])
}

fn two_col_shortcut(key1: &str, desc1: &str, key2: &str, desc2: &str) -> Line<'static> {
    Line::from(vec![
        Span::raw("    "),
        Span::styled(format!("{:8}", key1), Style::default().fg(Color::Cyan)),
        Span::styled(
            format!("{:16}", desc1),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(format!("{:8}", key2), Style::default().fg(Color::Cyan)),
        Span::styled(desc2.to_string(), Style::default().fg(Color::DarkGray)),
    ])
}

fn footer() -> Line<'static> {
    Line::from(Span::styled(
        "              Press any key to close",
        Style::default().fg(Color::Rgb(80, 80, 80)),
    ))
}
