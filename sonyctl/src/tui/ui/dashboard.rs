use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    Frame,
};

fn scroll_offset_for_selection(
    selected: usize,
    visible_height: usize,
    total_items: usize,
) -> usize {
    if total_items <= visible_height {
        return 0;
    }
    let half_visible = visible_height / 2;
    if selected < half_visible {
        0
    } else if selected >= total_items.saturating_sub(half_visible) {
        total_items.saturating_sub(visible_height)
    } else {
        selected.saturating_sub(half_visible)
    }
}

use crate::tui::app::{App, ConnectedCamera, DashboardState, EventsLogState, MediaSlotInfo};
use crate::tui::property::Property;
use crsdk::{property_category, property_display_name, PropertyCategoryId};

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
        is_connecting: app.is_connecting,
    };
    header::render(frame, layout[0], &header_state);
    render_panels(frame, layout[1], app);
    render_shortcuts(frame, layout[2], &app.dashboard);
}

fn render_panels(frame: &mut Frame, area: Rect, app: &App) {
    let columns =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);

    let left_panels =
        Layout::vertical([Constraint::Length(8), Constraint::Min(8)]).split(columns[0]);

    let is_connected = app.connected_camera.is_some();
    let is_ready = app.properties.is_loaded();
    render_camera_info_panel(
        frame,
        left_panels[0],
        &app.dashboard,
        is_connected,
        is_ready,
    );
    render_events_panel(frame, left_panels[1], &app.events_log);
    render_quick_settings_panel(frame, columns[1], app);
}

fn render_camera_info_panel(
    frame: &mut Frame,
    area: Rect,
    state: &DashboardState,
    is_connected: bool,
    is_ready: bool,
) {
    let border_style = if is_ready {
        Style::default().fg(Color::Rgb(60, 60, 60))
    } else {
        Style::default().fg(Color::Rgb(40, 40, 40))
    };

    let title_style = if is_ready {
        Style::default().fg(Color::Rgb(180, 180, 180))
    } else {
        Style::default().fg(Color::Rgb(80, 80, 80))
    };

    let block = Block::default()
        .title(Span::styled(" Camera ", title_style))
        .borders(Borders::ALL)
        .border_style(border_style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Show appropriate message when not ready
    if !is_ready {
        let msg = if is_connected {
            "  Connecting..."
        } else {
            "  Not connected - press 'd' to discover"
        };
        let paragraph = Paragraph::new(Line::from(vec![Span::styled(
            msg,
            Style::default().fg(Color::Rgb(60, 60, 60)),
        )]));
        frame.render_widget(paragraph, inner);
        return;
    }

    let info = &state.camera_info;
    let battery_color = if info.battery > 50 {
        Color::Green
    } else if info.battery > 20 {
        Color::Yellow
    } else {
        Color::Red
    };

    let battery_bar = render_battery_bar(info.battery);

    let mut lines = vec![
        Line::from(vec![
            Span::styled("  Lens      ", Style::default().fg(Color::DarkGray)),
            Span::styled(&info.lens, Style::default().fg(Color::White)),
            Span::styled(
                format!(" @ {}", info.focal_length),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Format    ", Style::default().fg(Color::DarkGray)),
            Span::styled(&info.image_format, Style::default().fg(Color::White)),
            Span::styled(" │ ", Style::default().fg(Color::Rgb(60, 60, 60))),
            Span::styled(&info.recording_format, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  Battery   ", Style::default().fg(Color::DarkGray)),
            Span::styled(battery_bar, Style::default().fg(battery_color)),
            Span::styled(
                format!(" {}%", info.battery),
                Style::default().fg(battery_color),
            ),
        ]),
    ];

    if let Some(ref temp) = info.temperature {
        lines.push(Line::from(vec![
            Span::styled("  Temp      ", Style::default().fg(Color::DarkGray)),
            Span::styled(temp, Style::default().fg(Color::Yellow)),
        ]));
    }

    // Only show slots that exist on this camera
    if let Some(ref slot1) = info.slot1 {
        lines.push(render_slot_line("Slot 1", slot1));
    }
    if let Some(ref slot2) = info.slot2 {
        lines.push(render_slot_line("Slot 2", slot2));
    }
    if let Some(ref slot3) = info.slot3 {
        lines.push(render_slot_line("Slot 3", slot3));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn render_battery_bar(percentage: u8) -> String {
    let filled = (percentage as usize * 10) / 100;
    let empty = 10 - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

fn render_slot_line<'a>(label: &'a str, slot: &'a MediaSlotInfo) -> Line<'a> {
    let label_span = Span::styled(
        format!("  {:10}", label),
        Style::default().fg(Color::DarkGray),
    );

    let mut spans = vec![label_span];
    if !slot.media_type.is_empty() {
        spans.push(Span::styled(
            &slot.media_type,
            Style::default().fg(Color::White),
        ));
        spans.push(Span::raw(" "));
    }

    // Use different color for "No card" vs actual info
    let color = if slot.free_space == "No card" || slot.free_space.contains("Error") {
        Color::Rgb(80, 80, 80)
    } else {
        Color::Green
    };
    spans.push(Span::styled(&slot.free_space, Style::default().fg(color)));

    Line::from(spans)
}

fn render_quick_settings_panel(frame: &mut Frame, area: Rect, app: &App) {
    let is_connected = app.connected_camera.is_some();
    let is_ready = app.properties.is_loaded();

    let border_style = if is_ready {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Rgb(40, 40, 40))
    };

    let title_style = if is_ready {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Rgb(60, 60, 60))
    };

    let block = Block::default()
        .title(Span::styled(" Quick Settings ", title_style))
        .borders(Borders::ALL)
        .border_style(border_style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Show appropriate message when not ready
    if !is_ready {
        let msg = if is_connected {
            "  Connecting..."
        } else {
            "  Not connected"
        };
        let paragraph = Paragraph::new(Line::from(vec![Span::styled(
            msg,
            Style::default().fg(Color::Rgb(60, 60, 60)),
        )]));
        frame.render_widget(paragraph, inner);
        return;
    }

    let pinned_ids = app.properties.pinned_ids();
    if pinned_ids.is_empty() {
        let msg = Paragraph::new(Line::from(vec![Span::styled(
            "  No pinned properties",
            Style::default().fg(Color::DarkGray),
        )]));
        frame.render_widget(msg, inner);
        return;
    }

    let mut lines: Vec<Line> = Vec::new();
    let mut current_category: Option<PropertyCategoryId> = None;
    let mut selected_line_index: usize = 0;

    for (idx, &prop_code) in pinned_ids.iter().enumerate() {
        let category = property_category(prop_code);
        if current_category != Some(category) {
            current_category = Some(category);
            lines.push(Line::from(vec![
                Span::styled(
                    format!("  ─── {} ", category.to_string().to_uppercase()),
                    Style::default().fg(Color::Rgb(80, 80, 80)),
                ),
                Span::styled("─".repeat(18), Style::default().fg(Color::Rgb(40, 40, 40))),
            ]));
        }

        if idx == app.dashboard.selected_property {
            selected_line_index = lines.len();
        }

        if let Some(prop) = app.properties.get(prop_code) {
            let selected = idx == app.dashboard.selected_property;
            let has_pending = app.has_pending_change(prop_code);
            let is_in_flight = app.is_in_flight(prop_code);
            lines.push(render_property_line(
                prop,
                selected,
                has_pending,
                is_in_flight,
            ));
        }
    }

    let visible_height = inner.height as usize;
    let scroll_offset =
        scroll_offset_for_selection(selected_line_index, visible_height, lines.len());

    let paragraph = Paragraph::new(lines.clone()).scroll((scroll_offset as u16, 0));
    frame.render_widget(paragraph, inner);

    if lines.len() > visible_height {
        let mut scrollbar_state = ScrollbarState::new(lines.len()).position(scroll_offset);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(Style::default().fg(Color::Rgb(60, 60, 60)));
        frame.render_stateful_widget(scrollbar, inner, &mut scrollbar_state);
    }
}

fn render_property_line(
    prop: &Property,
    selected: bool,
    has_pending: bool,
    is_in_flight: bool,
) -> Line<'static> {
    const NAME_WIDTH: usize = 28;
    let name = property_display_name(prop.code).to_string();
    let value = prop.current_value().to_string();
    let is_disabled = !prop.writable;

    if selected {
        if is_disabled {
            Line::from(vec![
                Span::styled("  ▸ ", Style::default().fg(Color::Rgb(80, 80, 80))),
                Span::styled(
                    format!("{:width$}", name, width = NAME_WIDTH),
                    Style::default().fg(Color::Rgb(80, 80, 80)),
                ),
                Span::raw("  "),
                Span::styled(
                    format!("{:>10}", value),
                    Style::default().fg(Color::Rgb(100, 100, 100)),
                ),
                Span::styled(" 🔒", Style::default().fg(Color::Rgb(80, 80, 80))),
            ])
        } else if is_in_flight {
            Line::from(vec![
                Span::styled("  ▸ ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{:width$}", name, width = NAME_WIDTH),
                    Style::default().fg(Color::White),
                ),
                Span::styled("◀ ", Style::default().fg(Color::Rgb(60, 60, 60))),
                Span::styled(
                    format!("{:>10}", value),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" ▶", Style::default().fg(Color::Rgb(60, 60, 60))),
            ])
        } else if has_pending {
            Line::from(vec![
                Span::styled("  ▸ ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{:width$}", name, width = NAME_WIDTH),
                    Style::default().fg(Color::White),
                ),
                Span::styled("◀ ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    format!("{:>10}", value),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" ▶", Style::default().fg(Color::Yellow)),
            ])
        } else {
            Line::from(vec![
                Span::styled("  ▸ ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{:width$}", name, width = NAME_WIDTH),
                    Style::default().fg(Color::White),
                ),
                Span::styled("◀ ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{:>10}", value),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" ▶", Style::default().fg(Color::Cyan)),
            ])
        }
    } else if is_disabled {
        Line::from(vec![
            Span::raw("    "),
            Span::styled(
                format!("{:width$}", name, width = NAME_WIDTH),
                Style::default().fg(Color::Rgb(80, 80, 80)),
            ),
            Span::raw("  "),
            Span::styled(
                format!("{:>10}", value),
                Style::default().fg(Color::Rgb(100, 100, 100)),
            ),
            Span::styled(" 🔒", Style::default().fg(Color::Rgb(60, 60, 60))),
        ])
    } else {
        Line::from(vec![
            Span::raw("    "),
            Span::styled(
                format!("{:width$}", name, width = NAME_WIDTH),
                Style::default().fg(Color::DarkGray),
            ),
            Span::raw("  "),
            Span::styled(format!("{:>10}", value), Style::default().fg(Color::White)),
            Span::raw("   "),
        ])
    }
}

fn render_events_panel(frame: &mut Frame, area: Rect, events: &EventsLogState) {
    let block = Block::default()
        .title(Span::styled(
            " Events ",
            Style::default().fg(Color::Rgb(180, 180, 180)),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 60)));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if events.events.is_empty() {
        let empty_msg = Paragraph::new(Line::from(vec![Span::styled(
            "  No events yet",
            Style::default().fg(Color::DarkGray),
        )]));
        frame.render_widget(empty_msg, inner);
        return;
    }

    let items: Vec<ListItem> = events
        .events
        .iter()
        .rev()
        .map(|e| {
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!(" {} ", e.timestamp),
                    Style::default().fg(Color::Rgb(60, 60, 60)),
                ),
                Span::styled(&e.event_type, Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!(" {}", e.details),
                    Style::default().fg(Color::Rgb(100, 100, 100)),
                ),
            ]))
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner);
}

fn render_shortcuts(frame: &mut Frame, area: Rect, state: &DashboardState) {
    let mut spans = vec![
        Span::styled(" ↑↓ ", Style::default().fg(Color::Cyan)),
        Span::styled("Select", Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled(" ←→ ", Style::default().fg(Color::Cyan)),
        Span::styled("Adjust", Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled(" o ", Style::default().fg(Color::Cyan)),
        Span::styled("Open", Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled(" Space ", Style::default().fg(Color::Cyan)),
        Span::styled("Capture", Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled(" v ", Style::default().fg(Color::Cyan)),
        Span::styled("Record", Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled(" p ", Style::default().fg(Color::Cyan)),
        Span::styled("Properties", Style::default().fg(Color::DarkGray)),
    ];

    if state.is_recording {
        spans.push(Span::raw("  "));
        spans.push(Span::styled(" s ", Style::default().fg(Color::Red)));
        spans.push(Span::styled("Stop", Style::default().fg(Color::Red)));
    }

    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}
