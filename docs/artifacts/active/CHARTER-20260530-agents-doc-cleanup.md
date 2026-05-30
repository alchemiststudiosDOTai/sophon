---
title: "Session charter: AGENTS.md and docs cleanup"
when_to_read:
  - "When reviewing the May 30, 2026 AGENTS.md cleanup and staged documentation changes."
summary: "Defines scope, constraints, evidence, and exit criteria for cleaning AGENTS.md, preparing new markdown control-plane docs, and removing .DS_Store files."
ontology_relations:
  - relation: "governs"
    target: "AGENTS.md"
    note: "Sets the session scope for rewriting the repository agent map."
---

# Session Charter

| Field | Value |
|---|---|
| Artifact Type | Session Charter |
| Status | Completed |
| Date | 2026-05-30 |
| Owner | Codex |
| Related Artifacts | `docs/artifacts/INDEX.md`, `docs/artifacts/evidence/EVID-20260530-agents-doc-cleanup.md`, `docs/artifacts/memory/MEM-20260530-agents-doc-cleanup.md` |
| Related Files | `AGENTS.md`, `.gitignore`, `docs/artifacts/`, `docs/process/`, `docs/project/` |

## Mission

Clean up `AGENTS.md` so it is a concise map, prepare the new markdown control-plane docs for staging, and remove `.DS_Store` files from the worktree/index.

## Work Type

Documentation

## Context

- `AGENTS.md` currently contains the original Rust CLI repository map plus a duplicated, poorly formatted paste of the markdown control-plane rules.
- `docs/artifacts/`, `docs/process/`, and `docs/project/` are untracked.
- Root `.DS_Store` is tracked and modified; `docs/.DS_Store` is untracked.
- `scripts/check_markdown_frontmatter.py` validates tracked Markdown files for YAML frontmatter, so new docs must be compatible before staging.

## Scope

### In Scope

- Rewrite `AGENTS.md` as a compact map that points to deeper docs.
- Add or normalize YAML frontmatter for new Markdown docs that will be staged.
- Add `.DS_Store` ignore coverage and remove existing `.DS_Store` files.
- Stage only the intended documentation and cleanup changes.

### Out of Scope

- Rust code changes.
- Architecture or product behavior changes.
- CI workflow changes unless required by documentation validation.
- Durable architecture decisions.

## Constraints

- Preserve provider/domain/application/CLI boundaries documented by the repository.
- Keep `AGENTS.md` as a map, not a full manual.
- Do not stage secrets or local environment values.
- Do not delete useful project documentation.

## Risk Areas

- Staging Markdown files without required YAML frontmatter could break `just check`.
- Removing the tracked root `.DS_Store` must be staged as a deletion.
- `AGENTS.md` must continue to reference real paths expected by CI.

## Regression Checks

- `scripts/check_markdown_frontmatter.py` should pass after staging new docs.
- `AGENTS.md` path references should exist.
- Git status should show only intended staged changes plus any pre-existing unrelated changes.

## Rollback Plan

Revert the staged documentation cleanup files and restore `.DS_Store` only if the user explicitly wants to keep it tracked.

## Plan

1. Verify repository paths, docs, validation scripts, and git state.
2. Rewrite `AGENTS.md` into one concise map.
3. Prepare new Markdown docs with required frontmatter.
4. Remove `.DS_Store` files and add ignore coverage.
5. Run focused validation, create evidence and memory artifacts, and stage intended files.

## Evidence Required

- Exact commands run.
- Markdown frontmatter validation result.
- Path/reference review result.
- Final staged file list.

## Exit Criteria

- `AGENTS.md` is concise and no longer duplicated.
- New docs intended for commit are stageable without breaking markdown frontmatter validation.
- `.DS_Store` files are removed and ignored.
- Intended files are staged.

## Clarifications Needed Before Editing

None.
