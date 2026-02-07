use crate::engine::ProcessorOutput;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestorConfig {
    pub database_url: Option<String>,
    pub collection_name: String,
    pub vector_size: u64,
    pub mappings: Option<Value>,
    pub openai_api_key: Option<String>,
    pub embed_field: Option<String>,
    pub relationships: Option<Value>,
}

#[async_trait]
pub trait Ingestor {
    async fn ingest(&self, output: &ProcessorOutput) -> Result<()>;
}
