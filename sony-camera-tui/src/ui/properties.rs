use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Wrap,
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

use crate::app::{App, ConnectedCamera, PropertyEditorFocus};
use crsdk::{property_description, property_display_name};

use super::header::{self, HeaderState};

pub fn render(frame: &mut Frame, app: &App, camera: &Option<ConnectedCamera>) {
    let area = frame.area();

    let layout = if app.property_editor.show_info {
        Layout::vertical([
            Constraint::Length(1), // Header
            Constraint::Min(10),   // Content
            Constraint::Length(5), // Info panel
            Constraint::Length(1), // Shortcuts
        ])
        .split(area)
    } else {
        Layout::vertical([
            Constraint::Length(1), // Header
            Constraint::Min(10),   // Content
            Constraint::Length(1), // Shortcuts
        ])
        .split(area)
    };

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
    render_content(frame, layout[1], app);

    if app.property_editor.show_info {
        render_info_panel(frame, layout[2], app);
        render_shortcuts(frame, layout[3], app);
    } else {
        render_shortcuts(frame, layout[2], app);
    }
}

fn render_content(frame: &mut Frame, area: Rect, app: &App) {
    // Show "not connected" message if properties not loaded
    if !app.properties.is_loaded() {
        let block = Block::default()
            .title(" Properties ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(40, 40, 40)));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let msg = if app.connected_camera.is_some() {
            "  Loading properties..."
        } else {
            "  Not connected - press Esc to return to dashboard"
        };
        let paragraph = Paragraph::new(Line::from(vec![Span::styled(
            msg,
            Style::default().fg(Color::Rgb(60, 60, 60)),
        )]));
        frame.render_widget(paragraph, inner);
        return;
    }

    let columns = Layout::horizontal([Constraint::Length(18), Constraint::Min(40)]).split(area);

    render_categories(frame, columns[0], app);
    render_property_values(frame, columns[1], app);
}

fn render_categories(frame: &mut Frame, area: Rect, app: &App) {
    let focused = app.property_editor.focus == PropertyEditorFocus::Categories;
    let categories = app.properties.available_categories();

    let title_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Rgb(180, 180, 180))
    };

    let block = Block::default()
        .title(Span::styled(" Categories ", title_style))
        .borders(Borders::ALL)
        .border_style(if focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::Rgb(60, 60, 60))
        });

    let inner = block.inner(area);
    let visible_height = inner.height as usize;
    let scroll_offset = scroll_offset_for_selection(
        app.property_editor.category_index,
        visible_height,
        categories.len(),
    );

    let items: Vec<ListItem> = categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let is_selected = i == app.property_editor.category_index;
            let style = if is_selected && focused {
                Style::default().fg(Color::Cyan)
            } else if is_selected {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            let prefix = if is_selected && focused { "▸ " } else { "  " };
            ListItem::new(Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(cat.to_string(), style),
            ]))
        })
        .collect();

    let visible_items: Vec<ListItem> = items.into_iter().skip(scroll_offset).collect();
    let list = List::new(visible_items).block(block);
    frame.render_widget(list, area);

    if categories.len() > visible_height {
        let mut scrollbar_state = ScrollbarState::new(categories.len()).position(scroll_offset);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(Style::default().fg(Color::Rgb(60, 60, 60)));
        frame.render_stateful_widget(scrollbar, inner, &mut scrollbar_state);
    }
}

fn render_property_values(frame: &mut Frame, area: Rect, app: &App) {
    let categories = app.properties.available_categories();
    let current_category = app.property_editor.current_category(&categories);
    let props_focused = app.property_editor.focus == PropertyEditorFocus::Properties;

    let block = Block::default()
        .title(format!(" {} ", current_category))
        .borders(Borders::ALL)
        .border_style(if props_focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::Rgb(60, 60, 60))
        });

    let properties = app.properties.properties_by_category(current_category);

    if properties.is_empty() {
        let paragraph = Paragraph::new("\n  No properties available")
            .style(Style::default().fg(Color::DarkGray))
            .block(block);
        frame.render_widget(paragraph, area);
        return;
    }

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let columns = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner_area);

    render_property_list(frame, columns[0], app, &properties);
    render_value_list(frame, columns[1], app, &properties);
}

fn render_property_list(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    properties: &[&crate::property::Property],
) {
    const NAME_WIDTH: usize = 34;
    let props_focused = app.property_editor.focus == PropertyEditorFocus::Properties;
    let visible_height = area.height as usize;
    let scroll_offset = scroll_offset_for_selection(
        app.property_editor.property_index,
        visible_height,
        properties.len(),
    );

    let items: Vec<ListItem> = properties
        .iter()
        .enumerate()
        .map(|(i, prop)| {
            let is_selected = i == app.property_editor.property_index;
            let is_pinned = app.properties.is_pinned(prop.code);

            let name_style = if !prop.writable {
                Style::default().fg(Color::Rgb(80, 80, 80))
            } else if is_selected && props_focused {
                Style::default().fg(Color::Cyan)
            } else if is_selected {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            let value_style = if is_selected && props_focused {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else if is_selected {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            let prefix = if is_selected && props_focused {
                "▸ "
            } else {
                "  "
            };

            let pin_indicator = if is_pinned { "★ " } else { "  " };
            let pin_style = if is_pinned {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };

            let suffix = if !prop.writable {
                Span::styled(" 🔒", Style::default().fg(Color::Rgb(80, 80, 80)))
            } else {
                Span::raw("")
            };

            ListItem::new(Line::from(vec![
                Span::styled(prefix, name_style),
                Span::styled(pin_indicator, pin_style),
                Span::styled(
                    format!(
                        "{:width$}",
                        property_display_name(prop.code),
                        width = NAME_WIDTH
                    ),
                    name_style,
                ),
                Span::styled(prop.current_value(), value_style),
                suffix,
            ]))
        })
        .collect();

    let visible_items: Vec<ListItem> = items.into_iter().skip(scroll_offset).collect();
    let list = List::new(visible_items);
    frame.render_widget(list, area);

    if properties.len() > visible_height {
        let mut scrollbar_state = ScrollbarState::new(properties.len()).position(scroll_offset);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(Style::default().fg(Color::Rgb(60, 60, 60)));
        frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
    }
}

fn render_value_list(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    properties: &[&crate::property::Property],
) {
    let Some(prop) = properties.get(app.property_editor.property_index) else {
        return;
    };

    let values_focused = app.property_editor.focus == PropertyEditorFocus::Values;

    let block = Block::default()
        .title(Line::from(vec![Span::styled(
            " Available ",
            Style::default().fg(if values_focused {
                Color::Cyan
            } else {
                Color::DarkGray
            }),
        )]))
        .borders(Borders::LEFT)
        .border_style(if values_focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::Rgb(60, 60, 60))
        });

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if prop.values.is_empty() {
        let hint = Paragraph::new(Line::from(vec![Span::styled(
            "\n  No values available",
            Style::default().fg(Color::DarkGray),
        )]));
        frame.render_widget(hint, inner);
        return;
    }

    let visible_height = inner.height as usize;
    let scroll_offset = scroll_offset_for_selection(
        app.property_editor.value_preview_index,
        visible_height,
        prop.values.len(),
    );

    let items: Vec<ListItem> = prop
        .values
        .iter()
        .enumerate()
        .map(|(i, val)| {
            let is_current = val == prop.current_value();
            let is_selected = values_focused && i == app.property_editor.value_preview_index;

            let style = if is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else if is_current {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            let prefix = if is_selected {
                "▸ "
            } else if is_current {
                "● "
            } else {
                "  "
            };

            ListItem::new(Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(val, style),
            ]))
        })
        .collect();

    let visible_items: Vec<ListItem> = items.into_iter().skip(scroll_offset).collect();
    let list = List::new(visible_items);
    frame.render_widget(list, inner);

    if prop.values.len() > visible_height {
        let mut scrollbar_state = ScrollbarState::new(prop.values.len()).position(scroll_offset);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(Style::default().fg(Color::Rgb(60, 60, 60)));
        frame.render_stateful_widget(scrollbar, inner, &mut scrollbar_state);
    }
}

fn render_info_panel(frame: &mut Frame, area: Rect, app: &App) {
    let categories = app.properties.available_categories();
    let current_category = app.property_editor.current_category(&categories);
    let properties = app.properties.properties_by_category(current_category);

    let (title, description) =
        if let Some(prop) = properties.get(app.property_editor.property_index) {
            let name = property_display_name(prop.code);
            let desc = property_description(prop.code);
            (name, desc)
        } else {
            (
                "No property selected",
                "Select a property to see its description.",
            )
        };

    let block = Block::default()
        .title(Span::styled(
            format!(" {} ", title),
            Style::default().fg(Color::Cyan),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 60)));

    let paragraph = Paragraph::new(description)
        .style(Style::default().fg(Color::DarkGray))
        .wrap(Wrap { trim: true })
        .block(block);

    frame.render_widget(paragraph, area);
}

fn render_shortcuts(frame: &mut Frame, area: Rect, app: &App) {
    let shortcuts = match app.property_editor.focus {
        PropertyEditorFocus::Categories | PropertyEditorFocus::Properties => Line::from(vec![
            Span::styled(" ↑↓ ", Style::default().fg(Color::Cyan)),
            Span::styled("Select", Style::default().fg(Color::DarkGray)),
            Span::raw("  "),
            Span::styled(" ←→ ", Style::default().fg(Color::Cyan)),
            Span::styled("Adjust", Style::default().fg(Color::DarkGray)),
            Span::raw("  "),
            Span::styled(" o ", Style::default().fg(Color::Cyan)),
            Span::styled("Values", Style::default().fg(Color::DarkGray)),
            Span::raw("  "),
            Span::styled(" i ", Style::default().fg(Color::Cyan)),
            Span::styled("Info", Style::default().fg(Color::DarkGray)),
            Span::raw("  "),
            Span::styled(" / ", Style::default().fg(Color::Cyan)),
            Span::styled("Search", Style::default().fg(Color::DarkGray)),
            Span::raw("  "),
            Span::styled(" * ", Style::default().fg(Color::Yellow)),
            Span::styled("Pin", Style::default().fg(Color::DarkGray)),
            Span::raw("  "),
            Span::styled(" Esc ", Style::default().fg(Color::Cyan)),
            Span::styled("Back", Style::default().fg(Color::DarkGray)),
        ]),
        PropertyEditorFocus::Values => Line::from(vec![
            Span::styled(" ↑↓ ", Style::default().fg(Color::Cyan)),
            Span::styled("Select", Style::default().fg(Color::DarkGray)),
            Span::raw("  "),
            Span::styled(" Enter ", Style::default().fg(Color::Cyan)),
            Span::styled("Apply", Style::default().fg(Color::DarkGray)),
            Span::raw("  "),
            Span::styled(" i ", Style::default().fg(Color::Cyan)),
            Span::styled("Info", Style::default().fg(Color::DarkGray)),
            Span::raw("  "),
            Span::styled(" Esc ", Style::default().fg(Color::Cyan)),
            Span::styled("Cancel", Style::default().fg(Color::DarkGray)),
        ]),
    };

    frame.render_widget(Paragraph::new(shortcuts), area);
}
