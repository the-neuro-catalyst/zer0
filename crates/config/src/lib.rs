use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MasterConfig {
    pub global: GlobalConfig,
    pub storage: StorageConfig,
    pub ai: AiConfig,
    pub execution: ExecutionConfig,
    pub streaming: StreamingConfig,
    pub tui: TuiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiConfig {
    #[serde(default = "default_tick_rate")]
    pub tick_rate_ms: u64,
    #[serde(default = "default_accent_color")]
    pub accent_color: String,
}

fn default_tick_rate() -> u64 {
    50
}
fn default_accent_color() -> String {
    "white".to_string()
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self { tick_rate_ms: default_tick_rate(), accent_color: default_accent_color() }
    }
}

impl TuiConfig {
    pub fn load() -> Self {
        MasterConfig::load().map(|c| c.tui).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StreamingConfig {
    pub kafka_brokers: Option<String>,
    pub kafka_topic: Option<String>,
    pub kafka_group_id: Option<String>,
    pub rabbitmq_url: Option<String>,
    pub rabbitmq_queue: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EngineMode {
    #[default]
    Baseline,
    Extended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    #[serde(default = "default_env")]
    pub env: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    pub telemetry_endpoint: Option<String>,
    #[serde(default)]
    pub engine_mode: EngineMode,
}

fn default_env() -> String {
    "development".to_string()
}
fn default_log_level() -> String {
    "info".to_string()
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            env: default_env(),
            log_level: default_log_level(),
            telemetry_endpoint: None,
            engine_mode: EngineMode::Baseline,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StorageConfig {
    pub postgres_url: Option<String>,
    pub mongo_uri: Option<String>,
    pub neo4j_uri: Option<String>,
    pub qdrant_url: Option<String>,
    pub s3_bucket: Option<String>,
    pub s3_region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AiConfig {
    pub openai_api_key: Option<String>,
    pub embedding_model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ExecutionConfig {
    pub concurrency: usize,
    pub batch_size: usize,
    pub plugin_dir: Option<String>,
}

impl MasterConfig {
    /// Loads configuration from the standard locations:
    /// 1. Defaults
    /// 2. .env file (if exists)
    /// 3. zero.toml (at project root)
    /// 4. Environment Variables (prefixed with ZERO_)
    pub fn load() -> anyhow::Result<Self> {
        // Load .env if present
        let _ = dotenvy::dotenv();

        let mut config = Self::default();

        // 1. Try to load from zero.toml (Check common locations and parent for workspace support)
        let mut config_path = PathBuf::from("zero.config.toml");

        // Search in common locations
        let common_locations = vec![
            PathBuf::from("zero.config.toml"),
            PathBuf::from("config/zero.config.toml"),
            PathBuf::from("/etc/zero/zero.config.toml"),
        ];

        for location in common_locations {
            if location.exists() {
                config_path = location;
                break;
            }
        }

        // Search in parent directories if still not found
        if !config_path.exists() {
            if let Ok(current) = std::env::current_dir() {
                let mut parent = current.parent();
                while let Some(p) = parent {
                    let p_path = p.join("zero.config.toml");
                    if p_path.exists() {
                        config_path = p_path;
                        break;
                    }
                    parent = p.parent();
                }
            }
        }
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let toml_config: MasterConfig = toml::from_str(&content)?;
            config = toml_config;
        }

        // 2. Override with Environment Variables (Explicit mapping for clarity)
        config.override_from_env();

        Ok(config)
    }

    pub fn set(&mut self, key: &str, value: &str) -> anyhow::Result<()> {
        info!("Setting config key: {} to {}", key, value);
        match key {
            "global.env" => {
                self.global.env = value.to_string();
            }
            "global.log_level" => {
                self.global.log_level = value.to_string();
            }
            "tui.tick_rate_ms" => {
                self.tui.tick_rate_ms = value.parse()?;
            }
            "tui.accent_color" => {
                self.tui.accent_color = value.to_string();
            }

            "ai.openai_api_key" => {
                self.ai.openai_api_key = Some(value.to_string());
            }
            "ai.embedding_model" => {
                self.ai.embedding_model = Some(value.to_string());
            }
            "storage.postgres_url" => {
                self.storage.postgres_url = Some(value.to_string());
            }
            "storage.mongo_uri" => {
                self.storage.mongo_uri = Some(value.to_string());
            }
            "storage.neo4j_uri" => {
                self.storage.neo4j_uri = Some(value.to_string());
            }
            "storage.qdrant_url" => {
                self.storage.qdrant_url = Some(value.to_string());
            }
            "storage.s3_bucket" => {
                self.storage.s3_bucket = Some(value.to_string());
            }
            "storage.s3_region" => {
                self.storage.s3_region = Some(value.to_string());
            }
            "execution.concurrency" => {
                self.execution.concurrency = value.parse()?;
            }
            "execution.batch_size" => {
                self.execution.batch_size = value.parse()?;
            }
            _ => {
                return Err(anyhow::anyhow!("Unknown configuration key: {}", key));
            }
        }
        Ok(())
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write("zero.toml", content)?;
        info!("Configuration saved to zero.toml");
        Ok(())
    }

    fn override_from_env(&mut self) {
        // Global
        if let Ok(val) = std::env::var("ZERO_ENV") {
            self.global.env = val;
        }
        if let Ok(val) = std::env::var("ZERO_LOG_LEVEL").or_else(|_| std::env::var("RUST_LOG")) {
            self.global.log_level = val;
        }
        if let Ok(val) = std::env::var("ZERO_TELEMETRY_ENDPOINT") {
            self.global.telemetry_endpoint = Some(val);
        }
        if let Ok(val) = std::env::var("ZERO_ENGINE_MODE") {
            self.global.engine_mode = match val.to_lowercase().as_str() {
                "extended" => EngineMode::Extended,
                _ => EngineMode::Baseline,
            };
        }

        // Storage with Fallbacks
        if let Ok(val) = std::env::var("ZERO_PG_URI")
            .or_else(|_| std::env::var("PG_URI"))
            .or_else(|_| std::env::var("DB_URL"))
        {
            self.storage.postgres_url = Some(val);
        }
        if let Ok(val) = std::env::var("ZERO_MONGO_URI").or_else(|_| std::env::var("MONGO_URI")) {
            self.storage.mongo_uri = Some(val);
        }
        if let Ok(val) = std::env::var("ZERO_NEO4J_URI").or_else(|_| std::env::var("NEO4J_URI")) {
            self.storage.neo4j_uri = Some(val);
        }
        if let Ok(val) = std::env::var("ZERO_QDRANT_URL").or_else(|_| std::env::var("QDRANT_URI")) {
            self.storage.qdrant_url = Some(val);
        }
        if let Ok(val) = std::env::var("ZERO_S3_BUCKET").or_else(|_| std::env::var("S3_BUCKET")) {
            self.storage.s3_bucket = Some(val);
        }
        if let Ok(val) = std::env::var("ZERO_S3_REGION").or_else(|_| std::env::var("AWS_REGION")) {
            self.storage.s3_region = Some(val);
        }

        // Streaming
        if let Ok(val) = std::env::var("ZERO_KAFKA_BROKERS") {
            self.streaming.kafka_brokers = Some(val);
        }
        if let Ok(val) = std::env::var("ZERO_KAFKA_TOPIC") {
            self.streaming.kafka_topic = Some(val);
        }
        if let Ok(val) = std::env::var("ZERO_KAFKA_GROUP_ID") {
            self.streaming.kafka_group_id = Some(val);
        }
        if let Ok(val) = std::env::var("ZERO_RABBITMQ_URL") {
            self.streaming.rabbitmq_url = Some(val);
        }
        if let Ok(val) = std::env::var("ZERO_RABBITMQ_QUEUE") {
            self.streaming.rabbitmq_queue = Some(val);
        }

        // AI with Fallbacks
        if let Ok(val) =
            std::env::var("ZERO_OPENAI_API_KEY").or_else(|_| std::env::var("OPENAI_API_KEY"))
        {
            self.ai.openai_api_key = Some(val);
        }

        // Execution
        if let Ok(val) = std::env::var("ZERO_CONCURRENCY")
            .and_then(|v| v.parse().map_err(|_| std::env::VarError::NotPresent))
        {
            self.execution.concurrency = val;
        }
        if let Ok(val) = std::env::var("ZERO_PLUGIN_DIR") {
            self.execution.plugin_dir = Some(val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_defaults() {
        // Ensure we can load even if zero.toml is missing
        let config = MasterConfig::load().unwrap();
        assert_eq!(config.global.env, "development");
    }

    #[test]
    fn test_tui_config_load() {
        let tui_config = TuiConfig::load();
        assert_eq!(tui_config.tick_rate_ms, 50);
    }
}
