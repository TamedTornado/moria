//! Immutable runtime asset declarations and their loading contract.

mod asset_ids;
mod assets;
mod basis_ktx2;
mod validation;

pub use asset_ids::{
    ASSET_COUNT, ASSET_DECLARATIONS, AssetDeclaration, AssetId, AssetLoadPolicy,
    AssetValidationFixture,
};
pub use assets::{
    AssetDeclarationError, AssetLoader, AssetMissingAction, RuntimeAssetProfile,
    validate_asset_declarations,
};
pub use basis_ktx2::{BasisKtx2Loader, BasisKtx2Plugin};
pub use validation::{
    AssetBudgetContract, AssetBudgetEntry, AssetBudgetRegistry, AssetLicenseEntry,
    AssetLicenseRegistry, AssetProvenance, AssetValidationError, AssetValidationPlugin,
    AssetValidationReport, AssetValidationStatus, ObjectRenderHandles, RenderAssetHandleId,
    TextureColorSpace, WorldRenderAssets, validate_asset_directory,
};
