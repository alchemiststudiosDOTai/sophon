# sophon-cli

> Named after the **Sophon** from Cixin Liu's *Three-Body Problem* — a sentient proton supercomputer. This is a tiny CLI that delegates its heavy lifting to distant search APIs.

![sophon](docs/sophon.png)

A provider-agnostic Rust CLI that queries the Brave Search API and prints normalized text results.

## Quick start

```bash
# Set your API key
echo "BRAVE_API_KEY=your_key_here" > .env

# Run a search
cargo run -- "rust programming"

# About
cargo run -- --about

# Run all checks
just check
```

## Docs

See the [architecture docs](docs/architecture.md) for the typed input-to-output flow and layer boundaries.

Built with `mdbook` — run `mdbook build` to generate the HTML site.
