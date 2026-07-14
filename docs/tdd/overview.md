# Technical Design Overview

## Purpose and requirement translation

Product One is a reusable sparse voxel-world library plus two consumers: the walkable validation demo and the benchmark runner. The library is authoritative for generation, queries, mutation, edit persistence, activation, derived terrain meshes, water, and registered world objects. Both consumers use the library's public API; neither can access or mutate voxel storage directly. This is the enforceable counterpart of the design requirement that the demo have no privileged world behavior.

Experiential requirements have the following testable meanings:

- **Walkable in under five seconds:** elapsed monotonic time from process entry to `DemoState::Playing`, with player input enabled and the collision neighborhood ready, is less than 5,000 ms on each named acceptance machine.
- **Surface update within two rendered frames:** `WorldEditWrite` stamps rendered frame `N` at the consumer's submit call. Every affected terrain, water, registered-object (including an active Horizon aggregate repartition), and dressing presentation is extracted, GPU-buffer-prepared/removed, and queued by the renderer for frame `N + 2` or earlier, including when frame `N` has zero fixed ticks or submission is after its fixed-drain cutoff. Collision and queries see committed truth in the first eligible fixed tick. `EditCommitted` and `EditSurfaceReady` record submit, commit, and renderer-ready frame indices.
- **No visible hitch for the representative carve:** while applying one 3 m-radius hillside dig on an acceptance machine, no rendered-frame interval may exceed 33.3 ms, and the two-frame surface-ready contract must also pass. This threshold catches an obvious missed 60 Hz frame while allowing the asynchronous two-frame contract to be measured independently.
- **Exact reload:** after save/load, every edited voxel's material, density, and reserved state byte equals the pre-save value; generated, unedited samples equal the same seed/config output. Derived meshes and dressing are intentionally regenerated and are not compared byte-for-byte.
- **Negligible detailed cost for idle wilderness:** untouched bricks outside an active band have no allocated 4,096-voxel array and no mesh. Their truth is represented by the seed/config plus, where needed, a compact procedural or uniform descriptor.
- **Forest dependency metadata remains sparse:** registered-object surface dependencies are fixed-size bounds plus a shared lazy analytic predicate, never retained coordinate sets; the complete immutable object index is capped at 16 MiB and only active objects probe at most 128 sparse delta bricks for authored eligibility.
- **Distant forests cannot resurrect base truth:** each 64 m Horizon object cell is rebuilt from current dependency eligibility. Intact trees contribute aggregate cards; edited trees are excluded and use revisioned owner-filtered coarse payloads (or empty tombstones), with eviction, transition, and load governed by the same token/readiness lifecycle.
- **Feasibility before breadth:** the byte-identical full curated forest must pass simultaneous content/index/startup Gate F1, then the representative and maximum-candidate 3 m carves must pass the complete production render path on the M4 in Gate F2. Failure blocks broad generation, final assets/dressing, traversal polish, persistence, and full benchmarks; targets are not tuned to turn a failure green.
- **Synchronous query cost is public API:** rays are limited to 64 m/448 cells, capsules to the player/camera envelope, sweeps to 12 m and 8,192 candidates, contact results to 512, and diagnostics to bounded pages. Exact limits, complexity, rejection behavior, and M4 timing proof are in [api.md](api.md).
- **Graphics memory below approximately 2 GB:** the portable implementation records application-requested GPU allocation estimates, but this does not prove resident driver/backend memory. The original product target remains unproven unless an acceptance harness supplies resident evidence; estimate-only substitution is the explicit Design Divergence below and has not been approved.

## Selected stack

- **Rust 2024 and Bevy 0.19:** Bevy supplies the ECS, schedules, task pools, asset system, window/input integration, PBR renderer, diagnostics, and portable wgpu backend required by the design. Rust provides explicit ownership around the authoritative store and compact fixed-width data types.
- **A Cargo workspace with four packages:** `moria-world` is the reusable library; `moria-demo` is the downloadable walkable binary; `moria-bench` is the separate validation binary; and `moria-curate` is a development-only CLI that deterministically derives and verifies the checked-in curated manifest. This is a deliberate workspace exception to the usual single-binary Bevy scaffold because the design requires separate deliverables, an external-consumer boundary, and generation tooling from the outset.
- **CPU world generation, mutation, collision, and meshing:** authoritative work uses portable CPU code and Bevy task pools. The renderer consumes ordinary Bevy meshes and shared material handles. No load-bearing Apple-only path, custom GPU mesher, 64-bit GPU atomic, or vendor extension is permitted.
- **Integer/fixed-point authoritative generation:** seeded hashes, feature evaluation, density falloff, and voxel coordinate conversion use integer or fixed-point math. Floating point is allowed only for presentation and query inputs that are quantized before authoritative evaluation. This makes base generation and edit replay stable across supported targets.
- **RON for authored/configured data; a compact custom binary delta format compressed with zstd for saves; JSON for benchmark results:** each format matches its use: reviewable configuration and stamps, exact compact runtime persistence, and machine-readable evidence.
- **Project-owned action mapping:** physical keyboard/mouse or gamepad inputs map to semantic gameplay/debug/UI actions in one input plugin before fixed simulation. This avoids device codes in movement and world logic without adding a rebinding dependency that the product does not require.

Minimum direct dependencies are `bevy = "0.19"`, `serde` with derive, `ron`, `zstd`, `sha2` (world/save/profile identities), `directories` (portable single-slot location), plus `serde_json` and `sysinfo` in the benchmark package only. `proptest` is a dev dependency for generation and behavior properties. Dependency versions are pinned in the workspace lockfile; `Cargo.lock` is committed because all shipped targets are applications. A crate may not add a dependency without documenting which TDD contract it implements.

## High-level architecture

```text
assets/config + seed + generated curated manifest + sparse ruin stamp
                              |
                              v
                    moria-world public plugins
       +----------------------+----------------------+
       | generation/store | query/edit | streaming   |
       | persistence      | meshing    | presentation|
       +----------------------+----------------------+
                              |
             public read/query methods and messages only
                       /                     \
                      v                       v
            moria-demo binary         moria-bench binary
        player/camera/debug UI       scripted path/carve storm
```

`moria-world` owns a private `WorldStore`. A consumer observes it with the read-only `WorldRead` system parameter and requests changes by submitting `WorldEditCommand` through `WorldEditWrite`, which stamps publication time. Accepted changes produce immutable result messages. Rendering is derived from the same store through library-owned plugins; a consumer can configure or observe it but cannot inject a mesh as world truth.

Generation is addressable: a `ColumnEvaluator` and `BaseVoxelEvaluator` calculate any requested column, brick classification, or voxel from the seed, curated parameters, and manifest. The whole 1 km x 1 km x 256 m volume is never expanded. A compact 32 m dependency/activation grid and 4 m sample grid keep forest queries bounded while sharing one 16 MiB retained-index cap. Active bricks are classified as procedural/uniform/detailed, and only bricks crossing a surface, containing an edit, or explicitly requested for raw inspection allocate voxel arrays. Edit deltas outlive active detail and overlay base evaluation.

Frame input is collected in `PreUpdate`. Fixed-rate movement, solid collision, water-state changes, edit commits, and activation decisions run in ordered `FixedUpdate` sets at 60 Hz. Background generation and mesh jobs operate on immutable snapshots. Their results are installed and visually interpolated in `Update`/`PostUpdate`; camera, UI, lighting, and diagnostics remain per-frame. The exact sets and dependencies are in [systems.md](systems.md).

## Package and module structure

```text
Cargo.toml
Cargo.lock
assets/
  config/
  stamps/
  vegetation/
  materials/
crates/
  moria-world/
    Cargo.toml
    src/
      lib.rs
      generation/       # seed, columns, geology, caves, water, biome, POIs
      curation/         # pure manifest derivation, behind the curation Cargo feature
      storage/          # brick representation, overlays, coordinates
      query/            # read-only consumer SystemParam and ray/sweep queries
      mutation/         # edit validation, deterministic kernels, dirty regions
      persistence/      # one-slot delta encoding and restoration
      streaming/        # activation bands, queues, task results
      terrain/          # mesh extraction and solid-collision helpers
      objects/          # registered voxel objects and derived dressing
      presentation/     # shared render assets, water, LOD, debug visualizers
      telemetry/        # frame/edit/allocation observations exposed to benchmark
      testing.rs        # fixed-tick/headless helpers, compiled for tests only
  moria-demo/
    Cargo.toml
    src/
      main.rs           # app construction only
      input/            # physical bindings -> actions -> intents
      player/           # movement, solid collision, paddle state, light
      camera/           # orbit and terrain avoidance
      debug_tools/      # edit targeting, view toggles, material selection
      flow/              # loading/playing state orchestration
      hud/               # minimal diagnostic labels and time slider
  moria-bench/
    Cargo.toml
    src/
      main.rs           # argument parsing and app construction only
      scenarios/        # flythrough and carve-storm scripts
      capture/          # metric windows, machine profile, JSON report
  moria-curate/
    Cargo.toml
    src/
      main.rs           # deterministic manifest generation/check command
      search.rs          # CLI orchestration over moria-world's curation API
```

The repository-root `assets/` layout is retained because it is Bevy's default asset path and is shared by the binaries. Feature ownership, not ECS item kind, determines modules. `main.rs` and `lib.rs` contain declarations, exports, and plugin wiring only. Private submodules can use local `components.rs` or `systems.rs` only after they are already scoped to a cohesive feature.

## Plugin composition

`MoriaWorldPlugin` is the public facade and installs, in order, `GenerationPlugin`, `StoragePlugin`, `QueryPlugin`, `MutationPlugin`, `PersistencePlugin`, `StreamingPlugin`, `TerrainPlugin`, `ObjectPlugin`, `WorldPresentationPlugin`, and `TelemetryPlugin`. Ordering at installation time does not imply system order; named schedule sets express every runtime dependency.

`moria-demo` adds `DefaultPlugins`, `MoriaWorldPlugin`, then `DemoFlowPlugin`, `ActionInputPlugin`, `PlayerPlugin`, `OrbitCameraPlugin`, `DebugToolsPlugin`, and `DemoHudPlugin`. `moria-bench` adds the same `DefaultPlugins` and `MoriaWorldPlugin` but substitutes `BenchmarkScenarioPlugin` and `MetricCapturePlugin`; it cannot import `moria-world` private modules. Logic tests add `MinimalPlugins` plus only the plugin slices under test.

## AGENTS.md specification

The implementation agent must create a repository-root `AGENTS.md` containing these exact operational rules.

### Commands

Run all commands from the repository root:

```sh
# Format check
cargo fmt --all -- --check

# Type and target check
cargo check --all-targets

# Lint; warnings fail CI
cargo clippy --all-targets -- -D warnings

# Automated unit, property, headless plugin, and integration tests
cargo test

# Build all library/binary/test targets
cargo build --all-targets

# Develop the walkable demo
cargo run -p moria-demo

# Run benchmark scenarios (headed, release build)
cargo run --release -p moria-bench -- --scenario flythrough --output target/bench/flythrough.json
cargo run --release -p moria-bench -- --scenario carve-storm --output target/bench/carve-storm.json

# Rebuild or verify the deterministic curated manifest
cargo run -p moria-curate -- generate
cargo run -p moria-curate -- check

# Blocking feasibility gates (release build on the 32 GB M4 acceptance machine)
cargo run --release -p moria-curate -- prove-forest --output target/feasibility/forest.json
cargo run --release -p moria-bench -- --scenario feasibility-carve --resolution 2560x1440 --forest-proof target/feasibility/forest.json --output target/feasibility/carve.json
```

Normal development uses the dev profile, not `--release`; release is reserved for acceptance benchmarks. CI runs the first four commands in the shown order and also runs `moria-curate check`. GPU/platform acceptance is a separate headed job on the named machines.

### Module, naming, and imports

- Use the package/module tree above. Add behavior beside the feature plugin that owns it; do not create repository-wide `systems`, `components`, `resources`, `types`, or `utils` buckets.
- Rust files and modules are `snake_case`; types, components, resources, messages, plugins, and system sets are `UpperCamelCase`; systems are verb-led `snake_case`; constants are `SCREAMING_SNAKE_CASE` and include units where ambiguity exists, such as `VOXEL_EDGE_METERS`.
- Suffix Bevy plugins with `Plugin`, public commands/results with an action/completed name (`WorldEditCommand`, `EditCommitted`), action enums with `Action`, and schedule sets with `Set`.
- Import through each package's public facade (`moria_world::{...}`) from other packages. `moria-demo` and `moria-bench` may not use path dependencies to private source modules, expose `WorldStore`, query arbitrary world entities, or construct authoritative voxel components.
- Within a feature, prefer `super` for the immediate module facade and `crate::feature` for cross-feature public APIs. Do not use glob imports except `bevy::prelude::*` inside implementation modules. Keep public exports explicit.
- A data type is either a singleton `Resource` or an entity `Component`, never both. Per-player, per-brick-render, per-object, and per-widget values are components; config, asset handle collections, queues, and the authoritative app/world singleton are resources.
- Public authoritative APIs use fixed-width integer coordinates and IDs. `usize` is limited to in-memory indexing. GPU-visible counters, allocation indices, and propagation labels are `u32`; shaders may use only 32-bit atomics.

### Project-specific constraints

- Authoritative generation/mutation must not depend on floating-point transcendental results, wall-clock time, thread order, render LOD, or hash-map iteration order. Sort coordinates before persistence and externally observed batch messages.
- Consumers mutate the world only through `WorldEditWrite`; all world inspection uses `WorldRead` (including bounded diagnostic pages) or public telemetry. Derived meshes, water meshes, dressing, and debug geometry are never serialized as truth.
- Simulation rules, collision, cooldowns, and edit commits belong in `FixedUpdate`; raw input edges, camera, UI, interpolation, lighting, task polling, and render installation belong in frame schedules. Convert input edges to durable intent before fixed ticks.
- Repeated vegetation and clutter share mesh/material handles. Do not allocate a material per instance. Add custom GPU rendering only after representative-scene diagnostics prove built-in batching, culling, visibility ranges, and LOD insufficient.
- Tests never sleep or open a window for logic. Use `MinimalPlugins`, seed only required state, and advance a controlled count of fixed ticks. Rendering correctness is verified with headed acceptance scenes and human visual review.
- Static water has no flow/pressure simulation; granular materials have no settling; reserved voxel state has no behavior; registered objects have no dynamics; excluded game systems must not acquire placeholder ECS types.
- Cargo dev profiles use `opt-level = 1` for project code and `opt-level = 3` for dependencies. Native release uses `lto = "thin"` and `codegen-units = 1`. No wasm profile is added because web is not a Product One target.
- Follow [implementation-plan.md](implementation-plan.md): issues `G1`, `V1`, `T1`, `S1`, and `B1` cannot start until digest-matched F1 and F2 artifacts pass on the M4. Any gate failure immediately requires a reviewed TDD revision describing the measured bottleneck and redesign before more implementation; do not weaken workload/content/timing thresholds or substitute a partial pipeline.

## Verification strategy

Pure tests cover coordinate conversion, deterministic column/voxel evaluation, edit kernels, delta comparison, encoding, route constraints, collision primitives, LOD selection, and metric aggregation. Property tests verify that generation is order-independent, all required route features intersect their declared constraints, no dressing survives without an eligible anchor, edits touch only the mathematical sphere plus mesh halo, and save/load is an exact round trip.

Headless `App` tests use `MinimalPlugins`, public messages, and explicit fixed-tick advancement to verify plugin boundaries, ordered state transitions, activation/eviction, edit messages, player collision, paddling, and invalid operations. Scripted integration tests run N controlled fixed ticks with semantic intent, then assert ECS/world properties. They never assume one `app.update()` equals one fixed tick.

Rendering, GPU integration, startup time, frame pacing, and allocation estimates require headed runs. Before implementation breadth, F1 validates the checked-in full forest/index and F2 validates the complete carve/query path on the M4. The later flythrough/carve-storm scenarios and manual visual checklist cover full acceptance; [benchmarks.md](benchmarks.md) defines measurements and [implementation-plan.md](implementation-plan.md) defines dependency gates.

## Design coverage and divergences

Every design entity maps to a type in [data-model.md](data-model.md), every consumer interaction maps to a contract in [api.md](api.md), and runtime ordering appears in [systems.md](systems.md) and [states.md](states.md). Visual requirements and asset ownership are covered by [rendering.md](rendering.md) and [assets.md](assets.md); all tunable values and acceptance invariants are centralized in [config.md](config.md).

| Design requirement group | Primary technical counterpart |
|---|---|
| Deterministic 1 km x 1 km x 256 m seed region and curated feature route | Fixed-point evaluators, generated/checked manifest, region/feature config, curation CLI |
| 25 cm material voxels and 16-cubed sparse bricks | Four-byte `Voxel`, private procedural/uniform/detailed brick store, on-demand columns |
| Smooth natural terrain, sharp cuts/strata/masonry, disposable surfaces | Material-aware CPU dual contouring, shared texture arrays, revisioned render tiles |
| Forest-scale registered objects and anchored grass/clutter | Analytic voxel object shapes, stable placements, shared-handle LOD visuals, revision anchors |
| Active distance bands and <2 GB / sparse wilderness intent | Four render bands, grouped LOD tiles, task budgets, eviction invariants, allocation ledger |
| Run/sprint/jump/paddle, voxel collision, orbit camera, cave light | Semantic intents, 60 Hz solid sweep/response, static-water state, per-frame camera/light |
| Dig/place and all diagnostic toggles through supported operations | Read-only `WorldRead`/diagnostic pages, `WorldEditWrite`, revision barriers, public focus/telemetry APIs |
| Two-rendered-frame 3 m carve and no hitch | Submit/commit/render-extract frame stamps, priority worker lane, explicit benchmark thresholds |
| High-risk sequencing | Blocking full-manifest F1 and production carve/query F2 before downstream feature breadth |
| Single-slot exact delta persistence under 50 MB | Base-relative sorted deltas, atomic zstd file, identity/checksum validation, round-trip scenario |
| Flythrough/carve-storm evidence with machine identity | Separate public-API benchmark consumer, metric schema, acceptance matrix, provisional Linux baseline |
| Explicit excluded systems | No registered runtime plugins/data/assets for dynamic fluids, granular settling, object dynamics, ecology, game loops, AI, multiplayer, or scripting |

### Design Divergence: resident graphics-memory evidence

- **Design requirement:** the full region must remain below approximately 2 GB resident graphics memory.
- **Planned substitute:** the implementation can portably enforce only an instrumented ledger of application-requested buffers/textures/render targets below 2,000 MiB. Driver/backend residency and overhead remain untracked.
- **Rationale:** no reviewed provider covering resident graphics allocations on both named Metal and NVIDIA/Vulkan acceptance machines is specified, and inventing one would be an unsupported technical claim. The ledger remains useful for optimization and comparisons.
- **User-visible/acceptance impact:** a ledger-passing build can still exceed 2 GB actual resident graphics memory. Reports therefore set `product_target_proven:false` and overall `passed:false` with `resident_graphics_memory_unproven` unless resident evidence is attached or Product explicitly approves the estimate substitution.
- **Approval required:** Product must either approve the estimate-only criterion and provide an approval ID recorded in reports, or approve a later TDD amendment naming acceptance-machine resident measurement providers. No approval is currently recorded. Regardless of all other green gates, final release status and issue `B1` cannot report Product One acceptance while this evidence/approval remains unresolved.

This is the only known Design Divergence. Features explicitly excluded by the design have no runtime system.
