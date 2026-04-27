#[path = "../common/cli.rs"]
mod cli;

use clap::Parser;
use sophon_cli::cli::args::{CliArgs, CliProvider, CliSafeSearch, CliSearchType};

#[test]
fn cli_about_flag_prints_description() {
    let output = cli::run_cli(&["--about"]);

    assert!(output.status.success());
    cli::assert_stderr_empty(&output);

    let stdout = cli::stdout_text(&output);
    assert!(stdout.contains("sophon-cli"));
    assert!(stdout.contains("Three-Body Problem"));
    assert!(stdout.contains("Brave Search"));
    assert!(stdout.contains("Exa"));
}

#[test]
fn cli_help_flag_prints_usage() {
    let output = cli::run_cli(&["--help"]);

    assert!(output.status.success());
    cli::assert_stderr_empty(&output);

    let stdout = cli::stdout_text(&output);
    assert!(stdout.contains("--provider"));
    assert!(stdout.contains("--search-type"));
    assert!(stdout.contains("--limit"));
    assert!(stdout.contains("--about"));
    assert!(stdout.contains("--safe-search"));
}

#[test]
fn cli_missing_query_exits_with_error() {
    let output = cli::run_cli(&[]);

    assert!(!output.status.success());
    cli::assert_stdout_empty(&output);

    let stderr = cli::stderr_text(&output);
    assert!(
        stderr.contains("missing query"),
        "stderr did not contain missing query: {stderr}"
    );
}

#[test]
fn cli_brave_provider_without_key_exits_with_provider_unavailable_error() {
    let output = cli::run_cli_without_keys(&["rust", "--provider", "brave"]);

    assert_explicit_provider_unavailable(output, "brave");
}

#[test]
fn cli_exa_provider_without_key_exits_with_provider_unavailable_error() {
    let output = cli::run_cli_without_keys(&["rust", "--provider", "exa"]);

    assert_explicit_provider_unavailable(output, "exa");
}

#[test]
fn cli_with_explicit_arguments_parses_correctly() {
    let args = CliArgs::try_parse_from([
        "sophon-cli",
        "rust search",
        "--provider",
        "all",
        "--search-type",
        "news",
        "--limit",
        "3",
        "--safe-search",
        "strict",
        "--country",
        "US",
        "--language",
        "en",
    ])
    .expect("explicit args parse");

    assert_eq!(args.query.as_deref(), Some("rust search"));
    assert_eq!(args.provider, CliProvider::All);
    assert_eq!(args.search_type, CliSearchType::News);
    assert_eq!(args.limit, Some(3));
    assert_eq!(args.safe_search, Some(CliSafeSearch::Strict));
    assert_eq!(args.country.as_deref(), Some("US"));
    assert_eq!(args.language.as_deref(), Some("en"));
}

fn assert_explicit_provider_unavailable(output: std::process::Output, provider: &str) {
    assert!(!output.status.success());
    cli::assert_stdout_empty(&output);

    let stderr = cli::stderr_text(&output);
    assert!(
        stderr.contains(&format!("provider `{provider}` is unavailable")),
        "stderr did not contain {provider} unavailable error: {stderr}"
    );
    assert!(
        stderr.contains("configured providers: []"),
        "stderr did not include configured provider list: {stderr}"
    );
    assert!(
        !stderr.contains("no configured providers") && !stderr.contains("NoProvidersAvailable"),
        "explicit provider should not use fan-out no-provider error: {stderr}"
    );
}
