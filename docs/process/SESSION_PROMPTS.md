---
title: "Session Prompts"
when_to_read:
  - "When working with session prompts in the repository markdown control plane."
summary: "Documents session prompts for the repository markdown control plane."
ontology_relations:
  - relation: "documents"
    target: "docs/process/SESSION_PROMPTS.md"
    note: "Keeps session prompts discoverable for future repository work."
---
# Session Prompts

Paste these into agent sessions.

## Exploration Start

```text
We are starting an exploration session.

Do not edit files yet.

Help clarify the problem, options, assumptions, and likely next step.

At the end, create an Exploration Memory artifact using:

docs/artifacts/templates/TEMPLATE_EXPLORATION_MEMORY.md

Do not present exploration notes as final decisions.
Do not update permanent project memory unless something durable was clearly learned.
```

## Execution Start

```text
Before starting this work session, create a Session Charter.

Do not edit files yet.

Use:

docs/artifacts/templates/TEMPLATE_SESSION_CHARTER.md

Define mission, work type, context, scope, constraints, plan, evidence required, and exit criteria.

For bug fixes or refactors, also include risk areas, regression checks, and rollback plan.

If mission, scope, constraints, or exit criteria are unclear, stop before editing files.
```

## Bug Fix Start

```text
We are starting a bug fix session.

Do not edit files until a Bug Fix Charter exists.

Use:

docs/artifacts/templates/TEMPLATE_BUG_FIX_CHARTER.md

First reproduce or define the bug, expected behavior, actual behavior, risk areas, regression checks, rollback plan, evidence required, and exit criteria.
```

## Refactor Start

```text
We are starting a refactor session.

Do not edit files until a Refactor Charter exists.

Use:

docs/artifacts/templates/TEMPLATE_REFACTOR_CHARTER.md

This refactor must preserve external behavior unless explicitly stated otherwise.

Define the behavior preservation contract, risk areas, regression checks, rollback plan, evidence required, and exit criteria.
```

## Session End

```text
At the end of this session, create a Session Memory artifact.

Save only durable, future-useful information:
- decisions made
- constraints agreed on
- files changed
- commands/tests run
- bugs found or fixed
- unresolved issues
- lessons future agents should not rediscover

Do not save temporary chat noise, speculation, or outdated ideas.

Use:

docs/artifacts/templates/TEMPLATE_SESSION_MEMORY.md

Keep it concise, factual, and tied to evidence.
```

## Evidence Pack Request

```text
Before claiming this work is done, create an Evidence Pack.

Use:

docs/artifacts/templates/TEMPLATE_EVIDENCE_PACK.md

Include exact commands, test results, manual verification, logs or excerpts, known gaps, and a final evidence judgment.
```
