//! Private read state and the public, immutable Bevy observation parameter.

use std::collections::BTreeMap;

use bevy::{ecs::system::SystemParam, prelude::*};

use crate::storage::WorldStore;
use crate::{
    AIR, BrickCoord, ColumnCoord, ColumnSample, MaterialRegistry, RunKind, VOXEL_EDGE_Q8,
    VoxelCoord, WATER, WaterBodyDef, WorldBounds, WorldIdentity, WorldPointQ8, evaluate_column,
};

use super::{ActiveBand, QueryError, QueryLimitKind, TraversalRoute, WaterSample, WorldSample};

/// Private authoritative state observed by [`WorldRead`].
#[derive(Resource)]
pub(crate) struct WorldReadState {
    store: WorldStore,
    materials: MaterialRegistry,
    water_bodies: Vec<WaterBodyDef>,
    route: TraversalRoute,
    active_bands: BTreeMap<BrickCoord, ActiveBand>,
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
        }
    }

    #[cfg(test)]
    pub(crate) fn commit_test_voxels(
        &mut self,
        changes: impl IntoIterator<Item = (VoxelCoord, crate::Voxel)>,
    ) {
        self.store.commit_current(changes);
    }
}

/// Read-only synchronous access to current authoritative world truth.
#[derive(SystemParam)]
pub struct WorldRead<'w, 's> {
    state: Option<Res<'w, WorldReadState>>,
    _system_state: Local<'s, ()>,
}

impl WorldRead<'_, '_> {
    pub(super) fn ready_bounds(&self) -> Result<WorldBounds, QueryError> {
        self.state
            .as_deref()
            .map(|state| state.store.identity().bounds)
            .ok_or(QueryError::NotReady)
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
