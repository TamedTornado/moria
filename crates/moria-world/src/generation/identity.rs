//! Fixed-point bounds and immutable world identity.

use serde::{Deserialize, Serialize};

use crate::WorldPointQ8;

/// An invalid min-inclusive/max-exclusive fixed-point bounds pair.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundsError {
    EmptyOrInverted,
}

/// An axis-aligned Q8 box with min-inclusive and max-exclusive corners.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AabbQ8 {
    pub min: WorldPointQ8,
    pub max_exclusive: WorldPointQ8,
}

impl AabbQ8 {
    pub fn new(min: WorldPointQ8, max_exclusive: WorldPointQ8) -> Result<Self, BoundsError> {
        (min.x < max_exclusive.x && min.y < max_exclusive.y && min.z < max_exclusive.z)
            .then_some(Self { min, max_exclusive })
            .ok_or(BoundsError::EmptyOrInverted)
    }

    #[must_use]
    pub const fn contains(self, point: WorldPointQ8) -> bool {
        point.x >= self.min.x
            && point.x < self.max_exclusive.x
            && point.y >= self.min.y
            && point.y < self.max_exclusive.y
            && point.z >= self.min.z
            && point.z < self.max_exclusive.z
    }
}

/// The public fixed-point bounds of an opened world.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorldBounds(pub AabbQ8);

impl WorldBounds {
    pub fn new(min: WorldPointQ8, max_exclusive: WorldPointQ8) -> Result<Self, BoundsError> {
        AabbQ8::new(min, max_exclusive).map(Self)
    }

    #[must_use]
    pub const fn min(self) -> WorldPointQ8 {
        self.0.min
    }

    #[must_use]
    pub const fn max_exclusive(self) -> WorldPointQ8 {
        self.0.max_exclusive
    }

    #[must_use]
    pub const fn contains(self, point: WorldPointQ8) -> bool {
        self.0.contains(point)
    }
}

/// The seed and canonical generation-input digest used to identify a world.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldSeed {
    pub value: u64,
    pub parameters_digest: [u8; 32],
}

/// Immutable identity exposed by a successfully opened world.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldIdentity {
    pub seed: u64,
    pub parameters_digest: [u8; 32],
    pub bounds: WorldBounds,
}

impl WorldIdentity {
    #[must_use]
    pub const fn new(seed: u64, parameters_digest: [u8; 32], bounds: WorldBounds) -> Self {
        Self {
            seed,
            parameters_digest,
            bounds,
        }
    }
}
