use anyhow::Result;
pub mod config;
pub mod inspect;
pub mod report;

use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigCommands {
    /// 📋  Show Current State | Display all active environment parameters and security tokens
    ///
    /// Prints the complete Master Configuration in a structured format.
    /// Use this to verify active log levels, tick rates, and endpoint URIs.
    Show,

    /// ✍️   Modify Parameter  | Update a specific configuration key with a new value
    ///
    /// Overwrites an existing configuration key. Changes are persisted
    /// immediately to the local environment configuration file.
    #[command(
        after_help = "EXAMPLES:\n  zc config set global.log_level debug\n  zc config set tui.tick_rate_ms 250"
    )]
    Set {
        /// The target configuration key (e.g., 'global.log_level')
        #[arg(value_name = "KEY")]
        key: String,
        /// The new value to assign to the key
        #[arg(value_name = "VALUE")]
        value: String,
    },
}

pub async fn report_command(file: String) -> Result<()> {
    report::run(file).await
}

pub async fn inspect_command(
    data_path: Option<std::path::PathBuf>,
    text: Option<String>,
    detailed: bool,
) -> Result<()> {
    inspect::run(data_path, text, detailed).await
}

pub async fn config_command_run(config_command: &ConfigCommands) -> anyhow::Result<()> {
    config::run(config_command).await.map_err(|e| anyhow::anyhow!(e.to_string()))
}
