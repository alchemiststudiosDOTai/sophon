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
fn test_app_does_not_import_cli() {
    let forbidden = ["use crate::cli::"];
    check_dir_for_forbidden_patterns("src/app", &forbidden);
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
