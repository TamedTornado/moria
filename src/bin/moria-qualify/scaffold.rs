//! Scaffold-only command parsing and result reporting.

use std::process::ExitCode;

const USAGE: &str = "usage: moria-qualify shaders validate | replay verify --fixture <path> --runs <positive-integer> --evidence <path> | scenario public-boundary --mode candidate --evidence <path>";

/// Runs the documented qualification commands that are meaningful before the
/// product contracts have been implemented.
pub(crate) fn run(arguments: impl Iterator<Item = String>) -> ExitCode {
    match parse(arguments) {
        Ok(Command::ShaderValidation) => {
            println!("PASS shaders validate: the scaffold references no WGSL modules");
            ExitCode::SUCCESS
        }
        Ok(Command::ReplayVerification) => {
            println!(
                "UNAVAILABLE replay verify: real-GPU replay qualification is not implemented by the scaffold"
            );
            ExitCode::SUCCESS
        }
        Ok(Command::PublicBoundaryScenario) => {
            println!(
                "UNAVAILABLE scenario public-boundary: candidate qualification is not implemented by the scaffold"
            );
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}\n{USAGE}");
            ExitCode::from(2)
        }
    }
}

enum Command {
    ShaderValidation,
    ReplayVerification,
    PublicBoundaryScenario,
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
    parse_options(arguments, &["--fixture", "--runs", "--evidence"])?;
    Ok(Command::ReplayVerification)
}

fn parse_scenario(arguments: &mut impl Iterator<Item = String>) -> Result<Command, String> {
    require_literal(arguments, "public-boundary")?;
    parse_options(arguments, &["--mode", "--evidence"])?;
    Ok(Command::PublicBoundaryScenario)
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
) -> Result<(), String> {
    let mut seen = vec![false; required.len()];

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
    }

    if let Some(index) = seen.iter().position(|was_seen| !was_seen) {
        return Err(format!("missing required option `{}`", required[index]));
    }
    Ok(())
}

fn require_end(arguments: &mut impl Iterator<Item = String>) -> Result<(), String> {
    match arguments.next() {
        Some(argument) => Err(format!("unexpected argument `{argument}`")),
        None => Ok(()),
    }
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
            Ok(Command::ReplayVerification)
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
            Ok(Command::PublicBoundaryScenario)
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
