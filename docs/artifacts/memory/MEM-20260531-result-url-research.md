---
title: "Session memory: result URL research"
when_to_read:
  - "When continuing work on URLs included in search results."
summary: "Summarizes the exploration-only research that mapped provider-returned URLs through domain results, app services, and CLI output."
uuid: "E34FAF94-9829-445E-8506-969CC613B214"
created_at: "2026-05-31T11:17:16-05:00"
ontology_relations:
  - relation: "summarizes"
    target: "docs/artifacts/explorations/EXP-20260531-result-url-surface.md"
    note: "Records the session that created the result URL surface research artifact."
---

# Session Memory

| Field | Value |
|---|---|
| Artifact Type | Session Memory |
| Status | Completed |
| Date | 2026-05-31 |
| Owner | Codex |
| Related Artifacts | `docs/artifacts/explorations/EXP-20260531-result-url-surface.md` |
| Related Files | `src/domain/result.rs`, `src/providers/brave/dto.rs`, `src/providers/brave/mapper.rs`, `src/providers/exa/dto.rs`, `src/providers/exa/mapper.rs`, `src/app/search_service.rs`, `src/app/fanout_search_service.rs`, `src/cli/output.rs`, `src/cli/runner.rs` |

## Decisions

- No durable technical decision was made.
- No implementation work was started.
- The session stayed in research/exploration mode.

## Files Changed

- `docs/artifacts/explorations/EXP-20260531-result-url-surface.md`: added factual research on how result URLs flow from provider DTOs through domain results to CLI output.
- `docs/artifacts/memory/MEM-20260531-result-url-research.md`: added this session memory.

## Evidence / Validation

- Read repository process and project context docs listed in `AGENTS.md`.
- Ran the research-phase structure, ast-scan, and symbol-index scripts.
- Reviewed source files and tests for result URL fields, provider DTO URL fields, mapper behavior, app response preservation, and CLI rendering.
- Ran direct frontmatter validation for the two new markdown files; it passed.
- Ran `python3 scripts/check_markdown_frontmatter.py`; it passed for tracked markdown.

## Open Issues

- No code issue was resolved in this session.
- A future implementation still requires a charter before source edits.

## Future Agent Notes

- The research found that current domain result variants already carry URL strings.
- The research found that current CLI text rendering already prints a `URL:` line for all current result variants.
- The research found mapper defaulting from optional provider URL fields to empty strings.
