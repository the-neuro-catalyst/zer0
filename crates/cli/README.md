# `LICENSE` **ZERO CLI (`zc`)** — The Industrial-Grade Control Lever

> **"Deterministic data auditing at the speed of the void."**

The ZERO CLI (`zc`) is a high-performance, zero-copy data inspection engine designed for deep resource mapping, security forensics, and real-time monitoring. Engineered in Rust for environments where data integrity and low-latency transparency are non-negotiable.

---

## 🛠️ CORE OPERATIONAL COMMANDS

| Command      | Action                 | Mission Profile                                                    |
| :----------- | :--------------------- | :----------------------------------------------------------------- |
| `zc audit`   | **Security Forensics** | Targeted scanning for PII, compromised signatures, and leaks.      |
| `zc monitor` | **TUI Observability**  | Real-time, interactive terminal interface for resource monitoring. |
| `zc read`    | **High-Speed Access**  | Zero-copy stream extraction directly to stdout or pipelines.       |
| `zc report`  | **Insight Synthesis**  | Generates structured audit logs (JSON) for downstream automation.  |

---

## 🌐 GLOBAL CONTROL FLAGS

*Global parameters governing engine behavior and diagnostics.*

| Flag               | Shortcut | Description                                                                           |
| :----------------- | :------- | :------------------------------------------------------------------------------------ |
| `--log-level`      | `-l`     | **System Verbosity:** Sets logging depth (`trace`, `debug`, `info`, `warn`, `error`). |
| `--message-format` | (N/A)    | **Diagnostic Structure:** Output format for system alerts (`human`, `json`, `short`). |
| `--help`           | `-h`     | **Contextual Guidance:** Displays technical documentation and command hierarchy.      |
| `--version`        | `-V`     | **Build Verification:** Returns the current binary version and build state.           |

---

## 🔍 COMMAND-SPECIFIC LEVERS

### 1. `audit` — Security Forensics

| Flag          | Shortcut | Description                                                                      |
| :------------ | :------- | :------------------------------------------------------------------------------- |
| `--data-path` | `-p`     | **Static Target:** Path to the physical resource for baseline auditing.          |
| `--text`      | `-t`     | **Stream Audit:** Direct text input for immediate validation/scanning.           |
| `--detailed`  | (N/A)    | **Deep Forensics:** Executes exhaustive structural analysis for hidden patterns. |

### 3. `read` — High-Speed Access

| Flag               | Shortcut    | Description                                                                      |
| :----------------- | :---------- | :------------------------------------------------------------------------------- |
| `--file` / `--dir` | `-f` / `-d` | **Target Selection:** Specifies a single file or an entire directory for access. |
| `--raw`            | `-r`        | **Unfiltered Pipe:** Outputs raw byte-stream/lines without formatting.           |
| `--limit`          | `-l`        | **Throughput Cap:** Constraints the number of records/lines accessed.            |

---

## 🖥️ INTERACTIVE MONITOR (TUI) SHORTCUTS

*Low-latency navigation within the real-time observability interface.*

| Key         | Action               | Mission Value                                                |
| :---------- | :------------------- | :----------------------------------------------------------- |
| `q` / `Esc` | **Terminate**        | Safe exit from the monitor mode.                             |
| `Tab`       | **Cycle Focus**      | Rotates focus between Viewports (Log, Metric, Hex).          |
| `↑` / `↓`   | **Vertical Scroll**  | Precise navigation through data streams or metadata lists.   |
| `f`         | **Real-time Filter** | Dynamic search within the active resource buffer.            |
| `Enter`     | **Inspect**          | Explodes the selected item for detailed metadata inspection. |

---

## ⚡ ENTERPRISE LEVERAGE EXAMPLES

### 🛡️ Instant Security Audit

Scan a massive production log for PII and output a JSON forensics report:

```bash
zc audit --path /var/log/prod_high_density.csv --detailed > audit_report.json
```

### 📖 High-Speed Data Piping

Extract 10,000 records from a large resource directly into a downstream processor:

```bash
zc read -f large_dataset.csv --limit 10000 --raw | grep "CRITICAL"
```

---

By **The Neuro-Catalyst Group**
