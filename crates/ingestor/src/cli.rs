// ingestor/src/cli.rs

use clap::Parser;

use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
    #[clap(long)]
    pub strict: bool,
    #[clap(long)]
    pub report: bool,
    #[clap(short, long, default_value_t = 4)]
    pub concurrency: usize,
}

#[derive(Parser, Debug)]
pub enum Commands {
    Mongo(MongoArgs),
    Neo4j(Neo4jArgs),
    Postgres(PostgresArgs),
    Qdrant(QdrantArgs),
    Pinecone(PineconeArgs),
    Sqlite(SqliteArgs),
}

#[derive(Parser, Debug)]
pub struct CommonIngestorArgs {
    #[clap(long)]
    pub collection_name: Option<String>,
    #[clap(long)]
    pub vector_size: Option<u64>,
    #[clap(long, value_parser = parse_key_val, value_delimiter = ',')]
    pub map: Option<Vec<(String, String)>>,
    #[clap(long, env = "OPENAI_API_KEY")]
    pub openai_api_key: Option<String>,
    #[clap(long)]
    pub embed_field: Option<String>,
    #[clap(long)]
    pub wasm_plugin: Option<PathBuf>,
    #[clap(long)]
    pub relationships: Option<String>,
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s.find(':').ok_or_else(|| format!("invalid KEY:VALUE: no `:` found in `{}`", s))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

#[derive(Parser, Debug)]
pub struct MongoArgs {
    #[clap(long, env = "NC_MONGO_URI")]
    pub uri: String,
    #[clap(short, long)]
    pub path: PathBuf,
    #[clap(flatten)]
    pub common: CommonIngestorArgs,
}

#[derive(Parser, Debug)]
pub struct Neo4jArgs {
    #[clap(long, env = "NC_NEO4J_URI")]
    pub uri: String,
    #[clap(short, long)]
    pub path: PathBuf,
    #[clap(flatten)]
    pub common: CommonIngestorArgs,
}

#[derive(Parser, Debug)]
pub struct PostgresArgs {
    #[clap(long, env = "NC_PG_URI")]
    pub uri: String,
    #[clap(short, long)]
    pub path: PathBuf,
    #[clap(flatten)]
    pub common: CommonIngestorArgs,
}

#[derive(Parser, Debug)]
pub struct QdrantArgs {
    #[clap(long, env = "NC_QDRANT_URL")]
    pub uri: String,
    #[clap(short, long)]
    pub path: PathBuf,
    #[clap(flatten)]
    pub common: CommonIngestorArgs,
}

#[derive(Parser, Debug)]
pub struct PineconeArgs {
    #[clap(long, env = "PINECONE_API_KEY")]
    pub api_key: String,
    #[clap(long, env = "PINECONE_ENV")]
    pub environment: Option<String>,
    #[clap(short, long)]
    pub path: PathBuf,
    #[clap(flatten)]
    pub common: CommonIngestorArgs,
}

#[derive(Parser, Debug)]
pub struct SqliteArgs {
    #[clap(long, env = "SQLITE_DB_PATH")]
    pub db_path: String,
    #[clap(short, long)]
    pub path: PathBuf,
    #[clap(flatten)]
    pub common: CommonIngestorArgs,
}
