---
title: "Agent Handoff Protocol"
when_to_read:
  - "When working with agent handoff protocol in the repository markdown control plane."
summary: "Documents agent handoff protocol for the repository markdown control plane."
ontology_relations:
  - relation: "documents"
    target: "docs/process/AGENT_HANDOFF.md"
    note: "Keeps agent handoff protocol discoverable for future repository work."
---
# Agent Handoff Protocol

Use this when a new agent enters the project.

## Read Order

1. `AGENTS.md`
2. `docs/artifacts/INDEX.md`
3. `docs/project/PROJECT_CONTEXT.md`
4. `docs/project/CONSTRAINTS.md`
5. `docs/project/ARCHITECTURE.md`
6. `docs/project/TESTING.md`
7. Latest files in:
   - `docs/artifacts/memory/`
   - `docs/artifacts/open-issues/`
   - `docs/artifacts/decisions/`

## Determine Session Type

Before work, classify the session:

- Exploration
- Bug Fix
- Refactor
- Feature
- QA / Verification
- Documentation
- Release Prep

## Required First Output

For exploration:

- ask focused questions or create exploration notes

For execution:

- create a Session Charter before editing files

## Handoff Summary

When handing off to another agent, include:

- active charter
- current status
- files changed
- commands run
- known issues
- next exact step

## Bad Handoffs

Avoid:

- "continue from above"
- "fix remaining stuff"
- "make it better"
- "see chat"
- "probably done"

## Good Handoffs

Use:

- artifact links
- exact file paths
- exact commands
- exact known failures
- exact next check
