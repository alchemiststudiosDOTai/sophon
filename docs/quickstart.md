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
- `BRAVE_API_KEY` and/or `EXA_API_KEY` in a `.env` file at the project root

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

## Run all configured providers

Set both provider keys when you want `--provider all` to query Brave and Exa in one run:

```bash
export BRAVE_API_KEY=your_brave_key
export EXA_API_KEY=your_exa_key
cargo run -- "rust async trait" --provider all
```

`--provider all` includes only providers enabled by the current environment variables. If neither `BRAVE_API_KEY` nor `EXA_API_KEY` is available, the command exits non-zero and prints `no configured providers; set BRAVE_API_KEY and/or EXA_API_KEY`.

The real provider tokens and their environment variables are declared in `src/bootstrap/provider_catalog.rs`. `all` is a CLI-only mode, not a real provider.

## Search news

```bash
cargo run -- "nvim treesitter" --search-type news --limit 3
```

## Safe search and country filters

```bash
cargo run -- "space x" --safe-search strict --country US --language en
```
