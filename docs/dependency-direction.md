---
title: "Dependency Direction Visual"
when_to_read:
  - "When you want a small visual map of sophon-cli dependency direction without the full transitive Cargo graph."
  - "When checking direct crate dependencies or internal layer dependency direction."
summary: "Embedded visual explainer for sophon-cli dependency direction: grouped direct Cargo dependencies and top-to-bottom internal module direction."
ontology_relations:
  - relation: "part_of"
    target: "docs/SUMMARY.md"
    note: "Belongs to the mdBook documentation set."
  - relation: "explains"
    target: "docs/architecture.md"
    note: "Visualizes the architecture dependency direction described by the architecture page."
---

# Dependency Direction Visual

This page is a small, organized alternative to the full `cargo visualize --all-deps` graph.

Use it for two questions:

1. **Which crates does `sophon-cli` directly depend on?**
2. **Which direction should internal modules depend on?**

Open the standalone visual page:

[Dependency direction visual](dependency-direction.html)

<iframe
  src="dependency-direction.html"
  title="sophon-cli dependency direction visual"
  style="width: 100%; min-height: 1100px; border: 1px solid var(--sidebar-spacer); border-radius: 12px; background: #faf7f5;"
></iframe>

If the embedded frame is cramped, open the standalone page above.

## Related commands

Direct dependencies only:

```bash
cargo tree --depth 1
```

Small browser graph:

```bash
cargo visualize --depth 1 --dedup-transitive-deps
```

Focused browser graph for one crate:

```bash
cargo visualize --focus reqwest --depth 2 --dedup-transitive-deps
```

Reverse dependency question:

```bash
cargo tree --invert tokio
```
