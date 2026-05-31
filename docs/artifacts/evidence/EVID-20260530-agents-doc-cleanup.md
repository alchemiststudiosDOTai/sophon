---
title: "Evidence: AGENTS.md and docs cleanup"
when_to_read:
  - "When verifying the May 30, 2026 AGENTS.md cleanup, staged docs, and .DS_Store removal."
summary: "Records validation commands, results, staged files, known gaps, and final judgment for the AGENTS.md/control-plane documentation cleanup."
ontology_relations:
  - relation: "proves"
    target: "CHARTER-20260530-agents-doc-cleanup"
    note: "Evidence pack for the documentation cleanup charter."
---

# Evidence Pack

| Field | Value |
|---|---|
| Artifact Type | Evidence Pack |
| Status | Completed |
| Date | 2026-05-30 |
| Owner | Codex |
| Related Artifacts | `docs/artifacts/active/CHARTER-20260530-agents-doc-cleanup.md`, `docs/artifacts/active/EXEC-20260530-agents-doc-cleanup.md`, `docs/artifacts/memory/MEM-20260530-agents-doc-cleanup.md` |
| Related Files | `AGENTS.md`, `.gitignore`, `CHANGELOG.md`, `docs/artifacts/`, `docs/process/`, `docs/project/` |

## Claim Being Proven

`AGENTS.md` was cleaned into a compact repository map, new markdown control-plane docs were prepared for staging with valid frontmatter, `.DS_Store` files were removed/ignored, the push-blocking changelog coverage gap for merged PR `#17` was fixed, and PR `#18` was added to the changelog before merge.

## Files Reviewed

- `AGENTS.md`
- `README.md`
- `HARNESS.md`
- `justfile`
- `Cargo.toml`
- `.github/workflows/validate-agents.yml`
- `scripts/check_markdown_frontmatter.py`
- `tests/architecture_test.rs`
- `tests/cicd_direction_test.rs`
- `CHANGELOG.md`
- `docs/artifacts/INDEX.md`
- `docs/process/RULES.md`
- `docs/process/WORKFLOW.md`
- `docs/SUMMARY.md`
- `docs/project/PROJECT_CONTEXT.md`
- `docs/project/CONSTRAINTS.md`
- `docs/project/ARCHITECTURE.md`
- `docs/project/TESTING.md`

## Files Changed

- `AGENTS.md` - collapsed duplicated content into one 86-line map.
- `.gitignore` - added `.DS_Store`.
- `.DS_Store` - staged tracked root file for deletion.
- `CHANGELOG.md` - added the missing merged PR `#17` reference and this cleanup PR's assigned `#18` reference for changelog coverage.
- `docs/artifacts/` - added artifact index, templates, README files, session charter/log, evidence, and memory.
- `docs/process/` - added workflow/rules/handoff/garbage-collection/session-prompt docs with frontmatter.
- `docs/project/` - added project context, constraints, architecture, and testing docs with concrete repo evidence.
- `docs/dependency-direction.md`, `docs/ideal-dependency-architecture-map.md`, `docs/import-organization.md` - filled mdBook chapters referenced by `docs/SUMMARY.md`.

## Commands Run

```bash
git status --short
sed -n '1,240p' AGENTS.md
sed -n '1,260p' scripts/check_markdown_frontmatter.py
find docs/artifacts docs/process docs/project -type f -name '*.md' | sort
wc -l AGENTS.md
rm -f .DS_Store docs/.DS_Store .artifacts/plan/.DS_Store
git add AGENTS.md .gitignore .DS_Store docs/artifacts docs/process docs/project
python3 scripts/check_markdown_frontmatter.py
just check
cargo test --test changelog_test
mdbook build
find . -name .DS_Store -not -path './.git/*' -print
git diff --cached --name-status
git diff --cached --check
git push -u origin chore/refactor-agent-docs
```

## Test Results

| Check | Command / Method | Result | Notes |
|---|---|---|---|
| Markdown frontmatter | `python3 scripts/check_markdown_frontmatter.py` | Passed | Run after staging the new docs. |
| mdBook build | `mdbook build` | Passed | HTML book written to `book/`. |
| Concrete path check | Python `Path.exists()` check for `AGENTS.md` concrete paths | Passed | Checked 29 concrete paths and one memory artifact. |
| Staged diff whitespace | `git diff --cached --check` | Passed | No whitespace errors reported. |
| Changelog coverage | `cargo test --test changelog_test` | Passed | Passes after adding PR `#17` to `CHANGELOG.md`; PR `#18` was also added after draft PR creation. |
| Canonical gate | `just check` | Passed | Passed after fixing the changelog coverage gap; also passed through the pre-push hook. |
| `.DS_Store` removal | `find . -name .DS_Store -not -path './.git/*' -print` | Passed | Produced no output after removal. |

## Manual Verification

1. Reviewed `AGENTS.md` diff and confirmed duplicated pasted process text was removed.
2. Confirmed `AGENTS.md` is 86 lines and links detailed workflow rules to `docs/process/` and `docs/artifacts/`.
3. Confirmed root `.DS_Store` is staged for deletion and `.DS_Store` is ignored.
4. Confirmed the changelog coverage failure was the missing merged PR `#17` reference and added the missing entry.
5. Confirmed mdBook chapters referenced by `docs/SUMMARY.md` exist and are staged.
6. Opened draft PR `#18` and added that assigned PR number to `CHANGELOG.md`.

## Logs / Output

```text
Initial failure before changelog fix:

CHANGELOG.md is missing PR IDs: [17]
```

```text
INFO Book building has started
INFO Running the html backend
INFO HTML book written to `/Users/tuna/sophon/book`
```

## Screenshots / Visual Evidence

Not applicable for this documentation cleanup.

## Known Gaps

- No live provider searches were run because this was documentation and staging cleanup.

## Final Evidence Judgment

Proven.

The requested documentation cleanup, frontmatter compatibility, mdBook build, `.DS_Store` removal, changelog coverage fix, PR-number changelog reference, and canonical check gate are proven.
