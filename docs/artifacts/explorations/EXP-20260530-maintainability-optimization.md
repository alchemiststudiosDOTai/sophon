---
title: "Exploration: Architecture and optimization focus"
when_to_read:
  - "When evaluating architecture or performance optimization work for sophon-cli."
  - "Before creating a charter for refactor, cleanup, provider fan-out, or performance measurement work."
summary: "Captures provisional architecture and optimization candidates for sophon-cli without committing to a specific implementation path."
ontology_relations:
  - relation: "explores"
    target: "sophon-cli-architecture-and-optimization"
    note: "Frames likely architecture and optimization work before execution planning begins."
---
# Exploration Memory

| Field | Value |
|---|---|
| Artifact Type | Exploration Memory |
| Status | Active |
| Date | 2026-05-30 |
| Owner | Codex |
| Related Artifacts | `docs/project/ARCHITECTURE.md`, `docs/project/CONSTRAINTS.md`, `docs/project/TESTING.md`, `HARNESS.md` |
| Related Files | `src/app/fanout_search_service.rs`, `src/app/search_service.rs`, `src/bootstrap/provider_registry.rs`, `src/domain/provider.rs`, `src/providers/brave/client.rs`, `src/providers/exa/client.rs`, `src/cli/args.rs`, `src/cli/request.rs`, `tests/` |

## Problem Being Explored

The current focus is architecture and optimization for `sophon-cli`.

The goal is to identify improvements that make the codebase easier to change, keep the existing seams clear, and improve runtime or development-loop efficiency where evidence shows a real bottleneck.

Out of scope for this exploration:

- Adding new providers or new user-facing search features.
- Changing public CLI behavior before a charter chooses that work.
- Weakening the existing domain, transport, provider, app, bootstrap, or CLI import contracts.
- Optimizing live provider latency without separating network time from local orchestration cost.

## Current Understanding

This understanding is provisional.

- The codebase already has clear top-level seams: domain, transport, provider adapters, application orchestration, bootstrap wiring, and CLI presentation are documented and tested.
- The highest-value architecture work is likely to deepen existing modules rather than introduce new top-level structure.
- `FanoutSearchService::search_all` currently queries providers sequentially. For `--provider all`, local orchestration can make total latency the sum of enabled provider latencies.
- `ProviderCapabilities` exists in the domain interface, but current production behavior does not use it to validate, route, or explain unsupported queries. The `#[allow(dead_code)]` markers in `src/domain/provider.rs` are a signal that this interface is currently shallow.
- Provider availability and stable provider ordering are encoded in `ProviderRegistry` through repeated knowledge of `ProviderId::Brave` and `ProviderId::Exa`. Adding another provider would touch registry construction, stable ordering, CLI parsing, docs, and tests.
- Brave and Exa request construction live inside provider adapters, which is correct for provider-specific behavior, but the current shape mixes validation, request encoding, HTTP dispatch, and response mapping inside each provider client.
- Tests are broad enough to protect behavior, but setup is repetitive. `SearchQuery`, `SearchResponse`, and mock provider fixtures appear in many modules and integration tests.
- The harness is intentionally strict. Any optimization work should preserve `just check` as the canonical proof command unless a later charter explicitly targets the harness.

## Options Considered

### Candidate 1: Concurrent provider fan-out

| Field | Value |
|---|---|
| Files | `src/app/fanout_search_service.rs`, `tests/integration/search_service_test.rs`, `tests/fanout_cli_test.rs` |
| Recommendation Strength | Strong |

Problem:

`FanoutSearchService::search_all` loops over providers and awaits each search before starting the next one. With Brave and Exa both enabled, the `all` provider path is likely dominated by cumulative network latency.

Possible direction:

Keep the `SearchProvider` seam and preserve stable output ordering, but run enabled provider searches concurrently. The module interface can still return one `SearchBatchResponse`; concurrency should stay inside the implementation.

Benefits:

- Better runtime behavior for `--provider all` without changing CLI output.
- Good locality: ordering, failure capture, and partial-success semantics remain in one module.
- Good leverage: one app-layer implementation improves every current and future fan-out caller.

Assumptions to validate:

- The provider trait object shape can support concurrent borrowed searches cleanly, or the provider storage needs an `Arc<dyn SearchProvider>` shape.
- Stable ordering must be preserved in `responses` and `failures`; current tests already assert order.
- Baseline timing should be captured with controlled mock providers before any live-provider claim.

### Candidate 2: Deepen or delete `ProviderCapabilities`

| Field | Value |
|---|---|
| Files | `src/domain/provider.rs`, `src/providers/brave/client.rs`, `src/providers/exa/client.rs`, `src/app/fanout_search_service.rs`, `src/cli/runner.rs` |
| Recommendation Strength | Strong |

Problem:

`ProviderCapabilities` is part of the provider interface, but it does not currently drive behavior. The deletion test is useful here: deleting the method would not change production behavior today, which means the interface is exposing knowledge without providing leverage.

Possible directions:

- Deepen it: make capabilities part of query eligibility, fan-out routing, and error messaging.
- Delete it: remove the method and struct if unsupported-query behavior should remain owned by each provider adapter.

Benefits:

- If deepened, unsupported query handling gains locality instead of being repeated implicitly inside provider clients.
- If deleted, the provider interface becomes smaller and more honest.
- Either path removes current shallow interface surface.

Assumptions to validate:

- Fan-out semantics must be chosen before deepening this module: should an unsupported provider be treated as a failure, skipped, or reported separately?
- The domain must remain provider-agnostic; capabilities can describe generic search features, not provider-specific fields.

### Candidate 3: Provider catalog for registry metadata

| Field | Value |
|---|---|
| Files | `src/bootstrap/provider_registry.rs`, `src/cli/args.rs`, `src/providers/*/config.rs`, `tests/integration/provider_registry_test.rs` |
| Recommendation Strength | Worth exploring |

Problem:

Provider identity, stable ordering, environment configuration, and provider builders are scattered across enum matches and hard-coded arrays. The current size is fine for two providers, but the interface gets shallow when every provider addition requires carefully editing several places.

Possible direction:

Create a bootstrap-owned provider catalog that is the single local source for stable provider order, env-backed construction, and provider display names. Keep concrete provider adapters under `src/providers/` and keep CLI parsing in `src/cli/`.

Benefits:

- Better locality for provider addition work.
- More leverage from registry tests: one catalog can be verified for stable order and construction.
- Less chance of adding a provider to one path but forgetting fan-out or unavailable-provider messages.

Assumptions to validate:

- This should not move provider-specific fields into the domain.
- This should not make CLI parsing depend on bootstrap internals; mapping from CLI provider choice to provider id may still live at the CLI/runner seam.

### Candidate 4: Provider-private request planning modules

| Field | Value |
|---|---|
| Files | `src/providers/brave/client.rs`, `src/providers/exa/client.rs`, `src/providers/brave/mapper.rs`, `src/providers/exa/mapper.rs` |
| Recommendation Strength | Worth exploring |

Problem:

Provider clients currently combine query validation, request construction, HTTP dispatch, and response mapping. Exa already has `build_request`; Brave builds endpoint, params, headers, dispatch type, and mapper choice inline.

Possible direction:

Introduce provider-private request planning functions or modules that return typed request plans. Keep them internal to each provider adapter. The external provider interface remains `SearchProvider::search`.

Benefits:

- Better locality for provider-specific request rules.
- More focused tests around query-to-request behavior without mocking full HTTP.
- Less cognitive load inside provider clients while preserving the provider adapter seam.

Assumptions to validate:

- The request plan abstraction must stay provider-private unless two provider adapters reveal a real shared interface.
- Shared helpers should only be introduced when they remove proven duplication; otherwise they risk making provider behavior harder to inspect.

### Candidate 5: Test fixture consolidation for domain search values

| Field | Value |
|---|---|
| Files | `tests/common/cli.rs`, `tests/integration/*.rs`, source module tests under `src/` |
| Recommendation Strength | Worth exploring |

Problem:

Tests repeat full `SearchQuery` and `SearchResponse` literals across app, bootstrap, provider, CLI, and integration tests. That repetition is easy to understand today, but it increases update cost when a domain result field changes.

Possible direction:

Add small test-only builders or fixtures for common `SearchQuery`, `SearchResponse`, and mock provider values. Keep unusual test cases explicit where the exact field list matters.

Benefits:

- Faster refactors across domain value shapes.
- Better test locality: a field added to `SearchQuery` can be defaulted once in tests that do not care about it.
- Less mechanical churn when architecture work changes domain structs.

Assumptions to validate:

- Fixtures should not hide fields that are meaningful to a test.
- Shared test helpers should stay small enough that tests remain readable.

### Candidate 6: Measure check-loop and docs-build cost before harness changes

| Field | Value |
|---|---|
| Files | `justfile`, `HARNESS.md`, `docs/project/TESTING.md`, `.github/workflows/validate-agents.yml` |
| Recommendation Strength | Speculative |

Problem:

The harness is strict by design. Optimizing it without timing data could weaken proof or only move cost between local and CI paths.

Possible direction:

Before changing the harness, capture timings for each `just check` step and `just hygiene` step. Only create an execution charter if a specific step is slow enough to justify a change.

Benefits:

- Avoids speculative harness work.
- Keeps evidence specific and preserves trust in `just check`.
- Separates product runtime optimization from developer-loop optimization.

Assumptions to validate:

- Local timings are representative enough to act on, or CI timing should be used instead.
- The docs metadata and mdBook checks are worth preserving unless replaced with equivalent coverage.

## Promising Directions

- Start with Candidate 1 if the goal is user-visible runtime optimization for `--provider all`.
- Start with Candidate 2 if the goal is architecture clarity: either make provider capabilities earn their place in the interface or remove the shallow surface.
- Use Candidate 4 as a low-risk follow-on after Candidate 2, because request eligibility and request construction are related.
- Use Candidate 5 as enabling work if a chosen refactor touches many tests.
- Treat Candidate 6 as measurement-only until there is evidence of a slow harness step.

## Weak / Rejected Directions

- Broad rewrites of the provider, CLI, or app layers without a specific failing pressure.
- Moving provider-specific fields into the domain model to reduce adapter code.
- Optimizing external provider latency without separating network time from local overhead.
- Changing the validation chain solely to make local checks faster without preserving equivalent coverage.
- Introducing a shared provider request abstraction before two provider adapters need the same interface.
- Making CLI parsing depend directly on provider adapter modules.

## Open Questions

- Should `--provider all` preserve provider order in output even if searches complete out of order? Current tests imply yes.
- For unsupported query fields in fan-out, should the UX show failures, skip ineligible providers, or fail before dispatch?
- Is `ProviderCapabilities` intended as user-facing discoverability, app-layer routing input, or just provider documentation?
- Should `SearchService` remain a separate application module for tracing and single-provider orchestration, or is it a shallow wrapper around the provider interface?
- Which optimization target matters most now: runtime fan-out latency, development-loop speed, or change cost?

## Assumptions To Validate

- The existing architecture tests catch the most important dependency-boundary regressions.
- Maintainability issues are more likely to be local complexity problems than missing top-level architecture.
- There is no confirmed performance bottleneck yet, so optimization needs measurement before code changes.
- Any shared abstractions must reduce real duplication without making provider behavior harder to inspect.
- App-layer concurrency can be introduced without changing provider adapter behavior or CLI output.
- The current `SearchProvider: Send + Sync` trait bound is enough for the likely fan-out implementation, but the storage shape may need review.
- Query capability behavior should be decided before using capabilities to filter fan-out providers.

## Suggested Next Step

Create a refactor or performance-investigation charter for exactly one primary slice:

1. Concurrent fan-out with stable ordering and mock-provider timing evidence.
2. Capability interface cleanup: deepen `ProviderCapabilities` into behavior or remove it.
3. Provider-private request planning for Brave and Exa.

The strongest first move is Candidate 1 if optimization means runtime behavior, or Candidate 2 if optimization means architecture clarity.

## Notes

This artifact is not a final plan or decision. It records provisional findings and should be converted into a charter before any implementation work.

Observed evidence during this exploration:

- `cargo test -- --list` reported 70 tests and 0 benchmarks.
- Source file sizes show the largest production modules are `src/providers/exa/client.rs`, `src/bootstrap/provider_registry.rs`, `src/providers/brave/mapper.rs`, `src/providers/exa/mapper.rs`, `src/providers/brave/client.rs`, and `src/transport/http.rs`.
- `src/app/fanout_search_service.rs` lines 14-27 show sequential provider execution.
- `src/domain/provider.rs` lines 5-23 show `ProviderCapabilities` and `SearchProvider`; capability methods are currently marked with `#[allow(dead_code)]`.
- `src/bootstrap/provider_registry.rs` lines 51-120 show env-backed provider construction, stable provider order, and fan-out service construction in one module.
