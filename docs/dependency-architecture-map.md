---
title: "Current Dependency Architecture Map"
when_to_read:
  - "When you need to see the current Rust import/dependency direction as a clean architecture map."
  - "When comparing the current module import shape against the ideal architecture map."
summary: "Current architecture-style map of sophon-cli module import direction, shown in the same visual language as the ideal dependency architecture map."
ontology_relations:
  - relation: "part_of"
    target: "docs/SUMMARY.md"
    note: "Belongs to the mdBook documentation set."
  - relation: "explains"
    target: "docs/architecture.md"
    note: "Visualizes the concrete import direction behind the architecture layers."
  - relation: "compares_with"
    target: "docs/ideal-dependency-architecture-map.md"
    note: "Pairs current architecture with ideal target architecture."
---

# Current Dependency Architecture Map

This is the current/actual Rust import shape after the runtime organization refactor: `src/main.rs` delegates to the CLI runner, the CLI surface performs user-facing branching and rendering, bootstrap owns the provider catalog plus registry composition, app services orchestrate domain provider traits, and domain remains the bottom layer.

Read every dependency as:

```text
importer -> imported dependency
```

Open the standalone visual page:

[Current dependency architecture map](dependency-architecture-map.html)

Compare with:

[Ideal dependency architecture map](ideal-dependency-architecture-map.md)

<iframe
  src="dependency-architecture-map.html"
  title="sophon-cli current dependency architecture map"
  style="width: 100%; min-height: 1400px; border: 1px solid var(--sidebar-spacer); border-radius: 12px; background: #0b1020;"
></iframe>

If the embedded frame is cramped, open the standalone page above.
