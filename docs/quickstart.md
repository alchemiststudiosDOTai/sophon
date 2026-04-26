---
title: "Quick Start"
when_to_read:
  - "When reading or editing the mdBook documentation surface."
  - "When checking how the CLI architecture, quickstart, and user-facing docs fit together."
summary: "mdBook documentation page for sophon-cli: Quick Start. It contributes user and maintainer guidance that is built by the docs gate."
ontology_relations:
  - relation: "part_of"
    target: "docs/SUMMARY.md"
    note: "Belongs to the mdBook documentation set."
---

# Quick Start

## Prerequisites

- Rust toolchain (edition 2024)
- `BRAVE_API_KEY` in a `.env` file at the project root

## Install the task runner

```bash
cargo install just
```

## Run checks

```bash
just check
```

## Run a search

```bash
cargo run -- "rust programming"
```

## Search news

```bash
cargo run -- "nvim treesitter" --search-type news --limit 3
```

## Safe search and country filters

```bash
cargo run -- "space x" --safe-search strict --country US --language en
```
