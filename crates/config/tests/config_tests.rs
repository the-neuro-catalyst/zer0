use config::{EngineMode, MasterConfig};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_config_serialization() {
    let mut config = MasterConfig::default();
    config.global.env = "test".to_string();
    config.global.engine_mode = EngineMode::Extended;

    let toml_str = toml::to_string(&config).unwrap();
    assert!(toml_str.contains("env = \"test\""));
    assert!(toml_str.contains("engine_mode = \"extended\""));
}

#[test]
fn test_config_set_key() {
    let mut config = MasterConfig::default();
    config.set("global.env", "production").unwrap();
    assert_eq!(config.global.env, "production");

    config.set("tui.tick_rate_ms", "100").unwrap();
    assert_eq!(config.tui.tick_rate_ms, 100);
}

#[test]
fn test_load_from_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("zero.config.toml");

    let toml_content = r#"[global]
env = "staging"
log_level = "debug"

[tui]
accent_color = "green"
"#;
    fs::write(&file_path, toml_content).unwrap();

    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();

    let config = MasterConfig::load().unwrap();
    assert_eq!(config.global.env, "staging");
    assert_eq!(config.tui.accent_color, "green");

    std::env::set_current_dir(original_dir).unwrap();
}
