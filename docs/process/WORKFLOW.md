---
title: "Workflow"
when_to_read:
  - "When working with workflow in the repository markdown control plane."
summary: "Documents workflow for the repository markdown control plane."
ontology_relations:
  - relation: "documents"
    target: "docs/process/WORKFLOW.md"
    note: "Keeps workflow discoverable for future repository work."
---
# Workflow

This system has three distinct memory types.

## 1. Exploration Memory

Purpose:

- preserve where thinking currently is
- keep orientation for future sessions
- avoid pretending provisional thinking is final

Use when:

- the team is brainstorming
- the problem is unclear
- no execution plan exists
- options are being compared

Do not use exploration memory to create permanent commitments.

## 2. Decision Memory

Purpose:

- preserve what was agreed
- document tradeoffs
- create constraints for future agents

Use when:

- a technical direction is chosen
- a product behavior is settled
- an architecture rule is created
- a constraint changes

Use Decision Records for this.

## 3. Execution Memory

Purpose:

- preserve what actually happened
- record files changed
- record tests and commands
- document unresolved issues

Use at the end of every real work session.

## Standard Flow

### Exploration Flow

1. Ask questions.
2. Read relevant context.
3. Compare options.
4. Mark assumptions.
5. Create Exploration Memory.

### Execution Flow

1. Create Session Charter.
2. Confirm no blocking clarifications remain.
3. Edit files.
4. Update Execution Log.
5. Run evidence checks.
6. Create Evidence Pack.
7. Create Session Memory.
8. Move completed charter/log out of active when appropriate.

### Decision Flow

1. Identify decision point.
2. List options.
3. State decision.
4. State rationale.
5. State constraints created.
6. Create ADR.

## Stop Conditions

Stop before editing files when:

- no mission exists
- scope is unclear
- exit criteria are missing
- hard constraints conflict
- the requested change appears unsafe
- required files are missing

Stop before claiming done when:

- evidence is missing
- tests were not run and no reason is documented
- unresolved issues are hidden
- changed files are not listed
