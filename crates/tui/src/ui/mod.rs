use crate::app::{MonitorApp, ViewMode};

use ratatui::{prelude::*, widgets::*};

mod hex;
mod standard;
mod table;

pub fn render(f: &mut Frame, app: &mut MonitorApp) {
    match app.view_mode {
        ViewMode::Standard => standard::render(f, app),
        ViewMode::HexView => hex::render(f, app),
        ViewMode::TableView => table::render(f, app),
    }

    if app.show_help {
        render_help(f, app);
    }

    if app.show_quit_confirm {
        render_quit_confirm(f, app);
    }

    if app.is_jumping {
        render_path_jumper(f, app);
    }

    if app.is_connecting {
        render_remote_dialog(f, app);
    }
}

fn render_remote_dialog(f: &mut Frame, app: &MonitorApp) {
    let accent_color = get_accent_color(app);
    let area = centered_rect(70, 15, f.size());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" [ REMOTE_INFILTRATION :: HTTP/S3 ] ")
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(accent_color));

    let paragraph = Paragraph::new(format!(" TARGET_URL > {}", app.remote_url_input))
        .block(block)
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, area);
}

fn render_path_jumper(f: &mut Frame, app: &MonitorApp) {
    let accent_color = get_accent_color(app);
    let area = centered_rect(60, 10, f.size());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title(" [ JUMP_TO_COORDINATES ] ")
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(accent_color));

    let paragraph = Paragraph::new(format!(" PATH > {}", app.jump_input))
        .block(block)
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, area);
}

fn render_quit_confirm(f: &mut Frame, app: &MonitorApp) {
    let accent_color = get_accent_color(app);
    let area = centered_rect(45, 25, f.size());
    f.render_widget(Clear, area);

    let text = vec![
        Line::from(vec![Span::styled(
            " EXIT INSPECTOR ",
            Style::default().bg(accent_color).fg(Color::Black).bold(),
        )]),
        Line::from(""),
        Line::from(vec![Span::raw("Close ZERO Data Inspector?")]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" [Y] ", Style::default().fg(Color::Green).bold()),
            Span::raw("YES"),
            Span::raw("    "),
            Span::styled(" [N] ", Style::default().fg(Color::Red).bold()),
            Span::raw("NO"),
        ]),
    ];

    let block = Block::default()
        .title(" CONFIRM ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(accent_color));

    let paragraph = Paragraph::new(text).block(block).alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

pub fn get_accent_color(app: &MonitorApp) -> Color {
    let color_str = app.settings.tui.accent_color.to_lowercase();
    match color_str.as_str() {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "yellow" => Color::Yellow,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "black" => Color::Black,
        "gray" | "grey" => Color::Gray,
        "darkgray" | "darkgrey" => Color::DarkGray,
        "lightblue" => Color::LightBlue,
        "lightgreen" => Color::LightGreen,
        "lightcyan" => Color::LightCyan,
        "lightred" => Color::LightRed,
        "lightmagenta" => Color::LightMagenta,
        "lightyellow" => Color::LightYellow,
        _ => Color::Cyan, // Default ZERO accent
    }
}

fn render_help(f: &mut Frame, app: &MonitorApp) {
    let accent_color = get_accent_color(app);
    let area = centered_rect(60, 50, f.size());
    f.render_widget(Clear, area);

    let mut help_text = vec![
        Line::from(vec![Span::styled(
            format!(" ZERO {:?} HELP ", app.view_mode).to_uppercase(),
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan),
        )]),
        Line::from(""),
    ];

    match app.view_mode {
        ViewMode::Standard => {
            help_text.extend(vec![
                Line::from(vec![
                    Span::styled("  q        ", Style::default().fg(accent_color)),
                    Span::raw("- Quit"),
                ]),
                Line::from(vec![
                    Span::styled("  h / ?    ", Style::default().fg(accent_color)),
                    Span::raw("- Toggle Help"),
                ]),
                Line::from(vec![
                    Span::styled("  s        ", Style::default().fg(accent_color)),
                    Span::raw("- Cycle Sort Mode"),
                ]),
                Line::from(vec![
                    Span::styled("  /        ", Style::default().fg(accent_color)),
                    Span::raw("- Jump to Path"),
                ]),
                Line::from(vec![
                    Span::styled("  C        ", Style::default().fg(accent_color)),
                    Span::raw("- Connect to Remote (HTTP/S3)"),
                ]),
                Line::from(vec![
                    Span::styled("  Enter    ", Style::default().fg(accent_color)),
                    Span::raw("- Analyze Metadata & Unlock Views"),
                ]),
                Line::from(vec![
                    Span::styled("  V        ", Style::default().fg(accent_color)),
                    Span::raw("- Table View (CSV/Parquet, requires Enter first)"),
                ]),
                Line::from(vec![
                    Span::styled("  T        ", Style::default().fg(accent_color)),
                    Span::raw("- Hex View (Raw bytes, requires Enter first)"),
                ]),
                Line::from(vec![
                    Span::styled("  Backspc  ", Style::default().fg(accent_color)),
                    Span::raw("- Go Up Directory"),
                ]),
                Line::from(vec![
                    Span::styled("  j / k    ", Style::default().fg(accent_color)),
                    Span::raw("- Navigate Files"),
                ]),
            ]);
        }
        ViewMode::HexView => {
            help_text.extend(vec![
                Line::from(vec![
                    Span::styled("  Esc      ", Style::default().fg(accent_color)),
                    Span::raw("- Back to Standard View"),
                ]),
                Line::from(vec![
                    Span::styled("  j / k    ", Style::default().fg(accent_color)),
                    Span::raw("- Scroll Hex Rows"),
                ]),
                Line::from(vec![
                    Span::styled("  q        ", Style::default().fg(accent_color)),
                    Span::raw("- Quit"),
                ]),
            ]);
        }
        ViewMode::TableView => {
            help_text.extend(vec![
                Line::from(vec![
                    Span::styled("  Esc      ", Style::default().fg(accent_color)),
                    Span::raw("- Back to Standard View"),
                ]),
                Line::from(vec![
                    Span::styled("  j / k    ", Style::default().fg(accent_color)),
                    Span::raw("- Scroll Table Rows"),
                ]),
                Line::from(vec![
                    Span::styled("  q        ", Style::default().fg(accent_color)),
                    Span::raw("- Quit"),
                ]),
            ]);
        }
    }

    let block = Block::default()
        .title(" HELP ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(accent_color));

    let paragraph = Paragraph::new(help_text).block(block).alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
