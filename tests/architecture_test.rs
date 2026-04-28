use std::fs;
use std::path::Path;

#[test]
fn test_domain_does_not_import_outer_layers() {
    let forbidden = [
        "use crate::providers::",
        "use crate::transport::",
        "use crate::cli::",
        "use crate::app::",
    ];
    check_dir_for_forbidden_patterns("src/domain", &forbidden);
}

#[test]
fn test_transport_does_not_import_higher_layers() {
    let forbidden = [
        "use crate::providers::",
        "use crate::cli::",
        "use crate::app::",
    ];
    check_dir_for_forbidden_patterns("src/transport", &forbidden);
}

#[test]
fn test_providers_do_not_import_cli_or_app() {
    let forbidden = ["use crate::cli::", "use crate::app::"];
    check_dir_for_forbidden_patterns("src/providers", &forbidden);
}

#[test]
fn test_app_depends_only_on_domain_contracts() {
    let forbidden = [
        "use crate::cli::",
        "use crate::bootstrap::",
        "use crate::providers::",
        "use crate::transport::",
    ];
    check_dir_for_forbidden_patterns("src/app", &forbidden);
}

#[test]
fn test_bootstrap_does_not_import_cli() {
    let forbidden = ["use crate::cli::"];
    check_dir_for_forbidden_patterns("src/bootstrap", &forbidden);
}

#[test]
fn test_render_text_only_called_from_cli() {
    let forbidden_dirs = ["src/domain", "src/transport", "src/providers", "src/app"];
    for dir in &forbidden_dirs {
        visit_rust_files(dir, &|path, content| {
            assert!(
                !content.contains("render_text"),
                "render_text should only be used in CLI layer, but found in {:?}",
                path
            );
        });
    }
}

#[test]
fn test_t001_cli_request_module_import_contract() {
    let cli_mod = read_repo_file("src/cli/mod.rs");
    assert!(
        cli_mod.contains("pub mod request;"),
        "T001 requires src/cli/mod.rs to expose the request module"
    );

    let request = read_repo_file("src/cli/request.rs");
    assert!(
        request.contains("crate::cli::args") && request.contains("CliArgs"),
        "T001 request module must import CliArgs from the CLI args boundary"
    );
    assert!(
        request.contains("crate::domain") && request.contains("SearchQuery"),
        "T001 request module must import SearchQuery from the domain boundary"
    );
    assert!(
        request.contains("pub fn build_search_query"),
        "T001 request module must expose build_search_query"
    );
}

#[test]
fn test_t002_cli_runner_module_import_contract() {
    let cli_mod = read_repo_file("src/cli/mod.rs");
    assert!(
        cli_mod.contains("pub mod runner;"),
        "T002 requires src/cli/mod.rs to expose the runner module"
    );

    let runner = read_repo_file("src/cli/runner.rs");
    let required_patterns = [
        ("use clap::Parser", "parse CLI args in run_from_env"),
        ("CliArgs", "accept parsed CLI args"),
        ("CliProvider", "branch on provider selection"),
        (
            "build_search_query",
            "delegate query normalization to cli::request",
        ),
        (
            "ProviderRegistry",
            "request provider services from bootstrap",
        ),
        (
            "ProviderId",
            "identify single-provider runs through bootstrap IDs",
        ),
        (
            "render_fanout_text",
            "render fan-out output through the CLI output boundary",
        ),
        (
            "pub async fn run_from_env() -> i32",
            "expose env-backed runner entrypoint",
        ),
        (
            "pub async fn run(args: CliArgs) -> i32",
            "expose parsed-args runner entrypoint",
        ),
    ];

    for (pattern, reason) in required_patterns {
        assert!(
            runner.contains(pattern),
            "T002 runner import contract missing {pattern:?} to {reason}"
        );
    }
}

#[test]
fn test_t003_entrypoint_import_contract() {
    let main = read_repo_file("src/main.rs");
    let forbidden_patterns = [
        "use clap::Parser",
        "sophon_cli::bootstrap::",
        "sophon_cli::domain::",
        "sophon_cli::cli::args",
        "sophon_cli::cli::output",
        "mod single_provider_search",
        "single_provider_search::",
    ];

    for pattern in forbidden_patterns {
        assert!(
            !main.contains(pattern),
            "T003 requires src/main.rs to drop direct import/declaration {pattern:?}"
        );
    }

    assert!(
        main.contains("sophon_cli::cli::runner::run_from_env().await"),
        "T003 requires src/main.rs to delegate runtime execution to cli::runner::run_from_env"
    );
    assert!(
        !Path::new("src/single_provider_search.rs").exists(),
        "T003 requires src/single_provider_search.rs to be removed after its logic moves to cli::runner"
    );
}

#[test]
fn test_entrypoint_delegates_only_to_cli_surface() {
    assert_entrypoint_delegates_only_to_cli_surface();
}

#[test]
fn test_t004_ideal_dependency_direction_import_contract() {
    assert_entrypoint_delegates_only_to_cli_surface();

    let forbidden_app_patterns = [
        "use crate::cli::",
        "use crate::bootstrap::",
        "use crate::providers::",
        "use crate::transport::",
    ];
    check_dir_for_forbidden_patterns("src/app", &forbidden_app_patterns);
}

#[test]
fn test_t005_import_organization_docs_contract() {
    let import_docs = read_repo_file("docs/import-organization.md");
    assert!(
        import_docs.starts_with("---\n"),
        "T005 requires docs/import-organization.md to have YAML frontmatter"
    );

    let required_guidance = [
        "main",
        "CLI runner",
        "CLI to bootstrap",
        "bootstrap to app",
        "app to domain only",
        "providers to transport",
        "transport to domain",
        "domain to no outer layers",
    ];
    for guidance in required_guidance {
        assert!(
            import_docs.contains(guidance),
            "T005 import organization docs missing guidance phrase {guidance:?}"
        );
    }

    let current_map = read_repo_file("docs/dependency-architecture-map.html");
    assert!(
        current_map.contains("cli::runner"),
        "T005 current dependency map should show cli::runner after the refactor"
    );
    assert!(
        !current_map.contains("single_provider_search"),
        "T005 current dependency map should no longer mention single_provider_search"
    );
}

fn assert_entrypoint_delegates_only_to_cli_surface() {
    let main = read_repo_file("src/main.rs");
    let forbidden_entrypoint_patterns = [
        "sophon_cli::bootstrap::",
        "sophon_cli::domain::",
        "sophon_cli::cli::output",
        "mod single_provider_search",
        "single_provider_search::",
    ];

    for pattern in forbidden_entrypoint_patterns {
        assert!(
            !main.contains(pattern),
            "T004 requires the entrypoint to delegate only to the CLI surface; found {pattern:?}"
        );
    }

    assert!(
        !Path::new("src/single_provider_search.rs").exists(),
        "T004 requires the binary-private single_provider_search helper to be absent"
    );
}

fn check_dir_for_forbidden_patterns(dir: &str, forbidden: &[&str]) {
    visit_rust_files(dir, &|path, content| {
        for pat in forbidden {
            assert!(
                !content.contains(pat),
                "Forbidden pattern {:?} found in {:?}",
                pat,
                path
            );
        }
    });
}

fn read_repo_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| {
        panic!("Expected {path} to exist for import-organization contract test: {error}")
    })
}

fn visit_rust_files(dir: &str, callback: &dyn Fn(&Path, &str)) {
    let path = Path::new(dir);
    if !path.exists() {
        panic!("Directory {:?} does not exist", dir);
    }
    visit_dir(path, callback);
}

fn visit_dir(dir: &Path, callback: &dyn Fn(&Path, &str)) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            visit_dir(&path, callback);
        } else if path.extension().map_or(false, |e| e == "rs") {
            let content = fs::read_to_string(&path).unwrap();
            callback(&path, &content);
        }
    }
}
