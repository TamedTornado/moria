# Public API, schedules, CLI, and viewer contracts

## Public facade

The existing privacy rule remains: `moria-world` exports immutable values,
plugins, messages, bounded read-only system parameters, and test support. It
does not export `WorldStore`, mutable brick/voxel slices, authoritative ECS
components, task queues, or render internals. `moria-demo`, `moria-bench`, and
`moria-curate` compile as external consumers through `moria_world::{...}`.

The corrected facade adds or changes these cohesive contracts:

```rust
pub struct MoriaWorldPlugin;

pub struct WorldOpenRequest {
    pub request_id: u64,
    pub definition: WorldDefinition,
    pub presentation: PresentationMode,
}

pub enum PresentationMode {
    Headless,
    Diagnostic,
}

pub struct WorldReady {
    pub request_id: u64,
    pub identity: WorldIdentity,
}

pub struct WorldOpenFailed {
    pub request_id: u64,
    pub error: WorldOpenError,
}
```

`MoriaWorldPlugin` installs services/messages but does not implicitly open a
hard-coded world. Exactly one accepted open request transitions
`WorldLifecycle` from `Uninitialized -> Loading -> Ready|Failed`. Duplicate or
second-world requests receive a typed rejection. `WorldReady` means definition,
materials, authored anchors, and identity validated; `WorldReadState`
installed; and public reads, activation, and edits may begin. Cell/presentation
readiness is reported separately by activation and reconciliation messages;
`WorldReady` does not mean any view or the complete world/object population
was generated. A `BoundedFixture` is consumer-owned scripted input: the
proof/viewer parses it and drives these same public activation, focus, read,
and edit contracts. It is never installed as a privileged world resource.

`WorldOpenError` contains at least `InvalidDefinition`,
`UnsupportedSchema`, `UnsupportedGenerator`, `Fixture`, `InitialActivation`,
`Asset`, and `Persistence`. Errors carry stable field/path or stage detail
without embedding platform-specific absolute paths in deterministic evidence.

The public read/edit/focus types remain the primary seams. These corrections
apply:

- `WorldRead::identity`, `bounds`, voxel/point/column/ray/capsule, active-band,
  and diagnostic-page methods remain bounded and read current base plus
  committed deltas.
- `WorldRead::route` and `TraversalRoute` are removed.
- coordinate validation uses the opened `WorldBounds`, not global Product One
  constants;
- an inactive in-bounds sample may generate/evaluate its bounded owner-cell
  context but cannot expand the whole world;
- diagnostic pages expose cell/core/context/accounting and reconciliation
  status through bounded values, never store/entity handles; and
- all owned vectors are sorted, capped, and fail rather than truncate.

[Design R2-R4, R14; Analysis: public facade]

## Generation-cell requests and activation

Focus input remains message-based:

```rust
pub enum FocusPurpose {
    Camera,
    Inspection,
    Mutation,
    Proof,
}

pub struct FocusSource {
    pub id: u32,
    pub position: WorldPointQ8,
    pub purpose: FocusPurpose,
}
```

The obsolete `Traversal` purpose is removed. `SetFocusSource` replaces by ID
and `RemoveFocusSource` releases it as today. The private planner maps focus
bands to brick requests and generation-cell core/context references.

For test/proof callers that need an exact bounded set independent of a camera,
add messages:

```rust
pub struct ActivateGenerationCells {
    pub request_id: u64,
    pub cells: Vec<GenerationCellCoord>,
}

pub struct GenerationCellsReady {
    pub request_id: u64,
    pub cells: Vec<GenerationCellCoord>,
    pub generated_owner_cells: u32,
    pub retained_owner_cells: u32,
    pub result_digest: [u8; 32],
}

pub struct ReleaseGenerationCells {
    pub request_id: u64,
}
```

Input cells must be nonempty, sorted/unique, in bounds, and at most 256. The
runtime rejects invalid input before scheduling. `GenerationCellsReady.cells`
is the exact core list. `generated_owner_cells` counts cache misses including
context; it is at most 27 times core count before deduplication.
`retained_owner_cells` is observed live state, not a whole-world count.
Releasing a request decrements only references owned by that request.

Cell task tokens contain world digest, cell coordinate, generation number, and
purpose. Results with any mismatch are discarded and counted; they are never
installed.

## Edit, attribution, and reconciliation protocol

`WorldEditWrite::submit(WorldEditCommand)` remains the only synchronous
mutation entry point and preserves current admission errors. Admission reads
opened bounds/config, not duplicated region constants. Accepted reservations
are consumed in `FixedUpdate`.

Add public messages:

```rust
pub struct EditBatchCommitted {
    pub request_id: u64,
    pub batch_index: u32,
    pub revision: u64,
    pub changed_bricks: Vec<BrickCoord>,
    pub affected_objects: Vec<ObjectId>,
}

pub struct EditPrimaryPresentationReady {
    pub request_id: u64,
    pub revision: u64,
    pub ready_items: u32,
    pub expected_primary_items: u32,
}

pub struct EditReconciliationComplete {
    pub request_id: u64,
    pub final_revision: u64,
    pub committed_batches: u32,
    pub changed_bricks_digest: [u8; 32],
    pub affected_objects_digest: [u8; 32],
    pub expected_items: u32,
    pub reconciled_items: u32,
}
```

For one request:

1. exactly one `EditAccepted` or synchronous `SubmitError` occurs;
2. batch indices start at zero and increase by one;
3. non-no-op batch revisions strictly increase;
4. changed bricks and affected objects are exact sorted/unique sets described
   in `data-model.md`;
5. committed truth is query-visible before the batch message can be observed;
6. primary readiness never means terminal reconciliation; and
7. exactly one terminal completion occurs after all batches and
   revision-matched derived install/removal acknowledgements.

A no-op accepted request emits one zero-change batch or an explicitly
documented zero-batch terminal with unchanged revision; the implementation
chooses one representation and validators reject mixing both. This TDD selects
the zero-batch terminal: `committed_batches == 0`, both canonical empty-set
digests, and expected/reconciled items zero.

The reconciliation key is:

```rust
pub struct ReconciliationItemKey {
    pub request_id: u64,
    pub revision: u64,
    pub kind: DerivedItemKind,
    pub spatial_key: DerivedSpatialKey,
}
```

Every dirty terrain chunk/seam, registered object, water patch, and diagnostic
derived item adds one expected key. Installation or explicit removal/tombstone
acknowledges the same key only after the result token and current revision
match. A stale result cannot decrement the barrier. Expected and acknowledged
key sets are bounded per batch and deduplicated in stable order.

[Design R7, AC6; Authority: exact mutation and complete reconciliation]

## Save/load protocol

Use public messages, not filesystem methods on `WorldRead`:

```rust
pub struct SaveWorldRequest {
    pub request_id: u64,
    pub path: PathBuf,
}

pub struct SaveWorldCompleted {
    pub request_id: u64,
    pub path: PathBuf,
    pub saved_revision: u64,
    pub bytes: u64,
    pub changed_voxels: u32,
    pub content_sha256: [u8; 32],
}

pub struct SaveWorldFailed {
    pub request_id: u64,
    pub error: SaveError,
}

pub struct LoadWorldRequest {
    pub request_id: u64,
    pub path: PathBuf,
}

pub struct LoadWorldStarted {
    pub request_id: u64,
    pub prior_revision: u64,
}

pub struct LoadWorldCompleted {
    pub request_id: u64,
    pub revision: u64,
    pub changed_voxels: u32,
}

pub struct LoadWorldFailed {
    pub request_id: u64,
    pub error: LoadError,
}
```

Only one persistence operation runs. I/O and compression run on an async task
pool; task polling is frame-based. A save snapshots an immutable sorted delta
map at request observation and later edits do not alter the file. Load staging
does not alter live truth. The successful delta-map swap occurs in
`FixedUpdate`; reads see wholly old or wholly new truth. Edits are rejected
with `LoadInProgress` from accepted load through terminal success/failure.
`LoadWorldCompleted` waits for active truth and derived reconciliation. Failure
leaves current truth/revision unchanged.

Paths are consumer-provided but output validation rejects directories,
non-`.moria-delta.zst` extensions, missing parents, and a temp path equal to the
destination. Proof tests use temporary directories. [Design R8, AC7]

## Schedule and plugin composition

Feature-local plugins composed by `MoriaWorldPlugin`:

```text
WorldDefinitionPlugin
GenerationPlugin
WorldQueryPlugin
WorldStreamingPlugin
WorldMutationPlugin
WorldPersistencePlugin
WorldTelemetryPlugin
DiagnosticPresentationPlugin (only with PresentationMode::Diagnostic)
```

The exact schedule:

```text
PreUpdate
  collect_open_and_persistence_requests
  apply_focus_messages
  collect_diagnostic_actions (viewer consumer, not moria-world)

FixedUpdate
  EditSet::Admit
  EditSet::Stage
  EditSet::Commit
  EditSet::Invalidate
  LoadSet::Swap

Update
  StreamSet::Plan
  GenerationSet::Launch
  GenerationSet::Poll
  GenerationSet::Install
  ReconciliationSet::Launch
  ReconciliationSet::Poll
  ReconciliationSet::Install
  PersistenceSet::Poll
  TelemetrySet::Publish

PostUpdate
  advance_frame_index
  publish_bounded_diagnostics
```

Required ordering:

- `EditSet::Commit -> EditSet::Invalidate`;
- `LoadSet::Swap -> EditSet::Invalidate` for load-produced dirty work;
- `StreamSet::Plan -> GenerationSet::Launch`;
- generation poll/install is token checked before query/index exposure;
- reconciliation poll/install is token/revision checked before barrier
  acknowledgement; and
- telemetry snapshots occur after installation for that frame.

`EditSet::Commit` and `LoadSet::Swap` must not run in the same fixed tick. The
load transaction owns the tick and edit admission remains blocked. Task
launch/poll never mutates authoritative voxel truth.

Headless mode registers the same generation, query, edit, persistence, and
logical reconciliation ledger. It omits Bevy render assets/entities and
acknowledges a derived item only after its pure extraction result is validated,
not at dirty discovery. Diagnostic mode additionally waits for real
installation/removal acknowledgements.

## Diagnostic viewer contract

`moria-demo/src/main.rs` only parses CLI, constructs Bevy, and adds:

```rust
ViewerPlugin
MoriaWorldPlugin
```

`ViewerPlugin` owns cohesive camera, semantic input, overlays, and review
state. Physical inputs map to:

```rust
pub enum DiagnosticAction {
    MoveCamera,
    LookCamera,
    RaiseLowerCamera,
    IncreaseSpeed,
    DecreaseSpeed,
    ToggleVoxelMaterials,
    ToggleBrickBounds,
    ToggleStreamingCells,
    SelectDig,
    SelectPlace,
    ApplyEdit,
    Save,
    Load,
    ToggleReconciliation,
    Exit,
}
```

Frame systems read raw devices, update free-camera/overlays, and latch edit/
save/load edges into durable intents. `FixedUpdate` consumes an edit intent at
most once. No action or component is named player, locomotion, sprint, jump,
swim, character, skeleton, animation, species, forest, or route.

The viewer:

- publishes one camera focus and one optional inspection/mutation focus through
  public messages;
- samples targeting, materials, bricks, cells, and reconciliation only through
  `WorldRead`/telemetry;
- submits changes only through `WorldEditWrite`;
- visibly distinguishes current authoritative voxel/material truth from
  pending/installed derived presentation;
- allows camera movement above and below the surface without collision or a
  player body; and
- shares handles for repeated neutral object visuals.

F2 requires the bounded fixture to contain visible surface and underground
geometry plus one neutral registered object whose dependency intersects a
review edit. The fixture is a proof aid, not a traversal or scenery
requirement.

## `moria-curate` CLI

Accepted forms are exact:

```text
moria-curate check
moria-curate prove-substrate --definition <ron> --fixture <ron> --output <json>
moria-curate compare-proofs --left <json> --right <json>
```

`check` uses the repository default paths and produces no generated world
artifact. `prove-substrate` requires existing readable inputs, a new `.json`
output whose parent exists, and refuses overwrite. Argument errors write no
report and exit 2. Once an output path is accepted, a runtime/proof failure
writes a valid non-pass report atomically and exits 1; a fully validated pass
exits 0.

`compare-proofs` validates both documents, then requires matching build commit,
world/fixture/report versions and digests, required stage IDs, normalized
stage-result digests, and `correctness_digest_sha256`. Machine/timing fields
must be present and must identify distinct Linux x86_64 and macOS arm64 stacks;
they need not match. It exits 1 with stable mismatch details and never rewrites
either artifact.

Removed forms (`generate`, `prove-forest`) are argument errors. No alias keeps
them active.

## `moria-bench` CLI and scenarios

Accepted common flags:

```text
--scenario <generated-object-stress|diagnostic-streaming|mutation-reconciliation>
--definition <ron>
--output <json>
[--fixture <ron>]
[--proof <json>]
[--resolution <width>x<height>]
[--cell-count <u32>]
[--object-candidates-per-cell <u32>]
```

Scenario rules:

- `generated-object-stress` requires `--cell-count` in `1..=4096` and
  `--object-candidates-per-cell` in `1..=65535`; it rejects fixture/proof. It
  also rejects a cell count larger than the definition's bounds. It generates
  reproducible workload candidates under `target/` and does not mutate the
  world definition.
- `diagnostic-streaming` requires a fixture; proof is optional. It measures
  activation, context retention, eviction, task queues, shared-handle
  presentation, and accounting along a generated diagnostic camera path, not a
  curated route.
- `mutation-reconciliation` requires fixture and a validated matching substrate
  proof. It measures public edit progress, exact affected sets, derived
  reconciliation, save/reload, and frame/resource evidence.

The fixed seed restriction, `--forest-proof`, feasibility-mutation,
flythrough, route coverage, and forest-count contracts are removed. Resolution
is measurement identity, not functional correctness. Existing path validation,
nullable failure schema, human summary, atomic temp-write/flush/rename, and
exit-code behavior remain.

## Testable interface invariants

- An external test crate cannot name `WorldStore`, generated-cell records,
  mutable voxel state, authoritative components, or private task queues.
- Viewer, benchmark, and proof compile using only facade exports.
- World opening generates no cell outside initial core plus one context ring.
- Every public vector is sorted/unique and either within its declared bound or
  the operation fails without partial output.
- Frame-local input is absent from fixed mutation systems.
- Same initial world, ordered edit commands, and fixed tick count produce the
  same committed messages and digests.
- A stale generation/reconciliation token cannot install or acknowledge work.
- Save/load terminal messages are exactly-once and failure preserves truth.
- No report validator accepts missing, partial, contradictory, or
  digest-mismatched evidence as passed.
