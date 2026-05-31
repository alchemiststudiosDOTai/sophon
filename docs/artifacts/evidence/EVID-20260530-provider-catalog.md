---
title: "Evidence: provider catalog"
when_to_read:
  - "When verifying the provider catalog refactor implementation."
  - "When checking why provider catalog execution was considered complete."
summary: "Evidence that provider identity, CLI parsing, production provider wiring, stable ordering, and docs now route through the bootstrap provider catalog."
ontology_relations:
  - relation: "supports"
    target: "docs/artifacts/active/CHARTER-20260530-provider-catalog.md"
    note: "Provides validation evidence for the provider catalog session charter."
  - relation: "implements"
    target: "docs/artifacts/decisions/ADR-0001-provider-catalog.md"
    note: "Shows the durable provider catalog decision was applied in code, tests, and docs."
---

# Evidence Pack

| Field | Value |
|---|---|
| Artifact Type | Evidence Pack |
| Status | Completed |
| Date | 2026-05-30 |
| Owner | Codex |
| Related Artifacts | `docs/artifacts/active/CHARTER-20260530-provider-catalog.md`, `docs/artifacts/active/EXEC-20260530-provider-catalog.md`, `docs/artifacts/decisions/ADR-0001-provider-catalog.md` |
| Related Files | `src/bootstrap/provider_catalog.rs`, `src/bootstrap/provider_registry.rs`, `src/bootstrap/mod.rs`, `src/cli/args.rs`, `src/cli/runner.rs`, `tests/architecture_test.rs`, `tests/integration/provider_registry_test.rs`, `tests/integration/cli_test.rs`, `README.md`, `docs/intro.md`, `docs/quickstart.md`, `docs/project/ARCHITECTURE.md`, `docs/architecture.md`, `docs/dependency-architecture-map.md`, `docs/dependency-architecture-map.html`, `HARNESS.md` |

## Claim Being Proven

Provider identity, CLI token lookup, display names, environment variable names, stable ordering, and production construction are now sourced from a bootstrap provider catalog. `all` remains a CLI-only aggregate mode, and the registry plus CLI no longer maintain independent real-provider wiring lists.

## Files Reviewed

- `docs/artifacts/active/CHARTER-20260530-provider-catalog.md`
- `docs/artifacts/active/EXEC-20260530-provider-catalog.md`
- `docs/artifacts/decisions/ADR-0001-provider-catalog.md`
- `src/bootstrap/provider_registry.rs`
- `src/bootstrap/mod.rs`
- `src/cli/args.rs`
- `src/cli/runner.rs`
- `tests/architecture_test.rs`
- `tests/integration/provider_registry_test.rs`
- `tests/integration/cli_test.rs`
- `README.md`
- `docs/intro.md`
- `docs/quickstart.md`
- `docs/project/ARCHITECTURE.md`
- `docs/architecture.md`
- `docs/dependency-architecture-map.md`
- `docs/dependency-architecture-map.html`
- `HARNESS.md`

## Files Changed

- Added `src/bootstrap/provider_catalog.rs` with `ProviderId`, `ProviderCatalogEntry`, the ordered `PROVIDER_CATALOG`, catalog lookup helpers, env-var hint formatting, display-name iteration, and production builders for Brave and Exa.
- Updated `src/bootstrap/provider_registry.rs` to iterate catalog entries for production registration and available-provider order.
- Updated `src/bootstrap/mod.rs` to expose `provider_catalog`.
- Updated `src/cli/args.rs` so real provider tokens parse through the catalog, while `all` remains CLI-only.
- Updated `src/cli/runner.rs` to run `CliProvider::Single(ProviderId)` directly and render about text from catalog display names.
- Added architecture and integration tests for catalog ownership, catalog metadata, CLI token parsing, and unknown-provider errors.
- Updated README, mdBook docs, project architecture docs, dependency map docs, and harness source index to point provider registration at the catalog.

## Commands Run

```bash
cargo test --test architecture_test test_provider_catalog_is_provider_wiring_source_of_truth
cargo test --test architecture_test test_provider_catalog_is_provider_wiring_source_of_truth
cargo test --test provider_registry_integration
cargo test --test cli_integration
cargo test cli::args
cargo fmt
cargo test --test architecture_test
cargo test --test provider_registry_integration
cargo test --test cli_integration
python3 scripts/check_markdown_frontmatter.py
mdbook build
cargo fmt --check
cargo run -- --help
just check
just check
rg -n 'BraveConfig::from_env|ExaConfig::from_env|BraveProvider::new|ExaProvider::new|BRAVE_API_KEY|EXA_API_KEY' src/bootstrap -g '*.rs'
rg -n 'CliProvider::Brave|CliProvider::Exa' src/cli -g '*.rs'
rg -n 'ProviderId::Brave|ProviderId::Exa' src/cli/runner.rs src/cli/args.rs
```

## Test Results

| Check | Command / Method | Result | Notes |
|---|---|---|---|
| Failing-first catalog regression | `cargo test --test architecture_test test_provider_catalog_is_provider_wiring_source_of_truth` before implementation | Failed as expected | Failed because `src/bootstrap/provider_catalog.rs` did not exist. |
| Catalog regression after implementation | `cargo test --test architecture_test test_provider_catalog_is_provider_wiring_source_of_truth` | Passed | Proves source-scan guardrail is active. |
| Architecture tests | `cargo test --test architecture_test` | Passed | 13 passed. |
| Provider registry integration | `cargo test --test provider_registry_integration` | Passed | 13 passed. |
| CLI integration | `cargo test --test cli_integration` | Passed | 8 passed. |
| CLI args unit filter | `cargo test cli::args` | Passed | 3 CLI provider parsing tests passed. |
| Markdown frontmatter | `python3 scripts/check_markdown_frontmatter.py` | Passed | No output. |
| mdBook | `mdbook build` | Passed | HTML written to `book`. |
| Formatting | `cargo fmt --check` | Passed | No output. |
| CLI help provider values | `cargo run -- --help` | Passed | Help lists `[possible values: brave, exa, all]` for `--provider`. |
| Canonical gate, sandboxed | `just check` | Failed for environment reason | `cargo test` failed in `transport::http::tests::test_post_json_decodes_success_response` because sandboxing denied binding `127.0.0.1:0`. |
| Canonical gate, outside sandbox | `just check` | Passed | `cargo fmt --check`, clippy, all tests, frontmatter, and mdBook passed. |
| Bootstrap wiring source scan | `rg ... src/bootstrap -g '*.rs'` | Passed with expected locations | Provider construction/env strings are in `provider_catalog.rs`; remaining env strings in `provider_registry.rs` are test-only fixtures. |
| CLI duplicate provider mode scan | `rg -n 'CliProvider::Brave|CliProvider::Exa' src/cli -g '*.rs'` | Passed | No matches. |
| CLI runner provider-id scan | `rg -n 'ProviderId::Brave|ProviderId::Exa' src/cli/runner.rs src/cli/args.rs` | Passed with expected test-only matches | Matches only in `src/cli/args.rs` unit tests, not runner implementation. |

## Manual Verification

1. Reviewed the final diff to confirm `ProviderRegistry::production_from_env` iterates `provider_catalog::provider_catalog()`.
2. Reviewed CLI parsing to confirm `CliProvider` has only `Single(ProviderId)` and `All`; real providers are not duplicated as CLI enum variants.
3. Reviewed `--help` output to confirm provider possible values still come from the catalog-backed parser.
4. Reviewed docs search results for stale provider-registration guidance and updated the mdBook dependency map that is embedded in docs.

## Logs / Output

```text
pre-implementation regression:
test_provider_catalog_is_provider_wiring_source_of_truth ... FAILED
provider identity and production wiring must live in src/bootstrap/provider_catalog.rs

canonical gate outside sandbox:
just check
...
test result: ok. 29 passed; 0 failed
...
test result: ok. 13 passed; 0 failed
...
test result: ok. 8 passed; 0 failed
...
python3 scripts/check_markdown_frontmatter.py
mdbook build
INFO HTML book written to `/Users/tuna/sophon/book`
```

## Screenshots / Visual Evidence

Not applicable. This change is CLI/library behavior and documentation text, not a visual UI.

## Known Gaps

- No live Brave or Exa API calls were run; live provider calls remain outside automated evidence.
- The catalog source-scan deliberately allows provider tokens and env-var strings in provider config modules, the catalog, and test fixtures.
- The sandboxed `just check` cannot prove the local TCP-listener HTTP unit test because sandbox permissions deny the bind operation; the same canonical gate passed outside the sandbox.

## Final Evidence Judgment

Proven. The failing-first ownership regression was captured, the catalog-backed implementation is covered by architecture and integration tests, documentation was updated, and the canonical `just check` gate passed outside the sandbox.
