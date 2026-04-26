---
title: "sophon-cli – Harness Map"
phase: Research
date: "2026-04-14"
owner: "agent"
tags: [research, harness, sophon-cli, rust]

when_to_read:
  - "When validating local checks, architecture gates, tests, docs builds, or CI coverage."
  - "When changing the repository harness or deciding which command proves the repo is healthy."
summary: "Harness map for sophon-cli, describing the canonical check command, test layers, architecture boundaries, documentation gates, and known validation gaps."
ontology_relations:
  - relation: "defines"
    target: "repository-harness"
    note: "Documents the checks that protect changes in this repository."
---

# sophon-cli – Harness Map

A living map of the mechanical checks, policies, workflows, and artifacts that make change safe in this repository.

## Canonical Entry Point

- `justfile:1-6` defines the `check` recipe:
  1. `cargo fmt --check`
  2. `cargo clippy -- -D warnings -W clippy::complexity -W clippy::cognitive_complexity`
  3. `cargo test`
  4. `python3 scripts/check_markdown_frontmatter.py`
  5. `mdbook build`

There is no Makefile, npm script, or other local entrypoint. `just check` is the canonical umbrella command.

## Harness Layers

### Layer 1: Local Checks
| Check | Command | Config | Enforces |
|-------|---------|--------|----------|
| Format | `cargo fmt --check` | `Cargo.toml` edition 2024 | Rust style consistency |
| Lint / complexity | `cargo clippy -- -D warnings -W clippy::complexity -W clippy::cognitive_complexity` | `Cargo.toml` | Correctness + complexity ceiling |
| Tests | `cargo test` | `Cargo.toml` | Behavioral verification |
| Docs metadata | `python3 scripts/check_markdown_frontmatter.py` | `scripts/check_markdown_frontmatter.py` | Required Markdown frontmatter except `AGENTS.md`, `README.md`, and `docs/SUMMARY.md` |
| Docs build | `mdbook build` | `book.toml` | Documentation compiles |

### Layer 2: Architecture Boundaries
| Contract | Source | Forbidden | Config |
|----------|--------|-----------|--------|
| domain-isolation | `src/domain/` | `crate::providers`, `crate::transport`, `crate::cli`, `crate::app` | `tests/architecture_test.rs` |
| transport-isolation | `src/transport/` | `crate::providers`, `crate::cli`, `crate::app` | `tests/architecture_test.rs` |
| provider-isolation | `src/providers/` | `crate::cli`, `crate::app` | `tests/architecture_test.rs` |
| app-isolation | `src/app/` | `crate::cli` | `tests/architecture_test.rs` |
| bootstrap-isolation | `src/bootstrap/` | `crate::cli` | `tests/architecture_test.rs` |
| render_text-isolation | all except `src/cli/` | `render_text` | `tests/architecture_test.rs` |

Architecture boundary tests run as part of `cargo test`.

### Layer 3: Structural Rules
No structural rule engine is currently configured (no ast-grep, semgrep, or custom lint rule packages).

### Layer 4: Behavioral Verification
| Test Suite | Command | Location | Notes |
|------------|---------|----------|-------|
| Unit tests (inline) | `cargo test` | `src/**/*.rs` under `#[cfg(test)]` | 22 tests across source modules |
| Mapper tests | `cargo test` | `src/providers/brave/mapper.rs` | 4 tests: web, news, images, videos DTO→domain mapping |
| Provider tests | `cargo test` | `src/providers/brave/client.rs` | 1 mock-HTTP test for `BraveProvider::search` |
| App-layer tests | `cargo test` | `src/app/search_service.rs` | 1 mock-provider test for `SearchService` delegation |
| Output tests | `cargo test` | `src/cli/output.rs` | 1 text-rendering test with mixed result types |
| Architecture tests | `cargo test` | `tests/architecture_test.rs` | 6 source-scan tests enforcing layer boundaries |

No snapshot, golden, or integration test suites exist.

### Layer 5: Docs Ratchet
| Check | Command | Allowlist | Notes |
|-------|---------|-----------|-------|
| Frontmatter | `python3 scripts/check_markdown_frontmatter.py` | `AGENTS.md`, `README.md`, `docs/SUMMARY.md` | Requires `title`, `when_to_read`, `summary`, and structured `ontology_relations` |
| Docs build | `mdbook build` | n/a | Fails if markdown or `book.toml` is malformed; `scripts/mdbook_strip_frontmatter.py` strips metadata from rendered HTML |

No link checker or nav check is configured.

### Layer 5.5: Git Hooks
| Hook | Source | Installed By | Runs |
|------|--------|--------------|------|
| pre-push | `.cargo-husky/hooks/pre-push` | `cargo-husky` dev dependency during `cargo test` | `just check` |

### Layer 6: CI Matrix
| Workflow | Trigger | Checks |
|----------|---------|--------|
| `.github/workflows/validate-agents.yml` | pull requests and pushes to `main` | Verifies key `AGENTS.md` referenced paths exist, installs `just`/`mdbook`, and runs `just check` |

### Layer 7: Evidence Workflow
| Artifact | Location | Tracking | Notes |
|----------|----------|----------|-------|
| Local agent artifacts | `.artifacts/` | Ignored by Git | Research, planning, execution logs, and generated design notes may exist locally but are not repository sources of truth |

These are human-maintained local artifacts, not mechanically enforced.

### Layer 8: Operator Surface
| Surface | Location | Purpose | Usage |
|---------|----------|---------|-------|
| README | `README.md` | User-facing package overview | Read for installation, configuration, and CLI examples |
| mdBook docs | `docs/` | Maintainer and user documentation | Read for architecture and quickstart details |
| Harness map | `HARNESS.md` | Checks, hooks, and validation chain | Read before changing repository gates |

`AGENTS.md` exists at the repository root. There is no `.codex/` directory inside the repo.

## Command Chain

Ordered list of checks as executed by the canonical entry point:
1. `cargo fmt --check`
2. `cargo clippy -- -D warnings -W clippy::complexity -W clippy::cognitive_complexity`
3. `cargo test`
4. `python3 scripts/check_markdown_frontmatter.py`
5. `mdbook build`

## Quick Reference
- **Run all local checks:** `just check`
- **Run tests only:** `cargo test`
- **Run lint only:** `cargo clippy -- -D warnings -W clippy::complexity -W clippy::cognitive_complexity`
- **Run formatter check only:** `cargo fmt --check`
- **Run CI locally:** `just check` covers the workflow's canonical command; path existence checks are in `.github/workflows/validate-agents.yml`
- **Add a new check:** Edit `justfile` and append to the `check` recipe

## Source Index
| File | What It Contributes |
|------|---------------------|
| `justfile:1-6` | Canonical local check gate |
| `.cargo-husky/hooks/pre-push` | Cargo-managed pre-push hook that runs `just check` |
| `scripts/check_markdown_frontmatter.py` | Markdown frontmatter and ontology relation validator |
| `scripts/mdbook_strip_frontmatter.py` | mdBook preprocessor that keeps metadata out of rendered HTML |
| `AGENTS.md` | Operator-facing navigational map |
| `.github/workflows/validate-agents.yml` | Pull-request AGENTS path and canonical harness check |
| `docs/` | mdBook source: intro, architecture, quickstart |
| `book.toml` | mdBook configuration |
| `Cargo.toml` | Project manifest, dependencies, edition 2024, cargo-husky hook installer |
| `src/bootstrap/provider_registry.rs` | Built-in provider registry and service construction tests |
| `src/providers/brave/mapper.rs` | 4 unit tests for DTO→domain mapping |
| `src/providers/brave/client.rs` | 1 mock-HTTP unit test for Brave provider |
| `src/app/search_service.rs` | 1 mock-provider unit test for SearchService |
| `src/cli/output.rs` | 1 unit test for text output rendering |
