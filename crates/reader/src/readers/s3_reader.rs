use std::io::Write;

use std::path::Path;

use aws_sdk_s3::Client;

use tempfile::NamedTempFile;

use crate::engine::file_engine::{get_file_format, read_file_to_data};

use crate::error::DataReaderError;

use crate::reader_result::DataReaderResult;

#[allow(dead_code)]
pub async fn read_s3_data(
    bucket: &str,
    key: &str,
    _region: Option<String>,
) -> Result<DataReaderResult, DataReaderError> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    let mut output = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| DataReaderError::InternalError(format!("S3 GetObject error: {}", e)))?;

    // Create a temp file to store the stream.
    // Even if we have to write to disk, streaming it chunk-by-chunk prevents memory exhaustion.
    let mut temp_file = NamedTempFile::new().map_err(|e| {
        DataReaderError::InternalError(format!("Failed to create temp file: {}", e))
    })?;

    while let Some(chunk_res) = output.body.next().await {
        let chunk = chunk_res
            .map_err(|e| DataReaderError::InternalError(format!("S3 Streaming error: {}", e)))?;
        temp_file.write_all(&chunk).map_err(|e| {
            DataReaderError::InternalError(format!("Failed to write chunk to temp file: {}", e))
        })?;
    }

    let temp_path = temp_file.path();
    let format = get_file_format(Path::new(key));

    // Reuse existing file reading logic
    let result = read_file_to_data(temp_path, None, format).await?;

    Ok(result)
}
