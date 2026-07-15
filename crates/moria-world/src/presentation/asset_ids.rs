//! The fixed runtime asset inventory from `docs/tdd/assets.md`.
//!
//! Paths are relative to Bevy's repository-root `assets/` directory. New asset
//! content must use one of these declarations rather than extending this table.

/// Number of runtime assets in the Product One inventory.
pub const ASSET_COUNT: usize = 30;

/// Immutable identifier for one declared runtime asset.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AssetId {
    ProductOneRegion,
    CuratedManifest,
    PresentationConfig,
    InputConfig,
    Materials,
    TerrainAlbedo,
    TerrainNormal,
    TerrainOrm,
    WaterNormal,
    RuinStamp,
    BirchNear,
    BirchMid,
    BirchFar,
    PineNear,
    PineMid,
    PineFar,
    BushNear,
    BushFar,
    GrassCluster,
    TreeHorizonCards,
    Boulder,
    Stump,
    Rock,
    Explorer,
    TerrainShader,
    WaterShader,
    VegetationShader,
    RawVoxelShader,
    AssetLicenses,
    AssetBudgets,
}

impl AssetId {
    /// Every immutable ID, in declaration-table order.
    pub const ALL: [Self; ASSET_COUNT] = [
        Self::ProductOneRegion,
        Self::CuratedManifest,
        Self::PresentationConfig,
        Self::InputConfig,
        Self::Materials,
        Self::TerrainAlbedo,
        Self::TerrainNormal,
        Self::TerrainOrm,
        Self::WaterNormal,
        Self::RuinStamp,
        Self::BirchNear,
        Self::BirchMid,
        Self::BirchFar,
        Self::PineNear,
        Self::PineMid,
        Self::PineFar,
        Self::BushNear,
        Self::BushFar,
        Self::GrassCluster,
        Self::TreeHorizonCards,
        Self::Boulder,
        Self::Stump,
        Self::Rock,
        Self::Explorer,
        Self::TerrainShader,
        Self::WaterShader,
        Self::VegetationShader,
        Self::RawVoxelShader,
        Self::AssetLicenses,
        Self::AssetBudgets,
    ];

    /// Stable, registry-facing text identity.
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::ProductOneRegion => "moria.config.product_one_region",
            Self::CuratedManifest => "moria.config.curated_manifest",
            Self::PresentationConfig => "moria.config.presentation",
            Self::InputConfig => "moria.config.input",
            Self::Materials => "moria.materials.registry",
            Self::TerrainAlbedo => "moria.materials.terrain_albedo",
            Self::TerrainNormal => "moria.materials.terrain_normal",
            Self::TerrainOrm => "moria.materials.terrain_orm",
            Self::WaterNormal => "moria.materials.water_normal",
            Self::RuinStamp => "moria.stamps.ruin_p1",
            Self::BirchNear => "moria.vegetation.birch_near",
            Self::BirchMid => "moria.vegetation.birch_mid",
            Self::BirchFar => "moria.vegetation.birch_far",
            Self::PineNear => "moria.vegetation.pine_near",
            Self::PineMid => "moria.vegetation.pine_mid",
            Self::PineFar => "moria.vegetation.pine_far",
            Self::BushNear => "moria.vegetation.bush_near",
            Self::BushFar => "moria.vegetation.bush_far",
            Self::GrassCluster => "moria.vegetation.grass_cluster",
            Self::TreeHorizonCards => "moria.vegetation.tree_horizon_cards",
            Self::Boulder => "moria.props.boulder",
            Self::Stump => "moria.props.stump",
            Self::Rock => "moria.props.rock",
            Self::Explorer => "moria.player.explorer",
            Self::TerrainShader => "moria.shaders.terrain",
            Self::WaterShader => "moria.shaders.water",
            Self::VegetationShader => "moria.shaders.vegetation",
            Self::RawVoxelShader => "moria.shaders.raw_voxel",
            Self::AssetLicenses => "moria.manifests.asset_licenses",
            Self::AssetBudgets => "moria.manifests.asset_budgets",
        }
    }
}

/// Missing-asset behavior declared before any individual asset is scaffolded.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AssetLoadPolicy {
    /// The asset is required in every runtime profile.
    Required,
    /// Development may substitute a declared fallback and must emit a warning.
    DevelopmentFallback,
    /// Loading failure must terminate release/benchmark validation.
    ReleaseFatal,
}

/// A per-asset hook owned by the corresponding isolated validation fixture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssetValidationFixture {
    pub asset_id: AssetId,
    pub key: &'static str,
}

/// Fixed identity, location, loading policy, and test hook for one asset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssetDeclaration {
    pub id: AssetId,
    pub path: &'static str,
    pub load_policy: AssetLoadPolicy,
    pub validation_fixture: AssetValidationFixture,
}

const fn declaration(
    id: AssetId,
    path: &'static str,
    load_policy: AssetLoadPolicy,
) -> AssetDeclaration {
    AssetDeclaration {
        id,
        path,
        load_policy,
        validation_fixture: AssetValidationFixture {
            asset_id: id,
            key: id.stable_id(),
        },
    }
}

/// Complete, immutable Product One runtime inventory.
pub const ASSET_DECLARATIONS: [AssetDeclaration; ASSET_COUNT] = [
    declaration(
        AssetId::ProductOneRegion,
        "config/product_one_region.ron",
        AssetLoadPolicy::Required,
    ),
    declaration(
        AssetId::CuratedManifest,
        "config/curated_manifest.ron",
        AssetLoadPolicy::Required,
    ),
    declaration(
        AssetId::PresentationConfig,
        "config/presentation.ron",
        AssetLoadPolicy::Required,
    ),
    declaration(
        AssetId::InputConfig,
        "config/input.ron",
        AssetLoadPolicy::Required,
    ),
    declaration(
        AssetId::Materials,
        "materials/materials.ron",
        AssetLoadPolicy::Required,
    ),
    declaration(
        AssetId::TerrainAlbedo,
        "materials/terrain_albedo.ktx2",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::TerrainNormal,
        "materials/terrain_normal.ktx2",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::TerrainOrm,
        "materials/terrain_orm.ktx2",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::WaterNormal,
        "materials/water_normal.ktx2",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::RuinStamp,
        "stamps/ruin_p1.ron",
        AssetLoadPolicy::Required,
    ),
    declaration(
        AssetId::BirchNear,
        "vegetation/birch_near.glb",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::BirchMid,
        "vegetation/birch_mid.glb",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::BirchFar,
        "vegetation/birch_far.glb",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::PineNear,
        "vegetation/pine_near.glb",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::PineMid,
        "vegetation/pine_mid.glb",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::PineFar,
        "vegetation/pine_far.glb",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::BushNear,
        "vegetation/bush_near.glb",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::BushFar,
        "vegetation/bush_far.glb",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::GrassCluster,
        "vegetation/grass_cluster.glb",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::TreeHorizonCards,
        "vegetation/tree_horizon_cards.ktx2",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::Boulder,
        "props/boulder.glb",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::Stump,
        "props/stump.glb",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::Rock,
        "props/rock.glb",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::Explorer,
        "player/explorer.glb",
        AssetLoadPolicy::DevelopmentFallback,
    ),
    declaration(
        AssetId::TerrainShader,
        "shaders/terrain.wgsl",
        AssetLoadPolicy::ReleaseFatal,
    ),
    declaration(
        AssetId::WaterShader,
        "shaders/water.wgsl",
        AssetLoadPolicy::ReleaseFatal,
    ),
    declaration(
        AssetId::VegetationShader,
        "shaders/vegetation.wgsl",
        AssetLoadPolicy::ReleaseFatal,
    ),
    declaration(
        AssetId::RawVoxelShader,
        "shaders/raw_voxel.wgsl",
        AssetLoadPolicy::ReleaseFatal,
    ),
    declaration(
        AssetId::AssetLicenses,
        "manifests/asset_licenses.ron",
        AssetLoadPolicy::Required,
    ),
    declaration(
        AssetId::AssetBudgets,
        "manifests/asset_budgets.ron",
        AssetLoadPolicy::Required,
    ),
];
