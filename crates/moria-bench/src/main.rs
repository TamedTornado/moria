use bevy::prelude::*;
use moria_bench::cli::{BenchmarkArgs, BenchmarkExitCode, validate_output_path};
use moria_world::MoriaWorldPlugin;

fn main() {
    let arguments = match BenchmarkArgs::parse_from(std::env::args().skip(1)) {
        Ok(arguments) => arguments,
        Err(error) => exit_argument_error(error),
    };
    if let Err(error) = validate_output_path(&arguments.output) {
        exit_argument_error(error);
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MoriaWorldPlugin)
        .run();
}

fn exit_argument_error(error: impl std::fmt::Display) -> ! {
    eprintln!("moria-bench: {error}");
    std::process::exit(BenchmarkExitCode::ArgumentError as i32);
}
