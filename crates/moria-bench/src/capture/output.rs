use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;

use super::schema::BenchmarkReport;
use crate::cli::sibling_temp_path;
use moria_world::telemetry::ReportValidationError;

#[derive(Debug)]
pub enum OutputError {
    Validation(ReportValidationError),
    Io(std::io::Error),
}

pub fn write_report_atomic(path: &Path, report: &BenchmarkReport) -> Result<(), OutputError> {
    let json = report
        .to_canonical_json()
        .map_err(OutputError::Validation)?;
    write_json_atomically(path, &json)
}

pub fn human_summary(path: &Path, report: &BenchmarkReport) -> String {
    let outcome = if report.passed { "passed" } else { "failed" };
    format!("benchmark {outcome}; report written to {}", path.display())
}

impl std::fmt::Display for OutputError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(error) => write!(formatter, "report validation failed: {error}"),
            Self::Io(error) => write!(formatter, "report output failed: {error}"),
        }
    }
}

impl std::error::Error for OutputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(error) => Some(error),
            Self::Io(error) => Some(error),
        }
    }
}

fn write_json_atomically(path: &Path, json: &str) -> Result<(), OutputError> {
    let temporary = sibling_temp_path(path).map_err(|error| {
        OutputError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, error))
    })?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(OutputError::Io)?;
    let result = write_and_flush(file, json).and_then(|()| fs::rename(&temporary, path));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result.map_err(OutputError::Io)
}

fn write_and_flush(mut file: File, json: &str) -> std::io::Result<()> {
    file.write_all(json.as_bytes())?;
    file.flush()?;
    file.sync_all()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{write_json_atomically, write_report_atomic};
    use crate::capture::schema::BenchmarkReport;

    #[test]
    fn atomic_write_replaces_the_target_without_leaving_a_temp_file() {
        let directory = tempfile_path("replace");
        fs::create_dir_all(&directory).unwrap();
        let output = directory.join("report.json");
        fs::write(&output, "old report").unwrap();

        write_json_atomically(&output, "{\"passed\":false}").unwrap();

        assert_eq!(fs::read_to_string(&output).unwrap(), "{\"passed\":false}");
        assert!(!directory.join("report.json.tmp").exists());
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn failed_temporary_write_preserves_the_existing_report() {
        let directory = tempfile_path("preserve");
        fs::create_dir_all(&directory).unwrap();
        let output = directory.join("report.json");
        let temporary = directory.join("report.json.tmp");
        fs::write(&output, "previous report").unwrap();
        fs::write(&temporary, "another writer").unwrap();

        assert!(write_json_atomically(&output, "new report").is_err());

        assert_eq!(fs::read_to_string(&output).unwrap(), "previous report");
        assert_eq!(fs::read_to_string(&temporary).unwrap(), "another writer");
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn invalid_report_never_replaces_an_existing_report() {
        let directory = tempfile_path("invalid");
        fs::create_dir_all(&directory).unwrap();
        let output = directory.join("report.json");
        fs::write(&output, "previous report").unwrap();
        let invalid = BenchmarkReport::failed_before_start("2026-07-17T00:00:00Z", "runtime");

        assert!(write_report_atomic(&output, &invalid).is_err());
        assert_eq!(fs::read_to_string(&output).unwrap(), "previous report");
        fs::remove_dir_all(directory).unwrap();
    }

    fn tempfile_path(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("moria-bench-output-{name}-{}", std::process::id()))
    }
}
