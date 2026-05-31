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
fn test_provider_catalog_is_provider_wiring_source_of_truth() {
    assert!(
        Path::new("src/bootstrap/provider_catalog.rs").exists(),
        "provider identity and production wiring must live in src/bootstrap/provider_catalog.rs"
    );

    let bootstrap_mod = read_repo_file("src/bootstrap/mod.rs");
    assert!(
        bootstrap_mod.contains("pub mod provider_catalog;"),
        "bootstrap must expose the provider catalog module"
    );

    let catalog = read_repo_file("src/bootstrap/provider_catalog.rs");
    for required_pattern in [
        "pub enum ProviderId",
        "ProviderCatalogEntry",
        "PROVIDER_CATALOG",
        "BraveProvider::new",
        "ExaProvider::new",
        "BRAVE_API_KEY",
        "EXA_API_KEY",
    ] {
        assert!(
            catalog.contains(required_pattern),
            "provider catalog must contain provider wiring pattern {required_pattern:?}"
        );
    }

    let registry = read_repo_file("src/bootstrap/provider_registry.rs");
    let registry_impl = implementation_region(&registry);
    for forbidden_pattern in [
        "pub enum ProviderId",
        "BraveConfig::from_env",
        "ExaConfig::from_env",
        "BraveProvider::new",
        "ExaProvider::new",
        "[ProviderId::Brave, ProviderId::Exa]",
        "BRAVE_API_KEY and/or EXA_API_KEY",
    ] {
        assert!(
            !registry_impl.contains(forbidden_pattern),
            "provider registry wiring must come from provider_catalog, but found {forbidden_pattern:?}"
        );
    }

    let cli_args = read_repo_file("src/cli/args.rs");
    let cli_args_impl = implementation_region(&cli_args);
    for forbidden_pattern in [
        "CliProvider {\n    Brave",
        "ValueEnum)]\npub enum CliProvider",
    ] {
        assert!(
            !cli_args_impl.contains(forbidden_pattern),
            "CLI provider parsing must resolve real providers from provider_catalog; found {forbidden_pattern:?}"
        );
    }

    let runner = read_repo_file("src/cli/runner.rs");
    let runner_impl = implementation_region(&runner);
    for forbidden_pattern in [
        "CliProvider::Brave",
        "CliProvider::Exa",
        "ProviderId::Brave",
        "ProviderId::Exa",
    ] {
        assert!(
            !runner_impl.contains(forbidden_pattern),
            "CLI runner must use catalog-backed provider IDs; found {forbidden_pattern:?}"
        );
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
fn test_t005_current_dependency_map_reflects_refactor() {
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

fn implementation_region(content: &str) -> &str {
    content.split("#[cfg(test)]").next().unwrap_or(content)
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
