---
title: "Session charter: provider catalog"
when_to_read:
  - "When implementing the provider catalog refactor for sophon-cli."
  - "Before changing provider selection, provider registry construction, or provider documentation."
summary: "Defines the feature scope, constraints, regression checks, and execution plan for making a provider catalog the authoritative source for provider identity and production wiring."
ontology_relations:
  - relation: "governs"
    target: "sophon-cli-provider-catalog"
    note: "Sets the execution scope for the provider catalog refactor."
  - relation: "depends_on"
    target: "docs/artifacts/explorations/EXP-20260530-provider-catalog-duplication.md"
    note: "Uses the focused exploration that mapped duplicated provider metadata."
  - relation: "implements"
    target: "docs/artifacts/decisions/ADR-0001-provider-catalog.md"
    note: "Executes the durable decision that provider identity and production wiring belong to a catalog."
---

# Session Charter

| Field | Value |
|---|---|
| Artifact Type | Session Charter |
| Status | Completed |
| Date | 2026-05-30 |
| Owner | Codex |
| Related Artifacts | `docs/artifacts/explorations/EXP-20260530-provider-catalog-duplication.md`, `docs/artifacts/memory/MEM-20260530-provider-catalog-research.md`, `docs/artifacts/decisions/ADR-0001-provider-catalog.md`, `docs/artifacts/evidence/EVID-20260530-provider-catalog.md`, `docs/artifacts/memory/MEM-20260530-provider-catalog-implementation.md` |
| Related Files | `src/bootstrap/provider_registry.rs`, `src/bootstrap/mod.rs`, `src/cli/args.rs`, `src/cli/runner.rs`, `tests/architecture_test.rs`, `tests/integration/provider_registry_test.rs`, `tests/integration/cli_test.rs`, `README.md`, `CHANGELOG.md`, `docs/intro.md`, `docs/quickstart.md`, `docs/project/ARCHITECTURE.md`, `docs/architecture.md`, `HARNESS.md` |

## Mission

Create a bootstrap-owned provider catalog so provider identity, CLI token lookup, display names, environment variable names, stable provider ordering, and production provider construction have one authoritative source.

## Work Type

Feature

## Context

- `docs/artifacts/explorations/EXP-20260530-provider-catalog-duplication.md` found live provider metadata duplication across CLI parsing, CLI runner selection, bootstrap registry IDs, provider env config, stable ordering, tests, and docs.
- Current source repeats `Brave` and `Exa` knowledge in `CliProvider`, `ProviderId`, `ProviderRegistry::production_from_env`, `ProviderRegistry::available_providers`, about text, tests, and docs.
- Grill decisions resolved that the catalog should be a static ordered slice, should contain only real providers, and should not include the `all` aggregate mode.
- Unknown provider tokens should fail gracefully at argument parse time with valid catalog tokens plus `all`.
- A durable decision record is required because provider identity ownership is changing.

## Scope

### In Scope

- Add a provider catalog module under `src/bootstrap/`.
- Move or re-home `ProviderId` so provider identity is defined through the catalog surface.
- Make the catalog authoritative for:
  - provider id
  - CLI token
  - display name
  - environment variable name
  - stable provider order
  - production builder
- Keep `all` as a CLI-only aggregate selection mode that builds every environment-enabled real provider in catalog order.
- Replace hardcoded CLI provider value parsing with catalog-backed provider token lookup.
- Preserve graceful parse-time errors for unknown provider tokens.
- Add a failing-first regression or architecture test that requires CLI and bootstrap provider wiring to use the catalog.
- Add source-scan guardrails that forbid duplicated provider identities in CLI and bootstrap wiring outside the catalog and provider config modules.
- Update provider registry and CLI integration tests to assert catalog-backed behavior.
- Clean up user and maintainer docs so provider registration points to the catalog as the single source of truth.
- Add this PR's assigned number to `CHANGELOG.md` before merge so changelog coverage remains satisfied after landing.
- Update architecture and harness docs if their current provider registry contract becomes stale.

### Out of Scope

- Adding a third provider.
- Changing provider API DTO mapping, request construction, or result rendering.
- Moving provider-specific request fields into `src/domain/`.
- Changing domain provider contracts or `ProviderCapabilities`.
- Changing fan-out concurrency, output format, stdout/stderr behavior, or exit-code semantics.
- Running live Brave or Exa API calls as required evidence.
- Refactoring unrelated test fixtures or docs.

## Constraints

- No source edits before the failing catalog regression test is added.
- `src/domain/` must remain provider-agnostic and must not import bootstrap, providers, transport, CLI, or app layers.
- `src/app/` must continue to orchestrate through domain contracts only.
- `src/bootstrap/` may import concrete providers, provider configs, app services, domain traits, and transport adapters, but must not import CLI.
- `src/cli/` may use the bootstrap catalog for provider token parsing and selection, but must not construct concrete provider adapters.
- Provider-specific config loading stays under `src/providers/<provider>/config.rs`.
- Provider-specific DTOs and mappers stay under `src/providers/<provider>/`.
- New tracked Markdown must pass `python3 scripts/check_markdown_frontmatter.py`.
- Preserve existing CLI behavior unless this charter explicitly says otherwise.

## Risk Areas

- Replacing `clap::ValueEnum` with catalog-backed parsing could weaken help text or error quality.
- Moving `ProviderId` could cause noisy test churn if compatibility exports are not planned carefully.
- Source-scan tests can become brittle if they forbid legitimate provider strings in provider adapters, docs, or domain test fixtures.
- The catalog could become too broad if provider capabilities, API mapping, or provider-specific request rules are pulled into it.
- Docs may continue to list providers manually even after source wiring becomes catalog-backed.

## Regression Checks

- Existing CLI defaults still select Brave when no `--provider` is passed.
- `--provider brave` and `--provider exa` still select explicit single-provider mode.
- `--provider all` still queries every environment-enabled provider in stable catalog order.
- Explicit unavailable-provider errors still mention the requested provider and configured providers.
- Empty or missing API keys still make a provider unavailable.
- Unknown provider tokens fail at argument parse time and mention valid provider tokens plus `all`.
- Provider registry order remains Brave, then Exa.
- Architecture boundary tests still enforce domain, transport, provider, app, bootstrap, and CLI layering.

## Rollback Plan

Revert the catalog module, restore the previous `CliProvider` and `ProviderId` definitions, restore direct `ProviderRegistry::production_from_env` construction, and remove the new catalog-specific regression tests and docs updates.

## Plan

1. Add the failing-first catalog regression tests.
2. Add `src/bootstrap/provider_catalog.rs` with an ordered static catalog of real providers.
3. Expose the catalog module from `src/bootstrap/mod.rs`.
4. Move provider identity and display/token metadata to the catalog surface.
5. Refactor `ProviderRegistry::production_from_env` and provider ordering to iterate catalog entries.
6. Refactor CLI provider parsing to resolve real provider tokens from the catalog and keep `all` as CLI-only aggregate mode.
7. Refactor CLI runner single-provider selection to use catalog-backed provider IDs instead of hardcoded provider matches.
8. Update integration tests for provider registry behavior, CLI parsing, unknown-provider errors, and all-provider ordering.
9. Update docs that describe supported providers, provider registration, env configuration, and runtime provider selection.
10. Run targeted tests, then `just check`, then create evidence and session memory.

## Evidence Required

- A pre-implementation failing result for the catalog regression test, or a documented reason it could not be captured.
- Passing targeted Rust tests for architecture, provider registry, and CLI parsing.
- `python3 scripts/check_markdown_frontmatter.py` passes after docs updates.
- `mdbook build` passes after docs updates.
- `just check` passes, or any failure is documented in the evidence pack with the exact failing command and output summary.
- Diff review confirms hardcoded provider identities were removed from CLI/bootstrap wiring outside allowed catalog and provider config locations.

## Exit Criteria

- Provider identity, CLI token lookup, display names, env var names, stable ordering, and production construction are sourced from the provider catalog.
- `all` remains a CLI-only aggregate mode and is not represented as a catalog provider.
- CLI and bootstrap no longer maintain separate provider lists for Brave and Exa.
- Regression tests protect the catalog as the provider wiring source of truth.
- User-facing and maintainer docs point future provider additions to the catalog.
- ADR-0001 exists and records the durable decision.
- Evidence and session memory artifacts exist for the implementation session.

## Clarifications Needed Before Editing

None.
