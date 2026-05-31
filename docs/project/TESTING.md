---
title: "Testing"
when_to_read:
  - "When choosing validation commands or evidence for sophon-cli changes."
summary: "Defines standard check commands, smoke tests, regression checks, evidence expectations, and known test gaps."
ontology_relations:
  - relation: "defines"
    target: "sophon-cli-validation"
    note: "Documents the testing and evidence commands future agents should use."
---

# Testing

| Field | Value |
|---|---|
| Artifact Type | Testing Guide |
| Status | Active |
| Date | 2026-05-30 |
| Owner | Project maintainers |
| Related Artifacts | `HARNESS.md` |
| Related Files | `justfile`, `scripts/check_markdown_frontmatter.py`, `tests/` |

## Standard Commands

```bash
# canonical local gate
just check

# Rust tests only
cargo test

# formatting only
cargo fmt --check

# lint only
cargo clippy -- -D warnings -W clippy::complexity -W clippy::cognitive_complexity

# docs metadata only
python3 scripts/check_markdown_frontmatter.py

# docs build only
mdbook build

# hygiene gate
just hygiene
```

Windows PowerShell should use:

```powershell
just --shell powershell --shell-arg -Command check
```

## Smoke Test

Use a no-network smoke test when API keys are not available:

1. Run `cargo run -- --about`.
2. Confirm package/about output is printed to stdout.
3. Confirm no panic or unexpected stderr error appears.

Use a live provider smoke test only when the relevant key is available:

```bash
cargo run -- "rust programming" --provider brave --limit 3
cargo run -- "vector database benchmarks" --provider exa --limit 3
```

## Regression Checks

Project-wide checks that must keep passing:

- Architecture boundary tests in `tests/architecture_test.rs`.
- CI/harness direction tests in `tests/cicd_direction_test.rs`.
- Changelog coverage tests in `tests/changelog_test.rs`.
- Fan-out CLI tests in `tests/fanout_cli_test.rs`.
- Integration tests under `tests/integration/`.
- Markdown frontmatter validation through `scripts/check_markdown_frontmatter.py`.
- mdBook build through `mdbook build`.

## Bug Fix Evidence

Minimum evidence for bug fixes:

- Reproduction before fix or a precise reason reproduction is not possible.
- Passing targeted regression check after fix.
- `just check` when practical.
- Changed files reviewed.

## Refactor Evidence

Minimum evidence for refactors:

- Before/after behavior check.
- Tests or manual verification proving behavior preservation.
- Diff review showing structural-only change where applicable.
- Known gaps documented.

## Release Evidence

Minimum evidence for release:

- `just check` passes.
- `just hygiene` passes or gaps are documented.
- Smoke test passes.
- Open issues reviewed.
- Rollback plan exists.

## Known Test Gaps

- Live Brave and Exa API calls are not part of automated tests.
- No snapshot or golden-output test suite is configured.
- No docs link checker is configured.
- `cargo +nightly udeps` requires a nightly toolchain and may be slower than the canonical check gate.
