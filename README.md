
# `UNLICENSE` ZERO - Universal High-Performance Data Suite, Read everything, Fast, Locally.

<div align="center">
<a href="https://www.producthunt.com/products/zero-unlicense?embed=true&amp;utm_source=badge-featured&amp;utm_medium=badge&amp;utm_campaign=badge-zero-unlicense" target="_blank" rel="noopener noreferrer"><img alt="ZERO | UNLICENSE - Sub-millisecond inspection for massive Parquet, S3 and more. | Product Hunt" width="250" height="54" src="https://api.producthunt.com/widgets/embed-image/v1/featured.svg?post_id=1069244&amp;theme=light&amp;t=1769576962207"></a>
</div>

> [!NOTE]
>
> ## ⚠️ Reality Check
>
> Built by one person. Bugs exist. That's normal, not a problem.
>
> If you find issues:
>
> - GitHub Issues (I respond within 24 hours)
> - Discord (active daily)
> - Slack (community helps)
>
> Perfection is a corporate lie. This is real work by a real human.

---

ZERO is a high-velocity, local-first engine for inspecting massive Parquet, S3, SQL, and Vector data. Built in Rust for sub-millisecond precision, it provides a universal bridge to your data without the cloud-bloat, privacy leaks, or the "complexity tax" of legacy enterprise tools.

---

## ⚡ PERFORMANCE LEVERAGE

ZERO is engineered for hardware-level efficiency. By utilizing memory-mapped I/O and zero-copy processing, we deliver performance that legacy Electron-based tools cannot match.

| Metric | Standard Tools | ZERO Engine | Leverage Factor |
| :--- | :--- | :--- | :--- |
| **Binary Size** | 120 MB+ | **~25 MB** | **5x Smaller** |
| **RAM Footprint** | 2GB - 4GB+ | **< 100 MB** | **20x+ Lighter** |
| **Startup Time** | 2 - 5 Seconds | **< 300ms** | **10x Faster** |
| **Data Locality** | Cloud-first | **100% Local** | **Absolute Privacy** |

---

## 🛠️ UNIVERSAL ACCESS

One engine, multiple entry points. Total visibility across your entire stack.

### 1. The Matrix (TUI)

High-speed terminal interface for SSH sessions and server forensics.

- **Hex View**: Raw binary inspection.
- **Live Metrics**: Real-time record counts and sampling.

### 2. The Cortex (GUI)

Visual structural mapping and drag-and-drop workflow built with **Tauri v2**.

- **Massive File Support**: Open 2GB+ Parquet/Logs with zero latency.

### 3. The Eyes (AI Extension)

A specialized **MCP** module that allows AI Agents (Claude, Gemini, Cursor) to perceive raw local data and SQL schemas directly.

---

## ✨ SUPPORT MATRIX

ZERO handles almost every modern format out-of-the-box:

- **Structured**: Parquet, CSV, JSON, JSONL, XML, TOML, YAML, XLSX/ODS.
- **Database**: SQLite, PostgreSQL, MySQL.
- **Cloud/Stream**: AWS S3, Kafka, RabbitMQ.
- **Forensics**: PII Redaction, Secret Detection, EXIF Analysis.

---

## 🚀 GETTING STARTED

### Installation & Build

1. **Build All Components:** `cargo build --release`
2. **Setup AI Layer:** `cd extensions/zero-gemini-extension && pnpm install && gemini extensions link .`

### Utility Scripts

- **Production Build:** `./scripts/release-ui.sh`, `./scripts/release-tui.sh`
- **Environment Setup:** `./scripts/setup_test_data.sh` (Generates PII-laden data for validation).

---

## 🌐  For ZERO

Connect with the project and the community:

- **Landing Page:** [zero.theneurocatalyst.com](https://zero.theneurocatalyst.com)

- **Source Code:** [github.com/the-neuro-catalyst/zer0](https://github.com/the-neuro-catalyst/zer0)

- **Slack (Community):** [Join Slack](https://join.slack.com/t/zero-byy4555/shared_invite/zt-3o65k8f2p-mIaEH8Hvb~UFSA0wolg5XA)

- **Discord (Dev):** [Join Discord](https://discord.gg/HCPXuC55HV)

- **X (Updates):** [@NeuroCatalyst](https://x.com/NeuroCatalyst)

*Note: If you need code fixes or support, we'll get to it as soon as the cat stops sitting on the keyboard (and we have a lot of cats).*

---

## 💼 Operational

**Free Tools, Not Insight.**

The ZERO engine is open and free. However, systemic transformation requires deeper architectural insight.

- **Architectural Consulting**: Strategic oversight for regional-scale organizations and above.
- **Structural Integrity**: Optimization of internal operational flows and public-facing infrastructure.
- **Strategic Calibration**: Aligning technical architecture with organizational objectives.

**Contact:**

- **Email:** [hi@theneurocatalyst.com](mailto:hi@theneurocatalyst.com)
- **Web:** [theneurocatalyst.com](https://theneurocatalyst.com)

> *"Good Image ≠  Loyalty -- Structural truth is the only terminal state."*

---

## ⚖️ LICENSE

Released under **UNLICENSE**. Simple. Direct. Optimized.
