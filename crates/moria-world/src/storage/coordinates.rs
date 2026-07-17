//! Fixed-width coordinates for the Product One authoritative region.

use core::fmt;

use serde::{Deserialize, Serialize};

/// Q8 units in one metre.
pub const Q8_UNITS_PER_METER: i32 = 256;
/// Q8 units along one authoritative voxel edge.
pub const VOXEL_EDGE_Q8: i32 = 64;
/// Voxels along one brick edge.
pub const BRICK_EDGE_VOXELS: i32 = 16;

const XZ_MIN_VOXEL: i32 = -2_000;
const XZ_MAX_VOXEL_EXCLUSIVE: i32 = 2_000;
const Y_MIN_VOXEL: i32 = -512;
const Y_MAX_VOXEL_EXCLUSIVE: i32 = 512;
const XZ_BRICK_COUNT: i16 = 250;
const Y_BRICK_COUNT: i16 = 64;

/// A coordinate outside the fixed Product One region.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinateError {
    OutOfBounds,
}

impl fmt::Display for CoordinateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBounds => {
                formatter.write_str("coordinate is outside the Product One region")
            }
        }
    }
}

impl std::error::Error for CoordinateError {}

/// An absolute voxel coordinate.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VoxelCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl VoxelCoord {
    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub const fn is_in_region(self) -> bool {
        self.x >= XZ_MIN_VOXEL
            && self.x < XZ_MAX_VOXEL_EXCLUSIVE
            && self.y >= Y_MIN_VOXEL
            && self.y < Y_MAX_VOXEL_EXCLUSIVE
            && self.z >= XZ_MIN_VOXEL
            && self.z < XZ_MAX_VOXEL_EXCLUSIVE
    }

    /// Converts a validated absolute voxel coordinate to its base-relative brick.
    pub fn to_brick_coord(self) -> Result<BrickCoord, CoordinateError> {
        self.to_brick_and_local_index().map(|(brick, _)| brick)
    }

    /// Converts a validated absolute voxel coordinate to its brick and local linear index.
    pub fn to_brick_and_local_index(self) -> Result<(BrickCoord, u16), CoordinateError> {
        if !self.is_in_region() {
            return Err(CoordinateError::OutOfBounds);
        }

        let relative_x = self.x - XZ_MIN_VOXEL;
        let relative_y = self.y - Y_MIN_VOXEL;
        let relative_z = self.z - XZ_MIN_VOXEL;
        let brick = BrickCoord::new(
            (relative_x / BRICK_EDGE_VOXELS) as i16,
            (relative_y / BRICK_EDGE_VOXELS) as i16,
            (relative_z / BRICK_EDGE_VOXELS) as i16,
        )?;
        let local_x = relative_x % BRICK_EDGE_VOXELS;
        let local_y = relative_y % BRICK_EDGE_VOXELS;
        let local_z = relative_z % BRICK_EDGE_VOXELS;
        let local_index = local_x + BRICK_EDGE_VOXELS * (local_z + BRICK_EDGE_VOXELS * local_y);

        Ok((brick, local_index as u16))
    }
}

/// A base-relative, validated 16-cubed brick coordinate.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BrickCoord {
    x: i16,
    y: i16,
    z: i16,
}

impl BrickCoord {
    /// Creates a brick coordinate after enforcing the fixed region bounds.
    pub fn new(x: i16, y: i16, z: i16) -> Result<Self, CoordinateError> {
        if !(0..XZ_BRICK_COUNT).contains(&x)
            || !(0..Y_BRICK_COUNT).contains(&y)
            || !(0..XZ_BRICK_COUNT).contains(&z)
        {
            return Err(CoordinateError::OutOfBounds);
        }
        Ok(Self { x, y, z })
    }

    #[must_use]
    pub const fn x(self) -> i16 {
        self.x
    }

    #[must_use]
    pub const fn y(self) -> i16 {
        self.y
    }

    #[must_use]
    pub const fn z(self) -> i16 {
        self.z
    }
}

/// A horizontal absolute voxel column coordinate.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColumnCoord {
    pub x: i32,
    pub z: i32,
}

/// An absolute point in metres multiplied by 256.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldPointQ8 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl WorldPointQ8 {
    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Floors a Q8 point to an in-bounds absolute voxel coordinate.
    pub fn to_voxel_coord(self) -> Result<VoxelCoord, CoordinateError> {
        let coordinate = VoxelCoord::new(
            self.x.div_euclid(VOXEL_EDGE_Q8),
            self.y.div_euclid(VOXEL_EDGE_Q8),
            self.z.div_euclid(VOXEL_EDGE_Q8),
        );
        coordinate
            .is_in_region()
            .then_some(coordinate)
            .ok_or(CoordinateError::OutOfBounds)
    }
}
