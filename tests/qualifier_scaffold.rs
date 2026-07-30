//! Process-level coverage for the scaffold qualification executable.

use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
};

fn qualify(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_moria-qualify"))
        .args(arguments)
        .output()
        .expect("moria-qualify should start")
}

fn qualify_with_target_dir(arguments: &[&str], target_directory: &PathBuf) -> Output {
    Command::new(env!("CARGO_BIN_EXE_moria-qualify"))
        .args(arguments)
        .env("CARGO_TARGET_DIR", target_directory)
        .output()
        .expect("moria-qualify should start")
}

fn evidence_directory(name: &str) -> PathBuf {
    std::env::temp_dir()
        .join("qualifier-scaffold-tests")
        .join(format!("{name}-{}", std::process::id()))
}

#[test]
fn documented_scaffold_commands_write_external_evidence() {
    let shaders = qualify(&["shaders", "validate"]);
    assert!(shaders.status.success());
    assert!(String::from_utf8_lossy(&shaders.stdout).contains("PASS shaders validate"));

    let target_directory = evidence_directory("target");
    let replay = qualify_with_target_dir(
        &[
            "replay",
            "verify",
            "--fixture",
            "fixtures/replay/core-v1",
            "--runs",
            "8",
            "--evidence",
            "target/moria-evidence/replay",
        ],
        &target_directory,
    );
    assert!(!replay.status.success());
    assert!(String::from_utf8_lossy(&replay.stderr).contains("UNAVAILABLE"));
    let replay_receipt =
        fs::read_to_string(target_directory.join("moria-evidence/replay/replay-scaffold-v1.txt"))
            .expect("replay evidence should be readable");
    assert!(replay_receipt.contains("claim=replay-verification"));
    assert!(replay_receipt.contains("result=unavailable"));
    assert!(replay_receipt.contains("replay_grade=false"));

    let scenario = qualify_with_target_dir(
        &[
            "scenario",
            "public-boundary",
            "--mode",
            "candidate",
            "--evidence",
            "target/moria-evidence/dev",
        ],
        &target_directory,
    );
    assert!(!scenario.status.success());
    assert!(String::from_utf8_lossy(&scenario.stderr).contains("UNAVAILABLE"));
    let scenario_receipt = fs::read_to_string(
        target_directory.join("moria-evidence/dev/public-boundary-scaffold-v1.txt"),
    )
    .expect("scenario evidence should be readable");
    assert!(scenario_receipt.contains("claim=public-boundary-scenario"));
    assert!(scenario_receipt.contains("result=unavailable"));
    assert!(scenario_receipt.contains("scenario_grade=false"));

    fs::remove_dir_all(target_directory).expect("evidence should be removable");
}

#[test]
fn malformed_commands_fail_with_usage_exit_code() {
    let output = qualify(&["replay", "verify"]);

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("usage: moria-qualify"));
}

#[test]
fn replay_requires_an_existing_fixture() {
    let output = qualify(&[
        "replay",
        "verify",
        "--fixture",
        "fixtures/replay/missing",
        "--runs",
        "8",
        "--evidence",
        "target/qualifier-scaffold-tests/missing-fixture",
    ]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("fixture"));
}
