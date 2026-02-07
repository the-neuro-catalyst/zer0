use crate::app::MonitorApp;

use ratatui::{prelude::*, widgets::*};

pub fn render(f: &mut Frame, app: &mut MonitorApp) {
    let accent_color = crate::ui::get_accent_color(app);
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Top Bar
            Constraint::Min(10),   // Hex Content
            Constraint::Length(1), // Footer
        ])
        .split(f.size());

    // 1. Header
    let header_text = format!(
        " [ ZERO HEX INSPECTOR ] | MODE: RAW_BYTE_ANALYSIS | NODE: {} ",
        app.selected_resource.as_ref().map(|n| n.name.as_str()).unwrap_or("None")
    );
    f.render_widget(
        Paragraph::new(header_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(accent_color)),
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(accent_color).add_modifier(Modifier::BOLD)),
        main_layout[0],
    );

    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(40), Constraint::Length(30)])
        .split(main_layout[1]);

    // 2. Hex View Engine
    if let Some(data) = &app.hex_data {
        let rows_to_show = content_layout[0].height.saturating_sub(2) as usize;
        let mut lines = Vec::new();

        let start_row = app.hex_scroll;
        let end_row = (start_row + rows_to_show).min(data.len().div_ceil(16));

        for row in start_row..end_row {
            let offset = row * 16;
            let mut hex_part = String::with_capacity(48);
            let mut ascii_part = String::with_capacity(16);

            for i in 0..16 {
                let idx = offset + i;
                if idx < data.len() {
                    let b = data[idx];
                    hex_part.push_str(&format!("{:02X} ", b));
                    if (32..=126).contains(&b) {
                        ascii_part.push(b as char);
                    } else {
                        ascii_part.push('.');
                    }
                } else {
                    hex_part.push_str("   ");
                    ascii_part.push(' ');
                }
            }

            lines.push(Line::from(vec![
                Span::styled(format!("{:08X}  ", offset), Style::default().fg(Color::DarkGray)),
                Span::raw(hex_part),
                Span::styled(" | ", Style::default().fg(Color::DarkGray)),
                Span::raw(ascii_part),
            ]));
        }

        f.render_widget(
            Paragraph::new(lines).block(
                Block::default()
                    .title(" [ HEX_STREAM_REALITY ] ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(accent_color)),
            ),
            content_layout[0],
        );
    } else {
        f.render_widget(
            Paragraph::new("BUFFER_EMPTY :: Await inspection activation.")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Thick)
                        .border_style(Style::default().fg(accent_color)),
                )
                .alignment(Alignment::Center),
            content_layout[0],
        );
    }

    // 3. Side Info
    let info_block = Block::default()
        .title(" [ BUFFER_METADATA ] ")
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(accent_color));
    let mut info_text = vec![
        Line::from(vec![
            Span::styled(" VIEW:   ", Style::default().bold().fg(Color::DarkGray)),
            Span::raw("HEX / ASCII"),
        ]),
        Line::from(vec![
            Span::styled(" STRIDE: ", Style::default().bold().fg(Color::DarkGray)),
            Span::raw("16 Bytes/Row"),
        ]),
    ];

    if let Some(data) = &app.hex_data {
        info_text.push(Line::from(vec![
            Span::styled(" LOADED: ", Style::default().bold().fg(Color::DarkGray)),
            Span::raw(format!("{} bytes", data.len())),
        ]));
        info_text.push(Line::from(vec![
            Span::styled(" OFFSET: ", Style::default().bold().fg(Color::DarkGray)),
            Span::raw(format!("0x{:08X}", app.hex_scroll * 16)),
        ]));
    }

    f.render_widget(Paragraph::new(info_text).block(info_block), content_layout[1]);

    // 4. Footer
    f.render_widget(
        Paragraph::new(" [ESC] DISENGAGE | [UP/DOWN] SCROLL | [Q] TERMINATE ")
            .style(Style::default().bg(accent_color).fg(Color::Black).bold()),
        main_layout[2],
    );
}
