//! Process-level coverage for the scaffold qualification executable.

use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

fn qualify(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_moria-qualify"))
        .args(arguments)
        .output()
        .expect("moria-qualify should start")
}

fn evidence_directory(name: &str) -> PathBuf {
    Path::new("target")
        .join("qualifier-scaffold-tests")
        .join(format!("{name}-{}", std::process::id()))
}

#[test]
fn documented_scaffold_commands_validate_inputs_and_write_evidence() {
    let shaders = qualify(&["shaders", "validate"]);
    assert!(shaders.status.success());
    assert!(String::from_utf8_lossy(&shaders.stdout).contains("PASS shaders validate"));

    let replay_evidence = evidence_directory("replay");
    let replay = qualify(&[
        "replay",
        "verify",
        "--fixture",
        "fixtures/replay/core-v1",
        "--runs",
        "8",
        "--evidence",
        replay_evidence.to_str().expect("test path is UTF-8"),
    ]);
    assert!(replay.status.success());
    assert!(String::from_utf8_lossy(&replay.stdout).contains("PASS replay verify"));
    let replay_receipt = fs::read_to_string(replay_evidence.join("replay-scaffold-v1.txt"))
        .expect("replay evidence should be readable");
    assert!(replay_receipt.contains("claim=scaffold-fixture-repeatability"));
    assert!(replay_receipt.contains("replay_grade=false"));

    let scenario_evidence = evidence_directory("scenario");
    let scenario = qualify(&[
        "scenario",
        "public-boundary",
        "--mode",
        "candidate",
        "--evidence",
        scenario_evidence.to_str().expect("test path is UTF-8"),
    ]);
    assert!(scenario.status.success());
    assert!(String::from_utf8_lossy(&scenario.stdout).contains("PASS scenario public-boundary"));
    let scenario_receipt =
        fs::read_to_string(scenario_evidence.join("public-boundary-scaffold-v1.txt"))
            .expect("scenario evidence should be readable");
    assert!(scenario_receipt.contains("claim=public-crate-linkage"));

    fs::remove_dir_all(replay_evidence).expect("replay evidence should be removable");
    fs::remove_dir_all(scenario_evidence).expect("scenario evidence should be removable");
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
