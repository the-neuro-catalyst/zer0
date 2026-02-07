use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::prelude::*;

use std::{io, sync::Arc, time::Duration};

pub mod app;
pub mod logger;
mod ui;

pub use app::MonitorApp;

use config::MasterConfig as Settings;

pub async fn run_monitor(
    log_buffer: Arc<std::sync::Mutex<Vec<String>>>,
    settings: Arc<Settings>,
) -> Result<()> {
    tracing::info!("Initializing terminal interface...");
    // --- PRODUCTION READINESS: TERMINAL SETUP ---
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    // --- ROBUSTNESS: PANIC HOOK ---
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen);
        eprintln!("\n[ZERO-TUI] CRITICAL SYSTEMIC COLLAPSE:");
        eprintln!("{}", panic_info);
    }));

    tracing::info!("Starting MonitorApp with tick rate: {}ms", settings.tui.tick_rate_ms);
    let mut app = MonitorApp::new(settings.clone(), log_buffer.clone())?;
    let tick_rate = Duration::from_millis(settings.tui.tick_rate_ms);

    // Initial directory scan
    tracing::info!("Performing initial directory scan: {:?}", app.current_path);
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let initial_path = app.current_path.clone();
    let initial_sort = app.sort_mode;
    let tx_clone = tx.clone();
    app.is_loading_dir = true;

    tokio::task::spawn_blocking(move || {
        let nodes = app::MonitorApp::scan_directory(initial_path, initial_sort);
        let _ = tx_clone.blocking_send(nodes);
    });

    while !app.should_quit {
        app.on_tick().await;

        if let Ok(nodes) = rx.try_recv() {
            app.set_nodes(nodes);
        }

        {
            if let Ok(real_logs) = log_buffer.lock() {
                app.log_entries = real_logs.clone();
            }
        }

        if terminal.draw(|f| ui::render(f, &mut app)).is_err() {
            // Silently fail render errors to avoid log spam
        }

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if app.is_jumping {
                    match key.code {
                        KeyCode::Enter => {
                            let jump_input = app.jump_input.clone();
                            let jump_path = std::path::PathBuf::from(&jump_input);
                            if jump_path.exists() {
                                app.current_path = (if jump_path.is_absolute() {
                                    jump_path
                                } else {
                                    app.current_path.join(&jump_path)
                                })
                                .canonicalize()
                                .unwrap_or_else(|_| std::path::PathBuf::from(&jump_input));

                                app.is_loading_dir = true;
                                let path = app.current_path.clone();
                                let sort = app.sort_mode;
                                let tx_inner = tx.clone();
                                tokio::task::spawn_blocking(move || {
                                    let nodes = app::MonitorApp::scan_directory(path, sort);
                                    let _ = tx_inner.blocking_send(nodes);
                                });

                                app.list_state.select(Some(0));
                            }
                            app.is_jumping = false;
                            app.jump_input.clear();
                        }
                        KeyCode::Esc => {
                            app.is_jumping = false;
                            app.jump_input.clear();
                        }
                        KeyCode::Backspace => {
                            app.jump_input.pop();
                        }
                        KeyCode::Char(c) => {
                            app.jump_input.push(c);
                        }
                        _ => {}
                    }
                } else if app.is_connecting {
                    match key.code {
                        KeyCode::Enter => {
                            app.inspect_remote_url().await?;
                            app.is_connecting = false;
                            app.remote_url_input.clear();
                        }
                        KeyCode::Esc => {
                            app.is_connecting = false;
                            app.remote_url_input.clear();
                        }
                        KeyCode::Backspace => {
                            app.remote_url_input.pop();
                        }
                        KeyCode::Char(c) => {
                            app.remote_url_input.push(c);
                        }
                        _ => {}
                    }
                } else if app.show_quit_confirm {
                    match key.code {
                        KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                            app.should_quit = true;
                        }
                        KeyCode::Esc
                        | KeyCode::Char('n')
                        | KeyCode::Char('N')
                        | KeyCode::Char('q') => {
                            app.show_quit_confirm = false;
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('t') | KeyCode::Char('T') => {
                            if app.selected_resource.is_some() && app.load_hex_data().is_ok() {
                                app.view_mode = app::ViewMode::HexView;
                            }
                        }
                        KeyCode::Char('v') | KeyCode::Char('V') => {
                            if let Some(
                                reader::reader_result::DataReaderResult::Csv(_, _)
                                | reader::reader_result::DataReaderResult::Parquet(_, _),
                            ) = &app.inspection_result
                            {
                                app.view_mode = app::ViewMode::TableView;
                                app.table_state.select(Some(0));
                            }
                        }
                        KeyCode::Char('/') => {
                            app.is_jumping = true;
                            app.jump_input.clear();
                            tracing::debug!("Path jump mode engaged");
                        }
                        KeyCode::Char('c') | KeyCode::Char('C') => {
                            app.is_connecting = true;
                            app.remote_url_input = "https://".to_string();
                            tracing::debug!("Remote connection mode engaged");
                        }
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            app.sort_mode = match app.sort_mode {
                                app::SortMode::NAME => app::SortMode::SIZE,
                                app::SortMode::SIZE => app::SortMode::MODIFIED,
                                app::SortMode::MODIFIED => app::SortMode::FORMAT,
                                app::SortMode::FORMAT => app::SortMode::NAME,
                            };
                            tracing::info!("Sort mode changed to: {:?}", app.sort_mode);
                            app.sort_nodes();
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            app.show_quit_confirm = true;
                        }
                        KeyCode::Char('?') | KeyCode::Char('h') | KeyCode::Char('H') => {
                            app.show_help = !app.show_help;
                        }
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            if let Ok(new_settings) = Settings::load() {
                                tracing::info!("Settings reloaded from zero.config.toml");
                                app.settings = Arc::new(new_settings);
                            }
                        }
                        KeyCode::Esc => {
                            if app.show_help {
                                app.show_help = false;
                            } else {
                                app.view_mode = app::ViewMode::Standard;
                                app.hex_data = None;
                                app.table_state.select(None);
                                app.selected_resource = None;
                                app.inspection_result = None;
                                tracing::debug!("View reset to standard");
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if app.view_mode == app::ViewMode::HexView {
                                app.hex_scroll = app.hex_scroll.saturating_sub(1);
                            } else if app.view_mode == app::ViewMode::TableView {
                                let i = match app.table_state.selected() {
                                    Some(i) => {
                                        if i == 0 {
                                            0
                                        } else {
                                            i - 1
                                        }
                                    }
                                    None => 0,
                                };
                                app.table_state.select(Some(i));
                            } else {
                                let i = match app.list_state.selected() {
                                    Some(i) => {
                                        if i == 0 {
                                            app.nodes.len().saturating_sub(1)
                                        } else {
                                            i - 1
                                        }
                                    }
                                    None => 0,
                                };
                                app.list_state.select(Some(i));
                                app.inspection_result = None;
                                app.selected_resource = None;
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if app.view_mode == app::ViewMode::HexView {
                                if let Some(data) = &app.hex_data {
                                    let max_rows = data.len().div_ceil(16);
                                    if app.hex_scroll + 1 < max_rows {
                                        app.hex_scroll += 1;
                                    }
                                }
                            } else if app.view_mode == app::ViewMode::TableView {
                                let i = match app.table_state.selected() {
                                    Some(i) => i + 1,
                                    None => 0,
                                };
                                app.table_state.select(Some(i));
                            } else {
                                let i = match app.list_state.selected() {
                                    Some(i) => {
                                        if i >= app.nodes.len().saturating_sub(1) {
                                            0
                                        } else {
                                            i + 1
                                        }
                                    }
                                    None => 0,
                                };
                                app.list_state.select(Some(i));
                                app.inspection_result = None;
                                app.selected_resource = None;
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(parent) = app.current_path.parent() {
                                app.current_path = parent.to_path_buf();
                                tracing::info!("Navigating up to: {:?}", app.current_path);
                                app.is_loading_dir = true;
                                let path = app.current_path.clone();
                                let sort = app.sort_mode;
                                let tx_inner = tx.clone();
                                tokio::task::spawn_blocking(move || {
                                    let nodes = app::MonitorApp::scan_directory(path, sort);
                                    let _ = tx_inner.blocking_send(nodes);
                                });
                                app.list_state.select(Some(0));
                                app.inspection_result = None;
                                app.selected_resource = None;
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(path) = app.select_node() {
                                if app.is_loading_dir {
                                    tracing::info!("Navigating into: {:?}", path);
                                    let sort = app.sort_mode;
                                    let tx_inner = tx.clone();
                                    tokio::task::spawn_blocking(move || {
                                        let nodes = app::MonitorApp::scan_directory(path, sort);
                                        let _ = tx_inner.blocking_send(nodes);
                                    });
                                } else {
                                    tracing::info!("Inspecting resource: {:?}", path);
                                    app.inspect_selected_node().await;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // --- GRACEFUL SHUTDOWN: TERMINAL RESTORATION ---
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
