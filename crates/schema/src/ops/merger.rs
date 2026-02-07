use crate::types::data_type::DataType;

use std::collections::HashMap;

/// A polymorphic container to handle data type conflicts.
/// Instead of failing on type mismatches, conflicting values are wrapped into a `Union` type.
#[derive(Debug, Clone, PartialEq)]
pub enum UnionValue {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    List(Vec<UnionValue>),
    Map(HashMap<String, UnionValue>),
    Union(Vec<UnionValue>),
    Null,
}

impl UnionValue {
    /// Merges another UnionValue into this one.
    /// If types match, they are merged (recursively for complex types).
    /// If they mismatch, they are wrapped into a Union variant.
    pub fn merge(self, other: UnionValue) -> UnionValue {
        match (self, other) {
            (UnionValue::Null, other) => other,
            (other, UnionValue::Null) => other,
            (UnionValue::Map(mut a), UnionValue::Map(b)) => {
                for (k, v) in b {
                    if let Some(existing) = a.remove(&k) {
                        a.insert(k, existing.merge(v));
                    } else {
                        a.insert(k, v);
                    }
                }
                UnionValue::Map(a)
            }
            (UnionValue::List(mut a), UnionValue::List(b)) => {
                // For lists, we extend the current list with new elements.
                a.extend(b);
                UnionValue::List(a)
            }
            (UnionValue::Union(mut v1), UnionValue::Union(v2)) => {
                for v in v2 {
                    if !v1.contains(&v) {
                        v1.push(v);
                    }
                }
                UnionValue::Union(v1)
            }
            (UnionValue::Union(mut v), other) | (other, UnionValue::Union(mut v)) => {
                if !v.contains(&other) {
                    v.push(other);
                }
                UnionValue::Union(v)
            }
            (a, b) => {
                if a == b {
                    a
                } else {
                    // Type widening/promotion following arrow-rs patterns (Int -> Float)
                    match (a, b) {
                        (UnionValue::Integer(i), UnionValue::Float(f))
                        | (UnionValue::Float(f), UnionValue::Integer(i)) => {
                            let f_i = i as f64;
                            if (f_i - f).abs() < f64::EPSILON {
                                UnionValue::Float(f)
                            } else {
                                UnionValue::Union(vec![
                                    UnionValue::Float(f_i),
                                    UnionValue::Float(f),
                                ])
                            }
                        }
                        (a, b) => UnionValue::Union(vec![a, b]),
                    }
                }
            }
        }
    }

    /// Alias for merge to support the promotion terminology.
    pub fn promote(self, other: UnionValue) -> UnionValue {
        self.merge(other)
    }
}

impl From<DataType> for UnionValue {
    fn from(t: DataType) -> Self {
        match t {
            DataType::String => UnionValue::Text("".to_string()),
            DataType::Number => UnionValue::Float(0.0),
            DataType::Integer => UnionValue::Integer(0),
            DataType::Float => UnionValue::Float(0.0),
            DataType::Boolean => UnionValue::Boolean(false),
            DataType::Null => UnionValue::Null,
            DataType::Array(inner) => UnionValue::List(vec![UnionValue::from(*inner)]),
            DataType::Object(map) => {
                UnionValue::Map(map.into_iter().map(|(k, v)| (k, UnionValue::from(v))).collect())
            }
            DataType::Union(variants) => {
                UnionValue::Union(variants.into_iter().map(UnionValue::from).collect())
            }
            DataType::Unknown => UnionValue::Null,
        }
    }
}

impl From<UnionValue> for DataType {
    fn from(v: UnionValue) -> Self {
        match v {
            UnionValue::Integer(_) => DataType::Integer,
            UnionValue::Float(_) => DataType::Float,
            UnionValue::Text(_) => DataType::String,
            UnionValue::Boolean(_) => DataType::Boolean,
            UnionValue::Map(m) => {
                let mut map = HashMap::new();
                for (k, v) in m {
                    map.insert(k, DataType::from(v));
                }
                DataType::Object(map)
            }
            UnionValue::List(l) => {
                if let Some(first) = l.first() {
                    DataType::Array(Box::new(DataType::from(first.clone())))
                } else {
                    DataType::Array(Box::new(DataType::Unknown))
                }
            }
            UnionValue::Union(u) => DataType::Union(u.into_iter().map(DataType::from).collect()),
            UnionValue::Null => DataType::Null,
        }
    }
}

impl From<serde_json::Value> for UnionValue {
    fn from(v: serde_json::Value) -> Self {
        match v {
            serde_json::Value::Null => UnionValue::Null,
            serde_json::Value::Bool(b) => UnionValue::Boolean(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    UnionValue::Integer(i)
                } else {
                    UnionValue::Float(n.as_f64().unwrap_or(0.0))
                }
            }
            serde_json::Value::String(s) => UnionValue::Text(s),
            serde_json::Value::Array(a) => {
                UnionValue::List(a.into_iter().map(UnionValue::from).collect())
            }
            serde_json::Value::Object(o) => {
                UnionValue::Map(o.into_iter().map(|(k, v)| (k, UnionValue::from(v))).collect())
            }
        }
    }
}

impl From<UnionValue> for serde_json::Value {
    fn from(v: UnionValue) -> Self {
        match v {
            UnionValue::Null => serde_json::Value::Null,
            UnionValue::Boolean(b) => serde_json::Value::Bool(b),
            UnionValue::Integer(i) => serde_json::Value::Number(i.into()),
            UnionValue::Float(f) => serde_json::Number::from_f64(f)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
            UnionValue::Text(s) => serde_json::Value::String(s),
            UnionValue::List(l) => {
                serde_json::Value::Array(l.into_iter().map(|v| v.into()).collect())
            }
            UnionValue::Map(m) => {
                serde_json::Value::Object(m.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
            UnionValue::Union(u) => {
                serde_json::Value::Array(u.into_iter().map(|v| v.into()).collect())
            }
        }
    }
}

/// Merges two UnionValues into a single compatible UnionValue.
///
/// This utilizes the robust Union type system to handle complex type mismatches
/// by wrapping conflicting values into a `Union` variant instead of failing.
pub fn merge_types(v1: UnionValue, v2: UnionValue) -> UnionValue {
    v1.merge(v2)
}

/// Helper function to merge two DataTypes (legacy support).
pub fn merge_data_types(t1: DataType, t2: DataType) -> DataType {
    if t1 == t2 {
        return t1;
    }

    match (t1, t2) {
        (DataType::Unknown, other) => other,
        (other, DataType::Unknown) => other,
        (DataType::Null, other) | (other, DataType::Null) => {
            merge_into_union_data_type(other, DataType::Null)
        }
        (DataType::Integer, DataType::Float) | (DataType::Float, DataType::Integer) => {
            DataType::Float
        }
        (DataType::Number, DataType::Integer) | (DataType::Integer, DataType::Number) => {
            DataType::Number
        }
        (DataType::Number, DataType::Float) | (DataType::Float, DataType::Number) => {
            DataType::Number
        }
        (DataType::Array(a), DataType::Array(b)) => {
            DataType::Array(Box::new(merge_data_types(*a, *b)))
        }
        (DataType::Object(mut a), DataType::Object(b)) => {
            for (k, v) in b {
                if let Some(existing) = a.remove(&k) {
                    a.insert(k, merge_data_types(existing, v));
                } else {
                    a.insert(k, v);
                }
            }
            DataType::Object(a)
        }
        (DataType::Union(mut v1), DataType::Union(v2)) => {
            for t in v2 {
                if !v1.contains(&t) {
                    v1.push(t);
                }
            }
            DataType::Union(v1)
        }
        (DataType::Union(mut v), other) | (other, DataType::Union(mut v)) => {
            if !v.contains(&other) {
                v.push(other);
            }
            DataType::Union(v)
        }
        (a, b) => DataType::Union(vec![a, b]),
    }
}

/// Helper function to encapsulate a type into a union with another type.
fn merge_into_union_data_type(t1: DataType, t2: DataType) -> DataType {
    match t1 {
        DataType::Union(mut v) => {
            if !v.contains(&t2) {
                v.push(t2);
            }
            DataType::Union(v)
        }
        _ => DataType::Union(vec![t1, t2]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_value_merge_simple() {
        let v1 = UnionValue::Integer(10);
        let v2 = UnionValue::Integer(10);
        assert_eq!(v1.merge(v2), UnionValue::Integer(10));

        let v1 = UnionValue::Integer(10);
        let v2 = UnionValue::Text("foo".to_string());
        let merged = v1.merge(v2);
        assert_eq!(
            merged,
            UnionValue::Union(vec![UnionValue::Integer(10), UnionValue::Text("foo".to_string())])
        );
    }

    #[test]
    fn test_union_value_promote_int_float() {
        let v1 = UnionValue::Integer(10);
        let v2 = UnionValue::Float(10.0);
        assert_eq!(v1.merge(v2), UnionValue::Float(10.0));

        let v1 = UnionValue::Integer(10);
        let v2 = UnionValue::Float(10.5);
        let merged = v1.merge(v2);
        // Note: 10.0 and 10.5 are different values, so they stay as a Union even if they are both "numbers"
        // depending on how we want to handle data merging.
        // In our current implementation, they become Union([Float(10.0), Float(10.5)])
        match merged {
            UnionValue::Union(v) => {
                assert!(v.contains(&UnionValue::Float(10.0)));
                assert!(v.contains(&UnionValue::Float(10.5)));
            }
            _ => panic!("Expected Union, got {:?}", merged),
        }
    }

    #[test]
    fn test_union_value_merge_recursive() {
        let mut m1 = HashMap::new();
        m1.insert("a".to_string(), UnionValue::Integer(1));
        let v1 = UnionValue::Map(m1);

        let mut m2 = HashMap::new();
        m2.insert("a".to_string(), UnionValue::Text("one".to_string()));
        m2.insert("b".to_string(), UnionValue::Boolean(true));
        let v2 = UnionValue::Map(m2);

        let merged = v1.merge(v2);
        if let UnionValue::Map(m) = merged {
            assert_eq!(m.len(), 2);
            assert!(matches!(m.get("a").unwrap(), UnionValue::Union(_)));
            assert_eq!(*m.get("b").unwrap(), UnionValue::Boolean(true));
        } else {
            panic!("Expected Map, got {:?}", merged);
        }
    }

    #[test]
    fn test_serde_integration() {
        let json = r#"{"a": 1, "b": [true, "mixed"]}"#;
        let v: serde_json::Value = serde_json::from_str(json).unwrap();
        let uv = UnionValue::from(v);

        if let UnionValue::Map(m) = &uv {
            assert_eq!(m.get("a").unwrap(), &UnionValue::Integer(1));
            if let UnionValue::List(l) = m.get("b").unwrap() {
                assert_eq!(l[0], UnionValue::Boolean(true));
                assert_eq!(l[1], UnionValue::Text("mixed".to_string()));
            } else {
                panic!("Expected List for 'b'");
            }
        } else {
            panic!("Expected Map");
        }

        let back_to_json: serde_json::Value = uv.into();
        assert_eq!(back_to_json["a"], 1);
        assert_eq!(back_to_json["b"][0], true);
        assert_eq!(back_to_json["b"][1], "mixed");
    }
}
