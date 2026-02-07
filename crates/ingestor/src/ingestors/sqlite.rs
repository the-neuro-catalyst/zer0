use crate::engine::ProcessorOutput;
use crate::traits::ingestor::IngestorConfig;
use anyhow::Result;
use async_trait::async_trait; // Added this line

pub struct SqliteIngestor;

impl SqliteIngestor {
    pub async fn new(_config: IngestorConfig) -> Result<Self> {
        Ok(SqliteIngestor)
    }
}

#[async_trait]
impl crate::traits::ingestor::Ingestor for SqliteIngestor {
    async fn ingest(&self, _output: &ProcessorOutput) -> Result<()> {
        Ok(())
    }
}
