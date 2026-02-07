pub mod commands;
pub mod menu;
pub mod models;
pub mod plugins;
pub mod state;

use crate::commands::{
    add_to_history, clear_history, get_history, get_process_memory, get_system_stats, inspect_file,
    purge_all_data,
    settings::{get_settings, update_setting},
    vault::{delete_secret, get_vault_entries, reveal_secret, save_secret},
};

use crate::plugins::register_plugins;

use crate::state::config::ConfigState;

use crate::state::db::DbState;

use log::{error, info};

use rusqlite::Connection;

use tauri::Manager;

use tauri_plugin_store::StoreExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // ... existing panic hook ...
    std::panic::set_hook(Box::new(|info| {
        let location = info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown".to_string());
        let msg = match info.payload().downcast_ref::<&str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };
        eprintln!("ZERO CRITICAL PANIC at {}: {}", location, msg);
    }));

    let mut builder = tauri::Builder::default();

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        builder = builder.plugin(tauri_plugin_window_state::Builder::default().build());
    }

    // Register all centralized plugins
    builder = register_plugins(builder);

    builder
        .setup(|app| {
            info!("ZERO Core Starting...");

            // 0. Initialize Database
            let app_dir = app.path().app_data_dir().map_err(|e| {
                error!("Failed to resolve app data dir: {}", e);
                e
            })?;
            if !app_dir.exists() {
                std::fs::create_dir_all(&app_dir)?;
            }
            let db_path = app_dir.join("history.db");
            let conn = Connection::open(db_path)?;
            conn.execute(
                "CREATE TABLE IF NOT EXISTS history (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    path TEXT NOT NULL UNIQUE,
                    format TEXT NOT NULL,
                    scanned_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )",
                [],
            )?;

            // 1. Manage State
            let config_state = ConfigState::new();

            // Sync ConfigState with persistent store on startup
            if let Ok(store) = app.store("settings.json") {
                if let Some(val) = store.get("config") {
                    if let Ok(saved_config) =
                        serde_json::from_value::<crate::state::config::AppConfig>(val)
                    {
                        if let Ok(mut config) = config_state.0.lock() {
                            *config = saved_config;
                            info!("ConfigState initialized from persistent store");
                        }
                    }
                }
            }

            app.manage(config_state);
            app.manage(DbState::new(conn));

            // 2. Setup System Tray safely
            if let Err(e) = crate::plugins::tray::init(app.handle()) {
                error!("Failed to initialize system tray: {}", e);
            } else {
                info!("System tray initialized");
            }

            info!("ZERO Setup Complete. Ready for events.");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_system_stats,
            inspect_file,
            get_history,
            add_to_history,
            clear_history,
            purge_all_data,
            get_settings,
            update_setting,
            get_vault_entries,
            save_secret,
            delete_secret,
            reveal_secret,
            get_process_memory
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
                info!("Window close requested -> Minimized to tray");
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
