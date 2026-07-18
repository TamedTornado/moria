use std::{fs, path::Path};

use moria_world::{
    CuratedManifest, ObjectIndexConfig, build_object_index, canonical_manifest_ron,
    generate_manifest, parameters_digest_from_bytes,
};

mod search;

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
    if !matches!(arguments.as_slice(), [command] if command == "generate" || command == "check") {
        return Err("usage: moria-curate <generate|check>".to_owned());
    }

    let manifest_path = repository_root.join(CURATED_MANIFEST_PATH);
    let region_config_path = repository_root.join(REGION_CONFIG_PATH);
    let region_config = fs::read(&region_config_path)
        .map_err(|error| format!("failed to read {}: {error}", region_config_path.display()))?;
    let ruin_stamp_path = repository_root.join(RUIN_STAMP_PATH);
    let ruin_stamp = fs::read(&ruin_stamp_path)
        .map_err(|error| format!("failed to read {}: {error}", ruin_stamp_path.display()))?;

    let generated =
        generate_manifest(&region_config, &ruin_stamp).map_err(|error| error.to_string())?;
    let canonical = canonical_manifest_ron(&generated).map_err(|error| error.to_string())?;

    match arguments.as_slice() {
        [command] if command == "generate" => fs::write(&manifest_path, canonical)
            .map_err(|error| format!("failed to write {}: {error}", manifest_path.display())),
        [command] if command == "check" => {
            check_manifest(&manifest_path, &canonical, &region_config, &ruin_stamp)
        }
        _ => unreachable!("validated command"),
    }
}

fn check_manifest(
    manifest_path: &Path,
    canonical: &str,
    region_config: &[u8],
    ruin_stamp: &[u8],
) -> Result<(), String> {
    let source = fs::read_to_string(manifest_path)
        .map_err(|error| format!("failed to read {}: {error}", manifest_path.display()))?;
    if source != canonical {
        return Err(format!(
            "{} is not canonical; run moria-curate generate",
            manifest_path.display()
        ));
    }
    let manifest: CuratedManifest = ron::de::from_str(&source)
        .map_err(|error| format!("failed to deserialize {}: {error}", manifest_path.display()))?;
    manifest
        .validate()
        .map_err(|error| format!("{}: {error}", manifest_path.display()))?;
    let index = build_object_index(&manifest.objects, &ObjectIndexConfig::default())
        .map_err(|error| format!("{}: {error}", manifest_path.display()))?;
    search::select_radius_three_stress_target(&index).ok_or_else(|| {
        format!(
            "{} has no legal radius-3 m forest stress target",
            manifest_path.display()
        )
    })?;
    if manifest.parameters_digest != parameters_digest_from_bytes(region_config, ruin_stamp) {
        return Err(format!(
            "{} parameters digest does not match {} and {}",
            manifest_path.display(),
            REGION_CONFIG_PATH,
            RUIN_STAMP_PATH
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        path::Path,
        sync::{Mutex, OnceLock},
    };

    use super::run;

    fn command_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn check_validates_the_checked_in_manifest() {
        let _lock = command_lock();
        let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

        assert_eq!(run(["check".to_owned()], &repository_root), Ok(()));
    }

    #[test]
    fn rejects_unknown_commands() {
        let _lock = command_lock();
        assert_eq!(
            run(["unknown".to_owned()], Path::new(".")),
            Err("usage: moria-curate <generate|check>".to_owned())
        );
    }

    #[test]
    fn generate_replaces_the_manifest_with_the_canonical_result() {
        let _lock = command_lock();
        let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

        assert_eq!(run(["generate".to_owned()], &repository_root), Ok(()));
        assert_eq!(run(["check".to_owned()], &repository_root), Ok(()));
    }
}
