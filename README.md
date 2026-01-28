# `ZERO` - Universal Data Inspection Engine

**Sub-millisecond access to massive datasets. Zero cloud dependency. Zero BS.**

---

<div align="center">
<tr>
<img src="public/screenshot/product_hunt/tui-all-colors-edited-ph.png" alt="zero tui" width="500"/>
</tr>
<tr>
<img src="public/screenshot/product_hunt/05.png" alt="zero gui" width="500"/>
</tr>

*A link containing [**ALL SCREENSHOTS**](docs/all-image.md) in a single Markdown file; no need to click on each image individually.*

</div>

## What It Does

Inspect Parquet, S3, SQL, Vector databases, and 20+ formats at hardware speed.

- **Performance**: 5-20x lighter than Electron bloatware
- **Privacy**: 100% local. Your data never leaves your machine.
- **Speed**: Sub-300ms startup. Memory-mapped I/O. Zero-copy processing.

Built in Rust. One binary. No runtime dependencies.

---

## Performance Reality

| Metric | Legacy Tools | ZERO |
|--------|-------------|------|
| **Binary Size** | 120 MB+ | **~25 MB** |
| **RAM Usage** | 2-4 GB | **<100 MB** |
| **Startup** | 2-5 sec | **<300 ms** |
| **Privacy** | Cloud-first | **Local-only** |

---

## Three Interfaces

### 1. Matrix (TUI)

Terminal interface for SSH/server environments.

- Hex inspection
- Live metrics
- Zero GUI overhead

### 2. Cortex (GUI)

Visual mapping with drag-and-drop workflows.

- Handle 2GB+ files
- Built with Tauri v2
- Native performance

### 3. Eyes (AI Extension)

MCP module for AI agents (Claude, Gemini, Cursor).

- Direct data perception
- Schema inspection
- No API middleman

---

## Supported Formats

**Structured:** Parquet, CSV, JSON, JSONL, XML, TOML, YAML, XLSX/ODS  
**Database:** SQLite, PostgreSQL, MySQL  
**Cloud/Stream:** AWS S3, Kafka, RabbitMQ  
**Forensics:** PII redaction, secret detection, EXIF analysis

---

## Installation

```sh
cargo build --release
```

**Production builds:**

```sh
./scripts/release-ui.sh   # GUI
./scripts/release-tui.sh  # Terminal
```

---
> [!NOTE]
>
> ## Reality Check
>
> **Built by one person. Bugs exist.**
>
> **If you find issues:**
>
> - **GitHub Issues** (24h response)
> - **Discord** (active daily)
> - **Slack** (community support)
>
> *Perfection is corporate theater. This is real work.*

---

## Links

- **Landing:** [zero.theneurocatalyst.com](https://zero.theneurocatalyst.com)
- **Source:** [github](https://github.com/the-neuro-catalyst/zer0)
- **Discord:** [discord](https://discord.gg/HCPXuC55HV)
- **Slack:** [Join Community](https://join.slack.com/t/zero-byy4555/shared_invite/zt-3o65k8f2p-mIaEH8Hvb~UFSA0wolg5XA)

---

## Operational Support

**ZERO** engine is free and complete (Unlicense).

**For organizations deploying at regional scale or above:**

- **Architectural Consulting**: Strategic system design
- **Structural Integrity**: Internal flow optimization
- **Strategic Calibration**: Alignment with organizational objectives

**Contact:** <hi@theneurocatalyst.com> | [theneurocatalyst.com](https://theneurocatalyst.com)

---

## License

**UNLICENSE**

Do whatever you want. No restrictions.
