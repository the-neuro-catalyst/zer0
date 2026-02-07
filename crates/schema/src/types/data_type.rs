use std::collections::HashMap;

use std::fmt;

use serde::{Deserialize, Serialize};

/// Represents the possible data types in the ZERO ecosystem.
/// Supports basic scalar types, complex nested types, and logic unions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    /// UTF-8 encoded string data.
    String,
    /// General numeric data (can be integer or float).
    Number,
    /// Signed 64-bit integer data.
    Integer,
    /// 64-bit floating point data.
    Float,
    /// Boolean (true/false) data.
    Boolean,
    /// Represents a missing or null value.
    Null,
    /// A collection of elements of a specific DataType.
    Array(Box<DataType>),
    /// A structured mapping of field names to their respective DataType.
    Object(HashMap<String, DataType>),
    /// Represents multiple possible types for a single field.
    Union(Vec<DataType>),
    /// Used when the type cannot be determined.
    Unknown,
}

pub type Schema = HashMap<String, DataType>;

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::String => write!(f, "String"),
            DataType::Number => write!(f, "Number"),
            DataType::Integer => write!(f, "Integer"),
            DataType::Float => write!(f, "Float"),
            DataType::Boolean => write!(f, "Boolean"),
            DataType::Null => write!(f, "Null"),
            DataType::Array(t) => write!(f, "Array<{}>", t),
            DataType::Object(m) => {
                write!(f, "Object{{")?;
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            DataType::Union(v) => {
                write!(f, "Union<")?;
                for (i, t) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ">")?;
                Ok(())
            }
            DataType::Unknown => write!(f, "Unknown"),
        }
    }
}
