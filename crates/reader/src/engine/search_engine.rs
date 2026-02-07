use crate::error::DataReaderError;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub file_path: String,
    pub line_number: usize,
    pub content: String,
}

pub async fn search_in_file(
    path: &Path,
    pattern: &str,
) -> Result<Vec<SearchResult>, DataReaderError> {
    let file = File::open(path)
        .map_err(|e| DataReaderError::FileReadError { path: path.to_path_buf(), source: e })?;
    let reader = BufReader::new(file);
    let mut results = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| DataReaderError::InternalError(e.to_string()))?;
        if line.contains(pattern) {
            results.push(SearchResult {
                file_path: path.to_string_lossy().into_owned(),
                line_number: index + 1,
                content: line.trim().to_string(),
            });
        }
    }

    Ok(results)
}
