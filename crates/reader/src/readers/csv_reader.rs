use std::borrow::Cow;

use std::collections::HashMap;

use std::fs::File;

use std::io::{self, BufRead};

use std::path::Path;

use schema::{merge_data_types, DataType, SchemaValue};

use serde::{Deserialize, Serialize};

use crate::error::DataReaderError;

use crate::reader_result::RecordStream;

#[derive(Debug, Serialize, Deserialize)]
pub struct CsvData {
    pub file_size: u64,
    pub num_rows: u64,
    pub column_headers: Vec<String>,
    /// Stores rows as an ordered list of values (Array-of-Arrays approach).
    /// Each SchemaValue here is expected to be a SchemaValue::Array.
    pub data_rows: Vec<SchemaValue<'static>>,
    pub total_size: u64,
    pub first_lines: Option<Vec<String>>,
    pub inferred_schema: Option<HashMap<String, DataType>>,
}

pub fn read_csv_stream(file_path: &Path) -> Result<(Vec<String>, RecordStream), DataReaderError> {
    let file = File::open(file_path)
        .map_err(|e| DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e })?;

    let decoder = crate::readers::charset::get_decoded_reader(file)
        .map_err(|e| DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e })?;
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(decoder);

    let headers = rdr
        .headers()
        .map_err(|e| DataReaderError::ParseError {
            path: file_path.to_path_buf(),
            source: Box::new(e),
        })?
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let path_clone = file_path.to_path_buf();

    let stream = rdr.into_records().map(move |result| {
        let record = result.map_err(|e| DataReaderError::ParseError {
            path: path_clone.clone(),
            source: Box::new(e),
        })?;

        let mut row_values = Vec::with_capacity(record.len());
        for field in record.iter() {
            let field_val = if field.is_empty() {
                SchemaValue::Null
            } else if let Ok(i_val) = field.parse::<i64>() {
                SchemaValue::Integer(i_val)
            } else if let Ok(f_val) = field.parse::<f64>() {
                SchemaValue::Float(f_val)
            } else if let Ok(b_val) = field.parse::<bool>() {
                SchemaValue::Boolean(b_val)
            } else {
                SchemaValue::String(Cow::Owned(field.to_string()))
            };
            row_values.push(field_val);
        }
        Ok(SchemaValue::Array(row_values))
    });

    Ok((headers, Box::new(stream)))
}

pub fn read_csv_data(file_path: &Path, head: Option<usize>) -> Result<CsvData, DataReaderError> {
    let num_lines_to_extract = head.unwrap_or(0);

    let base_metadata = std::fs::metadata(file_path)
        .map_err(|e| DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e })?;
    let file_size = base_metadata.len();

    let (headers, stream) = read_csv_stream(file_path)?;

    let mut first_lines = Vec::new();
    let mut records = Vec::new();
    let mut schema_map: HashMap<String, DataType> = HashMap::new();

    for (row_idx, result) in stream.enumerate() {
        let row: SchemaValue = result?;

        // Capture first lines for preview if requested
        if row_idx < num_lines_to_extract {
            if let SchemaValue::Array(ref vals) = row {
                let line = vals.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(",");
                first_lines.push(line);
            }
        }

        if let SchemaValue::Array(ref vals) = row {
            for (i, value) in vals.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    let current_type: DataType = value.clone().into();
                    schema_map
                        .entry(header.clone())
                        .and_modify(|t| *t = merge_data_types(t.clone(), current_type.clone()))
                        .or_insert(current_type);
                }
            }
        }

        records.push(row);
    }

    let num_rows = records.len() as u64;

    Ok(CsvData {
        file_size,
        num_rows,
        column_headers: headers,
        data_rows: records,
        total_size: file_size,
        first_lines: if first_lines.is_empty() { None } else { Some(first_lines) },
        inferred_schema: Some(schema_map),
    })
}

pub fn get_csv_raw_content(
    file_path: &Path,
    head: Option<usize>,
) -> Result<String, DataReaderError> {
    let data = read_csv_data(file_path, head)?;

    // Convert back to structured objects for the raw content API (backward compatibility)
    let mut compatible_records = Vec::new();
    for row in data.data_rows {
        if let SchemaValue::Array(vals) = row {
            let mut map = HashMap::new();
            for (i, val) in vals.into_iter().enumerate() {
                if let Some(header) = data.column_headers.get(i) {
                    map.insert(Cow::Owned(header.clone()), val);
                }
            }
            compatible_records.push(SchemaValue::Object(map));
        }
    }

    serde_json::to_string_pretty(&compatible_records).map_err(|e| {
        DataReaderError::InternalError(format!(
            "Failed to serialize CSV raw content to JSON: {}",
            e
        ))
    })
}

#[allow(dead_code)]
pub fn read_csv_raw_lines(
    file_path: &Path,
    limit: Option<usize>,
) -> Result<Vec<String>, DataReaderError> {
    let file = File::open(file_path)
        .map_err(|e| DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e })?;

    let decoder = crate::readers::charset::get_decoded_reader(file)
        .map_err(|e| DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e })?;

    let reader = io::BufReader::new(decoder);
    let lines = if let Some(l) = limit {
        reader.lines().take(l).map_while(Result::ok).collect()
    } else {
        reader.lines().map_while(Result::ok).collect()
    };

    Ok(lines)
}
