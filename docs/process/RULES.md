---
title: "Rules"
when_to_read:
  - "When working with rules in the repository markdown control plane."
summary: "Documents rules for the repository markdown control plane."
ontology_relations:
  - relation: "documents"
    target: "docs/process/RULES.md"
    note: "Keeps rules discoverable for future repository work."
---
# Rules

## Non-Negotiable Rules

1. No Charter = No Code.
2. No Evidence = No Done.
3. No durable decision without a Decision Record.
4. No permanent project memory from exploration unless something durable was learned.
5. No silent scope expansion.
6. No hidden unresolved issues.
7. No stale docs knowingly left behind.
8. No giant instruction manual in `AGENTS.md`.

## Markdown-Only Control Plane

The workflow is controlled by markdown files.

Allowed control artifacts:

- `.md` charters
- `.md` plans
- `.md` evidence packs
- `.md` decision records
- `.md` memory files
- `.md` issue records
- `.md` project context

The product code can be any language. The process state must live in markdown.

## Context Rule

If future agents need to know it, put it in the repo.

Chat history is not a reliable source of truth.

## Constraint Rule

Constraints must be explicit and discoverable.

Examples:

- no external dependencies
- stdlib only
- Windows compatible
- do not touch auth
- preserve existing API
- no database migration
- preserve save-file compatibility

## Evidence Rule

Evidence must be specific.

Bad:

- "tested it"
- "looks good"
- "should work"

Good:

- command run
- exact result
- file checked
- route tested
- screenshot reviewed
- known gap documented

## Scope Rule

Every session must say what is out of scope.

Out-of-scope sections prevent agents from turning a bug fix into a rewrite.

## Garbage Collection Rule

If an artifact is obsolete, mark it Superseded or Abandoned.

Do not delete useful history unless it is actively misleading.
