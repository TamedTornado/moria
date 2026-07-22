use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use super::schema::BenchmarkReport;
use crate::cli::sibling_temp_path;
use moria_world::telemetry::ReportValidationError;

#[derive(Debug)]
pub enum OutputError {
    Validation(ReportValidationError),
    Io(std::io::Error),
}

/// Evidence that a validated benchmark report was written successfully.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WrittenReport {
    path: PathBuf,
    passed: bool,
}

impl WrittenReport {
    #[must_use]
    pub const fn passed(&self) -> bool {
        self.passed
    }
}

pub fn write_report_atomic(
    path: &Path,
    report: &BenchmarkReport,
) -> Result<WrittenReport, OutputError> {
    let json = report
        .to_canonical_json()
        .map_err(OutputError::Validation)?;
    write_json_atomically(path, &json)?;
    Ok(WrittenReport {
        path: path.to_path_buf(),
        passed: report.passed,
    })
}

pub fn human_summary(written: &WrittenReport) -> String {
    let outcome = if written.passed { "passed" } else { "failed" };
    format!(
        "benchmark {outcome}; report written to {}",
        written.path.display()
    )
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
    write_json_atomically_with(path, json, write_and_flush, |from, to| fs::rename(from, to))
}

fn write_json_atomically_with<W, R>(
    path: &Path,
    json: &str,
    write: W,
    rename: R,
) -> Result<(), OutputError>
where
    W: FnOnce(File, &str) -> std::io::Result<()>,
    R: FnOnce(&Path, &Path) -> std::io::Result<()>,
{
    let temporary = sibling_temp_path(path).map_err(|error| {
        OutputError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, error))
    })?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(OutputError::Io)?;
    let result = write(file, json).and_then(|()| rename(&temporary, path));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result.map_err(OutputError::Io)
}

fn write_and_flush(file: File, json: &str) -> std::io::Result<()> {
    write_and_flush_with(
        file,
        json,
        |file, json| file.write_all(json.as_bytes()),
        |file| {
            file.flush()?;
            file.sync_all()
        },
    )
}

fn write_and_flush_with<W, F>(mut file: File, json: &str, write: W, flush: F) -> std::io::Result<()>
where
    W: FnOnce(&mut File, &str) -> std::io::Result<()>,
    F: FnOnce(&mut File) -> std::io::Result<()>,
{
    write(&mut file, json)?;
    flush(&mut file)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::{self, Write};

    use super::{
        write_and_flush_with, write_json_atomically, write_json_atomically_with,
        write_report_atomic,
    };
    use crate::capture::schema::BenchmarkReport;
    use crate::cli::{BenchmarkExitCode, exit_code_after_output};

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
    fn early_runtime_failure_replaces_an_existing_report_with_complete_null_json() {
        let directory = tempfile_path("invalid");
        fs::create_dir_all(&directory).unwrap();
        let output = directory.join("report.json");
        fs::write(&output, "previous report").unwrap();
        let report = BenchmarkReport::failed_before_start("2026-07-17T00:00:00Z", "runtime");

        write_report_atomic(&output, &report).unwrap();
        assert!(
            !BenchmarkReport::from_json(&fs::read_to_string(&output).unwrap())
                .unwrap()
                .passed
        );
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn only_a_validated_written_report_can_determine_the_exit_code() {
        let directory = tempfile_path("exit-code");
        fs::create_dir_all(&directory).unwrap();
        let output = directory.join("report.json");
        let report = valid_passing_report();

        let written = write_report_atomic(&output, &report).unwrap();

        assert_eq!(
            exit_code_after_output(Some(&written)),
            BenchmarkExitCode::Success
        );

        let mut fabricated = report;
        fabricated.resolution = None;
        let fabricated_written = write_report_atomic(&output, &fabricated);
        assert!(fabricated_written.is_err());
        assert_eq!(
            exit_code_after_output(fabricated_written.as_ref().ok()),
            BenchmarkExitCode::RuntimeFailure
        );
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn write_failure_preserves_the_existing_report_and_removes_the_temp_file() {
        assert_atomic_failure_preserves_target("write", |file, json| {
            write_and_flush_with(
                file,
                json,
                |_, _| Err(io::Error::other("write failed")),
                |file| file.flush(),
            )
        });
    }

    #[test]
    fn flush_failure_preserves_the_existing_report_and_removes_the_temp_file() {
        assert_atomic_failure_preserves_target("flush", |file, json| {
            write_and_flush_with(
                file,
                json,
                |file, json| file.write_all(json.as_bytes()),
                |_| Err(io::Error::other("flush failed")),
            )
        });
    }

    #[test]
    fn rename_failure_preserves_the_existing_report_and_removes_the_temp_file() {
        let directory = tempfile_path("rename");
        fs::create_dir_all(&directory).unwrap();
        let output = directory.join("report.json");
        fs::write(&output, "previous report").unwrap();

        assert!(
            write_json_atomically_with(&output, "new report", super::write_and_flush, |_, _| Err(
                io::Error::other("rename failed")
            ),)
            .is_err()
        );

        assert_eq!(fs::read_to_string(&output).unwrap(), "previous report");
        assert!(!directory.join("report.json.tmp").exists());
        fs::remove_dir_all(directory).unwrap();
    }

    fn assert_atomic_failure_preserves_target(
        name: &str,
        write_and_flush: impl FnOnce(std::fs::File, &str) -> io::Result<()>,
    ) {
        let directory = tempfile_path(name);
        fs::create_dir_all(&directory).unwrap();
        let output = directory.join("report.json");
        fs::write(&output, "previous report").unwrap();

        assert!(
            write_json_atomically_with(&output, "new report", write_and_flush, |from, to| {
                fs::rename(from, to)
            },)
            .is_err()
        );

        assert_eq!(fs::read_to_string(&output).unwrap(), "previous report");
        assert!(!directory.join("report.json.tmp").exists());
        fs::remove_dir_all(directory).unwrap();
    }

    fn tempfile_path(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("moria-bench-output-{name}-{}", std::process::id()))
    }

    fn valid_passing_report() -> BenchmarkReport {
        BenchmarkReport::from_json(
            r#"{
                "schema":"moria-product-one-benchmark",
                "timestamp_utc":"2026-07-17T00:00:00Z",
                "scenario":"flythrough",
                "passed":true,
                "failure_reasons":[],
                "baseline_status":"provisional",
                "build":{"cargo_profile":"release","git_commit":"0123456789abcdef0123456789abcdef01234567","rustc_version":"rustc test"},
                "world":{"seed":5570761738663448625,"parameters_digest":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"bounds":{"min":{"x":0,"y":0,"z":0},"max_exclusive":{"x":1,"y":1,"z":1}}},
                "assets":{"manifest_sha256":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","fallbacks":[],"warnings":[]},
                "machine":{"profile_id_sha256":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","os_name":"test","os_version":"1","architecture":"test","cpu_model":"test","logical_cores":1,"total_physical_memory_bytes":1,"gpu_adapter_name":"test","gpu_vendor":1,"gpu_device":1,"gpu_device_class":"test","wgpu_backend":"test","driver":null,"driver_metadata_available":false,"memory_architecture":"test","acceptance_label":"test"},
                "resolution":[2560,1440],
                "cold_start_ms":0.0,
                "frame_rate":{"sample_count":1,"measured_seconds":1.0,"arithmetic_fps":60.0,"one_percent_low_fps":60.0},
                "frame_time_ms":{"min":1.0,"p50":1.0,"p95":1.0,"p99":1.0,"max":1.0},
                "graphics_memory":{"application_ledger":{"peak_bytes":0,"end_bytes":0,"categories":{},"untracked_driver_overhead":true},"resident_measurement":null,"product_target_proven":false,"estimate_substitution_approval_id":"test-approval"},
                "mutation_latency":null,
                "save":{"attempted":false,"completed":false,"size_bytes":0,"changed_voxels":0,"changed_bricks":0,"round_trip":null},
                "coverage":{"route_tags_visited":[],"active_bands_entered":[],"edited_material_counts":{},"final_changed_spheres":0,"final_changed_region_cells":0,"workload_minimum_met":true},
                "streaming":{"peak_active_counts":{"bricks":0,"meshes":0,"objects":0},"peak_queue_depths":{"extraction":0,"installation":0,"render":0},"first_steady_derived_bytes":0,"return_steady_derived_bytes":0,"monotonic_growth_check_passed":true,"object_index":{"validation_ms":0.0,"build_ms":0.0,"retained_bytes":0,"retained_byte_categories":{},"placement_records":0,"dependency_grid_entries":0,"sample_grid_entries":0,"max_dependency_cell_entries":0,"max_sample_cell_entries":0,"max_horizon_tree_members_per_cell":0,"max_edit_candidates":0,"max_edit_affected_objects":0,"max_dependency_bricks":0,"dependency_coordinate_allocation_bytes":0}}
            }"#,
        )
        .unwrap()
    }
}
