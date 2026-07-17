//! Pure loading and declaration validation shared by asset fixtures and runtime wiring.

use std::{collections::BTreeSet, error::Error, fmt};

use super::{
    ASSET_COUNT, ASSET_DECLARATIONS, AssetDeclaration, AssetId, AssetLoadPolicy,
    AssetValidationFixture,
};

/// Runtime profile that determines how a missing declared asset is handled.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeAssetProfile {
    Development,
    Release,
}

/// Observable result when a declared asset cannot be loaded.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AssetMissingAction {
    Fatal,
    DevelopmentFallback { warning: &'static str },
}

/// Rejection raised by immutable table or path resolution checks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssetDeclarationError {
    TableCardinality { actual: usize },
    DuplicateId { stable_id: &'static str },
    DuplicatePath { path: &'static str },
    MissingId { stable_id: &'static str },
    InvalidFixtureHook { stable_id: &'static str },
    UndeclaredRuntimePath { path: String },
}

impl fmt::Display for AssetDeclarationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TableCardinality { actual } => write!(
                formatter,
                "asset declaration table has {actual} entries; expected {ASSET_COUNT}"
            ),
            Self::DuplicateId { stable_id } => write!(formatter, "duplicate asset ID {stable_id}"),
            Self::DuplicatePath { path } => write!(formatter, "duplicate asset path {path}"),
            Self::MissingId { stable_id } => write!(formatter, "missing asset ID {stable_id}"),
            Self::InvalidFixtureHook { stable_id } => {
                write!(formatter, "invalid validation fixture hook for {stable_id}")
            }
            Self::UndeclaredRuntimePath { path } => {
                write!(formatter, "undeclared runtime asset path {path}")
            }
        }
    }
}

impl Error for AssetDeclarationError {}

/// Resolves runtime loads exclusively through the immutable declaration table.
#[derive(Clone, Copy, Debug, Default)]
pub struct AssetLoader;

impl AssetLoader {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn declaration(self, id: AssetId) -> &'static AssetDeclaration {
        ASSET_DECLARATIONS
            .iter()
            .find(|declaration| declaration.id == id)
            .expect("the immutable asset declaration table includes every AssetId")
    }

    pub fn resolve_runtime_path(
        self,
        path: &str,
    ) -> Result<&'static AssetDeclaration, AssetDeclarationError> {
        ASSET_DECLARATIONS
            .iter()
            .find(|declaration| declaration.path == path)
            .ok_or_else(|| AssetDeclarationError::UndeclaredRuntimePath {
                path: path.to_owned(),
            })
    }

    #[must_use]
    pub fn validation_fixture(self, id: AssetId) -> AssetValidationFixture {
        self.declaration(id).validation_fixture
    }

    #[must_use]
    pub fn missing_asset_action(
        self,
        id: AssetId,
        profile: RuntimeAssetProfile,
    ) -> AssetMissingAction {
        let declaration = self.declaration(id);
        match (declaration.load_policy, profile) {
            (
                AssetLoadPolicy::DevelopmentFallback | AssetLoadPolicy::ReleaseFatal,
                RuntimeAssetProfile::Development,
            ) => AssetMissingAction::DevelopmentFallback {
                warning: declaration.validation_fixture.key,
            },
            _ => AssetMissingAction::Fatal,
        }
    }
}

/// Validates a candidate table against the fixed Product One inventory contract.
pub fn validate_asset_declarations(
    declarations: &[AssetDeclaration],
) -> Result<(), AssetDeclarationError> {
    if declarations.len() != ASSET_COUNT {
        return Err(AssetDeclarationError::TableCardinality {
            actual: declarations.len(),
        });
    }

    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    let mut fixtures = BTreeSet::new();
    for declaration in declarations {
        if !ids.insert(declaration.id) {
            return Err(AssetDeclarationError::DuplicateId {
                stable_id: declaration.id.stable_id(),
            });
        }
        if !paths.insert(declaration.path) {
            return Err(AssetDeclarationError::DuplicatePath {
                path: declaration.path,
            });
        }
        if declaration.validation_fixture.asset_id != declaration.id
            || !fixtures.insert(declaration.validation_fixture.key)
        {
            return Err(AssetDeclarationError::InvalidFixtureHook {
                stable_id: declaration.id.stable_id(),
            });
        }
    }

    for id in AssetId::ALL {
        if !ids.contains(&id) {
            return Err(AssetDeclarationError::MissingId {
                stable_id: id.stable_id(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_table_is_valid() {
        assert_eq!(validate_asset_declarations(&ASSET_DECLARATIONS), Ok(()));
    }

    #[test]
    fn validation_rejects_duplicate_ids_paths_and_missing_ids() {
        let mut duplicate_id = ASSET_DECLARATIONS;
        duplicate_id[1].id = duplicate_id[0].id;
        assert!(matches!(
            validate_asset_declarations(&duplicate_id),
            Err(AssetDeclarationError::DuplicateId { .. })
        ));

        let mut duplicate_path = ASSET_DECLARATIONS;
        duplicate_path[1].path = duplicate_path[0].path;
        assert!(matches!(
            validate_asset_declarations(&duplicate_path),
            Err(AssetDeclarationError::DuplicatePath { .. })
        ));

        assert_eq!(
            validate_asset_declarations(&ASSET_DECLARATIONS[..ASSET_COUNT - 1]),
            Err(AssetDeclarationError::TableCardinality {
                actual: ASSET_COUNT - 1,
            })
        );
    }

    #[test]
    fn loader_makes_fallbacks_and_release_fatal_behavior_observable() {
        let loader = AssetLoader::new();
        assert!(matches!(
            loader.missing_asset_action(AssetId::BirchNear, RuntimeAssetProfile::Development),
            AssetMissingAction::DevelopmentFallback { .. }
        ));
        assert_eq!(
            loader.missing_asset_action(AssetId::BirchNear, RuntimeAssetProfile::Release),
            AssetMissingAction::Fatal
        );
        assert_eq!(
            loader.missing_asset_action(AssetId::TerrainShader, RuntimeAssetProfile::Development),
            AssetMissingAction::DevelopmentFallback {
                warning: AssetId::TerrainShader.stable_id(),
            }
        );
    }
}
