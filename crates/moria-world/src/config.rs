//! Typed, file-independent Product One configuration contracts.
//!
//! The checked-in RON assets are intentionally owned by a later asset issue.  This module owns
//! their schema, normative defaults, and the authoritative digest boundary.

use serde::{Deserialize, Serialize};

use crate::config_validation::ConfigValidationError;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialId(pub u8);

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegionConfig {
    pub seed: u64,
    pub bounds: BoundsConfig,
    pub terrain: TerrainGenConfig,
    pub geology: GeologyConfig,
    pub cave: CaveConfig,
    pub water: WaterGenConfig,
    pub biome: BiomeConfig,
    pub objects: ObjectGenConfig,
    pub ruin_stamp: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BoundsConfig {
    pub x_min_m: i16,
    pub x_max_m: i16,
    pub z_min_m: i16,
    pub z_max_m: i16,
    pub y_min_m: i16,
    pub y_max_m: i16,
    pub voxel_edge_q8: u16,
    pub brick_edge_voxels: u8,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TerrainGenConfig {
    pub typical_surface_m: i16,
    pub broad_scale_m: u16,
    pub meander_scale_m: u16,
    pub relief_m: u8,
    pub topsoil_depth_q8: u16,
    pub subsoil_depth_q8: u16,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GeologyConfig {
    pub stratum_thickness_m: [u8; 4],
    pub tilt_degrees: i8,
    pub aquifer_thickness_m: u8,
    pub aquifer_material: MaterialId,
    pub iron_vein_radius_q8: u16,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CaveConfig {
    pub entrance_elevation_m: i16,
    pub entrance_tolerance_m: u8,
    pub floor_elevation_m: i16,
    pub floor_tolerance_m: u8,
    pub min_clear_width_q8: u16,
    pub min_clear_height_q8: u16,
    pub max_route_slope_degrees: u8,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WaterGenConfig {
    pub river_width_m: u8,
    pub river_depth_q8: u16,
    pub lake_min_diameter_m: u8,
    pub lake_depth_m: u8,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiomeConfig {
    pub meadow_min_area_m2: u32,
    pub forest_min_area_m2: u32,
    pub forest_tree_spacing_m: u8,
    pub tree_species_mix_percent: [u8; 2],
    pub bushes_per_hectare: u16,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectGenConfig {
    pub boulders_per_hectare: u16,
    pub stumps_per_hectare: u16,
    pub rocks_per_hectare: u16,
    pub max_anchor_slope_degrees: u8,
    pub route_clearance_m: u8,
    pub index_cell_size_m: u8,
    pub max_index_cells_per_object: u8,
    pub max_index_entries_per_cell: u16,
    pub sample_index_cell_size_m: u8,
    pub max_sample_cells_per_object: u8,
    pub max_sample_entries_per_cell: u8,
    pub max_edit_dependency_candidates: u16,
    pub max_affected_objects_per_edit: u8,
    pub max_dependency_bricks_per_object: u16,
    pub max_retained_index_bytes: u32,
    pub birch_trunk_radius_q8: [u16; 2],
    pub birch_height_m: [u8; 2],
    pub pine_trunk_radius_q8: [u16; 2],
    pub pine_height_m: [u8; 2],
    pub canopy_radius_q8: [u16; 2],
    pub bush_radius_q8: [u16; 2],
    pub boulder_radius_q8: [u16; 2],
    pub stump_radius_q8: [u16; 2],
    pub stump_height_q8: [u16; 2],
    pub rock_radius_q8: [u16; 2],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialRegistryConfig {
    pub definitions: Vec<MaterialDef>,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialDef {
    pub id: MaterialId,
    pub key: String,
    pub hardness: u8,
    pub granular: bool,
    pub collision: CollisionClass,
    pub surface: SurfaceClass,
    pub texture_layers: [u16; 3],
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CollisionClass {
    Empty,
    Fluid,
    Solid,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SurfaceClass {
    Empty,
    Water,
    Organic,
    Granular,
    Rock,
    Ore,
    OrganicObject,
    Masonry,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StreamingConfig {
    pub bands: [BandConfig; 4],
    pub hysteresis_m: u8,
    pub collision_radius_m: u8,
    pub vertical_surface_window_m: u8,
    pub prefetch_seconds_q8: u16,
    pub max_generation_jobs: u16,
    pub max_mesh_jobs: u16,
    pub max_install_bytes_per_frame: u32,
    pub edit_reserved_workers: u8,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BandConfig {
    pub min_distance_m: u16,
    pub max_distance_m: u16,
    pub lod_m: u8,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MutationConfig {
    pub fixed_hz: u16,
    pub debug_radius_q8: u16,
    pub min_radius_q8: u16,
    pub max_radius_q8: u16,
    pub dig_strength: u8,
    pub place_strength: u8,
    pub inner_full_strength_percent: u8,
    pub max_atomic_bricks: u16,
    pub max_progressive_bricks: u32,
    pub max_queued_edits: u8,
    pub max_commit_bricks_per_batch: u16,
    pub max_mutation_stage_ms_per_frame_q8: u16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerConfig {
    pub body: CapsuleConfig,
    pub run_speed_mps: f32,
    pub sprint_speed_mps: f32,
    pub ground_accel_mps2: f32,
    pub air_accel_mps2: f32,
    pub gravity_mps2: f32,
    pub jump_speed_mps: f32,
    pub step_height_m: f32,
    pub max_walk_slope_deg: f32,
    pub paddle_speed_mps: f32,
    pub paddle_surface_offset_m: f32,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CapsuleConfig {
    pub radius_m: f32,
    pub cylinder_height_m: f32,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CameraConfig {
    pub distance_m: f32,
    pub min_distance_m: f32,
    pub max_distance_m: f32,
    pub min_pitch_deg: f32,
    pub max_pitch_deg: f32,
    pub probe_radius_m: f32,
    pub collision_margin_m: f32,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LightConfig {
    pub depth_enable_m: f32,
    pub range_m: f32,
    pub intensity_lm: f32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InputConfig {
    pub bindings: Vec<ActionBinding>,
    pub stick_dead_zone: f32,
    pub mouse_sensitivity_degrees: f32,
    pub gamepad_orbit_degrees_per_second: f32,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActionBinding {
    pub action: InputAction,
    pub keyboard_mouse: String,
    pub gamepad: String,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum InputAction {
    Move,
    Sprint,
    Jump,
    Orbit,
    Zoom,
    Dig,
    Place,
    PreviousMaterial,
    NextMaterial,
    BrickBounds,
    RawVoxels,
    StreamingBands,
    TimeDown,
    TimeUp,
    TimeSliderFocus,
    Save,
    Load,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PresentationConfig {
    pub enabled: bool,
    pub rendering: RenderingConfig,
    pub streaming: StreamingConfig,
    pub mutation: MutationConfig,
    pub player: PlayerConfig,
    pub camera: CameraConfig,
    pub light: LightConfig,
    pub benchmark: BenchmarkConfig,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RenderingConfig {
    pub window_width: u32,
    pub window_height: u32,
    pub msaa_samples: u8,
    pub shadow_map_size: u32,
    pub time_of_day_hours: f32,
    pub time_min_hours: f32,
    pub time_max_hours: f32,
    pub time_keyboard_step_hours: f32,
    pub grass_normal_min_y: f32,
    pub grass_near_density_per_m2: f32,
    pub grass_middle_density_scale: f32,
    pub object_visibility_m: u16,
    pub cluster_visibility_m: u16,
    pub horizon_object_cell_size_m: u8,
    pub horizon_derived_lod_m: f32,
    pub max_horizon_tree_members_per_cell: u16,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkConfig {
    pub warmup_frames: u16,
    pub flythrough_duration_s: u16,
    pub colony_worker_streams: u8,
    pub colony_volume_m: [u8; 3],
    pub catastrophic_radius_q8: u16,
    pub watchdog_s: u16,
    pub fps_target: f32,
    pub max_mutation_frame_ms: f32,
    pub max_admission_ms: f32,
    pub max_first_commit_interactive_ms: u16,
    pub max_first_commit_colony_ms: u16,
    pub max_primary_ready_p95_ms: u16,
    pub max_primary_ready_ms: u16,
    pub min_changed_bricks_per_second: u16,
    pub max_runnable_wait_ms: u16,
    pub max_reconciliation_interactive_ms: u16,
    pub max_reconciliation_volume_ms: u32,
    pub cold_start_max_ms: u16,
    pub graphics_memory_max_bytes: u64,
}

/// The complete independent config assets. Only `region` is authoritative world identity.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigSet {
    pub region: RegionConfig,
    pub materials: MaterialRegistryConfig,
    pub presentation: PresentationConfig,
    pub input: InputConfig,
}

impl ConfigSet {
    pub fn validate(&self) -> Result<(), ConfigValidationError> {
        crate::config_validation::validate_config_set(self)
    }
    /// Stable RON bytes for world identity; presentation and controls are deliberately excluded.
    pub fn authoritative_ron(&self) -> Result<String, ron::Error> {
        ron::ser::to_string(&self.region)
    }
}
impl RegionConfig {
    pub fn validate(&self) -> Result<(), ConfigValidationError> {
        crate::config_validation::validate_region_config(self)
    }
}

impl Default for RegionConfig {
    fn default() -> Self {
        Self {
            seed: 0x4D4F_5249_415F_5031,
            bounds: BoundsConfig {
                x_min_m: -500,
                x_max_m: 500,
                z_min_m: -500,
                z_max_m: 500,
                y_min_m: -128,
                y_max_m: 128,
                voxel_edge_q8: 64,
                brick_edge_voxels: 16,
            },
            terrain: TerrainGenConfig {
                typical_surface_m: 64,
                broad_scale_m: 220,
                meander_scale_m: 72,
                relief_m: 34,
                topsoil_depth_q8: 256,
                subsoil_depth_q8: 768,
            },
            geology: GeologyConfig {
                stratum_thickness_m: [8, 12, 10, 18],
                tilt_degrees: 18,
                aquifer_thickness_m: 6,
                aquifer_material: MaterialId(5),
                iron_vein_radius_q8: 320,
            },
            cave: CaveConfig {
                entrance_elevation_m: 0,
                entrance_tolerance_m: 2,
                floor_elevation_m: -40,
                floor_tolerance_m: 2,
                min_clear_width_q8: 768,
                min_clear_height_q8: 768,
                max_route_slope_degrees: 35,
            },
            water: WaterGenConfig {
                river_width_m: 10,
                river_depth_q8: 512,
                lake_min_diameter_m: 80,
                lake_depth_m: 6,
            },
            biome: BiomeConfig {
                meadow_min_area_m2: 40_000,
                forest_min_area_m2: 120_000,
                forest_tree_spacing_m: 5,
                tree_species_mix_percent: [55, 45],
                bushes_per_hectare: 450,
            },
            objects: ObjectGenConfig {
                boulders_per_hectare: 24,
                stumps_per_hectare: 14,
                rocks_per_hectare: 90,
                max_anchor_slope_degrees: 32,
                route_clearance_m: 3,
                index_cell_size_m: 32,
                max_index_cells_per_object: 16,
                max_index_entries_per_cell: 1024,
                sample_index_cell_size_m: 4,
                max_sample_cells_per_object: 16,
                max_sample_entries_per_cell: 64,
                max_edit_dependency_candidates: 256,
                max_affected_objects_per_edit: 64,
                max_dependency_bricks_per_object: 128,
                max_retained_index_bytes: 16_777_216,
                birch_trunk_radius_q8: [51, 90],
                birch_height_m: [8, 14],
                pine_trunk_radius_q8: [64, 115],
                pine_height_m: [10, 18],
                canopy_radius_q8: [512, 1024],
                bush_radius_q8: [128, 307],
                boulder_radius_q8: [128, 461],
                stump_radius_q8: [64, 141],
                stump_height_q8: [64, 192],
                rock_radius_q8: [38, 154],
            },
            ruin_stamp: "stamps/ruin_p1.ron".into(),
        }
    }
}

impl Default for MaterialRegistryConfig {
    fn default() -> Self {
        let rows = [
            ("air", 0, false, CollisionClass::Empty, SurfaceClass::Empty),
            (
                "water",
                0,
                false,
                CollisionClass::Fluid,
                SurfaceClass::Water,
            ),
            (
                "topsoil",
                28,
                false,
                CollisionClass::Solid,
                SurfaceClass::Organic,
            ),
            (
                "subsoil",
                42,
                false,
                CollisionClass::Solid,
                SurfaceClass::Organic,
            ),
            (
                "sand",
                18,
                true,
                CollisionClass::Solid,
                SurfaceClass::Granular,
            ),
            (
                "gravel",
                35,
                true,
                CollisionClass::Solid,
                SurfaceClass::Granular,
            ),
            (
                "limestone",
                82,
                false,
                CollisionClass::Solid,
                SurfaceClass::Rock,
            ),
            (
                "sandstone",
                68,
                false,
                CollisionClass::Solid,
                SurfaceClass::Rock,
            ),
            (
                "shale",
                60,
                false,
                CollisionClass::Solid,
                SurfaceClass::Rock,
            ),
            (
                "granite",
                120,
                false,
                CollisionClass::Solid,
                SurfaceClass::Rock,
            ),
            (
                "iron_ore",
                135,
                false,
                CollisionClass::Solid,
                SurfaceClass::Ore,
            ),
            (
                "wood",
                55,
                false,
                CollisionClass::Solid,
                SurfaceClass::OrganicObject,
            ),
            (
                "leaf",
                8,
                false,
                CollisionClass::Solid,
                SurfaceClass::OrganicObject,
            ),
            (
                "cut_stone",
                100,
                false,
                CollisionClass::Solid,
                SurfaceClass::Masonry,
            ),
        ];
        Self {
            definitions: rows
                .into_iter()
                .enumerate()
                .map(
                    |(id, (key, hardness, granular, collision, surface))| MaterialDef {
                        id: MaterialId(id as u8),
                        key: key.into(),
                        hardness,
                        granular,
                        collision,
                        surface,
                        texture_layers: [id as u16; 3],
                    },
                )
                .collect(),
        }
    }
}
impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            bands: [
                BandConfig {
                    min_distance_m: 0,
                    max_distance_m: 64,
                    lod_m: 0,
                },
                BandConfig {
                    min_distance_m: 64,
                    max_distance_m: 160,
                    lod_m: 0,
                },
                BandConfig {
                    min_distance_m: 160,
                    max_distance_m: 320,
                    lod_m: 1,
                },
                BandConfig {
                    min_distance_m: 320,
                    max_distance_m: 720,
                    lod_m: 4,
                },
            ],
            hysteresis_m: 12,
            collision_radius_m: 12,
            vertical_surface_window_m: 12,
            prefetch_seconds_q8: 512,
            max_generation_jobs: 96,
            max_mesh_jobs: 64,
            max_install_bytes_per_frame: 16_777_216,
            edit_reserved_workers: 2,
        }
    }
}
impl Default for MutationConfig {
    fn default() -> Self {
        Self {
            fixed_hz: 60,
            debug_radius_q8: 768,
            min_radius_q8: 64,
            max_radius_q8: 4096,
            dig_strength: 255,
            place_strength: 255,
            inner_full_strength_percent: 70,
            max_atomic_bricks: 32,
            max_progressive_bricks: 8192,
            max_queued_edits: 32,
            max_commit_bricks_per_batch: 8,
            max_mutation_stage_ms_per_frame_q8: 1024,
        }
    }
}
impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            body: CapsuleConfig {
                radius_m: 0.4,
                cylinder_height_m: 1.0,
            },
            run_speed_mps: 5.0,
            sprint_speed_mps: 8.0,
            ground_accel_mps2: 35.0,
            air_accel_mps2: 10.0,
            gravity_mps2: 22.0,
            jump_speed_mps: 7.0,
            step_height_m: 0.3,
            max_walk_slope_deg: 48.0,
            paddle_speed_mps: 3.0,
            paddle_surface_offset_m: 0.55,
        }
    }
}
impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            distance_m: 5.5,
            min_distance_m: 2.0,
            max_distance_m: 9.0,
            min_pitch_deg: -65.0,
            max_pitch_deg: 75.0,
            probe_radius_m: 0.18,
            collision_margin_m: 0.12,
        }
    }
}
impl Default for LightConfig {
    fn default() -> Self {
        Self {
            depth_enable_m: 2.0,
            range_m: 18.0,
            intensity_lm: 1600.0,
        }
    }
}
impl Default for RenderingConfig {
    fn default() -> Self {
        Self {
            window_width: 2560,
            window_height: 1440,
            msaa_samples: 4,
            shadow_map_size: 2048,
            time_of_day_hours: 14.0,
            time_min_hours: 6.0,
            time_max_hours: 20.0,
            time_keyboard_step_hours: 0.25,
            grass_normal_min_y: 0.75,
            grass_near_density_per_m2: 5.0,
            grass_middle_density_scale: 0.25,
            object_visibility_m: 320,
            cluster_visibility_m: 720,
            horizon_object_cell_size_m: 64,
            horizon_derived_lod_m: 4.0,
            max_horizon_tree_members_per_cell: 1024,
        }
    }
}
impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            warmup_frames: 300,
            flythrough_duration_s: 120,
            colony_worker_streams: 8,
            colony_volume_m: [32, 32, 16],
            catastrophic_radius_q8: 4096,
            watchdog_s: 300,
            fps_target: 60.0,
            max_mutation_frame_ms: 33.3,
            max_admission_ms: 2.0,
            max_first_commit_interactive_ms: 100,
            max_first_commit_colony_ms: 250,
            max_primary_ready_p95_ms: 250,
            max_primary_ready_ms: 500,
            min_changed_bricks_per_second: 32,
            max_runnable_wait_ms: 500,
            max_reconciliation_interactive_ms: 1000,
            max_reconciliation_volume_ms: 30_000,
            cold_start_max_ms: 5000,
            graphics_memory_max_bytes: 2_097_152_000,
        }
    }
}
impl Default for PresentationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rendering: RenderingConfig::default(),
            streaming: StreamingConfig::default(),
            mutation: MutationConfig::default(),
            player: PlayerConfig::default(),
            camera: CameraConfig::default(),
            light: LightConfig::default(),
            benchmark: BenchmarkConfig::default(),
        }
    }
}
impl Default for InputConfig {
    fn default() -> Self {
        let rows = [
            (InputAction::Move, "W/A/S/D", "left_stick"),
            (InputAction::Sprint, "left_shift", "left_stick_press"),
            (InputAction::Jump, "space", "south"),
            (InputAction::Orbit, "captured_mouse", "right_stick"),
            (InputAction::Zoom, "mouse_wheel", "triggers"),
            (InputAction::Dig, "G/left_mouse", "right_shoulder"),
            (InputAction::Place, "P/right_mouse", "left_shoulder"),
            (InputAction::PreviousMaterial, "[", "dpad_left"),
            (InputAction::NextMaterial, "]", "dpad_right"),
            (InputAction::BrickBounds, "F1", ""),
            (InputAction::RawVoxels, "F2", ""),
            (InputAction::StreamingBands, "F3", ""),
            (InputAction::TimeDown, "-", "debug_dpad_down"),
            (InputAction::TimeUp, "=", "debug_dpad_up"),
            (InputAction::TimeSliderFocus, "Tab", ""),
            (InputAction::Save, "F5", ""),
            (InputAction::Load, "F9", ""),
        ];
        Self {
            bindings: rows
                .into_iter()
                .map(|(action, keyboard_mouse, gamepad)| ActionBinding {
                    action,
                    keyboard_mouse: keyboard_mouse.into(),
                    gamepad: gamepad.into(),
                })
                .collect(),
            stick_dead_zone: 0.15,
            mouse_sensitivity_degrees: 0.12,
            gamepad_orbit_degrees_per_second: 150.0,
        }
    }
}
