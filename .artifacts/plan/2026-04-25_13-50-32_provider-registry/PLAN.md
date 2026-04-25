---
title: "Provider registry implementation plan"
link: "provider-registry-plan"
type: implementation_plan
ontological_relations:
  - relates_to: [[search-service-flow-visual]]
  - relates_to: [[HARNESS]]
tags: [plan, provider-registry, rust, search-cli, coding]
uuid: "D81E9746-372F-41BA-9CAB-16A8CD55467D"
created_at: "2026-04-25T13:50:32Z"
parent_research: ".artifacts/interface-designs/search-service-flow-visual.html"
git_commit_at_plan: "39c796e"
---

## Goal

Implement a compile-time built-in provider registry/factory that replaces the inline provider construction `match` in `src/main.rs`, scales cleanly as more providers are added, and only registers providers whose required env config is present.

Out of scope: runtime plugin providers, external provider discovery, config files, redesigning the transport trait, changing provider request/mapper behavior, and adding new search providers.

## Scope & Assumptions

IN scope:
- Add a composition/bootstrap module that owns provider registration and `SearchService` construction.
- Add a provider-neutral `ProviderId` separate from `cli::args::CliProvider`.
- Add a registry that maps configured built-in providers to provider builder closures.
- Make production registry creation include Brave only when `BraveConfig::from_env()` succeeds and Exa only when `ExaConfig::from_env()` succeeds.
- Return a clear construction error when a selected provider is not registered because it is not configured.
- Keep `main` responsible for CLI parsing, query construction, output rendering, and process exit.
- Add focused tests for registry availability and `main` wiring boundaries where practical.

OUT of scope:
- `Box<dyn HttpClient>` or transport object erasure. Current `HttpClient` has generic methods and is not object-safe.
- Shared erased config bags such as `ProviderConfig { api_key, base_url, extra }`.
- Lazy provider setup that defers missing-key failures until `search`.
- Runtime provider plugins or dynamic loading.
- Reworking `SearchProvider`, `SearchService`, or provider DTO mapping.

Assumptions:
- `BraveConfig` and `ExaConfig` remain typed provider-owned config structs.
- `BraveConfig::from_env()` requires `BRAVE_API_KEY`; `ExaConfig::from_env()` requires `EXA_API_KEY`.
- `dotenvy::dotenv().ok()` remains in `main` before production registry creation.
- The architecture boundary tests may be extended, but existing `domain`, `transport`, `providers`, and `app` restrictions must continue to pass.
- The current untracked `.artifacts/interface-designs/` files are planning artifacts and should not affect source implementation.

## Deliverables

- `src/bootstrap/mod.rs`
- `src/bootstrap/provider_registry.rs`
- `src/main.rs` updates to call the registry/factory instead of inline provider construction
- Optional focused unit tests inside `src/bootstrap/provider_registry.rs`
- `tests/architecture_test.rs` updates if needed to document the new composition-layer boundary

## Readiness

Preconditions:
- Current source tree includes `src/providers/brave/*`, `src/providers/exa/*`, `src/app/search_service.rs`, `src/domain/provider.rs`, and `src/transport/http.rs`.
- The working tree may contain untracked planning artifacts under `.artifacts/interface-designs/`; execution should leave them intact.
- `just check` remains the canonical final verification command.

What must exist before starting:
- Rust toolchain and dependencies already present.
- No source files need to be generated outside the paths listed in this plan.

## Milestones

- M1: Bootstrap registry API and error contract
- M2: Production provider registration from typed env config
- M3: Main wiring migration and architecture guardrails
- M4: Focused tests and final check path

## Ticket Index

<!-- TICKET_INDEX:START -->

| Task | Title | Ticket |
|---|---|---|
| T001 | Add bootstrap module skeleton and ProviderId | [tickets/T001.md](tickets/T001.md) |
| T002 | Define registry and construction error contract | [tickets/T002.md](tickets/T002.md) |
| T003 | Add production registry that only includes configured providers | [tickets/T003.md](tickets/T003.md) |
| T004 | Convert CliProvider to ProviderId at the binary edge | [tickets/T004.md](tickets/T004.md) |
| T005 | Replace inline provider construction in main | [tickets/T005.md](tickets/T005.md) |
| T006 | Extend architecture guardrails for the composition layer | [tickets/T006.md](tickets/T006.md) |
| T007 | Add focused registry tests | [tickets/T007.md](tickets/T007.md) |
| T008 | Run final local gate and update docs only if source references require it | [tickets/T008.md](tickets/T008.md) |

<!-- TICKET_INDEX:END -->

## Work Breakdown (Tasks)

### T001: Add bootstrap module skeleton and ProviderId

**Summary**: Create the composition module and provider-neutral selection type without wiring any providers yet.

**Owner**: backend

**Estimate**: 45m

**Dependencies**: <none>

**Target milestone**: M1

**Acceptance test**: `cargo check` passes with `mod bootstrap;` declared and no provider behavior changed.

**Files/modules touched**:
- `src/main.rs`
- `src/bootstrap/mod.rs`
- `src/bootstrap/provider_registry.rs`

**Steps**:
1. Add `mod bootstrap;` near the other module declarations in `src/main.rs`.
2. Create `src/bootstrap/mod.rs` with `pub mod provider_registry;`.
3. Create `src/bootstrap/provider_registry.rs`.
4. Define `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub enum ProviderId { Brave, Exa }`.
5. Implement `Display` for `ProviderId` with lowercase labels `brave` and `exa`.
6. Do not import `crate::cli` in `src/bootstrap/provider_registry.rs`.

### T002: Define registry and construction error contract

**Summary**: Add the provider registry type, provider builder alias, and construction errors used by production service creation.

**Owner**: backend

**Estimate**: 1h

**Dependencies**: T001

**Target milestone**: M1

**Acceptance test**: Unit test constructs an empty registry and receives `ProviderUnavailable` when building `ProviderId::Brave`.

**Files/modules touched**:
- `src/bootstrap/provider_registry.rs`

**Steps**:
1. Import `std::collections::HashMap`, `crate::app::search_service::SearchService`, and `crate::domain::provider::SearchProvider`.
2. Define `pub type ProviderBuilder = Box<dyn Fn() -> Box<dyn SearchProvider> + Send + Sync>;`.
3. Define `pub struct ProviderRegistry { builders: HashMap<ProviderId, ProviderBuilder> }`.
4. Add `pub fn empty() -> Self`.
5. Add `pub fn register(&mut self, id: ProviderId, builder: ProviderBuilder)`.
6. Add `pub fn available_providers(&self) -> Vec<ProviderId>` that returns stable sorted order `[Brave, Exa]` when present.
7. Define `#[derive(Debug, thiserror::Error)] pub enum BuildSearchServiceError` with `ProviderUnavailable { provider: ProviderId, available: Vec<ProviderId> }`.
8. Implement `pub fn build(&self, provider: ProviderId) -> Result<SearchService, BuildSearchServiceError>` that creates `SearchService::new(builder())` when registered.
9. Add a unit test for empty registry unavailable behavior.

### T003: Add production registry that only includes configured providers

**Summary**: Implement env-backed production registration where missing provider keys mean the provider is omitted from the registry.

**Owner**: backend

**Estimate**: 1h

**Dependencies**: T002

**Target milestone**: M2

**Acceptance test**: Unit test sets only `BRAVE_API_KEY` and confirms `production_from_env()` lists Brave but not Exa.

**Files/modules touched**:
- `src/bootstrap/provider_registry.rs`

**Steps**:
1. Import `BraveProvider`, `BraveConfig`, `ExaProvider`, `ExaConfig`, and `ReqwestHttpClient`.
2. Add `pub fn production_from_env() -> Self`.
3. In `production_from_env()`, call `BraveConfig::from_env()`. If it returns `Ok(config)`, register `ProviderId::Brave` with a closure that clones `config` and returns `Box::new(BraveProvider::new(ReqwestHttpClient::new(), config.clone()))`.
4. In `production_from_env()`, call `ExaConfig::from_env()`. If it returns `Ok(config)`, register `ProviderId::Exa` with a closure that clones `config` and returns `Box::new(ExaProvider::new(ReqwestHttpClient::new(), config.clone()))`.
5. If a config load returns `Err(std::env::VarError::NotPresent)`, do not register that provider.
6. If a config load returns `Err(std::env::VarError::NotUnicode(_))`, do not register that provider for now; surface the same `ProviderUnavailable` if selected.
7. Add tests that isolate env vars using a small test lock if needed because env is process-global.
8. Ensure tests restore any modified `BRAVE_API_KEY` and `EXA_API_KEY` values.

### T004: Convert CliProvider to ProviderId at the binary edge

**Summary**: Add explicit conversion from CLI provider enum to bootstrap provider ID without making the registry depend on CLI types.

**Owner**: backend

**Estimate**: 30m

**Dependencies**: T001

**Target milestone**: M3

**Acceptance test**: `cargo test cli::args::tests::test_cli_provider_parses_exa_and_defaults_to_brave` still passes.

**Files/modules touched**:
- `src/main.rs`

**Steps**:
1. Import `crate::bootstrap::provider_registry::ProviderId` in `src/main.rs`.
2. Add `impl From<CliProvider> for ProviderId` in `src/main.rs`.
3. Map `CliProvider::Brave` to `ProviderId::Brave`.
4. Map `CliProvider::Exa` to `ProviderId::Exa`.
5. Keep `src/bootstrap/provider_registry.rs` free of `crate::cli` imports.

### T005: Replace inline provider construction in main

**Summary**: Remove the duplicated provider construction `match` from `main` and delegate service construction to the production registry.

**Owner**: backend

**Estimate**: 45m

**Dependencies**: T002,T003,T004

**Target milestone**: M3

**Acceptance test**: Running `cargo run -- "rust"` with no `BRAVE_API_KEY` prints a provider-unavailable/configuration message before any search request is attempted.

**Files/modules touched**:
- `src/main.rs`

**Steps**:
1. Remove direct imports of `SearchService`, `BraveProvider`, `BraveConfig`, `ExaProvider`, `ExaConfig`, and `ReqwestHttpClient` from `src/main.rs` if no longer used.
2. Import `ProviderRegistry` from `src/bootstrap/provider_registry.rs`.
3. Replace the inline `let service = match args.provider { ... };` block with:
   - `let provider_id = ProviderId::from(args.provider);`
   - `let registry = ProviderRegistry::production_from_env();`
   - `let service = registry.build(provider_id).unwrap_or_else(|error| { eprintln!("{error}"); std::process::exit(1); });`
4. Keep the existing `service.search(query).await` block unchanged.
5. Ensure missing provider config fails during service construction, not during `service.search`.

### T006: Extend architecture guardrails for the composition layer

**Summary**: Document the new `bootstrap` layer in architecture tests so its intentionally broad dependencies do not weaken existing boundaries.

**Owner**: backend

**Estimate**: 45m

**Dependencies**: T001,T005

**Target milestone**: M3

**Acceptance test**: `cargo test --test architecture_test` passes and includes a guard that `src/bootstrap` does not import `crate::cli`.

**Files/modules touched**:
- `tests/architecture_test.rs`

**Steps**:
1. Add a new architecture test named `test_bootstrap_does_not_import_cli`.
2. Use the existing `check_dir_for_forbidden_patterns` helper against `src/bootstrap`.
3. Forbid `use crate::cli::` in `src/bootstrap`.
4. Leave existing tests for `domain`, `transport`, `providers`, `app`, and `render_text` unchanged.
5. Do not forbid `bootstrap` from importing `app`, `providers`, `transport`, or `domain`; it is the composition layer.

### T007: Add focused registry tests

**Summary**: Add unit coverage for registration, available provider ordering, and successful service construction with fake providers.

**Owner**: backend

**Estimate**: 1h

**Dependencies**: T002

**Target milestone**: M4

**Acceptance test**: `cargo test bootstrap::provider_registry` passes.

**Files/modules touched**:
- `src/bootstrap/provider_registry.rs`

**Steps**:
1. Add a `#[cfg(test)]` module in `src/bootstrap/provider_registry.rs`.
2. Define a local `MockProvider` implementing `SearchProvider`.
3. Add a test that registers `ProviderId::Brave`, verifies `available_providers()` returns `vec![ProviderId::Brave]`, and verifies `build(ProviderId::Brave)` succeeds.
4. Add a test that registers both providers and verifies stable ordering is `vec![ProviderId::Brave, ProviderId::Exa]`.
5. Keep env-dependent tests separated from pure registry tests.

### T008: Run final local gate and update docs only if source references require it

**Summary**: Run the repository check gate and only update developer docs if the new bootstrap layer makes existing maps inaccurate.

**Owner**: backend

**Estimate**: 45m

**Dependencies**: T005,T006,T007

**Target milestone**: M4

**Acceptance test**: `just check` passes.

**Files/modules touched**:
- `HARNESS.md`
- `docs/src/architecture.md`

**Steps**:
1. Run `cargo fmt`.
2. Run `cargo test`.
3. Run `cargo clippy -- -D warnings -W clippy::complexity -W clippy::cognitive_complexity`.
4. Run `mdbook build`.
5. Run `just check` as the final umbrella gate.
6. If docs mention `src/main.rs` as directly wiring concrete providers, update only the affected lines in `docs/src/architecture.md`.
7. If `HARNESS.md` source index or architecture boundary section is now inaccurate because `src/bootstrap` exists, update only those lines.

## Risks & Mitigations

- Risk: `HttpClient` is not object-safe because it has generic methods.
  Mitigation: Do not put `Box<dyn HttpClient>` in the registry. Keep concrete `ReqwestHttpClient` inside provider builders.
- Risk: Env-var tests are process-global and can be flaky under parallel test execution.
  Mitigation: Prefer pure registry tests. If testing `production_from_env()`, guard env mutation with a test lock and restore variables.
- Risk: Omitting unconfigured providers may make the default `brave` provider fail differently than before.
  Mitigation: Use a clear `ProviderUnavailable` message that names the selected provider and lists configured providers.
- Risk: A new composition layer can become a dumping ground.
  Mitigation: Restrict it to provider registration and `SearchService` construction; keep query building and rendering in `main`/`cli`.
- Risk: Architecture tests could accidentally forbid the new composition layer from doing its job.
  Mitigation: Add only a `bootstrap` no-CLI guard; do not forbid imports of `app`, `providers`, `transport`, or `domain`.

## Test Strategy

- T002 adds one unit test for unavailable provider behavior.
- T003 adds one env-backed availability test if it can be made deterministic.
- T004 relies on the existing CLI parsing test as the acceptance proof.
- T005 uses one manual CLI acceptance proof for missing-key behavior.
- T006 adds one architecture boundary test for `src/bootstrap`.
- T007 adds one focused registry unit test for successful construction and ordering.
- T008 runs the full check gate.

## References

- `src/main.rs:55` current inline provider construction branch
- `src/app/search_service.rs:6` `SearchService` owns `Box<dyn SearchProvider>`
- `src/domain/provider.rs:19` `SearchProvider` trait object boundary
- `src/transport/http.rs:6` generic `HttpClient` methods are not object-safe
- `src/providers/brave/config.rs:8` Brave env config loading
- `src/providers/exa/config.rs:8` Exa env config loading
- `tests/architecture_test.rs:24` providers cannot import CLI or app
- `.artifacts/interface-designs/search-service-flow-visual.html` visual comparison of interface options

## Final Gate

- **Output summary**: plan dir path, milestone count, ticket count
- **Next step**: proceed to execute-phase with `.artifacts/plan/2026-04-25_13-50-32_provider-registry/PLAN.md`
