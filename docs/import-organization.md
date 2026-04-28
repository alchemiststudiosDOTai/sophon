---
title: "Import Organization"
when_to_read:
  - "When changing Rust module boundaries or adding imports across runtime layers."
  - "When checking whether a new module belongs in CLI, bootstrap, app, providers, transport, or domain."
summary: "Maintainer guidance for the allowed import direction in sophon-cli after the CLI runner and request-normalization refactor."
ontology_relations:
  - relation: "explains"
    target: "docs/architecture.md"
    note: "Documents the import rules behind the architecture page."
  - relation: "explains"
    target: "docs/dependency-architecture-map.md"
    note: "Describes the allowed edges shown by the current dependency map."
---

# Import Organization

Keep imports flowing from user-facing edges toward stable domain contracts. Do not move concrete provider construction into the application layer, and do not let provider-specific details leak into domain types.

## Allowed imports by layer

| Layer | Allowed direction |
|-------|-------------------|
| `src/main.rs` | `main` to CLI runner only for runtime delegation, plus process setup dependencies such as tracing and dotenv |
| `src/cli/` | CLI to bootstrap, domain, args, request, and output helpers; the CLI runner owns provider-mode branching and rendering |
| `src/bootstrap/` | bootstrap to app, providers, transport, and domain; this is the composition root for concrete services |
| `src/app/` | app to domain only; services orchestrate `SearchProvider` trait objects |
| `src/providers/` | providers to transport and domain only, plus provider-local DTO, mapper, and config modules |
| `src/transport/` | transport to domain only for shared `SearchError` contracts |
| `src/domain/` | domain to no outer layers |

## Review checklist

- `src/main.rs` should continue delegating to `sophon_cli::cli::runner::run_from_env().await`.
- `src/cli/request.rs` should remain the place where `CliArgs` plus query text become `SearchQuery`.
- `src/cli/runner.rs` may request services from `ProviderRegistry`, but concrete provider construction must stay in `src/bootstrap/provider_registry.rs`.
- `src/app/` modules should accept domain types and domain traits only.
- `src/providers/` modules should map provider DTOs into domain responses before crossing back upward.
- `src/transport/` should return domain errors rather than provider-specific error types.
