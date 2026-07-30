//! Process-level coverage for the scaffold qualification executable.

use std::process::{Command, Output};

fn qualify(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_moria-qualify"))
        .args(arguments)
        .output()
        .expect("moria-qualify should start")
}

#[test]
fn documented_scaffold_commands_succeed_with_truthful_results() {
    let shaders = qualify(&["shaders", "validate"]);
    assert!(shaders.status.success());
    assert!(String::from_utf8_lossy(&shaders.stdout).contains("PASS shaders validate"));

    let replay = qualify(&[
        "replay",
        "verify",
        "--fixture",
        "fixtures/replay/core-v1",
        "--runs",
        "8",
        "--evidence",
        "target/moria-evidence/replay",
    ]);
    assert!(replay.status.success());
    assert!(String::from_utf8_lossy(&replay.stdout).contains("UNAVAILABLE replay verify"));

    let scenario = qualify(&[
        "scenario",
        "public-boundary",
        "--mode",
        "candidate",
        "--evidence",
        "target/moria-evidence/dev",
    ]);
    assert!(scenario.status.success());
    assert!(
        String::from_utf8_lossy(&scenario.stdout).contains("UNAVAILABLE scenario public-boundary")
    );
}

#[test]
fn malformed_commands_fail_with_usage_exit_code() {
    let output = qualify(&["replay", "verify"]);

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("usage: moria-qualify"));
}
