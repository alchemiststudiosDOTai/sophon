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

- **Bootstrap**: Provider registry (`src/bootstrap/provider_registry.rs`) for compile-time provider registration and metadata discovery.
- **Docs Guard**: Markdown frontmatter validator and Cargo-managed pre-push hook for the canonical `just check` gate.

### Changed

- **Main**: Refactored to use provider registry for provider instantiation instead of direct constructor calls.
- **Architecture Tests**: Updated to allow `bootstrap` module imports from `main.rs`.
- **HARNESS.md**: Updated harness map to reflect current validation chain.
- **Artifacts**: `.artifacts/` is now ignored and no longer tracked in Git.
- **Exa**: Default `/search` `contents` now requests **highlights** (with `maxCharacters` and the user query) and a **query-scoped summary** object instead of full-page **`text`**, so the API is not asked for article bodies for normal CLI usage.
- **Exa**: Normalized `snippet` is derived as **summary** (trimmed, capped) if non-empty, else **joined highlights** (separator ` … `, capped); **`text` is never used** as a snippet fallback, even when present in the response.

### Fixed

- **Exa**: Web results no longer dump full extracted page markdown into the terminal when `summary` is missing.

### Added

- **CLI**: News rows print **`snippet`** when present, matching web results and providers that populate `NewsResult.snippet`.
