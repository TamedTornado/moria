//! Canonical material-cell and brick wire representations.

/// Number of cells on each edge of one canonical brick.
pub const BRICK_EDGE_CELLS: usize = 8;
/// Number of cells in one canonical brick.
pub const BRICK_CELL_COUNT: usize = BRICK_EDGE_CELLS * BRICK_EDGE_CELLS * BRICK_EDGE_CELLS;
/// Exact byte length of a dense canonical brick payload.
pub const DENSE_BRICK_BYTES: usize = BRICK_CELL_COUNT * CellWire::BYTE_LEN;

/// A four-byte canonical material cell wire record.
///
/// `material_id` is encoded first as little-endian `u16`; `density_q8_8`
/// follows as little-endian `i16`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CellWire {
    /// Consumer-defined material identity, where zero represents empty matter.
    pub material_id: u16,
    /// Signed Q8.8 density coverage.
    pub density_q8_8: i16,
}

/// A reason a canonical cell cannot be admitted.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CellValidationError {
    /// Empty matter cannot have positive density.
    EmptyMaterialPositiveDensity,
    /// Nonempty matter did not name a material registered at genesis.
    UnregisteredMaterial {
        /// The invalid nonzero material identity.
        material_id: u16,
    },
}

impl CellWire {
    /// Exact canonical byte width of one cell.
    pub const BYTE_LEN: usize = 4;

    /// Decodes one little-endian canonical cell record.
    #[must_use]
    pub const fn from_le_bytes(bytes: [u8; Self::BYTE_LEN]) -> Self {
        Self {
            material_id: u16::from_le_bytes([bytes[0], bytes[1]]),
            density_q8_8: i16::from_le_bytes([bytes[2], bytes[3]]),
        }
    }

    /// Encodes this cell in its exact four-byte canonical representation.
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; Self::BYTE_LEN] {
        let material = self.material_id.to_le_bytes();
        let density = self.density_q8_8.to_le_bytes();
        [material[0], material[1], density[0], density[1]]
    }

    /// Validates the material/density relation against genesis material facts.
    ///
    /// # Errors
    ///
    /// Returns [`CellValidationError::EmptyMaterialPositiveDensity`] when the
    /// empty material has positive density, and
    /// [`CellValidationError::UnregisteredMaterial`] when nonempty material is
    /// not registered. The callback is only consulted for nonzero material IDs.
    ///
    /// # Examples
    ///
    /// ```
    /// use moria::canonical::CellWire;
    ///
    /// let cell = CellWire { material_id: 1, density_q8_8: 0 };
    /// assert!(cell.validate_registered(|id| id == 1).is_ok());
    /// ```
    pub fn validate_registered(
        self,
        is_registered: impl FnOnce(u16) -> bool,
    ) -> Result<(), CellValidationError> {
        if self.material_id == 0 {
            if self.density_q8_8 > 0 {
                return Err(CellValidationError::EmptyMaterialPositiveDensity);
            }
            return Ok(());
        }
        if !is_registered(self.material_id) {
            return Err(CellValidationError::UnregisteredMaterial {
                material_id: self.material_id,
            });
        }
        Ok(())
    }
}

/// A reason dense canonical brick bytes cannot be decoded.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BrickDecodeError {
    /// The dense payload did not contain exactly 2,048 bytes.
    InvalidByteLength {
        /// Number of bytes supplied by the caller.
        actual: usize,
    },
}

/// A canonical brick, stored either as one uniform cell or as all 512 cells.
///
/// The dense payload is out of line so that a uniform brick retains compact
/// storage while dense bricks still own their fixed-size canonical cells.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Brick {
    /// A brick whose every cell is the same canonical record.
    Uniform(CellWire),
    /// Dense cells in x-major, then y, then z order.
    Dense(Box<[CellWire; BRICK_CELL_COUNT]>),
}

impl Brick {
    /// Creates a uniform brick without allocating.
    #[must_use]
    pub const fn uniform(cell: CellWire) -> Self {
        Self::Uniform(cell)
    }

    /// Creates a dense brick in canonical x-major, then y, then z order.
    #[must_use]
    pub fn dense(cells: [CellWire; BRICK_CELL_COUNT]) -> Self {
        Self::Dense(Box::new(cells))
    }

    /// Reports whether this brick uses its uniform representation.
    #[must_use]
    pub const fn is_uniform(&self) -> bool {
        matches!(self, Self::Uniform(_))
    }

    /// Decodes an exact dense canonical payload in x-major, then y, then z order.
    ///
    /// # Errors
    ///
    /// Returns [`BrickDecodeError::InvalidByteLength`] unless `bytes` is
    /// exactly [`DENSE_BRICK_BYTES`] long. The accepted bytes are copied into
    /// this brick's owned fixed-size dense representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use moria::canonical::{Brick, DENSE_BRICK_BYTES};
    ///
    /// let brick = Brick::try_from_dense_le_bytes(&[0; DENSE_BRICK_BYTES])?;
    /// assert!(!brick.is_uniform());
    /// # Ok::<(), moria::canonical::BrickDecodeError>(())
    /// ```
    pub fn try_from_dense_le_bytes(bytes: &[u8]) -> Result<Self, BrickDecodeError> {
        if bytes.len() != DENSE_BRICK_BYTES {
            return Err(BrickDecodeError::InvalidByteLength {
                actual: bytes.len(),
            });
        }
        Ok(Self::dense(core::array::from_fn(|index| {
            let offset = index * CellWire::BYTE_LEN;
            CellWire::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ])
        })))
    }

    /// Returns the cell at a valid local coordinate in `0..8` on every axis.
    ///
    /// # Panics
    ///
    /// Panics if any local coordinate lies outside `0..8`.
    #[must_use]
    pub fn cell_at(&self, local: [u8; 3]) -> CellWire {
        assert!(
            local
                .iter()
                .all(|&coordinate| (coordinate as usize) < BRICK_EDGE_CELLS)
        );
        match self {
            Self::Uniform(cell) => *cell,
            Self::Dense(cells) => cells[Self::dense_index(local)],
        }
    }

    /// Encodes this brick as an exact 2,048-byte dense canonical payload.
    #[must_use]
    pub fn to_dense_le_bytes(&self) -> [u8; DENSE_BRICK_BYTES] {
        let mut bytes = [0; DENSE_BRICK_BYTES];
        match self {
            Self::Uniform(cell) => {
                let cell_bytes = cell.to_le_bytes();
                for destination in bytes.chunks_exact_mut(CellWire::BYTE_LEN) {
                    destination.copy_from_slice(&cell_bytes);
                }
            }
            Self::Dense(cells) => {
                for (cell, destination) in
                    cells.iter().zip(bytes.chunks_exact_mut(CellWire::BYTE_LEN))
                {
                    destination.copy_from_slice(&cell.to_le_bytes());
                }
            }
        }
        bytes
    }

    fn dense_index(local: [u8; 3]) -> usize {
        local[0] as usize
            + BRICK_EDGE_CELLS * (local[1] as usize + BRICK_EDGE_CELLS * local[2] as usize)
    }
}
