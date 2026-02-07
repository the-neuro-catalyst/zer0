use crate::app::MonitorApp;

use ratatui::{prelude::*, widgets::*};

use reader::reader_result::{DataReaderResult, SchemaValue};

pub fn render(f: &mut Frame, app: &mut MonitorApp) {
    let accent_color = crate::ui::get_accent_color(app);
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Top Bar
            Constraint::Min(10),   // Table Content
            Constraint::Length(1), // Footer
        ])
        .split(f.size());

    // 1. Header
    let header_text = format!(
        " ZERO TABLE INSPECTOR | MODE: TABULAR_ANALYSIS | NODE: {} ",
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
        .constraints([Constraint::Min(40), Constraint::Length(50)])
        .split(main_layout[1]);

    // 2. Table Engine
    if let Some(res) = &app.inspection_result {
        let mut headers = Vec::new();
        let mut rows = Vec::new();

        match res {
            DataReaderResult::Csv(data, _) => {
                headers = data.column_headers.clone();
                for row in &data.data_rows {
                    if let SchemaValue::Array(vals) = row {
                        let row_data: Vec<String> = vals.iter().map(|v| format!("{}", v)).collect();
                        rows.push(Row::new(row_data));
                    }
                }
            }
            DataReaderResult::Parquet(data, _) => {
                headers = data.column_schemas.iter().map(|c| c.name.clone()).collect();
                if let Some(sample) = &data.sample_rows {
                    for prow in sample {
                        let mut row_vec = Vec::new();
                        for h_idx in 0..headers.len() {
                            let val = prow.0.get(h_idx).cloned().unwrap_or(SchemaValue::Null);
                            row_vec.push(format!("{}", val));
                        }
                        rows.push(Row::new(row_vec));
                    }
                }
            }
            _ => {
                f.render_widget(
                    Paragraph::new("INTERNAL_STATE_ERROR :: Table unsupported for this format.")
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
        }

        if !headers.is_empty() {
            let header_row = Row::new(headers.clone())
                .style(Style::default().fg(accent_color).bold())
                .bottom_margin(1);

            let widths: Vec<Constraint> = headers
                .iter()
                .map(|_| Constraint::Percentage(100 / (headers.len() as u16)))
                .collect();

            let table = Table::new(rows, widths)
                .header(header_row)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Thick)
                        .border_style(Style::default().fg(accent_color))
                        .title(" [ TABULAR_ANALYSIS ] ")
                        .title_style(Color::White),
                )
                .highlight_style(
                    Style::default().bg(accent_color).fg(Color::Black).add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(table, content_layout[0], &mut app.table_state);
        }
    }

    // 3. Side Info
    let info_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(accent_color))
        .title(" [ SCHEMA_METRICS ] ")
        .title_style(Color::White);

    let mut info_text = vec![
        Line::from(vec![
            Span::styled(" ENGINE:    ", Style::default().bold().fg(Color::DarkGray)),
            Span::raw("TABULAR_ANALYSIS"),
        ]),
        Line::from(vec![
            Span::styled(" STATUS:    ", Style::default().bold().fg(Color::DarkGray)),
            if app.inspection_result.as_ref().map(|r| r.get_metadata().compromised).unwrap_or(false)
            {
                Span::styled("COMPROMISED", Style::default().fg(Color::Red).bold())
            } else {
                Span::styled("CLEAN", Style::default().fg(Color::Green).bold())
            },
        ]),
    ];

    if let Some(res) = &app.inspection_result {
        let meta = res.get_metadata();
        let rows = meta.line_count.unwrap_or(0);
        let size = meta.size_bytes;

        let avg_size = if rows > 0 { size / (rows as u64) } else { 0 };

        info_text.push(Line::from(vec![
            Span::styled(" RECORDS:   ", Style::default().bold().fg(Color::DarkGray)),
            Span::raw(format!("{} (TOTAL_COUNT)", rows)),
        ]));
        info_text.push(Line::from(vec![
            Span::styled(" AVG_SIZE:  ", Style::default().bold().fg(Color::DarkGray)),
            Span::raw(format!("{} bytes/rec", avg_size)),
        ]));
        info_text.push(Line::from(vec![
            Span::styled(" SCAN_TYPE: ", Style::default().bold().fg(Color::DarkGray)),
            Span::styled("ZERO_COPY_READ", Style::default().fg(Color::Green)),
        ]));

        info_text.push(Line::from(""));
        info_text.push(Line::from(vec![Span::styled(
            " [ COLUMN_DEFINITION ] ",
            Style::default().fg(accent_color).bold(),
        )]));

        match res {
            DataReaderResult::Csv(data, _) => {
                if let Some(schema) = &data.inferred_schema {
                    // Iterate using headers to maintain original file order
                    for header in &data.column_headers {
                        if let Some(dtype) = schema.get(header) {
                            info_text.push(Line::from(vec![
                                Span::styled(
                                    format!(" - {}: ", header),
                                    Style::default().fg(Color::DarkGray),
                                ),
                                Span::raw(format!("{}", dtype).to_uppercase()),
                            ]));
                        }
                    }
                }
            }
            DataReaderResult::Parquet(data, _) => {
                for col in &data.column_schemas {
                    info_text.push(Line::from(vec![
                        Span::styled(
                            format!(" - {}: ", col.name),
                            Style::default().fg(Color::DarkGray),
                        ),
                        Span::raw(col.logical_type.clone().to_uppercase()),
                    ]));
                }
            }
            _ => {}
        }
    }

    f.render_widget(
        Paragraph::new(info_text).block(info_block).wrap(Wrap { trim: true }),
        content_layout[1],
    );

    // 4. Footer
    f.render_widget(
        Paragraph::new(" [ESC] DISENGAGE | [UP/DOWN] NAVIGATE | [Q] TERMINATE ")
            .style(Style::default().bg(accent_color).fg(Color::Black).bold()),
        main_layout[2],
    );
}
