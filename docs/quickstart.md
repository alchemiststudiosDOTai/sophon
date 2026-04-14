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
