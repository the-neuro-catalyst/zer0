use crate::ConfigCommands;

use config::MasterConfig as Settings;

use tracing::{info, warn};

/// Runs the config command.
pub async fn run(config_command: &ConfigCommands) -> Result<(), anyhow::Error> {
    info!("Managing configuration...");
    let mut settings = Settings::load().unwrap_or_else(|_| {
        warn!("Failed to load Settings, using defaults.");
        Settings::default()
    });

    match config_command {
        ConfigCommands::Show => {
            println!("{:#?}", settings);
        }
        ConfigCommands::Set { key, value } => {
            settings.set(key, value)?;
            settings.save()?;
            info!("Configuration key '{}' set to '{}' and saved.", key, value);
        }
    }
    Ok(())
}
