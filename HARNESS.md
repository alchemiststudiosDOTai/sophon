---
title: "sophon-cli – Harness Map"
phase: Research
date: "2026-04-14"
owner: "agent"
tags: [research, harness, sophon-cli, rust]
---

# sophon-cli – Harness Map

A living map of the mechanical checks, policies, workflows, and artifacts that make change safe in this repository.

## Canonical Entry Point

- `justfile:1-5` defines the `check` recipe:
  1. `cargo fmt --check`
  2. `cargo clippy -- -D warnings -W clippy::complexity -W clippy::cognitive_complexity`
  3. `cargo test`
  4. `mdbook build`

There is no Makefile, npm script, or other local entrypoint. `just check` is the canonical umbrella command.

## Harness Layers

### Layer 1: Local Checks
| Check | Command | Config | Enforces |
|-------|---------|--------|----------|
| Format | `cargo fmt --check` | `Cargo.toml` edition 2024 | Rust style consistency |
| Lint / complexity | `cargo clippy -- -D warnings -W clippy::complexity -W clippy::cognitive_complexity` | `Cargo.toml` | Correctness + complexity ceiling |
| Tests | `cargo test` | `Cargo.toml` | Behavioral verification |
| Docs build | `mdbook build` | `book.toml` | Documentation compiles |

### Layer 2: Architecture Boundaries
| Contract | Source | Forbidden | Config |
|----------|--------|-----------|--------|
| domain-isolation | `src/domain/` | `crate::providers`, `crate::transport`, `crate::cli`, `crate::app` | `tests/architecture_test.rs` |
| transport-isolation | `src/transport/` | `crate::providers`, `crate::cli`, `crate::app` | `tests/architecture_test.rs` |
| provider-isolation | `src/providers/` | `crate::cli`, `crate::app` | `tests/architecture_test.rs` |
| app-isolation | `src/app/` | `crate::cli` | `tests/architecture_test.rs` |
| render_text-isolation | all except `src/cli/` | `render_text` | `tests/architecture_test.rs` |

Architecture boundary tests run as part of `cargo test`.

### Layer 3: Structural Rules
No structural rule engine is currently configured (no ast-grep, semgrep, or custom lint rule packages).

### Layer 4: Behavioral Verification
| Test Suite | Command | Location | Notes |
|------------|---------|----------|-------|
| Unit tests (inline) | `cargo test` | `src/**/*.rs` under `#[cfg(test)]` | 7 tests across 4 modules |
| Mapper tests | `cargo test` | `src/providers/brave/mapper.rs` | 4 tests: web, news, images, videos DTO→domain mapping |
| Provider tests | `cargo test` | `src/providers/brave/client.rs` | 1 mock-HTTP test for `BraveProvider::search` |
| App-layer tests | `cargo test` | `src/app/search_service.rs` | 1 mock-provider test for `SearchService` delegation |
| Output tests | `cargo test` | `src/cli/output.rs` | 1 text-rendering test with mixed result types |
| Architecture tests | `cargo test` | `tests/architecture_test.rs` | 5 source-scan tests enforcing layer boundaries |

No snapshot, golden, or integration test suites exist.

### Layer 5: Docs Ratchet
| Check | Command | Allowlist | Notes |
|-------|---------|-----------|-------|
| Docs build | `mdbook build` | n/a | Fails if markdown or `book.toml` is malformed |

No link checker, frontmatter validator, or nav check is configured.

### Layer 6: CI Matrix
No CI is currently configured. There is no `.github/workflows/`, `.gitlab-ci.yml`, or equivalent.

### Layer 7: Evidence Workflow
| Artifact | Location | Triggers | Format |
|----------|----------|----------|--------|
| Plan | `.artifacts/plan/2026-04-14_search-cli/PLAN.md` | Manual (plan-phase) | Markdown |
| Tickets | `.artifacts/plan/2026-04-14_search-cli/tickets/T*.md` | Plan decomposition | Markdown |
| Execution log | `.artifacts/execute/2026-04-14_search-cli.md` | Per-task updates | Markdown |

These are human-maintained research/execution artifacts, not mechanically enforced.

### Layer 8: Operator Surface
| Surface | Location | Purpose | Usage |
|---------|----------|---------|-------|
| PRD | `PRD.md` | Product requirements & design rules | Read before implementing |
| Plan | `.artifacts/plan/2026-04-14_search-cli/PLAN.md` | Implementation plan | Execute-phase reference |
| Execution log | `.artifacts/execute/2026-04-14_search-cli.md` | Debug history & task status | Update after each ticket |

`AGENTS.md` exists at the repository root. There is no `.codex/` directory inside the repo.

## Command Chain

Ordered list of checks as executed by the canonical entry point:
1. `cargo fmt --check`
2. `cargo clippy -- -D warnings -W clippy::complexity -W clippy::cognitive_complexity`
3. `cargo test`
4. `mdbook build`

## Quick Reference
- **Run all local checks:** `just check`
- **Run tests only:** `cargo test`
- **Run lint only:** `cargo clippy -- -D warnings -W clippy::complexity -W clippy::cognitive_complexity`
- **Run formatter check only:** `cargo fmt --check`
- **Run CI locally:** Not applicable (no CI configured)
- **Add a new check:** Edit `justfile` and append to the `check` recipe

## Source Index
| File | What It Contributes |
|------|---------------------|
| `justfile:1-5` | Canonical local check gate |
| `AGENTS.md` | Operator-facing navigational map |
| `docs/` | mdBook source: intro, architecture, quickstart |
| `book.toml` | mdBook configuration |
| `Cargo.toml` | Project manifest, dependencies, edition 2024 |
| `src/providers/brave/mapper.rs` | 4 unit tests for DTO→domain mapping |
| `src/providers/brave/client.rs` | 1 mock-HTTP unit test for Brave provider |
| `src/app/search_service.rs` | 1 mock-provider unit test for SearchService |
| `src/cli/output.rs` | 1 unit test for text output rendering |
| `.artifacts/plan/2026-04-14_search-cli/PLAN.md` | Implementation plan (evidence) |
| `.artifacts/execute/2026-04-14_search-cli.md` | Execution log (evidence) |
| `PRD.md` | Product requirements & operator guidance |
