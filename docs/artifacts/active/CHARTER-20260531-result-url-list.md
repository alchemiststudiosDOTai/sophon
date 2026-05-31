---
title: "Session charter: result URL list"
when_to_read:
  - "Before adding an end-of-output URL list to CLI rendered search results."
  - "When changing result URL rendering behavior in src/cli/output.rs."
summary: "Defines the feature scope, constraints, regression checks, and minimal implementation plan for appending a result URL list to CLI output."
ontology_relations:
  - relation: "governs"
    target: "sophon-cli-result-url-list"
    note: "Sets the execution scope for adding an end-of-results URL list."
  - relation: "depends_on"
    target: "docs/artifacts/explorations/EXP-20260531-result-url-surface.md"
    note: "Uses research showing result URLs already flow through domain results, provider mappers, and CLI rendering."
---

# Session Charter

| Field | Value |
|---|---|
| Artifact Type | Session Charter |
| Status | Completed |
| Date | 2026-05-31 |
| Owner | Codex |
| Related Artifacts | `docs/artifacts/explorations/EXP-20260531-result-url-surface.md`, `docs/artifacts/memory/MEM-20260531-result-url-research.md`, `docs/artifacts/active/EXEC-20260531-result-url-list.md`, `docs/artifacts/evidence/EVID-20260531-result-url-list.md`, `docs/artifacts/memory/MEM-20260531-result-url-implementation.md` |
| Related Files | `src/cli/output.rs`, `src/domain/result.rs`, `docs/project/ARCHITECTURE.md`, `docs/project/TESTING.md`, `HARNESS.md` |

## Mission

Append a compact list of result URLs at the end of rendered CLI search results with the least amount of code.

## Work Type

Feature

## Context

- `docs/artifacts/explorations/EXP-20260531-result-url-surface.md` found that result URLs already exist in every normalized result variant.
- Brave and Exa mappers already populate the domain URL fields.
- `src/cli/output.rs` already prints an inline `URL:` line for web, news, image, and video results.
- The requested change is a presentation-layer addition, not a data-flow change.
- Fan-out output already delegates each successful provider response to `render_text`, so a single-provider renderer change naturally appears inside each provider section.

## Scope

### In Scope

- Update `src/cli/output.rs` so `render_text` appends a final URL summary block after all rendered result details.
- Include web, news, image, and video result URLs in the same order as `response.results`.
- Keep the existing per-result `URL:` lines unchanged.
- Skip empty or whitespace-only URL strings in the final list.
- Preserve duplicate URLs if duplicate results appear.
- Let `render_fanout_text` inherit provider-local URL lists through its existing `render_text(provider_response)` call.
- Add or update one focused unit test in `src/cli/output.rs` proving the URL list appears at the end.

### Out of Scope

- Changing domain result structs.
- Changing provider DTOs, mappers, HTTP clients, or application services.
- Adding URL validation, normalization, sorting, or de-duplication.
- Adding a separate global URL list for `--provider all`.
- Changing CLI arguments, stdout/stderr routing, provider selection, or query construction.
- Updating user-facing docs for this small output-format change unless execution reveals an existing stale claim.
- Running live Brave or Exa API calls.

## Constraints

- Keep the change localized to `src/cli/output.rs` unless a test-only helper must move.
- Do not add dependencies.
- Preserve architecture boundaries: output formatting stays in `src/cli/`; domain and providers stay presentation-agnostic.
- Preserve existing text output above the appended URL list.
- Use a small helper or match expression rather than adding methods to domain types.
- New tracked Markdown must pass `python3 scripts/check_markdown_frontmatter.py`.

## Risk Areas

- Output-format tests or downstream consumers may depend on the current trailing text shape.
- Provider mappers can default missing URLs to empty strings, so the final list must not print blank bullets.
- Fan-out output will contain one URL list per successful provider section, not one aggregate list after failures.

## Regression Checks

- Existing web, news, image, and video result details still render with title and inline `URL:` lines.
- Existing snippets, news source lines, fan-out success counts, fan-out failure counts, and failure messages remain unchanged.
- A response with no non-empty URLs does not append an empty URL summary block.
- Architecture boundary tests continue to pass because no non-CLI layer imports rendering code.

## Rollback Plan

Revert the `src/cli/output.rs` renderer and unit-test changes. No data model, provider, configuration, or migration rollback is required.

## Plan

1. Add a focused output-rendering test in `src/cli/output.rs` that expects the rendered text to end with a `URLs:` block containing result URLs in result order.
2. Add the smallest local URL extraction helper or match expression needed to read `url` from `SearchResult::{Web, News, Image, Video}`.
3. In `render_text`, collect non-empty trimmed URL strings after the existing per-result rendering loop.
4. If at least one URL exists, append a final block with the exact shape:

   ```text
   URLs:
   - https://example.com/one
   - https://example.com/two
   ```

5. Confirm `render_fanout_text` needs no new branching because it already calls `render_text` for each provider response.
6. Run targeted output tests, then the repository gate if practical.
7. Create evidence and session memory before claiming the implementation complete.

## Evidence Required

- `cargo test render_text_appends_url_list_at_end` passes, or the final test name used during execution is documented.
- `cargo test` passes after the renderer change.
- `python3 scripts/check_markdown_frontmatter.py` passes after artifact updates.
- `just check` passes, or any skipped/failed check is documented in the evidence pack with the reason and output summary.
- Diff review confirms no domain, provider, transport, app, bootstrap, CLI argument, or runner changes were needed.

## Exit Criteria

- Single-provider CLI text output ends with a compact URL list when results contain non-empty URLs.
- The URL list preserves result order and includes all result variants.
- Empty URL strings do not produce blank URL-list entries.
- Fan-out output includes the same provider-local URL list through the existing renderer delegation.
- Existing inline result URL lines and surrounding output remain unchanged.
- Evidence and session memory artifacts exist for the implementation session.

## Clarifications Needed Before Editing

None. Assumption: "at the end" means at the end of each rendered `SearchResponse`; fan-out will therefore show one URL list per successful provider section unless a later requirement asks for a global batch-level URL list.
