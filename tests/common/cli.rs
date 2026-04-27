#![allow(dead_code)]

use std::process::{Command, Output};

pub fn run_cli(args: &[&str]) -> Output {
    cli_command(args).output().expect("sophon-cli runs")
}

pub fn run_cli_without_keys(args: &[&str]) -> Output {
    cli_command(args)
        .env("BRAVE_API_KEY", "")
        .env("EXA_API_KEY", "")
        .env_remove("RUST_LOG")
        .current_dir(std::env::temp_dir())
        .output()
        .expect("sophon-cli runs")
}

fn cli_command(args: &[&str]) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_sophon-cli"));
    command.args(args);
    command
}

pub fn stdout_text(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

pub fn stderr_text(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

pub fn assert_stdout_empty(output: &Output) {
    let stdout = stdout_text(output);
    assert!(stdout.is_empty(), "unexpected stdout: {stdout}");
}

pub fn assert_stderr_empty(output: &Output) {
    let stderr = stderr_text(output);
    assert!(stderr.is_empty(), "unexpected stderr: {stderr}");
}
