pub mod cli;
pub mod common;
pub mod embeddings;
pub mod ingestors;
pub mod traits;

pub mod engine;
pub mod error;

pub mod processor;
pub mod schema_builder;

pub mod transformation;

pub const DEFAULT_COLLECTION_NAME: &str = "ingested_collection";
pub const DEFAULT_VECTOR_SIZE: u64 = 4;
pub const DEFAULT_SQL_TABLE_NAME: &str = "ingested_data";
