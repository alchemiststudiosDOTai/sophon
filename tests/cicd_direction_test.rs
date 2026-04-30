use std::fs;

#[test]
fn pre_push_hook_runs_the_canonical_check_gate() {
    let pre_push = read_repo_file(".cargo-husky/hooks/pre-push");

    assert!(
        pre_push.starts_with("#!/bin/sh"),
        "pre-push hook must remain executable by cargo-husky's POSIX hook runner"
    );
    assert!(
        pre_push.contains("set -e"),
        "pre-push hook must stop at the first failed command"
    );
    assert!(
        pre_push.contains("just check"),
        "pre-push hook must run the canonical local check gate"
    );
}

#[test]
fn ci_workflow_runs_canonical_check_before_hygiene() {
    let workflow = read_repo_file(".github/workflows/validate-agents.yml");

    assert_contains_in_order(
        &workflow,
        &[
            "uses: actions/checkout@v4",
            "Verify AGENTS.md referenced paths exist",
            "Setup Rust toolchain",
            "Install just, mdbook, and cargo-udeps",
            "Verify canonical command works",
            "run: just check",
            "Install nightly toolchain for cargo-udeps",
            "Setup Node.js for jscpd",
            "Install ripgrep for tech debt checks",
            "Run hygiene checks",
            "run: just hygiene",
        ],
        "CI validation must preserve setup -> canonical check -> hygiene direction",
    );
}

#[test]
fn harness_documents_pre_push_and_ci_direction() {
    let harness = read_repo_file("HARNESS.md");

    assert!(
        harness.contains("| pre-push | `.cargo-husky/hooks/pre-push`")
            && harness.contains("| `.github/workflows/validate-agents.yml`"),
        "HARNESS.md must document both the pre-push hook and CI workflow"
    );
    assert!(
        harness.contains("`tests/cicd_direction_test.rs`"),
        "HARNESS.md must list the CI/CD direction test as a harness source"
    );
}

fn read_repo_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| {
        panic!("Expected {path} to exist for CI/CD direction test: {error}")
    })
}

fn assert_contains_in_order(content: &str, patterns: &[&str], context: &str) {
    let mut search_from = 0;

    for pattern in patterns {
        let remaining = &content[search_from..];
        let Some(relative_index) = remaining.find(pattern) else {
            panic!("{context}; missing pattern {pattern:?}");
        };
        search_from += relative_index + pattern.len();
    }
}
