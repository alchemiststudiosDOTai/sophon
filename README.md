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

### Output format

Each result is printed inline with its title and `URL:`. A trailing `URLs:` block collects every non-empty result link so they're easy to copy in one pass:

```text
Provider: brave
Query: rust programming
Results: 2

1. [Rust Programming Language]
   URL: https://www.rust-lang.org
   A language empowering everyone to build reliable and efficient software.
2. [Rust (programming language) - Wikipedia]
   URL: https://en.wikipedia.org/wiki/Rust_(programming_language)

URLs:
- https://www.rust-lang.org
- https://en.wikipedia.org/wiki/Rust_(programming_language)
```

Result ordering and inline `URL:` lines are unchanged; whitespace-only URLs are skipped in the `URLs:` list.

## Supported providers

Real provider tokens:

- `brave` for web, news, images, and video search
- `exa` for Exa search results mapped into the shared domain model

`all` is a CLI-only selection mode. It queries every configured real provider in catalog order and prints per-provider failures when one provider rejects or fails a request.

Maintainers add or change provider identity, display names, environment variable names, stable ordering, and production wiring in `src/bootstrap/provider_catalog.rs`.

## Docs

See the [architecture docs](docs/architecture.md) for the typed input-to-output flow and layer boundaries.

Built with `mdbook` — run `mdbook build` to generate the HTML site.
