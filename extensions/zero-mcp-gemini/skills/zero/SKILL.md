---
name: zero-utils
description: General-purpose workspace utility collection. Provides helper scripts and common references for project maintenance and data alignment.
---

# ZERO Utils: Operational Guidance

Standard operating procedures for workspace management, data alignment, and deep technical analysis. Use these utilities to maintain high-efficiency standards and extract ground truth from diverse data sources.

## Core Mandates

1. **Perception first:** Use `list_directory` and `search_files` to understand the workspace structure before performing maintenance.
2. **Clean Standards:** All outputs and implementations must adhere to the project's direct and functional style (Janitor's Standard).
3. **Automated Validation:** Utilize scripts in the `scripts/` directory for routine system checks.
4. **Irreversible Perception:** Once a systemic inefficiency is identified, prioritize its simplification—this perception is irreversible.

## Technical Capabilities

- **Deep Inspection:** Use `inspect_resource` with the `head` parameter to sample data and calculate information density.
- **Schema Mapping:** Use `get_database_schema` to visualize SQL structures before writing queries.
- **Pattern Recognition:** Use `scan_patterns` to identify PII or sensitive data anomalies.
- **Connection Analysis:** Use `analyze_join_keys` to correlate data between Parquet files and SQL tables.

## Procedures

- **Data Alignment:** Identify the data source type (File vs. Database), map the structure, and ensure consistency between sources (Parquet, SQL, CSV).
- **Maintenance:** Run `scan-integrity.sh` periodically to ensure the workspace remains organized and free of redundant files.
- **Refactoring Support:** Assist in simplifying complex directory structures or data formats based on identified inefficiencies.

## Constraints

- **Read-Only Intelligence:** Do not attempt to modify data sources directly unless explicitly instructed for maintenance tasks.
- **Privacy First:** Always report if `compromised: true` is flagged in metadata during inspection.
