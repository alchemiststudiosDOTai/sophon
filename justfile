# Check gate: formatter, linter (+ complexity), tests, docs build
check:
    cargo fmt --check
    cargo clippy -- -D warnings -W clippy::complexity -W clippy::cognitive_complexity
    cargo test
    mdbook build
