use std::path::Path;

#[test]
fn curation_package_depends_on_the_feature_gated_world_facade_only() {
    let package_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = std::fs::read_to_string(package_root.join("Cargo.toml")).unwrap();
    let source = std::fs::read_to_string(package_root.join("src/main.rs")).unwrap();

    assert!(manifest.contains("features = [\"curation\"]"));
    for forbidden in [
        "build_object_index",
        "ObjectIndexConfig",
        "generate_manifest",
        "WorldStore",
    ] {
        assert!(
            !source.contains(forbidden),
            "curation CLI must not use {forbidden}"
        );
    }
    assert!(source.contains("derive_manifest("));
    assert!(!source.contains("derive_manifest_from_bytes"));
    assert!(source.contains("validate_manifest"));
}

#[test]
fn shipped_consumers_do_not_enable_the_curation_feature() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for package in ["moria-demo", "moria-bench"] {
        let manifest = std::fs::read_to_string(
            workspace_root
                .join("crates")
                .join(package)
                .join("Cargo.toml"),
        )
        .unwrap();
        assert!(
            !manifest.contains("curation"),
            "{package} must not enable curation"
        );
    }
}
