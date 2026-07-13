# Benchmarks and Acceptance Evidence

## Invocation and isolation

The release demo binary runs one deterministic validation scenario per process:

```text
cargo run --release -p walkable-demo -- --benchmark flythrough --resolution 2560x1440
cargo run --release -p walkable-demo -- --benchmark carve-storm --resolution 2560x1440
```

Optional CLI fields select an output directory and an injected empty/acceptance save path, but cannot change the Product One seed/config during a comparable run. Live player/debug input is disabled. VSync is disabled for raw frame measurement while the configured resolution remains fixed; reports state present mode. Each run begins from a fresh process so cold-start and caches are not contaminated by a prior scenario.

Reports are written atomically as JSON to `benchmarks/results/<UTC>-<scenario>-<machine-id>.json` and printed as a concise console summary. A successful report has a non-optional `RequiredMetrics` object containing frame, at least one mutation-latency sample, cold start, tracked graphics-memory high-water, and a completed canonical-save sample. Missing any member changes `status` to `failed`; successful schema never represents a required metric with `null`, an empty mutation array, or “unavailable.” Failure/abort writes a report with `status: failed`, cause, and explicitly partial optional observations; it is never treated as a baseline or confused with the success schema.

```rust
pub struct RequiredMetrics {
    pub frame: FrameMetrics,
    pub mutation: MutationMetrics, // samples: NonEmptyVec<MutationSample>
    pub cold_start_ms: f64,
    pub graphics: GraphicsMetrics, // tracked bytes always present
    pub save: SaveMetrics,         // completed on-disk canonical slot
}
```

## Machine profile

Every report contains:

```rust
pub struct MachineProfile {
    pub os_name: String,
    pub os_version: String,
    pub architecture: String,
    pub cpu_brand: String,
    pub physical_cores: u16,
    pub logical_cores: u16,
    pub system_memory_bytes: u64,
    pub gpu_adapter_name: String,
    pub gpu_backend: String,       // Metal | Vulkan | Dx12, etc.
    pub gpu_device_type: String,
    pub driver: Option<String>,
    pub unified_memory: bool,
}
```

Run metadata includes UTC timestamp, scenario/schema identifier, git commit/dirty flag, Rust target/toolchain, Bevy version, debug/release profile, resolution, present mode, seed, every component digest from the `config.md` fingerprint registry, the exact aggregate `acceptance_config_fingerprint`, placeholder-assets flag, and the Linux discrete target’s `provisional | rebaselined` status. A result without this profile/metadata is invalid comparable evidence. Digest keys use the registry names verbatim; a generic or independently computed `config_fingerprint` field is forbidden.

## Metric definitions

| Required metric | Exact measurement |
|---|---|
| Frame rate | `measured_presented_frames / measured_wall_seconds`; report mean FPS, median/p95/p99/max frame ms and 1% low FPS. Loading/warmup is excluded and its duration stated. Acceptance mean is >=60 at the target resolution. |
| Mutation-to-surface latency | For each accepted public command, `SurfaceCommitted.presented_frame - MutationAccepted.accepted_frame`; also monotonic milliseconds. Report every sample and p50/p95/max. Representative 3 m commands require max <=2 frames. |
| Cold start | Monotonic duration from process entry immediately after argument parsing starts to `WorldBootEvent::ControlReady`; shader/pipeline preparation and compatible save application required for the initial playable view are included. Target <5,000 ms. |
| Active graphics memory | Peak app-tracked GPU allocations on every platform; required device-local resident high-water from identified Vulkan/driver telemetry on the discrete Linux baseline; and process physical-footprint high-water on M4 unified memory. Tool/source and memory model are recorded; unsupported residency is null. Each applicable graphics figure targets <2,000,000,000 bytes. |
| Save size | Final compressed single-slot file bytes created by that scenario's specified public save phase; also uncompressed payload, changed voxel/brick counts, and canonical delta hash. Target <50,000,000 bytes in each scenario. |

CPU resident set/peak, dense/uniform brick counts, task queue depth, mesh vertices/triangles, main-thread mutation milliseconds, and frame-to-frame activation counts are additional diagnostics. On M4, graphics and CPU allocation metrics are reported alongside total process memory because memory is unified.

“No visible hitch” is evaluated for each representative carve: no presented frame above 25.0 ms and no frame with more than 4 ms of main-thread mutation/apply work. The raw per-frame window from two frames before acceptance through two after commit is retained in the report.

## Scripted flythrough

The generator’s named `RegionManifest` waypoints define a checked-in logical path, avoiding fragile hard-coded coordinates while remaining identical for the fixed seed. The scenario:

1. Reaches control-ready, holds 5 seconds for pipeline/streaming warmup, then resets frame counters.
2. Follows deterministic position/look-at splines at fixed world speeds through the meadow and dense two-species forest.
3. Frames boulders/stumps/rocks, the carved river and lake, exposed tilted-strata cliff, and cut-stone ruin/staircase.
4. Travels without scene/state transition from canopy-level cliff to cave mouth and approximately 40 m local depth, framing the aquifer and iron vein.
5. Turns back/outward far enough to exercise each active distance band and far-region presentation.
6. Measures the traversal for at least 90 seconds. It then keeps the camera on the named `benchmark_hillside` volume, submits exactly one quantized radius-3 m strength-255 public dig at the checked-in `flythrough_metric_carve` point, waits for `SurfaceCommitted`, and records its non-empty latency/no-hitch sample.
7. Through `SaveSingleSlot`, saves that one-carve delta to the injected isolated slot, waits for `SaveCompleted`, reads the public save metrics/hash, records final graphics high-water, writes the report, and exits.

The path is camera-driven validation rather than player gameplay, but it maintains the same public activation/query API. The metric carve and save are part of the literal scenario specification and use the same supported commands/events as the demo; their post-traversal frames are excluded from traversal FPS but retained in the hitch window. It does not teleport world state, preload raw voxel arrays, or use internal feature/storage access; named waypoint metadata is part of the supported manifest observation API.

Acceptance is measured at 2560x1440 on the identified 3060-class Linux machine and at the selected 1080p–1440p native test resolution on the 32 GB M4 Mac Mini. The 3060 result remains explicitly provisional until a real run is approved and re-baselined.

## Carve-storm scenario

Starting from an empty injected save slot, the script uses the public ray/query, activation, and mutation API at named hillside/cave/forest test volumes. It performs exactly 512 radius-3 m operations at quantized centres:

- 384 dig spheres advance through a deterministic 3D lattice/corridor, including the signature hillside tunnel and brick-boundary cases.
- 128 place spheres cycle through topsoil, limestone, shale, granite, iron ore, wood, and cut stone at separate/overlapping deterministic centres.
- Each command waits for the prior `SurfaceCommitted` event, retaining interactive latency meaning; camera/frustum stays on the affected surface and then advances to the next centre.

The scenario checks every accepted command has collision ready by the next fixed tick, visible commit within two presented frames, no hitch by the defined thresholds, and no stale revision. At the signature hillside sequence it drives the scripted player capsule through the opening using semantic intent and asserts no solid overlap. It also mutates dressing support and an object-backed volume to exercise derived removal/per-instance remeshing without invoking felling/physics.

After operation 512 it saves the canonical delta slot, unloads affected active caches by moving focus, reloads the slot in-process through the public persistence command, reactivates, and compares canonical delta hash plus deterministic sample probes for exactness. The final save is the reproducible “heavily defaced” `<50 MB` acceptance case.

Its FPS interval begins after the same five-second warmup and covers the complete 512-command camera-visible storm, excluding cold-start/warmup and final reload verification. Mutation metrics contain all 512 samples; graphics high-water covers warmup through reload; cold start is measured from process entry; and the final 512-operation canonical slot supplies its non-null save metric.

## Per-scenario completeness and pass rules

| Scenario | FPS source | Mutation source | Cold-start source | Graphics source | Save source |
|---|---|---|---|---|---|
| `flythrough` | >=90 s scripted traversal | one `flythrough_metric_carve` event pair | process entry -> `ControlReady` | peak tracked allocation over warmup, path, carve, save | completed canonical one-carve slot |
| `carve-storm` | all 512 visible command cycles | all 512 event pairs | process entry -> `ControlReady` | peak tracked allocation through unload/reload | completed canonical 512-operation slot |

Each successful scenario independently requires mean FPS >=60 at its target resolution, cold start <5,000 ms, every representative radius-3 m sample <=2 presented frames, the defined no-hitch ceilings, tracked graphics bytes <2,000,000,000, and compressed save bytes <50,000,000. The discrete Linux resident-memory requirement additionally applies when establishing/re-baselining that target. A scenario does not borrow a metric or pass from the other report.

## Baselines and comparison

Reports are comparable only when scenario ID, seed, `acceptance_config_fingerprint`, asset-placeholder flag, resolution, build profile, and relevant machine profile fields match. The report retains component digests to diagnose which member changed; aggregate equality is the normative configuration comparison. A baseline selector groups results by machine ID; it never combines M4 unified-memory and Linux discrete-GPU values as if they were one target.

The first passing run per named target becomes a candidate baseline. Re-baselining requires an explicit recorded reason and preserves the old JSON. Later substrate changes report absolute values and percent deltas for FPS/frame percentiles, mutation latency, cold start, graphics allocation, and save size.

## Automated and human gates

Headless CI validates scenario scripting, event correlation, success-schema rejection of missing/empty fields, deterministic path/operation lists, canonical saves, and partial failure-report behavior. Its dry-run observations are test fixtures, not successful comparable benchmark reports. Real performance/visual gates run release builds on the named machines because headless CI cannot produce or pass the required GPU frame and graphics-allocation metrics.

The benchmark recorder uses monotonic CPU timestamps and presented-frame IDs; it never sleeps to create fixed ticks. Fixed simulation is advanced by the actual app schedule during performance runs and explicitly controlled in headless script tests.
