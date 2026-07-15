use std::path::Path;

const CURATED_MANIFEST_PATH: &str = "assets/config/curated_manifest.ron";

fn main() {
    if let Err(error) = run(std::env::args().skip(1), Path::new(".")) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(arguments: impl IntoIterator<Item = String>, repository_root: &Path) -> Result<(), String> {
    let arguments = arguments.into_iter().collect::<Vec<_>>();

    if arguments.as_slice() != ["check"] {
        return Err("usage: moria-curate check".to_owned());
    }

    let manifest_path = repository_root.join(CURATED_MANIFEST_PATH);
    if manifest_path.exists() {
        return Err(format!(
            "{} is present, but canonical manifest verification is not implemented yet",
            manifest_path.display()
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::run;

    #[test]
    fn check_succeeds_before_a_manifest_is_scaffolded() {
        let missing_repository_root = Path::new("target/moria-curate-test-missing-root");

        assert_eq!(run(["check".to_owned()], missing_repository_root), Ok(()));
    }

    #[test]
    fn rejects_unknown_commands() {
        assert_eq!(
            run(["generate".to_owned()], Path::new(".")),
            Err("usage: moria-curate check".to_owned())
        );
    }
}
