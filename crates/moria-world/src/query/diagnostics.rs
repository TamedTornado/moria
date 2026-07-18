//! Immutable, bounded diagnostic page value types.

use crate::streaming::FocusPurpose;
use crate::{AabbQ8, ActiveBand, BrickCoord, MaterialId, WorldPointQ8};

/// Opaque token binding all pages in one diagnostic snapshot generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiagnosticSnapshotToken(
    pub(crate) u64,
    pub(crate) u64,
    pub(crate) u64,
    pub(crate) u64,
);

/// A bounded diagnostic page request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiagnosticPageRequest {
    pub snapshot: Option<DiagnosticSnapshotToken>,
    pub after_brick: Option<BrickCoord>,
    pub max_bricks: u16,
    pub include_cells: bool,
}

/// Owned, immutable diagnostic data for one page.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticPage {
    pub snapshot: DiagnosticSnapshotToken,
    pub frame: u64,
    pub revision: u64,
    pub bricks: Vec<DiagnosticBrick>,
    pub render_chunks: Vec<DiagnosticRenderChunk>,
    pub focuses: Vec<DiagnosticFocus>,
    pub next_after_brick: Option<BrickCoord>,
}

/// Metadata and, optionally, every cell of one active brick.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticBrick {
    pub coord: BrickCoord,
    pub bounds: AabbQ8,
    pub band: ActiveBand,
    pub purposes: FocusPurposeFlags,
    pub dirty: DiagnosticDirtyFlags,
    pub pin_count: u16,
    pub task: Option<DiagnosticTaskKind>,
    pub cells: Option<Vec<DiagnosticCell>>,
}

/// One current voxel in a diagnostic cell-bearing page.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiagnosticCell {
    pub local_index: u16,
    pub material: MaterialId,
    pub density: u8,
    pub material_present: bool,
    pub solid_collision: bool,
    pub water_volume: bool,
}

/// The focused purposes contributing to a brick.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FocusPurposeFlags(u8);

impl FocusPurposeFlags {
    #[must_use]
    pub const fn contains(self, purpose: FocusPurpose) -> bool {
        self.0 & purpose_bit(purpose) != 0
    }

    pub(crate) const fn insert(&mut self, purpose: FocusPurpose) {
        self.0 |= purpose_bit(purpose);
    }
}

const fn purpose_bit(purpose: FocusPurpose) -> u8 {
    match purpose {
        FocusPurpose::Traversal => 1,
        FocusPurpose::Camera => 2,
        FocusPurpose::Inspection => 4,
        FocusPurpose::Mutation => 8,
    }
}

/// Dirty representation categories retained by a brick.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DiagnosticDirtyFlags(pub(crate) u8);

/// A pending derived-work kind for a brick.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticTaskKind {
    Extraction,
    Installation,
    Render,
}

/// One derived render chunk included with a diagnostic page.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticRenderChunk {
    pub key: DiagnosticRenderChunkKey,
    pub bounds: AabbQ8,
    pub lod: u8,
    pub band: ActiveBand,
    pub revision: u64,
    pub phase: DiagnosticRenderChunkPhase,
}

/// Public value key for a derived render chunk.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticRenderChunkKey {
    pub brick: BrickCoord,
    pub lod: u8,
}

/// Whether a diagnostic render chunk is resident or still pending.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticRenderChunkPhase {
    Resident,
    Pending,
}

/// A public focus marker returned with a page.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiagnosticFocus {
    pub id: u32,
    pub position: WorldPointQ8,
    pub purpose: FocusPurpose,
}
