# sophon-cli

> Named after the **Sophon** from Cixin Liu's *Three-Body Problem* — a sentient proton supercomputer. This is a tiny CLI that delegates its heavy lifting to distant search APIs.

![sophon](docs/sophon.png)

A provider-agnostic Rust CLI that queries Brave Search, Exa, or every environment-enabled provider and prints normalized text results.

## Install

```bash
cargo install sophon-cli
```

## Quick start

```bash
# Run locally from the repo
cargo run -- "rust programming"

# Choose a provider explicitly
cargo run -- "rust programming" --provider brave
cargo run -- "rust programming" --provider exa

# Query every provider enabled by environment variables
cargo run -- "rust async trait" --provider all

# Show package info
cargo run -- --about

# Run all checks
just check
```

## Configuration

Set the API key for the provider you want to use:

```bash
# Brave
echo "BRAVE_API_KEY=your_key_here" > .env

# Exa
echo "EXA_API_KEY=your_key_here" > .env
```

You can also export the variables directly in your shell instead of using `.env`. `--provider all` queries every provider enabled by the current environment; for example, set both `BRAVE_API_KEY` and `EXA_API_KEY` to fan out to both providers.

## Example usage

```bash
# Web search with Brave
sophon-cli "rust programming" --provider brave

# News search with Brave
sophon-cli "open source ai" --provider brave --search-type news --limit 3

# Exa search
sophon-cli "vector database benchmarks" --provider exa --limit 5

# All configured providers, with per-provider successes and failures
sophon-cli "rust async trait" --provider all
```

## Supported providers

- `brave` for web, news, images, and video search
- `exa` for Exa search results mapped into the shared domain model
- `all` to query every configured provider in stable order and print per-provider failures when one provider rejects or fails a request

## Docs

See the [architecture docs](docs/architecture.md) for the typed input-to-output flow and layer boundaries.

Built with `mdbook` — run `mdbook build` to generate the HTML site.
