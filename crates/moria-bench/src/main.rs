use bevy::prelude::*;
use moria_bench::capture::{
    output::{human_summary, write_report_atomic},
    schema::BenchmarkReport,
};
use moria_bench::cli::{
    exit_code_after_output, validate_output_path, BenchmarkArgs, BenchmarkExitCode,
};
use moria_world::MoriaWorldPlugin;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let arguments = match BenchmarkArgs::parse_from(std::env::args().skip(1)) {
        Ok(arguments) => arguments,
        Err(error) => exit_argument_error(error),
    };
    if let Err(error) = validate_output_path(&arguments.output) {
        exit_argument_error(error);
    }

    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(MoriaWorldPlugin);
    let _ = app.run();

    // The current runner has not captured complete scenario evidence, so preserve that fact in
    // a valid failure report instead of fabricating metrics after the app exits.
    let report = BenchmarkReport::failed_before_start(&timestamp_utc_now(), "runtime");
    let written = match write_report_atomic(&arguments.output, &report) {
        Ok(written) => {
            println!("{}", human_summary(&written));
            Some(written)
        }
        Err(error) => {
            eprintln!("moria-bench: {error}");
            None
        }
    };
    std::process::exit(exit_code_after_output(written.as_ref()) as i32);
}

fn exit_argument_error(error: impl std::fmt::Display) -> ! {
    eprintln!("moria-bench: {error}");
    std::process::exit(BenchmarkExitCode::ArgumentError as i32);
}

fn timestamp_utc_now() -> String {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let seconds = i64::try_from(elapsed.as_secs()).unwrap_or(i64::MAX);
    let days = seconds.div_euclid(86_400);
    let seconds_of_day = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_date_from_unix_days(days);
    let hour = seconds_of_day / 3_600;
    let minute = (seconds_of_day % 3_600) / 60;
    let second = seconds_of_day % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

fn civil_date_from_unix_days(days_since_epoch: i64) -> (i64, u32, u32) {
    let era_days = days_since_epoch + 719_468;
    let era = if era_days >= 0 {
        era_days
    } else {
        era_days - 146_096
    } / 146_097;
    let day_of_era = era_days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    let year = year + i64::from(month <= 2);
    (year, month as u32, day as u32)
}
