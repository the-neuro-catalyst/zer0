use std::borrow::Cow;

use std::collections::HashMap;

use mysql_async::prelude::*;

use mysql_async::{Opts, Pool};

use schema::SchemaValue;

use crate::error::DataReaderError;

use crate::reader_result::{DataReaderResult, FileMetadata, RecordStream};

#[allow(dead_code)]
pub async fn read_mysql_data(url: &str, query: &str) -> Result<DataReaderResult, DataReaderError> {
    let opts = Opts::from_url(url)
        .map_err(|e| DataReaderError::ParseError { path: url.into(), source: Box::new(e) })?;

    let pool = Pool::new(opts);
    let mut conn = pool
        .get_conn()
        .await
        .map_err(|e| DataReaderError::InternalError(format!("MySQL connection error: {}", e)))?;

    let rows: Vec<mysql_async::Row> = conn
        .query(query)
        .await
        .map_err(|e| DataReaderError::InternalError(format!("MySQL query error: {}", e)))?;

    let schema_rows: Vec<Result<SchemaValue<'static>, DataReaderError>> =
        rows.into_iter().map(|row| Ok(mysql_row_to_schema_value(row))).collect();

    let stream: RecordStream = Box::new(schema_rows.into_iter());

    Ok(DataReaderResult::Stream(stream, FileMetadata::default()))
}

#[allow(dead_code)]
fn mysql_row_to_schema_value(row: mysql_async::Row) -> SchemaValue<'static> {
    let mut map = HashMap::new();
    let columns = row.columns();

    for (i, column) in columns.iter().enumerate() {
        let name = column.name_str();
        let value = if let Some(val) = row.get_opt::<mysql_async::Value, usize>(i) {
            match val {
                Ok(mysql_async::Value::Bytes(b)) => {
                    SchemaValue::String(Cow::Owned(String::from_utf8_lossy(&b).to_string()))
                }
                Ok(mysql_async::Value::Int(i)) => SchemaValue::Integer(i),
                Ok(mysql_async::Value::Float(f)) => SchemaValue::Float(f as f64),
                Ok(mysql_async::Value::Double(d)) => SchemaValue::Float(d),
                Ok(mysql_async::Value::NULL) => SchemaValue::Null,
                _ => SchemaValue::Unknown,
            }
        } else {
            SchemaValue::Null
        };
        map.insert(Cow::Owned(name.to_string()), value);
    }
    SchemaValue::Object(map)
}
