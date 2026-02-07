use std::borrow::Cow;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::ops::merger::merge_data_types;

use crate::types::data_type::DataType;

use std::fmt;

/// Represents the actual data values, mirroring the DataType enum but holding the data.
/// Uses Cow<'a, str> for strings to allow for zero-copy where possible.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SchemaValue<'a> {
    String(Cow<'a, str>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
    Array(Vec<SchemaValue<'a>>),
    Object(HashMap<Cow<'a, str>, SchemaValue<'a>>),
    Union(Vec<SchemaValue<'a>>),
    Unknown,
}

impl<'a> fmt::Display for SchemaValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchemaValue::String(s) => write!(f, "{}", s),
            SchemaValue::Integer(i) => write!(f, "{}", i),
            SchemaValue::Float(fl) => write!(f, "{}", fl),
            SchemaValue::Boolean(b) => write!(f, "{}", b),
            SchemaValue::Null => write!(f, "null"),
            SchemaValue::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            SchemaValue::Object(obj) => {
                write!(f, "{{")?;
                for (i, (k, v)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            SchemaValue::Union(_) => write!(f, "<Union>"),
            SchemaValue::Unknown => write!(f, "unknown"),
        }
    }
}

impl<'a> From<SchemaValue<'a>> for DataType {
    fn from(val: SchemaValue<'a>) -> Self {
        match val {
            SchemaValue::String(_) => DataType::String,
            SchemaValue::Integer(_) => DataType::Integer,
            SchemaValue::Float(_) => DataType::Float,
            SchemaValue::Boolean(_) => DataType::Boolean,
            SchemaValue::Null => DataType::Null,
            SchemaValue::Array(arr) => {
                let inner_types: Vec<DataType> = arr.into_iter().map(|v| v.into()).collect();
                if inner_types.is_empty() {
                    DataType::Array(Box::new(DataType::Unknown))
                } else {
                    let merged_type =
                        inner_types.into_iter().fold(DataType::Unknown, merge_data_types);
                    DataType::Array(Box::new(merged_type))
                }
            }
            SchemaValue::Object(obj) => {
                let mut schema_map = HashMap::new();
                for (k, v) in obj {
                    schema_map.insert(k.into_owned(), v.into());
                }
                DataType::Object(schema_map)
            }
            SchemaValue::Union(vec_val) => {
                let types: Vec<DataType> = vec_val.into_iter().map(|v| v.into()).collect();
                DataType::Union(types)
            }
            SchemaValue::Unknown => DataType::Unknown,
        }
    }
}
