![GitHub License](https://img.shields.io/github/license/the-neuro-catalyst/zer0?logo=unlicense&logoColor=ffffff&logoSize=auto&label=License&color=fff&cacheSeconds=3600)

 


# `ZERO` - Universal Data Inspector and AI MCP Auditor for Enterprises

**Category:** Local data analysis tool for structured/semi-structured files and databases  
**Brief:** Command-line & local web UI to inspect and slice Parquet/CSV/JSON/SQL files without leaving the machine; remote analysts can access it via a secure URL.

![ZERO_COVER](https://zero.theneurocatalyst.com/og_images/zero_og.png)


---

## Use When
 
| Scenario | Description | Don’t Use When | Feature | Reason |
|----------|-------------|----------------|---------|--------|
| **Local Inspection** | Inspect Parquet, CSV, JSON, SQL databases without cloud upload or SaaS export | **Real-Time Collaboration** | **Cloud Sync** | Does not sync to cloud storage (Dropbox, S3) or send data outside the local machine |
| **Restricted Environments** | Work over SSH, in air-gapped networks, or on heavily restricted corporate machines | **SaaS-Only Data** | **Pre-built Dashboards** | Does not provide pre-built dashboards, charts, or business KPIs (focus is on data inspection) |
| **Large Files** | Handle files >500 MB where spreadsheet/online tools time out or crash | **Shared Dashboards** | **Auto-Generated Insights** | Does not auto-generate summaries or "recommendations" (no LLM business commentary) |
| **Privacy-Critical Data** | Work with medical, financial, or internal logs that must stay on-prem | **Non-Technical Users** | **Live Streaming** | Does not connect to live streaming APIs (Kafka, Pub/Sub) without a local buffer (e.g. local file / DB) |
| **Remote Access** | Allow data scientists/engineers to inspect data via browser without shell access | | | |
 

***


| Category | Formats | Metric | Electron Tools | ZERO |
|----------|---------|--------|----------------|------|
| **Tabular** | Parquet, CSV, JSONL, XLSX, ODS | Binary size | 120MB+ | 25MB |
| **Document** | JSON, XML, TOML, YAML | RAM usage | 2-4GB | <100MB |
| **Database** | SQLite, PostgreSQL, MySQL | Startup time | 2-5s | <300ms |
| **Remote** | S3, Kafka, RabbitMQ | Data location | Cloud-first | Local-only |
| **Analysis** | PII redaction, secret scanning, EXIF metadata | | | |
---

## Architecture

| Component | Description | Technology |
|-----------|-------------|------------|
| **Data Flow** | Raw Data → Rust Reader → Node.js MCP Server → AI Client | |
| **Rust Layer** | Memory-mapped I/O for Parquet/CSV/SQL. Zero-copy processing | Rust |
| **MCP Layer** | Node.js server exposing tools to AI agents | Node.js |
| **Client Layer** | Gemini CLI or Claude Desktop integration | AI Agents |

### Data Flow Phases
| Phase | Tools | Description |
|-------|-------|-------------|
| **Perception** | `list_tree` → `search_files` → `scan_patterns` | Map workspace structure and content |
| **Analysis** | `inspect_resource` → `get_database_schema` → **(New: Web Project Introspection)** | Calculate quality metrics, visualize schema, and understand web application internals |
| **Execution** | Apply zero-utils maintenance scripts or ingest operations | |
---

## Constraints

| Constraint | Description |
|------------|-------------|
| **Read-Only by Default** | All tools operate in read-only mode except `ingest_data`. Prevents accidental data modification |
| **Privacy-First** | If `inspect_resource` or `scan_patterns` returns `compromised: true`, user action required before proceeding |
| **Binary Dependencies** | Requires compiled Rust binaries in `target/release`:<br>- `reader` (data parsing)<br>- `ingestor` (SQLite loading) |
| **Shell Variability** | Commands like `find` and `rg` may behave differently across OS platforms |

---

### Remote Access via URL (Key Feature)

| Feature | Description |
|---------|-------------|
| **Local Service** | ZERO Data Inspector runs as a local service (e.g. Node.js/Python HTTP server) on the data machine |
| **Internal Exposure** | Internally exposes a simple web UI and REST API on `http://localhost:$PORT` for file/database inspection |
| **Secure Tunnel** | Host uses a secure tunnel (e.g. `cloudflared`, `loclx`) to expose that local port as a public URL like `https://zero-data-abc123.example.com` |
| **Remote Access** | Analysts can open that URL in their browser to:<br>- Browse files in the protected directory<br>- Run ad-hoc queries on Parquet/CSV/JSON/SQL<br>- View samples, schemas, and basic stats |
| **Security** | Access is secured via:<br>- Short-lived token in the URL (e.g. `?token=xyz`) or basic auth<br>- Network-level controls (IP allowlist, firewall rules) |
| **Example URL** | `https://zero-data-abc123.example.com?token=readonly-xyz` |
| **Note** | While remote access provides convenience, performance for very large files (e.g., >500MB) will inherently depend on network bandwidth and latency, and may not be as instantaneous as local access |

---

### Interfaces

| Interface | Description | Technology |
|-----------|-------------|------------|
| **Matrix (TUI)** | Terminal interface. For SSH sessions, headless servers, or when GUI overhead matters | Terminal |
| **Cortex (GUI)** | Tauri-based desktop app. Handles 2GB+ files with native performance | Tauri |
| **Eyes (MCP Extension)** | Model Context Protocol server for AI agents (Claude, Gemini, Cursor). **Provides deep web project introspection**, direct schema inspection, and context-aware tool access | AI Agents |


---

### Installation

```sh
cargo build --release
./scripts/release-ui.sh   # GUI (Tauri)
./scripts/release-tui.sh  # Terminal
```

Binary outputs: `~25MB` | RAM: `<100MB` | Startup: `<300ms`


---

## MCP Tools (Gemini/Claude Extension)

| Category | Tool | Description | Example Usage |
|----------|------|-------------|----------------|
| **Core Operations** | `inspect_resource` | Schema + metadata + sample rows from file/URL. Returns information density metrics. | `/zero:inspect data.csv 5` |
|  | `query_database` | Read-only SQL execution against SQLite/PostgreSQL. | `SELECT * FROM users LIMIT 10` |
|  | `get_database_schema` | Extract table/column structure visualization. | `/zero:schema` |
|  | `analyze_join_keys` | Suggest join columns between two data sources. | `/zero:join users orders` |
| **Workspace Utilities** | `search_files` | Regex search via `ripgrep`. | `/zero:search "error"` |
|  | `list_tree` | Directory structure up to max depth. | `/zero:tree --depth 3` |
|  | `scan_patterns` | Auto-detect PII, API keys, secrets. | `/zero:scan` |
|  | `ingest_data` | Load data into SQLite with optional embeddings. | `/zero:ingest data.csv` |
| **Web Project Analysis** | `analyze_project` | Understand project structure and technologies. | `/zero:analyze` |
|  | `get_components` | Analyze React components and their structure. | `/zero:components` |
|  | `get_routing_structure` | Map application routing and paths. | `/zero:routes` |
|  | `analyze_dependencies` | Categorize project dependencies. | `/zero:deps` |
|  | `get_tailwind_usage` | Analyze Tailwind CSS class usage. | `/zero:tailwind` |
|  | `get_hooks_usage` | Detect and quantify React hook usage. | `/zero:hooks` |
|  | `analyze_api_calls` | Identify and characterize API calls. | `/zero:apis` |
|  | `analyze_database_schema` | Inspect code-defined database schemas. | `/zero:db-schema` |
 
## Links

- Docs: [Read more](https://zero.theneurocatalyst.com/docs)
- Source: [Download](https://github.com/the-neuro-catalyst/zer0)
- Discord: [Join Now](https://discord.gg/HCPXuC55HV)
- X.com: [Follow Now](https://x.com/@NeuroCatalyst)

**License:** LICENSE (public domain)

**Enterprise Support:** Regional deployment consulting available at <hi@theneurocatalyst.com>

 
