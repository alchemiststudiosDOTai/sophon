---
title: "Project Context"
when_to_read:
  - "When orienting on the sophon-cli repository before implementation work."
summary: "Summarizes the product, users, workflows, architecture, constraints, tests, and current risks for sophon-cli."
ontology_relations:
  - relation: "summarizes"
    target: "sophon-cli-project"
    note: "Provides the top-level project context that AGENTS.md points future agents to read."
---

# Project Context

| Field | Value |
|---|---|
| Artifact Type | Project Context |
| Status | Active |
| Date | 2026-05-30 |
| Owner | Project maintainers |
| Related Artifacts | `docs/artifacts/INDEX.md` |
| Related Files | `README.md`, `HARNESS.md`, `Cargo.toml`, `justfile` |

## Product / System

`sophon-cli` is a Rust command-line search tool. It accepts a search query, sends it to configured provider APIs, maps provider-specific responses into shared domain results, and prints normalized text output.

Supported provider modes:

- `brave`
- `exa`
- `all`, which queries every provider enabled by environment variables

## Users

- CLI users who want quick search results from Brave, Exa, or all configured providers.
- Maintainers changing provider adapters, CLI behavior, search orchestration, or validation gates.
- Agents working under the markdown artifact workflow.

## Current Goal

Keep the provider-agnostic CLI maintainable while preserving strict layer boundaries, validation gates, and documentation evidence for changes.

## Important Workflows

- Local search: `cargo run -- "<query>" --provider brave|exa|all`
- Full validation: `just check`
- Harness hygiene: `just hygiene`
- Documentation build: `mdbook build`
- Execution sessions: create a charter, execution log, evidence pack, and memory artifact under `docs/artifacts/`

## Current Architecture Summary

The CLI layer parses arguments and builds domain search requests. Bootstrap constructs provider-backed services. Application services orchestrate single-provider or fan-out searches through domain traits. Provider adapters map Brave or Exa DTOs into domain result types and use the transport boundary for HTTP.

## Current Constraints Summary

- Domain types and traits stay provider-agnostic.
- Provider-specific behavior belongs under `src/providers/`.
- CLI parsing/output stays under `src/cli/`.
- Execution work starts with a Session Charter and ends with evidence plus memory.
- New tracked Markdown must pass the frontmatter checker.

## Current Testing Summary

`just check` is the canonical proof command. It runs Rust formatting, clippy with complexity lints, all tests, Markdown frontmatter validation, and mdBook build. See `docs/project/TESTING.md` and `HARNESS.md` for details.

## Known Current Risks

- Live provider calls depend on `BRAVE_API_KEY` and/or `EXA_API_KEY` and are not part of automated tests.
- The docs ratchet validates Markdown metadata but does not currently run a link checker.
- The markdown control-plane docs are new and should stay compact to avoid duplicating `AGENTS.md`.

## Latest Reliable Memory

Use the latest `docs/artifacts/memory/MEM-*.md` file when one exists.
