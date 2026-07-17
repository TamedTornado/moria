use std::{fs, path::Path};

use moria_world::{CuratedManifest, parameters_digest_from_bytes};

const CURATED_MANIFEST_PATH: &str = "assets/config/curated_manifest.ron";
const REGION_CONFIG_PATH: &str = "assets/config/product_one_region.ron";
const RUIN_STAMP_PATH: &str = "assets/stamps/ruin_p1.ron";

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
    let source = fs::read_to_string(&manifest_path)
        .map_err(|error| format!("failed to read {}: {error}", manifest_path.display()))?;
    let manifest: CuratedManifest = ron::de::from_str(&source)
        .map_err(|error| format!("failed to deserialize {}: {error}", manifest_path.display()))?;

    manifest
        .validate()
        .map_err(|error| format!("{}: {error}", manifest_path.display()))?;

    let region_config_path = repository_root.join(REGION_CONFIG_PATH);
    let region_config = fs::read(&region_config_path)
        .map_err(|error| format!("failed to read {}: {error}", region_config_path.display()))?;
    let ruin_stamp_path = repository_root.join(RUIN_STAMP_PATH);
    let ruin_stamp = fs::read(&ruin_stamp_path)
        .map_err(|error| format!("failed to read {}: {error}", ruin_stamp_path.display()))?;
    if manifest.parameters_digest != parameters_digest_from_bytes(&region_config, &ruin_stamp) {
        return Err(format!(
            "{} parameters digest does not match {} and {}",
            manifest_path.display(),
            region_config_path.display(),
            ruin_stamp_path.display()
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::run;

    #[test]
    fn check_validates_the_checked_in_manifest() {
        let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

        assert_eq!(run(["check".to_owned()], &repository_root), Ok(()));
    }

    #[test]
    fn rejects_unknown_commands() {
        assert_eq!(
            run(["generate".to_owned()], Path::new(".")),
            Err("usage: moria-curate check".to_owned())
        );
    }
}
