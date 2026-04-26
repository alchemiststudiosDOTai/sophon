# Check gate: formatter, linter (+ complexity), tests, docs metadata, docs build
check:
    cargo fmt --check
    cargo clippy -- -D warnings -W clippy::complexity -W clippy::cognitive_complexity
    cargo test
    python3 scripts/check_markdown_frontmatter.py
    mdbook build

# Hygiene gate: dependencies, duplication, tech debt markers, large files
hygiene: udeps duplicates tech-debt large-files

# Uses nightly because cargo-udeps relies on nightly internals.
udeps:
    if ! cargo udeps --version >/dev/null 2>&1; then cargo install cargo-udeps --locked; fi
    cargo +nightly udeps

duplicates:
    npx --yes jscpd@4.0.5

tech-debt:
    bash scripts/check_tech_debt.sh

large-files:
    bash scripts/check_large_files.sh
