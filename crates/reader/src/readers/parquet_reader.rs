use std::borrow::Cow;

use std::collections::{HashMap, HashSet};

use std::fs::File;

use std::path::Path;

use arrow::array::Array;

use chrono::{DateTime, NaiveDate};

use parquet::arrow::arrow_reader::ArrowReaderBuilder;

use parquet::file::reader::{FileReader, SerializedFileReader};

use schema::SchemaValue;

use serde::{Deserialize, Serialize};

use crate::error::DataReaderError;

use crate::reader_result::RecordStream;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParquetColumnInfo {
    pub name: String,
    pub physical_type: String,
    pub logical_type: String,
    pub nullable: bool,
    pub encodings: Vec<String>,
    pub compression: String,
    pub null_count: Option<u64>,
}

/// A single row of Parquet data, stored as an ordered vector of values.
/// This is an "Array-of-Arrays" approach, significantly more memory-efficient
/// than storing field names (HashMap keys) for every row.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParquetRow(pub Vec<SchemaValue<'static>>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParquetData {
    pub file_size: u64,
    pub num_rows: i64,
    pub column_schemas: Vec<ParquetColumnInfo>,
    pub sample_rows: Option<Vec<ParquetRow>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParquetDataForAnalysis {
    pub num_rows: i64,
    pub column_null_counts: HashMap<String, u64>,
    pub column_distinct_counts: HashMap<String, u64>,
    pub column_uniqueness_percentages: HashMap<String, f64>,
}

#[allow(dead_code)]
pub struct ParquetStream {
    reader: parquet::arrow::arrow_reader::ParquetRecordBatchReader,
    current_batch: Option<arrow::record_batch::RecordBatch>,
    current_row: usize,
    path: std::path::PathBuf,
}

impl ParquetStream {
    #[allow(dead_code)]
    pub fn new(
        reader: parquet::arrow::arrow_reader::ParquetRecordBatchReader,
        path: std::path::PathBuf,
    ) -> Self {
        Self { reader, current_batch: None, current_row: 0, path }
    }
}

impl Iterator for ParquetStream {
    type Item = Result<SchemaValue<'static>, DataReaderError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(batch) = &self.current_batch {
                if self.current_row < batch.num_rows() {
                    let mut row_map = HashMap::new();
                    let schema = batch.schema();
                    for col_idx in 0..batch.num_columns() {
                        let column = batch.column(col_idx);
                        let column_name = schema.field(col_idx).name();
                        let value = arrow_to_schema_value(column, self.current_row);
                        row_map.insert(Cow::Owned(column_name.to_string()), value);
                    }
                    self.current_row += 1;
                    return Some(Ok(SchemaValue::Object(row_map)));
                } else {
                    self.current_batch = None;
                    self.current_row = 0;
                }
            }

            match self.reader.next() {
                Some(Ok(batch)) => {
                    self.current_batch = Some(batch);
                    self.current_row = 0;
                }
                Some(Err(e)) => {
                    return Some(Err(DataReaderError::ParseError {
                        path: self.path.clone(),
                        source: Box::new(e),
                    }));
                }
                None => return None,
            }
        }
    }
}

#[allow(dead_code)]
pub fn read_parquet_stream(file_path: &Path) -> Result<RecordStream, DataReaderError> {
    let file = File::open(file_path)
        .map_err(|e| DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e })?;

    let builder = ArrowReaderBuilder::try_new(file).map_err(|e| DataReaderError::ParseError {
        path: file_path.to_path_buf(),
        source: Box::new(e),
    })?;

    let reader = builder.build().map_err(|e| DataReaderError::ParseError {
        path: file_path.to_path_buf(),
        source: Box::new(e),
    })?;

    Ok(Box::new(ParquetStream::new(reader, file_path.to_path_buf())))
}

fn arrow_to_schema_value(column: &dyn arrow::array::Array, row_idx: usize) -> SchemaValue<'static> {
    if column.is_null(row_idx) {
        return SchemaValue::Null;
    }

    use arrow::array::*;

    use arrow::datatypes::DataType as ArrowType;

    match column.data_type() {
        ArrowType::Int64 => SchemaValue::Integer(
            column.as_any().downcast_ref::<Int64Array>().unwrap().value(row_idx),
        ),
        ArrowType::Int32 => SchemaValue::Integer(
            column.as_any().downcast_ref::<Int32Array>().unwrap().value(row_idx) as i64,
        ),
        ArrowType::Int16 => SchemaValue::Integer(
            column.as_any().downcast_ref::<Int16Array>().unwrap().value(row_idx) as i64,
        ),
        ArrowType::Int8 => SchemaValue::Integer(
            column.as_any().downcast_ref::<Int8Array>().unwrap().value(row_idx) as i64,
        ),
        ArrowType::UInt64 => SchemaValue::Integer(
            column.as_any().downcast_ref::<UInt64Array>().unwrap().value(row_idx) as i64,
        ),
        ArrowType::UInt32 => SchemaValue::Integer(
            column.as_any().downcast_ref::<UInt32Array>().unwrap().value(row_idx) as i64,
        ),
        ArrowType::UInt16 => SchemaValue::Integer(
            column.as_any().downcast_ref::<UInt16Array>().unwrap().value(row_idx) as i64,
        ),
        ArrowType::UInt8 => SchemaValue::Integer(
            column.as_any().downcast_ref::<UInt8Array>().unwrap().value(row_idx) as i64,
        ),
        ArrowType::Float64 => SchemaValue::Float(
            column.as_any().downcast_ref::<Float64Array>().unwrap().value(row_idx),
        ),
        ArrowType::Float32 => SchemaValue::Float(
            column.as_any().downcast_ref::<Float32Array>().unwrap().value(row_idx) as f64,
        ),
        ArrowType::Boolean => SchemaValue::Boolean(
            column.as_any().downcast_ref::<BooleanArray>().unwrap().value(row_idx),
        ),
        ArrowType::Utf8 => SchemaValue::String(Cow::Owned(
            column.as_any().downcast_ref::<StringArray>().unwrap().value(row_idx).to_string(),
        )),
        ArrowType::LargeUtf8 => SchemaValue::String(Cow::Owned(
            column.as_any().downcast_ref::<LargeStringArray>().unwrap().value(row_idx).to_string(),
        )),
        ArrowType::Date32 => {
            let days = column.as_any().downcast_ref::<Date32Array>().unwrap().value(row_idx);
            NaiveDate::from_ymd_opt(1970, 1, 1)
                .and_then(|d| d.checked_add_days(chrono::Days::new(days as u64)))
                .map_or(SchemaValue::Null, |d| SchemaValue::String(Cow::Owned(d.to_string())))
        }
        ArrowType::Timestamp(_, _) => {
            let ts_ns = column
                .as_any()
                .downcast_ref::<TimestampNanosecondArray>()
                .map(|a| a.value(row_idx))
                .or_else(|| {
                    column
                        .as_any()
                        .downcast_ref::<TimestampMicrosecondArray>()
                        .map(|a| a.value(row_idx) * 1000)
                })
                .or_else(|| {
                    column
                        .as_any()
                        .downcast_ref::<TimestampMillisecondArray>()
                        .map(|a| a.value(row_idx) * 1_000_000)
                })
                .or_else(|| {
                    column
                        .as_any()
                        .downcast_ref::<TimestampSecondArray>()
                        .map(|a| a.value(row_idx) * 1_000_000_000)
                });

            ts_ns
                .map(|ns| {
                    SchemaValue::String(Cow::Owned(DateTime::from_timestamp_nanos(ns).to_string()))
                })
                .unwrap_or(SchemaValue::Unknown)
        }
        _ => SchemaValue::Unknown,
    }
}

pub fn read_parquet_data(
    file_path: &Path,
    head: Option<usize>,
) -> Result<ParquetData, DataReaderError> {
    let file = File::open(file_path)
        .map_err(|e| DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e })?;
    let file_metanc_obj = file
        .metadata()
        .map_err(|e| DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e })?;
    let file_size = file_metanc_obj.len();

    let reader = SerializedFileReader::new(file).map_err(|e| DataReaderError::ParseError {
        path: file_path.to_path_buf(),
        source: Box::new(e),
    })?;
    let metadata = reader.metadata();
    let num_rows = metadata.file_metadata().num_rows();

    let parquet_schema_descr = metadata.file_metadata().schema_descr();

    let mut column_encodings: HashMap<String, HashSet<String>> = HashMap::new();
    let mut column_compressions: HashMap<String, String> = HashMap::new();
    let mut column_null_counts: HashMap<String, u64> = HashMap::new();

    for row_group_meta in metadata.row_groups() {
        for column_chunk_meta in row_group_meta.columns() {
            let column_path = column_chunk_meta.column_path();
            let col_name = column_path.as_ref().last().unwrap_or(&"".to_string()).to_string();

            let encodings_for_chunk: Vec<String> =
                column_chunk_meta.encodings().map(|e| format!("{:?}", e)).collect();
            column_encodings.entry(col_name.clone()).or_default().extend(encodings_for_chunk);

            if !column_compressions.contains_key(&col_name) {
                column_compressions
                    .insert(col_name.clone(), format!("{:?}", column_chunk_meta.compression()));
            } else if column_compressions[&col_name]
                != format!("{:?}", column_chunk_meta.compression())
            {
                column_compressions.insert(col_name.clone(), "Mixed".to_string());
            }

            if let Some(stats) = column_chunk_meta.statistics() {
                *column_null_counts.entry(col_name.clone()).or_insert(0) +=
                    stats.null_count_opt().unwrap_or(0);
            }
        }
    }

    let mut column_schemas_info = Vec::new();
    for i in 0..parquet_schema_descr.num_columns() {
        let column_descr = parquet_schema_descr.column(i);
        let col_name = column_descr.name().to_string();

        let encodings_vec: Vec<String> =
            column_encodings.get(&col_name).map_or(vec![], |s| s.iter().cloned().collect());
        let compression_str =
            column_compressions.get(&col_name).map_or("Unknown".to_string(), |s| s.clone());
        let null_count = column_null_counts.get(&col_name).cloned();

        column_schemas_info.push(ParquetColumnInfo {
            name: col_name.clone(),
            physical_type: format!("{:?}", column_descr.physical_type()),
            logical_type: column_descr
                .logical_type_ref()
                .as_ref()
                .map(|t| format!("{:?}", t))
                .unwrap_or_else(|| "None".to_string()),
            nullable: column_descr.self_type().is_optional(),
            encodings: encodings_vec,
            compression: compression_str,
            null_count,
        });
    }

    let mut sample_rows: Option<Vec<ParquetRow>> = None;
    if let Some(num_rows_to_read) = head {
        let file_for_arrow = File::open(file_path).map_err(|e| DataReaderError::FileReadError {
            path: file_path.to_path_buf(),
            source: e,
        })?;
        let builder = ArrowReaderBuilder::try_new(file_for_arrow).map_err(|e| {
            DataReaderError::ParseError { path: file_path.to_path_buf(), source: Box::new(e) }
        })?;
        let mut reader = builder.build().map_err(|e| DataReaderError::ParseError {
            path: file_path.to_path_buf(),
            source: Box::new(e),
        })?;

        let mut rows_read = 0;
        let mut collected_rows: Vec<ParquetRow> = Vec::new();

        while let Some(batch) = reader.next().transpose().map_err(|e| {
            DataReaderError::ParseError { path: file_path.to_path_buf(), source: Box::new(e) }
        })? {
            for row_idx in 0..batch.num_rows() {
                if rows_read >= num_rows_to_read {
                    break;
                }
                let mut row_values = Vec::with_capacity(batch.num_columns());
                for col_idx in 0..batch.num_columns() {
                    let column = batch.column(col_idx);
                    row_values.push(arrow_to_schema_value(column, row_idx));
                }
                collected_rows.push(ParquetRow(row_values));
                rows_read += 1;
            }
            if rows_read >= num_rows_to_read {
                break;
            }
        }
        sample_rows = Some(collected_rows);
    }

    Ok(ParquetData { file_size, num_rows, column_schemas: column_schemas_info, sample_rows })
}

#[allow(dead_code)]
pub fn read_parquet_nc_for_analysis(
    file_path: &Path,
) -> Result<ParquetDataForAnalysis, DataReaderError> {
    let file = File::open(file_path)
        .map_err(|e| DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e })?;

    let reader_builder = ArrowReaderBuilder::try_new(file).map_err(|e| {
        DataReaderError::ParseError { path: file_path.to_path_buf(), source: Box::new(e) }
    })?;

    let num_rows = reader_builder.metadata().file_metadata().num_rows();
    let schema_ref = reader_builder.schema();
    let schema = schema_ref.clone();

    let mut arrow_reader = reader_builder.build().map_err(|e| DataReaderError::ParseError {
        path: file_path.to_path_buf(),
        source: Box::new(e),
    })?;

    let mut column_null_counts: HashMap<String, u64> = HashMap::new();
    let mut column_distinct_values: HashMap<String, std::collections::HashSet<String>> =
        HashMap::new();

    for field in schema.fields() {
        let col_name = field.name().to_string();
        column_null_counts.insert(col_name.clone(), 0);
        column_distinct_values.insert(col_name, std::collections::HashSet::new());
    }

    while let Some(record_batch) = arrow_reader.next().transpose().map_err(|e| {
        DataReaderError::ParseError { path: file_path.to_path_buf(), source: Box::new(e) }
    })? {
        for (idx, field) in schema.fields().iter().enumerate() {
            let column_name = field.name().to_string();
            let array = record_batch.column(idx);

            *column_null_counts.get_mut(&column_name).unwrap() += array.null_count() as u64;

            let distinct_set = column_distinct_values.get_mut(&column_name).unwrap();
            for i in 0..array.len() {
                if !array.is_null(i) {
                    let value = arrow_to_schema_value(array, i);
                    distinct_set.insert(format!("{:?}", value));
                }
            }
        }
    }

    let mut column_distinct_counts: HashMap<String, u64> = HashMap::new();
    let mut column_uniqueness_percentages: HashMap<String, f64> = HashMap::new();

    for (col_name, distinct_set) in column_distinct_values {
        let distinct_count = distinct_set.len() as u64;
        column_distinct_counts.insert(col_name.clone(), distinct_count);

        let null_count = *column_null_counts.get(&col_name).unwrap_or(&0);
        let non_null_count = num_rows.saturating_sub(null_count as i64) as f64;

        let uniqueness_percentage = if non_null_count > 0.0 {
            (distinct_count as f64 / non_null_count) * 100.0
        } else {
            0.0
        };
        column_uniqueness_percentages.insert(col_name, uniqueness_percentage);
    }

    Ok(ParquetDataForAnalysis {
        num_rows,
        column_null_counts,
        column_distinct_counts,
        column_uniqueness_percentages,
    })
}

#[allow(dead_code)]
pub fn read_full_parquet_content(file_path: &Path) -> Result<Vec<ParquetRow>, DataReaderError> {
    let file = File::open(file_path)
        .map_err(|e| DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e })?;
    let builder = ArrowReaderBuilder::try_new(file).map_err(|e| DataReaderError::ParseError {
        path: file_path.to_path_buf(),
        source: Box::new(e),
    })?;
    let mut reader = builder.build().map_err(|e| DataReaderError::ParseError {
        path: file_path.to_path_buf(),
        source: Box::new(e),
    })?;

    let mut all_rows: Vec<ParquetRow> = Vec::new();

    while let Some(batch) = reader.next().transpose().map_err(|e| DataReaderError::ParseError {
        path: file_path.to_path_buf(),
        source: Box::new(e),
    })? {
        for row_idx in 0..batch.num_rows() {
            let mut row_values = Vec::with_capacity(batch.num_columns());
            for col_idx in 0..batch.num_columns() {
                let column = batch.column(col_idx);
                row_values.push(arrow_to_schema_value(column, row_idx));
            }
            all_rows.push(ParquetRow(row_values));
        }
    }

    Ok(all_rows)
}
