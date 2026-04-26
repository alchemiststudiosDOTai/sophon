---
title: "Pull Request Template"
when_to_read:
  - "When opening or maintaining pull request guidance for contributors."
  - "When changing review, validation, or documentation expectations for proposed changes."
summary: "GitHub pull request template that prompts contributors for change scope, validation evidence, and reviewer context."
ontology_relations:
  - relation: "supports"
    target: "github-pr-review"
    note: "Guides pull request authors and reviewers."
---

## Description

<!-- What does this PR do and why? -->

## Testing Done

<!-- How did you verify the change? Include commands run. -->
- [ ] `just check` passes locally (fmt, clippy, tests, mdbook build)
- [ ] Unit tests added or updated for new behavior
- [ ] Architecture boundary tests still pass (`cargo test`)

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Refactor / internal change
- [ ] Documentation update

## Relevant Context

<!-- Link to issues, PRs, or design docs. Mention any breaking changes. -->
