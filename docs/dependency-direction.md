---
title: "Dependency Direction Visual"
when_to_read:
  - "When reviewing the intended dependency direction between sophon-cli layers."
summary: "Shows the high-level dependency direction that source-scan architecture tests enforce."
ontology_relations:
  - relation: "visualizes"
    target: "sophon-cli-dependency-direction"
    note: "Provides an mdBook chapter for the dependency direction referenced by docs/SUMMARY.md."
---

# Dependency Direction Visual

This is the high-level direction future changes should preserve.

```mermaid
flowchart LR
    main["src/main.rs"] --> cli["src/cli/"]
    cli --> bootstrap["src/bootstrap/"]
    cli --> domain["src/domain/"]
    bootstrap --> app["src/app/"]
    bootstrap --> providers["src/providers/"]
    app --> domain
    providers --> domain
    providers --> transport["src/transport/"]
    transport --> reqwest["reqwest"]
```

Architecture boundary tests in `tests/architecture_test.rs` enforce the critical reverse-import bans.
