---
title: "ADR-0001: Provider catalog"
when_to_read:
  - "When adding, removing, renaming, or wiring a search provider."
  - "When changing CLI provider parsing, provider registry construction, or provider ordering."
summary: "Records the decision that a bootstrap-owned provider catalog is the source of truth for provider identity, CLI tokens, display metadata, environment variables, stable ordering, and production construction."
ontology_relations:
  - relation: "decides"
    target: "sophon-cli-provider-catalog"
    note: "Establishes catalog ownership for provider identity and production wiring."
  - relation: "informed_by"
    target: "docs/artifacts/explorations/EXP-20260530-provider-catalog-duplication.md"
    note: "Uses the exploration that found live provider metadata duplication."
  - relation: "implemented_by"
    target: "docs/artifacts/active/CHARTER-20260530-provider-catalog.md"
    note: "The active charter defines the implementation work for this decision."
---

# ADR-0001: Provider Catalog

| Field | Value |
|---|---|
| Artifact Type | Decision Record |
| Status | Active |
| Date | 2026-05-30 |
| Owner | Codex |
| Related Artifacts | `docs/artifacts/explorations/EXP-20260530-provider-catalog-duplication.md`, `docs/artifacts/memory/MEM-20260530-provider-catalog-research.md`, `docs/artifacts/active/CHARTER-20260530-provider-catalog.md` |
| Related Files | `src/bootstrap/provider_registry.rs`, `src/cli/args.rs`, `src/cli/runner.rs`, `tests/architecture_test.rs`, `tests/integration/provider_registry_test.rs`, `tests/integration/cli_test.rs`, `README.md`, `docs/intro.md`, `docs/quickstart.md`, `docs/project/ARCHITECTURE.md`, `docs/architecture.md`, `HARNESS.md` |

## Context

The current codebase repeats supported-provider knowledge across CLI parsing, CLI runner selection, bootstrap provider IDs, production provider construction, stable provider ordering, user-facing docs, maintainer docs, and tests.

The focused exploration found that this repetition is live behavior, not dead code. Adding another provider would currently require editing several separate provider lists and matches. That raises the chance of adding a provider to one path while forgetting CLI parsing, fan-out ordering, unavailable-provider errors, or docs.

## Decision

Create a bootstrap-owned static ordered provider catalog. The catalog contains only real providers and is authoritative for:

- provider id
- CLI token
- display name
- environment variable name
- stable provider order
- production builder

The CLI `all` option is not a catalog entry. It remains a CLI-only aggregate selection mode that asks the registry to build every environment-enabled provider in catalog order.

CLI provider parsing must resolve real provider tokens through the catalog. Unknown provider tokens must fail gracefully at argument parse time and show valid catalog provider tokens plus `all`.

Regression tests must protect this ownership model by requiring CLI and bootstrap provider wiring to use the catalog and by forbidding duplicated hardcoded provider identities in CLI/bootstrap source outside approved locations.

## Rationale

A static ordered catalog is simpler and more maintainable than scattered enum matches and arrays. Provider count is small, so a linear catalog scan is efficient enough and keeps ordering explicit.

Keeping the catalog in bootstrap preserves the existing architecture: bootstrap owns concrete construction, provider adapters remain provider-specific, and the domain remains provider-agnostic.

Keeping `all` out of the catalog avoids mixing real providers with CLI selection modes. This keeps env checks, construction, and ordering focused on actual provider entries.

## Options Considered

### Option A: Keep Current Explicit Matches

Pros:

- Minimal immediate code churn.
- Current behavior is easy to inspect for two providers.

Cons:

- Provider identity and ordering stay duplicated.
- Adding a provider remains a multi-surface edit with weak locality.
- Tests can prove behavior but not ownership of provider metadata.

### Option B: Static Bootstrap Provider Catalog

Pros:

- One authoritative source for provider identity, order, env metadata, and construction.
- Efficient enough for the small provider set.
- Preserves current layer boundaries.
- Makes provider addition work easier to explain and test.

Cons:

- Requires replacing `clap::ValueEnum` provider parsing with catalog-backed parsing.
- Requires careful source-scan exemptions so provider adapter internals and docs are not over-constrained.

### Option C: Dynamic Runtime Provider Registry Only

Pros:

- Could support plugin-like provider discovery later.

Cons:

- More machinery than the current product needs.
- Makes stable order and graceful help/error text harder to reason about.
- Risks obscuring the simple built-in provider model.

## Consequences

### Positive

- Provider additions get one main registration path.
- Stable provider order becomes catalog declaration order.
- CLI parsing, registry construction, and tests can share provider metadata.
- Architecture tests can protect against provider-list drift.

### Negative

- The CLI layer will intentionally depend on the bootstrap catalog for provider token parsing.
- The provider parsing implementation must preserve or replace the helpful errors previously supplied by `clap::ValueEnum`.

### Neutral / Operational

- User-facing docs may still mention supported providers explicitly, but maintainer docs must point provider registration work to the catalog.
- Provider adapters can still return provider strings in mapped domain responses; this ADR governs provider wiring metadata, not every provider name literal.

## Constraints Created

- Real provider identity, CLI tokens, display names, env var names, stable order, and production builders must be added to the provider catalog.
- `all` must remain a CLI aggregate mode and must not be represented as a real provider catalog entry.
- Provider-specific DTOs, request rules, and config loading stay in provider modules.
- Domain types must remain provider-agnostic.
- CLI/bootstrap code must not maintain a second hardcoded list of real providers.

## Evidence

- `docs/artifacts/explorations/EXP-20260530-provider-catalog-duplication.md` maps the duplicated provider metadata.
- `docs/artifacts/memory/MEM-20260530-provider-catalog-research.md` summarizes the exploration session.
- `docs/artifacts/active/CHARTER-20260530-provider-catalog.md` defines the execution plan.
- Future implementation evidence must be captured in `docs/artifacts/evidence/`.

## Revisit Trigger

Revisit this decision if providers become dynamically loaded, if provider configuration no longer comes from environment-backed built-ins, or if CLI provider selection needs to support provider groups beyond `all`.
