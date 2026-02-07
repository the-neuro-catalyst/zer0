use std::borrow::Cow;

use std::collections::HashMap;

use std::fs::File;

use std::path::Path;

use schema::{DataType, Schema, SchemaValue, merge_data_types}; /* MODIFIED: Added
 * SchemaValue, Schema */

use serde::{Deserialize, Serialize};

use serde_json::{self, Value}; // Keep serde_json::Value for parsing from string initially

use crate::error::DataReaderError;

use crate::reader_result::RecordStream;

// REMOVED JsonSchema struct

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonData {
    pub value: SchemaValue<'static>, // MODIFIED: Changed to SchemaValue
    pub first_lines: Option<Vec<String>>,
    pub inferred_schema: Option<Schema>, // MODIFIED: Changed to Schema
    pub line_count: Option<usize>,
}

fn infer_json_type(value: &SchemaValue) -> DataType {
    // MODIFIED: input type
    match value {
        SchemaValue::Null => DataType::Null,
        SchemaValue::Boolean(_) => DataType::Boolean,
        SchemaValue::Integer(_) => DataType::Integer, // Directly map Integer
        SchemaValue::Float(_) => DataType::Float,     // Directly map Float
        SchemaValue::String(_) => DataType::String,
        SchemaValue::Array(arr) => {
            if arr.is_empty() {
                DataType::Array(Box::new(DataType::Unknown))
            } else {
                let mut element_type = infer_json_type(&arr[0]);
                for item in arr.iter().skip(1) {
                    element_type = merge_data_types(element_type, infer_json_type(item));
                }
                DataType::Array(Box::new(element_type))
            }
        }
        SchemaValue::Object(obj) => {
            let mut properties = HashMap::new();
            for (key_cow, val) in obj {
                properties.insert(key_cow.to_string(), infer_json_type(val)); // Convert Cow to String for Schema keys
            }
            DataType::Object(properties)
        }
        SchemaValue::Union(v) => {
            let types: Vec<DataType> = v.iter().map(|val| infer_json_type(val)).collect();
            DataType::Union(types)
        }
        SchemaValue::Unknown => DataType::Unknown,
    }
}

fn infer_json_schema(value: &SchemaValue) -> Schema {
    // MODIFIED: input type and return type
    let mut schema_map = HashMap::new();
    if let SchemaValue::Object(obj) = value {
        for (key_cow, val) in obj {
            schema_map.insert(key_cow.to_string(), infer_json_type(val));
        }
    } else {
        // If the top-level is not an object, it's a schema for a single value.
        // This case might need further refinement depending on how single-value schemas are
        // handled. For now, it will return an empty schema if not an object.
        // Or, we could wrap it under a generic key like "value": infer_json_type(value)
    }
    schema_map
}

// Removed merge_json_schemas function

// Helper function to convert serde_json::Value to SchemaValue<'static>
pub fn convert_json_value_to_schema_value(value: serde_json::Value) -> SchemaValue<'static> {
    match value {
        serde_json::Value::Null => SchemaValue::Null,
        serde_json::Value::Bool(b) => SchemaValue::Boolean(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                SchemaValue::Integer(i)
            } else if let Some(f) = n.as_f64() {
                SchemaValue::Float(f)
            } else {
                SchemaValue::Unknown // Should not happen for valid numbers
            }
        }
        serde_json::Value::String(s) => SchemaValue::String(Cow::Owned(s)),
        serde_json::Value::Array(arr) => {
            SchemaValue::Array(arr.into_iter().map(convert_json_value_to_schema_value).collect())
        }
        serde_json::Value::Object(obj) => SchemaValue::Object(
            obj.into_iter()
                .map(|(k, v)| (Cow::Owned(k), convert_json_value_to_schema_value(v)))
                .collect(),
        ),
    }
}

pub fn read_json_stream(file_path: &Path) -> Result<RecordStream, DataReaderError> {
    // MODIFIED: return type
    let is_jsonl = file_path.extension().is_some_and(|ext| ext == "jsonl");
    let file = File::open(file_path)
        .map_err(|e| DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e })?;
    let path_clone = file_path.to_path_buf();
    let decoder = crate::readers::charset::get_decoded_reader(file)
        .map_err(|e| DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e })?;

    if is_jsonl {
        use std::io::{BufRead, BufReader};
        let reader = BufReader::new(decoder);
        let stream = reader.lines().filter_map(move |line_res| match line_res {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    match serde_json::from_str::<Value>(trimmed) {
                        Ok(v) => Some(Ok(convert_json_value_to_schema_value(v))), // MODIFIED: Convert to SchemaValue
                        Err(e) => Some(Err(DataReaderError::ParseError {
                            path: path_clone.clone(),
                            source: Box::new(e),
                        })),
                    }
                }
            }
            Err(e) => {
                Some(Err(DataReaderError::FileReadError { path: path_clone.clone(), source: e }))
            }
        });
        Ok(Box::new(stream))
    } else {
        use std::io::BufReader;
        let reader = BufReader::new(decoder);
        let stream =
            serde_json::Deserializer::from_reader(reader).into_iter::<Value>().map(move |res| {
                res.map_err(|e| DataReaderError::ParseError {
                    path: path_clone.clone(),
                    source: Box::new(e),
                })
                .map(convert_json_value_to_schema_value) // MODIFIED: Convert to SchemaValue
            });
        Ok(Box::new(stream))
    }
}

pub fn read_json_value(file_path: &Path, head: Option<usize>) -> Result<JsonData, DataReaderError> {
    let num_lines_to_extract = head.unwrap_or(0);
    let is_jsonl = file_path.extension().is_some_and(|ext| ext == "jsonl");

    let stream = read_json_stream(file_path)?;
    let mut values: Vec<SchemaValue<'static>> = Vec::new(); // MODIFIED: stores SchemaValue

    let mut inferred_schema: Option<Schema> = None; // MODIFIED: uses Schema

    for value_result in stream {
        let value = value_result?;
        // infer_json_schema takes &SchemaValue and returns Schema (HashMap<String, DataType>)
        let current_schema_map = if let SchemaValue::Object(_) = &value {
            infer_json_schema(&value)
        } else {
            // If the top-level JSON is not an object, we infer the type of the value itself.
            let mut map = HashMap::new();
            map.insert("value".to_string(), value.clone().into()); // Convert SchemaValue to DataType
            map
        };

        inferred_schema = match inferred_schema {
            Some(mut prev_schema) => {
                for (k, v) in current_schema_map {
                    prev_schema
                        .entry(k)
                        .and_modify(|t| *t = merge_data_types(t.clone(), v.clone()))
                        .or_insert(v);
                }
                Some(prev_schema)
            }
            None => Some(current_schema_map),
        };
        values.push(value);
    }

    let first_lines = if num_lines_to_extract > 0 {
        use std::io::{BufRead, BufReader};
        let file = File::open(file_path).map_err(|e| DataReaderError::FileReadError {
            path: file_path.to_path_buf(),
            source: e,
        })?;
        let decoder = crate::readers::charset::get_decoded_reader(file).map_err(|e| {
            DataReaderError::FileReadError { path: file_path.to_path_buf(), source: e }
        })?;
        let reader = BufReader::new(decoder);
        let lines: Vec<String> =
            reader.lines().take(num_lines_to_extract).filter_map(|l| l.ok()).collect();
        if !lines.is_empty() { Some(lines) } else { None }
    } else {
        None
    };

    let final_value = if values.len() == 1 && !is_jsonl {
        values.into_iter().next().unwrap()
    } else {
        SchemaValue::Array(values) // MODIFIED: Creates SchemaValue::Array
    };

    let line_count = match &final_value {
        SchemaValue::Array(arr) => Some(arr.len()),
        _ => None,
    };

    Ok(JsonData { value: final_value, first_lines, inferred_schema, line_count })
}

// Helper function to convert SchemaValue to serde_json::Value
fn convert_schema_value_to_json_value(value: SchemaValue<'static>) -> serde_json::Value {
    match value {
        SchemaValue::Null => serde_json::Value::Null,
        SchemaValue::Boolean(b) => serde_json::Value::Bool(b),
        SchemaValue::Integer(i) => serde_json::Value::Number(serde_json::Number::from(i)),
        SchemaValue::Float(f) => serde_json::Value::Number(
            serde_json::Number::from_f64(f).unwrap_or_else(|| serde_json::Number::from(0)),
        ),
        SchemaValue::String(s) => serde_json::Value::String(s.into_owned()),
        SchemaValue::Array(arr) => serde_json::Value::Array(
            arr.into_iter().map(convert_schema_value_to_json_value).collect(),
        ),
        SchemaValue::Object(obj) => serde_json::Value::Object(
            obj.into_iter()
                .map(|(k, v)| (k.into_owned(), convert_schema_value_to_json_value(v)))
                .collect(),
        ),
        SchemaValue::Union(v) => {
            // For Union, we might choose to serialize as an array of its contained values,
            // or just the first non-null value, or return Unknown if no clear representation.
            // For simplicity, let's try to convert the first non-Null value.
            if let Some(val) = v.into_iter().find(|val| !matches!(val, SchemaValue::Null)) {
                convert_schema_value_to_json_value(val)
            } else {
                serde_json::Value::Null
            }
        }
        SchemaValue::Unknown => serde_json::Value::Null, // Represent Unknown as Null in JSON
    }
}

pub fn get_json_raw_content(
    file_path: &Path,
    head: Option<usize>,
) -> Result<String, DataReaderError> {
    let json_data = read_json_value(file_path, head)?;

    let json_value = convert_schema_value_to_json_value(json_data.value); // Convert SchemaValue to serde_json::Value

    serde_json::to_string_pretty(&json_value)
        .map_err(|e| DataReaderError::InternalError(format!("Failed to serialize JSON: {}", e)))
}
