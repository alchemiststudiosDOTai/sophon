---
title: "Documentation Garbage Collection"
when_to_read:
  - "When working with documentation garbage collection in the repository markdown control plane."
summary: "Documents documentation garbage collection for the repository markdown control plane."
ontology_relations:
  - relation: "documents"
    target: "docs/process/GARBAGE_COLLECTION.md"
    note: "Keeps documentation garbage collection discoverable for future repository work."
---
# Documentation Garbage Collection

Agent workflows create documentation entropy.

This file defines how to keep the markdown control plane useful.

## Goals

- remove stale instructions
- mark superseded artifacts
- keep indexes current
- prevent duplicate decisions
- prevent old exploration notes from becoming fake truth

## Cadence

Run doc cleanup:

- after major feature completion
- before release
- after architecture changes
- when an agent reports conflicting docs

## Cleanup Checklist

1. Check active charters.
   - Move completed ones out of `active/`.
   - Mark abandoned ones as Abandoned.

2. Check decisions.
   - Ensure new constraints are reflected in `docs/project/CONSTRAINTS.md`.
   - Mark superseded ADRs.

3. Check open issues.
   - Close resolved issues.
   - Link issues to evidence when fixed.

4. Check memory.
   - Keep concise.
   - Remove temporary chat noise.
   - Preserve durable lessons.

5. Check `AGENTS.md`.
   - Keep it as a map.
   - Move detailed instructions into docs.

## Superseding Artifacts

Do not rewrite history.

At the top of the old artifact, set:

```md
| Status | Superseded |
| Superseded By | path/to/new-artifact.md |
```

## Deleting Artifacts

Deletion is allowed only when the artifact is:

- duplicate
- empty
- misleading and already superseded
- temporary scratch accidentally committed

Prefer superseding over deletion.
