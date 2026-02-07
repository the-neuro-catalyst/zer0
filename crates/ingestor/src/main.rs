use clap::Parser;

use ingestor::cli::{Cli, Commands};

use ingestor::engine::{DataProcessor, ProcessorInput};

use ingestor::error::Result;

use ingestor::ingestors::sqlite::SqliteIngestor;

use ingestor::traits::ingestor::{Ingestor, IngestorConfig};

use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    info!("ZERO Ingestor Initialized.");

    match cli.command {
        Commands::Sqlite(args) => {
            info!("Mode: SQLite Ingestion");

            let config = IngestorConfig {
                database_url: Some(args.db_path),
                collection_name: args.common.collection_name.unwrap_or_default(),
                vector_size: args.common.vector_size.unwrap_or_default(),
                mappings: None,
                openai_api_key: args.common.openai_api_key,
                embed_field: args.common.embed_field,
                relationships: None,
            };

            let ingestor = SqliteIngestor::new(config).await?;
            let mut processor = DataProcessor::new(None);
            processor.initialize().map_err(ingestor::error::IngestorError::AlignmentError)?;

            // In a real scenario, we'd use the reader to get data.
            // Here we'll simulate one record for the demonstration of the pipeline.
            let input = ProcessorInput {
                data: serde_json::json!({
                    "event": "INGEST_START",
                    "source": args.path.to_string_lossy().to_string(),
                    "status": "ACTIVE"
                }),
                context: "CLI_TRIGGER".to_string(),
            };

            let output = processor
                .process_data(input)
                .map_err(ingestor::error::IngestorError::AlignmentError)?;
            ingestor.ingest(&output).await?;

            info!("Ingestion complete.");
        }
        _ => {
            error!("Command not yet fully implemented in this version.");
        }
    }

    Ok(())
}
