//! Scaffold-only command parsing and result reporting.

use std::{
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

const USAGE: &str = "usage: moria-qualify shaders validate | replay verify --fixture <path> --runs <positive-integer> --evidence <path> | scenario public-boundary --mode candidate --evidence <path>";
const REPLAY_FIXTURE_FILE: &str = "fixture.txt";
const REPLAY_FIXTURE_VERSION: &str = "moria-replay-scaffold-v1";

/// Runs the documented qualification commands available in the scaffold.
pub(crate) fn run(arguments: impl Iterator<Item = String>) -> ExitCode {
    match parse(arguments) {
        Ok(Command::ShaderValidation) => {
            println!("PASS shaders validate: the scaffold references no WGSL modules");
            ExitCode::SUCCESS
        }
        Ok(Command::ReplayVerification {
            fixture,
            runs,
            evidence,
        }) => report("replay verify", replay_verify(&fixture, runs, &evidence)),
        Ok(Command::PublicBoundaryScenario { evidence }) => report(
            "scenario public-boundary",
            public_boundary_scenario(&evidence),
        ),
        Err(message) => {
            eprintln!("{message}\n{USAGE}");
            ExitCode::from(2)
        }
    }
}

enum Command {
    ShaderValidation,
    ReplayVerification {
        fixture: PathBuf,
        runs: u32,
        evidence: PathBuf,
    },
    PublicBoundaryScenario {
        evidence: PathBuf,
    },
}

fn report(command: &str, result: Result<String, String>) -> ExitCode {
    match result {
        Ok(message) => {
            println!("PASS {command}: {message}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("FAIL {command}: {message}");
            ExitCode::FAILURE
        }
    }
}

fn parse(mut arguments: impl Iterator<Item = String>) -> Result<Command, String> {
    let Some(command) = arguments.next() else {
        return Err("missing command".to_owned());
    };

    match command.as_str() {
        "shaders" => parse_shaders(&mut arguments),
        "replay" => parse_replay(&mut arguments),
        "scenario" => parse_scenario(&mut arguments),
        _ => Err(format!("unknown command `{command}`")),
    }
}

fn parse_shaders(arguments: &mut impl Iterator<Item = String>) -> Result<Command, String> {
    require_literal(arguments, "validate")?;
    require_end(arguments)?;
    Ok(Command::ShaderValidation)
}

fn parse_replay(arguments: &mut impl Iterator<Item = String>) -> Result<Command, String> {
    require_literal(arguments, "verify")?;
    let options = parse_options(arguments, &["--fixture", "--runs", "--evidence"])?;
    let runs = options[1]
        .parse::<u32>()
        .expect("positive --runs values are validated while parsing");
    Ok(Command::ReplayVerification {
        fixture: PathBuf::from(&options[0]),
        runs,
        evidence: PathBuf::from(&options[2]),
    })
}

fn parse_scenario(arguments: &mut impl Iterator<Item = String>) -> Result<Command, String> {
    require_literal(arguments, "public-boundary")?;
    let options = parse_options(arguments, &["--mode", "--evidence"])?;
    Ok(Command::PublicBoundaryScenario {
        evidence: PathBuf::from(&options[1]),
    })
}

fn require_literal(
    arguments: &mut impl Iterator<Item = String>,
    expected: &str,
) -> Result<(), String> {
    match arguments.next().as_deref() {
        Some(actual) if actual == expected => Ok(()),
        Some(actual) => Err(format!("expected `{expected}`, got `{actual}`")),
        None => Err(format!("missing `{expected}`")),
    }
}

fn parse_options(
    arguments: &mut impl Iterator<Item = String>,
    required: &[&str],
) -> Result<Vec<String>, String> {
    let mut seen = vec![false; required.len()];
    let mut values = vec![String::new(); required.len()];

    while let Some(option) = arguments.next() {
        let Some(index) = required
            .iter()
            .position(|required_option| *required_option == option)
        else {
            return Err(format!("unexpected option `{option}`"));
        };
        if seen[index] {
            return Err(format!("duplicate option `{option}`"));
        }

        let Some(value) = arguments.next() else {
            return Err(format!("missing value for `{option}`"));
        };
        if value.is_empty() {
            return Err(format!("empty value for `{option}`"));
        }
        if option == "--runs" && value.parse::<u32>().ok().filter(|runs| *runs > 0).is_none() {
            return Err("`--runs` must be a positive integer".to_owned());
        }
        if option == "--mode" && value != "candidate" {
            return Err("`--mode` must be `candidate`".to_owned());
        }

        seen[index] = true;
        values[index] = value;
    }

    if let Some(index) = seen.iter().position(|was_seen| !was_seen) {
        return Err(format!("missing required option `{}`", required[index]));
    }
    Ok(values)
}

fn require_end(arguments: &mut impl Iterator<Item = String>) -> Result<(), String> {
    match arguments.next() {
        Some(argument) => Err(format!("unexpected argument `{argument}`")),
        None => Ok(()),
    }
}

fn replay_verify(fixture: &Path, runs: u32, evidence: &Path) -> Result<String, String> {
    let fixture_path = fixture.join(REPLAY_FIXTURE_FILE);
    let fixture_bytes = fs::read(&fixture_path)
        .map_err(|error| format!("cannot read fixture `{}`: {error}", fixture_path.display()))?;
    let fixture_text = std::str::from_utf8(&fixture_bytes)
        .map_err(|_| format!("fixture `{}` is not UTF-8", fixture_path.display()))?;
    if fixture_text.lines().next() != Some(REPLAY_FIXTURE_VERSION) {
        return Err(format!(
            "fixture `{}` does not declare `{REPLAY_FIXTURE_VERSION}`",
            fixture_path.display()
        ));
    }

    let expected_digest = fixture_digest(&fixture_bytes);
    for _ in 1..runs {
        if fixture_digest(&fixture_bytes) != expected_digest {
            return Err("fixture digest diverged between runs".to_owned());
        }
    }

    write_evidence(
        evidence,
        "replay-scaffold-v1.txt",
        &format!(
            "schema=moria-scaffold-evidence-v1\nclaim=scaffold-fixture-repeatability\nreplay_grade=false\ngpu_execution=not-claimed\nfixture={}\nruns={runs}\ndigest={expected_digest:016x}\n",
            fixture_path.display()
        ),
    )?;

    Ok("scaffold fixture repeated consistently; no replay-grade GPU claim is made".to_owned())
}

fn public_boundary_scenario(evidence: &Path) -> Result<String, String> {
    write_evidence(
        evidence,
        "public-boundary-scaffold-v1.txt",
        "schema=moria-scaffold-evidence-v1\nclaim=public-crate-linkage\nmode=candidate\npublic_surface=crate-root\n",
    )?;
    Ok("candidate crate linkage validated through the public facade".to_owned())
}

fn write_evidence(directory: &Path, name: &str, contents: &str) -> Result<(), String> {
    fs::create_dir_all(directory).map_err(|error| {
        format!(
            "cannot create evidence directory `{}`: {error}",
            directory.display()
        )
    })?;
    let path = directory.join(name);
    fs::write(&path, contents)
        .map_err(|error| format!("cannot write evidence `{}`: {error}", path.display()))
}

fn fixture_digest(bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .fold(0xcbf2_9ce4_8422_2325_u64, |digest, byte| {
            (digest ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
        })
}

#[cfg(test)]
mod tests {
    use super::{Command, parse};

    fn parse_arguments(arguments: &[&str]) -> Result<Command, String> {
        parse(arguments.iter().map(|argument| (*argument).to_owned()))
    }

    #[test]
    fn parses_every_scaffold_command() {
        assert!(matches!(
            parse_arguments(&["shaders", "validate"]),
            Ok(Command::ShaderValidation)
        ));
        assert!(matches!(
            parse_arguments(&[
                "replay",
                "verify",
                "--fixture",
                "fixtures/replay/core-v1",
                "--runs",
                "8",
                "--evidence",
                "target/evidence",
            ]),
            Ok(Command::ReplayVerification { .. })
        ));
        assert!(matches!(
            parse_arguments(&[
                "scenario",
                "public-boundary",
                "--mode",
                "candidate",
                "--evidence",
                "target/evidence",
            ]),
            Ok(Command::PublicBoundaryScenario { .. })
        ));
    }

    #[test]
    fn rejects_missing_or_ambiguous_options() {
        assert!(parse_arguments(&["replay", "verify"]).is_err());
        assert!(
            parse_arguments(&[
                "scenario",
                "public-boundary",
                "--mode",
                "candidate",
                "--mode",
                "candidate",
                "--evidence",
                "target/evidence",
            ])
            .is_err()
        );
    }
}
