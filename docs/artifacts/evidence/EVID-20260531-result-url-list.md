---
title: "Evidence: result URL list"
when_to_read:
  - "When verifying the CLI result URL summary implementation."
  - "When checking why the result URL list feature was considered complete."
summary: "Evidence that rendered CLI search results now append a provider-local final URL list while preserving existing inline result details."
ontology_relations:
  - relation: "supports"
    target: "docs/artifacts/active/CHARTER-20260531-result-url-list.md"
    note: "Provides validation evidence for the result URL list session charter."
  - relation: "recorded_by"
    target: "docs/artifacts/active/EXEC-20260531-result-url-list.md"
    note: "Matches the execution log for this implementation session."
---

# Evidence Pack

| Field | Value |
|---|---|
| Artifact Type | Evidence Pack |
| Status | Completed |
| Date | 2026-05-31 |
| Owner | Codex |
| Related Artifacts | `docs/artifacts/active/CHARTER-20260531-result-url-list.md`, `docs/artifacts/active/EXEC-20260531-result-url-list.md`, `docs/artifacts/memory/MEM-20260531-result-url-implementation.md` |
| Related Files | `src/cli/output.rs`, `CHANGELOG.md` |

## Claim Being Proven

Rendered single-provider CLI output now ends with a compact `URLs:` summary block when results contain non-empty URLs. The list preserves `response.results` order, trims whitespace for summary entries, skips whitespace-only URLs, preserves existing inline `URL:` lines, and fan-out output inherits one provider-local URL list per successful provider through the existing `render_text` delegation.

The unreleased changelog also references PR `#20` for this user-visible output change.

## Files Reviewed

- `docs/artifacts/active/CHARTER-20260531-result-url-list.md`
- `docs/artifacts/active/EXEC-20260531-result-url-list.md`
- `src/cli/output.rs`
- `src/domain/result.rs`

## Files Changed

- `src/cli/output.rs`: appended URL summary rendering to `render_text`, added a local `result_url` helper, and added the focused `render_text_appends_url_list_at_end` unit test.
- `CHANGELOG.md`: added the unreleased result URL list entry with PR `#20`.
- `docs/artifacts/active/EXEC-20260531-result-url-list.md`: recorded execution progress and validation.
- `docs/artifacts/evidence/EVID-20260531-result-url-list.md`: added this evidence pack.
- `docs/artifacts/memory/MEM-20260531-result-url-implementation.md`: added session memory.

## Commands Run

```bash
cargo fmt
cargo test render_text_appends_url_list_at_end
cargo test
just check
cargo test --test changelog_test
python3 scripts/check_markdown_frontmatter.py
git diff -- src/cli/output.rs docs/artifacts/active/EXEC-20260531-result-url-list.md
```

## Test Results

| Check | Command / Method | Result | Notes |
|---|---|---|---|
| Focused renderer test | `cargo test render_text_appends_url_list_at_end` | Passed | 1 passed; proves final URL block ordering and whitespace-only URL skipping in the summary. |
| Full Rust suite | `cargo test` | Passed | 30 lib tests, 13 architecture tests, 2 changelog tests, 3 CI direction tests, 8 CLI integration tests, 1 fan-out CLI test, 13 provider registry tests, and 6 search service tests passed. |
| Canonical gate | `just check` | Passed | Includes `cargo fmt --check`, clippy with warnings denied, `cargo test`, markdown frontmatter, and `mdbook build`. |
| Changelog coverage | `cargo test --test changelog_test` | Passed | Passes after adding PR `#20` to `CHANGELOG.md`. |
| Markdown frontmatter | `python3 scripts/check_markdown_frontmatter.py` | Passed | Final artifact check completed without errors. |
| Diff review | `git diff -- src/cli/output.rs ...` and source review | Passed | Code changes are limited to CLI rendering and unit tests; no domain, provider, transport, app, bootstrap, CLI argument, or runner changes were needed. |

## Manual Verification

1. Reviewed `src/cli/output.rs` and confirmed existing per-result inline `URL:` lines remain unchanged for web, news, image, and video results.
2. Confirmed `render_text` collects URLs after the normal result rendering loop, trims entries for the summary, skips whitespace-only URLs, and appends the summary only when at least one URL remains.
3. Confirmed `render_fanout_text` still calls `render_text(provider_response)` for each successful provider, so fan-out inherits provider-local URL lists without new fan-out branching.
4. Reviewed architecture scope: no domain, provider, transport, app, bootstrap, CLI argument, or runner code changed.

## Known Gaps

- No live Brave or Exa API calls were run; the charter explicitly excluded live provider calls.
- Existing uncommitted documentation/artifact changes predated implementation and were left in place.
- A rollback commit was not created because it would have committed pre-existing work outside this scope.

## Final Evidence Judgment

Proven. The renderer now appends the requested URL list, the focused regression covers the output ending and URL filtering behavior, fan-out inherits the behavior through the existing renderer delegation, and the canonical `just check` gate passed.
