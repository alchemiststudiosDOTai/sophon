---
title: "Exploration: Provider catalog duplication"
when_to_read:
  - "When evaluating provider registry, CLI provider selection, or adding a provider to sophon-cli."
  - "Before creating a charter for a provider catalog or provider registry refactor."
summary: "Maps where supported-provider knowledge is currently repeated across CLI, bootstrap, provider config, docs, and tests."
ontology_relations:
  - relation: "explores"
    target: "sophon-cli-provider-catalog-duplication"
    note: "Captures factual research about duplicated provider metadata before any refactor charter."
  - relation: "relates_to"
    target: "docs/artifacts/explorations/EXP-20260530-maintainability-optimization.md"
    note: "Deepens Candidate 3 from the broader maintainability exploration."
---

# Exploration Memory

| Field | Value |
|---|---|
| Artifact Type | Exploration Memory |
| Status | Active |
| Date | 2026-05-30 |
| Owner | Codex |
| Related Artifacts | `docs/artifacts/explorations/EXP-20260530-maintainability-optimization.md` |
| Related Files | `src/bootstrap/provider_registry.rs`, `src/cli/args.rs`, `src/cli/runner.rs`, `src/providers/brave/config.rs`, `src/providers/exa/config.rs`, `tests/integration/provider_registry_test.rs`, `tests/architecture_test.rs`, `docs/project/ARCHITECTURE.md`, `README.md` |

## Research Intent

Research type: `explore`

Commitment status: `decision-support`

Goal: map where the repository currently states which search providers exist, how those facts connect at runtime, and what files currently encode provider ordering, env configuration, and CLI selection.

Out of scope:

- Implementing a provider catalog.
- Adding a provider.
- Changing CLI behavior, output, errors, or provider order.
- Deciding the final refactor shape.

## Question

Where does the codebase currently say "the supported providers are Brave and Exa," and is that repetition dead code or live behavior?

## Short Answer

No dead provider-selection code was found in this path. The repeated provider facts are live runtime and test behavior.

The smell is that provider identity, CLI selection, display text, environment-backed construction, stable ordering, and tests each encode overlapping knowledge of Brave and Exa.

## Runtime Path

- `src/cli/args.rs:42` defines `CliProvider` with `Brave`, `Exa`, and `All`.
- `src/cli/runner.rs:29` creates `ProviderRegistry::production_from_env()` during normal CLI execution.
- `src/cli/runner.rs:31` branches on the parsed `CliProvider`.
- `src/cli/runner.rs:32` maps `CliProvider::Brave` to `ProviderId::Brave`.
- `src/cli/runner.rs:33` maps `CliProvider::Exa` to `ProviderId::Exa`.
- `src/cli/runner.rs:34` maps `CliProvider::All` to all enabled providers through the registry.
- `src/bootstrap/provider_registry.rs:51` builds the production registry from environment variables.
- `src/bootstrap/provider_registry.rs:54` reads Brave config and registers a Brave provider builder when configured.
- `src/bootstrap/provider_registry.rs:66` reads Exa config and registers an Exa provider builder when configured.
- `src/bootstrap/provider_registry.rs:103` builds a fan-out service from every available provider.

## Duplication Map

| Provider Fact | Current Locations | Observed Role |
|---|---|---|
| Provider IDs | `src/bootstrap/provider_registry.rs:13`, `src/cli/args.rs:42` | Bootstrap and CLI each define provider choices in local enums. |
| Provider display strings | `src/bootstrap/provider_registry.rs:18`, `src/cli/runner.rs:46`, `README.md:7`, `README.md:40`, `README.md:43` | Error/log formatting, about text, and docs each name Brave and Exa. |
| Env var names | `src/providers/brave/config.rs:9`, `src/providers/exa/config.rs:9`, `src/bootstrap/provider_registry.rs:40`, `README.md:40`, `README.md:43`, `tests/integration/provider_registry_test.rs:47` | Config loading, user docs, error text, and tests repeat `BRAVE_API_KEY` and `EXA_API_KEY`. |
| Provider construction | `src/bootstrap/provider_registry.rs:54`, `src/bootstrap/provider_registry.rs:66` | Production bootstrap knows which config type and client type construct each provider. |
| Stable provider order | `src/bootstrap/provider_registry.rs:85`, `tests/integration/provider_registry_test.rs:148`, `tests/integration/provider_registry_test.rs:245` | The order `Brave`, then `Exa`, is encoded by the registry and asserted by tests. |
| CLI-to-bootstrap mapping | `src/cli/runner.rs:31` | Parsed CLI provider values are translated to bootstrap provider IDs. |
| Provider registration documentation | `docs/project/ARCHITECTURE.md:91` | Project docs identify `src/bootstrap/provider_registry.rs` as the preferred provider registration location. |
| Architecture import contract | `tests/architecture_test.rs:93` | The architecture test expects `runner.rs` to use `CliProvider`, `ProviderRegistry`, and `ProviderId`. |

## Current Change Surface For A Third Provider

Based on the current shape, adding a third provider would touch at least these surfaces:

- Provider adapter files under `src/providers/<provider>/`.
- Config loading for the provider-specific API key.
- `ProviderId` in `src/bootstrap/provider_registry.rs:13`.
- `Display for ProviderId` in `src/bootstrap/provider_registry.rs:18`.
- `ProviderRegistry::production_from_env` in `src/bootstrap/provider_registry.rs:51`.
- Stable ordering in `ProviderRegistry::available_providers` at `src/bootstrap/provider_registry.rs:85`.
- `CliProvider` in `src/cli/args.rs:42`.
- Provider branching in `src/cli/runner.rs:31`.
- About text in `src/cli/runner.rs:46`.
- Registry tests in `tests/integration/provider_registry_test.rs:148` and `tests/integration/provider_registry_test.rs:245`.
- CLI parsing tests in `src/cli/args.rs:85` and `tests/integration/cli_test.rs:85`.
- User and project docs that list supported providers or env vars.

## Test Coverage Found

- Empty registry behavior is tested in `tests/integration/provider_registry_test.rs:105` and `tests/integration/provider_registry_test.rs:125`.
- Manual provider registration is tested in `tests/integration/provider_registry_test.rs:139`.
- Stable provider order is tested in `tests/integration/provider_registry_test.rs:147` and `tests/integration/provider_registry_test.rs:244`.
- Env-backed registry construction is tested for no keys, empty keys, Brave-only, Exa-only, and both providers in `tests/integration/provider_registry_test.rs:159`.
- Explicit unavailable-provider errors are tested in `tests/integration/provider_registry_test.rs:207`.
- CLI provider parsing for `exa`, `all`, and default `brave` is tested in `src/cli/args.rs:85`.
- The architecture test records a current dependency expectation from CLI runner to bootstrap provider IDs in `tests/architecture_test.rs:93`.

## Boundary Facts

- `docs/project/ARCHITECTURE.md:87` says new provider API mapping belongs under `src/providers/<provider>/`.
- `docs/project/ARCHITECTURE.md:89` says CLI flag or command behavior belongs under `src/cli/args.rs`, `src/cli/request.rs`, and `src/cli/runner.rs`.
- `docs/project/ARCHITECTURE.md:91` says provider registration belongs in `src/bootstrap/provider_registry.rs`.
- `src/bootstrap/provider_registry.rs:4` imports app services.
- `src/bootstrap/provider_registry.rs:5` imports the domain provider trait.
- `src/bootstrap/provider_registry.rs:6` imports concrete provider adapters and configs.
- `src/bootstrap/provider_registry.rs:10` imports the HTTP transport adapter.
- `src/cli/runner.rs:3` imports `ProviderId` and `ProviderRegistry` from bootstrap.

## Conclusion

The duplicated provider knowledge is live, tested behavior. The current shape is understandable for two providers, but the same facts are repeated across CLI parsing, bootstrap IDs, display text, env-backed construction, stable ordering, docs, and tests. This supports the earlier "Worth exploring" classification for a provider catalog, without deciding that a catalog should be implemented.
