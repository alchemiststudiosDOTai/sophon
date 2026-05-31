---
title: "sophon-cli"
when_to_read:
  - "When reading or editing the mdBook documentation surface."
  - "When checking how the CLI architecture, quickstart, and user-facing docs fit together."
summary: "mdBook documentation page for sophon-cli: sophon-cli. It contributes user and maintainer guidance that is built by the docs gate."
ontology_relations:
  - relation: "part_of"
    target: "docs/SUMMARY.md"
    note: "Belongs to the mdBook documentation set."
---

# sophon-cli

A provider-agnostic Rust CLI that queries Brave Search, Exa, or every environment-enabled provider and prints normalized text results.

## What it does

- Parses CLI arguments
- Builds a provider-agnostic `SearchQuery`
- Resolves real provider tokens through the bootstrap provider catalog
- Delegates to a single `SearchProvider` for `--provider brave` or `--provider exa`
- Fans out sequentially to all environment-enabled providers for `--provider all`
- Renders single-provider or per-provider fan-out results as human-readable text

## Project structure

- `src/domain/` — pure types and traits; no HTTP, no CLI
- `src/transport/` — `HttpClient` trait + `reqwest` adapter
- `src/providers/brave/` — Brave-specific DTOs, mapper, and client
- `src/providers/exa/` — Exa-specific DTOs, mapper, and client
- `src/bootstrap/` — provider catalog, provider registry, and service construction
- `src/app/` — `SearchService` and `FanoutSearchService` orchestrators
- `src/cli/` — argument parsing and output rendering
- `tests/architecture_test.rs` — boundary tests enforcing layer isolation

## Running checks

```bash
just check
```

This runs `cargo fmt --check`, `cargo clippy` (with complexity lints), and `cargo test`.
