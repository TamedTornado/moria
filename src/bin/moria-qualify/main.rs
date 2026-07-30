//! External-consumer qualification executable.

use moria as _;

mod scaffold;

fn main() -> std::process::ExitCode {
    scaffold::run(std::env::args().skip(1))
}
