# ZERO Gemini CLI Extension: User Guide

**What this is:** MCP server that connects ZERO's Rust data engine and **provides deep web project analysis** to Gemini CLI

**Read this if:** You're using Gemini CLI and need to inspect local data files, databases, **or analyze web project codebases**

---

## Quick Start

This extension gives Gemini CLI direct access to ZERO's data inspection tools. Instead of uploading files or describing data manually, Gemini can read schemas, run queries, scan for sensitive information, **and deeply analyze web project codebases (React, Next.js, etc.)** directly on your machine.

**Installation:** Follow MCP server setup in Gemini CLI documentation, point to `zero-mcp-gemini` binary

**Verify:** Run `/zero:inspect <filepath>` in Gemini CLI

---

## How It Works

```
Your Question → Gemini CLI → MCP Server → ZERO Rust Engine (+ Node.js Analysis) → Results
```

Gemini sends tool requests through the MCP protocol. The server calls ZERO's compiled Rust binaries (`reader`, `ingestor`) to process data, **or executes Node.js analysis logic for web projects**. Results return as structured JSON.

**Example:**

- You: "What's the schema of data.parquet?"
- Gemini calls: `inspect_resource(path="data.parquet")`
- ZERO returns: Column names, types, row count, sample data
- Gemini shows: Formatted summary in chat

---

## Available Tools

### Data Inspection

**`inspect_resource`**  
Get schema, metadata, and sample rows from a file or URL.

```typescript
// Gemini calls this internally when you ask about a file
{
  path: "sales_data.parquet",
  head: 10  // Number of sample rows (optional)
}
```

Returns: Column types, row count, information density score, whether file contains PII/secrets

**`get_database_schema`**  
Extract table and column structure from a database (Rust-based).

```typescript
{
  url: "sqlite://analytics.db"
  // or "postgres://user:pass@localhost/mydb"
}
```

Returns: List of tables with column names and types

**`query_database`**  
Run read-only SQL queries.

```typescript
{
  url: "sqlite://data.db",
  query: "SELECT COUNT(*) FROM users WHERE created_at > '2024-01-01'"
}
```

⚠️ **Read-only:** `INSERT`, `UPDATE`, `DELETE` will fail

**`analyze_join_keys`**  
Find common columns between two data sources.

```typescript
{
  sourceA: "customers.csv",
  sourceB: "sqlite://orders.db"
}
```

Returns: Suggested join columns (case-insensitive matching)

### File Operations

**`search_files`**  
Search for patterns using ripgrep.

```typescript
{
  pattern: "API_KEY",
  dirPath: "src/",       // Optional, defaults to current dir
  include: "*.py"        // Optional, glob pattern
}
```

**`list_tree`**  
Show directory structure.

```typescript
{
  path: "./data",
  maxDepth: 3  // Optional, default is 3
}
```

**`scan_patterns`**  
Auto-scan for PII, API keys, passwords, tokens.

```typescript
{
  dirPath: "."  // Optional, defaults to current dir
}
```

Uses predefined regex patterns for common secrets and sensitive data.

### Data Loading

**`ingest_data`**  
Load data into SQLite with optional vector embeddings.

```typescript
{
  db_path: "my_data.db",
  collection_name: "documents",
  path: "articles.json",
  vector_size: 1536,           // Optional
  openai_api_key: "sk-...",    // Optional, for embeddings
  embed_field: "content"       // Optional, field to embed
}
```

---

### Web Project Analysis (New!)

These tools leverage Node.js runtime to perform deeper analysis of web application codebases.

**`analyze_project`**  
Understand the project's overall structure, detected frameworks (React, Next.js), TypeScript usage, and key dependencies.

**`analyze_dependencies`**  
Get a categorized list of project dependencies (e.g., UI, state management, styling, build tools) from `package.json`.

**`get_components`**  
Identify and analyze React components, extracting details about exports, inferred props, state management (useState, setState), effects (useEffect), and JSX usage.

**`get_routing_structure`**  
Map the application's routing configuration, including defined paths, associated components, and potential protected routes.

**`get_tailwind_usage`**  
Analyze Tailwind CSS class usage patterns across the codebase, providing statistics on most used classes and categorizing common patterns (layout, colors, typography, responsive).

**`get_hooks_usage`**  
Detect and quantify the usage of both built-in React hooks (e.g., useState, useEffect) and custom hooks within the codebase.

**`analyze_api_calls`**  
Identify and characterize external API calls made within the codebase, including usage of `fetch`, `axios`, and Supabase client methods, along with detected API endpoints.

**`analyze_database_schema`**  
Inspect database schemas as defined within the codebase (e.g., from SQL migration files, TypeScript type definitions for ORMs, or Supabase schema definitions), listing tables, types, functions, and relationships.

---

## Project Resources

Gemini can read these special resources to get structured information about the project:

- **`project://structure`**: Overview of the project's file and directory structure.
- **`project://package`**: Raw content of the `package.json` file.
- **`project://components`**: Detailed analysis of React components found in the project.
- **`project://routes`**: Analysis of the application's routing configuration.

---

## Prompt Templates

These templates enable Gemini to perform complex tasks by generating specific prompts based on your request:

- **`code_review`**: Review React component code for improvements in structure, performance, accessibility, and best practices.
  - *Arguments:* `component_path`
- **`refactor_suggest`**: Analyze a file for refactoring opportunities to improve code duplication, function extraction, naming, error handling, and readability.
  - *Arguments:* `file_path`
- **`performance_audit`**: Perform an audit on a specified component to check for unnecessary re-renders, memory leaks, bundle size impact, and runtime bottlenecks.
  - *Arguments:* `component_name`

---

## CLI Commands

These are shortcuts exposed directly in Gemini CLI:

**`/zero:inspect <path> [rows]`**  
Inspect file metadata and sample data.

```
/zero:inspect data.parquet
/zero:inspect logs.csv 20
```

Shows: Information density, structural depth, PII/secrets flag

**`/zero:scan [pattern]`**  
Scan for security risks or custom patterns.

```
/zero:scan                    # Scan for common secrets
/zero:scan "internal_token"   # Search for specific pattern
```

**`/zero:ingest_data`**  
Start guided ingestion workflow (Gemini will prompt for parameters).

---

## Design Principles

The extension follows ZERO's operational style:

**Perception First**  
Before acting on data, tools map structure (`list_tree`, `inspect_resource`, **`analyze_project`**). Gemini sees the workspace before making decisions.

**Clean Standards**  
Outputs are direct and functional. No marketing language, no unnecessary formatting.

**Privacy First**  
If `inspect_resource` or `scan_patterns` returns `compromised: true`, it means PII or secrets were detected. You'll see a warning before any data is processed further.

**Zero Friction**  
No API keys required (except for optional embedding features). No data leaves your machine unless you explicitly configure cloud features.

---

## Typical Workflows

### Understanding New Data

1. You: "What's in this file?"
2. Gemini → `inspect_resource` → Shows schema + sample rows
3. You: "Are there any issues?"
4. Gemini → `scan_patterns` → Reports PII/secrets if found

### Database Analysis

1. You: "What tables are in analytics.db?"
2. Gemini → `get_database_schema` → Lists tables
3. You: "Show me recent orders"
4. Gemini → `query_database` → Runs SQL

### Web Project Audit (New!)

1. You: "What frameworks does this project use?"
2. Gemini → `analyze_project` → Reports React, Next.js, TypeScript, etc.
3. You: "List all React components and their props."
4. Gemini → `get_components` → Provides a detailed list.
5. You: "Are there any performance issues in `MyBigComponent`?"
6. Gemini → `performance_audit(component_name='MyBigComponent')` → Analyzes and suggests optimizations.
7. You: "Summarize the project's API calls."
8. Gemini → `analyze_api_calls` → Details fetch, axios, Supabase usage.

### Finding Data Relationships (across files/DBs and code)

1. You: "Can I join customers.csv with the orders table?"
2. Gemini → `analyze_join_keys` → Suggests common columns
3. You: "Show me how the `User` table is used in the codebase."
4. Gemini → `analyze_database_schema` (web-based) → Finds code definitions and references to `User` table.
5. You write SQL using those suggestions

---

## Limitations

**No Direct Modification**  
Tools operate in read-only mode by default. Only `ingest_data` writes to disk, and only to SQLite databases.

**Requires Rust Binaries**  
The extension calls compiled binaries in `target/release/`. If these aren't built, tools will fail.

**Platform-Specific Commands**  
`find` and `ripgrep` behavior may vary on Windows vs Linux/macOS.

**Not for Real-Time Streams**  
Works with static files and databases. Not designed for live Kafka/streaming analysis (though it can connect to buffers).

**Node.js Analysis Limitations**  
Web project analysis relies on regex pattern matching and file system traversal, not full AST parsing. While powerful, it might miss highly complex or dynamic code patterns.

---

## Troubleshooting

**Tool not found errors**  
Run `cargo build --release` to compile Rust binaries.

**Permission denied on database**  
Check file permissions. For PostgreSQL, verify connection string has correct credentials.

**Slow performance on large files**  
Use `head` parameter in `inspect_resource` to limit sample size. ZERO is fast, but 10GB files still take time to parse.

**PII warnings appearing**  
`scan_patterns` is conservative. Review flagged content manually—false positives happen with test data.

---

## Architecture Details

### Three Layers

1. **Rust Engine** (`reader`, `ingestor`)  
    Memory-mapped I/O for Parquet/CSV/SQL. Handles actual data processing.

2. **Node.js MCP Server** (`zero-mcp-gemini`)  
    Exposes Rust tools and **Node.js-based web analysis tools** via MCP protocol. Manages tool registration and argument parsing.

3. **Gemini CLI**  
    Calls MCP tools based on your questions. Formats responses for chat.

### Data Flow

```
File/DB/Code → (Rust Reader or Node.js Analysis) → JSON Output → MCP Server → Gemini → You
```

All processing happens locally. No data is sent to external services unless you configure embedding features with your own API key.

---

## Related Documentation

- **ZERO Main README:** Core tool overview and installation
- **Gemini CLI MCP Docs:** How to configure MCP servers
- **ZERO Technical Docs:** [zero.theneurocatalyst.com/docs](https://zero.theneurocatalyst.com/docs)
