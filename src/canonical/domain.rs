//! Canonical local-cell and brick-coordinate domains.

use super::cell::BRICK_EDGE_CELLS;

/// A volume-local point on the integer cell lattice.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LocalCellPoint(pub [i32; 3]);

/// A coordinate of an 8-cubed canonical brick.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BrickCoord(pub [i32; 3]);

/// A nonempty half-open local-cell domain.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LocalCellAabb {
    /// Inclusive lower cell coordinate.
    pub min: LocalCellPoint,
    /// Exclusive upper cell coordinate.
    pub max: LocalCellPoint,
}

/// A nonempty half-open brick-coordinate domain.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BrickAabb {
    /// Inclusive lower brick coordinate.
    pub min: BrickCoord,
    /// Exclusive upper brick coordinate.
    pub max: BrickCoord,
}

/// A reason a canonical domain is outside its declared bounds.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VolumeDomainError {
    /// A half-open domain has no cells or bricks on the named axis.
    EmptyOrInvertedAxis { axis: usize },
    /// A local-cell side exceeds its 8,191-cell maximum.
    SideTooLong { axis: usize },
    /// A local-cell corner lies more than 4,095 cells from the pivot.
    PivotRadiusExceeded { axis: usize },
}

impl LocalCellPoint {
    /// Returns this point's Euclidean local cell coordinate within its brick.
    ///
    /// The result is in `0..8` on every axis, including for negative cells.
    #[must_use]
    pub const fn brick_local_index(self) -> [u8; 3] {
        [
            self.0[0].rem_euclid(BRICK_EDGE_CELLS as i32) as u8,
            self.0[1].rem_euclid(BRICK_EDGE_CELLS as i32) as u8,
            self.0[2].rem_euclid(BRICK_EDGE_CELLS as i32) as u8,
        ]
    }
}

impl BrickCoord {
    /// Maps a local cell point to its containing brick by Euclidean division.
    ///
    /// # Examples
    ///
    /// ```
    /// use moria::canonical::{BrickCoord, LocalCellPoint};
    ///
    /// assert_eq!(BrickCoord::from_local_cell(LocalCellPoint([-1, 0, 0])), BrickCoord([-1, 0, 0]));
    /// ```
    #[must_use]
    pub const fn from_local_cell(cell: LocalCellPoint) -> Self {
        Self([
            cell.0[0].div_euclid(BRICK_EDGE_CELLS as i32),
            cell.0[1].div_euclid(BRICK_EDGE_CELLS as i32),
            cell.0[2].div_euclid(BRICK_EDGE_CELLS as i32),
        ])
    }
}

impl LocalCellAabb {
    /// Largest permitted local-cell side length.
    pub const MAX_SIDE_CELLS: i32 = 8_191;
    /// Largest permitted cell-corner distance from a volume pivot.
    pub const MAX_PIVOT_RADIUS_CELLS: i32 = 4_095;

    /// Validates and creates a nonempty bounded half-open local-cell domain.
    ///
    /// # Errors
    ///
    /// Returns [`VolumeDomainError`] if a side is empty, exceeds 8,191 cells,
    /// or if either inclusive cell corner lies farther than 4,095 cells from
    /// `pivot`. The exclusive upper boundary is not itself a cell corner.
    pub fn try_new(
        min: LocalCellPoint,
        max: LocalCellPoint,
        pivot: LocalCellPoint,
    ) -> Result<Self, VolumeDomainError> {
        for axis in 0..3 {
            let side = i64::from(max.0[axis]) - i64::from(min.0[axis]);
            if side <= 0 {
                return Err(VolumeDomainError::EmptyOrInvertedAxis { axis });
            }
            if side > i64::from(Self::MAX_SIDE_CELLS) {
                return Err(VolumeDomainError::SideTooLong { axis });
            }
            let low = i64::from(min.0[axis]) - i64::from(pivot.0[axis]);
            let high = i64::from(max.0[axis]) - 1 - i64::from(pivot.0[axis]);
            let radius = i64::from(Self::MAX_PIVOT_RADIUS_CELLS);
            if low < -radius || high > radius {
                return Err(VolumeDomainError::PivotRadiusExceeded { axis });
            }
        }
        Ok(Self { min, max })
    }
}

impl BrickAabb {
    /// Validates and creates a nonempty half-open brick-coordinate domain.
    ///
    /// # Errors
    ///
    /// Returns [`VolumeDomainError::EmptyOrInvertedAxis`] when `min >= max` on
    /// any axis. Brick coordinates otherwise span their full `i32` domain.
    pub fn try_new(min: BrickCoord, max: BrickCoord) -> Result<Self, VolumeDomainError> {
        for axis in 0..3 {
            if min.0[axis] >= max.0[axis] {
                return Err(VolumeDomainError::EmptyOrInvertedAxis { axis });
            }
        }
        Ok(Self { min, max })
    }
}
