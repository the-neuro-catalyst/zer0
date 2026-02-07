use crate::state::config::{AppConfig, ConfigState};

use tauri::{AppHandle, State};

use tauri_plugin_store::StoreExt;

use log::{debug, info};

const SETTINGS_FILE: &str = "settings.json";

#[tauri::command]
pub fn get_settings(app: AppHandle, state: State<'_, ConfigState>) -> Result<AppConfig, String> {
    debug!("Syncing settings with persistent store...");

    // 1. Get the store from Tauri
    let store = app.get_store(SETTINGS_FILE).ok_or("Store not found")?;

    // 2. Refresh memory state from persistent store if it exists
    let mut config = state.0.lock().map_err(|e| e.to_string())?;

    // Attempt to map store values back to our AppConfig struct
    // This is much more robust than manual file reading
    if let Some(val) = store.get("config")
        && let Ok(saved_config) = serde_json::from_value::<AppConfig>(val)
    {
        *config = saved_config;
        info!("Settings synchronized with persistent store");
    }

    Ok(config.clone())
}

#[tauri::command]

pub fn update_setting(
    app: AppHandle,
    state: State<'_, ConfigState>,
    key: String,
    value: bool,
) -> Result<(), String> {
    info!("Updating setting: {} -> {}", key, value);

    // 1. Update In-Memory State

    let mut config = state.0.lock().map_err(|e| e.to_string())?;

    match key.as_str() {
        "zero_copy" => config.zero_copy = value,

        "schema_inference" => config.schema_inference = value,

        "pii_redaction" => config.pii_redaction = value,

        "strict_mode" => config.strict_mode = value,

        _ => return Err(format!("Unknown setting key: {}", key)),
    }

    // 2. Persist to Store

    let store = app.store(SETTINGS_FILE).map_err(|e| e.to_string())?;

    store.set("config", serde_json::to_value(&*config).map_err(|e| e.to_string())?);

    // 3. Save to disk explicitly

    store.save().map_err(|e| e.to_string())?;

    debug!("Settings pushed to persistent layer and saved to disk");

    Ok(())
}
