//! A deliberately dense, bounded oracle for public-path conformance tests.
//!
//! This module is test support, not a world-store implementation.  It owns a
//! complete voxel array for a tiny caller-selected region and deliberately has
//! no dependency on production storage, feature indexes, task scheduling, or
//! persistence.  Production integration tests should drive `MoriaWorldPlugin`
//! through its public facade, then compare the public observations with this
//! model after each committed boundary.

use std::collections::BTreeMap;

use crate::{CoordinateError, Voxel, VoxelCoord};

/// Upper bound that keeps an accidentally large test region from allocating a
/// production-sized dense world.
pub const MAX_DENSE_VOXELS: usize = 1_048_576;

/// Errors reported by the bounded dense oracle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DenseWorldError {
    /// The exclusive upper coordinate is not strictly above the lower one.
    EmptyRegion,
    /// The requested dense region would exceed [`MAX_DENSE_VOXELS`].
    TooLarge,
    /// A coordinate is outside the oracle's caller-selected region.
    OutOfBounds,
}

impl From<CoordinateError> for DenseWorldError {
    fn from(_: CoordinateError) -> Self {
        Self::OutOfBounds
    }
}

/// A small half-open coordinate region owned by the oracle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DenseRegion {
    min: VoxelCoord,
    max_exclusive: VoxelCoord,
    dimensions: [usize; 3],
}

impl DenseRegion {
    /// Creates a bounded region with `min <= coordinate < max_exclusive`.
    pub fn new(min: VoxelCoord, max_exclusive: VoxelCoord) -> Result<Self, DenseWorldError> {
        let width = i64::from(max_exclusive.x) - i64::from(min.x);
        let height = i64::from(max_exclusive.y) - i64::from(min.y);
        let depth = i64::from(max_exclusive.z) - i64::from(min.z);
        if width <= 0 || height <= 0 || depth <= 0 {
            return Err(DenseWorldError::EmptyRegion);
        }

        let voxel_count = width
            .checked_mul(height)
            .and_then(|count| count.checked_mul(depth))
            .ok_or(DenseWorldError::TooLarge)?;
        if voxel_count > MAX_DENSE_VOXELS as i64 {
            return Err(DenseWorldError::TooLarge);
        }

        Ok(Self {
            min,
            max_exclusive,
            dimensions: [width as usize, height as usize, depth as usize],
        })
    }

    #[must_use]
    pub const fn min(self) -> VoxelCoord {
        self.min
    }

    #[must_use]
    pub const fn max_exclusive(self) -> VoxelCoord {
        self.max_exclusive
    }

    #[must_use]
    pub const fn contains(self, coordinate: VoxelCoord) -> bool {
        coordinate.x >= self.min.x
            && coordinate.x < self.max_exclusive.x
            && coordinate.y >= self.min.y
            && coordinate.y < self.max_exclusive.y
            && coordinate.z >= self.min.z
            && coordinate.z < self.max_exclusive.z
    }

    fn index_of(self, coordinate: VoxelCoord) -> Result<usize, DenseWorldError> {
        if !self.contains(coordinate) {
            return Err(DenseWorldError::OutOfBounds);
        }
        let x = (coordinate.x - self.min.x) as usize;
        let y = (coordinate.y - self.min.y) as usize;
        let z = (coordinate.z - self.min.z) as usize;
        Ok(x + self.dimensions[0] * (z + self.dimensions[2] * y))
    }

    /// Returns the deterministic `x, z, y` dense-order coordinate at `index`.
    #[must_use]
    pub fn coordinate_at(self, index: usize) -> Option<VoxelCoord> {
        if index >= self.voxel_count() {
            return None;
        }
        Some(self.coordinate_at_unchecked(index))
    }

    #[must_use]
    pub fn voxel_count(self) -> usize {
        self.dimensions.iter().product()
    }

    fn coordinate_at_unchecked(self, index: usize) -> VoxelCoord {
        let x = index % self.dimensions[0];
        let yz = index / self.dimensions[0];
        let z = yz % self.dimensions[2];
        let y = yz / self.dimensions[2];
        VoxelCoord::new(
            self.min.x + x as i32,
            self.min.y + y as i32,
            self.min.z + z as i32,
        )
    }
}

/// The first authoritative voxel difference encountered by a comparison.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DenseVoxelMismatch {
    coordinate: VoxelCoord,
    expected: Voxel,
    actual: Voxel,
}

impl DenseVoxelMismatch {
    #[must_use]
    pub const fn coordinate(self) -> VoxelCoord {
        self.coordinate
    }

    #[must_use]
    pub const fn expected(self) -> Voxel {
        self.expected
    }

    #[must_use]
    pub const fn actual(self) -> Voxel {
        self.actual
    }
}

/// A comparison either found a voxel mismatch or could not obtain a public observation.
#[derive(Debug)]
pub enum DenseComparisonError<E> {
    Observation(E),
    Mismatch(DenseVoxelMismatch),
}

impl<E> DenseComparisonError<E> {
    #[must_use]
    pub const fn mismatch(&self) -> Option<DenseVoxelMismatch> {
        match self {
            Self::Observation(_) => None,
            Self::Mismatch(mismatch) => Some(*mismatch),
        }
    }
}

/// Base-feature precedence in the documented lowest-to-highest order.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum DenseFeaturePrecedence {
    Air = 0,
    Water = 1,
    Topsoil = 2,
    Subsoil = 3,
    HostStratum = 4,
    Aquifer = 5,
    OreVein = 6,
    CaveVoid = 7,
    Object = 8,
    Ruin = 9,
}

/// Explicit cells from one independently registered base feature.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DenseFeature {
    id: u32,
    precedence: DenseFeaturePrecedence,
    voxels: BTreeMap<VoxelCoord, Voxel>,
}

impl DenseFeature {
    #[must_use]
    pub fn new(
        id: u32,
        precedence: DenseFeaturePrecedence,
        voxels: impl IntoIterator<Item = (VoxelCoord, Voxel)>,
    ) -> Self {
        Self {
            id,
            precedence,
            voxels: voxels.into_iter().collect(),
        }
    }
}

/// Complete small-world truth: dense base cells plus only non-base edit values.
#[derive(Clone, Debug)]
pub struct DenseWorld {
    region: DenseRegion,
    unfeatured_base: Vec<Voxel>,
    features: Vec<DenseFeature>,
    overlays: BTreeMap<VoxelCoord, Voxel>,
}

impl DenseWorld {
    /// Evaluates the procedural base once for every cell in a small region.
    #[must_use]
    pub fn from_base(region: DenseRegion, mut evaluate: impl FnMut(VoxelCoord) -> Voxel) -> Self {
        let count = region.voxel_count();
        let unfeatured_base = (0..count)
            .map(|index| evaluate(region.coordinate_at_unchecked(index)))
            .collect();
        Self {
            region,
            unfeatured_base,
            features: Vec::new(),
            overlays: BTreeMap::new(),
        }
    }

    /// Registers immutable base truth; equal-precedence features use lowest ID.
    pub fn register_feature(&mut self, feature: DenseFeature) -> Result<(), DenseWorldError> {
        if feature
            .voxels
            .keys()
            .any(|coordinate| !self.region.contains(*coordinate))
        {
            return Err(DenseWorldError::OutOfBounds);
        }
        self.features.push(feature);
        let reverted = self
            .overlays
            .iter()
            .filter_map(|(coordinate, voxel)| {
                (*voxel == self.base_sample(*coordinate).expect("overlay is in bounds"))
                    .then_some(*coordinate)
            })
            .collect::<Vec<_>>();
        for coordinate in reverted {
            self.overlays.remove(&coordinate);
        }
        Ok(())
    }

    /// Reads the fully composed authoritative value.
    pub fn sample(&self, coordinate: VoxelCoord) -> Result<Voxel, DenseWorldError> {
        self.region.index_of(coordinate)?;
        Ok(self.overlays.get(&coordinate).copied().unwrap_or_else(|| {
            self.base_sample(coordinate)
                .expect("coordinate was validated before dense base sampling")
        }))
    }

    /// Applies an explicit base-relative overlay, returning whether truth changed.
    pub fn set_overlay(
        &mut self,
        coordinate: VoxelCoord,
        voxel: Voxel,
    ) -> Result<bool, DenseWorldError> {
        let base = self.base_sample(coordinate)?;
        let current = self.sample(coordinate)?;
        if current == voxel {
            return Ok(false);
        }
        if voxel == base {
            self.overlays.remove(&coordinate);
        } else {
            self.overlays.insert(coordinate, voxel);
        }
        Ok(true)
    }

    /// Removes one edit overlay, returning whether the value changed.
    pub fn revert_overlay(&mut self, coordinate: VoxelCoord) -> Result<bool, DenseWorldError> {
        self.region.index_of(coordinate)?;
        Ok(self.overlays.remove(&coordinate).is_some())
    }

    #[must_use]
    pub const fn region(&self) -> DenseRegion {
        self.region
    }

    #[must_use]
    pub fn voxel_count(&self) -> usize {
        self.unfeatured_base.len()
    }

    #[must_use]
    pub fn overlay_len(&self) -> usize {
        self.overlays.len()
    }

    /// Compares every authoritative byte with a public production-path sample.
    ///
    /// The callback intentionally accepts an error type supplied by the caller:
    /// integration tests must surface public query failures rather than masking
    /// them with an oracle-side fallback.
    pub fn compare_voxels<E>(
        &self,
        mut sample: impl FnMut(VoxelCoord) -> Result<Voxel, E>,
    ) -> Result<(), DenseComparisonError<E>> {
        for (coordinate, expected) in self.all_voxels() {
            let actual = sample(coordinate).map_err(DenseComparisonError::Observation)?;
            if actual != expected {
                return Err(DenseComparisonError::Mismatch(DenseVoxelMismatch {
                    coordinate,
                    expected,
                    actual,
                }));
            }
        }
        Ok(())
    }

    /// Iterates all composed voxels in deterministic `x, z, y` dense order.
    pub fn all_voxels(&self) -> impl Iterator<Item = (VoxelCoord, Voxel)> + '_ {
        (0..self.voxel_count()).map(|index| {
            let coordinate = self.region.coordinate_at_unchecked(index);
            (
                coordinate,
                self.sample(coordinate)
                    .expect("coordinates generated from the dense region are valid"),
            )
        })
    }

    fn base_sample(&self, coordinate: VoxelCoord) -> Result<Voxel, DenseWorldError> {
        let index = self.region.index_of(coordinate)?;
        let mut selected: Option<(DenseFeaturePrecedence, u32, Voxel)> = None;
        for feature in &self.features {
            let Some(voxel) = feature.voxels.get(&coordinate).copied() else {
                continue;
            };
            if selected.is_none_or(|(precedence, id, _)| {
                feature.precedence > precedence
                    || (feature.precedence == precedence && feature.id < id)
            }) {
                selected = Some((feature.precedence, feature.id, voxel));
            }
        }
        Ok(selected.map_or(self.unfeatured_base[index], |(_, _, voxel)| voxel))
    }
}
