# Check gate: formatter, linter (+ complexity), tests, docs metadata, docs build
check:
    cargo fmt --check
    cargo clippy -- -D warnings -W clippy::complexity -W clippy::cognitive_complexity
    cargo test
    python3 scripts/check_markdown_frontmatter.py
    mdbook build
