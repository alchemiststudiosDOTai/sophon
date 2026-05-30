# AGENTS.md

## Project Overview

- Rust CLI binary (`sophon-cli`) that queries configured search providers and prints normalized text results.
- Provider-agnostic domain layer with Brave and Exa adapters behind trait boundaries.
- Repository process state is a markdown-only control plane under `docs/artifacts/`, `docs/process/`, and `docs/project/`.

## Read First

- Artifact system index: `docs/artifacts/INDEX.md`
- Process rules: `docs/process/RULES.md` and `docs/process/WORKFLOW.md`
- Project context: `docs/project/PROJECT_CONTEXT.md`, `docs/project/CONSTRAINTS.md`, `docs/project/ARCHITECTURE.md`, `docs/project/TESTING.md`
- Current work: `docs/artifacts/active/README.md`, `docs/artifacts/open-issues/README.md`, and the latest `docs/artifacts/memory/MEM-*.md`
- Harness map: `HARNESS.md`
- User-facing docs: `README.md` and `docs/`

## Repository Map

- `src/main.rs` - CLI entrypoint; delegates runtime execution to `sophon_cli::cli::runner::run_from_env`.
- `src/lib.rs` - library module surface.
- `src/domain/` - provider-agnostic query, result, type, error, and provider contracts.
- `src/providers/brave/` - Brave DTOs, mapper, config, and provider client.
- `src/providers/exa/` - Exa DTOs, mapper, config, and provider client.
- `src/transport/` - `HttpClient` trait and `ReqwestHttpClient` adapter.
- `src/app/` - single-provider and fan-out search orchestration.
- `src/bootstrap/` - provider registry and service construction.
- `src/cli/` - `clap` args, request construction, runner, and text output rendering.
- `tests/` - architecture, CI direction, changelog, fan-out CLI, and integration tests.
- `docs/` - mdBook source plus process, project, and artifact-control docs.

## Commands

- `just check` - canonical gate: `cargo fmt --check`, clippy with complexity lints, `cargo test`, markdown frontmatter check, and `mdbook build`.
- `just hygiene` - dependency, duplication, tech-debt, and large-file checks.
- `cargo test` - run unit, architecture, and integration tests.
- `cargo run -- "<query>" --provider brave` - run Brave search with `BRAVE_API_KEY`.
- `cargo run -- "<query>" --provider exa` - run Exa search with `EXA_API_KEY`.
- `cargo run -- "<query>" --provider all` - query every environment-enabled provider.
- Windows PowerShell: use `just --shell powershell --shell-arg -Command check` instead of plain `just check`.

## Architecture Boundaries

- Domain (`src/domain/`) stays pure: no HTTP, CLI, app, or provider imports.
- Transport (`src/transport/`) has no provider, CLI, or app imports.
- Providers (`src/providers/`) adapt external APIs into domain types and do not import CLI or app layers.
- Application (`src/app/`) orchestrates domain contracts only; it does not import CLI, bootstrap, providers, or transport.
- Bootstrap (`src/bootstrap/`) wires provider registry/service construction and does not import CLI.
- CLI (`src/cli/`) owns parsing, request construction, runner behavior, and text rendering.
- Boundary tests in `tests/architecture_test.rs` enforce these contracts.

## Artifact Workflow

- Core rule: No Charter = No Code.
- Pick exactly one primary mode: Exploration, Bug Fix, Refactor, Feature, QA / Verification, Documentation, or Release Prep.
- Exploration ends with `docs/artifacts/explorations/EXP-YYYYMMDD-short-slug.md`.
- Execution starts with `docs/artifacts/active/CHARTER-YYYYMMDD-short-slug.md`.
- During execution, update `docs/artifacts/active/EXEC-YYYYMMDD-short-slug.md`.
- Before claiming success, create `docs/artifacts/evidence/EVID-YYYYMMDD-short-slug.md`.
- End every real work session with `docs/artifacts/memory/MEM-YYYYMMDD-short-slug.md`.
- Durable decisions require `docs/artifacts/decisions/ADR-0000-short-slug.md`.

## Sources Of Truth

- `HARNESS.md` - validation chain, test layers, CI, and known harness gaps.
- `justfile` - canonical local check and hygiene commands.
- `Cargo.toml` - package metadata, dependencies, Rust edition, and integration test targets.
- `docs/process/` - workflow rules and handoff protocol.
- `docs/project/` - project context, constraints, architecture, and testing guides.
- `.github/workflows/validate-agents.yml` - CI validation path and command order.

## Change Guardrails

- Run `just check` before committing. If skipped, document why in the evidence pack.
- Keep `AGENTS.md` a compact map; move detailed process text into `docs/process/` or `docs/artifacts/`.
- New tracked Markdown files must pass `python3 scripts/check_markdown_frontmatter.py` unless explicitly exempted in that script.
- Keep domain types provider-agnostic; put provider-specific behavior in `src/providers/`.
- If a boundary change is intentional, update `tests/architecture_test.rs`, `HARNESS.md`, and relevant docs together.
- Do not stage secrets, `.env`, generated build output, or `.DS_Store`.

## Validation Checklist

- [ ] `just check` passes, or the skipped/failed check is documented in evidence.
- [ ] Every concrete path listed in this file exists; naming patterns point to existing directories.
- [ ] New staged Markdown files satisfy the frontmatter checker.
- [ ] Evidence and session memory artifacts exist for real work sessions.
