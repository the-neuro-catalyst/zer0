use clap::ValueEnum;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
    Csv,
    Text,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum OutputMode {
    Default,
    SchemaOnly,
    FullRaw,
    Analyze,
    Stream,
}
