# Workspace Architecture: Modular Resource Management

## Design Principles

- **Baseline Initialization:** Defining the initial state for data processing tasks.
- **Overhead Reduction:** Identifying and removing redundant logic in legacy systems.
- **Optimization via Simplification:** Achieving performance gains through modular and direct implementation.

## Implementation Phases

### 1. Perception Layer (Discovery)

- **Rust Engine (`reader`):** High-performance data ingestion for Parquet, CSV, JSON, and SQL.
- **Tools:** `list_tree`, `search_files`, `scan_patterns`.
- **Goal:** Rapid mapping of workspace structure and content.

### 2. Analysis Layer (Synthesis)

- **Information Density:** Automated calculation of data quality metrics.
- **Schema Mapping:** Visualization of database and file structures.
- **Tools:** `inspect_resource`, `get_database_schema`.
- **Goal:** Understanding data utility without manual inspection.

### 3. Deployment Layer (Execution)

- **Zero Utils:** Maintenance scripts and procedural guidance.
- **Goal:** Keeping the workspace organized and efficient.

## Data Flow

`Raw Source` -> `Rust Reader` -> `Node.js MCP` -> `Gemini Client`

*Note: This architecture prioritizes "Zero Friction" interaction patterns.*
