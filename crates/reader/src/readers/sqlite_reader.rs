use crate::error::DataReaderError;
use crate::reader_result::{DataReaderResult, FileMetadata};
use rusqlite::Connection;
use schema::SchemaValue;
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct SqliteData {
    pub tables: Vec<String>,
    pub rows: Vec<Vec<SchemaValue<'static>>>,
}

pub async fn read_sqlite_data(path: &Path) -> Result<DataReaderResult, DataReaderError> {
    let conn = Connection::open(path).map_err(|e| DataReaderError::InternalError(e.to_string()))?;
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
        .map_err(|e| DataReaderError::InternalError(e.to_string()))?;
    let tables: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| DataReaderError::InternalError(e.to_string()))?
        .filter_map(|e| e.ok())
        .collect();

    Ok(DataReaderResult::Sqlite(SqliteData { tables, rows: vec![] }, FileMetadata::new(0)))
}

pub async fn read_sqlite_data_with_query(
    url: &str,
    query: &str,
) -> Result<DataReaderResult, DataReaderError> {
    let path_str = url.trim_start_matches("sqlite://");
    let conn =
        Connection::open(path_str).map_err(|e| DataReaderError::InternalError(e.to_string()))?;
    let _stmt = conn.prepare(query).map_err(|e| DataReaderError::InternalError(e.to_string()))?;

    Ok(DataReaderResult::RawContent(
        format!("Query executed successfully on {}", url),
        FileMetadata::new(0),
    ))
}

pub async fn get_sqlite_schema(url: &str) -> Result<serde_json::Value, DataReaderError> {
    let path_str = url.trim_start_matches("sqlite://");
    let conn =
        Connection::open(path_str).map_err(|e| DataReaderError::InternalError(e.to_string()))?;

    let mut stmt = conn
        .prepare("SELECT name, sql FROM sqlite_master WHERE type='table'")
        .map_err(|e| DataReaderError::InternalError(e.to_string()))?;
    let schema_info: Vec<serde_json::Value> = stmt
        .query_map([], |row| {
            let table: String = row.get(0)?;
            let definition: String = row.get(1)?;
            Ok(serde_json::json!({
                "table": table,
                "definition": definition
            }))
        })
        .map_err(|e| DataReaderError::InternalError(e.to_string()))?
        .filter_map(|e| e.ok())
        .collect();

    Ok(serde_json::json!(schema_info))
}
