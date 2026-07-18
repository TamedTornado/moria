//! Private read state and the public, immutable Bevy observation parameter.

use std::collections::{BTreeMap, Bound};

use bevy::{ecs::system::SystemParam, prelude::*};

use crate::storage::WorldStore;
use crate::{
    AIR, BrickCoord, ColumnCoord, ColumnSample, MaterialRegistry, RunKind, VOXEL_EDGE_Q8,
    VoxelCoord, WATER, WaterBodyDef, WorldBounds, WorldIdentity, WorldPointQ8, evaluate_column,
    material_present, solid_collision, water_volume,
};

use super::{
    ActiveBand, DiagnosticBrick, DiagnosticCell, DiagnosticDirtyFlags, DiagnosticFocus,
    DiagnosticPage, DiagnosticPageRequest, DiagnosticSnapshotToken, FocusPurposeFlags, QueryError,
    QueryLimitKind, TraversalRoute, WaterSample, WorldSample,
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
    diagnostic_generation: u64,
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
            diagnostic_generation: 0,
        }
    }

    #[allow(
        dead_code,
        reason = "streaming installs and removes active bands once its lifecycle is available"
    )]
    pub(crate) fn set_active_band(&mut self, brick: BrickCoord, band: Option<ActiveBand>) {
        let previous = match band {
            Some(band) => self.active_bands.insert(brick, band),
            None => self.active_bands.remove(&brick),
        };
        if previous != band {
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
    _system_state: Local<'s, ()>,
}

impl WorldRead<'_, '_> {
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
        let state = self.state.as_deref().ok_or(QueryError::NotReady)?;
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
        let state = self.state.as_deref().ok_or(QueryError::NotReady)?;
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

    pub fn water_surface_at(
        &self,
        x_q8: i32,
        z_q8: i32,
    ) -> Result<Option<WaterSample>, QueryError> {
        let state = self.state.as_deref().ok_or(QueryError::NotReady)?;
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
        let state = self.state.as_deref().ok_or(QueryError::NotReady)?;
        let focus_state = self.focuses.as_deref();
        if focus_state.is_some_and(|focuses| focuses.sources().len() > 16) {
            return Err(QueryError::LimitExceeded(QueryLimitKind::DiagnosticFocuses));
        }
        let snapshot = diagnostic_snapshot_token(state, focus_state);
        if let Some(requested) = request.snapshot
            && requested != snapshot
        {
            return Err(QueryError::SnapshotExpired);
        }

        let mut bricks = Vec::with_capacity(usize::from(request.max_bricks));
        let mut candidates = state.active_bands.range((
            request
                .after_brick
                .map_or(Bound::Unbounded, Bound::Excluded),
            Bound::Unbounded,
        ));
        for (&coord, &band) in candidates.by_ref().take(usize::from(request.max_bricks)) {
            bricks.push(diagnostic_brick(
                state,
                focus_state,
                coord,
                band,
                request.include_cells,
            ));
        }
        let next_after_brick = candidates
            .next()
            .map(|(&coord, _)| coord)
            .and_then(|_| bricks.last().map(|brick| brick.coord));

        let focuses = if bricks.is_empty() {
            diagnostic_focuses(focus_state)
        } else {
            Vec::new()
        };
        Ok(DiagnosticPage {
            snapshot,
            frame: 0,
            revision: state.store.revision(),
            bricks,
            render_chunks: Vec::new(),
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
) -> DiagnosticSnapshotToken {
    DiagnosticSnapshotToken(
        state.diagnostic_generation,
        state.store.revision(),
        focuses.map_or(0, FocusState::generation),
    )
}

fn diagnostic_brick(
    state: &WorldReadState,
    focuses: Option<&FocusState>,
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
    if let Some(focuses) = focuses {
        for focus in focuses.sources().values() {
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
    }
    let cells = include_cells.then(|| diagnostic_cells(state, coord));
    DiagnosticBrick {
        coord,
        bounds: crate::AabbQ8::new(origin, max).expect("brick bounds are valid"),
        band,
        purposes,
        dirty: DiagnosticDirtyFlags::default(),
        pin_count: 0,
        task: None,
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

fn diagnostic_focuses(focuses: Option<&FocusState>) -> Vec<DiagnosticFocus> {
    let Some(focuses) = focuses else {
        return Vec::new();
    };
    focuses
        .sources()
        .values()
        .map(|focus| DiagnosticFocus {
            id: focus.id,
            position: focus.position,
            purpose: focus.purpose,
        })
        .collect()
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
        AIR, ActiveBand, BrickCoord, ColumnCoord, FeatureInstance, FeatureKind, GRANITE,
        MaterialRegistry, MoriaWorldPlugin, ObjectId, ObjectKind, ObjectPlacement,
        QuantizedTransform, RouteTag, RouteWaypoint, SpeciesId, Voxel, VoxelCoord,
        VoxelObjectShape, WaterBodyDef, WaterKind, WorldBounds, WorldIdentity, WorldPointQ8,
        evaluate_base_voxel,
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
        app.insert_resource(state);
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
        app.insert_resource(ready_state());
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
        app.insert_resource(ready_state());
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
        app.insert_resource(ready_state());
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
        app.insert_resource(state);
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
    fn diagnostic_snapshot_tokens_expire_after_a_revision_change() {
        let mut app = App::new();
        app.insert_resource(ready_state())
            .insert_resource(SnapshotUnderTest::default());
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
        app.insert_resource(state);
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
