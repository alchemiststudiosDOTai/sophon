---
name: sophon-cli
description: Rust CLI for provider-agnostic web search using Brave Search or Exa APIs
---

# sophon-cli Agent Skill

## Project
- Rust CLI binary (`sophon-cli`) with a provider-agnostic domain layer
- Brave Search and Exa adapters behind a trait boundary

## Key Files
- `src/main.rs` — entrypoint wiring CLI args, app service, providers, and HTTP transport
- `src/domain/` — pure types and traits (no HTTP, no CLI parsing)
- `src/providers/brave/` — Brave-specific DTOs, mapper, config, and client
- `src/providers/exa/` — Exa-specific DTOs, mapper, config, and client
- `src/transport/` — `HttpClient` trait and `ReqwestHttpClient` adapter
- `src/app/` — `SearchService` orchestrator
- `src/cli/` — `clap` argument parsing and text rendering
- `tests/architecture_test.rs` — source-scan tests enforcing layer boundaries

## Quality Gate
Run `just check` before committing. This runs:
1. `cargo fmt --check`
2. `cargo clippy -- -D warnings -W clippy::complexity -W clippy::cognitive_complexity`
3. `cargo test`
4. `mdbook build`

## Architecture Boundaries
- `src/domain/` must NOT import `crate::providers`, `crate::transport`, `crate::cli`, `crate::app`
- `src/transport/` must NOT import `crate::providers`, `crate::cli`, `crate::app`
- `src/providers/` must NOT import `crate::cli`, `crate::app`
- `src/app/` must NOT import `crate::cli`
- Only `src/cli/` may import `render_text`

## Environment Setup
Copy `.env.example` to `.env` and fill in the API key for the provider you want to use:
- `BRAVE_API_KEY` for Brave Search
- `EXA_API_KEY` for Exa

## Common Tasks
- Add a new provider: create `src/providers/<name>/` with `config.rs`, `dto.rs`, `mapper.rs`, `client.rs`, then register in `src/providers/mod.rs`
- Add a domain type: place in `src/domain/` and update `src/domain/mod.rs`
- Modify CLI output: edit `src/cli/output.rs` and add unit tests there
- Update docs: edit files in `docs/` and run `mdbook build` to verify
