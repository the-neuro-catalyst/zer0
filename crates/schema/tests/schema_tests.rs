use schema::{merge_types, DataType, SchemaValue, UnionValue};
use std::borrow::Cow;
use std::collections::HashMap;

#[test]
fn test_schema_value_equality() {
    let v1 = SchemaValue::Integer(42);
    let v2 = SchemaValue::Integer(42);
    let v3 = SchemaValue::String(Cow::Borrowed("42"));

    assert_eq!(v1, v2);
    assert_ne!(v1, v3);
}

#[test]
fn test_merge_types_basic() {
    let t1 = UnionValue::Integer(10);
    let t2 = UnionValue::Integer(20);
    let merged = merge_types(t1, t2);
    // Since 10 != 20, they become a Union if not identical, OR if we handle values.
    // In our merger.rs, if they are both Integer but different values, they stay as Union?
    // Wait, let's re-read merger.rs:
    // (a, b) => { if a == b { a } else { ... Union([a, b]) } }
    match merged {
        UnionValue::Union(v) => assert_eq!(v.len(), 2),
        _ => panic!("Expected union for different values"),
    }
}

#[test]
fn test_schema_creation() {
    let mut fields = HashMap::new();
    fields.insert("id".to_string(), DataType::Integer);
    fields.insert("name".to_string(), DataType::String);

    let schema: schema::Schema = fields;
    assert_eq!(schema.len(), 2);
}
