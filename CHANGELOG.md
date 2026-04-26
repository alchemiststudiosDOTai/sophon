# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Bootstrap**: Provider registry (`src/bootstrap/provider_registry.rs`) for compile-time provider registration and metadata discovery.
- **Plans**: Complete provider registry implementation plan with tickets T001–T008 under `.artifacts/plan/2026-04-25_13-50-32_provider-registry/`.
- **Interface Designs**: Visual HTML documentation for search service flow and interface options under `.artifacts/interface-designs/`.

### Changed

- **Main**: Refactored to use provider registry for provider instantiation instead of direct constructor calls.
- **Architecture Tests**: Updated to allow `bootstrap` module imports from `main.rs`.
- **HARNESS.md**: Updated harness map to reflect current validation chain.
- **Exa**: Default `/search` `contents` now requests **highlights** (with `maxCharacters` and the user query) and a **query-scoped summary** object instead of full-page **`text`**, so the API is not asked for article bodies for normal CLI usage.
- **Exa**: Normalized `snippet` is derived as **summary** (trimmed, capped) if non-empty, else **joined highlights** (separator ` … `, capped); **`text` is never used** as a snippet fallback, even when present in the response.

### Fixed

- **Exa**: Web results no longer dump full extracted page markdown into the terminal when `summary` is missing.

### Added

- **CLI**: News rows print **`snippet`** when present, matching web results and providers that populate `NewsResult.snippet`.
