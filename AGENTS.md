# AGENTS.md

## Project Overview
- Rust CLI binary (`sophon-cli`) that queries the Brave Search API and prints normalized text results.
- Provider-agnostic domain layer with a Brave-specific adapter behind a trait boundary.

## Where To Start
- Harness map (checks, tests, gaps): `HARNESS.md`
- User-facing docs: `README.md` and `docs/`

## Repository Map
- `src/main.rs` — CLI entrypoint; wires `cli::args`, `app::search_service`, `providers::brave`, and `transport::http`
- `src/domain/` — provider-agnostic types (`query.rs`, `result.rs`, `types.rs`, `error.rs`, `provider.rs`)
- `src/providers/brave/` — Brave-specific DTOs, mapper, config, and `BraveProvider` client
- `src/transport/` — `HttpClient` trait and `ReqwestHttpClient` adapter
- `src/app/` — `SearchService` orchestrator (`Box<dyn SearchProvider>`)
- `src/cli/` — `clap` argument parsing (`args.rs`) and text renderer (`output.rs`)
- `tests/architecture_test.rs` — source-scan tests enforcing layer boundaries
- `docs/` — mdBook source: intro, architecture, quickstart

## Commands
- `just check` — run formatter check, clippy (with complexity/cognitive lints), tests, and mdBook docs build
- Windows PowerShell: do **not** run plain `just check`; use `just --shell powershell --shell-arg -Command check` because `just` otherwise looks for `sh`.
- `cargo test` — run all inline `#[cfg(test)]` unit tests and architecture boundary tests
- `cargo run -- "<query>"` — run a live web search (requires `BRAVE_API_KEY` in `.env`)
- `cargo run -- "<query>" --search-type news --limit 3` — run a live news search

## Boundaries
- **Domain** (`src/domain/`) — pure types and traits; no HTTP, no CLI parsing
- **Providers** (`src/providers/`) — adapter boundary; Brave DTOs map into domain types
- **Application** (`src/app/`) — orchestration only
- **CLI** (`src/cli/`) — arg parsing and output rendering only

## Sources Of Truth
- `HARNESS.md` — current harness map and validation chain
- `justfile` — canonical local check gate
- `Cargo.toml` — dependencies and edition 2024

## Observability
- Structured logging is provided by `tracing` (with `tracing-subscriber` formatting).
- Log output is written to **stderr** so stdout remains clean for CLI results.
- Control verbosity via the `RUST_LOG` environment variable (e.g. `RUST_LOG=debug`).
- Key spans: `main` (startup), `SearchService::search` (orchestration), `BraveProvider::search` / `ExaProvider::search` (provider adapters), `ReqwestHttpClient::{get_json,post_json}` (transport).

## Change Guardrails
- Run `just check` before committing. In Windows PowerShell, run `just --shell powershell --shell-arg -Command check` instead of plain `just check`.
- Keep domain types provider-agnostic; add provider-specific logic in `src/providers/`.
- Architecture boundary tests enforce import direction; if you add a cross-layer `use`, update boundaries intentionally and adjust `tests/architecture_test.rs` if needed.

## Validation Checklist
- [ ] `just check` passes, or on Windows PowerShell `just --shell powershell --shell-arg -Command check` passes (fmt, clippy, tests, docs build)
- [ ] Every path listed above still exists
