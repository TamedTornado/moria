use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};

const CURATED_SEED: u64 = 0x4D4F_5249_415F_5031;
const DEFAULT_RESOLUTION: [u32; 2] = [2560, 1440];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BenchmarkScenario {
    FeasibilityMutation,
    Flythrough,
    MutationWorkloads,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BenchmarkArgs {
    pub scenario: BenchmarkScenario,
    pub output: PathBuf,
    pub resolution: [u32; 2],
    pub seed: u64,
    pub forest_proof: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CliError(String);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BenchmarkExitCode {
    Success = 0,
    RuntimeFailure = 1,
    ArgumentError = 2,
}

impl std::fmt::Display for CliError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for CliError {}

impl BenchmarkArgs {
    pub fn parse_from<I>(arguments: I) -> Result<Self, CliError>
    where
        I: IntoIterator<Item = String>,
    {
        let mut arguments = arguments.into_iter();
        let mut scenario = None;
        let mut output = None;
        let mut resolution = None;
        let mut seed = None;
        let mut forest_proof = None;

        while let Some(flag) = arguments.next() {
            let value = arguments
                .next()
                .ok_or_else(|| CliError(format!("missing value for {flag}")))?;
            match flag.as_str() {
                "--scenario" => set_once(&mut scenario, parse_scenario(&value)?, "--scenario")?,
                "--output" => set_once(&mut output, PathBuf::from(value), "--output")?,
                "--resolution" => {
                    set_once(&mut resolution, parse_resolution(&value)?, "--resolution")?;
                }
                "--seed" => {
                    let parsed = value
                        .parse()
                        .map_err(|_| CliError("invalid --seed".into()))?;
                    if parsed != CURATED_SEED {
                        return Err(CliError(
                            "--seed is not the curated Product One seed".into(),
                        ));
                    }
                    set_once(&mut seed, parsed, "--seed")?;
                }
                "--forest-proof" => {
                    set_once(&mut forest_proof, PathBuf::from(value), "--forest-proof")?;
                }
                _ => return Err(CliError(format!("unknown argument {flag}"))),
            }
        }

        let scenario = scenario.ok_or_else(|| CliError("missing --scenario".into()))?;
        let output = output.ok_or_else(|| CliError("missing --output".into()))?;
        if output
            .extension()
            .is_none_or(|extension| extension != "json")
        {
            return Err(CliError("--output must name a .json file".into()));
        }
        match scenario {
            BenchmarkScenario::FeasibilityMutation if forest_proof.is_none() => {
                return Err(CliError(
                    "feasibility-mutation requires --forest-proof".into(),
                ));
            }
            BenchmarkScenario::Flythrough | BenchmarkScenario::MutationWorkloads
                if forest_proof.is_some() =>
            {
                return Err(CliError(
                    "--forest-proof applies only to feasibility-mutation".into(),
                ));
            }
            _ => {}
        }

        Ok(Self {
            scenario,
            output,
            resolution: resolution.unwrap_or(DEFAULT_RESOLUTION),
            seed: seed.unwrap_or(CURATED_SEED),
            forest_proof,
        })
    }
}

pub fn validate_output_path(path: &Path) -> Result<(), CliError> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    if !parent.is_dir() {
        return Err(CliError("output directory does not exist".into()));
    }
    if path.is_dir() {
        return Err(CliError("--output must name a file".into()));
    }
    let temporary = sibling_temp_path(path)?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|_| CliError("output path is not writable".into()))?;
    file.sync_all()
        .map_err(|_| CliError("output path is not writable".into()))?;
    fs::remove_file(temporary).map_err(|_| CliError("output path is not writable".into()))
}

#[must_use]
pub const fn exit_code_after_output(
    report_passed: bool,
    output_succeeded: bool,
) -> BenchmarkExitCode {
    if !output_succeeded || !report_passed {
        BenchmarkExitCode::RuntimeFailure
    } else {
        BenchmarkExitCode::Success
    }
}

fn parse_scenario(value: &str) -> Result<BenchmarkScenario, CliError> {
    match value {
        "feasibility-mutation" => Ok(BenchmarkScenario::FeasibilityMutation),
        "flythrough" => Ok(BenchmarkScenario::Flythrough),
        "mutation-workloads" => Ok(BenchmarkScenario::MutationWorkloads),
        _ => Err(CliError("unknown --scenario".into())),
    }
}

fn parse_resolution(value: &str) -> Result<[u32; 2], CliError> {
    let Some((width, height)) = value.split_once('x') else {
        return Err(CliError("invalid --resolution".into()));
    };
    let width = width
        .parse()
        .map_err(|_| CliError("invalid --resolution".into()))?;
    let height = height
        .parse()
        .map_err(|_| CliError("invalid --resolution".into()))?;
    if width == 0 || height == 0 {
        return Err(CliError("invalid --resolution".into()));
    }
    Ok([width, height])
}

fn set_once<T>(target: &mut Option<T>, value: T, flag: &str) -> Result<(), CliError> {
    if target.replace(value).is_some() {
        return Err(CliError(format!("duplicate {flag}")));
    }
    Ok(())
}

pub(crate) fn sibling_temp_path(path: &Path) -> Result<PathBuf, CliError> {
    let file_name = path
        .file_name()
        .ok_or_else(|| CliError("--output must name a file".into()))?;
    Ok(path.with_file_name(format!("{}.tmp", file_name.to_string_lossy())))
}

#[cfg(test)]
mod tests {
    use super::{BenchmarkArgs, BenchmarkExitCode, BenchmarkScenario, exit_code_after_output};

    #[test]
    fn parses_a_flythrough_with_default_resolution_and_curated_seed() {
        let args = BenchmarkArgs::parse_from([
            "--scenario".into(),
            "flythrough".into(),
            "--output".into(),
            "target/bench/flythrough.json".into(),
        ])
        .unwrap();

        assert_eq!(args.scenario, BenchmarkScenario::Flythrough);
        assert_eq!(args.resolution, [2560, 1440]);
        assert_eq!(args.seed, 0x4D4F_5249_415F_5031);
    }

    #[test]
    fn rejects_unknown_missing_noncurated_and_invalid_arguments() {
        for arguments in [
            vec!["--scenario", "unknown", "--output", "out.json"],
            vec!["--scenario", "flythrough"],
            vec![
                "--scenario",
                "flythrough",
                "--output",
                "out.json",
                "--seed",
                "7",
            ],
            vec![
                "--scenario",
                "flythrough",
                "--output",
                "out.json",
                "--resolution",
                "0x1080",
            ],
            vec![
                "--scenario",
                "flythrough",
                "--output",
                "out.json",
                "--unexpected",
            ],
        ] {
            assert!(BenchmarkArgs::parse_from(arguments.into_iter().map(String::from)).is_err());
        }
    }

    #[test]
    fn requires_a_forest_proof_only_for_feasibility_mutation() {
        assert!(
            BenchmarkArgs::parse_from([
                "--scenario".into(),
                "feasibility-mutation".into(),
                "--output".into(),
                "out.json".into(),
            ])
            .is_err()
        );
    }

    #[test]
    fn maps_only_a_written_passing_report_to_success() {
        assert_eq!(
            exit_code_after_output(true, true),
            BenchmarkExitCode::Success
        );
        assert_eq!(
            exit_code_after_output(false, true),
            BenchmarkExitCode::RuntimeFailure
        );
        assert_eq!(
            exit_code_after_output(true, false),
            BenchmarkExitCode::RuntimeFailure
        );
    }
}
