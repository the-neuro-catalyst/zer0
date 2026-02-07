use anyhow::Result;
use config::MasterConfig as Settings;
use std::sync::Arc;
use tracing_subscriber::prelude::*;
use tui::logger::{LogRegistry, TuiLoggerLayer};
use tui::run_monitor;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize Log Registry for TUI
    let log_registry = LogRegistry::new();
    let tui_layer = TuiLoggerLayer::new(log_registry.logs.clone());

    // 2. Load Settings first
    let settings = match Settings::load() {
        Ok(s) => Arc::new(s),
        Err(_) => Arc::new(Settings::default()),
    };

    // 3. Setup tracing with TUI layer using config level
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&settings.global.log_level));

    tracing_subscriber::registry().with(filter).with(tui_layer).init();

    // Initial heartbeats
    tracing::info!("ZERO Data Inspector: SYSTEM_READY");
    tracing::info!("Environment: {}", settings.global.env);
    tracing::info!("Log Level: {}", settings.global.log_level);
    tracing::info!("Working Directory: {:?}", std::env::current_dir().unwrap_or_default());

    // 4. Run the Monitor
    run_monitor(log_registry.logs, settings).await?;

    Ok(())
}
