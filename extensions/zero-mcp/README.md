# ZERO MCP Server

**What this is:** Model Context Protocol server for AI agents to inspect local data files and databases

**Compatible with:** Claude Desktop, Cline, any MCP-compatible AI tool

---

## Quick Start

Add to your MCP settings file:

```json
{
  "mcpServers": {
    "zero-mcp": {
      "command": "node",
      "args": ["/path/to/zero-mcp/dist/index.js"]
    }
  }
}
```

**Requirements:**

- Node.js installed
- ZERO Rust binaries compiled (`cargo build --release`)
- `ripgrep` installed for search features

---

## What AI Agents Can Do

When connected, AI agents can:

- Read schemas from Parquet/CSV/JSON files without loading full datasets
- Query SQLite/PostgreSQL databases (read-only)
- Search codebases for patterns or security issues
- Scan for PII, API keys, secrets automatically
- Find join keys between different data sources
- Load data into SQLite with optional vector embeddings

**Note:** Specific ZERO MCP extensions (like `zero-mcp-gemini`) can further expand these capabilities with specialized tools for domains like web project analysis.

**Key difference from manual work:** Agent sees actual data structure, not your description of it.

---

## Available Tools

### `inspect_resource`

Read file metadata and sample rows.

**Input:**

```typescript
{
  path: "data.parquet",
  head: 10  // Optional, defaults to 10
}
```

**Returns:** JSON with schema, column types, row count, sample data

**How it works:** Calls Rust `reader` binary with 50MB output buffer for large schemas

**Use when:** Agent needs to understand file structure before processing

---

### `search_files`

Search for patterns using ripgrep.

**Input:**

```typescript
{
  pattern: "TODO:",
  dirPath: "src/",     // Optional
  include: "*.ts"      // Optional glob
}
```

**Returns:** Line-by-line matches with file paths

**Use when:** Agent needs to find specific code, config values, or occurrences

---

### `list_tree`

Show directory structure up to max depth.

**Input:**

```typescript
{
  path: "./data",
  maxDepth: 3  // Optional, defaults to 3
}
```

**Returns:** File tree using `find` command (hidden files excluded)

**Use when:** Agent needs workspace overview before file operations

---

### `scan_patterns`

Auto-detect sensitive data across directory.

**Input:**

```typescript
{
  dirPath: "."  // Optional
}
```

**Scans for:**

- API_KEY, SECRET, PASSWORD, TOKEN
- access_key, private_key, Bearer
- Email addresses (regex: `[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}`)

**Returns:** Matches or "No sensitive patterns detected"

**Use when:** Security audit, pre-commit checks, compliance verification

---

### `query_database`

Execute read-only SQL queries.

**Input:**

```typescript
{
  url: "sqlite://analytics.db",
  // or "postgres://user:pass@localhost/mydb"
  query: "SELECT * FROM users LIMIT 5"
}
```

**Returns:** Query results as text

**Restrictions:** Write operations (`INSERT`, `UPDATE`, `DELETE`) will fail

**Use when:** Agent needs to verify data content or run analytics

---

### `get_database_schema`

Extract table and column definitions.

**Input:**

```typescript
{
  url: "sqlite://data.db"
}
```

**Returns:** List of tables with column names and types

**Use when:** Agent needs to understand database structure before querying

---

### `analyze_join_keys`

Find common columns between two sources.

**Input:**

```typescript
{
  sourceA: "customers.csv",
  sourceB: "sqlite://orders.db"
}
```

**Process:**

1. Reads schemas from both sources
2. Compares column names (case-insensitive)
3. Returns intersection

**Returns:**

```json
{
  "sourceA_columns": ["id", "name", "email"],
  "sourceB_columns": ["customer_id", "order_date"],
  "potential_join_keys": ["id"] // or warning if none found
}
```

**Use when:** Agent is designing queries across multiple datasets

---

### `ingest_data`

Load data into SQLite database.

**Input:**

```typescript
{
  db_path: "my_data.db",
  collection_name: "documents",
  path: "source_data.json",
  vector_size: 1536,           // Optional
  openai_api_key: "sk-...",    // Optional
  embed_field: "content"       // Optional
}
```

**Process:** Calls Rust `ingestor` binary to parse and load data

**Optional features:**

- `vector_size` + `openai_api_key` + `embed_field`: Generate embeddings for specified field

**Use when:** Agent needs to consolidate data for querying or RAG workflows

---

## Architecture

```
AI Agent → MCP Protocol → Node.js Server → Rust Binaries → Data
```

### Component Responsibilities

**Node.js layer (`index.ts`):**

- Registers MCP tools with Zod schemas
- Spawns child processes to Rust binaries
- Handles errors and output parsing

**Rust layer (`reader`, `ingestor`):**

- Memory-mapped file I/O
- Parquet/CSV/JSON parsing
- Database connections
- Vector embedding generation

### Binary Paths

Server expects compiled binaries at:

```
../../../target/release/reader
../../../target/release/ingestor
```

Relative to `dist/index.js` location.

---

## Error Handling

**Common errors:**

```typescript
// Binary not found
"Execution Error: spawn ENOENT"
→ Run `cargo build --release`

// Ripgrep not installed
"Execution Error: spawn rg ENOENT"
→ Install ripgrep: `brew install ripgrep` or `apt install ripgrep`

// Database connection failed
"Database Error: connection refused"
→ Check URL format and credentials

// Buffer overflow
"maxBuffer exceeded"
→ File schema too large (default: 50MB)
```

**Exit codes:**

- `ripgrep` returns exit code 1 when no matches found (handled as success)
- Rust binaries return non-zero on errors (captured in stderr)

---

## Limitations

**Read-only by default:** Only `ingest_data` writes to disk

**No streaming:** Tools load full results into memory before returning

**Platform dependencies:**

- `find` command behavior varies (Linux vs macOS vs Windows)
- `ripgrep` must be in PATH

**Schema extraction assumptions:**

- Database tools assume standard table structures
- `analyze_join_keys` uses simple name matching (no semantic analysis)

---

## Security Considerations

**Local execution only:** All data processing happens on the machine running the MCP server

**No network calls except:**

- Database connections (when using PostgreSQL URLs)
- Optional OpenAI API for embeddings (user must provide key)

**Sensitive data scanning:**

- `scan_patterns` uses basic regex, not ML-based detection
- False positives common in test data
- Email regex catches all email-like strings (including comments)

---

## Debugging

**Enable verbose output:**

```bash
# Run server directly to see stdout/stderr
node dist/index.js
```

**Test tools individually:**

```bash
# Test Rust reader
./target/release/reader --file data.csv --head 5 --format json

# Test ripgrep
rg "pattern" --vimgrep --no-heading
```

**Check MCP connection:**

- Most AI tools have MCP server logs in settings
- Look for "zero-mcp" connection status
- Verify tool registration appears in agent capabilities

---

## Differences from Gemini CLI Extension

| Feature | `zero-mcp` (this) | `zero-mcp-gemini` |
|---------|-------------------|-------------------|
| CLI commands | No | Yes (`/zero:inspect`, `/zero:scan`) |
| Protocol | Standard MCP | Gemini-specific MCP |
| Compatible with | Claude, Cline, etc. | Gemini CLI only |
| Installation | MCP settings JSON | Gemini CLI config |
| Specialized Web Analysis | No | Yes |

Both call the same Rust binaries. Only the Node.js wrapper differs, with `zero-mcp-gemini` providing additional specialized analysis.

---

## Related Documentation

- **ZERO Main README:** Core tool overview and installation
- **MCP Specification:** <https://modelcontextprotocol.io>
- **Rust binaries:** See `reader/` and `ingestor/` crates in repo
