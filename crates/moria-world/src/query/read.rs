//! Private read state and the public, immutable Bevy observation parameter.

use std::collections::{BTreeMap, Bound};

use bevy::{ecs::system::SystemParam, prelude::*};

use crate::storage::WorldStore;
use crate::telemetry::WorldTelemetryState;
use crate::{
    AIR, BrickCoord, ColumnCoord, ColumnSample, MaterialRegistry, RunKind, VOXEL_EDGE_Q8,
    VoxelCoord, WATER, WaterBodyDef, WorldBounds, WorldIdentity, WorldLifecycle, WorldPointQ8,
    evaluate_column, material_present, solid_collision, water_volume,
};

use super::{
    ActiveBand, DiagnosticBrick, DiagnosticCell, DiagnosticDirtyFlags, DiagnosticFocus,
    DiagnosticPage, DiagnosticPageRequest, DiagnosticRenderChunk, DiagnosticRenderChunkKey,
    DiagnosticSnapshotToken, DiagnosticTaskKind, FocusPurposeFlags, QueryError, QueryLimitKind,
    QueryMask, TraversalRoute, WaterSample, WorldHit, WorldRayQ8, WorldSample, ray,
};
use crate::streaming::FocusState;

/// Private authoritative state observed by [`WorldRead`].
#[derive(Resource)]
pub(crate) struct WorldReadState {
    store: WorldStore,
    materials: MaterialRegistry,
    water_bodies: Vec<WaterBodyDef>,
    route: TraversalRoute,
    active_bands: BTreeMap<BrickCoord, ActiveBand>,
    diagnostic_statuses: BTreeMap<BrickCoord, DiagnosticBrickStatus>,
    render_chunks: BTreeMap<DiagnosticRenderChunkKey, DiagnosticRenderChunk>,
    diagnostic_generation: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct DiagnosticBrickStatus {
    dirty: DiagnosticDirtyFlags,
    pin_count: u16,
    task: Option<DiagnosticTaskKind>,
}

impl WorldReadState {
    #[allow(
        dead_code,
        reason = "world lifecycle installs read state after asynchronous opening"
    )]
    pub(crate) fn new(
        identity: WorldIdentity,
        materials: MaterialRegistry,
        water_bodies: Vec<WaterBodyDef>,
        route: TraversalRoute,
    ) -> Self {
        Self {
            store: WorldStore::new(identity),
            materials,
            water_bodies,
            route,
            active_bands: BTreeMap::new(),
            diagnostic_statuses: BTreeMap::new(),
            render_chunks: BTreeMap::new(),
            diagnostic_generation: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn commit_test_voxels(
        &mut self,
        changes: impl IntoIterator<Item = (VoxelCoord, crate::Voxel)>,
    ) {
        self.store.commit_current(changes);
    }

    #[allow(
        dead_code,
        reason = "streaming installs and removes active bands once its lifecycle is available"
    )]
    pub(crate) fn set_active_band(
        &mut self,
        brick: BrickCoord,
        band: Option<ActiveBand>,
        focuses: &mut FocusState,
    ) {
        let was_active = self.active_bands.contains_key(&brick);
        let mut auxiliary_changed = false;
        let previous = match band {
            Some(band) => self.active_bands.insert(brick, band),
            None => {
                auxiliary_changed |= self.diagnostic_statuses.remove(&brick).is_some();
                let previous_chunk_count = self.render_chunks.len();
                self.render_chunks.retain(|key, _| key.brick != brick);
                auxiliary_changed |= self.render_chunks.len() != previous_chunk_count;
                self.active_bands.remove(&brick)
            }
        };
        let is_active = self.active_bands.contains_key(&brick);
        if was_active != is_active {
            focuses.set_brick_active(brick, is_active);
        }
        if previous != band || auxiliary_changed {
            self.invalidate_diagnostics();
        }
    }

    pub(crate) fn is_active_brick(&self, brick: BrickCoord) -> bool {
        self.active_bands.contains_key(&brick)
    }

    #[allow(
        dead_code,
        reason = "streaming and mutation publish brick state once their lifecycle is available"
    )]
    pub(crate) fn set_diagnostic_status(
        &mut self,
        brick: BrickCoord,
        dirty: DiagnosticDirtyFlags,
        pin_count: u16,
        task: Option<DiagnosticTaskKind>,
    ) {
        let status = DiagnosticBrickStatus {
            dirty,
            pin_count,
            task,
        };
        if self.diagnostic_statuses.get(&brick) != Some(&status) {
            self.diagnostic_statuses.insert(brick, status);
            self.invalidate_diagnostics();
        }
    }

    #[allow(
        dead_code,
        reason = "render installation publishes chunks once its lifecycle is available"
    )]
    pub(crate) fn set_render_chunk(&mut self, chunk: DiagnosticRenderChunk) {
        debug_assert_eq!(chunk.key.lod, chunk.lod);
        if self.render_chunks.get(&chunk.key) != Some(&chunk) {
            self.render_chunks.insert(chunk.key, chunk);
            self.invalidate_diagnostics();
        }
    }

    #[allow(
        dead_code,
        reason = "render eviction removes chunks once its lifecycle is available"
    )]
    pub(crate) fn remove_render_chunk(&mut self, key: DiagnosticRenderChunkKey) {
        if self.render_chunks.remove(&key).is_some() {
            self.invalidate_diagnostics();
        }
    }

    #[allow(
        dead_code,
        reason = "streaming and mutation mark dirty/task changes before their plugins are installed"
    )]
    pub(crate) fn invalidate_diagnostics(&mut self) {
        self.diagnostic_generation = self
            .diagnostic_generation
            .checked_add(1)
            .expect("diagnostic generation cannot wrap");
    }
}

/// Read-only synchronous access to current authoritative world truth.
#[derive(SystemParam)]
pub struct WorldRead<'w, 's> {
    state: Option<Res<'w, WorldReadState>>,
    focuses: Option<Res<'w, FocusState>>,
    telemetry: Option<Res<'w, WorldTelemetryState>>,
    lifecycle: Option<Res<'w, WorldLifecycle>>,
    _system_state: Local<'s, ()>,
}

impl WorldRead<'_, '_> {
    pub(super) fn ready_bounds(&self) -> Result<WorldBounds, QueryError> {
        Ok(self.ready_state()?.store.identity().bounds)
    }

    fn ready_state(&self) -> Result<&WorldReadState, QueryError> {
        if !self
            .lifecycle
            .as_deref()
            .is_some_and(WorldLifecycle::is_ready)
        {
            return Err(QueryError::NotReady);
        }
        self.state.as_deref().ok_or(QueryError::NotReady)
    }

    #[must_use]
    pub fn identity(&self) -> &WorldIdentity {
        self.state
            .as_deref()
            .map(|state| state.store.identity())
            .expect("WorldRead identity is observed only after its world state is installed")
    }

    #[must_use]
    pub fn bounds(&self) -> WorldBounds {
        self.identity().bounds
    }

    pub fn sample_voxel(&self, coordinate: VoxelCoord) -> Result<WorldSample, QueryError> {
        let state = self.ready_state()?;
        if !coordinate.is_in_region() {
            return Err(QueryError::OutOfBounds);
        }

        let voxel = state.store.current_voxel(coordinate);
        Ok(WorldSample::from_voxel(
            coordinate,
            voxel,
            &state.materials,
            state.store.revision(),
        ))
    }

    pub fn sample_point(&self, point: WorldPointQ8) -> Result<WorldSample, QueryError> {
        let coordinate = point
            .to_voxel_coord()
            .map_err(|_| QueryError::OutOfBounds)?;
        self.sample_voxel(coordinate)
    }

    pub fn sample_column(&self, coordinate: ColumnCoord) -> Result<ColumnSample, QueryError> {
        let state = self.ready_state()?;
        if !VoxelCoord::new(coordinate.x, 0, coordinate.z).is_in_region() {
            return Err(QueryError::OutOfBounds);
        }

        let mut column = evaluate_column(state.store.identity(), coordinate);
        column.runs.clear();
        let min_y = state
            .store
            .identity()
            .bounds
            .min()
            .y
            .div_euclid(VOXEL_EDGE_Q8);
        let max_y = state
            .store
            .identity()
            .bounds
            .max_exclusive()
            .y
            .div_euclid(VOXEL_EDGE_Q8);

        for y in min_y..max_y {
            let voxel = state
                .store
                .current_voxel(VoxelCoord::new(coordinate.x, y, coordinate.z));
            let kind = sample_run_kind(voxel.material.0);
            if let Some(last) = column.runs.last_mut()
                && last.material == voxel.material
                && last.kind == kind
            {
                last.y_max_voxel_exclusive = (y + 1) as i16;
                continue;
            }

            if column.runs.len() == 64 {
                return Err(QueryError::LimitExceeded(QueryLimitKind::ColumnRuns));
            }
            column.runs.push(crate::ColumnRun {
                y_min_voxel: y as i16,
                y_max_voxel_exclusive: (y + 1) as i16,
                material: voxel.material,
                kind,
            });
        }
        Ok(column)
    }

    /// Returns the first solid or water voxel reached by a bounded normalized ray.
    pub fn ray_cast(
        &self,
        ray: WorldRayQ8,
        max_distance_q8: u32,
        mask: QueryMask,
    ) -> Result<Option<WorldHit>, QueryError> {
        let state = self.ready_state()?;
        ray::cast(ray, max_distance_q8, mask, |coordinate| {
            WorldSample::from_voxel(
                coordinate,
                state.store.current_voxel(coordinate),
                &state.materials,
                state.store.revision(),
            )
        })
    }

    pub fn water_surface_at(
        &self,
        x_q8: i32,
        z_q8: i32,
    ) -> Result<Option<WaterSample>, QueryError> {
        let state = self.ready_state()?;
        let bounds = state.store.identity().bounds;
        if x_q8 < bounds.min().x
            || x_q8 >= bounds.max_exclusive().x
            || z_q8 < bounds.min().z
            || z_q8 >= bounds.max_exclusive().z
        {
            return Err(QueryError::OutOfBounds);
        }

        Ok(state
            .water_bodies
            .iter()
            .find(|body| footprint_contains(body, x_q8, z_q8))
            .map(|body| WaterSample::from_body(body, state.store.revision())))
    }

    #[must_use]
    pub fn route(&self) -> &TraversalRoute {
        self.state
            .as_deref()
            .map(|state| &state.route)
            .expect("WorldRead route is observed only after its world state is installed")
    }

    #[must_use]
    pub fn active_band(&self, brick: BrickCoord) -> Option<ActiveBand> {
        self.state
            .as_deref()
            .and_then(|state| state.active_bands.get(&brick).copied())
    }

    /// Returns one owned, bounded page of active-world diagnostics.
    pub fn diagnostic_snapshot(
        &self,
        request: DiagnosticPageRequest,
    ) -> Result<DiagnosticPage, QueryError> {
        validate_diagnostic_request(request)?;
        let state = self.ready_state()?;
        let focus_state = self.focuses.as_deref();
        let current_frame = self
            .telemetry
            .as_deref()
            .map_or(0, WorldTelemetryState::frame_index);
        let snapshot = match request.snapshot {
            Some(requested) if diagnostic_snapshot_is_current(requested, state, focus_state) => {
                requested
            }
            Some(_) => return Err(QueryError::SnapshotExpired),
            None => diagnostic_snapshot_token(state, focus_state, current_frame),
        };
        let frame = snapshot.3;

        let mut page_bricks = Vec::with_capacity(usize::from(request.max_bricks));
        let mut candidates = state.active_bands.range((
            request
                .after_brick
                .map_or(Bound::Unbounded, Bound::Excluded),
            Bound::Unbounded,
        ));
        for (&coord, &band) in candidates.by_ref().take(usize::from(request.max_bricks)) {
            page_bricks.push((coord, band));
        }
        let has_more_bricks = candidates.next().map(|(&coord, _)| coord).is_some();

        let render_chunks = diagnostic_render_chunks(state, &page_bricks)?;
        let focuses = diagnostic_focuses(focus_state, &page_bricks)?;
        let next_after_brick = (has_more_bricks
            || (!page_bricks.is_empty()
                && focus_state.is_some_and(|focuses| !focuses.inactive_sources().is_empty())))
        .then(|| page_bricks.last().map(|(coord, _)| *coord))
        .flatten();
        let bricks = page_bricks
            .into_iter()
            .map(|(coord, band)| {
                diagnostic_brick(state, &focuses, coord, band, request.include_cells)
            })
            .collect();
        Ok(DiagnosticPage {
            snapshot,
            frame,
            revision: state.store.revision(),
            bricks,
            render_chunks,
            focuses,
            next_after_brick,
        })
    }
}

fn validate_diagnostic_request(request: DiagnosticPageRequest) -> Result<(), QueryError> {
    if request.max_bricks == 0 {
        return Err(QueryError::InvalidInput);
    }
    if request.max_bricks > 256 {
        return Err(QueryError::LimitExceeded(QueryLimitKind::DiagnosticBricks));
    }
    if request.include_cells && request.max_bricks > 2 {
        return Err(QueryError::LimitExceeded(QueryLimitKind::DiagnosticCells));
    }
    Ok(())
}

fn diagnostic_snapshot_token(
    state: &WorldReadState,
    focuses: Option<&FocusState>,
    frame: u64,
) -> DiagnosticSnapshotToken {
    DiagnosticSnapshotToken(
        state.diagnostic_generation,
        state.store.revision(),
        focuses.map_or(0, FocusState::generation),
        frame,
    )
}

fn diagnostic_snapshot_is_current(
    snapshot: DiagnosticSnapshotToken,
    state: &WorldReadState,
    focuses: Option<&FocusState>,
) -> bool {
    snapshot.0 == state.diagnostic_generation
        && snapshot.1 == state.store.revision()
        && snapshot.2 == focuses.map_or(0, FocusState::generation)
}

fn diagnostic_brick(
    state: &WorldReadState,
    focuses: &[DiagnosticFocus],
    coord: BrickCoord,
    band: ActiveBand,
    include_cells: bool,
) -> DiagnosticBrick {
    let origin = brick_origin(coord);
    let max = WorldPointQ8::new(
        origin.x + 16 * VOXEL_EDGE_Q8,
        origin.y + 16 * VOXEL_EDGE_Q8,
        origin.z + 16 * VOXEL_EDGE_Q8,
    );
    let mut purposes = FocusPurposeFlags::default();
    for focus in focuses {
        if focus
            .position
            .to_voxel_coord()
            .ok()
            .and_then(|point| point.to_brick_coord().ok())
            .as_ref()
            == Some(&coord)
        {
            purposes.insert(focus.purpose);
        }
    }
    let cells = include_cells.then(|| diagnostic_cells(state, coord));
    let status = state
        .diagnostic_statuses
        .get(&coord)
        .copied()
        .unwrap_or_default();
    DiagnosticBrick {
        coord,
        bounds: crate::AabbQ8::new(origin, max).expect("brick bounds are valid"),
        band,
        purposes,
        dirty: status.dirty,
        pin_count: status.pin_count,
        task: status.task,
        cells,
    }
}

fn diagnostic_cells(state: &WorldReadState, brick: BrickCoord) -> Vec<DiagnosticCell> {
    let origin = brick_origin(brick);
    let origin_voxel = VoxelCoord::new(
        origin.x / VOXEL_EDGE_Q8,
        origin.y / VOXEL_EDGE_Q8,
        origin.z / VOXEL_EDGE_Q8,
    );
    (0..4096)
        .map(|local_index| {
            let local_x = local_index % 16;
            let local_z = (local_index / 16) % 16;
            let local_y = local_index / 256;
            let voxel = state.store.current_voxel(VoxelCoord::new(
                origin_voxel.x + local_x,
                origin_voxel.y + local_y,
                origin_voxel.z + local_z,
            ));
            DiagnosticCell {
                local_index: local_index as u16,
                material: voxel.material,
                density: voxel.density,
                material_present: material_present(voxel),
                solid_collision: solid_collision(voxel, &state.materials),
                water_volume: water_volume(voxel),
            }
        })
        .collect()
}

fn brick_origin(coord: BrickCoord) -> WorldPointQ8 {
    WorldPointQ8::new(
        (-2_000 + i32::from(coord.x()) * 16) * VOXEL_EDGE_Q8,
        (-512 + i32::from(coord.y()) * 16) * VOXEL_EDGE_Q8,
        (-2_000 + i32::from(coord.z()) * 16) * VOXEL_EDGE_Q8,
    )
}

fn diagnostic_render_chunks(
    state: &WorldReadState,
    bricks: &[(BrickCoord, ActiveBand)],
) -> Result<Vec<DiagnosticRenderChunk>, QueryError> {
    let mut chunks = Vec::new();
    for (brick, _) in bricks {
        let first = DiagnosticRenderChunkKey {
            brick: *brick,
            lod: 0,
        };
        let last = DiagnosticRenderChunkKey {
            brick: *brick,
            lod: u8::MAX,
        };
        for chunk in state
            .render_chunks
            .range(first..=last)
            .map(|(_, chunk)| chunk)
        {
            if chunks.len() == 512 {
                return Err(QueryError::LimitExceeded(QueryLimitKind::DiagnosticChunks));
            }
            chunks.push(chunk.clone());
        }
    }
    Ok(chunks)
}

fn diagnostic_focuses(
    focuses: Option<&FocusState>,
    bricks: &[(BrickCoord, ActiveBand)],
) -> Result<Vec<DiagnosticFocus>, QueryError> {
    let Some(focuses) = focuses else {
        return Ok(Vec::new());
    };
    let mut page_focuses = Vec::new();
    if bricks.is_empty() {
        page_focuses.extend(
            focuses
                .inactive_sources()
                .values()
                .take(17)
                .map(diagnostic_focus),
        );
    } else {
        for (brick, _) in bricks {
            page_focuses.extend(
                focuses
                    .sources_at(*brick)
                    .into_iter()
                    .flat_map(|sources| sources.values().take(17))
                    .map(diagnostic_focus),
            );
        }
    }
    page_focuses.sort_unstable_by_key(|focus| focus.id);
    if page_focuses.len() > 16 {
        return Err(QueryError::LimitExceeded(QueryLimitKind::DiagnosticFocuses));
    }
    Ok(page_focuses)
}

fn diagnostic_focus(focus: &crate::FocusSource) -> DiagnosticFocus {
    DiagnosticFocus {
        id: focus.id,
        position: focus.position,
        purpose: focus.purpose,
    }
}

const fn sample_run_kind(material: u8) -> RunKind {
    if material == AIR.0 {
        RunKind::Air
    } else if material == WATER.0 {
        RunKind::Water
    } else {
        RunKind::Matter
    }
}

fn footprint_contains(body: &WaterBodyDef, x_q8: i32, z_q8: i32) -> bool {
    let point_count = body.footprint.len();
    if point_count < 3 {
        return false;
    }

    let mut inside = false;
    let mut previous = &body.footprint[point_count - 1];
    for current in &body.footprint {
        let crosses = (current.z > z_q8) != (previous.z > z_q8);
        if crosses {
            let left = i64::from(x_q8 - current.x) * i64::from(previous.z - current.z);
            let right = i64::from(previous.x - current.x) * i64::from(z_q8 - current.z);
            if (previous.z > current.z && left < right) || (previous.z < current.z && left > right)
            {
                inside = !inside;
            }
        }
        previous = current;
    }
    inside
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::{
        AIR, ActiveBand, BrickCoord, ColumnCoord, DiagnosticRenderChunk, DiagnosticRenderChunkKey,
        DiagnosticRenderChunkPhase, FeatureInstance, FeatureKind, FocusPurpose, FocusSource,
        GRANITE, MaterialRegistry, MoriaWorldPlugin, ObjectId, ObjectKind, ObjectPlacement,
        QuantizedTransform, QueryMask, RemoveFocusSource, RouteTag, RouteWaypoint, SetFocusSource,
        SpeciesId, Voxel, VoxelCoord, VoxelObjectShape, WaterBodyDef, WaterKind, WorldBounds,
        WorldIdentity, WorldLifecycle, WorldPointQ8, WorldRayQ8, evaluate_base_voxel,
    };

    use super::{TraversalRoute, WorldRead, WorldReadState};

    #[derive(Resource, Default)]
    struct SnapshotUnderTest {
        token: Option<crate::DiagnosticSnapshotToken>,
        phase: u8,
    }

    fn identity() -> WorldIdentity {
        WorldIdentity::new(
            0xD3E1_A5E5,
            [7; 32],
            WorldBounds::new(
                WorldPointQ8::new(-128_000, -32_768, -128_000),
                WorldPointQ8::new(128_000, 32_768, 128_000),
            )
            .unwrap(),
        )
    }

    fn ready_state() -> WorldReadState {
        WorldReadState::new(
            identity(),
            MaterialRegistry::default(),
            vec![WaterBodyDef {
                id: 4,
                kind: WaterKind::Lake,
                surface_y_q8: 100,
                footprint: vec![
                    WorldPointQ8::new(-100, 0, -100),
                    WorldPointQ8::new(100, 0, -100),
                    WorldPointQ8::new(100, 0, 100),
                    WorldPointQ8::new(-100, 0, 100),
                ],
                bed_profile_key: 0,
            }],
            TraversalRoute::new(vec![RouteWaypoint {
                order: 0,
                point: WorldPointQ8::new(0, 0, 0),
                tags: vec![RouteTag::Lake],
            }]),
        )
    }

    fn install_ready_state(app: &mut App, state: WorldReadState) {
        let mut lifecycle = WorldLifecycle::default();
        lifecycle.start_loading().unwrap();
        lifecycle.mark_ready().unwrap();
        app.insert_resource(state).insert_resource(lifecycle);
    }

    #[test]
    fn unavailable_world_reads_are_not_ready() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(MoriaWorldPlugin);
        app.add_systems(Update, |read: WorldRead| {
            assert!(matches!(
                read.sample_voxel(VoxelCoord::new(0, 0, 0)),
                Err(super::super::QueryError::NotReady)
            ));
            assert_eq!(
                read.diagnostic_snapshot(crate::DiagnosticPageRequest {
                    snapshot: None,
                    after_brick: None,
                    max_bricks: 1,
                    include_cells: false,
                }),
                Err(super::super::QueryError::NotReady)
            );
        });

        app.update();
    }

    #[test]
    fn installed_read_state_without_a_lifecycle_is_not_ready() {
        let mut app = App::new();
        app.insert_resource(ready_state());
        app.add_systems(Update, |read: WorldRead| {
            assert!(matches!(
                read.sample_voxel(VoxelCoord::new(0, 0, 0)),
                Err(super::super::QueryError::NotReady)
            ));
            assert_eq!(
                read.diagnostic_snapshot(crate::DiagnosticPageRequest {
                    snapshot: None,
                    after_brick: None,
                    max_bricks: 1,
                    include_cells: false,
                }),
                Err(super::super::QueryError::NotReady)
            );
        });

        app.update();
    }

    #[test]
    fn ready_reads_observe_inactive_base_deltas_and_metadata() {
        let mut app = App::new();
        let coordinate = VoxelCoord::new(0, 0, 0);
        let edited = Voxel::new(AIR, 0, 0, 0);
        let mut state = ready_state();
        state.store.commit_current([(coordinate, edited)]);
        state
            .active_bands
            .insert(BrickCoord::new(125, 32, 125).unwrap(), ActiveBand::Near);
        install_ready_state(&mut app, state);
        app.add_systems(Update, move |read: WorldRead| {
            assert_eq!(read.identity(), &identity());
            assert_eq!(read.bounds(), identity().bounds);
            assert_eq!(
                read.sample_voxel(VoxelCoord::new(1, 0, 0))
                    .unwrap()
                    .material,
                evaluate_base_voxel(&identity(), VoxelCoord::new(1, 0, 0)).material
            );
            let sample = read.sample_voxel(coordinate).unwrap();
            assert_eq!(sample.material, AIR);
            assert_eq!(sample.revision, 1);
            assert!(!sample.material_present);
            assert_eq!(
                read.sample_point(WorldPointQ8::new(-1, 0, -1))
                    .unwrap()
                    .coordinate,
                VoxelCoord::new(-1, 0, -1)
            );
            assert!(
                read.sample_column(ColumnCoord { x: 0, z: 0 })
                    .unwrap()
                    .runs
                    .iter()
                    .any(|run| run.kind == crate::RunKind::Air
                        && run.y_min_voxel <= 0
                        && run.y_max_voxel_exclusive > 0)
            );
            assert_eq!(read.route().waypoints().len(), 1);
            assert_eq!(read.water_surface_at(0, 0).unwrap().unwrap().body_id, 4);
            assert_eq!(
                read.active_band(BrickCoord::new(125, 32, 125).unwrap()),
                Some(ActiveBand::Near)
            );
        });

        app.update();
    }

    #[test]
    fn reads_reject_out_of_bounds_coordinates_before_sampling() {
        let mut app = App::new();
        install_ready_state(&mut app, ready_state());
        app.add_systems(Update, |read: WorldRead| {
            assert!(matches!(
                read.sample_voxel(VoxelCoord::new(2_000, 0, 0)),
                Err(super::super::QueryError::OutOfBounds)
            ));
            assert!(matches!(
                read.sample_point(WorldPointQ8::new(128_000, 0, 0)),
                Err(super::super::QueryError::OutOfBounds)
            ));
            assert!(matches!(
                read.sample_column(ColumnCoord { x: -2_001, z: 0 }),
                Err(super::super::QueryError::OutOfBounds)
            ));
            assert!(matches!(
                read.water_surface_at(128_000, 0),
                Err(super::super::QueryError::OutOfBounds)
            ));
        });

        app.update();
    }

    #[test]
    fn water_samples_do_not_report_water_outside_the_footprint() {
        let mut app = App::new();
        install_ready_state(&mut app, ready_state());
        app.add_systems(Update, |read: WorldRead| {
            assert!(read.water_surface_at(101, 0).unwrap().is_none());
            assert!(read.water_surface_at(0, 101).unwrap().is_none());
            assert!(
                !read
                    .sample_voxel(VoxelCoord::new(0, 0, 0))
                    .unwrap()
                    .water_volume
            );
        });

        app.update();
    }

    #[test]
    fn diagnostic_pages_reject_cell_requests_that_would_exceed_the_page_limit() {
        let mut app = App::new();
        install_ready_state(&mut app, ready_state());
        app.add_systems(Update, |read: WorldRead| {
            assert_eq!(
                read.diagnostic_snapshot(crate::DiagnosticPageRequest {
                    snapshot: None,
                    after_brick: None,
                    max_bricks: 3,
                    include_cells: true,
                }),
                Err(crate::QueryError::LimitExceeded(
                    crate::QueryLimitKind::DiagnosticCells
                ))
            );
        });

        app.update();
    }

    #[test]
    fn diagnostic_pages_are_ordered_bounded_and_include_every_requested_cell() {
        let mut state = ready_state();
        let first = BrickCoord::new(1, 0, 0).unwrap();
        let second = BrickCoord::new(2, 0, 0).unwrap();
        let third = BrickCoord::new(3, 0, 0).unwrap();
        state.active_bands.insert(third, ActiveBand::Horizon);
        state.active_bands.insert(first, ActiveBand::Near);
        state.active_bands.insert(second, ActiveBand::Middle);

        let mut app = App::new();
        install_ready_state(&mut app, state);
        app.add_systems(Update, move |read: WorldRead| {
            let first_page = read
                .diagnostic_snapshot(crate::DiagnosticPageRequest {
                    snapshot: None,
                    after_brick: None,
                    max_bricks: 2,
                    include_cells: true,
                })
                .unwrap();
            assert_eq!(
                first_page
                    .bricks
                    .iter()
                    .map(|brick| brick.coord)
                    .collect::<Vec<_>>(),
                vec![first, second]
            );
            assert!(first_page.bricks.iter().all(|brick| {
                brick
                    .cells
                    .as_ref()
                    .is_some_and(|cells| cells.len() == 4096)
            }));
            assert_eq!(first_page.next_after_brick, Some(second));

            let second_page = read
                .diagnostic_snapshot(crate::DiagnosticPageRequest {
                    snapshot: Some(first_page.snapshot),
                    after_brick: first_page.next_after_brick,
                    max_bricks: 2,
                    include_cells: false,
                })
                .unwrap();
            assert_eq!(second_page.bricks.len(), 1);
            assert_eq!(second_page.bricks[0].coord, third);
            assert!(second_page.bricks[0].cells.is_none());
            assert_eq!(second_page.next_after_brick, None);
        });

        app.update();
    }

    #[test]
    fn diagnostic_focus_limit_applies_to_the_requested_page() {
        let mut state = ready_state();
        let mut app = App::new();
        app.init_resource::<crate::streaming::FocusState>()
            .add_message::<SetFocusSource>()
            .add_message::<RemoveFocusSource>()
            .add_systems(Update, crate::streaming::apply_focus_messages);

        for x in 0..17 {
            let brick = BrickCoord::new(x, 0, 0).unwrap();
            state.active_bands.insert(brick, ActiveBand::Near);
            app.world_mut().write_message(SetFocusSource(FocusSource {
                id: x as u32,
                position: super::brick_origin(brick),
                purpose: FocusPurpose::Inspection,
            }));
        }

        install_ready_state(&mut app, state);
        app.add_systems(PostUpdate, |read: WorldRead| {
            let page = read
                .diagnostic_snapshot(crate::DiagnosticPageRequest {
                    snapshot: None,
                    after_brick: None,
                    max_bricks: 16,
                    include_cells: false,
                })
                .unwrap();
            assert_eq!(page.focuses.len(), 16);
            assert!(page.focuses.windows(2).all(|pair| pair[0].id < pair[1].id));
        });

        app.update();
    }

    #[test]
    fn diagnostic_pages_continue_to_the_terminal_inactive_focus_page() {
        let mut state = ready_state();
        let active_brick = BrickCoord::new(0, 0, 0).unwrap();
        let inactive_brick = BrickCoord::new(1, 0, 0).unwrap();
        state.active_bands.insert(active_brick, ActiveBand::Near);

        let mut app = App::new();
        app.init_resource::<crate::streaming::FocusState>()
            .add_message::<SetFocusSource>()
            .add_message::<RemoveFocusSource>()
            .add_systems(Update, crate::streaming::apply_focus_messages);
        app.world_mut().write_message(SetFocusSource(FocusSource {
            id: 1,
            position: super::brick_origin(active_brick),
            purpose: FocusPurpose::Inspection,
        }));
        app.world_mut().write_message(SetFocusSource(FocusSource {
            id: 2,
            position: super::brick_origin(inactive_brick),
            purpose: FocusPurpose::Camera,
        }));
        install_ready_state(&mut app, state);
        app.add_systems(PostUpdate, move |read: WorldRead| {
            let brick_page = read
                .diagnostic_snapshot(crate::DiagnosticPageRequest {
                    snapshot: None,
                    after_brick: None,
                    max_bricks: 1,
                    include_cells: false,
                })
                .unwrap();
            assert_eq!(brick_page.bricks.len(), 1);
            assert_eq!(
                brick_page
                    .focuses
                    .iter()
                    .map(|focus| focus.id)
                    .collect::<Vec<_>>(),
                [1]
            );
            assert_eq!(brick_page.next_after_brick, Some(active_brick));

            let terminal_page = read
                .diagnostic_snapshot(crate::DiagnosticPageRequest {
                    snapshot: Some(brick_page.snapshot),
                    after_brick: brick_page.next_after_brick,
                    max_bricks: 1,
                    include_cells: false,
                })
                .unwrap();
            assert!(terminal_page.bricks.is_empty());
            assert_eq!(
                terminal_page
                    .focuses
                    .iter()
                    .map(|focus| focus.id)
                    .collect::<Vec<_>>(),
                [2]
            );
            assert_eq!(terminal_page.next_after_brick, None);
        });

        app.update();
    }

    #[test]
    fn diagnostic_focus_pages_ignore_many_noncontributing_sources() {
        let mut state = ready_state();
        let requested_brick = BrickCoord::new(0, 0, 0).unwrap();
        let noncontributing_brick = BrickCoord::new(1, 0, 0).unwrap();
        state.active_bands.insert(requested_brick, ActiveBand::Near);

        let mut app = App::new();
        app.init_resource::<crate::streaming::FocusState>()
            .add_message::<SetFocusSource>()
            .add_message::<RemoveFocusSource>()
            .add_systems(Update, crate::streaming::apply_focus_messages);
        app.world_mut().write_message(SetFocusSource(FocusSource {
            id: 1,
            position: super::brick_origin(requested_brick),
            purpose: FocusPurpose::Inspection,
        }));
        for id in 2..=257 {
            app.world_mut().write_message(SetFocusSource(FocusSource {
                id,
                position: super::brick_origin(noncontributing_brick),
                purpose: FocusPurpose::Camera,
            }));
        }
        install_ready_state(&mut app, state);
        app.add_systems(PostUpdate, |read: WorldRead| {
            let page = read
                .diagnostic_snapshot(crate::DiagnosticPageRequest {
                    snapshot: None,
                    after_brick: None,
                    max_bricks: 1,
                    include_cells: false,
                })
                .unwrap();
            assert_eq!(
                page.focuses
                    .iter()
                    .map(|focus| focus.id)
                    .collect::<Vec<_>>(),
                [1]
            );
        });

        app.update();
    }

    #[test]
    fn diagnostic_chunk_limit_rejects_the_513th_chunk_without_truncation() {
        let mut state = ready_state();
        let bricks = [
            BrickCoord::new(0, 0, 0).unwrap(),
            BrickCoord::new(1, 0, 0).unwrap(),
            BrickCoord::new(2, 0, 0).unwrap(),
        ];
        let mut focuses = crate::streaming::FocusState::default();
        for brick in bricks {
            state.set_active_band(brick, Some(ActiveBand::Near), &mut focuses);
        }
        for brick in &bricks[..2] {
            for lod in 0..=u8::MAX {
                state.set_render_chunk(diagnostic_chunk(*brick, lod));
            }
        }
        state.set_render_chunk(diagnostic_chunk(bricks[2], 0));

        let mut app = App::new();
        install_ready_state(&mut app, state);
        app.add_systems(Update, |read: WorldRead| {
            let maximum = read
                .diagnostic_snapshot(crate::DiagnosticPageRequest {
                    snapshot: None,
                    after_brick: None,
                    max_bricks: 2,
                    include_cells: false,
                })
                .unwrap();
            assert_eq!(maximum.render_chunks.len(), 512);
            assert!(
                maximum
                    .render_chunks
                    .windows(2)
                    .all(|pair| pair[0].key < pair[1].key)
            );

            assert_eq!(
                read.diagnostic_snapshot(crate::DiagnosticPageRequest {
                    snapshot: None,
                    after_brick: None,
                    max_bricks: 3,
                    include_cells: false,
                }),
                Err(crate::QueryError::LimitExceeded(
                    crate::QueryLimitKind::DiagnosticChunks
                ))
            );
        });

        app.update();
    }

    fn diagnostic_chunk(brick: BrickCoord, lod: u8) -> DiagnosticRenderChunk {
        let min = super::brick_origin(brick);
        let max = WorldPointQ8::new(
            min.x + 16 * crate::VOXEL_EDGE_Q8,
            min.y + 16 * crate::VOXEL_EDGE_Q8,
            min.z + 16 * crate::VOXEL_EDGE_Q8,
        );
        DiagnosticRenderChunk {
            key: DiagnosticRenderChunkKey { brick, lod },
            bounds: crate::AabbQ8::new(min, max).unwrap(),
            lod,
            band: ActiveBand::Near,
            revision: 0,
            phase: DiagnosticRenderChunkPhase::Resident,
        }
    }

    #[test]
    fn ray_reads_committed_truth_without_exposing_storage() {
        let mut app = App::new();
        let mut state = ready_state();
        let y = 500;
        state.store.commit_current([
            (VoxelCoord::new(0, y, 0), Voxel::new(AIR, 0, 0, 0)),
            (VoxelCoord::new(1, y, 0), Voxel::new(AIR, 0, 0, 0)),
            (VoxelCoord::new(2, y, 0), Voxel::new(GRANITE, u8::MAX, 0, 0)),
        ]);
        install_ready_state(&mut app, state);
        app.add_systems(Update, move |read: WorldRead| {
            let ray = WorldRayQ8::new(WorldPointQ8::new(0, y * 64, 0), [65_536, 0, 0])
                .expect("axis direction is normalized");
            let hit = read
                .ray_cast(ray, 256, QueryMask::SOLID)
                .expect("bounded ray succeeds")
                .expect("committed solid is hit");
            assert_eq!(hit.voxel, VoxelCoord::new(2, y, 0));
            assert_eq!(hit.normal_q16, [-65_536, 0, 0]);
            assert_eq!(hit.distance_q8, 128);
            assert_eq!(hit.revision, 1);
        });

        app.update();
    }

    #[test]
    fn diagnostic_snapshot_tokens_expire_after_a_revision_change() {
        let mut app = App::new();
        install_ready_state(&mut app, ready_state());
        app.insert_resource(SnapshotUnderTest::default());
        app.add_systems(
            Update,
            |read: WorldRead, mut snapshot: ResMut<SnapshotUnderTest>| match snapshot.phase {
                0 => {
                    snapshot.token = Some(
                        read.diagnostic_snapshot(crate::DiagnosticPageRequest {
                            snapshot: None,
                            after_brick: None,
                            max_bricks: 1,
                            include_cells: false,
                        })
                        .unwrap()
                        .snapshot,
                    );
                    snapshot.phase = 1;
                }
                1 => {
                    assert_eq!(
                        read.diagnostic_snapshot(crate::DiagnosticPageRequest {
                            snapshot: snapshot.token,
                            after_brick: None,
                            max_bricks: 1,
                            include_cells: false,
                        }),
                        Err(crate::QueryError::SnapshotExpired)
                    );
                    snapshot.phase = 2;
                }
                _ => {}
            },
        );

        app.update();
        app.world_mut()
            .resource_mut::<WorldReadState>()
            .store
            .commit_current([(VoxelCoord::new(0, 0, 0), Voxel::new(AIR, 0, 0, 0))]);
        app.update();
        assert_eq!(app.world().resource::<SnapshotUnderTest>().phase, 2);
    }

    #[test]
    fn diagnostic_snapshot_tokens_remain_valid_across_frame_advances() {
        let mut app = App::new();
        install_ready_state(&mut app, ready_state());
        app.init_resource::<crate::telemetry::WorldTelemetryState>()
            .insert_resource(SnapshotUnderTest::default())
            .add_systems(
                Update,
                (
                    crate::telemetry::advance_frame_index,
                    |read: WorldRead, mut snapshot: ResMut<SnapshotUnderTest>| match snapshot.phase
                    {
                        0 => {
                            snapshot.token = Some(
                                read.diagnostic_snapshot(crate::DiagnosticPageRequest {
                                    snapshot: None,
                                    after_brick: None,
                                    max_bricks: 1,
                                    include_cells: false,
                                })
                                .unwrap()
                                .snapshot,
                            );
                            snapshot.phase = 1;
                        }
                        1 => {
                            assert!(
                                read.diagnostic_snapshot(crate::DiagnosticPageRequest {
                                    snapshot: snapshot.token,
                                    after_brick: None,
                                    max_bricks: 1,
                                    include_cells: false,
                                })
                                .is_ok()
                            );
                            snapshot.phase = 2;
                        }
                        _ => {}
                    },
                )
                    .chain(),
            );

        app.update();
        app.update();
        assert_eq!(app.world().resource::<SnapshotUnderTest>().phase, 2);
    }

    #[test]
    fn reads_overlay_registered_objects_and_generated_features_before_deltas() {
        let mut app = App::new();
        let mut state = ready_state();
        state.store.install_curated_truth(
            vec![FeatureInstance {
                id: 1,
                kind: FeatureKind::Stratum,
                bounds: crate::AabbQ8::new(
                    WorldPointQ8::new(0, 0, 0),
                    WorldPointQ8::new(256, 256, 256),
                )
                .unwrap(),
                host_material: GRANITE,
                depth_q8: 0,
                orientation_q16: [0, 0, 0, 65_536],
                generator_key: 0,
            }],
            vec![ObjectPlacement {
                id: ObjectId(1),
                kind: ObjectKind::TreeA,
                transform_q: QuantizedTransform {
                    translation: WorldPointQ8::new(128, 0, 128),
                    yaw_quarter_turns: 0,
                    uniform_scale_q8: 256,
                },
                species: Some(SpeciesId(1)),
                shape: VoxelObjectShape::Tree {
                    trunk_radius_q8: 128,
                    trunk_height_q8: 256,
                    canopy_radii_q8: [128, 128, 128],
                },
                anchor: VoxelCoord::new(2, 0, 2),
            }],
        );
        let coordinate = VoxelCoord::new(2, 0, 2);
        state
            .store
            .commit_current([(coordinate, Voxel::new(AIR, 0, 0, 0))]);
        install_ready_state(&mut app, state);
        app.add_systems(Update, move |read: WorldRead| {
            assert_eq!(
                read.sample_voxel(VoxelCoord::new(0, 0, 0))
                    .unwrap()
                    .material,
                GRANITE
            );
            assert_eq!(read.sample_voxel(coordinate).unwrap().material, AIR);
        });
        app.update();
    }
}
