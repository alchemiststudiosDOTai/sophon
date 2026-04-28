---
title: "Ideal Dependency Architecture Map"
when_to_read:
  - "When you need the clean target dependency architecture for sophon-cli."
  - "When deciding whether a Rust import points in the intended direction."
summary: "Clean ideal architecture map for sophon-cli showing the allowed top-to-bottom dependency direction across entrypoint, CLI, bootstrap, app, adapters, transport, and domain."
ontology_relations:
  - relation: "part_of"
    target: "docs/SUMMARY.md"
    note: "Belongs to the mdBook documentation set."
  - relation: "explains"
    target: "docs/architecture.md"
    note: "Visualizes the ideal dependency direction behind the architecture layers."
  - relation: "compares_with"
    target: "docs/dependency-architecture-map.md"
    note: "Pairs ideal target architecture with the current architecture map."
---

# Ideal Dependency Architecture Map

This is the clean target architecture for Rust import direction.

Read the map top-to-bottom:

```text
higher layer imports lower layer
```

The ideal shape is:

```text
Entrypoint
  ↓
CLI surface
  ↓
Bootstrap / composition root
  ↓
Application orchestration
  ↓
Adapters: providers + transport
  ↓
Domain core
```

Open the standalone visual page:

[Ideal dependency architecture map](ideal-dependency-architecture-map.html)

Compare with:

[Current dependency architecture map](dependency-architecture-map.md)

<iframe
  src="ideal-dependency-architecture-map.html"
  title="sophon-cli ideal dependency architecture map"
  style="width: 100%; min-height: 1400px; border: 1px solid var(--sidebar-spacer); border-radius: 12px; background: #0b1020;"
></iframe>

If the embedded frame is cramped, open the standalone page above.
