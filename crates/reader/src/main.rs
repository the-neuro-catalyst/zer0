use clap::{CommandFactory, Parser, Subcommand};

use std::path::PathBuf;

use tracing::{error, info};

mod analysis;
mod engine;
mod error;
mod output;
mod reader_result;
mod readers;

use engine::file_engine::FileReaderOptions;

use output::{OutputFormat, OutputMode};
#[derive(Parser)]
#[command(
    author,
    version,
    about = "ZERO Data Reader - Perception Layer",
    long_about = r#"ZERO Data Reader (zero-reader): Universal Data Inspector & Extractor

zero-reader is a versatile CLI tool designed for high-performance reading, analysis, and extraction of data from various sources. Whether it's local files, databases, or S3 buckets, zero-reader helps you quickly access the data you need.

Usage:
You can run zero-reader using `cargo run` or directly execute the binary if it has been built.

*   Via Cargo:
    cargo run --bin zero-reader -- [COMMAND/OPTIONS]
*   Via Direct Binary (after build):
    ./target/debug/zero-reader [COMMAND/OPTIONS]

---

1. General Options:

zero-reader can be used to read files or directories directly, with options for controlling output and processing.

Options:
  -f, --file <FILE>          Specify the path to the file to read
  -d, --dir <DIR>            Specify the path to the directory to read
      --head <HEAD>          (Default: 10) Read up to HEAD items/lines from the beginning of the data
  -o, --output <PATH>        Specify the output file path for results
  -F, --format <FORMAT>      (Default: json) Output format for results (json, yaml, csv, text)
  -m, --mode <MODE>          (Default: default) Processing mode (default, schema-only, full-raw, analyze, stream)
      --pii-redaction        Enable PII (Personally Identifiable Information) redaction in the output (default: true)
      --zero-copy            Enable zero-copy memory mapping for faster data access (default: true)
  -r, --recursive            Process files in directories recursively
  -h, --help                 Display usage information
  -V, --version              Display zero-reader version

Examples:

*   Read a JSON file and show the first 5 lines:
    cargo run --bin zero-reader -- --file data.json --head 5
*   Read an entire directory and save results as YAML:
    cargo run --bin zero-reader -- --dir ./my_data_folder --recursive --format yaml --output output.yaml

---

2. Subcommands:

zero-reader includes subcommands for more specific operations.

2.1. `search` - Search Data:
Used to search for a pattern within a specified file.

Options:
*   `--pattern <PATTERN>`: The pattern (string or regex) to search for
*   `--path <PATH>`: The path to the file to search

Example:
*   Search for an IP address in an access log file:
    cargo run --bin zero-reader -- search --pattern \"\\b\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\b\" --path /var/log/nginx/access.log

2.2. `db` - Read from Database (Feature: `database`):
Extracts data from a database using a connection URL and a specified query.

(Note: zero-reader must be compiled with the `database` feature enabled, e.g., `cargo run --bin zero-reader --features database -- db ...`)

Options:
*   `--url <DATABASE_URL>`: Connection URL for the database (e.g., postgres://user:pass@host:port/dbname)
*   `--query <SQL_QUERY>`: The SQL query to execute

Example:
*   Retrieve active users from PostgreSQL:
    cargo run --bin zero-reader --features database -- db --url \"postgresql://user:password@localhost:5432/mydb\" --query \"SELECT * FROM users WHERE status = 'active';\"

2.3. `db-schema` - Extract Database Schema:
Extracts the schema (structure) of the specified database.

Options:
*   `--url <DATABASE_URL>`: Connection URL for the database

Example:
*   Extract the schema of an SQLite database:
    cargo run --bin zero-reader --features database -- db-schema --url \"sqlite://./data.db\"

2.4. `s3` - Read from AWS S3 (Feature: `cloud`):
Accesses and reads data from an S3 bucket.

(Note: zero-reader must be compiled with the `cloud` feature enabled, e.g., `cargo run --bin zero-reader --features cloud -- s3 ...`)

Options:
*   `--bucket <BUCKET_NAME>`: The name of the S3 bucket
*   `--key <OBJECT_KEY>`: The key of the object to read in the S3 bucket

Example:
*   Read a JSON file from S3:
    cargo run --bin zero-reader --features cloud -- s3 --bucket my-data-bucket --key \"logs/today.json\"
"#
)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    #[arg(short, long, value_name = "DIR")]
    dir: Option<PathBuf>,

    #[arg(long, default_value_t = 10)]
    head: usize,

    #[arg(short, long, value_name = "PATH", help = "Output file path for results")]
    output: Option<PathBuf>,

    #[arg(
        short = 'F',
        long,
        value_enum,
        default_value = "json",
        help = "Output format for results (json, yaml, csv, text)"
    )]
    format: OutputFormat,

    #[arg(
        short,
        long,
        value_enum,
        default_value = "default",
        help = "Processing mode (default, schema-only, full-raw, analyze, stream)"
    )]
    mode: OutputMode,

    #[arg(
        long,
        default_value_t = true,
        help = "Redact sensitive PII data in output (default: true)"
    )]
    pii_redaction: bool,

    #[arg(
        long,
        default_value_t = true,
        help = "Enable zero-copy memory mapping for faster access (default: true)"
    )]
    zero_copy: bool,

    #[arg(short, long, default_value_t = false, help = "Process directories recursively")]
    recursive: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Search {
        #[arg(short, long)]
        pattern: String,
        #[arg(short, long)]
        path: PathBuf,
    },
    #[cfg(feature = "database")]
    Db {
        #[arg(long)]
        url: String,
        #[arg(short, long)]
        query: String,
    },
    DbSchema {
        #[arg(long)]
        url: String,
    },
    #[cfg(feature = "cloud")]
    S3 {
        #[arg(long)]
        bucket: String,
        #[arg(long)]
        key: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic logging setup
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    info!("ZERO Data Reader Initialized.");

    let options = FileReaderOptions {
        head: Some(cli.head),
        file_type_override: None,
        output_mode: cli.mode,
        output_format: cli.format,
        pii_redaction: cli.pii_redaction,
        zero_copy: cli.zero_copy,
        recursive: cli.recursive,
        filter_exts: None,
        output_path: cli.output.clone(),
    };

    if let Some(ref command) = cli.command {
        match command {
            Commands::Search { pattern, path } => {
                info!("Searching for '{}' in {:?}", pattern, path);
                let results = engine::search_engine::search_in_file(path, pattern).await?;
                println!("{}", serde_json::to_string_pretty(&results)?);
            }
            #[cfg(feature = "database")]
            Commands::Db { url, query } => {
                info!("Reading from database: {}", url);
                let db_options = engine::db_engine::DatabaseReaderOptions {
                    db_type: engine::db_engine::DatabaseType::from_url(url),
                    db_url: url.clone(),
                    query: query.clone(),
                };
                let result = engine::db_engine::read_database_content(db_options).await?;
                println!("{}", serde_json::to_string_pretty(&result)?);
            }
            Commands::DbSchema { url } => {
                info!("Extracting schema from: {}", url);
                let schema = engine::db_engine::get_database_schema(url).await?;
                println!("{}", serde_json::to_string_pretty(&schema)?);
            }
            #[cfg(feature = "cloud")]
            Commands::S3 { bucket, key } => {
                info!("Reading from S3: {}/{}", bucket, key);
            }
            #[allow(unreachable_patterns)]
            _ => {
                error!("Command requires additional features to be enabled.");
            }
        }
        return Ok(());
    }

    if let Some(file_path) = cli.file {
        info!("Reading file: {:?}", file_path);
        let result = engine::file_engine::read_file_content(&file_path, options).await?;
        if cli.format == OutputFormat::Json {
            println!("{}", serde_json::to_string_pretty(&result)?);
        } else {
            println!("{:?}", result);
        }
    } else if let Some(dir_path) = cli.dir {
        info!("Reading directory: {:?}", dir_path);
        let result = engine::file_engine::read_directory_content(&dir_path, options).await?;
        println!("{:?}", result);
    } else {
        let _ = Cli::command().print_help();
    }

    Ok(())
}
