use anyhow::Result;
use clap::builder::styling::{AnsiColor, Styles};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tracing::info;

mod commands;
use commands::{config_command_run, inspect_command, report_command, ConfigCommands};

const CLAP_STYLING: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default().bold())
    .usage(AnsiColor::Yellow.on_default().bold())
    .literal(AnsiColor::Cyan.on_default().bold())
    .placeholder(AnsiColor::Green.on_default());

#[derive(Parser)]
#[command(
    name = "zc",
    author,
    version,
    about = "ZERO Data Inspector — The Industrial-Grade Control Lever",
    long_about = "ZERO Data Inspector (zc) — The Industrial-Grade Control Lever\n\n\
                  A high-performance, zero-copy data inspection engine designed for deep resource mapping, \
                  security forensics, and real-time monitoring.\n\n\
                  STRATEGIC WORKFLOW:\n\n\
                  1. INSPECT  : Execute focused forensics against identified resources.\n\
                  2. MONITOR  : Engage interactive TUI for real-time observability.\n\
                  3. REPORT   : Synthesize findings into structured intelligence (JSON).\n\n\
                  --------------------------------------------------------------------------------\n\n\
                  GLOBAL OPTIONS:\n\n\
                  -l, --log-level <LEVEL>\n\
                      Sets the global filtering level for diagnostic logs.\n\
                      Supported values: trace, debug, info, warn, error.\n\
                      Default: info (standard operational visibility).\n\n\
                  --message-format <FMT>\n\
                      Defines how internal system messages are rendered.\n\
                      human : ANSI-colored, multi-line readable format.\n\
                      json  : Machine-readable JSON for log aggregation.\n\
                      short : Minimalist single-line status updates.\n\n\
                  --------------------------------------------------------------------------------\n\n\
                  COMMANDS:\n\n\
                  monitor (mon)\n\
                      🖥️  Terminal Monitor  | Real-time interactive observability (TUI)\n\
                      Engages the interactive visual engine. Optimized for real-time monitoring\n\
                      of data flows, memory consumption, and resource exploration without\n\
                      leaving the terminal environment.\n\n\
                  check\n\
                      ✅  Integrity Check   | Validate core environment and service status\n\
                      Executes a comprehensive diagnostic scan of the ZERO execution environment.\n\
                      Verifies binary integrity, configuration validity, and connector status.\n\n\
                  report\n\
                      📊  Report            | Generate formal JSON insights from recent tasks\n\
                      Compiles metadata and findings from the most recent 'perceive' task\n\
                      into a structured JSON report suitable for SIEM integration or inspect logging.\n\
                      Arguments:\n\
                      --file <FILE> : Path to write the generated JSON insight report.\n\n\
                  config (cfg)\n\
                      🛠️   Configuration     | Manage operational parameters and security tokens\n\
                      Direct interface to the Master Configuration system. Use this to\n\
                      tune tick rates, change default log levels, or manage API credentials.\n\n\
                  read\n\
                      📖  Read              | Direct zero-copy resource extraction and piping\n\
                      Provides low-level, high-speed access to raw data. Engineered for\n\
                      piping large-scale resources directly into downstream CLI tools\n\
                      (grep, awk, jq) with minimal overhead.\n\
                      Arguments:\n\
                      -f, --file <FILE> : File resource to access.\n\
                      -d, --dir <DIR>   : Directory resource to scan.\n\
                      --raw             : Disable all formatting for direct binary/text piping.\n\
                      --limit <LIMIT>   : Throughput cap (record/line limit).\n\
                      EXAMPLE:\n\
                      zc read -f data.log --limit 1000 --raw | grep \"ERROR\"\n\n\
                  inspect\n\
                      🛡️   inspect             | Focused forensic scan for PII and sensitive signatures\n\
                      Executes a security-baseline inspect against established PII patterns\n\
                      (Emails, API Keys, Credit Cards, etc.). Use '--detailed' for a\n\
                      deep-structure forensic scan that bypasses superficial checks.\n\
                      Arguments:\n\
                      -p, --data-path <PATH> : Location of the resource to be inspected.\n\
                      -s, --text <TEXT>      : Direct string input or '-' for stdin inspect.\n\
                      --detailed             : Execute deep forensic analysis.\n\
                      EXAMPLE:\n\
                      zc inspect --path users_dump.json --detailed\n\
                      echo \"secret_key=XYZ\" | zc inspect --text - \n\n\
                  --------------------------------------------------------------------------------\n\n\
                  MISSION ADVISORY:\n\
                  Use 'read --raw' only for pipeline integration.",
    styles = CLAP_STYLING
)]
struct Cli {
    /// Global system verbosity level
    #[arg(
        short,
        long,
        default_value = "info",
        value_name = "LEVEL",
        help_heading = "GLOBAL OPTIONS",
        long_help = "Sets the global filtering level for diagnostic logs.\n\
                     Supported values: trace, debug, info, warn, error.\n\
                     Default: info (standard operational visibility)."
    )]
    log_level: String,

    /// Diagnostic output structure for system alerts
    #[arg(
        long,
        value_name = "FMT",
        default_value = "human",
        help_heading = "GLOBAL OPTIONS",
        long_help = "Defines how internal system messages are rendered.\n\
                     human : ANSI-colored, multi-line readable format.\n\
                     json  : Machine-readable JSON for log aggregation.\n\
                     short : Minimalist single-line status updates."
    )]
    message_format: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 🖥️  Terminal Monitor  | Real-time interactive observability (TUI)
    ///
    /// Engages the interactive visual engine. Optimized for real-time monitoring
    /// of data flows, memory consumption, and resource exploration without
    /// leaving the terminal environment.
    #[command(visible_alias = "mon")]
    Monitor,

    /// ✅  Integrity Check   | Validate core environment and service status
    ///
    /// Executes a comprehensive diagnostic scan of the ZERO execution environment.
    /// Verifies binary integrity, configuration validity, and connector status.
    Check,

    /// 📊  Report            | Generate formal JSON insights from recent tasks
    ///
    /// Compiles metadata and findings from the most recent 'perceive' task
    /// into a structured JSON report suitable for SIEM integration or inspect logging.
    Report {
        /// Destination path for the insight report
        #[arg(
            short,
            long,
            value_name = "FILE",
            help = "Path to write the generated JSON insight report"
        )]
        file: String,
    },

    /// 🛠️   Configuration     | Manage operational parameters and security tokens
    ///
    /// Direct interface to the Master Configuration system. Use this to
    /// tune tick rates, change default log levels, or manage API credentials.
    #[command(visible_alias = "cfg")]
    Config {
        #[command(subcommand)]
        config_command: ConfigCommands,
    },

    /// 📖  Read              | Direct zero-copy resource extraction and piping
    ///
    /// Provides low-level, high-speed access to raw data. Engineered for
    /// piping large-scale resources directly into downstream CLI tools
    /// (grep, awk, jq) with minimal overhead.
    #[command(
        after_help = "EXAMPLE:\n  zc read -f data.log --limit 1000 --raw | grep \"ERROR\"",
        group(clap::ArgGroup::new("source").required(true).args(["file", "dir"]))
    )]
    Read {
        /// File resource to access
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
        /// Directory resource to scan
        #[arg(short, long, value_name = "DIR")]
        dir: Option<PathBuf>,
        /// Output raw byte-stream without formatting
        #[arg(
            short,
            long,
            default_value_t = false,
            help = "Disable all formatting for direct binary/text piping"
        )]
        raw: bool,
        /// Throughput cap (record/line limit)
        #[arg(
            short,
            long,
            value_name = "LIMIT",
            help = "Limit the number of processed records to prevent buffer overflow"
        )]
        limit: Option<usize>,
    },

    /// 🛡️   inspect             | Focused forensic scan for PII and sensitive signatures
    ///
    /// Executes a security-baseline inspect against established PII patterns
    /// (Emails, API Keys, Credit Cards, etc.). Use '--detailed' for a
    /// deep-structure forensic scan that bypasses superficial checks.
    #[command(
        after_help = "EXAMPLE:\n  zc inspect --path users_dump.json --detailed\n  echo \"secret_key=XYZ\" | zc inspect --text -",
        group(clap::ArgGroup::new("target").required(true).args(["data_path", "text"]))
    )]
    Inspect {
        /// Physical resource target for inspecting
        #[arg(
            short = 'p',
            long,
            value_name = "PATH",
            help = "Location of the resource to be inspected"
        )]
        data_path: Option<PathBuf>,
        /// Direct text stream for validation
        #[arg(
            short,
            long,
            value_name = "TEXT",
            help = "Direct string input or '-' for stdin inspect"
        )]
        text: Option<String>,
        /// Execute deep forensic analysis
        #[arg(
            long,
            default_value_t = false,
            help = "Examine structural depth and non-standard data segments"
        )]
        detailed: bool,
    },
}

use config::MasterConfig as Settings;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let settings = Settings::load().unwrap_or_default();

    let log_level =
        if cli.log_level != "info" { &cli.log_level } else { &settings.global.log_level };

    let filter = format!("{},zero_core={},cli={}", log_level, log_level, log_level);
    tracing_subscriber::fmt().with_env_filter(filter).init();

    info!("ZERO Inspector Initialized.");

    match cli.command {
        Commands::Monitor => {
            let log_buffer = Arc::new(Mutex::new(Vec::new()));
            let settings_arc = Arc::new(settings);
            tui::run_monitor(log_buffer, settings_arc).await?;
        }
        Commands::Check => {
            info!("System Integrity: OK");
        }
        Commands::Report { file } => {
            report_command(file).await?;
        }
        Commands::Config { config_command } => {
            config_command_run(&config_command).await?;
        }
        Commands::Read { file, dir, raw, limit } => {
            info!("Accessing data sources...");
            if let Some(f) = file {
                if raw {
                    info!("Reading raw content from file: {:?}", f);
                    match reader::engine::file_engine::read_file_raw_lines(&f, limit) {
                        Ok(lines) => {
                            for line in lines {
                                println!("{}", line);
                            }
                        }
                        Err(e) => anyhow::bail!("Failed to read raw file: {}", e),
                    }
                } else {
                    info!("Reading file: {:?}", f);
                }
            }
            if let Some(d) = dir {
                info!("Scanning directory: {:?}", d);
            }
        }
        Commands::Inspect { data_path, text, detailed } => {
            inspect_command(data_path, text, detailed).await?;
        }
    }

    Ok(())
}
