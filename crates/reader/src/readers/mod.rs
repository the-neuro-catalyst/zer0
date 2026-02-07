#![allow(dead_code)]
// crates/reader/src/readers/mod.rs
pub mod charset;

#[cfg(feature = "base-formats")]
pub mod csv_reader;
#[cfg(not(feature = "base-formats"))]
pub mod csv_reader {

    use crate::error::DataReaderError;

    use schema::DataType;

    use schema::SchemaValue;

    use serde::{Deserialize, Serialize};

    use std::collections::HashMap;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct CsvData {
        pub file_size: u64,
        pub num_rows: u64,
        pub column_headers: Vec<String>,
        pub data_rows: Vec<SchemaValue<'static>>,
        pub total_size: u64,
        pub first_lines: Option<Vec<String>>,
        pub inferred_schema: Option<HashMap<String, DataType>>,
    }

    pub async fn read_csv_stream(
        _path: &std::path::Path,
    ) -> Result<(Vec<String>, Box<dyn std::any::Any>), DataReaderError> {
        Err(DataReaderError::InternalError("CSV feature disabled".to_string()))
    }
    pub fn read_csv_data(
        _path: &std::path::Path,
        _head: Option<usize>,
    ) -> Result<CsvData, DataReaderError> {
        Err(DataReaderError::InternalError("CSV feature disabled".to_string()))
    }
    pub fn get_csv_raw_content(
        _path: &std::path::Path,
        _head: Option<usize>,
    ) -> Result<String, DataReaderError> {
        Err(DataReaderError::InternalError("CSV feature disabled".to_string()))
    }
}

#[cfg(feature = "extra-docs")]
pub mod gzip_reader;
#[cfg(feature = "cloud")]
pub mod http_reader;
#[cfg(feature = "extra-docs")]
pub mod image_reader;
pub mod json_reader;
#[cfg(feature = "streaming")]
pub mod kafka_reader;
#[cfg(feature = "extra-docs")]
pub mod md_reader;
#[cfg(feature = "database")]
pub mod mysql_reader;

#[cfg(feature = "analytics")]
pub mod parquet_reader;
#[cfg(not(feature = "analytics"))]
pub mod parquet_reader {

    use crate::error::DataReaderError;

    use schema::SchemaValue;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ParquetData {
        pub num_rows: i64,
        pub file_size: u64,
        pub column_schemas: Vec<ParquetColumnSchema>,
        pub sample_rows: Option<Vec<ParquetRow>>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ParquetColumnSchema {
        pub name: String,
        pub physical_type: String,
        pub logical_type: String,
        pub nullable: bool,
        pub encodings: Vec<String>,
        pub compression: String,
        pub null_count: Option<u64>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ParquetRow(pub Vec<SchemaValue<'static>>);

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ParquetDataForAnalysis {
        pub num_rows: i64,
    }

    pub fn read_parquet_data(
        _p: &std::path::Path,
        _h: Option<usize>,
    ) -> Result<ParquetData, DataReaderError> {
        Err(DataReaderError::InternalError("Analytics feature disabled".to_string()))
    }
    #[allow(dead_code)]
    pub fn read_full_parquet_content(
        _p: &std::path::Path,
    ) -> Result<Vec<ParquetRow>, DataReaderError> {
        Err(DataReaderError::InternalError("Analytics feature disabled".to_string()))
    }
    #[allow(dead_code)]
    pub fn read_parquet_stream(
        _p: &std::path::Path,
    ) -> Result<Box<dyn std::any::Any>, DataReaderError> {
        Err(DataReaderError::InternalError("Analytics feature disabled".to_string()))
    }
    #[allow(dead_code)]
    pub fn read_parquet_nc_for_analysis(
        _p: &std::path::Path,
    ) -> Result<ParquetDataForAnalysis, DataReaderError> {
        Err(DataReaderError::InternalError("Analytics feature disabled".to_string()))
    }
}

#[cfg(feature = "extra-docs")]
pub mod pdf_reader;
#[cfg(feature = "database")]
pub mod postgresql_reader;
#[cfg(feature = "streaming")]
pub mod rabbitmq_reader;
#[cfg(feature = "cloud")]
pub mod s3_reader;
#[cfg(feature = "extra-docs")]
pub mod spreadsheet_reader;
#[cfg(feature = "database")]
pub mod sqlite_reader;

#[cfg(feature = "base-formats")]
pub mod toml_reader;
#[cfg(not(feature = "base-formats"))]
pub mod toml_reader {
    pub fn read_toml_value(
        _p: &std::path::Path,
        _h: Option<usize>,
    ) -> Result<Box<dyn std::any::Any>, crate::error::DataReaderError> {
        Err(crate::error::DataReaderError::InternalError("TOML feature disabled".to_string()))
    }
    pub fn get_toml_raw_content(
        _p: &std::path::Path,
        _h: Option<usize>,
    ) -> Result<String, crate::error::DataReaderError> {
        Err(crate::error::DataReaderError::InternalError("TOML feature disabled".to_string()))
    }
}

pub mod txt_reader;

#[cfg(feature = "base-formats")]
pub mod xml_reader;
#[cfg(not(feature = "base-formats"))]
pub mod xml_reader {
    pub fn read_xml_content(
        _p: &std::path::Path,
        _h: Option<usize>,
    ) -> Result<Box<dyn std::any::Any>, crate::error::DataReaderError> {
        Err(crate::error::DataReaderError::InternalError("XML feature disabled".to_string()))
    }
    pub fn create_xml_stream(
        _p: &std::path::Path,
    ) -> Result<Box<dyn std::any::Any>, crate::error::DataReaderError> {
        Err(crate::error::DataReaderError::InternalError("XML feature disabled".to_string()))
    }
}

#[cfg(feature = "base-formats")]
pub mod yaml_reader;
#[cfg(not(feature = "base-formats"))]
pub mod yaml_reader {
    pub fn read_yaml_value(
        _p: &std::path::Path,
        _h: Option<usize>,
    ) -> Result<Box<dyn std::any::Any>, crate::error::DataReaderError> {
        Err(crate::error::DataReaderError::InternalError("YAML feature disabled".to_string()))
    }
    pub fn get_yaml_raw_content(
        _p: &std::path::Path,
        _h: Option<usize>,
    ) -> Result<String, crate::error::DataReaderError> {
        Err(crate::error::DataReaderError::InternalError("YAML feature disabled".to_string()))
    }
}

#[cfg(feature = "extra-docs")]
pub mod zip_reader;
