# CONTRIBUTING.md: Operational Standards

## 1. Objective

All contributions must improve systemic efficiency or stability. Subjective or narrative-driven changes are classified as noise and will be rejected.

## 2. Technical Constraints

- **Schema Fidelity**: All modifications must strictly align with the data contracts in `crates/schema`.
- **Memory Safety**: Application-level code must avoid `unsafe` blocks. Use Rust’s ownership model to ensure integrity.
- **Efficiency**: Prioritize zero-copy operations and minimize memory footprint.
- **Verification**: Functional changes must be supported by unit or integration tests.

## 3. Workflow

1. **Rationale**: Identify the technical constraint or inefficiency being addressed.
2. **Implementation**: Adhere to existing project style and architectural patterns.
3. **Validation**: Execute `cargo test` and `pnpm tsc` before submission.
4. **Submission**: Use the provided Pull Request template.

## 4. Noise Reduction

Communication must be restricted to technical facts and data. Flattery, social fillers, and non-technical agendas will be pruned to maintain focus on the objective.
