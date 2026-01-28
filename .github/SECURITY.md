# SECURITY.md

## 1. Supported Versions

| Version | Status | Supported          |
| ------- | ------ | ------------------ |
| 0.1.x   | Beta   | :white_check_mark: |
| < 0.1   | Legacy | :x:                |

---

## 2. Technical Security Properties

ZERO utilizes specific technical implementations to mitigate common risks. These are properties of the current architecture, not absolute security guarantees.

### I. Memory and Execution

- **Implementation Language**: Core components are written in Rust to leverage compile-time memory safety.
- **Unsafe Code Policy**: Application-level crates minimize the use of `unsafe` blocks to maintain memory integrity.
- **Data Ingestion**: Input is constrained by schema validation at entry points to mitigate injection-style risks.

### II. Identity and Secrets

- **Environment Isolation**: Secrets and keys are never committed to the repository. They are managed via environment variables.
- **Data Protection**: Application-level secrets are handled via the Stronghold protocol (AES-256-GCM).
- **Log Privacy**: Observability layers are configured to redact common sensitive patterns from output logs.

### III. Network and Sandbox

- **Encryption**: Communications with external sinks (e.g., PostgreSQL, Qdrant) utilize TLS 1.2+.
- **Frontend Isolation**: The UI environment is proxied through the Rust core to maintain separation from the host system.

---

## 3. Reporting a Vulnerability

Do not open public issues for security vulnerabilities. Report them directly to:

**Email:** [developer@zero.theneurocatalyst.com](mailto:developer@zero.theneurocatalyst.com)

Please provide:

- Description of the finding.
- Steps to reproduce.
- Potential technical impact.

Response target: 48 hours.
