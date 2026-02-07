use schema::SchemaValue;

#[cfg(feature = "base-formats")]
use crate::readers::xml_reader::{XmlSchema, XmlSchemaType};

/// Calculates the structural depth of a SchemaValue.
pub fn calculate_schema_depth(value: &SchemaValue) -> usize {
    match value {
        SchemaValue::Object(map) => {
            map.values().map(calculate_schema_depth).max().map(|d| d + 1).unwrap_or(1)
        }
        SchemaValue::Array(arr) => {
            arr.iter().map(calculate_schema_depth).max().map(|d| d + 1).unwrap_or(1)
        }
        SchemaValue::Union(variants) => {
            variants.iter().map(calculate_schema_depth).max().unwrap_or(0)
        }
        _ => 0,
    }
}

/// Calculates the structural depth of a serde_json::Value.
#[allow(dead_code)]
pub fn calculate_json_depth(value: &serde_json::Value) -> usize {
    match value {
        serde_json::Value::Object(map) => {
            map.values().map(calculate_json_depth).max().map(|d| d + 1).unwrap_or(1)
        }
        serde_json::Value::Array(arr) => {
            arr.iter().map(calculate_json_depth).max().map(|d| d + 1).unwrap_or(1)
        }
        _ => 0,
    }
}

#[cfg(feature = "base-formats")]
/// Calculates the structural depth of an XML schema.
pub fn calculate_xml_depth(schema: &XmlSchema) -> usize {
    schema
        .children
        .values()
        .map(|child_type| match child_type {
            XmlSchemaType::Element(s) => calculate_xml_depth(s),
            XmlSchemaType::Array(s) => calculate_xml_depth(s),
            XmlSchemaType::Union(variants) => variants
                .iter()
                .map(|v| match v {
                    XmlSchemaType::Element(s) => calculate_xml_depth(s),
                    XmlSchemaType::Array(s) => calculate_xml_depth(s),
                    _ => 0,
                })
                .max()
                .unwrap_or(0),
            _ => 0,
        })
        .max()
        .map(|d| d + 1)
        .unwrap_or(1)
}
