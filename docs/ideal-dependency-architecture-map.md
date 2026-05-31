---
title: "Ideal Dependency Architecture Map"
when_to_read:
  - "When comparing a proposed dependency change against the intended sophon-cli architecture."
summary: "Documents the ideal layered map for CLI, bootstrap, application, domain, providers, and transport."
ontology_relations:
  - relation: "describes"
    target: "sophon-cli-ideal-architecture"
    note: "Provides an mdBook chapter for the ideal dependency map referenced by docs/SUMMARY.md."
---

# Ideal Dependency Architecture Map

The ideal structure keeps policy and I/O decisions at the edges while domain contracts remain stable.

```text
main
  -> cli
      -> bootstrap
      -> domain

bootstrap
  -> app
  -> providers
  -> transport

app
  -> domain

providers
  -> domain
  -> transport

transport
  -> external HTTP client
```

Use this map when deciding where a new function belongs:

- CLI behavior belongs in `src/cli/`.
- Provider construction belongs in `src/bootstrap/`.
- Orchestration belongs in `src/app/`.
- Provider DTOs, request payloads, and response mapping belong in `src/providers/`.
- Shared query/result contracts belong in `src/domain/`.
