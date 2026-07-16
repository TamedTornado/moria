use std::{collections::BTreeMap, fs, path::PathBuf};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};
use serde::Deserialize;
use sha2::{Digest, Sha256};

const RUIN_STAMP_PATH: &str = "stamps/ruin_p1.ron";
const RUIN_STAMP_SHA256: &str = "8d8d5ef2a40197b0329a813f51e921da3461ee4203dba0afd1abf8dda4b514cf";
const CUT_STONE_MATERIAL_ID: u8 = 13;
const AIR_MATERIAL_ID: u8 = 0;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SparseVoxelStamp {
    key: String,
    size_voxels: [u16; 3],
    pivot_voxel: [i16; 3],
    palette: Vec<u8>,
    runs: Vec<StampRun>,
    tags: BTreeMap<String, [u16; 3]>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StampRun {
    start_linear: u32,
    len: u16,
    palette_index: u8,
    density: u8,
}

#[test]
fn ruin_stamp_placeholder_uses_the_declared_required_runtime_path() {
    let loader = AssetLoader::new();
    let declaration = loader
        .resolve_runtime_path(RUIN_STAMP_PATH)
        .expect("the ruin stamp path must remain declared");

    assert_eq!(declaration.id, AssetId::RuinStamp);
    assert_eq!(declaration.id.stable_id(), "moria.stamps.ruin_p1");
    assert_eq!(declaration.path, RUIN_STAMP_PATH);
    assert_eq!(
        loader.validation_fixture(AssetId::RuinStamp).key,
        "moria.stamps.ruin_p1"
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::RuinStamp, RuntimeAssetProfile::Development),
        AssetMissingAction::Fatal,
        "an authoritative stamp must never use a development fallback"
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::RuinStamp, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );
}

#[test]
fn ruin_stamp_placeholder_is_a_digest_identified_sparse_cut_stone_staircase() {
    let bytes =
        fs::read(asset_path()).expect("the ruin stamp placeholder must exist at its final path");
    let actual_digest = format!("{:x}", Sha256::digest(&bytes));
    assert_eq!(actual_digest, RUIN_STAMP_SHA256);

    let stamp: SparseVoxelStamp = ron::de::from_bytes(&bytes)
        .expect("the ruin stamp placeholder must use the documented RON schema");
    assert_eq!(stamp.key, "moria.stamps.ruin_p1");
    assert_eq!(stamp.size_voxels, [8, 8, 8]);
    assert_eq!(stamp.pivot_voxel, [4, 0, 4]);
    assert_eq!(stamp.palette, vec![CUT_STONE_MATERIAL_ID, AIR_MATERIAL_ID]);
    assert_eq!(stamp.tags.len(), 3);

    for required_tag in ["stair_bottom", "stair_top", "entrance"] {
        let coordinate = stamp.tags.get(required_tag).unwrap_or_else(|| {
            panic!("the stamp must include the {required_tag} tag");
        });
        assert!(coordinate_in_bounds(*coordinate, stamp.size_voxels));
        assert_quarter_turns_remain_integral(*coordinate, stamp.size_voxels);
    }

    assert!(
        stamp.runs.len() < stamp.size_voxels.iter().product::<u16>() as usize,
        "the stamp must stay sparse"
    );
    let mut previous_end = 0;
    for run in &stamp.runs {
        assert!(run.len > 0);
        assert!(
            run.start_linear >= previous_end,
            "runs must be sorted and non-overlapping"
        );
        let end = run.start_linear + u32::from(run.len);
        assert!(end <= u32::from(stamp.size_voxels.iter().product::<u16>()));
        let material = *stamp
            .palette
            .get(usize::from(run.palette_index))
            .expect("run palette indices must be valid");
        assert!(matches!(material, CUT_STONE_MATERIAL_ID | AIR_MATERIAL_ID));
        assert_eq!(
            run.density,
            if material == CUT_STONE_MATERIAL_ID {
                255
            } else {
                0
            },
            "cut stone is solid and air-carves are empty"
        );
        previous_end = end;
    }

    assert!(
        stamp
            .runs
            .iter()
            .any(|run| { stamp.palette[usize::from(run.palette_index)] == CUT_STONE_MATERIAL_ID })
    );
    assert!(
        stamp
            .runs
            .iter()
            .any(|run| { stamp.palette[usize::from(run.palette_index)] == AIR_MATERIAL_ID })
    );
}

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets")
        .join(RUIN_STAMP_PATH)
}

fn coordinate_in_bounds(coordinate: [u16; 3], size: [u16; 3]) -> bool {
    coordinate
        .into_iter()
        .zip(size)
        .all(|(value, limit)| value < limit)
}

fn assert_quarter_turns_remain_integral(coordinate: [u16; 3], size: [u16; 3]) {
    let [x, y, z] = coordinate;
    let [width, height, depth] = size;
    assert!(y < height);

    let rotations = [
        (x, z),
        (z, width - 1 - x),
        (width - 1 - x, depth - 1 - z),
        (depth - 1 - z, x),
    ];
    assert!(
        rotations
            .iter()
            .all(|&(rotated_x, rotated_z)| { rotated_x < width && rotated_z < depth })
    );
}
