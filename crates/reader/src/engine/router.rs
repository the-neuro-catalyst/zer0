use std::path::Path;

use crate::engine::file_engine::{read_file_content, FileReaderOptions};

use crate::error::DataReaderError;

use crate::reader_result::DataReaderResult;
#[cfg(feature = "cloud")]
use crate::readers::{http_reader, s3_reader};

/// Reads data from a specified source, which can be a local file path, an HTTP/HTTPS URL,
/// an S3 URI, or a database connection string (PostgreSQL, MySQL).
///
/// This function acts as a central router for all data ingestion, intelligently
/// determining the source type based on the input string's prefix and delegating
/// to the appropriate reader engine. It supports various data formats and applies
/// processing options such as PII redaction.
///
/// # Arguments
///
/// * `input` - A string slice (`&str`) representing the data source. This can be:
///     * A local file path (e.g., `"data.json"`, `"/var/log/app.log"`)
///     * An HTTP/HTTPS URL (e.g., `"https://example.com/api/data.csv"`) - requires `cloud` feature.
///     * An S3 URI (e.g., `"s3://my-bucket/path/to/object.parquet"`) - requires `cloud` feature.
///     * A PostgreSQL connection string (e.g., `"postgres://user:pass@host:port/dbname"`) - requires `database` feature.
///     * A MySQL connection string (e.g., `"mysql://user:pass@host:port/dbname"`) - requires `database` feature.
///
/// * `query` - An optional string slice (`Option<&str>`) used for database queries.
///   If the `input` is a database URI, this query will be executed.
///   If `None`, a default query (e.g., to list tables) might be used.
///   For non-database sources, this argument is ignored.
///
/// * `options` - A `FileReaderOptions` struct containing various parameters
///   to control how the data is read and processed, such as `head` (limit),
///   `output_mode`, `output_format`, and `pii_redaction`.
///
/// # Returns
///
/// A `Result` which is:
/// * `Ok(DataReaderResult)`: On successful data retrieval and processing, containing
///   the read data and associated metadata.
/// * `Err(DataReaderError)`: If an error occurs during source detection, reading,
///   parsing, or if a required feature is not enabled.
///
/// # Examples
///
/// ```no_run
/// use reader::engine::router::read_source;
/// use reader::engine::file_engine::FileReaderOptions;
/// use reader::output::{OutputFormat, OutputMode};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let file_options = FileReaderOptions {
///         head: Some(100),
///         file_type_override: None,
///         output_mode: OutputMode::Default,
///         output_format: OutputFormat::Json,
///         pii_redaction: false,
///         zero_copy: true,
///         recursive: false,
///         filter_exts: None,
///         output_path: None,
///     };
///
///     // Read from a local JSON file
///     let file_result = read_source("data.json", None, file_options.clone()).await?;
///     println!("Read from file: {:?}", file_result.get_metadata());
///
///     // Read from an HTTP URL (requires "cloud" feature)
///     // let http_result = read_source("https://api.example.com/users", None, file_options.clone()).await?;
///     // println!("Read from HTTP: {:?}", http_result.get_metadata());
///
///     // Read from a PostgreSQL database with a custom query (requires "database" feature)
///     // let db_options = file_options.clone();
///     // let db_result = read_source(
///     //     "postgres://user:password@localhost:5432/mydb",
///     //     Some("SELECT * FROM my_table LIMIT 10"),
///     //     db_options
///     // ).await?;
///     // println!("Read from DB: {:?}", db_result.get_metadata());
///
///     Ok(())
/// }
/// ```
#[allow(dead_code)]
pub async fn read_source(
    input: &str,
    _query: Option<&str>,
    options: FileReaderOptions,
) -> Result<DataReaderResult, DataReaderError> {
    let result = match input {
        s if s.starts_with("http://") || s.starts_with("https://") => {
            #[cfg(feature = "cloud")]
            {
                http_reader::read_http_data(input).await
            }
            #[cfg(not(feature = "cloud"))]
            {
                Err(DataReaderError::InternalError("Cloud/HTTP feature disabled".to_string()))
            }
        }
        s if s.starts_with("s3://") => {
            #[cfg(feature = "cloud")]
            {
                let s3_path =
                    input.strip_prefix("s3://").ok_or_else(|| DataReaderError::ParseError {
                        path: input.into(),
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "S3 URI must start with 's3://'",
                        )),
                    })?;
                let parts: Vec<&str> = s3_path.splitn(2, '/').collect();
                if parts.len() != 2 {
                    Err(DataReaderError::ParseError {
                        path: input.into(),
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Invalid S3 URI format. Expected s3://bucket/key",
                        )),
                    })
                } else {
                    let bucket = parts[0];
                    let key = parts[1];
                    s3_reader::read_s3_data(bucket, key, None).await
                }
            }
            #[cfg(not(feature = "cloud"))]
            {
                Err(DataReaderError::InternalError("Cloud/S3 feature disabled".to_string()))
            }
        }
        s if s.starts_with("postgres://") || s.starts_with("postgresql://") => {
            #[cfg(feature = "database")]
            {
                let effective_query = query.unwrap_or(
                    "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'",
                );
                crate::readers::postgresql_reader::read_postgresql_data(input, effective_query)
                    .await
            }
            #[cfg(not(feature = "database"))]
            {
                Err(DataReaderError::InternalError("Database feature disabled".to_string()))
            }
        }
        s if s.starts_with("mysql://") => {
            #[cfg(feature = "database")]
            {
                let effective_query = query.unwrap_or("SHOW TABLES");
                crate::readers::mysql_reader::read_mysql_data(input, effective_query).await
            }
            #[cfg(not(feature = "database"))]
            {
                Err(DataReaderError::InternalError("Database feature disabled".to_string()))
            }
        }
        _ => read_file_content(Path::new(input), options.clone()).await,
    };

    // Apply PII Redaction if requested for ALL sources
    if let Ok(mut res) = result {
        if options.pii_redaction {
            res.redact();
        }
        return Ok(res);
    }

    result
}
