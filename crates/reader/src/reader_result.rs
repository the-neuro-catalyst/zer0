use crate::analysis::security::SecretScanner;

use crate::error::DataReaderError;

pub use schema::SchemaValue;

use serde::{Deserialize, Serialize};

use std::fmt;

use std::iter::Iterator;

// Mock structures for gated features to maintain Enum consistency
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct GzipDataStub;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ImageDataStub;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct MarkdownDataStub {
    pub content: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct PdfDataStub {
    pub content: String,
    pub line_count: usize,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct SpreadsheetDataStub;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct SqliteDataStub;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct ZipDataStub;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct TomlDataStub;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct YamlDataStub;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct XmlDataStub {
    pub content: String,
    pub root_element: Option<String>,
    pub element_counts: std::collections::HashMap<String, usize>,
    pub first_lines: Option<Vec<String>>,
}

#[cfg(feature = "extra-docs")]
use crate::readers::gzip_reader::GzipData;
#[cfg(feature = "extra-docs")]
use crate::readers::image_reader::ImageData;

use crate::readers::json_reader::JsonData;
#[cfg(feature = "extra-docs")]
use crate::readers::md_reader::MarkdownData;

use crate::readers::parquet_reader::{ParquetData, ParquetDataForAnalysis};
#[cfg(feature = "extra-docs")]
use crate::readers::pdf_reader::PdfData;
#[cfg(feature = "extra-docs")]
use crate::readers::spreadsheet_reader::SpreadsheetData;
#[cfg(feature = "database")]
use crate::readers::sqlite_reader::SqliteData;

#[cfg(feature = "base-formats")]
use crate::readers::toml_reader::TomlData;

use crate::readers::txt_reader::TextData;

#[cfg(feature = "base-formats")]
use crate::readers::xml_reader::XmlData;

#[cfg(feature = "base-formats")]
use crate::readers::yaml_reader::YamlData;
#[cfg(feature = "extra-docs")]
use crate::readers::zip_reader::ZipData;

/// Type alias for record streams used across readers
pub type RecordStream =
    Box<dyn Iterator<Item = Result<SchemaValue<'static>, DataReaderError>> + Send>;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FileMetadata {
    pub size_bytes: u64,
    pub line_count: Option<usize>,
    pub information_density: Option<f64>,
    pub structural_depth: Option<usize>,
    pub compromised: bool,
}

impl FileMetadata {
    pub fn new(size_bytes: u64) -> Self {
        Self {
            size_bytes,
            line_count: None,
            information_density: None,
            structural_depth: None,
            compromised: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum DataReaderResult {
    Csv(crate::readers::csv_reader::CsvData, FileMetadata),
    #[cfg(feature = "extra-docs")]
    Gzip(GzipData, FileMetadata),
    #[cfg(not(feature = "extra-docs"))]
    Gzip(GzipDataStub, FileMetadata),

    #[cfg(feature = "extra-docs")]
    Image(ImageData, FileMetadata),
    #[cfg(not(feature = "extra-docs"))]
    Image(ImageDataStub, FileMetadata),

    Json(JsonData, FileMetadata),

    #[cfg(feature = "extra-docs")]
    Markdown(MarkdownData, FileMetadata),
    #[cfg(not(feature = "extra-docs"))]
    Markdown(MarkdownDataStub, FileMetadata),

    Parquet(ParquetData, FileMetadata),
    ParquetAnalysis(ParquetDataForAnalysis, FileMetadata),

    #[cfg(feature = "extra-docs")]
    Pdf(PdfData, FileMetadata),
    #[cfg(not(feature = "extra-docs"))]
    Pdf(PdfDataStub, FileMetadata),

    #[cfg(feature = "extra-docs")]
    Spreadsheet(SpreadsheetData, FileMetadata),
    #[cfg(not(feature = "extra-docs"))]
    Spreadsheet(SpreadsheetDataStub, FileMetadata),

    #[cfg(feature = "database")]
    Sqlite(SqliteData, FileMetadata),
    #[cfg(not(feature = "database"))]
    Sqlite(SqliteDataStub, FileMetadata),

    #[cfg(feature = "base-formats")]
    Toml(TomlData, FileMetadata),
    #[cfg(not(feature = "base-formats"))]
    Toml(TomlDataStub, FileMetadata),
    Text(TextData, FileMetadata),
    #[cfg(feature = "base-formats")]
    Xml(XmlData, FileMetadata),
    #[cfg(not(feature = "base-formats"))]
    Xml(XmlDataStub, FileMetadata),
    #[cfg(feature = "base-formats")]
    Yaml(YamlData, FileMetadata),
    #[cfg(not(feature = "base-formats"))]
    Yaml(YamlDataStub, FileMetadata),

    #[cfg(feature = "extra-docs")]
    Zip(ZipData, FileMetadata),
    #[cfg(not(feature = "extra-docs"))]
    Zip(ZipDataStub, FileMetadata),

    RawContent(String, FileMetadata),
    #[serde(skip)]
    Stream(#[allow(dead_code)] RecordStream, FileMetadata),
    DirectoryResults(Vec<(std::path::PathBuf, DataReaderResult)>, FileMetadata),
}

impl DataReaderResult {
    pub fn redact(&mut self) {
        match self {
            DataReaderResult::Csv(data, metadata) => {
                let mut compromised = false;
                for row in &mut data.data_rows {
                    if SecretScanner::redact_schema_value(row) {
                        compromised = true;
                    }
                }
                if compromised {
                    metadata.compromised = true;
                }
            }
            DataReaderResult::Text(data, metadata) => {
                let (redacted_content, found) = SecretScanner::redact(&data.content);
                if found {
                    data.content = redacted_content;
                    metadata.compromised = true;
                }
            }
            DataReaderResult::RawContent(content, metadata) => {
                let (redacted_content, found) = SecretScanner::redact(content);
                if found {
                    *content = redacted_content;
                    metadata.compromised = true;
                }
            }
            DataReaderResult::Json(data, metadata) => {
                let mut compromised = false;
                if SecretScanner::redact_schema_value(&mut data.value) {
                    compromised = true;
                }
                if compromised {
                    metadata.compromised = true;
                }
            }
            #[cfg(feature = "base-formats")]
            DataReaderResult::Toml(data, metadata) => {
                // TomlData uses serde_json::Value internally
                let mut compromised = false;
                if SecretScanner::redact_json_value(&mut data.value) {
                    compromised = true;
                }
                if compromised {
                    metadata.compromised = true;
                }
            }
            #[cfg(not(feature = "base-formats"))]
            DataReaderResult::Toml(_, _) => {}
            #[cfg(feature = "base-formats")]
            DataReaderResult::Yaml(data, metadata) => {
                // YamlData uses serde_json::Value internally
                let mut compromised = false;
                if SecretScanner::redact_json_value(&mut data.value) {
                    compromised = true;
                }
                if compromised {
                    metadata.compromised = true;
                }
            }
            #[cfg(not(feature = "base-formats"))]
            DataReaderResult::Yaml(_, _) => {}
            #[cfg(feature = "base-formats")]
            DataReaderResult::Xml(data, metadata) => {
                let (redacted_content, found) = SecretScanner::redact(&data.content);
                if found {
                    data.content = redacted_content;
                    metadata.compromised = true;
                }
            }
            #[cfg(not(feature = "base-formats"))]
            DataReaderResult::Xml(_, _) => {}
            #[cfg(feature = "extra-docs")]
            DataReaderResult::Markdown(data, metadata) => {
                let (redacted_content, found) = SecretScanner::redact(&data.content);
                if found {
                    data.content = redacted_content;
                    metadata.compromised = true;
                }
            }
            #[cfg(not(feature = "extra-docs"))]
            DataReaderResult::Markdown(_, _) => {}
            #[cfg(feature = "extra-docs")]
            DataReaderResult::Pdf(data, metadata) => {
                let (redacted_content, found) = SecretScanner::redact(&data.content);
                if found {
                    data.content = redacted_content;
                    metadata.compromised = true;
                }
            }
            #[cfg(not(feature = "extra-docs"))]
            DataReaderResult::Pdf(_, _) => {}
            // Other variants don't have redactable content or are binary
            _ => {}
        }
    }
    #[allow(dead_code)]
    pub fn get_metadata(&self) -> &FileMetadata {
        match self {
            DataReaderResult::Csv(_, m) => m,
            DataReaderResult::Gzip(_, m) => m,
            DataReaderResult::Image(_, m) => m,
            DataReaderResult::Json(_, m) => m,
            DataReaderResult::Markdown(_, m) => m,
            DataReaderResult::Parquet(_, m) => m,
            DataReaderResult::ParquetAnalysis(_, m) => m,
            DataReaderResult::Pdf(_, m) => m,
            DataReaderResult::Spreadsheet(_, m) => m,
            DataReaderResult::Sqlite(_, m) => m,
            DataReaderResult::Toml(_, m) => m,
            DataReaderResult::Text(_, m) => m,
            DataReaderResult::Xml(_, m) => m,
            DataReaderResult::Yaml(_, m) => m,
            DataReaderResult::Zip(_, m) => m,
            DataReaderResult::RawContent(_, m) => m,
            DataReaderResult::Stream(_, m) => m,
            DataReaderResult::DirectoryResults(_, m) => m,
        }
    }

    #[allow(dead_code)]
    pub fn get_content_preview(&self) -> String {
        match self {
            DataReaderResult::Text(data, _) => data.content.clone(),
            DataReaderResult::RawContent(content, _) => content.clone(),
            DataReaderResult::Csv(data, _) => {
                data.column_headers.join(",")
                    + "\n"
                    + &data
                        .data_rows
                        .iter()
                        .take(10)
                        .map(|row| format!("{}", row))
                        .collect::<Vec<_>>()
                        .join("\n")
            }
            DataReaderResult::Json(data, _) => serde_json::to_string_pretty(&data.value)
                .unwrap_or_else(|_| "Error serializing JSON".to_string()),
            #[cfg(feature = "base-formats")]
            DataReaderResult::Toml(data, _) => serde_json::to_string_pretty(&data.value)
                .unwrap_or_else(|_| "Error serializing TOML".to_string()),
            #[cfg(not(feature = "base-formats"))]
            DataReaderResult::Toml(_, _) => {
                "[TOML Data (base-formats feature not enabled)]".to_string()
            }
            #[cfg(feature = "base-formats")]
            DataReaderResult::Yaml(data, _) => serde_json::to_string_pretty(&data.value)
                .unwrap_or_else(|_| "Error serializing YAML".to_string()),
            #[cfg(not(feature = "base-formats"))]
            DataReaderResult::Yaml(_, _) => {
                "[YAML Data (base-formats feature not enabled)]".to_string()
            }
            #[cfg(feature = "base-formats")]
            DataReaderResult::Xml(data, _) => data.content.clone(),
            #[cfg(not(feature = "base-formats"))]
            DataReaderResult::Xml(_, _) => {
                "[XML Data (base-formats feature not enabled)]".to_string()
            }
            #[cfg(feature = "extra-docs")]
            DataReaderResult::Markdown(data, _) => data.content.clone(),
            #[cfg(feature = "extra-docs")]
            DataReaderResult::Pdf(data, _) => data.content.clone(),
            DataReaderResult::Parquet(data, _) => {
                // Return a summary of columns and row count
                format!(
                    "Parquet Data: {} rows, {} columns\nColumns: {:?}",
                    data.num_rows,
                    data.column_schemas.len(),
                    data.column_schemas.iter().map(|c| &c.name).collect::<Vec<_>>()
                )
            }
            DataReaderResult::Image(_, _) => "[Image Data Binary]".to_string(),
            DataReaderResult::Gzip(_, _) => "[Gzip Compressed Data]".to_string(),
            DataReaderResult::Zip(_, _) => "[Zip Archive Data]".to_string(),
            DataReaderResult::Sqlite(_, _) => "[SQLite Database]".to_string(),
            DataReaderResult::Spreadsheet(_, _) => "[Spreadsheet Data]".to_string(),
            _ => "Preview not available for this format".to_string(),
        }
    }
}

impl fmt::Debug for DataReaderResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataReaderResult::Csv(_, m) => f.debug_tuple("Csv").field(m).finish(),
            DataReaderResult::Gzip(_, m) => f.debug_tuple("Gzip").field(m).finish(),
            DataReaderResult::Image(_, m) => f.debug_tuple("Image").field(m).finish(),
            DataReaderResult::Json(_, m) => f.debug_tuple("Json").field(m).finish(),
            DataReaderResult::Markdown(_, m) => f.debug_tuple("Markdown").field(m).finish(),
            DataReaderResult::Parquet(_, m) => f.debug_tuple("Parquet").field(m).finish(),
            DataReaderResult::ParquetAnalysis(_, m) => {
                f.debug_tuple("ParquetAnalysis").field(m).finish()
            }
            DataReaderResult::Pdf(_, m) => f.debug_tuple("Pdf").field(m).finish(),
            DataReaderResult::Spreadsheet(_, m) => f.debug_tuple("Spreadsheet").field(m).finish(),
            DataReaderResult::Sqlite(_, m) => f.debug_tuple("Sqlite").field(m).finish(),
            DataReaderResult::Toml(_, m) => f.debug_tuple("Toml").field(m).finish(),
            DataReaderResult::Text(_, m) => f.debug_tuple("Text").field(m).finish(),
            DataReaderResult::Xml(_, m) => f.debug_tuple("Xml").field(m).finish(),
            DataReaderResult::Yaml(_, m) => f.debug_tuple("Yaml").field(m).finish(),
            DataReaderResult::Zip(_, m) => f.debug_tuple("Zip").field(m).finish(),
            DataReaderResult::RawContent(_, m) => f.debug_tuple("RawContent").field(m).finish(),
            DataReaderResult::Stream(_, m) => f.debug_tuple("Stream").field(m).finish(),
            DataReaderResult::DirectoryResults(_, m) => {
                f.debug_tuple("DirectoryResults").field(m).finish()
            }
        }
    }
}
