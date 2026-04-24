# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **Exa**: Default `/search` `contents` now requests **highlights** (with `maxCharacters` and the user query) and a **query-scoped summary** object instead of full-page **`text`**, so the API is not asked for article bodies for normal CLI usage.
- **Exa**: Normalized `snippet` is derived as **summary** (trimmed, capped) if non-empty, else **joined highlights** (separator ` … `, capped); **`text` is never used** as a snippet fallback, even when present in the response.

### Fixed

- **Exa**: Web results no longer dump full extracted page markdown into the terminal when `summary` is missing.

### Added

- **CLI**: News rows print **`snippet`** when present, matching web results and providers that populate `NewsResult.snippet`.
