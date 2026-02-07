use serde::{Deserialize, Serialize};

use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub zero_copy: bool,
    pub schema_inference: bool,
    pub pii_redaction: bool,
    pub strict_mode: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self { zero_copy: true, schema_inference: true, pii_redaction: true, strict_mode: false }
    }
}

pub struct ConfigState(pub Mutex<AppConfig>);

impl ConfigState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ConfigState {
    fn default() -> Self {
        Self(Mutex::new(AppConfig::default()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_config_default_values() {
        let config = AppConfig::default();
        assert!(config.zero_copy);
        assert!(config.pii_redaction);
        assert!(!config.strict_mode);
    }

    #[test]
    fn test_config_state_update() {
        let state = ConfigState::new();
        {
            let mut config = state.0.lock().unwrap();
            config.strict_mode = true;
        }

        let final_config = state.0.lock().unwrap();
        assert!(final_config.strict_mode);
    }
}
