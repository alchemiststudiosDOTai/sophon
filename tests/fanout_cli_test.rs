#[path = "common/cli.rs"]
mod cli;

#[test]
fn provider_all_without_config_exits_nonzero_with_no_provider_error() {
    let output = cli::run_cli_without_keys(&["rust", "--provider", "all"]);

    assert!(!output.status.success());
    cli::assert_stdout_empty(&output);

    let stderr = cli::stderr_text(&output);
    assert!(
        stderr.contains("no configured providers"),
        "stderr did not contain no configured providers: {stderr}"
    );
}
