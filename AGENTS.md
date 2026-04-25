# AGENTS.md

## Project Overview
- Rust CLI binary (`sophon-cli`) that queries the Brave Search API and prints normalized text results.
- Provider-agnostic domain layer with a Brave-specific adapter behind a trait boundary.

## Where To Start
- Implementation plan: `.artifacts/plan/2026-04-14_search-cli/PLAN.md`
- Harness map (checks, tests, gaps): `HARNESS.md`

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

## Change Guardrails
- Run `just check` before committing.
- Keep domain types provider-agnostic; add provider-specific logic in `src/providers/`.
- Architecture boundary tests enforce import direction; if you add a cross-layer `use`, update boundaries intentionally and adjust `tests/architecture_test.rs` if needed.

## Validation Checklist
- [ ] `just check` passes (fmt, clippy, tests, docs build)
- [ ] Every path listed above still exists
