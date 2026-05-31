---
title: "Architecture"
when_to_read:
  - "When changing module boundaries, provider adapters, CLI flow, or search orchestration."
summary: "Maps the sophon-cli layer boundaries, dependency direction, data flow, interfaces, invariants, and preferred change locations."
ontology_relations:
  - relation: "describes"
    target: "sophon-cli-architecture"
    note: "Documents the current architecture that tests and docs expect future changes to preserve."
---

# Architecture

| Field | Value |
|---|---|
| Artifact Type | Architecture Map |
| Status | Active |
| Date | 2026-05-30 |
| Owner | Project maintainers |
| Related Artifacts | `HARNESS.md` |
| Related Files | `src/`, `tests/architecture_test.rs`, `docs/architecture.md`, `docs/dependency-architecture-map.md`, `docs/dependency-direction.md`, `docs/ideal-dependency-architecture-map.md`, `docs/import-organization.md` |

## System Overview

`sophon-cli` is organized around a provider-agnostic domain core. CLI modules parse user input and render output. Bootstrap constructs provider-backed services. Application services orchestrate search requests through domain traits. Provider adapters translate external API DTOs into domain results and use the transport adapter for HTTP.

## Boundaries

| Boundary | Path | Responsibility |
|---|---|---|
| Entrypoint | `src/main.rs` | Start async runtime and delegate to `cli::runner::run_from_env`. |
| CLI | `src/cli/` | Parse args, build requests, run CLI workflow, render text output. |
| Bootstrap | `src/bootstrap/` | Own the provider catalog, register configured providers, and build services. |
| Application | `src/app/` | Orchestrate single-provider and fan-out search behavior. |
| Domain | `src/domain/` | Define provider-agnostic query, result, type, error, and provider contracts. |
| Providers | `src/providers/brave/`, `src/providers/exa/` | Map Brave/Exa APIs into domain contracts. |
| Transport | `src/transport/` | Provide HTTP client traits and reqwest adapter. |

## Dependency Direction

```text
main -> cli -> bootstrap -> app -> domain
                 providers -> domain
                 providers -> transport
```

The app layer depends on domain contracts, not concrete provider or transport implementations.

## Data Flow

1. CLI parses `CliArgs`.
2. CLI request builder creates a domain `SearchQuery`.
3. CLI runner asks bootstrap for a provider or fan-out service.
4. Application service invokes `SearchProvider` domain contracts.
5. Provider adapter builds HTTP requests and maps external DTOs into domain results.
6. CLI output renderer prints normalized text results to stdout.

## External Interfaces

- CLI arguments parsed with `clap`.
- Environment variables loaded with `dotenvy` or exported shell variables.
- Brave Search API via `BRAVE_API_KEY`.
- Exa API via `EXA_API_KEY`.
- HTTP transport via `reqwest`.
- Structured logs via `tracing`, written to stderr.

## Invariants

- Domain stays provider-agnostic.
- Provider adapters do not import CLI or app layers.
- Application orchestration does not depend on concrete providers or transport.
- Text rendering stays in CLI code.
- `src/main.rs` remains thin and delegates to the CLI runner.

## Common Agent Mistakes

- Putting provider-specific fields into domain types.
- Calling `render_text` outside `src/cli/`.
- Adding direct provider or transport imports inside `src/app/`.
- Updating docs without preserving Markdown frontmatter required by the checker.
- Editing `AGENTS.md` into a long manual instead of linking deeper docs.

## Where To Make Changes

| Change Type | Preferred Location | Notes |
|---|---|---|
| New provider API mapping | `src/providers/<provider>/` | Keep DTOs and mapper provider-specific. |
| Shared result/query shape | `src/domain/` | Preserve provider-agnostic naming. |
| CLI flag or command behavior | `src/cli/args.rs`, `src/cli/request.rs`, `src/cli/runner.rs` | Keep parsing, request construction, and execution separated. |
| Output formatting | `src/cli/output.rs` | Keep presentation out of domain/app layers. |
| Provider registration | `src/bootstrap/provider_catalog.rs` | Add real provider identity, CLI token, display name, env var, stable order, and production builder here. Update registry and CLI integration tests when adding providers. |
| Validation gate | `justfile`, `HARNESS.md`, `.github/workflows/validate-agents.yml` | Keep local and CI docs aligned. |

## Where Not To Make Changes

| Area | Reason |
|---|---|
| `src/domain/` for provider-specific API fields | Breaks provider-agnostic boundary. |
| `src/app/` for concrete provider construction | Breaks orchestration boundary. |
| `src/main.rs` for search logic | Entrypoint should stay thin. |
| `AGENTS.md` for detailed process manuals | Use `docs/process/` and `docs/artifacts/`. |
