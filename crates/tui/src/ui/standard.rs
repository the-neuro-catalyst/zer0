use crate::app::MonitorApp;

use ratatui::{prelude::*, widgets::*};

pub fn render(f: &mut Frame, app: &mut MonitorApp) {
    let accent_color = crate::ui::get_accent_color(app);
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Top Bar
            Constraint::Min(20),    // Explorer + Details (Flexible)
            Constraint::Length(14), // Intelligence Dashboard (Expanded for Axioms)
            Constraint::Length(1),  // Footer
        ])
        .split(f.size());

    // 1. Header
    let header_text: String = format!(
        " ZERO DATA INSPECTOR | MODE: ZERO-COPY | NODE: {} ",
        app.current_path.to_string_lossy().bold()
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

    let mid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(main_layout[1]);

    // 2. File Explorer
    let items: Vec<ListItem> = app
        .nodes
        .iter()
        .map(|n| {
            let is_selected_resource =
                app.selected_resource.as_ref().is_some_and(|t| t.path == n.path);
            let color = if n.is_dir { Color::Yellow } else { Color::White };
            let mut style = Style::default().fg(color);
            if is_selected_resource {
                style = style.bg(accent_color).fg(Color::Black).add_modifier(Modifier::BOLD);
            }

            let content = Line::from(vec![
                Span::styled(format!("{} ", if n.is_dir { "󰉋" } else { "󰈚" }), style),
                Span::styled(format!("{:<20}", n.name), style),
                Span::styled(format!(" | {:<6}", n.format), Style::default().fg(Color::DarkGray)),
            ]);
            ListItem::new(content)
        })
        .collect();

    let sort_label = format!(" [ FS_NODES::Sort:: {:?} ] ", app.sort_mode);
    if app.is_loading_dir {
        f.render_widget(
            Paragraph::new("\n\n  INITIALIZING SCAN...\n  (Zero-Copy Infiltration)")
                .block(
                    Block::default()
                        .title(sort_label)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Thick)
                        .border_style(Style::default().fg(accent_color)),
                )
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Yellow)),
            mid_chunks[0],
        );
    } else {
        f.render_stateful_widget(
            List::new(items)
                .block(
                    Block::default()
                        .title(sort_label)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Thick)
                        .border_style(Style::default().fg(accent_color)),
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .fg(accent_color)
                        .add_modifier(Modifier::DIM)
                        .remove_modifier(Modifier::DIM),
                ),
            mid_chunks[0],
            &mut app.list_state,
        );
    }

    // 3. Details Panel
    let details_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(accent_color))
        .title_style(Color::White)
        .title(" [ INSPECTION_DATA ] ")
        .bold();

    if app.is_inspecting {
        f.render_widget(
            Paragraph::new("ANALYZING STRUCTURE... (t=0 logic)")
                .block(details_block)
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Yellow)),
            mid_chunks[1],
        );
    } else if app.inspection_result.is_some() || app.selected_resource.is_some() {
        let details_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(8), Constraint::Min(5)])
            .split(mid_chunks[1]);

        let mut metadata_lines = Vec::new();

        if let Some(res) = &app.inspection_result {
            let meta = res.get_metadata();

            let (name, path, format) = if let Some(node) = &app.selected_resource {
                (node.name.clone(), node.path.to_string_lossy().to_string(), node.format.clone())
            } else {
                let fmt = match res {
                    reader::reader_result::DataReaderResult::Csv(_, _) => "CSV",
                    reader::reader_result::DataReaderResult::Json(_, _) => "JSON",
                    reader::reader_result::DataReaderResult::Parquet(_, _) => "PARQUET",
                    reader::reader_result::DataReaderResult::Toml(_, _) => "TOML",
                    reader::reader_result::DataReaderResult::Yaml(_, _) => "YAML",
                    reader::reader_result::DataReaderResult::Text(_, _) => "TEXT",
                    reader::reader_result::DataReaderResult::RawContent(_, _) => "RAW",
                    _ => "REMOTE_DATA",
                };
                (
                    "REMOTE_TARGET".to_string(),
                    app.remote_url.clone().unwrap_or_else(|| "UNKNOWN".to_string()),
                    fmt.to_string(),
                )
            };

            metadata_lines.push(Line::from(vec![
                Span::styled(" RESOURCE: ", Style::default().bold().fg(Color::DarkGray)),
                Span::raw(name),
            ]));
            metadata_lines.push(Line::from(vec![
                Span::styled(" TARGET:   ", Style::default().bold().fg(Color::DarkGray)),
                Span::raw(path),
            ]));
            metadata_lines.push(Line::from(vec![
                Span::styled(" FORMAT:   ", Style::default().bold().fg(Color::DarkGray)),
                Span::styled(format, Style::default().fg(accent_color)),
                Span::raw(" | "),
                Span::styled("SIZE: ", Style::default().bold().fg(Color::DarkGray)),
                Span::raw(format!("{:.2} KB", (meta.size_bytes as f64) / 1024.0)),
            ]));
            metadata_lines.push(Line::from(vec![
                Span::styled(" SECURITY: ", Style::default().bold().fg(Color::DarkGray)),
                if meta.compromised {
                    Span::styled(
                        "COMPROMISED (PII_EXPOSED)",
                        Style::default().fg(Color::Red).bold(),
                    )
                } else {
                    Span::styled("SHIELD_ACTIVE (SECURE)", Style::default().fg(Color::Green))
                },
            ]));

            let can_table = matches!(
                res,
                reader::reader_result::DataReaderResult::Csv(_, _)
                    | reader::reader_result::DataReaderResult::Parquet(_, _)
            );
            metadata_lines.push(Line::from(""));
            metadata_lines.push(Line::from(vec![
                Span::styled(" VECTORS:  ", Style::default().bold().fg(Color::DarkGray)),
                if can_table {
                    Span::styled("[V] TABLE_ENGAGED ", Style::default().fg(Color::Green).bold())
                } else {
                    Span::styled("[V] TABLE_UNSUPPORTED ", Style::default().fg(Color::DarkGray))
                },
                Span::styled(" | ", Style::default().fg(Color::DarkGray)),
                Span::styled("[T] RAW_HEX_STREAM", Style::default().fg(Color::Green).bold()),
            ]));
        } else if let Some(t) = &app.selected_resource {
            metadata_lines.push(Line::from(vec![
                Span::styled(" RESOURCE: ", Style::default().bold().fg(Color::DarkGray)),
                Span::raw(&t.name),
            ]));
            metadata_lines.push(Line::from(vec![
                Span::styled(" TARGET:   ", Style::default().bold().fg(Color::DarkGray)),
                Span::raw(t.path.to_string_lossy()),
            ]));
            metadata_lines.push(Line::from(vec![
                Span::styled(" FORMAT:   ", Style::default().bold().fg(Color::DarkGray)),
                Span::styled(&t.format, Style::default().fg(accent_color)),
                Span::raw(" | "),
                Span::styled("SIZE: ", Style::default().bold().fg(Color::DarkGray)),
                Span::raw(format!("{:.2} KB", t.size_kb)),
            ]));
            metadata_lines.push(Line::from(vec![
                Span::styled(" ACCESS:   ", Style::default().bold().fg(Color::DarkGray)),
                Span::raw(&t.permissions),
                Span::raw(" | "),
                Span::styled("STAMP: ", Style::default().bold().fg(Color::DarkGray)),
                Span::raw(&t.modified),
            ]));

            if !t.is_dir {
                metadata_lines.push(Line::from(""));
                metadata_lines.push(Line::from(vec![
                    Span::styled(" STATUS:   ", Style::default().bold().fg(Color::DarkGray)),
                    Span::styled("AWAITING_INSPECTION_COMMAND", Style::default().fg(Color::Yellow)),
                ]));
                metadata_lines.push(Line::from(vec![
                    Span::styled(" ACTION:   ", Style::default().bold().fg(Color::DarkGray)),
                    Span::raw("Press "),
                    Span::styled("ENTER", Style::default().fg(accent_color).bold()),
                    Span::raw(" to activate Core Engine."),
                ]));
            }
        }

        f.render_widget(Paragraph::new(metadata_lines).block(details_block), details_layout[0]);

        // Content Preview
        let preview_raw = if let Some(res) = &app.inspection_result {
            res.get_content_preview()
        } else if let Some(t) = &app.selected_resource {
            if t.is_dir {
                "NODE_DIRECTORY_SELECTED::Navigate to resource".to_string()
            } else {
                "ENGINE_IDLE::Request inspection via ENTER".to_string()
            }
        } else {
            "DATA_UNAVAILABLE".to_string()
        };

        let mut spans = Vec::new();
        let parts: Vec<&str> = preview_raw.split("[REDACTED]").collect();
        for (i, part) in parts.iter().enumerate() {
            spans.push(Span::raw(*part));
            if i < parts.len() - 1 {
                spans.push(Span::styled("[REDACTED]", Style::default().fg(Color::Red).bold()));
            }
        }

        f.render_widget(
            Paragraph::new(Line::from(spans))
                .block(
                    Block::default()
                        .title(" [ SOURCE_SKELETON ] ")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Thick)
                        .border_style(Style::default().fg(accent_color)),
                )
                .wrap(Wrap { trim: true }),
            details_layout[1],
        );
    } else {
        f.render_widget(
            Paragraph::new(" [ SELECT_TARGET_NODE_FOR_AUDIT ] ")
                .block(details_block)
                .alignment(Alignment::Center),
            mid_chunks[1],
        );
    }

    // 4. Intelligence Dashboard (Bottom Area)
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(main_layout[2]);

    // 4a. Activity Log
    f.render_widget(
        List::new(
            app.log_entries
                .iter()
                .rev()
                .take(15)
                .map(|s| ListItem::new(s.clone()).style(Style::default().fg(Color::Gray)))
                .collect::<Vec<_>>(),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(accent_color))
                .title_style(Color::White)
                .title(" [ SYSTEM_LOG ] ")
                .bold(),
        ),
        bottom_chunks[0],
    );

    let stats = vec![
        Line::from(vec![
            Span::styled(" STATUS:    ", Style::default().bold().fg(Color::White)),
            Span::styled("ACTIVE_NODE", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled(" RAM:       ", Style::default().bold().fg(Color::White)),
            Span::styled(
                format!("{:.2} MB", app.session_ram_usage),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(" SCANNED:   ", Style::default().bold().fg(Color::White)),
            Span::raw(format!("{} objects", app.session_scanned)),
        ]),
        Line::from(vec![
            Span::styled(" PII:       ", Style::default().bold().fg(Color::White)),
            Span::styled(
                format!("{}", app.session_pii_hits),
                if app.session_pii_hits > 0 {
                    Style::default().fg(Color::Red).bold()
                } else {
                    Style::default().fg(Color::White)
                },
            ),
        ]),
        Line::from(vec![
            Span::styled(" ACCENT:    ", Style::default().bold().fg(Color::White)),
            Span::styled(
                app.settings.tui.accent_color.to_uppercase(),
                Style::default().fg(accent_color).bold(),
            ),
        ]),
        Line::from(""),
    ];

    f.render_widget(
        Paragraph::new(stats).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(accent_color))
                .title_style(Color::White)
                .title(" [ INTELLIGENCE_MATRIX ] ")
                .bold(),
        ),
        bottom_chunks[1],
    );

    // 5. Footer
    f.render_widget(
        Paragraph::new(" [Q] QUIT | [ENTER] PERCEIVE | [H] HELP | [S] SORT | [V] TABLE | [T] HEX | [/] JUMP | [C] REMOTE ")
            .style(Style::default().bg(accent_color).fg(Color::Black).bold()),
        main_layout[3],
    );
}
