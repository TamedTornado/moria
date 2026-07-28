# Public Consumer Contract

The signatures below are normative API shapes. Exact module paths may be
refined during implementation, but ownership, milestones, errors, and bounds
must remain.

## Identities and revisions

```rust
pub struct MoriaHandle { /* Send + Sync */ }
pub struct WorldHandle { /* Send + Sync, opaque generation */ }
pub struct ValidatedMoria { /* consumed installation plan */ }
pub struct MoriaPlugin { /* Bevy Plugin */ }
pub struct CommandPermit { /* owned queue reservation */ }
pub struct QueryPermit { /* owned queue reservation */ }
pub struct CheckpointPermit { /* owned queue reservation */ }
pub struct BehaviorTickPermit {
    /*
    one tick + every declared input/view/collision/handoff/effect/feedback,
    component-extraction, placement-stream, and egress maximum
    */
}
pub struct ExtensionPermit { /* owned queue/job reservation */ }
pub struct EffectBatchPermit { /* owned child-command batch reservation */ }

pub struct WorldId(u64);
pub struct VolumeId(u64);
#[repr(transparent)]
pub struct MaterialId(u16);
pub struct InterestId(u64);
pub struct SubscriberId(u64);
pub struct CommandId(u64);
pub struct QueryId(u64);
pub struct BehaviorTickId(u64);
pub struct BehaviorEngineId(u32);
pub struct ExtensionId(u64);
pub struct GpuStateId(u64);
pub struct DressingStyleId(u32);
pub struct OperationId(u64);

pub struct WorldKey(uuid::Uuid);
pub struct VolumeKey(uuid::Uuid);
pub struct MaterialKey(uuid::Uuid);
pub struct BehaviorEngineKey(uuid::Uuid);
pub struct ExtensionKey(uuid::Uuid);
pub struct DressingStyleKey(uuid::Uuid);
pub struct CheckpointKey(uuid::Uuid);

pub struct VolumeRevision(NonZeroU64);
pub struct ObservationSequence(NonZeroU64);
pub struct DeviceGeneration(NonZeroU64);

pub enum ObservationFrontier {
    Empty,
    Retained {
        oldest: ObservationSequence,
        head: ObservationSequence,
    },
}
```

Runtime IDs are process-local generational handles. A stale ID is rejected
before GPU work. Stable keys for externally registered records are
consumer-supplied and persisted; source-bound derived children receive the
Moria-generated, persisted `VolumeKey` defined in
[adapter-substrate-contracts.md](adapter-substrate-contracts.md). Numeric
runtime IDs and physical slot numbers are never durable.

Observation sequences start at one. `ObservationFrontier::Empty` means no fact
has ever been appended. Once the first fact is appended, the frontier is
`Retained { oldest, head }`, with `oldest <= head`; the configured ring always
retains its newest fact, so a world never returns to `Empty`. Zero is therefore
available as the ABI encoding of an absent sequence, but is never a Rust
`ObservationSequence`. Startup does not synthesize a fact merely to make this
frontier nonempty.

## Configuration

```rust
pub struct MoriaBuilder { /* registrations */ }

impl MoriaBuilder {
    pub fn new(world: WorldDefinition, config: MoriaConfig) -> Self;
    pub fn register_material(
        &mut self,
        definition: MaterialDefinition,
    ) -> Result<MaterialId, RegistrationError>;
    pub fn register_dressing_style(
        &mut self,
        descriptor: DressingDescriptor,
    ) -> Result<DressingStyleId, RegistrationError>;
    pub fn register_volume(
        &mut self,
        definition: VolumeDefinition,
        source: Arc<dyn BaseContentSource>,
    ) -> Result<VolumeId, RegistrationError>;
    pub fn register_cpu_behavior(
        &mut self,
        descriptor: BehaviorEngineDescriptor,
        planner: Box<dyn BehaviorAccessPlanner>,
        adapter: Box<dyn CpuBehaviorEngine>,
    ) -> Result<BehaviorEngineId, RegistrationError>;
    pub fn register_gpu_behavior(
        &mut self,
        descriptor: BehaviorEngineDescriptor,
        planner: Box<dyn BehaviorAccessPlanner>,
        adapter: Box<dyn GpuBehaviorEngine>,
    ) -> Result<BehaviorEngineId, RegistrationError>;
    pub fn checkpoint_store(
        &mut self,
        store: Arc<dyn CheckpointStore>,
    ) -> &mut Self;
    pub fn restore_from(
        &mut self,
        request: RestoreRequest,
    ) -> Result<&mut Self, RegistrationError>;
    pub fn validate(&self) -> Result<ValidatedMoria, ConfigurationErrors>;
}

pub struct WorldDefinition {
    pub key: WorldKey,
    pub debug_name: String,              // 1..=96 UTF-8 bytes
}

pub struct BevyInstallation {
    pub plugin: MoriaPlugin,
    pub moria: MoriaHandle,
    pub world: WorldHandle,
    pub startup: Receipt<StartupApplied>,
}

impl ValidatedMoria {
    pub fn into_bevy(self) -> BevyInstallation;
}

pub struct StartupApplied {
    pub world: WorldId,
    pub key: WorldKey,
    pub effective_config: EffectiveConfig,
    pub adapter: AdapterCapabilityReport,
    pub mode: StartupModeApplied,
}

pub enum StartupModeApplied {
    Fresh,
    Restored(RestoreApplied),
}
```

The installation handles exist in `Configured` state so they can be inserted
into consumer resources before `App::add_plugins(installation.plugin)`.
`startup` becomes ready only after the plugin is installed and startup or
restore reaches `Ready`; submitting through the world earlier returns
`WorldNotAccepting`. `ValidatedMoria` is consumed exactly once. The
`test-support` driver consumes it through the same internal installation
routine and exposes no additional consumer operation.

Registration does not allocate GPU storage or invoke a source. Duplicate
stable keys, invalid domains, missing material references, invalid
fingerprints, and impossible limits are registration/configuration failures.

```rust
pub struct MaterialDefinition {
    pub key: MaterialKey,
    pub debug_name: String,              // 1..=96 UTF-8 bytes
    pub presentation: SurfaceDescriptor,
    pub opaque_metadata: Vec<u8>,        // <= config.limits.max_material_metadata_bytes
}

pub struct SurfaceDescriptor {
    pub class: SurfaceClass,
    pub material: SurfaceMaterialInput,
    pub triplanar: Option<TriplanarTextures>,
    pub tint_linear_rgba: [f32; 4],
    pub roughness: f32,                   // finite, 0..=1
    pub metallic: f32,                    // finite, 0..=1
}

pub enum SurfaceMaterialInput {
    Bevy(bevy::asset::Handle<bevy::pbr::StandardMaterial>),
    NeutralDiagnostic,                   // only used when fallback policy permits
}

pub struct TriplanarTextures {
    pub albedo: bevy::asset::Handle<bevy::image::Image>,
    pub normal: Option<bevy::asset::Handle<bevy::image::Image>>,
    pub meters_per_repeat: f32,           // finite and > 0
}

pub enum SurfaceClass {
    Organic,
    Constructed,
}

pub struct DressingDescriptor {
    pub key: DressingStyleKey,
    pub debug_name: String,              // 1..=96 UTF-8 bytes
    pub materials: Vec<MaterialKey>,      // sorted unique, 1..=64
    pub mesh: bevy::asset::Handle<bevy::mesh::Mesh>,
    pub material: bevy::asset::Handle<bevy::pbr::StandardMaterial>,
    pub density_per_square_meter: f32,   // finite, 0..=4,096
    pub coverage_range: [u8; 2],         // inclusive, 128 <= min <= max
    pub scale_range: [f32; 2],           // finite, 0 < min <= max <= 64
    pub yaw_range_radians: [f32; 2],     // finite, min <= max
    pub normal_offset: f32,              // finite local meters
    pub max_instances_per_artifact: u32, // 1..=4,096
}

pub struct VolumeDefinition {
    pub key: VolumeKey,
    pub debug_name: String,              // 1..=96 UTF-8 bytes
    pub domain: CellAabb,                // finite, min inclusive/max exclusive
    pub cell_size: f32,                  // finite and > 0
    pub mode: VolumeMode,                // Static | Dynamic
    pub initial_placement: RigidPlacement,
    pub lineage: ContentLineage,
    pub reconstruction: ReconstructionFingerprint,
}
```

All debug-name limits are measured in UTF-8 bytes. A validated builder copies
each accepted volume name into an exact-length `Box<str>` directory record and
drops the input `String`, so an input vector capacity is never retained.

Opaque metadata is returned only through material registry inspection. Moria
does not interpret it and does not upload it to occupancy kernels.

Surface inputs are embedded by value in each material registration. Dressing
uses a separate builder-time registry because one style may filter several
materials and owns independent mesh/material assets. `materials` is the exact
material filter; every key must resolve by `validate`, and no material outside
that list receives the style. Duplicate style keys, exhausted
`dressing_styles`, invalid ranges, unknown material keys, or a descriptor whose
per-artifact maximum exceeds the configured instance pool are structured
`RegistrationError`/`ConfigurationErrors`. V1 has no runtime presentation
registration or mutation.

Asset handles may be registered before their assets finish loading. A missing
or removed asset makes the affected artifact
`PresentationError::AssetUnavailable { style, asset_kind }`; it does not fail
material truth or collision. `NeutralDiagnostic` is rendered only when the
consumer selected diagnostic fallback.

```rust
pub enum RegistrationError {
    CapabilityDisabled,
    DuplicateMaterial(MaterialKey),
    DuplicateVolume(VolumeKey),
    DuplicateDressingStyle(DressingStyleKey),
    MaterialCapacity { limit: u32 },
    MaterialMetadataCapacity { requested: u64, available: u64 },
    LiveVolumeCapacity { limit: u32 },
    VolumeRecordCapacity { limit: u32 },
    DressingStyleCapacity { limit: u32 },
    BehaviorEngineCapacity { limit: u32 },
    BehaviorOrderCapacity { limit: u32 },
    BehaviorGpuBufferCapacity { requested: u64, available: u64 },
    BehaviorOrderCycle { cycle: Vec<BehaviorEngineKey> },
    DuplicateBehaviorEngine(BehaviorEngineKey),
    InvalidDefinition(Vec<Violation>),
}

pub struct ConfigurationErrors {
    pub violations: Vec<Violation>,      // deterministic field-path order
}

pub struct StaleHandleError {
    pub kind: StaleHandleKind,
}

pub enum StaleHandleKind {
    World,
    Volume,
    Material,
    Subscriber,
    Extension,
    GpuState,
}

pub struct ShaderDiagnostic {
    pub stage: ShaderStage,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub message: String,                 // <=1,024 UTF-8 bytes
}

pub enum ShaderStage {
    Parse,
    Validate,
    Pipeline,
}
```

### Configuration schema

Every field below is public and constructible. `MoriaConfig::default()` supplies
the stated defaults; zero is invalid except where a capability or the optional
dressing subfeature is explicitly disabled below.
Count fields are `u32`, byte fields are `u64`, and conversion to platform
`usize` is checked.

```rust
pub struct MoriaConfig {
    pub capabilities: CapabilityConfig,
    pub limits: ResourceLimits,
    pub overload: OverloadPolicies,
    pub workers: WorkerConfig,
    pub presentation: PresentationConfig,
}

pub struct CapabilityConfig {
    pub presentation: bool,              // default true
    pub persistence: bool,               // default false; must be explicitly enabled
    pub behavior_hooks: bool,             // default false
    pub gpu_extensions: bool,             // default false; also needs feature
}

pub enum OverloadPolicy {
    Reject,
    WaitForPermit,
}

pub struct OverloadPolicies {
    pub commands: OverloadPolicy,         // default WaitForPermit
    pub queries: OverloadPolicy,          // default WaitForPermit
    pub checkpoints: OverloadPolicy,      // default Reject
    pub behavior_ticks: OverloadPolicy,   // default Reject
    pub extensions: OverloadPolicy,       // default Reject
}

pub struct WorkerConfig {
    pub content_threads: NonZeroU8,       // default 2, legal 1..=8
    pub persistence_threads: NonZeroU8,   // default 1, legal 1..=8
}

pub struct GpuCapacityLimit {
    pub desired: u32,
    pub minimum: u32,
}

pub struct EffectiveConfig {
    pub requested: MoriaConfig,
    pub capabilities: CapabilityConfig,
    pub overload: OverloadPolicies,
    pub workers: WorkerConfig,
    pub presentation: PresentationConfig,
    pub limits: Vec<EffectiveLimit>,     // exactly one per ResourceKind
}

pub struct EffectiveLimit {
    pub resource: ResourceKind,
    pub requested: u64,
    pub minimum: Option<u64>,
    pub effective: u64,
    pub disposition: LimitDisposition,
}

pub enum LimitDisposition {
    Exact,
    AdapterClamped { adapter_max: u64 },
}

pub struct AdapterCapabilityReport {
    pub backend: String,
    pub adapter: String,
    pub driver: String,
    pub device_type: String,
    pub features: Vec<String>,
    pub downlevel_flags: Vec<String>,
    pub numeric_limits: Vec<(String, u64)>,
    pub software_fallback: bool,
}

pub struct ResourceLimits {
    pub nonempty_materials: u32,
    pub max_material_metadata_bytes: u32,
    pub material_metadata_bytes: u64,
    pub live_volumes: u32,
    pub volume_records: u32,
    pub interest_leases: u32,
    pub bricks_per_interest: u32,
    pub detailed_bricks: GpuCapacityLimit,
    pub page_keys: GpuCapacityLimit,
    pub page_versions: GpuCapacityLimit,
    pub versions_per_brick: u32,
    pub dirty_scar_bricks: GpuCapacityLimit,
    pub command_records: u32,
    pub command_payload_bytes: u64,
    pub query_records: u32,
    pub query_result_bytes: u64,
    pub observation_facts: u32,
    pub observation_payload_bytes: u64,
    pub subscribers: u32,
    pub volumes_per_filter: u32,
    pub staging_maps: u32,
    pub staging_bytes: GpuCapacityLimit, // bytes; v1 hard max fits u32
    pub content_requests: u32,
    pub content_bricks_per_request: u32,
    pub content_response_bytes: u64,
    pub persistence_requests: u32,
    pub persistence_staged_bytes: u64,
    pub extraction_records: u32,
    pub extraction_bytes: u64,
    pub presentation_jobs: u32,
    pub presentation_artifacts: u32,
    pub presentation_dirty_records: u32,
    pub mesh_vertices: u32,
    pub mesh_indices: u32,
    pub dressing_styles: u32,
    pub dressing_instances: GpuCapacityLimit,
    pub behavior_engines: u32,
    pub behavior_order_edges: u32,
    pub behavior_scopes_per_engine: u32,
    pub behavior_view_volumes: u32,
    pub behavior_view_bricks: u32,
    pub behavior_view_cells: u32,
    pub behavior_cpu_view_bytes: u64,
    pub behavior_gpu_view_bytes: GpuCapacityLimit,
    pub behavior_input_records: u32,
    pub behavior_input_bytes: u64,
    pub behavior_gpu_input_bytes: GpuCapacityLimit,
    pub behavior_collision_calls: u32,
    pub behavior_collision_contacts: u32,
    pub behavior_collision_bytes: u64,
    pub behavior_handoff_maps: u32,
    pub behavior_handoff_bytes: GpuCapacityLimit,
    pub behavior_proposal_records: u32,
    pub behavior_proposal_bytes: u64,
    pub behavior_effect_cells: u32,
    pub behavior_effect_bricks: u32,
    pub behavior_directory_effects: u32,
    pub behavior_conflict_checks: u64,
    pub behavior_feedback_bytes: GpuCapacityLimit,
    pub behavior_gpu_buffers: u32,
    pub behavior_gpu_buffer_bytes: GpuCapacityLimit,
    pub behavior_gpu_pipelines: u32,
    pub behavior_gpu_bind_groups: u32,
    pub behavior_gpu_wgsl_bytes: u64,
    pub behavior_gpu_dispatches: u32,
    pub behavior_gpu_workgroups: u64,
    pub behavior_placement_updates: u32,
    pub behavior_placement_bytes: u64,
    pub behavior_component_extraction_proposals: u32,
    pub behavior_component_extraction_children: u32,
    pub behavior_component_extraction_assignment_cells: u32,
    pub behavior_component_extraction_child_bricks: u32,
    pub behavior_component_extraction_bytes: u64,
    pub behavior_egress_maps: u32,
    pub behavior_egress_receipts: u32,
    pub behavior_egress_records: u32,
    pub behavior_egress_device_bytes: GpuCapacityLimit,
    pub behavior_egress_staging_bytes: GpuCapacityLimit,
    pub behavior_egress_host_bytes: u64,
    pub extension_jobs: u32,
    pub extension_registrations: u32,
    pub extension_registry_bytes: u64,
    pub extension_packet_bytes: u64,
    pub extension_state_bytes: u64,
    pub extension_candidate_effects: u32,
}

pub struct PresentationConfig {
    pub stale_view_policy: StaleViewPolicy,
    pub retry_count: u8,
    pub diagnostic_fallback: bool,
}
```

`ResourceLimits` has the following fields and relationships. “Hard maximum”
means validation rejects a larger request before startup. An
adapter-negotiated field uses `GpuCapacityLimit`: startup chooses
`effective = min(desired, adapter_legal)` and fails with
`UnsupportedCapabilities` if effective is below `minimum`, below one enabled
maximum legal operation, or violates a cross-limit.

| Field | Default | Hard maximum / relationship |
| --- | ---: | --- |
| `nonempty_materials` | 4,096 | 65,535; empty ID 0 is additional |
| `max_material_metadata_bytes` | 4 KiB | 1 MiB per registration; per-record gauge, not aggregate capacity |
| `material_metadata_bytes` | 16 MiB | 1 GiB retained aggregate; `>= max_material_metadata_bytes` |
| `live_volumes` | 1,024 | 65,535 |
| `volume_records` | 4,096 | 65,535; `>= live_volumes`; counts every live key and retained tombstone for the world's lifetime |
| `interest_leases` / `bricks_per_interest` | 64 / 4,096 | 4,096 / 65,536 |
| `detailed_bricks: GpuCapacityLimit` | 32,768 / 8,192 | `min(u32::MAX, adapter allocation/2,048)`; segmented by binding limit |
| `page_keys: GpuCapacityLimit` | 131,072 / 32,768 | largest power of two within adapter allocation and `u32`; live load <=70% |
| `page_versions: GpuCapacityLimit` | 262,144 / 65,536 | adapter allocation/entry size and `u32`; `>= page_keys`; covers command reservations |
| `versions_per_brick` | 8 | 64 |
| `dirty_scar_bricks: GpuCapacityLimit` | 32,768 / 8,192 | adapter allocation/2,048 and `u32`; `>= max_command_bricks` |
| `command_records` / `command_payload_bytes` | 1,024 / 64 MiB | 65,536 / 1 GiB; records `>= extension_candidate_effects` when enabled; bytes >= maximum patch |
| `query_records` / `query_result_bytes` | 256 / 32 MiB | 16,384 / 1 GiB; bytes >= largest enabled query result |
| `observation_facts` | 4,096 | 1,048,576 |
| `observation_payload_bytes` | 32 MiB | 1 GiB; `>= 192 + 32 * volume_records` so one maximum checkpoint fact plus its 128-byte filter envelope fits |
| `subscribers` / `volumes_per_filter` | 64 / 256 | 4,096 / `min(live_volumes, 256)` |
| `staging_maps` / `staging_bytes: GpuCapacityLimit` | 8 / 32 MiB desired, 8 MiB minimum | maps 1..=256; bytes <=1 GiB and adapter allocation; covers largest enabled readback chunk |
| `content_requests` / `content_bricks_per_request` / `content_response_bytes` | 64 / 512 / 32 MiB | 4,096 requests / 4,096 bricks per request / 1 GiB; bytes `>= 2,080 * content_bricks_per_request + 256` |
| `persistence_requests` / `persistence_staged_bytes` | 8 / 64 MiB | 256 / 1 GiB; staged bytes >= 8 MiB chunk decode bound when enabled |
| `extraction_records` / `extraction_bytes` | 2,048 / 32 MiB | 65,536 / 1 GiB; bytes cover one maximum enabled patch, scheduled consumer-input header/payload, behavior processor-transition record, or extension input packet plus inline state; records are at least 1 |
| `presentation_jobs` | 1,024 | 65,536; zero only when presentation disabled |
| `presentation_artifacts` / `presentation_dirty_records` | 16,384 / 16,384 | 1,048,576 each; artifacts `>= presentation_jobs`; dirty records `>= presentation_jobs + live_volumes` and reserve one marker per live-volume slot; both zero only when presentation is disabled |
| `mesh_vertices` / `mesh_indices` | 2,097,152 / 12,582,912 | `u32` and adapter allocation bound; each covers one maximum artifact when enabled |
| `dressing_styles` / `dressing_instances: GpuCapacityLimit` | 256 / 1,048,576 desired, 65,536 minimum | 4,096 styles / adapter allocation and `u32` instances; instances cover one descriptor's `max_instances_per_artifact`; both may be zero together to disable dressing only |
| `behavior_engines` / `behavior_order_edges` | 16 / 64 | 256 / 4,096; zero together only when behavior hooks are disabled; order DAG storage is fixed at validation |
| `behavior_scopes_per_engine` | 8 | 256; bounds each host planning result |
| `behavior_view_volumes` / `behavior_view_bricks` / `behavior_view_cells` | 256 / 8,192 / 262,144 | fixed v2 maxima 65,535 / 65,536 / 1,048,576; counts the sum of per-participant filtered exports, not a shared readable union |
| `behavior_cpu_view_bytes` | 8 MiB | 256 MiB; covers aggregate live CPU participant exports, is `<= staging_bytes.effective`, and is zero only when no CPU adapter is registered |
| `behavior_gpu_view_bytes: GpuCapacityLimit` | 32 MiB desired / 8 MiB minimum | `min(256 MiB, adapter allocation)` aggregate; every participant ABI v1 view independently fits `max_storage_buffer_binding_size` |
| `behavior_input_records` / `behavior_input_bytes` | 16 / 4 MiB | 256 records / 256 MiB host bytes; one record per input-capable participant and the checked sum of every descriptor maximum must fit |
| `behavior_gpu_input_bytes: GpuCapacityLimit` | 4 MiB desired / 64 KiB minimum | `min(256 MiB, adapter allocation)` aggregate dedicated transport charged as one staging plus one device range for every GPU participant header/aligned declared payload (zero payload for `None`); each device range independently fits `max_storage_buffer_binding_size`; zero only when no GPU participant is registered |
| `behavior_collision_calls` / `behavior_collision_contacts` / `behavior_collision_bytes` | 128 / 4,096 / 320 KiB | 4,096 / fixed 4,096 / 320 KiB; calls are aggregate per tick, while one reusable exact 80-byte slot per contact structurally bounds CPU helper output |
| `behavior_handoff_maps` / `behavior_handoff_bytes: GpuCapacityLimit` | 4 / 24 MiB desired, 3 MiB minimum | 256 maps / 1 GiB and adapter allocation; startup reserves at most three times every declared edge capacity for host/device/staging representations, and every individual device binding fits the adapter binding limit |
| `behavior_proposal_records` / `behavior_proposal_bytes` | 1,024 / 64 MiB | 65,536 / 1 GiB aggregate; each GPU participant's effect allocation fits one storage binding, aggregate declared maxima fit, and command/transaction completion capacity covers the same tick |
| `behavior_effect_cells` / `behavior_effect_bricks` / `behavior_directory_effects` | 262,144 / 4,096 / 16 | 1,048,576 / 65,536 / 1,024 aggregate per tick; each proposal still obeys ordinary command maxima and aggregate declared adapter maxima must fit |
| `behavior_conflict_checks` | 1,048,576 | 4,294,967,296 candidate whole-proposal overlap comparisons per tick; overflow fails before publication |
| `behavior_feedback_bytes: GpuCapacityLimit` | 1 MiB desired / 128 KiB minimum | 64 MiB and adapter allocation; holds two slots for every GPU participant, each containing a 64-byte header, one 64-byte terminal participant record, and its maximum 48-byte proposal records |
| `behavior_gpu_buffers` / `behavior_gpu_buffer_bytes: GpuCapacityLimit` | 256 / 256 MiB desired, 64 MiB minimum | 65,536 handles / `min(1 GiB, adapter max_buffer_size)` aggregate live registered bytes; every descriptor maximum and their checked sum must fit the requested desired value at registration and the effective value at startup |
| `behavior_gpu_pipelines` / `behavior_gpu_bind_groups` / `behavior_gpu_wgsl_bytes` | 64 / 256 / 4 MiB | 65,536 handles each / 64 MiB borrowed cumulative pipeline source per device creation; descriptor maxima must sum within them |
| `behavior_gpu_dispatches` / `behavior_gpu_workgroups` | 256 / 1,048,576 | 65,536 / 4,294,967,296 aggregate scheduled adapter dispatches/workgroups per tick; each dimension also obeys the adapter device limit |
| `behavior_placement_updates` / `behavior_placement_bytes` | 65,536 / 4 MiB | 1,048,576 / 64 MiB; bytes `>= 64 * updates`; complete aggregate descriptor maxima fit |
| `behavior_component_extraction_proposals` / `behavior_component_extraction_children` / `behavior_component_extraction_assignment_cells` / `behavior_component_extraction_child_bricks` / `behavior_component_extraction_bytes` | 16 / 256 / 262,144 / 4,096 / 32 MiB | 1,024 / 4,096 / 1,048,576 / 65,536 / 256 MiB; children also fit reserved live/lifetime directory records and all transfer/scar/page pools |
| `behavior_egress_maps` / `behavior_egress_receipts` / `behavior_egress_records` | 16 / 64 / 16,384 | 256 / 4,096 / 1,048,576; maps cover one active tick's enabled participants, receipts bound retained terminal/pending results, and aggregate descriptor record maxima fit |
| `behavior_egress_device_bytes: GpuCapacityLimit` / `behavior_egress_staging_bytes: GpuCapacityLimit` / `behavior_egress_host_bytes` | 16 MiB desired, 1 MiB minimum / 16 MiB desired, 1 MiB minimum / 16 MiB | 256 MiB and adapter allocation / 256 MiB and adapter allocation / 256 MiB; device capacity covers `align4(80 + maximum_bytes)` per enabled adapter, while staging and decoded-host pools cover exact payload maxima |
| `extension_jobs` | 64 | 4,096; zero only when extensions disabled |
| `extension_registrations` / `extension_registry_bytes` | 32 / 4 MiB | 1,024 / 64 MiB; owns all registered WGSL and entry-point bytes |
| `extension_packet_bytes` / `extension_state_bytes` | 16 MiB / 1 MiB | fixed v1 pool maxima 64 MiB / 4 MiB; state pool holds at least one prior+next pair |
| `extension_candidate_effects` | 256 | fixed v1 maximum 256 and `<= command_records` |

The fixed request maxima remain: 32,768 cells and 512 bricks per matter
command, 16 MiB patch payload, 262,144 cells per region read, 8,192 candidate
bricks and 65,536 candidate cells per collision traversal, 4,096 collision
hits, 256 world-scope volumes, 2,048 vertices/12,288 indices per brick artifact,
13,824 unique halo invalidations per matter command, 4,096 dressing instances
per artifact, 256 scheduled behavior engines, 4,096 declared behavior order
edges, 1 MiB per behavior handoff edge, 65,535 view volumes, 65,536 view bricks,
1,048,576 view cells, 65,536 behavior proposals,
1 MiB opaque consumer input per behavior participant,
1,048,576 compact placement updates, 4,096 component-extraction children,
1,048,576 component-extraction assignment cells, 65,536 child bricks,
1,048,576 opaque egress records with at most 65,536 bytes per record,
96 UTF-8 bytes for every debug name, 1 MiB WGSL and 128 UTF-8 bytes for one
extension entry point, and 256 candidate effects. They are exported in
`contract_limits`; they are not independently configurable.

`PresentationConfig` defaults to `stale_view_policy = DisplayStale`,
`retry_count = 1` (legal `0..=3`), and
`diagnostic_fallback = false`. It contains no camera or content policy.

Supplying a checkpoint store or restore request requires the consumer to set
`capabilities.persistence = true`; neither call mutates the capability flag.
Enabling persistence without a store is a configuration error. Enabling GPU
extensions requires the Cargo feature, nonzero extension limits, extraction
bytes large enough for one maximum configured packet, and a command queue
capable of reserving the configured worst-case batch. Disabling presentation
requires all presentation artifact, dirty, mesh, dressing, and job pool fields
to be zero and makes presentation interest an explicit `CapabilityDisabled`
error.

Disabling behavior hooks rejects behavior adapter registration/tick
reservation with `CapabilityDisabled` and leaves the configured capacity
inactive. It does not disable ordinary queries or the separately configured
asynchronous `gpu_extensions` facility.

With presentation enabled, `dressing_styles == 0` is legal only when
`dressing_instances.desired == minimum == 0`; registering a style then returns
`DressingStyleCapacity { limit: 0 }`. With dressing enabled, registry count and
instance desired/minimum are nonzero and the effective instance pool holds at
least one descriptor's `max_instances_per_artifact`. Extension registry bytes
must be at least 1 MiB + 128 bytes when extensions are enabled, so one maximum
legal descriptor is representable.

`EffectiveConfig.requested` mirrors every requested field. Its `limits` vector
records each numeric `ResourceLimits` field in `ResourceKind` order plus
`Exact | AdapterClamped { adapter_max }`; the remaining public config records
are repeated as their exact effective values.
It is returned by startup, available through
`MoriaHandle::effective_config()`, and embedded in telemetry/evidence. Values
not marked adapter-negotiated must equal their request. No clamp can weaken an
enabled operation below its fixed public maximum; such an adapter fails
startup instead.

Enabling scheduled behavior hooks requires one active-tick record, nonzero
behavior registration/view/proposal/feedback/collision/handoff limits,
aggregate declared
adapter maxima within those limits, enough command/page/scar/directory
transaction capacity for the aggregate worst-case proposal set, and enough
extraction capacity for one maximum transition record. CPU-view bytes may be
zero only when every registered adapter is GPU. GPU-view bytes may be zero only
when every adapter is CPU. Handoff maps/bytes may be zero only when no registered
edge declares a payload. Feedback capacity includes current and prior slots.
`behavior_input_records` must cover every descriptor whose input policy is not
`None`, and `behavior_input_bytes` must cover the checked sum of their declared
maxima. `behavior_gpu_input_bytes` must cover
`2 * align4(64 + maximum_consumer_input_bytes)` for every GPU participant,
including a zero-payload absent header for policy `None`: one staging range and
one device range. It may be zero only when no GPU participant is registered.
Input capacity is independent of adapter handoff capacity: a participant never
needs a predecessor merely to receive current consumer bytes.
GPU buffer/count/pipeline/bind-group/WGSL pools may be zero only when every
adapter is CPU. For GPU registrations, the checked sum of
`maximum_owned_gpu_bytes` must fit `behavior_gpu_buffer_bytes.desired` during
builder registration and its adapter-clamped effective value during startup;
the count and WGSL descriptor maxima must likewise fit their aggregate pools.
`behavior_gpu_buffer_bytes.minimum` must be nonzero, no greater than desired,
and large enough for the largest registered descriptor maximum. An adapter
whose clamp makes either the largest descriptor or the aggregate sum
unrepresentable fails startup with the deterministic
`UnsupportedCapabilities` report before `create_device_state`.
The behavior ordering graph and all maximum access envelopes are validated
before startup; runtime planning may narrow but never expand them.
Scheduled ABI v2 component-extraction registration additionally checked-sums candidate
children, assignments, child bricks, payload bytes, and every associated
live/lifetime directory, page, scar, observation, receipt, and presentation
record. Placement registration checked-sums its 64-byte entries and the ordinary
directory/revision capacity required for one multi-volume publication of that
stream. Multi-volume proposals charge existing directory-entry, lifetime,
page/brick/scar, observation, and presentation pools; they do not create a
second directory architecture. Egress registration requires an
exact `record_stride * maximum_records == maximum_bytes` product. The device
pool checked-sums `align4(80 + maximum_bytes)` for every enabled adapter;
staging and host pools checked-sum payload maxima, and map/receipt counts cover
the enabled adapter count. None of those pools aliases proposal, handoff,
generic staging, or adapter-owned factory bytes.

Metadata registration reserves both one material record and its exact retained
metadata bytes. Exhausting `material_metadata_bytes` returns
`RegistrationError::MaterialMetadataCapacity` without retaining the
definition. `max_material_metadata_bytes` is enforced first. Its telemetry
usage is the largest current record and high-water is the largest record ever
accepted; `MaterialMetadataBytes` reports the aggregate retained pool.

The observation ring owns independent fact-slot and encoded-payload capacities.
Append evicts oldest whole facts until both fit and never splits a fact.
Every retained ring record also owns one fixed 128-byte `FilterEnvelopeV1`;
that envelope is included in payload usage and high-water telemetry.
Checkpoint revision vectors encode as 32 bytes per entry plus a 64-byte fact
header and the envelope, so one maximum legal fact always fits. Subscriber
cursor/revision arrays are fixed-capacity allocations derived from
`subscribers * volumes_per_filter` at startup and do not grow with history.

`content_bricks_per_request` is the exact count bound for each source callback.
Moria partitions larger materialization demand into stable brick-order batches,
with at most `content_requests` callbacks in flight. Before invoking consumer
code, the scheduler atomically acquires both one callback slot and a
`content_response_bytes` permit for
`256 + 2,080 * request.bricks.len()` bytes. It never holds one resource while
waiting for the other and never shrinks or splits an already formed batch to
fit currently free bytes. The fixed 256-byte portion covers the complete
by-value `ContentError`, batch bookkeeping, and poison state; a source cannot
return variable owned diagnostic storage through that allowance.

```rust
impl MoriaHandle {
    pub fn effective_config(&self) -> Option<EffectiveConfig>;
}
```

## Bounds and coordinates

`CellCoord` and `BrickCoord` are signed local coordinates. `CellAabb` is
half-open and validated with checked arithmetic. `WorldPoint` and
`WorldVector` contain finite `f32` values. World queries transform into each
volume's local address space with its committed placement.

```rust
pub struct CellCoord { pub x: i32, pub y: i32, pub z: i32 }
pub struct BrickCoord { pub x: i32, pub y: i32, pub z: i32 }
pub struct CellAabb { pub min: CellCoord, pub max: CellCoord }
pub struct WorldPoint { pub x: f32, pub y: f32, pub z: f32 }
pub struct WorldVector { pub x: f32, pub y: f32, pub z: f32 }
pub struct WorldAabb { pub min: WorldPoint, pub max: WorldPoint }

pub struct RigidPlacement {
    pub translation: WorldVector,
    pub rotation_xyzw: [f32; 4],
}

pub enum VolumeMode { Static, Dynamic }

pub enum RevisionPrecondition {
    AnyCommitted,
    Exact(VolumeRevision),
}

pub enum MinimumRevision {
    AnyCommitted,
    AtLeast { volume: VolumeId, revision: VolumeRevision },
    Exact { volume: VolumeId, revision: VolumeRevision },
}

#[repr(C)]
pub struct MaterialSample {
    pub material: MaterialId,
    pub coverage: u8,
    pub flags: u8,
}

#[repr(transparent)]
pub struct Correlation([u8; 16]);
```

`Correlation::NONE` is all zeroes; `Correlation::from_bytes` and
`Correlation::as_bytes` construct/inspect the fixed 16-byte value. It is
copied into command, observation, checkpoint diagnostic, and candidate-effect
records. It has no separately allocated payload and is never interpreted by
Moria.

`RigidPlacement` contains translation and a unit quaternion. Construction
normalizes only within a small documented tolerance; zero, non-finite, scale,
and shear inputs are rejected. Static volumes reject placement commands.

Every region method takes explicit bounds. It either accepts the complete
request, rejects it with `SupportedBounds`, or honors an explicit
`PartialPolicy::Allow { max_omitted_regions }`. Silent clipping is forbidden.

## Admission, permits, and receipts

Ingress is bounded independently by record count and owned payload bytes.

```rust
pub enum ReserveError {
    Full { available_records: u32, available_bytes: u64 },
    Closed,
    PayloadTooLarge { requested: u64, limit: u64 },
    CapabilityDisabled,
}

pub enum TryReserveError {
    Full { available_records: u32, available_bytes: u64 },
    Closed,
    PayloadTooLarge { requested: u64, limit: u64 },
    CapabilityDisabled,
}

pub struct ReserveFuture<P> {
    /* Future<Output = Result<P, ReserveError>> + Send */
}

impl WorldHandle {
    pub fn try_reserve_command(
        &self,
        payload_bytes: u64,
    ) -> Result<CommandPermit, TryReserveError>;

    pub fn reserve_command(
        &self,
        payload_bytes: u64,
    ) -> ReserveFuture<CommandPermit>;

    pub fn try_reserve_query(
        &self,
        result_budget_bytes: u64,
    ) -> Result<QueryPermit, TryReserveError>;

    pub fn reserve_query(
        &self,
        result_budget_bytes: u64,
    ) -> ReserveFuture<QueryPermit>;

    pub fn try_reserve_checkpoint(
        &self,
        staged_bytes: u64,
    ) -> Result<CheckpointPermit, TryReserveError>;

    pub fn reserve_checkpoint(
        &self,
        staged_bytes: u64,
    ) -> ReserveFuture<CheckpointPermit>;

    pub fn try_reserve_behavior_tick(
        &self,
    ) -> Result<BehaviorTickPermit, TryReserveError>;

    pub fn reserve_behavior_tick(
        &self,
    ) -> ReserveFuture<BehaviorTickPermit>;

    pub fn try_reserve_extension(
        &self,
        job_bytes: u64,
    ) -> Result<ExtensionPermit, TryReserveError>;

    pub fn reserve_extension(
        &self,
        job_bytes: u64,
    ) -> ReserveFuture<ExtensionPermit>;

    pub fn try_reserve_effect_batch(
        &self,
        max_effects: u16,
        command_payload_bytes: u64,
    ) -> Result<EffectBatchPermit, TryReserveError>;

    pub fn reserve_effect_batch(
        &self,
        max_effects: u16,
        command_payload_bytes: u64,
    ) -> ReserveFuture<EffectBatchPermit>;
}
```

A command/query/checkpoint permit reserves one record and the declared bytes in
that operation's queue. A behavior tick permit reserves the single active tick
record, one input record and the complete declared host bytes for every
input-capable participant, all required GPU ingress upload/device bytes, and
all registered per-participant views, reusable CPU collision scratch/calls,
handoff host/device/staging/maps, proposal, transaction, completion, and
current/prior feedback maxima before any planner or adapter runs. It also owns
the declared component-extraction identities/transfers/directory versions,
placement-stream entries/directory versions, and egress device/staging/host/
map/receipt maxima. An
extension permit reserves one
job and the complete job allocation (header, packet, two state ranges,
candidates, diagnostics, and effect payload); it must fit the configured
packet/state and descriptor effect bounds. An `EffectBatchPermit` reserves
`max_effects` ordinary command records, their aggregate encoded payload bytes,
and the same number of child receipt/completion slots. Dropping an unused
permit releases all capacity.

`ReserveFuture<P>` has output `Result<P, ReserveError>`. With the queue's
configured `WaitForPermit`, it waits in bounded FIFO waiter storage; with
`Reject`, it immediately resolves to `ReserveError::Full`. Every `try_` method
is always immediate regardless of policy. Dropping the future removes its
waiter. Each queue has at most its configured record count in waiter slots; an
additional waiter resolves `Full` rather than allocating. Queue close resolves
every waiter as `Closed`. Effect-batch reservation uses the command queue's
overload policy because it reserves ordinary child command capacity.

Submission consumes its permit and owned input. Structural rejection returns
the input unchanged and releases the submitted operation permit's capacity. A
rejected `GpuExtensionRequest` still owns its nested `EffectBatchPermit`, so
the caller may correct/resubmit it or drop it to release child capacity.
Declared bytes are an upper bound; admission rejects an input whose encoded
size exceeds them and releases unused bytes immediately after successful
encoding.

```rust
pub enum SubmitError<T> {
    Invalid { command: T, violations: Vec<Violation> },
    StaleHandle { command: T },
    WorldNotAccepting { command: T, state: WorldState },
    PermitMismatch { command: T },
}

pub struct Receipt<T: Clone> {
    /* Clone + Future<Output = Result<T, OperationError>> + Send + Sync */
}

impl<T: Clone> Receipt<T> {
    pub fn id(&self) -> OperationId;
    pub fn try_status(&self) -> ReceiptStatus<T>;
    pub fn request_cancel(&self) -> CancelRequest;
}

pub enum ReceiptStatus<T> {
    Pending(OperationStage),
    Ready(T),
    Failed(OperationError),
}

pub enum CancelRequest {
    Accepted,
    TooLate { stage: OperationStage },
    NotCancellable,
    AlreadyTerminal,
}

pub enum OperationStage {
    Queued,
    WaitingForMatter,
    Preparing,
    Submitted,
    AwaitingReadback,
    AwaitingPersistence,
    Recovering,
}

pub enum WorldState {
    Configured,
    Starting,
    Ready,
    Recovering,
    ShuttingDown,
    Stopped,
    Failed,
}

#[repr(u8)]
pub enum Retryability {
    Never,
    AfterInputChange,
    AfterPressureRelief,
    AfterRecovery,
    Immediate,
}

pub struct OperationError {
    pub operation: OperationId,
    pub scope: OperationScope,
    pub retryability: Retryability,
    pub device_generation: Option<DeviceGeneration>,
    pub revision_changed: bool,
    pub kind: OperationErrorKind,
    pub diagnostic: String,
}

pub enum OperationScope {
    World(WorldId),
    Volume(VolumeId),
    Region { volume: VolumeId, bounds: CellAabb },
    Checkpoint(CheckpointKey),
    BehaviorTick(BehaviorTickId),
    BehaviorEgress { tick: BehaviorTickId, engine: BehaviorEngineId },
    Extension(ExtensionId),
}

pub enum OperationErrorKind {
    Validation,
    Conflict { expected: VolumeRevision, current: VolumeRevision },
    Unavailable,
    BudgetExhausted(ResourceKind),
    OutputOverflow { required: u64, limit: u64 },
    Content(ContentErrorKind),
    GpuValidation,
    OutOfMemory,
    DeviceLost,
    Readback,
    Decode,
    Persistence(PersistenceErrorKind),
    Startup(StartupFailure),
    Behavior(BehaviorEngineFailure),
    BehaviorEgress(BehaviorEgressFailure),
    CancelledBeforePreparation,
    ShuttingDown,
    InternalInvariant,
}

pub struct StartupFailure {
    pub stage: StartupStage,
    pub causes: Vec<StartupCause>,       // sorted, <= config fields + required features
}

pub enum StartupStage {
    RendererLookup,
    AdapterQualification,
    DeviceResources,
    PersistenceOpen,
    RestoreRead,
    DirectoryPublication,
}

pub enum StartupCause {
    RendererUnavailable,
    UnsupportedCapabilities {
        adapter: Option<AdapterCapabilityReport>,
        missing_features: Vec<CapabilityRequirement>,
        unmet_limits: Vec<LimitRequirement>,
    },
    Shader(ShaderDiagnostic),
    OutOfMemory,
    Persistence(PersistenceErrorKind),
    Restore(PersistenceErrorKind),
    InternalInvariant,
}

pub struct CapabilityRequirement {
    pub capability: RequiredCapability,
    pub available: bool,
}

pub enum RequiredCapability {
    ComputeShaders,
    StorageBuffers,
    BufferToBufferCopy,
    FourWritableStorageBindings,
    SixStorageBindingsForBehaviorGroupZero,
}

pub struct LimitRequirement {
    pub resource: ResourceKind,
    pub requested_minimum: u64,
    pub adapter_maximum: Option<u64>,
}

pub struct Violation {
    pub path: String,                    // <=128 UTF-8 bytes
    pub code: ViolationCode,
    pub supported: Option<SupportedBounds>,
    pub diagnostic: String,              // <=512 UTF-8 bytes
}

pub enum ViolationCode {
    InvalidValue,
    InvalidBounds,
    TooLarge,
    MissingIdentity,
    DuplicateIdentity,
    UnknownBehaviorParticipant,
    DuplicateBehaviorInput,
    MissingBehaviorInput,
    UnexpectedBehaviorInput,
    BehaviorInputTooLarge,
    StaleHandle,
    LiveVolumeCapacity,
    VolumeRecordCapacity,
    RetiredVolumeKey,
    CapabilityDisabled,
    CrossLimit,
}

pub struct SupportedBounds {
    pub maximum_records: Option<u32>,
    pub maximum_bytes: Option<u64>,
    pub maximum_cells: Option<u32>,
    pub maximum_bricks: Option<u32>,
    pub maximum_candidate_cells: Option<u32>,
    pub maximum_candidate_bricks: Option<u32>,
    pub maximum_volumes: Option<u16>,
}
```

Receipts are cloneable observers of one shared terminal state. Dropping every
observer does not cancel work. For command, query, checkpoint, and extension
operations, the cancellation linearization point is the atomic transition from
`Queued | WaitingForMatter` to `Preparing`. `request_cancel` wins that race by
installing terminal `CancelledBeforePreparation` and returns `Accepted`; if
preparation won, it returns `TooLate` with the observed stage and the operation
continues. `Accepted` means the terminal state and capacity release are visible
before the method returns. Startup and shutdown receipts return
`NotCancellable`; terminal receipts return `AlreadyTerminal`.

Every cancellable accepted operation starts in `Queued`, may enter
`WaitingForMatter`, and must win exactly one transition to `Preparing` before
it can acquire transaction slots, pin checkpoint versions, freeze a query
snapshot, enter behavior planning, allocate an extension job, or enter any
later family-specific stage.
This rule applies even when an operation needs no GPU dispatch; no family has
an alternate cancellation boundary.

An operation stage is diagnostic, not a latency promise.
`OperationErrorKind` is `#[non_exhaustive]` for source compatibility, while all
initial public variants and their fields are listed above. `Violation` vectors are sorted
by `path` then `code` and bounded by the submitted record's field count; they
never contain arbitrary consumer payloads.

Startup failures use `OperationErrorKind::Startup`; the outer operation error
still supplies world scope, retryability, and diagnostic. Causes are aggregated
rather than fail-fast, sorted by capability/`ResourceKind`/shader stage, and
bounded by the closed required-feature list plus the numeric config fields.
`UnsupportedCapabilities` carries the queried adapter report when one exists
and one `LimitRequirement` for every unmet minimum, so startup never collapses
renderer absence, adapter insufficiency, and restore failure into
`Unavailable`.

A failed matter mutation always reports `revision_changed = false`.

### Facade operations

The following methods are the only admission and inspection facade. `submit_*`
does no hidden waiting; callers acquire the matching permit first.

```rust
impl MoriaHandle {
    pub fn world(&self, id: WorldId) -> Result<WorldHandle, StaleHandleError>;
}

impl WorldHandle {
    pub fn submit_material_registry(
        &self,
        permit: QueryPermit,
        page: RegistryPageRequest,
    ) -> Result<Receipt<MaterialRegistryPage>, SubmitError<RegistryPageRequest>>;
    pub fn material(
        &self,
        id: MaterialId,
    ) -> Result<Arc<MaterialRegistration>, StaleHandleError>;

    pub fn submit_matter(
        &self,
        permit: CommandPermit,
        command: MatterCommand,
    ) -> Result<Receipt<MatterApplied>, SubmitError<MatterCommand>>;

    pub fn submit_volume(
        &self,
        permit: CommandPermit,
        command: VolumeCommand,
    ) -> Result<Receipt<VolumeApplied>, SubmitError<VolumeCommand>>;

    pub fn submit_query(
        &self,
        permit: QueryPermit,
        query: Query,
    ) -> Result<Receipt<QueryOutcome>, SubmitError<Query>>;

    pub fn declare_interest(
        &self,
        request: InterestRequest,
    ) -> Result<InterestLease, InterestError>;

    pub fn subscribe(
        &self,
        subscription: Subscription,
        start: SubscriptionStart,
    ) -> Result<ObservationSubscriber, SubscriptionError>;

    pub fn request_checkpoint(
        &self,
        permit: CheckpointPermit,
        request: CheckpointRequest,
    ) -> Result<Receipt<CheckpointApplied>, SubmitError<CheckpointRequest>>;

    pub fn request_behavior_tick(
        &self,
        permit: BehaviorTickPermit,
        request: BehaviorTickRequest,
    ) -> Result<BehaviorTickAdmission, SubmitError<BehaviorTickRequest>>;

    pub fn register_gpu_extension(
        &self,
        descriptor: GpuExtensionDescriptor,
    ) -> Result<ExtensionId, ExtensionRegistrationError>;

    pub fn submit_gpu_extension(
        &self,
        permit: ExtensionPermit,
        request: GpuExtensionRequest,
    ) -> Result<Receipt<GpuExtensionDispatched>, SubmitError<GpuExtensionRequest>>;

    pub fn telemetry(&self) -> TelemetrySnapshot;

    pub fn shutdown(
        &self,
        policy: ShutdownPolicy,
    ) -> Result<Receipt<ShutdownReport>, ShutdownStartError>;

    pub fn shutdown_receipt(&self) -> Option<Receipt<ShutdownReport>>;
}
```

`request_behavior_tick` is available only when scheduled behavior hooks are
enabled and at least one adapter is registered. It does not choose a clock;
the consumer calls it from its desired Bevy schedule or tool loop. Its
`inputs` slice has exact boxed ownership and at most one entry per registered
participant. Successful admission returns the tick receipt plus one egress
receipt for every egress-enabled participant in validated participant schedule
order, so cancellation or failure before a `BehaviorTickCompleted` value still
has an observable egress terminal outcome. The same receipt is referenced from
the later participant report.
Structural input rejection uses
`UnknownBehaviorParticipant | DuplicateBehaviorInput |
MissingBehaviorInput | UnexpectedBehaviorInput | BehaviorInputTooLarge`,
returns the complete request unchanged, releases the permit, and assigns no
tick ID.

`register_gpu_extension` is available only when the Cargo feature and
configured capability are enabled. `shutdown` is the only accepted operation
that does not use an ordinary queue permit: the world preallocates exactly one
shutdown record during startup, and the first call atomically consumes it.
Later calls return `AlreadyShuttingDown` and may obtain the same receipt
through `WorldHandle::shutdown_receipt()`.

Synchronous facade errors are closed enums:

```rust
pub enum InterestError {
    Invalid(Vec<Violation>),
    Full { limit: u32 },
    StaleHandle,
    WorldNotAccepting(WorldState),
    CapabilityDisabled(InterestCapabilities),
}

pub enum SubscriptionError {
    Invalid(Vec<Violation>),
    Full { limit: u32 },
    StartNotRetained { requested: ObservationSequence, oldest: ObservationSequence },
    WorldNotAccepting(WorldState),
}

pub enum ObservationError {
    Closed,
    StaleSubscriber,
}

pub enum ResumeError {
    SnapshotScopeMismatch,
    SnapshotOlderThanGap,
    NotWaitingForSnapshot,
    StaleSubscriber,
}

pub enum ExtensionRegistrationError {
    CapabilityDisabled,
    DuplicateKey,
    InvalidDescriptor(Vec<Violation>),
    ShaderValidation(ShaderDiagnostic),
    RegistryFull { registrations: u32, limit: u32 },
    RegistryBytes {
        requested: u64,
        available: u64,
        per_descriptor_limit: u64,
    },
    WorldNotAccepting(WorldState),
}

pub enum ShutdownStartError {
    AlreadyShuttingDown,
    StaleWorld,
}
```

Registration/configuration and asynchronous operation errors remain the
structured types specified above and in the failure table; none is replaced by
a string. Every error exposes its stable category plus a human-readable
diagnostic generated from the same fields.

```rust
pub struct RegistryPageRequest {
    pub after: Option<MaterialKey>,
    pub max_records: u32,
    pub max_bytes: u64,
}

pub struct MaterialRegistryPage {
    pub records: Vec<MaterialRegistration>,
    pub registry_digest: [u8; 32],
    pub next_after: Option<MaterialKey>,
}

pub struct MaterialRegistration {
    pub id: MaterialId,
    pub key: MaterialKey,
    pub debug_name: String,
    pub presentation: SurfaceDescriptor,
    pub opaque_metadata: Vec<u8>,
}
```

The request is bounded by `nonempty_materials`, the `QueryPermit`, and
`query_result_bytes`. The page is stable-key sorted and owned. It never splits
a record: if the first eligible registration exceeds `max_bytes`, it returns
terminal `OperationError::OutputOverflow` with the required one-record size.
Repeated pages therefore provide the stated opaque-metadata inspection path
without an unbounded registry allocation or an unreserved concurrent copy.

## Interest

```rust
bitflags! {
    pub struct InterestCapabilities: u8 {
        const INSPECTION = 0b0001;
        const COLLISION  = 0b0010;
        const PRESENTATION = 0b0100;
    }
}

pub struct InterestRequest {
    pub scope: InterestScope,
    pub capabilities: InterestCapabilities,
    pub priority: InterestPriority,      // Background | Normal | Urgent
    pub max_bricks: u32,
}

pub enum InterestScope {
    VolumeLocal { volume: VolumeId, bounds: CellAabb },
    World {
        bounds: WorldAabb,
        volumes: BoundedVolumeFilter,
    },
}

pub enum InterestPriority {
    Background,
    Normal,
    Urgent,
}

pub enum BoundedVolumeFilter {
    Include(Vec<VolumeId>),              // sorted unique, <= volumes_per_filter
    All { max_volumes: u16 },            // 1..=min(256, volumes_per_filter)
}

pub struct AcceptedInterest {
    pub id: InterestId,
    pub scope: InterestScope,
    pub capabilities: InterestCapabilities,
    pub priority: InterestPriority,
    pub resolved_bricks: u32,
    pub regions: Vec<ResolvedInterestRegion>,
}

pub struct ResolvedInterestRegion {
    pub volume: VolumeSnapshotRef,
    pub bricks: Vec<BrickCoord>,         // sorted unique; total <= request.max_bricks
}

pub struct InterestState {
    pub capabilities: Vec<CapabilityReadiness>, // one record per requested bit
}

pub struct CapabilityReadiness {
    pub capability: InterestCapability,
    pub ready_bricks: u32,
    pub total_bricks: u32,
    pub state: CapabilityState,
}

pub enum InterestCapability {
    Inspection,
    Collision,
    Presentation,
}

pub enum CapabilityState {
    Pending,
    Ready { snapshots: Vec<VolumeSnapshotRef> },
    Failed { regions: Vec<UnavailableRegion> },
}

pub struct InterestLease { /* Send + Sync */ }

impl InterestLease {
    pub fn id(&self) -> InterestId;
    pub fn accepted(&self) -> AcceptedInterest;
    pub fn state(&self) -> InterestState;
    pub fn update(
        &self,
        replacement: InterestRequest,
    ) -> Result<AcceptedInterest, InterestError>;
}
```

`declare_interest` validates and returns a lease plus the accepted bounded
scope. Cloning the lease retains interest. `update` atomically replaces the
request after validation. Dropping the last clone withdraws it. Withdrawal
does not cancel commands, invalidate completed results, or discard dirty scars.

Long-lived interest filters are snapshots, not live queries.
`Include` resolves the named live handles and `All` resolves every volume live
at acceptance, sorted by stable key; exceeding `max_volumes` rejects instead of
clipping. A world-scope interest then freezes the exact local brick set
intersecting its world bound at the captured placements. `AcceptedInterest`
exposes those IDs, placements/revisions, and bricks. Later create, retire, or
move commands do not add, substitute, or spatially recompute this set. A
retired member reports `RegionFailureKind::Retired`; it is not replaced by a
new volume. The consumer calls `update` to take a new bounded membership and
placement snapshot.

Interest declarations use the configured `interest_leases` slots and return
`InterestError::Full` synchronously when exhausted; they have no payload queue
and therefore no wait policy. A consumer may retry after receiving a resource-
pressure observation.

Every vector above is bounded by `volumes_per_filter`, `max_bricks`, and the
three-bit capability set. `AcceptedInterest.scope` is the normalized,
fully accepted request (including sorted filters), not a silently clipped
scope. `CapabilityReadiness.ready_bricks == total_bricks` is required for
`Ready`; a failed region is never counted ready.

`Urgent` changes ordering only; it cannot exceed budgets or preempt an admitted
transaction.

## Base content source

```rust
pub trait BaseContentSource: Send + Sync + 'static {
    fn descriptor(&self) -> &SourceDescriptor;
    fn load_bricks(
        &self,
        request: BaseBrickRequest,
        output: &mut BaseBrickOutput<'_>,
        cancel: &CancellationToken,
    ) -> Result<(), ContentError>;
}

pub struct ContentLineage {
    pub family: uuid::Uuid,
    pub version: u32,
    pub opaque: Box<[u8]>,               // exact allocation, <=256 bytes
}

pub struct ReconstructionFingerprint(pub [u8; 32]);

pub struct SourceDescriptor {
    pub lineage: ContentLineage,
    pub reconstruction: ReconstructionFingerprint,
}

pub struct BaseBrickRequest {
    pub volume: VolumeKey,
    pub lineage: ContentLineage,
    pub reconstruction: ReconstructionFingerprint,
    pub bricks: Vec<BrickCoord>,         // sorted unique, <= content_bricks_per_request
    pub intersections: Vec<CellAabb>,    // one per brick
    pub material_registry_digest: [u8; 32],
    pub maximum_encoded_bytes: u64,
}

pub struct BaseBrickOutput<'permit> {
    /* opaque Moria-owned, permit-backed, exact-length sink; not Send or constructible */
}

impl BaseBrickOutput<'_> {
    pub fn len(&self) -> u32;
    pub fn written(&self) -> u32;
    pub fn push_homogeneous(
        &mut self,
        sample: MaterialSample,
    ) -> Result<(), ContentWriteError>;
    pub fn push_detailed(
        &mut self,
        samples: &[MaterialSample; 512],
    ) -> Result<(), ContentWriteError>;
}

pub struct ContentWriteError {
    pub index: u32,
    pub kind: ContentWriteErrorKind,
}

pub enum ContentWriteErrorKind {
    TooManyResults,
    UnknownMaterial,
    InvalidSample,
    NonEmptyOutsideDomain,
}

#[repr(C)]
pub struct ContentError {
    pub kind: ContentErrorKind,
    pub retryability: Retryability,
    pub diagnostic: ContentDiagnostic,
}

#[repr(u8)]
pub enum ContentErrorKind {
    Unavailable,
    InvalidBatch,
    Cancelled,
    Panicked,
}

pub const CONTENT_DIAGNOSTIC_MAX_BYTES: usize = 192;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ContentDiagnostic {
    len: u8,
    bytes: [u8; CONTENT_DIAGNOSTIC_MAX_BYTES],
}

impl ContentDiagnostic {
    pub fn try_from_str(value: &str) -> Result<Self, ContentDiagnosticError>;
    pub fn as_str(&self) -> &str;
}

pub struct ContentDiagnosticError {
    pub actual_bytes: u32,
    pub maximum_bytes: u16, // always 192 in ABI v1
}

const _: () = assert!(std::mem::size_of::<ContentError>() == 195);

pub struct CancellationToken { /* Clone + Send + Sync */ }

impl CancellationToken {
    pub fn is_cancelled(&self) -> bool;
}
```

A request contains one volume key, its lineage/fingerprint, at most
`content_bricks_per_request` sorted unique brick coordinates, the intersected
domain, material registry digest, and maximum encoded bytes. Larger demand is
partitioned in stable coordinate order; one callback never sees a hidden
larger batch. Before invocation, Moria atomically reserves one
`content_requests` slot and the exact conservative worst-case response charge
`256 + 2,080 * bricks.len()` from the aggregate
`content_response_bytes` pool and constructs an exact-length, Moria-owned
`BaseBrickOutput` inside that permit. The sink owns one fixed worst-case slot
for every requested coordinate before consumer code runs.
`maximum_encoded_bytes` is exactly that byte permit. If either resource is
unavailable, the materialization batch remains queued and emits
`ResourcePressure { action: Deferred }`; consumer code is not invoked, and no
count-only or byte-only reservation is retained. Stable batch order plus retry
on permit release prevents a later batch for the same volume from bypassing
it. Urgent priority changes selection between volumes only. Cancellation while
queued removes the batch without acquiring either permit. The callback runs on
a Moria worker, never a render or Bevy main thread.

The source fills the sink in request order, once per coordinate. A homogeneous
write copies one `MaterialSample`; a detailed write borrows exactly
`&[MaterialSample; 512]` and copies it into the already reserved slot. No
capacity-bearing collection, box, or other result ownership crosses from the
consumer into Moria. The callback's only by-value return is a discriminant,
retryability, and the fixed inline `ContentDiagnostic`; it cannot transfer a
`String`, `Box`, vector capacity, or another variable allocation. Diagnostics
must be valid UTF-8 of at most 192 bytes, unused bytes are zero, and
`ContentDiagnostic::try_from_str` rejects rather than truncates one-over input.
The `repr(u8)` error/retry tags and `repr(C)` inline diagnostic make
`ContentError` exactly 195 bytes; a compile-time size assertion protects the
256-byte allowance.
`push_*` validates material IDs, v1 flags, and
outside-domain canonical emptiness before advancing `written`. A rejected
write, an attempted `(len + 1)`th result, callback success with
`written != len`, or callback error/panic poisons the whole batch. Rejected
writes allocate nothing and remain terminal even if consumer code ignores the
returned `ContentWriteError`. `ContentWriteError` converts to
`ContentErrorKind::InvalidBatch` for ordinary `?` propagation.

The fixed response charge is a 256-byte callback/batch allowance plus 32 bytes
of control/tag storage and 2,048 bytes of sample storage for each exact output
slot. Moria may encode a completed homogeneous slot compactly only after the
callback returns successfully, but the full worst-case sink remains charged
until validation and copy/upload installation complete. The response echoes no
`SourceDescriptor` or other variable payload. `descriptor()` is an immutable
borrow tied to `&self`, never an owned return. Moria validates that borrowed
descriptor at registration, copies it into canonical bounded world ownership,
and compares the borrow with the request's fixed lineage/fingerprint
immediately before invocation. The borrowed source allocation remains
consumer-owned throughout and cannot become live Moria response ownership.
`ContentLineage.opaque` in Moria's canonical copy is an exact-length boxed
slice of at most 256 bytes.

On success, the already-owned sink becomes the validated installation input;
there is no second returned batch to reserve. On cancellation, source error,
panic, poisoned/incomplete output, or installation failure, Moria drops the
sink and then releases both permits. Late return after cancellation follows the
same drop-before-release rule. Consumer-internal transient allocations during
its own callback or behind its borrowed descriptor are outside Moria
ownership. Across the port the callback can only borrow source identity, copy
or borrow fixed-size values into the sink, and return the fixed inline error
record. Thus every simultaneously live Moria-owned output is covered before
invocation. Failed content is never installed partially.

The source descriptor supplies both:

- `ContentLineage`: stable family/version identity used for migration policy;
- `ReconstructionFingerprint([u8; 32])`: digest identifying the exact base
  inputs/algorithm needed to reproduce unscarred matter.

Restore requires both to match the checkpoint. A source cannot merely assert
lineage compatibility.

## Matter commands

```rust
pub enum MatterCommand {
    Fill {
        volume: VolumeId,
        target: CellAabb,
        sample: MaterialSample,
        precondition: RevisionPrecondition,
        correlation: Correlation,
    },
    Patch {
        volume: VolumeId,
        patch: MaterialPatch,
        precondition: RevisionPrecondition,
        correlation: Correlation,
    },
}

pub struct MaterialPatch {
    pub bounds: CellAabb,
    pub encoding: PatchEncoding,
}

pub enum PatchEncoding {
    Dense(Vec<MaterialSample>),
    Runs(Vec<MaterialRun>),
}

pub struct MaterialRun {
    pub start_index: u32,
    pub length: NonZeroU32,
    pub sample: MaterialSample,
}

pub struct MatterApplied {
    pub command: CommandId,
    pub volume: VolumeId,
    pub affected: CellAabb,
    pub revision: VolumeRevision,
    pub correlation: Correlation,
}
```

Canonical removal is `Fill` with `MaterialSample::EMPTY`; place/replace is
`Fill` with a registered nonempty sample. `Patch` covers consumer stamps.
Patch coordinates are volume-local. Dense order is X-fastest, then Y, then Z.
Dense length must equal the checked cell count. Run indices use that same
linear order; runs must be sorted, non-overlapping, inside the dense length,
and collectively within the configured cell/byte limits. Cells not covered by
a run are unchanged.

One command targets exactly one volume. Admission verifies identity, bounds,
materials, static structure, permit, and current precondition. Cold target
bricks may be admitted and materialized. Immediately before prepare, the
precondition is checked again; an intervening commit produces terminal
`Conflict` with no effect. Per-volume admitted commands prepare in FIFO order.

After all affected current bricks are available, Moria reserves every required
new slot, page node, scar record, and completion record. It writes new versions,
validates the transaction, then publishes one new revision. Failure before
publication releases reservations. Device loss after submission makes that
device generation unavailable and never reports a partial success.

`affected` is the requested intersection after complete-validation; commands
outside the volume are rejected rather than clipped. No-op writes are valid and
still commit a revision because the accepted command and correlation are
observable; telemetry marks `changed_samples = 0`.

## Volume commands

```rust
pub enum VolumeCommand {
    Create {
        definition: VolumeDefinition,
        source: Arc<dyn BaseContentSource>,
        correlation: Correlation,
    },
    Move {
        volume: VolumeId,
        placement: RigidPlacement,
        precondition: RevisionPrecondition,
        correlation: Correlation,
    },
    Retire {
        volume: VolumeId,
        precondition: RevisionPrecondition,
        correlation: Correlation,
    },
}

pub enum VolumeApplied {
    Created {
        command: CommandId,
        volume: VolumeId,
        key: VolumeKey,
        revision: VolumeRevision,
        correlation: Correlation,
    },
    Moved {
        command: CommandId,
        volume: VolumeId,
        placement: RigidPlacement,
        revision: VolumeRevision,
        correlation: Correlation,
    },
    Retired {
        command: CommandId,
        key: VolumeKey,
        terminal_revision: VolumeRevision,
        correlation: Correlation,
    },
}
```

Create reserves a stable/runtime identity and becomes applied only after its
directory entry is committed; content remains cold until interest. Move is
valid only for `Dynamic` and commits the placement at one new volume revision.
It does not resample local cells. Retire rejects new work, waits for admitted
work and checkpoint obligations, commits a tombstone revision, emits an
observation, and invalidates the runtime handle. Durable tombstones prevent a
saved key from being accidentally reused.

Builder registration and runtime `Create` both validate
`definition.debug_name` as 1..=96 UTF-8 bytes before accepting the definition.
`register_volume` returns `RegistrationError::InvalidDefinition`; a runtime
create returns `SubmitError::Invalid` and releases its command permit without
entering the queue. On acceptance, the directory canonicalizes the name to an
exact-length `Box<str>`; retirement transfers that box into the tombstone and
checkpoint encoding reads only the boxed length. No command-payload
reservation or discarded input `String` capacity becomes retained world
state.

`live_volumes` bounds concurrently live directory entries.
`volume_records` separately bounds all stable keys ever accepted by the world,
including current live volumes and retained retirement tombstones. Retiring a
volume releases one live slot but never releases its lifetime record. A create
at the live limit is synchronously invalid with
`ViolationCode::LiveVolumeCapacity`; a create after lifetime-key exhaustion is
`ViolationCode::VolumeRecordCapacity` with the supported limit. Neither enters
GPU work, and a retired key is always `ViolationCode::RetiredVolumeKey`, even
if live capacity is available. The whole-world manifest therefore contains at most
`volume_records` live-plus-tombstone entries.

Create, move, and retire do not share atomicity with matter commands or another
volume. Their receipts state whether a revision changed on failure.

## Queries

```rust
pub enum Query {
    Sample(SampleQuery),
    Region(RegionQuery),
    Occupancy(OccupancyQuery),
    Trace(TraceQuery),
    Overlap(OverlapQuery),
    Sweep(SweepQuery),
    Snapshot(SnapshotQuery),
}

pub struct QueryOptions {
    pub minimum: MinimumRevision,
    pub readiness: ReadinessPolicy,
    pub partial: PartialPolicy,
    pub max_results: u32,
    pub traversal: Option<TraversalAuthorization>,
}

pub struct TraversalAuthorization {
    pub max_candidate_bricks: NonZeroU32, // <= 8,192
    pub max_candidate_cells: NonZeroU32,  // <= 65,536
}

pub enum ReadinessPolicy {
    Pending,
    Materialize,
}

pub enum PartialPolicy {
    Deny,
    Allow { max_omitted_regions: u16 },
}

pub enum QueryScope {
    VolumeCells { volume: VolumeId, bounds: CellAabb },
    WorldBounds {
        bounds: WorldAabb,
        volumes: BoundedVolumeFilter,
    },
}

pub enum SampleAddress {
    VolumeCell { volume: VolumeId, cell: CellCoord },
    WorldPoint {
        point: WorldPoint,
        volumes: BoundedVolumeFilter,
    },
}

pub struct SampleQuery {
    pub address: SampleAddress,
    pub options: QueryOptions,
}

pub enum RegionEncoding {
    Dense,
    HomogeneousRuns,
}

pub struct RegionQuery {
    pub scope: QueryScope,
    pub encoding: RegionEncoding,
    pub options: QueryOptions,
}

pub enum OccupancyTarget {
    Point(SampleAddress),
    Region(QueryScope),
    Shape {
        shape: CollisionShape,
        volumes: BoundedVolumeFilter,
    },
}

pub struct OccupancyQuery {
    pub target: OccupancyTarget,
    pub include_first: u16,              // 0..=min(max_results, 4,096)
    pub options: QueryOptions,
}

pub struct WorldSegment {
    pub start: WorldPoint,
    pub end: WorldPoint,
}

pub struct TraceQuery {
    pub segment: WorldSegment,
    pub volumes: BoundedVolumeFilter,
    pub options: QueryOptions,
}

pub enum CollisionShape {
    Sphere { center: WorldPoint, radius: f32 },
    Aabb(WorldAabb),
    Capsule {
        a: WorldPoint,
        b: WorldPoint,
        radius: f32,
    },
}

pub struct OverlapQuery {
    pub shape: CollisionShape,
    pub volumes: BoundedVolumeFilter,
    pub options: QueryOptions,
}

pub struct SweepQuery {
    pub shape_at_start: CollisionShape,
    pub displacement: WorldVector,
    pub volumes: BoundedVolumeFilter,
    pub options: QueryOptions,
}

bitflags! {
    pub struct SnapshotContents: u8 {
        const VOLUME_STATE = 0b0001;
        const REGION_LIFECYCLE = 0b0010;
        const MATERIAL_SAMPLES = 0b0100;
        const OCCUPANCY = 0b1000;
    }
}

pub enum SnapshotScope {
    SubscriptionGap { subscriber: SubscriberId, max_bricks: u32 },
    SubscriptionState { subscriber: SubscriberId, max_bricks: u32 },
    Regions(Vec<QueryScope>),            // sorted, non-overlapping, <=256
}

pub struct SnapshotQuery {
    pub scope: SnapshotScope,
    pub contents: SnapshotContents,
    pub options: QueryOptions,
}

pub enum QueryAvailability {
    Ready(QueryResult),
    Pending {
        required: Vec<RequiredRegion>,
        retryability: Retryability,
    },
    Unavailable {
        scopes: Vec<UnavailableRegion>,
        retryability: Retryability,
    },
}

pub struct QueryOutcome {
    pub query: QueryId,
    pub inspected: Vec<InspectedRegion>,
    pub snapshots: Vec<VolumeSnapshotRef>,
    pub device_generation: DeviceGeneration,
    pub completeness: QueryCompleteness,
    pub availability: QueryAvailability,
}

pub enum QueryCompleteness {
    Complete,
    PartialRequested {
        coverage: CoverageMask,
        omitted: Vec<UnavailableRegion>,
    },
}

pub struct CoverageMask {
    pub bricks: Vec<BrickCoverage>,
}

pub struct BrickCoverage {
    pub volume: VolumeId,
    pub brick: BrickCoord,
    pub inspected: bool,
}

pub struct RequiredRegion {
    pub scope: QueryScope,
    pub reason: RequiredReason,
}

pub enum RequiredReason {
    Cold,
    Materializing,
    MinimumRevision,
}

pub struct UnavailableRegion {
    pub scope: QueryScope,
    pub reason: UnavailableReason,
}

pub enum UnavailableReason {
    FailedContent,
    RetiredVolume,
    RevisionUnavailable,
    CapabilityDisabled,
    Budget,
    DeviceUnavailable,
}

pub struct InspectedRegion {
    pub scope: QueryScope,
    pub revision: VolumeRevision,
}

pub struct VolumeSnapshotRef {
    pub volume: VolumeId,
    pub key: VolumeKey,
    pub revision: VolumeRevision,
    pub placement: RigidPlacement,
}

pub enum QueryResult {
    Samples(Vec<SampleFact>),
    Region(RegionSamples),
    Occupancy(OccupancyFact),
    Trace(Vec<ContactFact>),
    Overlap(Vec<ContactFact>),
    Sweep(Vec<ContactFact>),
    Snapshot(WorldSnapshot),
}

pub struct SampleFact {
    pub volume: VolumeId,
    pub revision: VolumeRevision,
    pub cell: CellCoord,
    pub sample: MaterialSample,
    pub world_center: WorldPoint,
}

pub struct RegionSamples {
    pub volumes: Vec<VolumeRegionSamples>,
}

pub struct VolumeRegionSamples {
    pub volume: VolumeId,
    pub revision: VolumeRevision,
    pub bounds: CellAabb,
    pub encoding: RegionSampleEncoding,
}

pub enum RegionSampleEncoding {
    Dense(Vec<MaterialSample>),
    HomogeneousRuns(Vec<RegionRun>),
}

pub struct RegionRun {
    pub start_index: u32,
    pub length: NonZeroU32,
    pub sample: MaterialSample,
}

pub struct OccupancyFact {
    pub occupied: bool,
    pub first: Vec<OccupiedCellFact>,
}

pub struct OccupiedCellFact {
    pub volume: VolumeId,
    pub revision: VolumeRevision,
    pub cell: CellCoord,
    pub sample: MaterialSample,
}

pub struct ContactFact {
    pub volume: VolumeId,
    pub revision: VolumeRevision,
    pub material: MaterialId,
    pub cell: CellCoord,
    pub world_point: WorldPoint,
    pub world_normal: WorldVector,
    pub penetration_or_toi: f32,
}

pub struct WorldSnapshot {
    pub scope: SnapshotScope,
    pub resolved_subscription: Option<AcceptedSubscription>,
    pub observation_frontier: ObservationFrontier,
    pub volumes: Vec<VolumeStateSnapshot>,
    pub regions: Vec<RegionStateSnapshot>,
    pub samples: Vec<SampleFact>,
    pub occupancy: Vec<OccupiedCellFact>,
    pub resume: Option<GapResumeToken>,
}

pub struct VolumeStateSnapshot {
    pub volume: VolumeId,
    pub key: VolumeKey,
    pub state: SnapshotVolumeState,
}

pub enum SnapshotVolumeState {
    Live {
        revision: VolumeRevision,
        placement: RigidPlacement,
        mode: VolumeMode,
    },
    Retired {
        terminal_revision: VolumeRevision,
    },
}

pub struct RegionStateSnapshot {
    pub volume: VolumeId,
    pub bounds: CellAabb,
    pub state: RegionLifecycleState,
}

pub enum RegionLifecycleState {
    Cold,
    Requested,
    Materializing,
    Ready { revision: VolumeRevision },
    Retiring,
    Failed { failure: RegionFailure },
}

pub struct RegionFailure {
    pub kind: RegionFailureKind,
    pub retryability: Retryability,
    pub device_generation: Option<DeviceGeneration>,
    pub diagnostic: String,              // <=512 UTF-8 bytes
}

pub enum RegionFailureKind {
    Content(ContentErrorKind),
    InvalidContent,
    BudgetExhausted(ResourceKind),
    DeviceLost,
    RevisionUnavailable,
    Retired,
    InternalInvariant,
}

pub struct GapResumeToken { /* opaque subscriber/scope/head digest */ }
```

Every concrete query embeds exactly one `QueryOptions`; there is no implicit
default attached by `submit_query`. `max_results` must be nonzero except for an
occupancy request with `include_first = 0`, and cannot exceed the fixed limit
for that query family or its permit bytes. World filters, query scopes, and
snapshot region vectors are normalized and validated before admission.
Collision inputs require finite values, positive radii, a nonzero trace segment
or sweep displacement, and complete shape bounds inside the supported checked
coordinate range.

Shape/region occupancy, trace, overlap, and sweep require
`Some(TraversalAuthorization)`; point occupancy and noncollision query
families require `None`. Before admission, the control plane transforms the
finite world bounds into each selected volume, intersects its domain, and
checked-sums the conservative brick-AABB and cell-AABB counts across volumes.
Both totals must fit the request authorization and the fixed 8,192-brick /
65,536-cell maxima. The authorization therefore bounds worst-case traversal
work even when occupancy is sparse or `max_results` is small. An excess is a
synchronous `ViolationCode::TooLarge` whose `SupportedBounds` reports both
candidate maxima; partial coverage never authorizes extra traversal.

All query outcomes include query ID, actual inspected bounds, device
generation, a sorted `VolumeSnapshotRef` vector, completeness, and
result-specific facts.

- `Sample` returns every volume sample covering a world point, or the one
  addressed local sample. Overlapping volumes are preserved.
- `Region` returns row-major samples or homogeneous runs for a bounded
  volume-local region. World-region queries return records grouped by volume;
  they never merge materials.
- `Occupancy` returns occupied/unoccupied only when every required sample is
  ready. It can optionally return the first occupied facts.
- `Trace` returns ordered cell encounters along a finite segment.
- `Overlap` tests a supported shape at one placement.
- `Sweep` tests a supported shape along a finite displacement and reports
  time-of-impact facts without moving anything.
- `Snapshot` returns lifecycle/revision/placement summaries and may include
  bounded material data for observation-gap recovery.

`SnapshotScope::SubscriptionGap` is accepted only while that subscriber is in
`NeedsSnapshot`, requires `SnapshotContents::VOLUME_STATE`, and requires
`QueryOptions.max_results` to cover every pinned member; either shortfall is
rejected before admission rather than clipping membership. Its result has
`resolved_subscription = Some` containing the
exact accepted subscription and pinned volume IDs, and `GapResumeToken` binds
subscriber ID, resolved-scope digest, gap head, captured nonempty observation
frontier, and captured revisions. A gap implies at least one prior fact, so
this snapshot's frontier is always `Retained`. Its `volumes` vector has exactly
one stable-key-sorted record for every pinned member. A live member uses
`SnapshotVolumeState::Live`; a member retired before the captured head uses
`Retired { terminal_revision }` even when the retirement fact was overwritten.
The historical `VolumeId` in a retired record identifies the accepted member
but remains stale for every operation; `VolumeKey` is its durable identity.
The retained tombstone supplies this state without recreating a live handle.
Explicit region snapshots accept live scopes only and therefore return only
`Live` volume records. Explicit region snapshots have
`resolved_subscription = None` and `resume = None`. `resume_after` rejects a token from another subscriber,
scope, or an older gap; this is the complete race-closing contract and does not
depend on caller-provided sequence arithmetic.

`SnapshotScope::SubscriptionState` is the non-resuming form used to reconcile
a nonadvancing GPU observation view. It has the same bounded, exact pinned
live/retired membership requirement as `SubscriptionGap`, captures
`observation_frontier`, and has `resume = None`. It is legal regardless of the
CPU subscriber cursor state and never changes that cursor. Before the first
fact it returns `ObservationFrontier::Empty`; otherwise it returns the retained
oldest/head pair. The caller restarts an independent GPU delta read after the
frontier head, using `after = None` for `Empty`; this reconciles current state
rather than pretending overwritten or unsupported facts were delivered.
Explicit region snapshots use the same frontier rule, so a snapshot before
sequence one is representable without inventing an observation.

Supported collision shapes are sphere, axis-aligned box, and capsule. Inputs
are finite and nondegenerate. Trace/sweep results sort by parametric distance,
then stable volume key, then local cell coordinate. Coincident overlaps are all
retained up to the explicit result cap.

If readiness is `Pending`, cold data yields `QueryAvailability::Pending` with
the required regions and no fabricated facts. `Materialize` creates internal
query interest bounded by the query permit. It does not evade interest or
residency budgets.

The `Query` variant and `QueryResult` variant must match as follows:
`Sample -> Samples`, `Region -> Region`, `Occupancy -> Occupancy`,
`Trace -> Trace`, `Overlap -> Overlap`, `Sweep -> Sweep`, and
`Snapshot -> Snapshot`. Pending/unavailable outcomes contain no result variant.
All vectors are bounded by the permit, `max_results`, and fixed request
maxima; decoding an excess count is `OutputOverflow`.

`PartialPolicy::Deny` either returns complete facts or a non-success
availability/error. `Allow` returns one `BrickCoverage` for every resolved
brick, explicit unavailable omitted regions, and
`Complete | PartialRequested`; the coverage vector is bounded by the request's
resolved brick maximum. Hitting a result cap without prior partial
authorization is `OutputOverflow`, not success. In v1, partial means only that
explicitly named unavailable spatial regions were omitted. It never permits
hit truncation inside an inspected brick: if collision output exceeds
`max_results`, the receipt fails `OutputOverflow { required, limit }` under
both partial policies and returns no `QueryResult`.

## Observation

```rust
pub struct Subscription {
    pub volumes: BoundedVolumeFilter,
    pub bounds: Option<ObservationBounds>,
    pub kinds: ObservationKinds,
}

pub enum ObservationBounds {
    VolumeLocal { volume: VolumeId, bounds: CellAabb },
    World(WorldAabb),
}

bitflags! {
    pub struct ObservationKinds: u16 {
        const MATTER = 0x0001;
        const VOLUME = 0x0002;
        const LIFECYCLE = 0x0004;
        const PRESENTATION = 0x0008;
        const CHECKPOINT = 0x0010;
        const PRESSURE = 0x0020;
        const DEVICE = 0x0040;
    }
}

pub enum ObservationItem {
    Fact(Observation),
    Gap(ObservationGap),
}

pub struct Observation {
    pub sequence: ObservationSequence,
    pub fact: ObservationFact,
}

pub enum ObservationFact {
    MatterCommitted {
        command: CommandId,
        volume: VolumeId,
        affected: CellAabb,
        revision: VolumeRevision,
        correlation: Correlation,
    },
    VolumeCreated {
        command: CommandId,
        volume: VolumeId,
        key: VolumeKey,
        revision: VolumeRevision,
        correlation: Correlation,
    },
    VolumeMoved {
        command: CommandId,
        volume: VolumeId,
        revision: VolumeRevision,
        placement: RigidPlacement,
        correlation: Correlation,
    },
    VolumeRetired {
        command: CommandId,
        key: VolumeKey,
        terminal_revision: VolumeRevision,
        correlation: Correlation,
    },
    RegionLifecycle(RegionStateSnapshot),
    Presentation {
        volume: VolumeId,
        brick: BrickCoord,
        state: PresentationState,
    },
    Checkpoint {
        key: CheckpointKey,
        outcome: CheckpointObservation,
    },
    ResourcePressure(ResourcePressureFact),
    Device {
        generation: DeviceGeneration,
        state: DeviceObservationState,
    },
}

pub enum CheckpointObservation {
    Durable { revisions: Vec<(VolumeKey, VolumeRevision)> },
    Failed {
        kind: PersistenceErrorKind,
        retryability: Retryability,
    },
}

pub struct ResourcePressureFact {
    pub resource: ResourceKind,
    pub used: u64,
    pub limit: u64,
    pub action: PressureAction,
}

pub enum PressureAction {
    Deferred,
    Rejected,
    Coalesced,
    EvictedDerived,
    EvictedAuthority,
}

pub enum ResourceKind {
    Materials,
    MaterialMetadataPerRegistration,
    MaterialMetadataBytes,
    LiveVolumes,
    VolumeRecords,
    InterestLeases,
    BricksPerInterest,
    DetailedBricks,
    PageKeys,
    PageVersions,
    VersionsPerBrick,
    DirtyScars,
    CommandRecords,
    CommandPayloadBytes,
    QueryRecords,
    QueryResultBytes,
    ObservationFacts,
    ObservationPayloadBytes,
    Subscribers,
    VolumesPerFilter,
    StagingMaps,
    StagingBytes,
    ContentRequests,
    ContentBricksPerRequest,
    ContentResponseBytes,
    PersistenceRequests,
    PersistenceStagedBytes,
    ExtractionRecords,
    ExtractionBytes,
    PresentationJobs,
    PresentationArtifacts,
    PresentationDirtyRecords,
    MeshVertices,
    MeshIndices,
    DressingStyles,
    DressingInstances,
    BehaviorEngines,
    BehaviorOrderEdges,
    BehaviorScopesPerEngine,
    BehaviorViewVolumes,
    BehaviorViewBricks,
    BehaviorViewCells,
    BehaviorCpuViewBytes,
    BehaviorGpuViewBytes,
    BehaviorInputRecords,
    BehaviorInputBytes,
    BehaviorGpuInputBytes,
    BehaviorCollisionCalls,
    BehaviorCollisionContacts,
    BehaviorCollisionBytes,
    BehaviorHandoffMaps,
    BehaviorHandoffBytes,
    BehaviorProposalRecords,
    BehaviorProposalBytes,
    BehaviorEffectCells,
    BehaviorEffectBricks,
    BehaviorDirectoryEffects,
    BehaviorConflictChecks,
    BehaviorFeedbackBytes,
    BehaviorGpuBuffers,
    BehaviorGpuBufferBytes,
    BehaviorGpuPipelines,
    BehaviorGpuBindGroups,
    BehaviorGpuWgslBytes,
    BehaviorGpuDispatches,
    BehaviorGpuWorkgroups,
    BehaviorPlacementUpdates,
    BehaviorPlacementBytes,
    BehaviorComponentExtractionProposals,
    BehaviorComponentExtractionChildren,
    BehaviorComponentExtractionAssignmentCells,
    BehaviorComponentExtractionChildBricks,
    BehaviorComponentExtractionBytes,
    BehaviorEgressMaps,
    BehaviorEgressReceipts,
    BehaviorEgressRecords,
    BehaviorEgressDeviceBytes,
    BehaviorEgressStagingBytes,
    BehaviorEgressHostBytes,
    ExtensionJobs,
    ExtensionRegistrations,
    ExtensionRegistryBytes,
    ExtensionPacketBytes,
    ExtensionStateBytes,
    ExtensionCandidateEffects,
}

pub enum DeviceObservationState {
    Lost,
    Recovering,
    Ready,
    Failed,
}

pub struct ObservationGap {
    pub last_delivered: Option<ObservationSequence>,
    pub oldest_retained: ObservationSequence,
    pub current_head: ObservationSequence,
    pub scope: AcceptedSubscription,
    pub trustworthy_revisions: Vec<(VolumeId, VolumeRevision)>,
}

pub struct AcceptedSubscription {
    pub request: Subscription,
    pub volumes: Vec<VolumeSnapshotRef>, // stable-key sorted snapshot
    pub initial_after: Option<ObservationSequence>,
}

pub enum SubscriptionStart {
    CurrentHead,
    Retained(ObservationSequence),       // requested sequence is first candidate
}

pub struct ObservationSubscriber { /* Send + Sync, bounded cursor */ }

impl ObservationSubscriber {
    pub fn id(&self) -> SubscriberId;
    pub fn accepted(&self) -> AcceptedSubscription;
    pub fn try_next(&self) -> Result<Option<ObservationItem>, ObservationError>;
    pub fn resume_after(
        &self,
        snapshot: &WorldSnapshot,
    ) -> Result<(), ResumeError>;
}
```

Facts cover committed matter, volume create/move/retire, lifecycle changes,
presentation status, checkpoint completion/failure, resource pressure, and
device recovery. Every fact has a world sequence; change facts also carry the
relevant volume revision and correlation.

Subscriptions are bounded by volume count and one optional spatial bound. Each
subscriber has a cursor into a shared configured ring. Polling never blocks a
commit. If overwritten, the next item is exactly one `Gap` containing the last
delivered sequence, current oldest sequence, current head, affected subscription
scope, and last trustworthy revisions known at the cursor. The subscriber must
obtain a bounded `Snapshot` and call `resume_after(snapshot)`; no later facts
are delivered before that.

Filtering at poll time uses immutable append-time metadata, never the current
volume directory. Each ring record contains a private, fixed-size 128-byte
`FilterEnvelopeV1` encoded as a valid-field mask, the affected `VolumeId`, and
four six-word AABBs: prior/current volume-local bounds and prior/current world
bounds. Local words are `i32`; world words are finite `f32` bit patterns.
Unused fields are zero and masked out. Matter, lifecycle, and presentation
facts record their affected local bounds and conservative transformed world
bounds at the fact revision. Create records the new domain; retirement records
the last live domain/placement; move records both the prior and new placed
domain. The envelope is built before an old directory version can be reclaimed,
is retained and evicted atomically with its public fact, and is charged to
`observation_payload_bytes`. It is private filtering metadata, not another
world-truth representation or a public fact payload.

`ObservationBounds::VolumeLocal` compares the envelope's valid local extents
after matching the pinned ID. `ObservationBounds::World` compares either valid
world extent, which makes a move match its old or new placement even after both
directory versions are reclaimed. Non-spatial checkpoint, pressure, and device
facts are selected by `ObservationKinds` and ignore the optional spatial
predicate. A malformed or missing required envelope is an internal invariant
failure at append, not a poll-time guess from current placement.

Subscription volume membership is snapshotted at `subscribe`. `Include`
captures the named live handles and `All` captures every then-live volume,
stable-key sorted; `max_volumes` is checked against that complete set. Later
creates are excluded, retirement of a captured volume is delivered when
`VOLUME` is selected and then leaves that pinned member terminal, and no new
volume substitutes for it. `accepted()` and every gap expose the resolved
membership.

`AcceptedSubscription.initial_after` is the subscriber's immutable lower
cursor bound. `CurrentHead` records `Some(head)` for a retained frontier and
`None` for an empty frontier. `Retained(S)` records the checked predecessor of
`S`, or `None` when `S` is the first world sequence; `S` must still be
retained. CPU polling and nonadvancing GPU reads therefore share an explicit
legal start without sharing cursor mutation.

Unlike interest brick residency, an optional subscription spatial bound is an
event predicate over that pinned membership: matter/lifecycle/presentation
facts match their affected world bounds at the fact revision, and a move fact
matches when either its prior or new placed domain intersects the bound. Thus a
captured dynamic volume can move into or out of the bound without changing the
finite ID set. To include later-created volumes, the consumer creates a new
subscription and closes/drops the old subscriber.

Checkpoint revision vectors and gap revision vectors are stable-key/runtime-ID
sorted and bounded by `volume_records` and `volumes_per_filter` respectively.
Checkpoint vectors and all other variable retained fact payloads are charged
to `observation_payload_bytes`, as is every fixed filter envelope; overwrite
advances on whole fact-plus-envelope records until both ring count and byte
capacity are available. Gap vectors are materialized from the subscriber's
fixed-capacity revision array reserved at subscription time, not from an
unbounded allocation.
`ResourceKind` is the closed set of every field in `ResourceLimits`, so
pressure on extraction, lifetime volume records, presentation artifacts/dirty
records/instances, or extension registry storage is observable through the
same fact.

Observations are not a command bus. A subscriber receives no storage or
mutation privilege.

## Presentation API

Presentation is requested through interest. Consumers register material
surface inputs and choose:

```rust
pub enum StaleViewPolicy {
    DisplayStale,
    HideUntilCurrent,
    DiagnosticBounds,
}

pub enum PresentationState {
    Absent,
    Building { target: VolumeRevision },
    Current { source: VolumeRevision },
    Stale { visible: VolumeRevision, target: VolumeRevision },
    Failed { target: VolumeRevision, error: PresentationError },
}

pub enum PresentationError {
    AssetUnavailable {
        style: Option<DressingStyleKey>,
        asset_kind: PresentationAssetKind,
    },
    OutputOverflow { resource: ResourceKind, required: u64, limit: u64 },
    InvalidGeometry,
    GpuValidation,
    DeviceLost,
}

pub enum PresentationAssetKind {
    SurfaceMaterial,
    TriplanarAlbedo,
    TriplanarNormal,
    DressingMesh,
    DressingMaterial,
}
```

The Bevy adapter owns render entities and mesh assets and tags them with opaque
volume/brick/revision components for diagnostics. These components do not grant
query access. Surface registration is the `MaterialDefinition.presentation`
value, and dressing registration is
`MoriaBuilder::register_dressing_style`; there is no unnamed presentation
registration path. When presentation is enabled, validation requires every
style filter key to resolve and every material to have a valid surface
descriptor before presentation interest can become ready.

Each dressing descriptor is fixed-size except its 1..=64 material-key filter
and 96-byte name. Descriptor count is bounded by `dressing_styles`; generated
instances are bounded by both `max_instances_per_artifact` and the global
`dressing_instances` pool. Dressing has no occupancy. Matter-backed objects use
ordinary volume creation instead.

## Persistence API

```rust
pub struct ChunkDigest(pub [u8; 32]);

pub struct PersistenceError {
    pub kind: PersistenceErrorKind,
    pub retryability: Retryability,
    pub diagnostic: String,
}

pub enum PersistenceErrorKind {
    NotFound,
    Io,
    UnsupportedDurability,
    UnexpectedEof,
    SizeChanged,
    Bounds,
    SizeLimit,
    Corrupt,
    UnsupportedVersion { saved: u16, supported: u16 },
    RestoreMismatch(RestoreMismatch),
    Panicked,
}

pub enum RestoreMismatch {
    WorldKey,
    MaterialMissing(MaterialKey),
    MaterialDefinition(MaterialKey),
    VolumeMembership,
    TombstonedVolume(VolumeKey),
    VolumeDefinition(VolumeKey),
    Lineage(VolumeKey),
    ReconstructionFingerprint(VolumeKey),
}

pub trait CheckpointStore: Send + Sync + 'static {
    fn begin(&self, checkpoint: CheckpointKey)
        -> Result<Box<dyn CheckpointWriter>, PersistenceError>;
    fn open(&self, checkpoint: CheckpointKey)
        -> Result<Box<dyn CheckpointReader>, PersistenceError>;
}

pub trait CheckpointWriter: Send {
    fn put_chunk(&mut self, id: ChunkDigest, bytes: &[u8])
        -> Result<(), PersistenceError>;
    fn commit_manifest(self: Box<Self>, bytes: &[u8])
        -> Result<(), PersistenceError>;
    fn abort(self: Box<Self>) -> Result<(), PersistenceError>;
}

pub trait CheckpointReader: Send {
    fn manifest_len(&mut self) -> Result<u64, PersistenceError>;
    fn read_manifest(
        &mut self,
        offset: u64,
        destination: &mut [u8],
    ) -> Result<(), PersistenceError>;
    fn chunk_len(&mut self, id: ChunkDigest) -> Result<u64, PersistenceError>;
    fn read_chunk(
        &mut self,
        id: ChunkDigest,
        offset: u64,
        destination: &mut [u8],
    ) -> Result<(), PersistenceError>;
}
```

`commit_manifest` is the atomic durability point. After it succeeds, every
referenced chunk must be durable and readable. A store incapable of this
contract is rejected. Moria includes a native filesystem store using sibling
temporary files, file sync, atomic rename, and parent-directory sync where the
platform supports it.

Reader lengths are authoritative size discovery and may be queried more than
once. Reads must fill the complete destination or return
`PersistenceError::UnexpectedEof`; `offset + destination.len()` is checked and
must not exceed the discovered length. The reader retains ownership of store
handles, while Moria owns each bounded destination buffer from its persistence
staging pool. A manifest over `max_manifest_bytes` (64 MiB v2), a chunk over
4 MiB encoded, a changing reported length, missing data, or backend I/O failure
is returned as a distinct persistence error before allocation/decoding. Reader
methods execute only on persistence workers and never receive a Moria storage
handle.

```rust
pub enum CheckpointScope {
    WholeWorld,
}

pub struct CheckpointRequest {
    pub key: CheckpointKey,
    pub scope: CheckpointScope,
}

pub struct CheckpointApplied {
    pub key: CheckpointKey,
    pub durable: Vec<(VolumeKey, VolumeRevision)>,
    pub manifest: ChunkDigest,
}

pub struct RestoreRequest {
    pub checkpoint: CheckpointKey,
    pub world: RestoreWorldMode,
}

pub enum RestoreWorldMode {
    RequireSameKey,
    ImportAs(WorldKey),
}

pub struct RestoreApplied {
    pub checkpoint: CheckpointKey,
    pub saved_world: WorldKey,
    pub active_world: WorldKey,
    pub imported: bool,
    pub revisions: Vec<(VolumeKey, VolumeRevision, RigidPlacement)>,
    pub manifest: ChunkDigest,
}
```

The checkpoint frontier is captured when the request is admitted. Later
commits remain dirty and are excluded. V2 checkpoints are whole-world only:
the manifest contains every live volume at the captured frontier plus every
known retirement tombstone. It cannot omit a live volume, and no partial-scope
variant is reserved.

`restore_from` selects the builder's startup mode and may be called once.
`RequireSameKey` requires manifest and `WorldDefinition` keys to match.
`ImportAs(k)` requires the builder world key to equal `k`, preserves material
and volume keys, and changes only the containing world identity. Restore
validates all registrations and base fingerprints before publishing any
volume. Its output is `StartupApplied::mode =
StartupModeApplied::Restored(RestoreApplied)`; startup failure uses the normal
receipt error with a restore-specific stage and publishes no world directory.

The current live volume registration set must exactly equal the manifest's
`ExternalBase` live-volume key set. Missing and extra external volumes are
both `RestoreMismatch::VolumeMembership`; tombstoned and
`DerivedExtraction` keys may not be registered. Moria reconstructs saved derived
children from their persisted sparse base/provenance and rejects a missing or
corrupt derived base before directory publication.
Every persisted material must have a matching current key and
occupancy-relevant definition. Extra current materials are allowed regardless
of presentation inputs, because no persisted sample refers to them; they must
have distinct keys and valid ordinary definitions. There is no
“presentation-only material” category in v2.

## Scheduled behavior-engine hook

The types in this section are the first-class substrate-tick integration.
The full schedule, synchronization, composition, and state-ownership contract
is in [behavior-scheduling.md](behavior-scheduling.md).

```rust
pub struct BehaviorEngineDescriptor {
    pub key: BehaviorEngineKey,
    pub debug_name: String,                  // 1..=96 UTF-8 bytes
    pub execution: BehaviorExecution,
    pub runs_after: Vec<BehaviorEngineKey>,  // sorted unique
    pub consumer_input: BehaviorConsumerInputPolicy,
    pub maximum_consumer_input_bytes: u32,   // 0 or 1..=1 MiB
    pub maximum_access: BehaviorAccessEnvelope,
    pub maximum_proposals: u32,
    pub maximum_proposal_bytes: u64,
    pub maximum_effect_cells: u32,
    pub maximum_effect_bricks: u32,
    pub maximum_directory_effects: u32,
    pub maximum_collision_calls: u32,       // zero for GPU
    pub maximum_collision_contacts: u32,    // zero for GPU
    pub handoffs_to: Vec<BehaviorHandoffDescriptor>,
    pub readiness: BehaviorReadiness,
    pub failure: BehaviorFailurePolicy,
    pub conflict: BehaviorConflictPolicy,
    pub maximum_owned_gpu_bytes: u64,
    pub maximum_gpu_buffers: u32,             // zero for CPU
    pub maximum_gpu_pipelines: u32,           // zero for CPU
    pub maximum_gpu_bind_groups: u32,         // zero for CPU
    pub maximum_gpu_wgsl_bytes: u64,           // zero for CPU
    pub maximum_gpu_dispatches: u32,          // zero for CPU
    pub maximum_gpu_workgroups: u64,          // zero for CPU
    pub maximum_placement_updates: u32,       // zero for CPU
    pub maximum_component_extraction_proposals: u32,      // zero for CPU
    pub maximum_component_extraction_children: u32,       // zero for CPU
    pub maximum_component_extraction_assignment_cells: u32, // zero for CPU
    pub maximum_component_extraction_child_bricks: u32,   // zero for CPU
    pub maximum_component_extraction_payload_bytes: u64,  // zero for CPU
    pub cpu_egress: BehaviorCpuEgressDescriptor,
}

pub enum BehaviorExecution { Cpu, Gpu }
pub enum BehaviorConsumerInputPolicy { None, Optional, Required }
pub enum BehaviorReadiness { RequireReady, Materialize }
pub enum BehaviorFailurePolicy { AbortTick, SkipParticipant }
pub enum BehaviorConflictPolicy { RejectLater, ReplaceEarlier, FailTick }

pub struct BehaviorCpuEgressDescriptor {
    pub schema: [u8; 16],
    pub record_stride: u32,                 // multiple of 4, 4..=65,536
    pub maximum_records: u32,
    pub maximum_bytes: u64,
}

pub struct BehaviorHandoffDescriptor {
    pub successor: BehaviorEngineKey,
    pub maximum_bytes: u32,                 // 1..=1 MiB; opaque to Moria
}

pub struct BehaviorAccessEnvelope {
    pub allowed_volumes: BoundedVolumeFilter,
    pub local_bounds: Vec<(VolumeId, CellAabb)>,
    pub world_bounds: Option<WorldAabb>,
    pub maximum_scopes: u32,
    pub maximum_volumes: u32,
    pub maximum_bricks: u32,
    pub maximum_cells: u32,
    pub traversal: TraversalAuthorization,
}

pub struct BehaviorAccessSink<'a> { /* Moria-owned exact-capacity scope sink */ }

impl BehaviorAccessSink<'_> {
    pub fn push(&mut self, scope: BehaviorScope) -> Result<(), BehaviorAdapterError>;
}

pub enum BehaviorScope {
    VolumeCells { volume: VolumeId, bounds: CellAabb },
    WorldCells { bounds: WorldAabb, maximum_volumes: u32 },
}

pub struct BehaviorPlanContext<'a> {
    pub tick: BehaviorTickId,
    pub correlation: Correlation,
    pub consumer_input_present: bool,
    pub consumer_input: &'a [u8],
}

pub struct BehaviorTickRequest {
    pub correlation: Correlation,
    pub inputs: Box<[BehaviorParticipantInput]>,
}

pub struct BehaviorParticipantInput {
    pub engine: BehaviorEngineId,
    pub bytes: Box<[u8]>,
}

pub type BehaviorTickReceipt = Receipt<BehaviorTickCompleted>;

pub struct BehaviorTickAdmission {
    pub tick: BehaviorTickReceipt,
    pub egress: Vec<BehaviorEgressAdmission>,
}

pub struct BehaviorEgressAdmission {
    pub engine: BehaviorEngineId,
    pub receipt: BehaviorEgressReceipt,
}

pub struct BehaviorTickCompleted {
    pub tick: BehaviorTickId,
    pub correlation: Correlation,
    pub snapshot: Vec<VolumeSnapshotRef>,
    pub disposition: BehaviorTickDisposition,
    pub revision_changed: bool,
    pub participants: Vec<BehaviorParticipantOutcome>,
    pub proposals: Vec<BehaviorProposalOutcome>,
    pub published: Vec<(VolumeId, VolumeRevision)>,
}

pub enum BehaviorTickDisposition {
    Published,
    NoPublication { cause: BehaviorTickAbortCause },
    PublishedWithNotificationFailure { failed_hooks: u32 },
}

pub enum BehaviorTickAbortCause {
    ParticipantAbort { engine: BehaviorEngineId },
    ConflictFailTick {
        earlier_engine: BehaviorEngineId,
        earlier_proposal: u32,
        later_engine: BehaviorEngineId,
        later_proposal: u32,
    },
    TransitionFailure {
        predecessor: BehaviorEngineId,
        successor: BehaviorEngineId,
        stage: BehaviorTransitionStage,
    },
    DeviceLost { generation: DeviceGeneration },
    PreparationFailure,
}

pub struct BehaviorParticipantOutcome {
    pub engine: BehaviorEngineId,
    pub execution: BehaviorParticipantExecution,
    pub publication: BehaviorParticipantPublication,
    pub notification: BehaviorNotificationOutcome,
    pub egress: BehaviorEgressTerminal,
}

pub enum BehaviorParticipantExecution {
    Completed,
    Skipped { failure: BehaviorEngineFailure },
    NotRun { reason: BehaviorParticipantNotRunReason },
}

pub enum BehaviorParticipantNotRunReason {
    InputPreflightAborted { failed_engine: BehaviorEngineId },
    DeviceLost { generation: DeviceGeneration },
}

pub enum BehaviorParticipantPublication {
    Published { revision_changed: bool },
    NoSelectedEffect,
    DiscardedByTick { cause: BehaviorTickAbortCause },
}

pub enum BehaviorNotificationOutcome {
    Delivered,
    NotApplicable,
    FailedAfterTerminalDecision { publication_was_complete: bool },
}

pub enum BehaviorProposalOutcome {
    AdmittedMatter {
        engine: BehaviorEngineId,
        proposal: u32,
        command: CommandId,
        receipt: Receipt<MatterApplied>,
    },
    AdmittedVolume {
        engine: BehaviorEngineId,
        proposal: u32,
        command: CommandId,
        receipt: Receipt<VolumeApplied>,
    },
    AdmittedPlacementStream {
        engine: BehaviorEngineId,
        proposal: u32,
        command: CommandId,
        receipt: Receipt<PlacementStreamApplied>,
    },
    AdmittedComponentExtraction {
        engine: BehaviorEngineId,
        proposal: u32,
        command: CommandId,
        receipt: Receipt<ComponentExtractionApplied>,
    },
    Rejected {
        engine: BehaviorEngineId,
        proposal: u32,
        reason: BehaviorProposalRejection,
    },
}

pub enum BehaviorProposalRejection {
    OverlapsEarlier { engine: BehaviorEngineId, proposal: u32 },
    ReplacedByLater { engine: BehaviorEngineId, proposal: u32 },
    Invalid,
    ParticipantFailed,
    PreparationFailed { volume: VolumeId },
    TickAborted { cause: BehaviorTickAbortCause },
    MatterConservation,
    ComponentIdentity,
    PlacementStreamInvalid,
}

pub enum BehaviorEngineFailure {
    Planning,
    ConsumerInputUpload,
    Unavailable { regions: Vec<UnavailableRegion> },
    AccessLimit { resource: ResourceKind, requested: u64, limit: u64 },
    EffectLimit { requested_records: u32, requested_bytes: u64 },
    InvalidProposal,
    Panicked,
    GpuValidation,
    Transition {
        predecessor: BehaviorEngineId,
        successor: BehaviorEngineId,
        stage: BehaviorTransitionStage,
    },
    DeviceLost { generation: DeviceGeneration },
    NotReadyForGeneration { generation: DeviceGeneration },
    Shutdown,
}

pub enum BehaviorTransitionStage {
    CpuWrite,
    Upload,
    GpuValidate,
    GpuCopy,
    Map,
    Decode,
}

#[repr(C)]
pub struct BehaviorAdapterError {
    pub kind: BehaviorAdapterErrorKind,
    pub diagnostic: BehaviorDiagnostic, // fixed inline, <=192 UTF-8 bytes
}

#[repr(u32)]
pub enum BehaviorAdapterErrorKind {
    NotReady,
    InvalidPlan,
    InvalidState,
    Capacity,
    WorldGpuBufferCapacity,
    RendererOutOfMemory,
    Device,
    Internal,
}

#[repr(C)]
pub struct BehaviorDiagnostic {
    length: u16,
    reserved: u16,
    bytes: [u8; 192],
}

pub struct DirectoryGeneration(NonZeroU64); // multi-volume visibility; aliases world directory generation
pub struct ComponentPieceHandle(NonZeroU32);

pub enum ComponentPieceDisposition {
    PublishChild,
    RemoveFromMatter,
}

pub struct ComponentChildApplied {
    pub piece: ComponentPieceHandle,
    pub volume: VolumeId,
    pub key: VolumeKey,
    pub revision: VolumeRevision,
    pub local_domain: CellAabb,
    pub placement: RigidPlacement,
    pub sample_count: u32,
    pub sample_digest: [u8; 32],
}

pub struct ComponentExtractionApplied {
    pub command: CommandId,
    pub source: VolumeId,
    pub source_revision: Option<VolumeRevision>,
    pub children: Vec<ComponentChildApplied>,
    pub removed_cells: u32,
    pub removed_digest: [u8; 32],
    pub directory_generation: DirectoryGeneration,
    pub correlation: Correlation,
}

pub struct PlacementStreamApplied {
    pub command: CommandId,
    pub updated: Vec<(VolumeId, VolumeRevision)>,
    pub directory_generation: DirectoryGeneration,
    pub correlation: Correlation,
}

pub type BehaviorEgressReceipt = Receipt<BehaviorEgressCompleted>;

pub struct BehaviorEgressCompleted {
    pub tick: BehaviorTickId,
    pub engine: BehaviorEngineId,
    pub correlation: Correlation,
    pub schema: [u8; 16],
    pub record_stride: u32,
    pub record_count: u32,
    pub bytes: Box<[u8]>,
}

pub enum BehaviorEgressTerminal {
    Disabled,
    Pending { receipt: BehaviorEgressReceipt },
    Unavailable { reason: BehaviorEgressFailure },
}

pub enum BehaviorEgressFailure {
    ParticipantUnavailable { reason: BehaviorEgressParticipantUnavailable },
    Overflow { required_records: u32, capacity: u32 },
    CounterOverflow,
    InvalidHeader,
    GpuValidation,
    ReadbackMap,
    Decode,
    CancelledBeforePreparation,
    Shutdown,
    DeviceLost { generation: DeviceGeneration },
}

pub enum BehaviorEgressParticipantUnavailable {
    Skipped(BehaviorEngineFailure),
    NotRun(BehaviorParticipantNotRunReason),
}
```

`BehaviorDiagnostic::try_from_str` accepts at most 192 UTF-8 bytes and rejects
193 rather than truncating; reserved is always zero. The diagnostic is exactly
196 bytes and `BehaviorAdapterError` is exactly 200 bytes.
`BehaviorResourceReport` is a fixed-layout registry snapshot computed by
Moria's restricted factory; it is never an adapter-supplied capacity claim.
Planning writes scopes only into the preallocated sink; a failed, incomplete,
or over-capacity plan transfers no collection ownership.

Once a tick atomically enters `Preparing`, whose first behavior-specific stage
is GPU input preflight, its receipt resolves successfully with
`BehaviorTickCompleted` even when `disposition` is `NoPublication`; this keeps
every participant/proposal discard outcome observable. In that variant
`revision_changed` is false and every otherwise valid discarded proposal is
`TickAborted` with the same closed cause. Generic receipt failure remains only
for loss/cancellation before input preflight or a broken coordinator that
cannot produce the required report. An input-preflight failure has empty
`snapshot`, `proposals`, and `published` vectors. The upload-failed participant
is `Skipped { ConsumerInputUpload }`; every other participant is
`NotRun { InputPreflightAborted { failed_engine } }`. On preflight device loss,
every participant is `NotRun { DeviceLost { generation } }`. All participant
publications are `DiscardedByTick`, all notifications are `NotApplicable`, and
no planner, adapter, or `on_tick_report` hook is called. `on_tick_report` is
post-decision only for ticks that reached planning: its returned
error or panic changes only `notification` after the terminal decision and,
when publication completed, the disposition to
`PublishedWithNotificationFailure`; it never applies `AbortTick`
retroactively or replaces an existing no-publication cause.

`BehaviorParticipantPublication` is not a tick-wide alias. On a published
terminal path, `NoSelectedEffect` means no proposal from that participant
survived into preparation. If at least one did, the participant receives
`Published { revision_changed }`; the boolean is true exactly when one of that
participant's selected volumes appears in `BehaviorTickCompleted::published`.
It is false when all of that participant's selected volume transactions fail
preparation, even if an independent participant publishes and makes the
tick-wide `BehaviorTickCompleted::revision_changed` true. On
`NoPublication`, every participant receives `DiscardedByTick` with the same
cause.

`BehaviorEngineDescriptor` is copied into exact bounded registration storage.
The builder rejects duplicate keys, unresolved ordering keys, cycles, an
execution/trait mismatch, access outside configured maxima, or aggregate
proposal/transaction capacity above configured pools.
`BehaviorConsumerInputPolicy::None` requires a zero maximum;
`Optional | Required` requires `1..=1 MiB`. The builder checked-sums one record
and the declared maximum bytes for every input-capable participant against the
independent ingress pools. `Required` means the request must contain exactly
one record even when that record's byte slice is empty; `Optional` may omit
the record. Moria never assigns meaning to any byte.
Every `handoffs_to` key must also be a direct `runs_after` predecessor edge of
the named successor, each capacity is `1..=1 MiB`, duplicate edge handoffs are
rejected, and the worst-case three-copy byte charge plus required mixed-edge
map slots must fit the configured handoff pools before startup.
Topological ties use stable key byte order.
No named behavior phase exists.
CPU descriptors require all GPU maxima to be zero and declare collision calls/
contacts within the configured sink. GPU descriptors set collision maxima to
zero. Factory allocation must remain within `maximum_owned_gpu_bytes`, and
every counted dispatch/copy plus total workgroups must remain within both
descriptor and configured tick maxima.

Every adapter has a main-world access planner. A GPU-resident engine may use a
constant conservative planner and filter the exported view on GPU, so it does
not have to read back its GPU working set merely to plan narrower access.

```rust
pub trait BehaviorAccessPlanner: Send + 'static {
    fn plan_tick(
        &mut self,
        context: &BehaviorPlanContext<'_>,
        access: &mut BehaviorAccessSink<'_>,
    ) -> Result<(), BehaviorAdapterError>;
}
```

CPU adapters use only core public value types:

```rust
pub trait CpuBehaviorEngine: Send + 'static {
    fn ready(&self) -> bool;

    fn run_tick(
        &mut self,
        context: &BehaviorCpuTickContext<'_>,
        view: &CpuBehaviorView<'_>,
        collision: &mut BehaviorCollisionSink<'_>,
        handoffs: &mut BehaviorCpuHandoffs<'_>,
        effects: &mut BehaviorEffectSink<'_>,
    ) -> Result<(), BehaviorAdapterError>;

    fn on_tick_report(
        &mut self,
        report: &BehaviorParticipantReport<'_>,
    ) -> Result<(), BehaviorAdapterError>;
    fn on_shutdown(&mut self);
}

pub struct CpuBehaviorView<'a> { /* borrowed exact stable-view records */ }

pub struct BehaviorCpuTickContext<'a> {
    pub tick: BehaviorTickId,
    pub correlation: Correlation,
    pub snapshot: &'a [BehaviorVolumeRecordV1],
    pub consumer_input_present: bool,
    pub consumer_input: &'a [u8],
}

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ScheduledU64LeV1 {
    pub low: u32,
    pub high: u32,
}

impl ScheduledU64LeV1 {
    pub const ZERO: Self = Self { low: 0, high: 0 };
    pub const fn pack(value: u64) -> Self;
    pub const fn unpack(self) -> u64;
    pub const fn is_zero(self) -> bool;
}

#[repr(C)]
pub struct BehaviorVolumeRecordV1 {
    pub volume: ScheduledU64LeV1,
    pub revision: ScheduledU64LeV1,
    pub key: [u8; 16],
    pub translation: [f32; 4],
    pub rotation_xyzw: [f32; 4],
    pub cell_size: f32,
    pub flags: u32,                  // v1 zero
    pub local_domain_min: [i32; 4],  // xyz + zero
    pub local_domain_max: [i32; 4],  // exclusive xyz + zero
    pub reserved: [u32; 2],          // v1 zero
}

impl BehaviorVolumeRecordV1 {
    pub const fn volume_id(&self) -> u64;
    pub const fn volume_revision(&self) -> u64;
}

#[repr(C)]
pub struct BehaviorCellRecordV1 {
    pub volume_index: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub sample: u32,
    pub occupied: u32,
}

#[repr(C)]
pub struct BehaviorInputHeaderV1 {
    pub magic: u32,                       // MORI
    pub version: u32,                     // 2
    pub engine: u32,
    pub presence: u32,                    // Absent=0, Present=1
    pub payload_bytes: u32,
    pub total_bytes: u32,
    pub tick: ScheduledU64LeV1,
    pub generation: ScheduledU64LeV1,
    pub reserved: [u32; 6],               // v1 zero
}

pub enum BehaviorCollisionRequest {
    Trace {
        segment: WorldSegment,
        volumes: BoundedVolumeFilter,
        max_contacts: u32,
    },
    Overlap {
        shape: CollisionShape,
        volumes: BoundedVolumeFilter,
        max_contacts: u32,
    },
    Sweep {
        shape_at_start: CollisionShape,
        displacement: WorldVector,
        volumes: BoundedVolumeFilter,
        max_contacts: u32,
    },
}

pub struct BehaviorCollisionSink<'a> {
    /* Moria-owned exact-capacity reusable 80-byte contact slots */
}

pub struct BehaviorCollisionFacts<'a> {
    pub contacts: &'a [ContactFact],
    pub inspected_bricks: u32,
    pub inspected_cells: u32,
}

pub enum BehaviorViewError {
    OutsideAuthorizedView,
    TraversalLimit { requested: u64, limit: u64 },
    InvalidRequest,
    CallLimit { requested: u32, limit: u32 },
    ResultOverflow { required: u32, capacity: u32 },
    Poisoned,
}

impl CpuBehaviorView<'_> {
    pub fn volumes(&self) -> impl ExactSizeIterator<Item = &BehaviorVolumeRecordV1>;
    pub fn cells(&self) -> impl ExactSizeIterator<Item = &BehaviorCellRecordV1>;
    pub fn sample(
        &self,
        volume: VolumeId,
        cell: CellCoord,
    ) -> Result<MaterialSample, BehaviorViewError>;
    pub fn collision<'result>(
        &self,
        request: BehaviorCollisionRequest,
        sink: &'result mut BehaviorCollisionSink<'_>,
    ) -> Result<BehaviorCollisionFacts<'result>, BehaviorViewError>;
}

pub struct BehaviorCpuHandoffs<'a> { /* exact registered edge slots */ }

impl BehaviorCpuHandoffs<'_> {
    pub fn incoming(&self, predecessor: BehaviorEngineId) -> Option<&[u8]>;
    pub fn outgoing(
        &mut self,
        successor: BehaviorEngineId,
    ) -> Result<BehaviorHandoffWriter<'_>, BehaviorAdapterError>;
}

pub struct BehaviorHandoffWriter<'a> { /* zeroed exact-capacity slot */ }

impl BehaviorHandoffWriter<'_> {
    pub fn capacity(&self) -> usize;
    pub fn write(&mut self, bytes: &[u8]) -> Result<(), BehaviorAdapterError>;
}

pub struct BehaviorEffectSink<'a> { /* Moria-owned exact-capacity sink */ }

pub enum BehaviorEffectError {
    Full { records: u32, bytes: u64 },
    OutsideAuthorizedView,
    InvalidCommand,
    Poisoned,
}

impl BehaviorEffectSink<'_> {
    pub fn fill(
        &mut self,
        volume: VolumeId,
        target: CellAabb,
        sample: MaterialSample,
        correlation: Correlation,
    ) -> Result<u32, BehaviorEffectError>;
    pub fn patch_dense(
        &mut self,
        volume: VolumeId,
        bounds: CellAabb,
        samples: &[MaterialSample],
        correlation: Correlation,
    ) -> Result<u32, BehaviorEffectError>;
    pub fn patch_runs(
        &mut self,
        volume: VolumeId,
        bounds: CellAabb,
        runs: &[MaterialRun],
        correlation: Correlation,
    ) -> Result<u32, BehaviorEffectError>;
    pub fn move_volume(
        &mut self,
        volume: VolumeId,
        placement: RigidPlacement,
        correlation: Correlation,
    ) -> Result<u32, BehaviorEffectError>;
    pub fn retire_volume(
        &mut self,
        volume: VolumeId,
        correlation: Correlation,
    ) -> Result<u32, BehaviorEffectError>;
}
```

Each outgoing CPU writer is zeroed and one-shot. `write` accepts
`0..=capacity` bytes, copies them, records the initialized prefix, and rejects/
poisons a second or oversized write. Calling `write(&[])` explicitly marks a
valid empty payload; returning without one write leaves `Empty` and fails the
successor transition. A successor sees only the initialized prefix.
GPU handoffs use the equivalent header `written_bytes` validation. Handoff
storage is Moria-owned transport; payload meaning and any durable copy remain
consumer-owned.

`ScheduledU64LeV1` is the retained record representation of a logical 64-bit
integer in Scheduled ABI v2. It is exactly 8 bytes, aligned to 4, with the least
significant `u32` at offset 0 and the most significant `u32` at offset 4.
`pack(v)` stores `v as u32` and `(v >> 32) as u32`; `unpack` performs the
inverse. Equality compares both words, and logical zero requires both words to
be zero. WGSL declares the two words as separate `u32` fields rather than a
nonexistent `u64` scalar. Each word is encoded little-endian in the packed
buffer. Host helpers expose logical IDs/revisions without changing the wire
record.

`BehaviorVolumeRecordV1` is exactly 112 bytes with offsets
`0, 8, 16, 32, 48, 64, 68, 72, 88, 104` in field order above.
`BehaviorCellRecordV1` is exactly 24 bytes with offsets
`0, 4, 8, 12, 16, 20`. The finite positive cell size and half-open local
domain are part of every CPU/GPU export, including a volume created after
adapter registration. Reserved/flags words are zero in v1.
`BehaviorInputHeaderV1` is exactly 64 bytes with field offsets
`0, 4, 8, 12, 16, 20, 24, 32, 40`; payload begins at byte 64 and total bytes
are four-byte aligned. Its presence, size, generation, padding, and failure
rules are normative in
[behavior-scheduling.md](behavior-scheduling.md#scheduled-gpu-abi-v1).

`CpuBehaviorView` is valid only for the callback and contains every cell in the
accepted scopes, including empty cells.
Its collision helper debits the descriptor traversal authorization and
`maximum_collision_calls`, uses material occupancy rather than a mesh, clears
and reuses the Moria-owned exact sink on every call, and returns a borrow that
ends before the next call. Overflow returns no partial contacts and poisons an
ignored-error sink. The aggregate call counter and one maximum 80-byte contact
slot array are pre-reserved; no `Vec`, capacity, or other result ownership
crosses the callback.
Moria invokes the callback directly when the scheduler-owned view is ready;
the adapter does not submit or poll a query receipt.
The sink copies or borrows only these fixed/bounded values into its already
reserved storage; no capacity-bearing proposal collection crosses the callback
return. It binds matter/move/retire effects to the exact addressed snapshot
revision and rejects a target outside the authorized view.
Scheduled ABI v2 still excludes arbitrary `VolumeCommand::Create` because a
consumer-owned Rust content-source object is a control-plane registration.
Its one closed creation operation is `ExtractComponents`: a GPU adapter labels
samples already owned by one pinned source and maps them to pre-reserved
dynamic child identities. It cannot introduce content or transport a
`BaseContentSource`. Child frames, atomic directory publication, conservation,
failure, and persistence are normative in
[adapter-substrate-contracts.md](adapter-substrate-contracts.md).

GPU adapters are isolated under the Bevy integration module and deliberately
use the renderer-compatible API. This trait supports an independently
implemented adapter written for or substantially adapted to Moria's restricted
factory, fixed group-0 ABI, and counted encoder. It does not accept an
arbitrary existing engine's raw device/resources or engine-owned command and
submission model:

```rust
pub trait GpuBehaviorEngine: Send + 'static {
    fn ready(&self, generation: DeviceGeneration) -> bool;

    fn create_device_state(
        &mut self,
        context: &mut moria::bevy::behavior::BehaviorGpuDeviceContext<'_>,
    ) -> Result<(), BehaviorAdapterError>;

    fn encode_tick(
        &mut self,
        context: &mut moria::bevy::behavior::BehaviorGpuTickContext<'_>,
    ) -> Result<(), BehaviorAdapterError>;

    fn on_device_lost(&mut self, generation: DeviceGeneration);
    fn on_shutdown(&mut self);
}

#[repr(C)]
pub struct BehaviorResourceReport {
    pub generation: DeviceGeneration,
    pub owned_gpu_bytes: u64,
    pub buffers: u32,
    pub pipelines: u32,
    pub bind_groups: u32,
}

pub struct BehaviorGpuBuffer { /* opaque, generation-bound registered handle */ }
pub struct BehaviorGpuPipeline { /* opaque, generation-bound registered handle */ }
pub struct BehaviorGpuBindGroup { /* opaque, generation-bound registered handle */ }
pub struct BehaviorGpuBindGroupLayout { /* opaque registered handle */ }

pub struct BehaviorGpuResourceFactory<'a> {
    /* restricted renderer-device facade; no raw device/queue/encoder access */
}

pub struct BehaviorGpuBufferDescriptor {
    pub bytes: u64,
    pub usage: BehaviorGpuBufferUsage,
    pub debug_name: BehaviorDiagnostic,
}

pub enum BehaviorGpuBufferUsage {
    StorageRead,
    StorageReadWrite,
    Uniform,
    CopyStaging,
}

pub struct BehaviorGpuBindingDescriptor {
    pub binding: u32,
    pub access: BehaviorGpuBindingAccess,
    pub minimum_bytes: u64,
}

pub enum BehaviorGpuBindingAccess {
    UniformRead,
    StorageRead,
    StorageReadWrite,
}

pub struct BehaviorGpuBinding<'a> {
    pub binding: u32,
    pub buffer: &'a BehaviorGpuBuffer,
    pub offset: u64,
    pub bytes: u64,
}

pub struct BehaviorGpuPipelineDescriptor<'a> {
    pub wgsl: &'a str,
    pub entry_point: &'a str,
    pub groups_1_plus: &'a [BehaviorGpuBindGroupLayout],
}

impl BehaviorGpuResourceFactory<'_> {
    pub fn create_buffer(
        &mut self,
        descriptor: &BehaviorGpuBufferDescriptor,
    ) -> Result<BehaviorGpuBuffer, BehaviorAdapterError>;
    pub fn initialize_buffer(
        &mut self,
        destination: &BehaviorGpuBuffer,
        offset: u64,
        bytes: &[u8],
    ) -> Result<(), BehaviorAdapterError>;
    pub fn create_bind_group_layout(
        &mut self,
        entries: &[BehaviorGpuBindingDescriptor],
    ) -> Result<BehaviorGpuBindGroupLayout, BehaviorAdapterError>;
    pub fn create_bind_group(
        &mut self,
        layout: &BehaviorGpuBindGroupLayout,
        entries: &[BehaviorGpuBinding<'_>],
    ) -> Result<BehaviorGpuBindGroup, BehaviorAdapterError>;
    pub fn create_compute_pipeline(
        &mut self,
        descriptor: &BehaviorGpuPipelineDescriptor<'_>,
    ) -> Result<BehaviorGpuPipeline, BehaviorAdapterError>;
    pub fn usage(&self) -> BehaviorResourceReport;
}

pub struct BehaviorGpuDeviceContext<'a> {
    generation: DeviceGeneration,
    resources: BehaviorGpuResourceFactory<'a>,
}

impl<'device> BehaviorGpuDeviceContext<'device> {
    pub fn generation(&self) -> DeviceGeneration;
    pub fn resources(&mut self) -> &mut BehaviorGpuResourceFactory<'device>;
}

pub struct BehaviorGpuTickContext<'a> {
    /* opaque group-0 views and coordinator-owned encoder */
}

impl BehaviorGpuTickContext<'_> {
    pub fn tick(&self) -> BehaviorTickId;
    pub fn generation(&self) -> DeviceGeneration;
    pub fn view_counts(&self) -> (u32, u32);
    pub fn prior_feedback(&self) -> BehaviorPriorFeedback;
    pub fn consumer_input(&self) -> BehaviorGpuConsumerInput;
    pub fn component_reservations(&self) -> BehaviorGpuComponentReservations;
    pub fn cpu_egress(&self) -> BehaviorGpuCpuEgress;
    pub fn incoming_handoff_count(&self) -> u32;
    pub fn outgoing_handoff_count(&self) -> u32;
    pub fn encoder(&mut self) -> &mut BehaviorGpuEncoder<'_>;
}

pub struct BehaviorGpuConsumerInput {
    pub present: bool,
    pub bytes: u32,
}

pub struct BehaviorGpuComponentReservations {
    pub records: u32,
}

pub struct BehaviorGpuCpuEgress {
    pub enabled: bool,
    pub record_stride: u32,
    pub record_capacity: u32,
}

pub enum BehaviorPriorFeedback {
    NoneYet,
    Ready { tick: BehaviorTickId, proposals: u32 },
    UnavailablePreviousGeneration,
}

pub struct BehaviorGpuEncoder<'a> {
    /* Moria-owned counted encoder; no raw CommandEncoder access */
}

impl BehaviorGpuEncoder<'_> {
    pub fn dispatch(
        &mut self,
        pipeline: &BehaviorGpuPipeline,
        bind_groups_1_plus: &[&BehaviorGpuBindGroup],
        workgroups: [u32; 3],
    ) -> Result<(), BehaviorAdapterError>;
    pub fn copy_owned_buffer(
        &mut self,
        source: &BehaviorGpuBuffer,
        source_offset: u64,
        destination: &BehaviorGpuBuffer,
        destination_offset: u64,
        bytes: u64,
    ) -> Result<(), BehaviorAdapterError>;
}

pub struct BehaviorParticipantReport<'a> {
    pub tick: BehaviorTickId,
    pub engine: BehaviorEngineId,
    pub snapshot: &'a [VolumeSnapshotRef],
    pub execution: &'a BehaviorParticipantExecution,
    pub publication: &'a BehaviorParticipantPublication,
    pub proposals: &'a [BehaviorProposalReport],
}

pub struct BehaviorProposalReport {
    pub proposal: u32,
    pub command: Option<CommandId>,
    pub revision: Option<VolumeRevision>,
    pub rejection: Option<BehaviorProposalRejection>,
    pub failure: Option<BehaviorEngineFailure>,
}
```

`BehaviorPriorFeedback::Ready::proposals` is the count of fixed outcome
records from this adapter's preceding dispatch. Each record carries the
original zero-based proposal index. Scheduled ABI v2 does not copy the prior
`VolumeSnapshotRef` vector into feedback; a GPU adapter that needs it retains
its own proposal-index-to-snapshot correlation in factory-created,
consumer-owned state. This retained state is not checkpointed or interpreted
by Moria and becomes invalid with its device generation.

Factory usage variants map to fixed safe unions:
`StorageRead = STORAGE | COPY_SRC | COPY_DST`,
`StorageReadWrite = STORAGE | COPY_SRC | COPY_DST`,
`Uniform = UNIFORM | COPY_DST`, and
`CopyStaging = COPY_SRC | COPY_DST`. No factory-created adapter buffer is
`MAP_READ`, `MAP_WRITE`, or `INDIRECT`; cross-processor mapping belongs only to
Moria's handoff pool and indirect dispatch is not exposed in scheduled v2.
`StorageRead` includes `COPY_DST` because `initialize_buffer` uses a Moria
staging-buffer copy into the destination; the adapter still receives no
write-capable shader binding for that usage. New buffers are zero-initialized.
`initialize_buffer` copies only a borrowed
slice, validates the registered range and cumulative initialized bytes against
`maximum_owned_gpu_bytes`, chunks through the configured Moria staging pool,
and keeps the adapter not-ready until the renderer submission completes. It
does not expose a queue or retain consumer ownership.

`BehaviorGpuDeviceContext` contains only a
`BehaviorGpuResourceFactory`, the current generation, and fixed scheduled
group-0 layout metadata. It contains no `RenderDevice`, `wgpu::Device`,
`RenderQueue`, raw resource handle, or encoder constructor. Borrowed WGSL and
entry points are charged to `maximum_gpu_wgsl_bytes` and the aggregate
`behavior_gpu_wgsl_bytes` before parsing; creation
validates group 0 exactly and only permits declared groups 1 and above.
Opaque handles are usable only with the factory generation that created them.
Before calling the renderer for `create_buffer`, the factory atomically
reserves the descriptor's requested bytes from both the adapter's
`maximum_owned_gpu_bytes` registry and the world-wide effective
`behavior_gpu_buffer_bytes` pool, then reserves one buffer handle. Failure of
either reservation returns `WorldGpuBufferCapacity` (or the adapter-local
`Capacity`) and records a rejection without invoking a backend allocation.
If the renderer nevertheless reports allocation OOM, the factory releases all
three reservations, registers no handle, and returns
`RendererOutOfMemory`; an uncaptured renderer OOM is never the admission
mechanism.

Dropping a buffer handle stops new uses but does not release its byte charge
while a registered bind group still refers to it or an encoded/submitted use
has not completed. The registry releases the handle and aggregate byte permit
only after every opaque dependency is dropped and the last-use submission is
complete. On device loss the terminal generation is quarantined, its backend
handles and dependencies are destroyed, and its byte charges reach zero
before `create_device_state` may reserve replacement-generation bytes. Thus
recovery cannot temporarily hide two generations outside the pool.
Every create/drop/dependency/last-use transition updates current, high-water,
limit, and rejection telemetry for `BehaviorGpuBufferBytes` as well as
adapter-local `BehaviorResourceReport`; the latter remains computed telemetry,
not authority. Backend-private pipeline memory is not observable and is
bounded by the pipeline count, not invented byte accounting.

`BehaviorGpuTickContext` contains a counted `BehaviorGpuEncoder`, this
participant's read-only filtered `BehaviorGpuViewV1`, write-only fixed
`BehaviorGpuEffectTargetV1`, read-only incoming/write-only outgoing handoff
tables, read-only prior feedback, the current participant's read-only opaque
consumer-input binding, read-only component-extraction identity reservations, optional
write-only opaque CPU egress, record/input counts, tick ID, and device
generation. It contains no raw command encoder, aggregate cross-participant
view, authoritative Moria buffer, or submission method. Dispatch calls
automatically bind Moria group 0, accept only factory-created groups 1 and
above, validate dimensions, and debit descriptor/configured dispatch and
workgroup budgets. Buffer copies accept only registered opaque handles; each
copy debits one operation slot and byte budget. The wrapper rejects a
generation mismatch or Moria authority resource.

CPU and GPU proposal records have identical logical meanings.
Scheduled GPU ABI v2 supports fill, run patch, move, retire, compact placement
stream, and extract-components against supplied snapshot records.
Moria GPU kernels validate the entire participant batch, resolve declared
whole-proposal conflicts, prepare copy-on-write transactions, and publish
without material or proposal readback.
Outcome metadata is made CPU-visible later for typed receipts.

`BehaviorParticipantReport` borrows coordinator-owned snapshot/proposal arrays;
no capacity-bearing report collection crosses the callback. The CPU report
therefore includes the participant's snapshot revision vector. GPU feedback
does not include that vector: it contains tick, participant status,
proposal-indexed selection/rejection, command ID, published revision, and
failure category, and the adapter correlates each index with consumer-owned
state retained from its prior dispatch. The GPU terminal decision is not
reduced: its documented 64-byte participant record losslessly maps the
terminal tick disposition, every `BehaviorTickAbortCause`, participant
publication/notification, tick-wide and participant-specific
`revision_changed`, exact failed-hook count, and defined flags. Scheduled
logical 64-bit fields use `ScheduledU64LeV1` low/high words throughout.
Variable Rust failure detail remains CPU-report-only; GPU feedback exposes its
closed category and does not promise snapshot vectors, unavailable-region
vectors, or diagnostic text.
Moria does not roll back adapter-owned state when a proposal is rejected.
Adapters reconcile from this report/feedback according to their own policy.

`BehaviorTickRequest::inputs` is sorted only after structural validation and
is otherwise opaque. Submission rejects and returns the complete request
unchanged for an unknown/stale engine, duplicate engine record, input supplied
to `None`, missing `Required` record, a participant maximum violation, or an
aggregate record/byte violation. No planner, adapter callback, input upload, or
tick ID is reached on those paths. Accepted exact boxed slices are charged to
the permit and remain immutable. The same borrowed bytes are supplied to that
participant's planner and CPU tick context. For GPU participants Moria records
an ordered staging copy into the dedicated ingress allocation before
`RunningAdapters` and exposes it only through scheduled group-0 binding 5.
All GPU ingress uploads for the tick complete and are confirmed before any
planner or adapter runs. An upload failure produces the tick-global
`NoPublication(PreparationFailure)` outcome and records
`BehaviorEngineFailure::ConsumerInputUpload` for the addressed participant;
every other participant is
`NotRun { InputPreflightAborted { failed_engine } }`. Device loss yields the
existing typed no-publication device-loss outcome and every participant is
`NotRun { DeviceLost { generation } }`. `SkipParticipant` does not weaken
input preflight, and in both cases no consumer callback executes. Cancellation
that wins before the transition to input preflight drops the admitted input
and releases every host/device/staging charge before returning
`CancelledBeforePreparation`; after input preflight starts it is too late, and
submitted GPU ranges remain charged through completion or generation
quarantine.

Current GPU feedback is finalized only after the terminal publication decision
and CPU report hooks, then retained in one of two Moria-owned per-participant
slots. The next `encode_tick` receives
the completed slot as binding 4 until that submission completes. The first
tick receives `NoneYet`; device recovery receives
`UnavailablePreviousGeneration`. Neither state is an empty successful report.
The ABI, layout, retention, and handoff rules are normative in
[behavior-scheduling.md](behavior-scheduling.md).
The Scheduled ABI v2 binding-6/binding-7 layouts, component-extraction public
results, placement stream, multi-volume directory generation, opaque egress receipt ordering,
resource relationships, and failure outcomes are normative in
[adapter-substrate-contracts.md](adapter-substrate-contracts.md).

## Asynchronous WGSL inspection/effect jobs

This optional facility is an asynchronous inspection/tool consumer.
It is retained for bounded custom WGSL, observation delta processing, and
consumers that do not participate in the substrate tick.
It is not the general scheduled CPU/GPU behavior-engine hook above.

The optional `gpu-extension` feature is deliberately descriptor based:

```rust
pub struct GpuExtensionDescriptor {
    pub key: ExtensionKey,
    pub wgsl: String,                    // 1..=1 MiB UTF-8 bytes
    pub entry_point: String,             // 1..=128 UTF-8 bytes
    pub max_invocations: u32,            // 1..=1,048,576
    pub max_inspection_records: u32,     // 1..=262,144
    pub max_candidate_effects: u32,      // 0..=256
    pub max_effect_payload_bytes: u32,   // 0..=command_payload_bytes
    pub state_bytes: u32,                // 0..=min(4 MiB, extension_state_bytes / 2)
}

pub enum GpuInspectionQuery {
    Samples {
        scope: QueryScope,
        maximum_records: u32,
        minimum: MinimumRevision,
        readiness: ReadinessPolicy,
    },
    Occupancy {
        scope: QueryScope,
        maximum_records: u32,
        traversal: TraversalAuthorization,
        minimum: MinimumRevision,
        readiness: ReadinessPolicy,
    },
    Lifecycle {
        scope: QueryScope,
        maximum_records: u32,
    },
    ObservationDeltas {
        subscriber: SubscriberId,
        after: Option<ObservationSequence>,
        maximum_records: u32,
    },
}

pub enum GpuStateInput {
    Zeroed,
    Inline(Vec<u8>),
    Previous(GpuStateOutput),
}

pub struct GpuExtensionRequest {
    pub extension: ExtensionId,
    pub query: GpuInspectionQuery,
    pub opaque_state: GpuStateInput,
    pub effect_batch: EffectBatchPermit,
}

pub struct GpuExtensionDispatched {
    pub extension: ExtensionId,
    pub snapshot: Vec<VolumeSnapshotRef>,
    pub inspection: GpuInspectionOutcome,
    pub diagnostics: ExtensionDiagnostics,
    pub state: Option<GpuStateOutput>,
    pub effects: Vec<AdmittedEffect>,
}

pub enum GpuInspectionOutcome {
    Snapshot,
    ObservationDeltas(GpuObservationDeltaOutcome),
}

pub struct GpuObservationDeltaOutcome {
    pub status: GpuObservationDeltaStatus,
    pub frontier: ObservationFrontier,
    pub cursor: Option<ObservationSequence>,
    pub records: u32,
}

pub enum GpuObservationDeltaStatus {
    Complete,
    MoreAvailable,
    NeedsSnapshot { requested_after: Option<ObservationSequence> },
    UnsupportedFact {
        sequence: ObservationSequence,
        kind: ObservationFactKind,
    },
}

pub enum ObservationFactKind {
    MatterCommitted,
    VolumeCreated,
    VolumeMoved,
    VolumeRetired,
    RegionLifecycle,
    Presentation,
    Checkpoint,
    ResourcePressure,
    Device,
}

pub struct ExtensionDiagnostics {
    pub flags: u32,
    pub counters: [u32; 8],
    pub inspection_records: u32,
    pub produced_effects: u32,
    pub effect_payload_bytes: u32,
}

pub struct GpuStateOutput {
    /* Clone + Send + Sync lease */
    pub id: GpuStateId,
    pub extension: ExtensionId,
    pub device_generation: DeviceGeneration,
    pub bytes: u32,
}

pub enum AdmittedEffect {
    Matter {
        command: CommandId,
        receipt: Receipt<MatterApplied>,
    },
    Volume {
        command: CommandId,
        receipt: Receipt<VolumeApplied>,
    },
}
```

Registration is world-lifetime and consumes one `extension_registrations`
record plus `wgsl.len() + entry_point.len()` from
`extension_registry_bytes`; there is no unbounded pipeline or descriptor
cache. Reaching either limit returns the corresponding
`ExtensionRegistrationError` without compiling a pipeline. Registration
validates WGSL and the fixed ABI. A request captures exactly one closed
`GpuInspectionQuery` into an extension-owned packet. `maximum_records` must be
nonzero and fit the packet permit, descriptor maximum, and fixed public query
limit; unsupported partial coverage is rejected rather than clipped.

`ObservationDeltas` is an independent, nonadvancing read of the retained
observation ring using only the named subscriber's accepted membership, kinds,
spatial predicate, and immutable `FilterEnvelopeV1` records. It never reads,
advances, gaps, or resumes the subscriber's CPU cursor. `after` is legal only
when it is at or after `accepted().initial_after` (with `None` ordered before
every sequence) and no later than the atomically captured frontier head.
For `ObservationFrontier::Empty`, only `after = None` is legal. Otherwise the
extension receipt fails `OperationErrorKind::Validation` before shader
dispatch. A stale subscriber is likewise a validation failure.

Capture atomically records the closed `ObservationFrontier`. For
`Retained { oldest, head }`, it scans in sequence order only through `head`.
Nonmatching facts advance the scan cursor using their retained envelopes but
emit no record. Matching supported facts emit in order. If `maximum_records`
fills before the scan reaches the head, status is `MoreAvailable` and `cursor`
is the greatest sequence examined without skipping the next matching fact; the
caller pages with that exact cursor. Reaching the head is `Complete` and
`cursor = Some(head)`.

`ObservationFrontier::Empty` with `after = None` is a distinct successful
capture: status is `Complete`, `records = 0`, and `cursor = None`. It is not a
gap. After sequence one is appended, another read from `after = None` examines
that first fact. For a nonempty frontier, `after = None` means "before sequence
one"; if `oldest` is greater than sequence one, its successor was overwritten
and the status is `NeedsSnapshot`.

If the successor of `after` has been overwritten, status is `NeedsSnapshot`,
the packet has zero observation records, and `cursor = after`. If a matching
fact has no complete ABI v1 representation, status is `UnsupportedFact`, the
packet has zero records, and `cursor` plus `sequence` identify that first
unsupported fact; no later fact is scanned or silently skipped. In either
status the shader is dispatched so it can observe the status, but candidate
count and payload must remain zero or the whole extension fails GPU validation.
The CPU owner reconciles through an ordinary bounded
`SnapshotScope::SubscriptionState`, then restarts this independent view after
the snapshot frontier head, or from `None` when that frontier is `Empty`. It
does not call `resume_after` unless the ordinary CPU subscriber independently
entered `NeedsSnapshot`.

When `descriptor.state_bytes > 0`, `GpuStateInput::Inline` must contain exactly
that many bytes; `Zeroed` creates that many zero bytes, and `Previous` must name the same
extension, byte count, world, and current device generation. Shader-written
state remains in an extension-owned GPU buffer and returns as an opaque
`Some(GpuStateOutput)`; it is accepted by a later request but grants no buffer
access. A state ID becomes stale on device loss or world shutdown. State is
external-behavior working data, is not checkpointed, and never affects Moria
truth unless a candidate effect is admitted. For `state_bytes == 0`, only
`Zeroed` is valid and the output state is `None`.

`Previous` owns a cloned immutable state lease until submission; each dispatch
writes a distinct next-state allocation, so concurrent requests may safely
branch from one prior state. Dropping the last `GpuStateOutput` clone makes its
bytes reclaimable after the last GPU reader completes. Live prior/next states
and in-flight state ranges together may not exceed `extension_state_bytes`;
pressure rejects/waits through the extension permit policy and never allocates
an unbounded state history. For a descriptor with zero candidate capacity,
`try_reserve_effect_batch(0, 0)` returns a zero-capacity permit without
consuming command records; any nonzero candidate output is invalid.

### Extension ABI v1

All ABI words are 32-bit little-endian values, every offset is from extension
job-allocation byte zero and divisible by four, and all reserved words must be
zero on input and output. Rust mirrors are
`#[repr(C)] + Pod + Zeroable`; layout tests assert each offset and total size
against WGSL constants. An extension declares only Moria bind group 0:
read/write control header, read-only packet records/input state, read/write
next-state, write-only candidate records, and write-only effect payload. These
are nonoverlapping bounded ranges in an extension-owned job allocation; no
range aliases Moria storage. Any other group/binding is rejected by Naga
validation.

The 128-byte packet header is:

| Byte | Field |
| ---: | --- |
| 0 | magic `0x4d4f5249` |
| 4 | ABI version `1` |
| 8 | inspection kind: samples `1`, occupancy `2`, lifecycle `3`, observation delta `4` |
| 12 | inspection status: non-delta `0`, delta complete `1`, more available `2`, needs snapshot `3`, unsupported fact `4` |
| 16, 20 | snapshot count, inspection record count |
| 24, 28 | state byte count, candidate capacity |
| 32, 36 | output candidate count (initially zero), diagnostic word count (`8`) |
| 40, 44 | effect-payload capacity, output payload bytes (initially zero) |
| 48 | total packet bytes |
| 52 | unsupported observation-kind tag, otherwise zero |
| 56..63 | reserved zero |
| 64, 68 | snapshot-record offset, inspection-record offset |
| 72, 76 | input-state offset, output-state offset |
| 80, 84 | candidate-record offset, effect-payload offset |
| 88, 92 | device generation low/high words |
| 96, 100 | operation ID low/high words |
| 104, 108 | delta oldest-retained sequence; zero for an empty frontier or non-delta inspection |
| 112, 116 | delta captured-head sequence; zero for an empty frontier or non-delta inspection |
| 120, 124 | delta cursor: scanned-through, requested-after, or unsupported sequence according to status; zero represents `None` |

For delta inspection, oldest and head are either both zero
(`ObservationFrontier::Empty`) or both nonzero
(`ObservationFrontier::Retained`); a one-zero/one-nonzero header is invalid.
Thus an empty complete history is encoded as delta kind, `Complete`, zero
records, and zero in all three sequence fields. This cannot be confused with
`NeedsSnapshot`, whose nonempty captured frontier and distinct status are
mandatory.

Snapshot records are 64 bytes: runtime volume ID at 0, revision at 8,
translation `[f32; 4]` at 16, quaternion `[f32; 4]` at 32, and stable
`VolumeKey` bytes at 48. Sample and occupancy records are 32 bytes: snapshot
index at 0, signed local cell XYZ at 4/8/12, packed
`material:u16|coverage:u8|flags:u8` at 16, occupancy `0|1` at 20, and reserved
zero through 31. Lifecycle records are 32 bytes: snapshot index at 0, signed
brick XYZ at 4/8/12, lifecycle tag at 16, retryability tag at 20, and reserved
failure-kind tag at 24 (`0` for nonfailed, otherwise the closed
`RegionFailureKind` tags: content `1`, invalid content `2`, budget `3`, device
lost `4`, revision unavailable `5`, retired `6`, internal invariant `7`) with
the closed `ContentErrorKind` or `ResourceKind` tag at 28 where applicable,
zero otherwise. Observation-delta records are 128 bytes. Their common prefix is
sequence `u64` at 0, closed kind/flags words at 8/12, runtime volume ID at 16,
revision or terminal revision at 24, command ID (or zero) at 32, and the
16-byte correlation at 40. Bytes 56..127 are a zero-filled tagged payload:
matter stores the local affected AABB; create/retire store the stable volume
key; move stores translation `[f32; 4]` and quaternion `[f32; 4]`;
presentation stores brick XYZ, state/error tags, and visible/target revisions;
resource pressure stores resource tag, used/limit `u64`, and action; device
stores generation and state. The fixed complete v1 tags are therefore
`MatterCommitted`, `VolumeCreated`, `VolumeMoved`, `VolumeRetired`,
`Presentation`, `ResourcePressure`, and `Device`.
`RegionLifecycle` (bounded diagnostic) and `Checkpoint` (variable revision
vector) are explicitly unsupported v1 delta facts. They cause the
`UnsupportedFact` boundary only when they match the accepted filter/kinds;
irrelevant facts may be scanned past.

Candidate records are fixed 128-byte slots:

| Byte | Field |
| ---: | --- |
| 0 | kind: unused `0`, fill `1`, patch-runs `2`, move `3` |
| 4 | flags; v1 zero |
| 8 | runtime volume ID `u64` |
| 16 | mandatory exact expected `VolumeRevision` `u64`; zero is invalid |
| 24 | 16-byte `Correlation` |
| 40, 44, 48 | target min XYZ `i32` |
| 52, 56, 60 | target max XYZ `i32` |
| 64 | packed material sample |
| 68, 72 | effect-payload offset and byte length |
| 76 | reserved zero |
| 80 | placement translation `[f32; 4]` |
| 96 | placement quaternion `[f32; 4]` |
| 112..127 | reserved zero |

Fill uses target/sample and has zero payload. Patch-runs uses target plus a
payload slice of 20-byte records
`{ start_index:u32, length:u32, sample:u32, reserved:[u32;2] }`; runs use the
same X-fastest rules as `MaterialPatch`, and both reserved words are zero.
Move uses placement and requires target/sample/payload fields to be zero.
Every kind carries the mandatory exact revision precondition from the captured
snapshot. Create, retire, dense patches, and any unknown kind are not extension
ABI effects; consumers submit them through the ordinary CPU facade.

The shader may write only next-state bytes, eight consumer-defined diagnostic
counter words/flags,
candidate count/records, and effect payload. It cannot change the snapshot,
inspection records, capacities, IDs, or offsets. Diagnostics are copied as the
fixed `ExtensionDiagnostics`; next state remains GPU-owned.

The request's batch permit must reserve at least the descriptor's declared
`max_candidate_effects`, not merely an expected count, and enough aggregate
encoded command payload bytes for the worst-case records permitted by the
descriptor. This reservation happens before packet capture or shader dispatch.
The extension queue permit independently bounds packet/state/diagnostic work.
Registration rejects a descriptor whose candidate count exceeds
`extension_candidate_effects`, whose aggregate effect bytes exceed
`command_payload_bytes`, or whose worst record exceeds the fixed matter-command
limits. It also rejects WGSL/entry-point sizes above their per-descriptor
limits, zero/excess invocation or observation counts, state above
`extension_state_bytes`, or a descriptor that cannot fit one extraction batch.

Moria checks output count, offsets/alignment, reserved words, coordinates,
material IDs/flags, mandatory revision preconditions, record lengths, and
aggregate bytes on GPU. It then copies the 64-byte outcome/diagnostic block and
exactly the produced fixed candidate records/effect payload through the bounded
staging pool; host validation decodes that same bounded transport
representation into ordinary owned commands. Readback is therefore at most
`64 + 128 * produced_count + produced_payload_bytes`, never the inspection
packet or material samples. Any invalid record, overflow, duplicate effect
slot, or mismatch with the batch reservation fails the extension receipt and
admits zero child commands. No command ID is assigned before whole-array
validation succeeds.

For delta statuses `NeedsSnapshot` and `UnsupportedFact`, output validation
additionally requires candidate count and effect payload bytes to remain zero.
The returned `GpuInspectionOutcome` is decoded from the same fixed header and
must agree with its public query, captured bounds, record count, and status;
an empty `Complete` packet can therefore never be confused with a gap,
unsupported boundary, or capacity page.

After successful validation, Moria converts every candidate into an ordinary
`MatterCommand` or `VolumeCommand`, consumes the matching reserved record/byte
slice, assigns a normal command ID, and returns every child receipt in
`GpuExtensionDispatched.effects` in shader output order. Unused record, byte,
and completion capacity is released immediately after the produced count and
encoded sizes are validated. The outer extension receipt completes at this
all-children-admitted milestone; it does not wait for child completion. Each
child can then apply, conflict, be cancelled before preparation, or fail
independently under the normal per-volume queue. Thus validation/admission is
all-or-none while terminal effects are deliberately independent. Cross-volume
atomicity is not implied.

The packet/effect buffers are not Moria storage and contain only the explicitly
requested bounded snapshot. No extension receives page-table, brick-pool,
scar-pool, presentation, or renderer buffer handles. CPU-oriented
asynchronous consumers use the ordinary query/observation/command APIs;
scheduled CPU adapters use the direct tick view defined above.

## Telemetry

```rust
pub struct TelemetrySnapshot {
    pub world: WorldId,
    pub state: WorldState,
    pub device_generation: Option<DeviceGeneration>,
    pub effective_config: Option<EffectiveConfig>,
    pub resources: Vec<ResourceUsage>,   // exactly one per ResourceKind
    pub lifecycle_regions: [u64; 6],     // Cold..Failed enum order
    pub active_interests: u32,
    pub operation_stages: [u64; 7],      // OperationStage enum order
    pub observation_backlog: u32,
    pub observation_gaps: u64,
    pub presentation_states: [u64; 5],   // Absent..Failed enum order
    pub maximum_truth_view_revision_lag: u64,
    pub dirty_scar_bricks: u32,
    pub checkpoint_frontier_lag: u64,
    pub behavior_ticks: u64,
    pub behavior_processor_transitions: u64,
    pub behavior_view_bytes: u64,
    pub behavior_proposal_bytes: u64,
    pub behavior_feedback_bytes: u64,
    pub behavior_placement_updates: u64,
    pub behavior_component_extraction_children: u64,
    pub behavior_component_extraction_bytes: u64,
    pub behavior_egress_bytes: u64,
    pub behavior_egress_overflows: u64,
    pub behavior_egress_failures: u64,
    pub extension_packet_bytes: u64,
    pub extension_effect_readback_bytes: u64,
}

pub struct ResourceUsage {
    pub resource: ResourceKind,
    pub requested: u64,
    pub effective: u64,
    pub used: u64,
    pub high_water: u64,
    pub waiting_or_deferred: u64,
    pub rejected: u64,
    pub coalesced: u64,
}
```

Additional latency counters use fixed buckets versioned in the evidence schema.
`ResourceUsage` is the normative capacity/gauge accounting shape, and its
vector must contain every `ResourceKind` in enum order even when the value is
zero or its capability is disabled. Pool kinds report aggregate use.
Per-operation/per-record kinds (`MaterialMetadataPerRegistration`,
`BricksPerInterest`, `VersionsPerBrick`, and
`ContentBricksPerRequest`) report the largest current value in `used` and the
largest observed value in `high_water`; they are never added as though they
were aggregate pools.

`WorldHandle::telemetry()` returns an immutable aggregate snapshot containing:

- world/device state and adapter capability context;
- lifecycle region counts and active interest by priority/capability;
- configured versus used detail/scar/page/mesh/staging capacity;
- live versus lifetime volume records; extraction records/bytes; presentation
  artifact, dirty-record, dressing-style, and instance capacity;
- queue records/bytes, high-water marks, rejection, and latency histograms;
- command/query stages and terminal outcomes;
- observation ring use/gaps;
- presentation state and truth-to-view revision lag;
- checkpoint frontier/progress/dirty coverage;
- scheduled behavior tick frontier/stage/latency, adapter order and outcomes,
  CPU/GPU handoff maps/bytes, isolated view/collision/proposal/feedback bytes,
  conflicts, skipped/failed participants, and factory-enforced external GPU
  bytes/counts;
- extension registration/registry bytes, packet/state/effect bytes, stale state
  IDs, and candidate/diagnostic readback bytes;
- resource-pressure decisions.

Coordinates, raw samples, shader buffers, and consumer opaque metadata are not
telemetry. Histograms have fixed buckets defined in the evidence schema.

## Shutdown

```rust
pub enum ShutdownPolicy {
    Drain { require_checkpoint: Option<CheckpointKey> },
    CancelNotPrepared { require_checkpoint: Option<CheckpointKey> },
}

pub struct ShutdownReport {
    pub final_revisions: Vec<(VolumeKey, VolumeRevision)>,
    pub durable_revisions: Vec<(VolumeKey, VolumeRevision)>,
    pub cancelled: Vec<OperationId>,
    pub failed: Vec<(OperationId, OperationErrorKind)>,
    pub clean: bool,
}
```

Shutdown atomically closes permits/admission. `CancelNotPrepared` installs
`CancelledBeforePreparation` for every operation still in
`Queued | WaitingForMatter` using the same transition race as explicit
cancellation; `Preparing` and later operations drain. It then waits for
submitted GPU work or device terminal state, completes required
checkpointing, emits the report, then releases resources. A failed required
checkpoint yields `clean = false`; dirty data is not described as durable.
The application may still terminate, but must make that loss decision outside
Moria.
