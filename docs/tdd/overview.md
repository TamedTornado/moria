# Moria Product One Technical Design

## Purpose and requirement interpretation

Product One is implemented as a reusable voxel-world library plus a separate Bevy validation application. The authoritative state is a deterministic, procedural base sample overlaid by sparse absolute voxel deltas. Terrain meshes, water meshes, object meshes, dressing, collision candidates, and diagnostics are all derived views. The demo has no storage access: it uses the same public query, command, event, activation, and persistence interfaces available to another Bevy consumer.

The region uses metres in public APIs and Bevy transforms, with X/Z horizontal and Y up. Its half-open bounds are `[0, 1000) x [-128, 128) x [0, 1000)` metres. Typical surface Y is about `64 m`, leaving about `64 m` to the top and `192 m` to the bottom. “Cave floor at -40 m” is represented as a route depth of approximately 40 m below the local surface/cave entrance (normally world Y near 24 m), not world Y = -40 m. This makes that experiential requirement consistent with the required regional elevation envelope. Route validation records both world elevation and local-surface-relative depth.

There are no product-design divergences. Two scope clarifications are testable contracts:

- “No visible hitch” means the representative 3 m carve never produces a presented frame longer than 25.0 ms in the acceptance scene and consumes at most 4 ms of main-thread mutation/apply work in any one frame. The harder requirement remains that all affected visible surfaces commit by the end of the second presented frame after command acceptance.
- Place selects from solid, placeable seed materials. Air is produced by dig; water is not placeable because Product One has generated static water bodies and deliberately excludes dynamic fluid editing.

## Selected stack

- Rust stable edition 2024, with `rust-toolchain.toml` pinning the project’s tested stable toolchain. Rust fits the memory-sensitive sparse world representation and permits pure, deterministic generation/query functions.
- Bevy 0.19 for ECS, application schedules, tasks, assets, renderer, UI text, input, and cross-platform wgpu graphics. This is the binding engine version for the project. No Apple-only rendering API is allowed.
- A Cargo workspace with exactly two packages: `moria-world` (library and Bevy plugin) and `walkable-demo` (binary). A workspace is warranted here because the world product and validation demo are explicitly separate deliverables sharing core logic. Additional crates or binaries require a concrete new deliverable.
- Project-owned action mapping over Bevy input. Product One has a fixed single-player keyboard/mouse control set and no rebinding UI, so an external action-mapping dependency is unnecessary; gameplay still consumes semantic actions rather than raw keys.
- CPU generation, voxel mutation, collision queries, and terrain meshing on Bevy task pools. The load-bearing path does not require GPU compute or 64-bit GPU atomics. GPU counters, allocation indices, indirect counts, and propagation labels are `u32`; CPU-side reports may accumulate them into `u64`.
- `serde` plus a fixed binary codec and Zstandard compression for the single delta save and JSON serialization for benchmark reports. The precise save envelope is specified in `persistence.md`; it has compatibility rejection but no migration machinery.

Workspace dependencies are deliberately limited to: `bevy 0.19`; `serde` derive, `serde_json`, and `ron` for typed data/report encoding; `zstd` and `crc32fast` for the save envelope; `blake3` for 32-byte config/material/asset fingerprints; `smallvec` for bounded query metadata; `thiserror` for typed public errors; `directories` for the platform save root; `sysinfo` and `time` for machine/timestamp reporting; and `proptest` as a development dependency for world properties. Exact compatible patch releases are fixed by committed `Cargo.lock`. There is no physics engine, general noise generator, action-input package, database, networking stack, scripting runtime, or custom GPU-compute dependency because the required collision, deterministic sampling, fixed controls, single local slot, and CPU-portable renderer are project-owned or provided by Bevy.

Cargo profiles follow the Bevy application convention:

```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3

[profile.release]
codegen-units = 1
lto = "thin"
```

Release is used for acceptance benchmarks and distributable builds, not as the normal development loop.

The reusable deliverable is the `moria-world` crate source/API. Each downloadable native demo artifact is a zip/bundle containing the release `walkable-demo` executable, the root `assets/` tree in Bevy's expected relative location, asset licenses, and metadata with the seed plus exact `generation_fingerprint`, `material_fingerprint`, `persistence_fingerprint`, and aggregate `acceptance_config_fingerprint` defined in `config.md`; it never embeds a second private world implementation. Bundle verification recomputes these digests from the packaged preset/assets before publication. Initial verified bundles target macOS arm64 and the Linux acceptance machine. The wgpu path must continue to compile for Vulkan/DirectX-class targets, but Product One does not promise a web build or installer.

## High-level architecture

```text
curated seed + generation config
              |
              v
  pure procedural base sampler -----> deterministic region manifest
              |                              |
              v                              v
       composite WorldQuery <------ object/stamp spatial index
              ^
              |
      sparse EditDeltaStore <------ WorldCommand (dig/place/load)
              |
       +------+--------------------+
       |             |             |
       v             v             v
  collision      streaming      persistence
  queries        activation      single slot
                      |
             revisioned worker jobs
               /       |        \
              v        v         v
        terrain mesh  object    dressing/water
              \        |         /
               derived render world

walkable-demo -> public world API only -> player/camera/debug/benchmarks
```

The base sampler answers a material/density query without expanding the region. It combines column terrain, 3D strata/cave/ore/aquifer functions, generated water, deterministic object placements, and the ruin stamp. The delta store contains only coordinates whose current composite voxel differs from this base. A queried voxel is `delta(coord)` when present and `base(coord)` otherwise. Immutable base object provenance separately assigns each exterior composite crossing to one terrain brick or one object artifact, so shared/unique object rendering never duplicates a terrain surface.

Streaming selects representations rather than deciding truth. Uniform bricks remain a constant-sized descriptor. Only surface-crossing, cave-visible, inspected, or edited bricks become dense. Revision-tagged worker jobs create disposable meshes and dressing; stale results are discarded. Public eager bulk queries have hard scanned-cell/result caps and do not activate caches. Mutation updates authoritative voxel deltas synchronously, so collision queries observe the edit on the next fixed tick; an exclusive two-thread mutation pool, bounded extraction domain, two reserved upload packets, and atomic commit barrier ensure the visible revision replaces the old surface within two rendered frames on acceptance hardware even when normal queues are saturated.

The demo collects device input per frame into semantic intent. Deterministic player movement and voxel collision run in `FixedUpdate`; camera, streaming priorities, rendering commits, lighting, UI/diagnostic presentation, and input collection run in frame schedules. Save and benchmark orchestration use public commands/events, never internal storage.

## Repository and module structure

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml
AGENTS.md
assets/
  generation/
  materials/
  vegetation/
  stamps/ruin.ron
crates/
  moria-world/
    Cargo.toml
    src/
      lib.rs
      coordinates.rs
      material.rs
      generation/
        mod.rs
        columns.rs
        geology.rs
        hydrology.rs
        manifest.rs
        objects.rs
      storage/
        mod.rs
        brick.rs
        delta.rs
      query/
        mod.rs
        raycast.rs
      mutation/mod.rs
      streaming/mod.rs
      meshing/mod.rs
      rendering/mod.rs
      dressing/mod.rs
      persistence/mod.rs
      diagnostics/mod.rs
      testing.rs
  walkable-demo/
    Cargo.toml
    src/
      main.rs
      app.rs
      input/mod.rs
      player/
        mod.rs
        movement.rs
        camera.rs
      debug/mod.rs
      benchmark/
        mod.rs
        flythrough.rs
        carve_storm.rs
        report.rs
      ui/mod.rs
    tests/
      product_acceptance.rs
benchmarks/results/.gitkeep
docs/tdd/
```

`moria-world/src/lib.rs` is a thin public facade exporting the supported consumer API and `MoriaWorldPlugin`. Storage and generation implementation modules remain private. `walkable-demo/src/main.rs` parses launch mode, constructs the app, adds top-level plugins, and runs it; feature logic stays in domain plugins. Unit tests live beside cohesive pure logic. Cross-package API and headless acceptance tests live under the consuming package's `tests/` directory so ordinary workspace `cargo test` discovers them.

## Plugin and schedule ownership

`MoriaWorldPlugin` owns generation, material registry, query service, mutation, streaming, meshing, world rendering, dressing, persistence, and diagnostics. It exposes documented configuration resources, commands, and events. `WalkableDemoPlugin` groups `InputPlugin`, `PlayerPlugin`, `DebugToolsPlugin`, `DemoUiPlugin`, and optionally `BenchmarkPlugin`.

The schedule contract is:

- `PreUpdate`: read physical input, update `ActionState`, convert edge events into buffered `PlayerIntent`/`DebugIntent`, and accept public world commands.
- `FixedUpdate`: consume buffered movement/jump/paddle intent, query voxel truth, resolve the kinematic player, and publish the authoritative player pose. Explicit sets run `Intent -> Move -> Collide -> Finalize`.
- `Update`: choose activation bands, dispatch/collect worker jobs, run camera orbit/collision, update the underground light and fixed sun, drive benchmark scripts, and update diagnostics/UI. Mutation-result collection is ordered before normal streaming commits.
- `PostUpdate`: install revision-valid render assets/transforms, synchronize dressing visibility, and record the presented-frame commit marker used for latency measurement.

No fixed system reads `just_pressed` or raw device state. Tests control fixed time explicitly and do not assume one `App::update()` equals one fixed tick.

## AGENTS.md specification

The repository root `AGENTS.md` created during implementation must contain the following commands verbatim:

```text
Format check: cargo fmt --all -- --check
Compile check: cargo check --all-targets
Lint: cargo clippy --all-targets -- -D warnings
Test: cargo test
Build: cargo build --workspace
Dev: cargo run -p walkable-demo
Release demo: cargo build --release -p walkable-demo
Flythrough benchmark: cargo run --release -p walkable-demo -- --benchmark flythrough --resolution 2560x1440
Carve-storm benchmark: cargo run --release -p walkable-demo -- --benchmark carve-storm --resolution 2560x1440
```

It must also state:

- Keep `Cargo.lock` committed. Keep root `assets/` because Bevy’s default `AssetServer` path is used.
- Organize by feature/domain plugin, not global `components.rs`, `resources.rs`, `systems.rs`, `types.rs`, or `utils.rs` buckets. Keep `main.rs` and `lib.rs` to wiring and public facade work.
- Rust modules/files use `snake_case`; types/plugins/messages use `UpperCamelCase`; systems use verb-first `snake_case`; constants use `SCREAMING_SNAKE_CASE`. World coordinates end in `_world`, voxel coordinates in `_voxel`, and brick coordinates in `_brick` when the type alone is not obvious.
- Within a crate, import through `crate::feature`; between packages, import only from `moria_world` public exports. Do not import private storage/generation internals into the demo. Avoid glob imports except Bevy’s documented prelude in module headers; do not create a project-wide prelude.
- Components represent per-entity data; resources are globally unique configuration/services. Never derive the same type as both. Queries must include meaningful filters and must not use unfiltered `Query<Entity>` because Bevy 0.19 resources are component-backed.
- Collect raw input only in `input`; fixed simulation consumes semantic intent. Put deterministic movement, collision, and cooldown-like rules in `FixedUpdate`; keep camera smoothing, visual interpolation, UI, audio, and effects in frame schedules. Declare ordering whenever a system reads another system’s output.
- The world library owns voxel storage. Consumers, including the demo, may use only public queries, commands, activation requests, and events. Derived meshes/dressing are never saved or treated as collision truth.
- All procedural generation uses stable integer/fixed-point math and the project stable hash, never `DefaultHasher`, iteration order, wall time, or platform-dependent random sources.
- Repeated rendered objects share mesh/material handles until an instance is actually edited. Do not create per-instance materials. Measure representative forest/carve scenes before proposing a custom render pipeline.
- Shaders and GPU-visible counters/indices/labels use 32-bit operations; no load-bearing Apple-only path or required 64-bit GPU atomic is permitted.
- Logic tests use `MinimalPlugins` or selected plugins, explicit time, seeded entities/resources, and world-state assertions. Never open a window or sleep in a test. Rendering correctness is manual/benchmark validation.
- Have the independent adversarial test-author session own acceptance/property tests. Implementation work may satisfy those tests but must not delete, relax, or rewrite their contracts to make a failure pass.

## Requirement coverage index

| Product requirement | Technical representation |
|---|---|
| Deterministic curated 1 km region and guaranteed route features | `data-model.md` generation pipeline, manifest, coordinates, feature validators |
| Lazy columns/bricks, compact uniform matter, active bands | `data-model.md`, `systems.md`, `config.md` |
| Material truth, objects, dressing, smooth/sharp derived presentation | `data-model.md`, `rendering.md`, `assets.md` |
| Run/sprint/jump/paddle, voxel collision, orbit camera/light | `systems.md`, `states.md`, `config.md` |
| Dig/place and all diagnostic toggles through supported API | `api.md`, `systems.md`, `states.md` |
| Two-frame carve and no hitch | `systems.md`, `benchmarks.md` |
| Single exact delta save | `persistence.md` |
| Flythrough/carve storm and all required metrics | `benchmarks.md` |
| Scope exclusions | `systems.md` non-systems list and API capability boundaries |
