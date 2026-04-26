use std::process::Command;

#[test]
fn provider_all_without_config_exits_nonzero_with_no_provider_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_sophon-cli"))
        .args(["rust", "--provider", "all"])
        .env_remove("BRAVE_API_KEY")
        .env_remove("EXA_API_KEY")
        .current_dir(std::env::temp_dir())
        .output()
        .expect("sophon-cli runs");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("no configured providers"),
        "stderr did not contain no configured providers: {stderr}"
    );
}
