use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;

use schema::SchemaValue;
use tokio_postgres::{Config as TokioPgConfig, Row};
use tokio_postgres_rustls::MakeRustlsConnect;

use crate::error::DataReaderError;
use crate::reader_result::{DataReaderResult, FileMetadata, RecordStream};

pub async fn read_postgresql_data(
    db_url: &str,
    query: &str,
) -> Result<DataReaderResult, DataReaderError> {
    let pg_config = TokioPgConfig::from_str(db_url).map_err(|e| DataReaderError::ParseError {
        path: "postgresql_url".into(),
        source: Box::new(e),
    })?;

    // Create a rustls configuration.
    // ZERO prioritizes self-contained logic over host-dependent certificate stores.
    let root_store = rustls::RootCertStore::empty();
    // Note: For production use, load system certificates or use webpki-roots here.

    let config =
        rustls::ClientConfig::builder().with_root_certificates(root_store).with_no_client_auth();

    let tls = MakeRustlsConnect::new(config);

    let (client, connection) = pg_config
        .connect(tls)
        .await
        .map_err(|e| DataReaderError::ParseError { path: db_url.into(), source: Box::new(e) })?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query(query, &[])
        .await
        .map_err(|e| DataReaderError::ParseError { path: query.into(), source: Box::new(e) })?;

    let schema_rows: Vec<Result<SchemaValue<'static>, DataReaderError>> =
        rows.into_iter().map(|row| Ok(postgres_row_to_schema_value(&row))).collect();

    let stream: RecordStream = Box::new(schema_rows.into_iter());

    Ok(DataReaderResult::Stream(stream, FileMetadata::default()))
}

fn postgres_row_to_schema_value(row: &Row) -> SchemaValue<'static> {
    let mut map = HashMap::new();
    for column in row.columns() {
        let name = column.name();
        let value = get_column_value(row, name);
        map.insert(Cow::Owned(name.to_string()), value);
    }
    SchemaValue::Object(map)
}

fn get_column_value(row: &Row, name: &str) -> SchemaValue<'static> {
    // Basic type mapping, can be expanded
    if let Ok(val) = row.try_get::<_, String>(name) {
        SchemaValue::String(Cow::Owned(val))
    } else if let Ok(val) = row.try_get::<_, i64>(name) {
        SchemaValue::Integer(val)
    } else if let Ok(val) = row.try_get::<_, i32>(name) {
        SchemaValue::Integer(val as i64)
    } else if let Ok(val) = row.try_get::<_, f64>(name) {
        SchemaValue::Float(val)
    } else if let Ok(val) = row.try_get::<_, bool>(name) {
        SchemaValue::Boolean(val)
    } else {
        SchemaValue::Null
    }
}
