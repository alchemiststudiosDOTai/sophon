---
title: "Changelog"
when_to_read:
  - "When reviewing user-visible changes across releases or unreleased work."
  - "When preparing release notes or checking recent provider and CLI behavior changes."
summary: "Chronological project change log for sophon-cli, used to track additions, fixes, and behavior changes that matter to operators and users."
ontology_relations:
  - relation: "records"
    target: "sophon-cli-release-history"
    note: "Tracks release-facing changes for the CLI."
---

# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `--provider all` fan-out mode that queries every environment-enabled provider and renders per-provider successes and failures. (#11)
- Provider registry composition layer for built-in provider registration, provider metadata discovery, and `SearchService` construction. (#7)
- Environment-filtered structured tracing spans for startup, search orchestration, provider adapters, and HTTP transport. Logs are written to stderr so CLI result output stays clean. (#8)
- Markdown frontmatter validation and mdBook frontmatter stripping in the canonical `just check` gate. (#9)
- Cargo-managed pre-push hook that runs `just check`. (#9)
- Repository operating surfaces: `.env.example`, CODEOWNERS, issue and PR templates, label definitions, AGENTS validation CI, and a `sophon-cli` agent skill. (#6)

### Changed

- `main.rs` now selects providers through `ProviderId` and `ProviderRegistry` instead of directly constructing Brave and Exa clients. (#7)
- Production startup registers only providers with valid environment configuration, and provider-unavailable errors list configured providers. (#7)
- Architecture tests now include the `bootstrap` composition layer boundary. (#7)
- `HARNESS.md` and architecture docs now reflect the bootstrap layer, docs metadata guard, cargo-husky hook, and current validation chain. (#7, #9)
- Exa `/search` requests use highlights plus a query-scoped summary instead of requesting full-page `text` for normal CLI output. (#5)
- Exa snippet normalization now prefers trimmed summaries, then capped joined highlights; `text` is not used as a snippet fallback. (#5)
- CLI news output now prints `snippet` when present, matching web-result rendering. (#5)

### Fixed

- Exa web results no longer dump full extracted page markdown into the terminal when `summary` is missing. (#5)

### Removed

- Tracked `.artifacts/` planning and execution files; future local artifact output is ignored by Git. (#9)
- Legacy pre-commit configuration in favor of the Cargo-managed pre-push hook. (#9)

### Security

- Structured logging avoids recording provider authentication headers or API keys. (#8)
