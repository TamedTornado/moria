use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};
use serde::Deserialize;
use sha2::{Digest, Sha256};

const RUIN_STAMP_PATH: &str = "stamps/ruin_p1.ron";
const AIR_MATERIAL_ID: u8 = 0;
const CUT_STONE_MATERIAL_ID: u8 = 13;
const EXPECTED_PLACEHOLDER_SHA256: &str =
    "2739866842665f2af4bbb7f5ddbf783d0a640bde0bc4a1e98ff4a375fedc934a";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SparseVoxelStamp {
    key: String,
    size_voxels: [u16; 3],
    pivot_voxel: [i16; 3],
    palette: Vec<u8>,
    runs: Vec<StampRun>,
    tags: BTreeMap<String, VoxelCoord>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StampRun {
    start_linear: u32,
    len: u16,
    palette_index: u8,
    density: u8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct VoxelCoord {
    x: i32,
    y: i32,
    z: i32,
}

fn asset_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("assets")
        .join(RUIN_STAMP_PATH)
}

#[test]
fn ruin_stamp_placeholder_uses_the_immutable_runtime_declaration_and_schema() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::RuinStamp);

    assert_eq!(declaration.id.stable_id(), "moria.stamps.ruin_p1");
    assert_eq!(declaration.path, RUIN_STAMP_PATH);
    assert_eq!(loader.resolve_runtime_path(RUIN_STAMP_PATH), Ok(declaration));
    assert_eq!(
        loader.validation_fixture(AssetId::RuinStamp).key,
        declaration.id.stable_id()
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::RuinStamp, RuntimeAssetProfile::Development),
        AssetMissingAction::Fatal
    );

    let bytes = fs::read(asset_path()).expect("ruin stamp placeholder exists at its declared path");
    assert_eq!(hex_digest(&bytes), EXPECTED_PLACEHOLDER_SHA256);
    let stamp: SparseVoxelStamp = ron::from_str(
        std::str::from_utf8(&bytes).expect("ruin stamp placeholder is valid UTF-8"),
    )
    .expect("placeholder uses stamp schema");

    assert_eq!(stamp.key, declaration.id.stable_id());
    assert!(stamp.size_voxels.iter().all(|&edge| edge > 0));
    assert!(stamp.size_voxels.iter().all(|&edge| edge <= 16));
    for (pivot, edge) in stamp.pivot_voxel.iter().zip(stamp.size_voxels) {
        assert!(*pivot >= 0 && *pivot < i16::try_from(edge).unwrap());
    }
    assert_eq!(stamp.palette, vec![AIR_MATERIAL_ID, CUT_STONE_MATERIAL_ID]);

    let volume = stamp.size_voxels.iter().map(|&edge| u32::from(edge)).product();
    let mut previous_end = 0;
    for run in &stamp.runs {
        assert!(run.len > 0);
        assert!(run.start_linear >= previous_end, "runs are sorted and non-overlapping");
        let end = run.start_linear + u32::from(run.len);
        assert!(end <= volume, "run is inside stamp bounds");
        let material = stamp.palette[usize::from(run.palette_index)];
        assert!(matches!(material, AIR_MATERIAL_ID | CUT_STONE_MATERIAL_ID));
        if material == AIR_MATERIAL_ID {
            assert_eq!(run.density, 0, "air-carve runs have empty density");
        } else {
            assert_eq!(run.density, u8::MAX, "cut-stone runs are fully solid");
        }
        previous_end = end;
    }
    assert!(!stamp.runs.is_empty());
    assert!(stamp.runs.len() < usize::try_from(volume).unwrap());

    for tag in ["stair_bottom", "stair_top", "entrance"] {
        let coordinate = stamp.tags.get(tag).expect("required ruin tag");
        assert_in_bounds(coordinate, stamp.size_voxels);
        for quarter_turn in 0..4 {
            assert_in_bounds(
                &rotate_about_pivot(coordinate, stamp.pivot_voxel, quarter_turn),
                stamp.size_voxels,
            );
        }
    }

    let solid_voxels = expand_solid_voxels(&stamp);
    assert!(solid_voxels.contains(&[2, 0, 2]));
    assert!(solid_voxels.contains(&[2, 1, 3]));
    assert!(solid_voxels.contains(&[2, 2, 4]));
}

fn assert_in_bounds(coordinate: &VoxelCoord, size: [u16; 3]) {
    assert!(coordinate.x >= 0 && coordinate.x < i32::from(size[0]));
    assert!(coordinate.y >= 0 && coordinate.y < i32::from(size[1]));
    assert!(coordinate.z >= 0 && coordinate.z < i32::from(size[2]));
}

fn rotate_about_pivot(coordinate: &VoxelCoord, pivot: [i16; 3], quarter_turn: u8) -> VoxelCoord {
    let x = coordinate.x - i32::from(pivot[0]);
    let z = coordinate.z - i32::from(pivot[2]);
    let (x, z) = match quarter_turn {
        0 => (x, z),
        1 => (-z, x),
        2 => (-x, -z),
        3 => (z, -x),
        _ => unreachable!("quarter turns are restricted to 0 through 3"),
    };
    VoxelCoord {
        x: x + i32::from(pivot[0]),
        y: coordinate.y,
        z: z + i32::from(pivot[2]),
    }
}

fn expand_solid_voxels(stamp: &SparseVoxelStamp) -> Vec<[u16; 3]> {
    let width = u32::from(stamp.size_voxels[0]);
    let depth = u32::from(stamp.size_voxels[2]);
    stamp
        .runs
        .iter()
        .filter(|run| stamp.palette[usize::from(run.palette_index)] == CUT_STONE_MATERIAL_ID)
        .flat_map(|run| run.start_linear..run.start_linear + u32::from(run.len))
        .map(|linear| {
            let x = linear % width;
            let remainder = linear / width;
            let z = remainder % depth;
            let y = remainder / depth;
            [
                u16::try_from(x).unwrap(),
                u16::try_from(y).unwrap(),
                u16::try_from(z).unwrap(),
            ]
        })
        .collect()
}

fn hex_digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
