use serde::{Deserialize, Serialize};

use std::path::PathBuf;

use thiserror::Error;

use tracing::info;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Execution error: {0}")]
    ExecutionError(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Processor not initialized")]
    NotInitialized,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessorInput {
    pub data: serde_json::Value,
    pub context: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessorOutput {
    pub result: serde_json::Value,
    pub quality_score: f64,
}

pub struct DataProcessor {
    initialized: bool,
}

impl DataProcessor {
    pub fn new(_plugin_path: Option<PathBuf>) -> Self {
        Self { initialized: false }
    }
    pub fn initialize(&mut self) -> Result<(), EngineError> {
        info!("Data Processor initialized.");
        self.initialized = true;
        Ok(())
    }
    pub fn process_data(&self, input: ProcessorInput) -> Result<ProcessorOutput, EngineError> {
        if !self.initialized {
            return Err(EngineError::NotInitialized);
        }
        Ok(ProcessorOutput { result: input.data, quality_score: 1.0 })
    }
}
