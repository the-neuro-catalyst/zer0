use crate::error::DataReaderError;
use crate::reader_result::DataReaderResult;

#[cfg(feature = "database")]
use crate::readers::{postgresql_reader, sqlite_reader};

#[allow(dead_code)]
pub enum DatabaseType {
    Postgresql,
    Sqlite,
}

impl DatabaseType {
    #[allow(dead_code)]
    pub fn from_url(url: &str) -> Self {
        if url.starts_with("postgres") {
            DatabaseType::Postgresql
        } else {
            DatabaseType::Sqlite
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseReaderOptions {
    pub db_type: DatabaseType,
    pub db_url: String,
    pub query: String,
}

#[allow(dead_code)]
pub async fn read_database_content(
    options: DatabaseReaderOptions,
) -> Result<DataReaderResult, DataReaderError> {
    #[cfg(feature = "database")]
    {
        match options.db_type {
            DatabaseType::Postgresql => {
                postgresql_reader::read_postgresql_data(&options.db_url, &options.query).await
            }
            DatabaseType::Sqlite => {
                sqlite_reader::read_sqlite_data_with_query(&options.db_url, &options.query).await
            }
        }
    }
    #[cfg(not(feature = "database"))]
    {
        let _ = options;
        Err(DataReaderError::InternalError("Database feature disabled".to_string()))
    }
}

pub async fn get_database_schema(url: &str) -> Result<serde_json::Value, DataReaderError> {
    #[cfg(feature = "database")]
    {
        let db_type = DatabaseType::from_url(url);
        match db_type {
            DatabaseType::Postgresql => {
                // Simplified for now, can be expanded
                Ok(
                    serde_json::json!({"error": "Postgres schema mapping not yet implemented in perception layer"}),
                )
            }
            DatabaseType::Sqlite => sqlite_reader::get_sqlite_schema(url).await,
        }
    }
    #[cfg(not(feature = "database"))]
    {
        let _ = url;
        Err(DataReaderError::InternalError("Database feature disabled".to_string()))
    }
}
