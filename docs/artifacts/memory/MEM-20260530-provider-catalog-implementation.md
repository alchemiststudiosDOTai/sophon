---
title: "Session memory: provider catalog implementation"
when_to_read:
  - "When continuing after the provider catalog implementation session."
  - "When adding or changing a search provider after the provider catalog refactor."
summary: "Records the implementation session that moved provider identity, CLI token parsing, stable ordering, env-var metadata, production construction, and the PR changelog reference into the bootstrap provider catalog work."
ontology_relations:
  - relation: "summarizes"
    target: "docs/artifacts/active/CHARTER-20260530-provider-catalog.md"
    note: "Captures the completed execution session for the active provider catalog charter."
  - relation: "supported_by"
    target: "docs/artifacts/evidence/EVID-20260530-provider-catalog.md"
    note: "Points to the validation evidence for this implementation session."
  - relation: "implements"
    target: "docs/artifacts/decisions/ADR-0001-provider-catalog.md"
    note: "Records that ADR-0001 was applied in code, tests, and docs."
---

# Session Memory

| Field | Value |
|---|---|
| Artifact Type | Session Memory |
| Status | Completed |
| Date | 2026-05-30 |
| Owner | Codex |
| Related Artifacts | `docs/artifacts/active/CHARTER-20260530-provider-catalog.md`, `docs/artifacts/active/EXEC-20260530-provider-catalog.md`, `docs/artifacts/decisions/ADR-0001-provider-catalog.md`, `docs/artifacts/evidence/EVID-20260530-provider-catalog.md` |
| Related Files | `src/bootstrap/provider_catalog.rs`, `src/bootstrap/provider_registry.rs`, `src/cli/args.rs`, `src/cli/runner.rs`, `tests/architecture_test.rs`, `tests/integration/provider_registry_test.rs`, `tests/integration/cli_test.rs`, `README.md`, `CHANGELOG.md`, `docs/architecture.md`, `docs/dependency-architecture-map.md`, `docs/dependency-architecture-map.html`, `docs/intro.md`, `docs/project/ARCHITECTURE.md`, `docs/quickstart.md`, `HARNESS.md` |

## Decisions

- Applied ADR-0001: real provider identity, display names, env vars, stable order, CLI tokens, and production builders now live in `src/bootstrap/provider_catalog.rs`.
- Kept `all` out of the catalog. It remains `CliProvider::All`, a CLI-only aggregate mode.
- Kept a compatibility re-export of `ProviderId` and `ProviderBuilder` from `provider_registry.rs` while making their defining surface the catalog.
- Preserved the existing default-provider behavior by deriving it from the first catalog entry, currently Brave.
- Added draft PR `#19` to `CHANGELOG.md` before marking the PR ready.

## Constraints

- No production code changed before the failing catalog ownership regression was added and captured.
- Domain, app, provider, transport, bootstrap, and CLI layer boundaries remain enforced by `tests/architecture_test.rs`.
- Provider-specific config loading stays in `src/providers/<provider>/config.rs`; the catalog calls those config loaders for production builders.
- A rollback commit was not created because the worktree already contained staged and untracked user artifacts. Creating one would have committed user-created work outside this execution scope.

## Files Changed

- `src/bootstrap/provider_catalog.rs`: new ordered catalog and provider construction surface.
- `src/bootstrap/provider_registry.rs`: registry now consumes catalog entries for env-backed registration and provider order.
- `src/cli/args.rs`: `CliProvider` is now `Single(ProviderId)` or `All`; real provider parsing resolves through the catalog.
- `src/cli/runner.rs`: single-provider selection uses the parsed catalog-backed provider ID; about text uses catalog display names.
- `tests/architecture_test.rs`: new guardrail proves the catalog is the provider wiring source of truth.
- `tests/integration/provider_registry_test.rs`: new catalog metadata assertions.
- `tests/integration/cli_test.rs`: new parse-time unknown-provider and real-token assertions.
- `CHANGELOG.md`: added PR `#19` to the Unreleased provider catalog entry.
- Docs and harness files: provider registration guidance now points at `src/bootstrap/provider_catalog.rs`.

## Evidence / Tests

- Evidence pack: `docs/artifacts/evidence/EVID-20260530-provider-catalog.md`.
- Failing-first regression was captured before implementation.
- Focused tests passed: `cargo test --test architecture_test`, `cargo test --test provider_registry_integration`, `cargo test --test cli_integration`, and `cargo test cli::args`.
- Docs checks passed: `python3 scripts/check_markdown_frontmatter.py` and `mdbook build`.
- Changelog follow-up checks passed: `cargo test --test changelog_test`, `python3 scripts/check_markdown_frontmatter.py`, and `just check`.
- Canonical gate passed outside the sandbox: `just check`.

## Open Issues

- The sandboxed `just check` fails in an existing HTTP unit test because the sandbox denies binding a local TCP listener. The same command passes outside the sandbox.
- No live Brave or Exa API calls were run.
- Existing staged exploration artifacts and untracked provider-catalog planning artifacts predated the implementation; they were left in place.
- `scripts/__pycache__/` is still untracked and was not removed because it was unrelated generated output.

## Future Agent Notes

- Add a real provider by editing `src/bootstrap/provider_catalog.rs` first, then provider-specific config/client/mapper modules and tests.
- Do not add real provider variants to `CliProvider`; keep it as `Single(ProviderId)` plus CLI-only aggregate modes.
- If changing provider order, change the catalog order and update catalog/registry expectations together.
- `ProviderRegistry` should stay a registry/composition consumer of catalog entries, not the owner of provider identity.
