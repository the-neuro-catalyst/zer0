use std::path::Path;

use serde::{Deserialize, Serialize}; // Added Serialize and Deserialize

use crate::error::DataReaderError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PdfData {
    pub content: String,
    pub first_lines: Option<Vec<String>>,

    pub page_count: Option<usize>, // Using Option because pdf_extract doesn't expose it directly
    pub line_count: usize,         // From extracted text
    pub total_size: u64,           // In bytes
}

pub fn read_pdf_text(file_path: &Path, head: Option<usize>) -> Result<PdfData, DataReaderError> {
    let num_lines_to_extract = head.unwrap_or(0);

    // 1. Read raw bytes first for fallback and memory-based extraction
    let bytes = std::fs::read(file_path)
        .map_err(|e| DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e })?;
    let total_size = bytes.len() as u64;

    // 2. Attempt extraction from memory with panic protection
    let bytes_for_extraction = bytes.clone();
    let content_result =
        std::panic::catch_unwind(move || pdf_extract::extract_text_from_mem(&bytes_for_extraction));

    // 3. Handle result with a robust fallback to lossy UTF-8
    let content = match content_result {
        Ok(Ok(text)) => text,
        _ => {
            // FALLBACK: If library fails/panics, show raw content as lossy string
            // This prevents "Inspection failed" errors and app crashes.
            let lossy_text = String::from_utf8_lossy(&bytes);
            format!(
                "[COMPLEX ENCODING DETECTED - SHOWING RAW STRUCTURE]\n\n{}",
                lossy_text.chars().take(5000).collect::<String>()
            )
        }
    };

    let line_count = content.lines().count();
    let first_lines: Option<Vec<String>> = if num_lines_to_extract > 0 {
        let lines: Vec<String> =
            content.lines().take(num_lines_to_extract).map(|s: &str| s.to_string()).collect();
        Some(lines)
    } else {
        None
    };

    Ok(PdfData { content, first_lines, page_count: None, line_count, total_size })
}
