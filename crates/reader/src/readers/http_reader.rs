use crate::error::DataReaderError;
use crate::reader_result::{DataReaderResult, FileMetadata};
use crate::readers::json_reader::convert_json_value_to_schema_value;
use reqwest::Client;

pub async fn read_http_data(url: &str) -> Result<DataReaderResult, DataReaderError> {
    let client = Client::new();
    let response =
        client.get(url).send().await.map_err(|e| DataReaderError::InternalError(e.to_string()))?;

    let status = response.status();
    if !status.is_success() {
        return Err(DataReaderError::InternalError(format!("HTTP Error: {}", status)));
    }

    let headers = response.headers().clone();
    let content_length = headers
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);

    let content_type = headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    let bytes =
        response.bytes().await.map_err(|e| DataReaderError::InternalError(e.to_string()))?;
    let content = String::from_utf8_lossy(&bytes).to_string();

    let mut metadata = FileMetadata::new(content_length);
    // Apply redacted metadata if needed later, for now we just create it
    let _ = &mut metadata;

    if content_type.contains("json") {
        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&content) {
            return Ok(DataReaderResult::Json(
                crate::readers::json_reader::JsonData {
                    value: convert_json_value_to_schema_value(json_val),
                    line_count: Some(0),
                    first_lines: None,
                    inferred_schema: None,
                },
                metadata,
            ));
        }
    }

    Ok(DataReaderResult::RawContent(content, metadata))
}
