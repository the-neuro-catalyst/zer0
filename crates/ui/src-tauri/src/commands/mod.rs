pub mod settings;
pub mod vault;

use crate::state::db::DbState;

use crate::state::config::ConfigState;

use tauri::State;

use reader::engine::file_engine::FileReaderOptions;

use reader::engine::router::read_source;

use reader::output::{OutputFormat, OutputMode};

use reader::reader_result::DataReaderResult;

use tauri::Manager;

use std::fs;

use log::{error, info, warn};

use crate::models::{FileInspection, InspectionMetadata, SystemStats};
// ... existing imports ...

#[tauri::command]
pub async fn get_history(db: State<'_, DbState>) -> Result<Vec<serde_json::Value>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT path, format, scanned_at FROM history ORDER BY scanned_at DESC LIMIT 50")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(serde_json::json!({
                "path": row.get::<_, String>(0)?,
                "format": row.get::<_, String>(1)?,
                "scanned_at": row.get::<_, String>(2)?
            }))
        })
        .map_err(|e| e.to_string())?;

    let mut history = Vec::new();
    for r in rows.flatten() {
        history.push(r);
    }

    info!("History Query: Retrieved {} records", history.len());
    Ok(history)
}

#[tauri::command]
pub async fn add_to_history(
    db: State<'_, DbState>,
    path: String,
    format: String,
) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT OR REPLACE INTO history (path, format, scanned_at) VALUES (?, ?, CURRENT_TIMESTAMP)", 
        [&path, &format]
    ).map_err(|e| {
        error!("Failed to insert history record: {}", e);
        e.to_string()
    })?;

    info!("History Logged: {} [{}]", path, format);
    Ok(())
}

#[tauri::command]
pub async fn clear_history(db: State<'_, DbState>) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let count = conn.execute("DELETE FROM history", []).map_err(|e| e.to_string())?;

    info!("History Purged: Removed {} records from database", count);
    Ok(())
}

#[tauri::command]
pub async fn purge_all_data(app: tauri::AppHandle) -> Result<(), String> {
    warn!("CRITICAL: Global data purge initiated!");
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;

    if app_dir.exists() {
        // Remove the entire app data directory (SQLite, Vault, Master Key)
        fs::remove_dir_all(&app_dir)
            .map_err(|e| format!("Failed to purge app directory: {}", e))?;
        info!("All application data destroyed successfully at {:?}", app_dir);
    }

    // Optionally also clear cache if available
    if let Ok(cache_dir) = app.path().app_cache_dir() {
        if cache_dir.exists() {
            let _ = fs::remove_dir_all(&cache_dir);
            info!("Application cache cleared.");
        }
    }

    Ok(())
}

#[tauri::command]
pub fn get_system_stats() -> SystemStats {
    SystemStats {
        total_sessions: 1402,
        total_events: 89432,
        active_nodes: 12,
        validity_confidence: 0.984,
    }
}

#[tauri::command]
pub fn get_process_memory() -> f64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/self/statm") {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() > 1 {
                if let Ok(pages) = parts[1].parse::<u64>() {
                    return (pages * 4096) as f64 / 1024.0 / 1024.0;
                }
            }
        }
    }
    0.0
}

#[tauri::command]
pub async fn inspect_file(
    path: String,
    state: State<'_, ConfigState>,
    head: Option<usize>, // เพิ่มพารามิเตอร์ head
) -> Result<FileInspection, String> {
    if !std::path::Path::new(&path).exists()
        && !path.starts_with("http")
        && !path.starts_with("s3://")
    {
        error!("Inspection Failed: Resource not found -> {}", path);
        return Err(format!("Resource access denied: {}", path));
    }

    let config = state.0.lock().map_err(|e| e.to_string())?.clone();
    info!("Starting Inspection: {} [Redaction: {}]", path, config.pii_redaction);

    let options = FileReaderOptions {
        head, // ใช้พารามิเตอร์ head ที่รับเข้ามา
        file_type_override: None,
        output_mode: OutputMode::Default,
        output_format: OutputFormat::Json,
        pii_redaction: config.pii_redaction,
        zero_copy: config.zero_copy,
        recursive: false,
        filter_exts: None,
        output_path: None,
    };

    let result = read_source(&path, None, options).await.map_err(|e| {
        error!("Engine Core Error during read: {}", e);
        format!("Engine Core Error: {}", e)
    })?;

    let metadata = result.get_metadata();

    let format_str = match &result {
        DataReaderResult::Csv(_, _) => "CSV",
        DataReaderResult::Json(_, _) => "JSON",
        DataReaderResult::Parquet(_, _) => "PARQUET",
        DataReaderResult::Sqlite(_, _) => "SQLITE",
        DataReaderResult::Text(_, _) => "TEXT",
        DataReaderResult::Markdown(_, _) => "MARKDOWN",
        DataReaderResult::Pdf(_, _) => "PDF",
        DataReaderResult::Yaml(_, _) => "YAML",
        DataReaderResult::Toml(_, _) => "TOML",
        DataReaderResult::Xml(_, _) => "XML",
        DataReaderResult::Image(_, _) => "IMAGE",
        DataReaderResult::Spreadsheet(_, _) => "SPREADSHEET",
        DataReaderResult::Zip(_, _) => "ZIP",
        DataReaderResult::Gzip(_, _) => "GZIP",
        _ => "BINARY",
    }
    .to_string();

    info!("Inspection Complete: Detected {} ({} bytes)", format_str, metadata.size_bytes);

    Ok(FileInspection {
        path,
        format: format_str,
        size_bytes: metadata.size_bytes,
        content_preview: result.get_content_preview(),
        metadata: InspectionMetadata {
            line_count: metadata.line_count,
            information_density: metadata.information_density,
            structural_depth: metadata.structural_depth,
            has_sensitive_data: metadata.compromised,
            redacted: config.pii_redaction,
        },
    })
}
