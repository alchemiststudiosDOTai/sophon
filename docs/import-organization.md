---
title: "Import Organization Visual"
when_to_read:
  - "When checking whether a new import crosses a forbidden sophon-cli layer boundary."
summary: "Lists allowed and forbidden import directions for the main source layers."
ontology_relations:
  - relation: "documents"
    target: "sophon-cli-import-boundaries"
    note: "Provides an mdBook chapter for the import organization page referenced by docs/SUMMARY.md."
---

# Import Organization Visual

Use this quick map before adding a cross-module `use`.

| Layer | May Import | Must Not Import |
|---|---|---|
| `src/domain/` | standard library and domain-local modules | `crate::providers`, `crate::transport`, `crate::cli`, `crate::app` |
| `src/transport/` | domain-neutral transport dependencies | `crate::providers`, `crate::cli`, `crate::app` |
| `src/providers/` | `crate::domain`, `crate::transport`, provider-local modules | `crate::cli`, `crate::app` |
| `src/app/` | `crate::domain`, app-local modules | `crate::cli`, `crate::bootstrap`, `crate::providers`, `crate::transport` |
| `src/bootstrap/` | app, provider, transport, and domain construction surfaces | `crate::cli` |
| `src/cli/` | CLI, bootstrap, app-facing runner calls, and domain request types | provider DTO internals |

`tests/architecture_test.rs` is the executable source of truth for the enforced import bans.
