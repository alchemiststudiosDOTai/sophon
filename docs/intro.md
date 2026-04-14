# sophon-cli

A provider-agnostic Rust CLI that queries the Brave Search API and prints normalized text results.

## What it does

- Parses CLI arguments
- Builds a provider-agnostic `SearchQuery`
- Delegates to a `SearchProvider` (currently Brave)
- Renders results as human-readable text

## Project structure

- `src/domain/` — pure types and traits; no HTTP, no CLI
- `src/transport/` — `HttpClient` trait + `reqwest` adapter
- `src/providers/brave/` — Brave-specific DTOs, mapper, and client
- `src/app/` — `SearchService` orchestrator
- `src/cli/` — argument parsing and output rendering
- `tests/architecture_test.rs` — boundary tests enforcing layer isolation

## Running checks

```bash
just check
```

This runs `cargo fmt --check`, `cargo clippy` (with complexity lints), and `cargo test`.
