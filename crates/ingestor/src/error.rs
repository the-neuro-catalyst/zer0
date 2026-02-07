use reader::error::DataReaderError;

use anyhow::Error as AnyhowError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IngestorError {
    #[error("Failed to connect to database: {0}")]
    ConnectionError(String),
    #[error("Failed to ingest data: {0}")]
    IngestionError(String),
    #[error("Invalid configuration: {0}")]
    ConfigurationError(String),
    #[error("Database specific error: {0}")]
    DatabaseError(String),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Data reader error: {0}")]
    DataReaderError(#[from] DataReaderError),
    #[error("Engine error: {0}")]
    AlignmentError(#[from] crate::engine::EngineError),
    #[error("Anyhow error: {0}")]
    Anyhow(#[from] AnyhowError),
    #[error("Other error: {0}")]
    Other(String),
}

impl IngestorError {
    pub fn is_transient(&self) -> bool {
        match self {
            IngestorError::ConnectionError(_) => true,
            IngestorError::DatabaseError(msg) => {
                let m = msg.to_lowercase();
                m.contains("timeout") || m.contains("connection")
            }
            _ => false,
        }
    }
}

pub type Result<T> = std::result::Result<T, IngestorError>;
