use std::process::Command;

#[test]
fn cli_about_flag_prints_description() {
    let output = Command::new(env!("CARGO_BIN_EXE_sophon-cli"))
        .args(["--about"])
        .output()
        .expect("sophon-cli runs");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("sophon-cli"));
    assert!(stdout.contains("Three-Body Problem"));
    assert!(stdout.contains("Brave Search"));
    assert!(stdout.contains("Exa"));
}

#[test]
fn cli_help_flag_prints_usage() {
    let output = Command::new(env!("CARGO_BIN_EXE_sophon-cli"))
        .args(["--help"])
        .output()
        .expect("sophon-cli runs");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--provider"));
    assert!(stdout.contains("--search-type"));
    assert!(stdout.contains("--limit"));
    assert!(stdout.contains("--about"));
    assert!(stdout.contains("--safe-search"));
}

#[test]
fn cli_missing_query_exits_with_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_sophon-cli"))
        .output()
        .expect("sophon-cli runs");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing query"),
        "stderr did not contain missing query: {stderr}"
    );
}

#[test]
fn cli_brave_provider_without_key_exits_with_unavailable_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_sophon-cli"))
        .args(["rust", "--provider", "brave"])
        .env_remove("BRAVE_API_KEY")
        .env_remove("EXA_API_KEY")
        .current_dir(std::env::temp_dir())
        .output()
        .expect("sophon-cli runs");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unavailable") || stderr.contains("NoProvidersAvailable"),
        "stderr did not contain expected error: {stderr}"
    );
}

#[test]
fn cli_exa_provider_without_key_exits_with_unavailable_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_sophon-cli"))
        .args(["rust", "--provider", "exa"])
        .env_remove("BRAVE_API_KEY")
        .env_remove("EXA_API_KEY")
        .current_dir(std::env::temp_dir())
        .output()
        .expect("sophon-cli runs");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unavailable") || stderr.contains("NoProvidersAvailable"),
        "stderr did not contain expected error: {stderr}"
    );
}

#[test]
fn cli_with_explicit_arguments_parses_correctly() {
    let output = Command::new(env!("CARGO_BIN_EXE_sophon-cli"))
        .args(["--help"])
        .output()
        .expect("sophon-cli runs");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("sophon-cli"));
}
