# Benchmarks and Acceptance Evidence

## Goals and reproducibility

`moria-bench` is a separate headed binary that uses the same public `MoriaWorldPlugin`, `WorldRead`, `WorldEditWrite`, focus messages, save requests, assets, seed, and presentation defaults as the demo. It has no player movement or private store access. Its only special behavior is scripted camera/action input and metric capture. A benchmark build fails review if dependency inspection shows access to a non-public `moria-world` module.

Every run starts in a fresh process, uses an isolated temporary save directory unless an explicit test directory is supplied by the harness, and records the git commit, Cargo profile, world seed/config digest, asset warning/fallback list, resolution, adapter/backend, and machine profile. Results without the complete machine profile are invalid. Different machines are grouped by `machine_profile_id`; aggregate tooling never silently merges them.

The Linux 3060-class result begins with `baseline_status: "provisional"`. After the first verified run on the designated Linux test machine, the checked-in baseline metadata records that exact adapter/CPU/driver/profile and becomes `verified`; any numeric re-baseline is an explicit reviewed data change, not an automatic weakening. The M4 Mac Mini 32 GB results are separately identified for 1080p and 1440p.

## Pre-implementation feasibility evidence

Before broad feature implementation, the release-mode M4 runs defined in [implementation-plan.md](implementation-plan.md) must produce passing, digest-matched `ForestFeasibilityReport` and `CarveFeasibilityReport` artifacts. These are proof gates, not reduced versions of final acceptance.

The forest proof uses the complete checked-in placement population and requires area, density/counts, species mix, 5 m spacing, 2–4 m canopy range coverage, 3 m route clearance, voxel-shape/ruin disjointness, both object-index tables, the 64 m Horizon-cell member cap, edit-candidate maxima, <=1,000 ms production object validation, <=250 ms index construction, <=16 MiB retained bytes, and zero dependency-coordinate allocation to pass simultaneously. The carve proof consumes that artifact and runs the same full manifest plus public production edit path at 2560x1440/Metal through render extraction; its normal headed startup must remain <5,000 ms. Both its signature and maximum-candidate stress roles must meet the two-frame and 33.3 ms rules; dirty discovery plus dependency eligibility must be <=1.0 ms. The stress role also forces the affected tree cell through the active Horizon aggregate path and proves that its base card is excluded in favor of a revision-matched derived payload/tombstone before readiness. Its query subrun also validates the public synchronous limits/cost budgets in [api.md](api.md).

The report validators reject a stale manifest/git digest, wrong profile/machine/backend/resolution, missing pipeline stage, incomplete extraction/GPU-prepare/render-queue barrier, theoretical-only object work evidence, or asset fallback. A failed gate remains failed and blocks downstream issue IDs; acceptance targets cannot be tuned by the report producer. Final flythrough/carve-storm runs do not retroactively excuse a skipped gate.

## Run lifecycle

1. At process entry, record monotonic `process_start` before Bevy app construction.
2. Parse arguments and create the acceptance-size window/render targets.
3. Load the curated seed/config and enter normal world startup.
4. On `WorldReady` plus scenario-view readiness, record `control_ready`; `cold_start_ms = control_ready - process_start`.
5. Run 300 rendered warmup frames along the first route segment. Warmup is excluded from FPS distributions but its allocations stay resident/accounted.
6. Reset frame/edit sample buffers without resetting the world or renderer.
7. Execute exactly one scenario.
8. Complete required save, metrics, and report validation.
9. Write `<output>.tmp`, flush, and atomically rename to the requested JSON path.
10. Exit 0 only if the report is complete and all scenario-applicable contracts pass.

The watchdog is 300 seconds. A timeout writes `passed: false`, includes the last state/waypoint/request ID and missing metrics, and exits 1. It never fills a metric with zero or a fabricated value.

Startup telemetry separately records complete object-validation duration, object-index build duration, retained index bytes, placement/record count, dependency/sample-grid entry and per-cell maxima, maximum Horizon tree members in one 64 m cell, maximum edit candidates/affected IDs, maximum dependency bricks observed, and dependency-coordinate allocation bytes for the full checked-in manifest. These values populate `streaming.object_index`. Report validation rejects validation/build times above 1,000/250 ms on the M4, retained index bytes above 16 MiB, a footprint above 128 bricks, dependency/sample cells above 1,024/64 members, a Horizon cell above 1,024 trees, edit candidates/affected IDs above 256/64, or any nonzero dependency-coordinate allocation. This phase remains inside—not in addition to—the process-to-control-ready `< 5,000 ms` acceptance window.

## Scripted flythrough

Duration is 120 seconds after warmup. A C1-continuous Catmull-Rom camera spline passes the ordered manifest tags:

1. meadow and grass dressing;
2. dense forest with visible birch, pine, bushes, boulders, stumps, and rocks;
3. river channel and lake;
4. cliff top and exposed tilted strata;
5. ruin exterior and intact staircase;
6. surface cave mouth;
7. cave descent with visible aquifer and iron vein; and
8. cave floor near -40 m.

Spline control points are generated from route waypoints and checked for in-bounds, solid-collision-free camera probes. The camera travels continuously—no teleport or level transition—and publishes normal camera focus so band activation/eviction is exercised. Script speed slows at tagged evidence views but is fixed by manifest timestamps. The underground light uses the same system/config as the demo.

Flythrough captures:

- every rendered-frame wall interval and effective FPS;
- cold-start time;
- active graphics allocation estimate each frame and peak/category totals;
- streaming queue depths, active brick/mesh/object counts, and allocation high-water marks;
- resolution, build/world/asset identity, and machine profile; and
- current clean save-slot size (`0` when the isolated slot is absent).

At completion, coverage validation requires at least one second of measured frames inside each tagged scene and at least one observed transition into every active visual distance band. Missing coverage fails the run even if performance numbers pass.

## Carve storm and heavy save

The scenario uses 128 deterministic, nonoverlapping or intentionally intersecting target definitions generated from manifest-valid surface/cave points. It includes hillside topsoil/subsoil, exposed rock/strata, cave wall near aquifer/ore, at least one registered-object material edit, and reversible place/dig stress. Every operation is a radius-3 m public `WorldEditCommand` submitted through `WorldEditWrite` with monotonically increasing request ID. Placement cycles the placeable palette; air/water are never passed.

For each operation:

1. Publish mutation focus and wait until the target is inspection-ready.
2. Submit one command; `WorldEditWrite` stamps `submitted_frame` internally, and the runner records the same public frame index and call-site time.
3. Require one matching `EditCommitted` or fail on rejection.
4. Require matching `EditSurfaceReady` no later than `submitted_frame + 2` and `committed_frame + 2`; do not issue the next edit while the prior readiness barrier is open.
5. Record all rendered-frame intervals from input through readiness.
6. Query representative inside/outside samples to assert the committed density/material effect.

The first operation is the exact signature hillside dig. After readiness, the scripted camera travels through the carved opening and a public capsule sweep must report a clear path. This validates traversability against changed solid collision rather than only mesh timing.

The operation list finishes with at least 100 spheres whose final values differ from base and are spatially distributed across at least 32 bricks/region cells; reversions used to test delta removal do not count toward heavy defacement. Then the runner sends `SaveWorldRequest` and records the final compressed file byte count. After the headed Bevy app returns to the benchmark's `main`, the orchestrator creates a second `App` with `MinimalPlugins`, required asset/time plugins, and `MoriaWorldPlugin` with presentation disabled. It loads through `LoadWorldRequest` and compares every saved delta coordinate plus deterministic random unedited samples before final report writing. The headed phase remains the source of render metrics; the second phase is exact restoration evidence and does not enter FPS samples.

Carve storm captures all flythrough identity/startup/memory/frame fields plus per-edit commit frame, ready frame, elapsed milliseconds, changed voxel/brick counts, maximum frame time during the operation window, save size, delta counts, and round-trip result.

## Metric definitions

### Frame rate and frame time

Rendered-frame interval is wall time between consecutive main-frame boundaries whose state is acknowledged by the render extraction schedule, measured with a monotonic clock after warmup. This includes main-world work, render extraction/queuing, and any backend/presentation back-pressure visible to the app; it does not claim unavailable hardware scan-out timestamps. Frames where the OS reports the window minimized/occluded or the app loses focus invalidate the run and are not silently discarded. The report stores sample count, duration, arithmetic FPS (`sample_count / measured_seconds`), median/p95/p99/max frame time, and one-percent-low FPS (reciprocal of mean slowest 1% frame times).

The Product One 60 fps contract passes when, at the requested resolution on the acceptance machine:

- arithmetic FPS is at least 60.0 in an uncapped/non-power-saving presentation mode;
- p95 frame time is at most 16.67 ms; and
- no asset fallback or missing route coverage invalidates the visual workload.

One-percent-low and p99 are reported for comparisons but do not independently fail the general flythrough in Product One. The carve hitch has its own stricter local maximum rule below. Display refresh/presentation mode is recorded; a 60 Hz vsync-capped environment that cannot measure 60.0 without rounding is rerun uncapped rather than changing the threshold.

### Mutation-to-surface latency

For edit `E`:

- submitted-visible frame latency = `EditSurfaceReady.ready_frame - EditCommitted.submitted_frame`;
- commit-visible frame latency = `EditSurfaceReady.ready_frame - EditCommitted.committed_frame`;
- elapsed latency = readiness monotonic time minus commit monotonic time.

Pass requires every accepted radius-3 m edit to have submitted-visible and commit-visible frame latency `<= 2`. Ready means every barrier item has reached render extraction, GPU prepare/free, and render queue acknowledgement for that frame. The submitted value is the consumer publication frame and is a pass/fail metric, not supplemental telemetry. A dedicated harness case submits in a rendered frame configured for zero fixed ticks, and another submits after that frame's fixed-drain cutoff; both must be ready by their submission frame plus two or be synchronously rejected before acceptance. The representative first carve additionally requires no rendered-frame interval from request through readiness above 33.3 ms. The report includes max and p50/p95 across all edits. A no-op edit is excluded from performance distributions but must still obey the two frame protocol.

### Cold start

`cold_start_ms` is process entry to the normal readiness barrier: config/manifest/save handled, supported spawn solid-collision samples ready, initial visible view installed, and control could be enabled. Pass is `< 5,000 ms`; reporting only asset-load completion is invalid. Both flythrough and carve runs report it, but a baseline summary may use the slower valid run.

### Active graphics memory

Every run records the peak sum of instrumented application GPU allocation requests defined in [rendering.md](rendering.md). The report contains peak/end totals by category, `untracked_driver_overhead: true`, and adapter/backend. A monotonic-growth check requires end resident derived-chunk bytes after returning to the initial band to be within 5% of first steady-state bytes; global cache/shared assets are compared separately. This detects failure to evict.

The ledger does not prove the `< 2,097,152,000 bytes` resident graphics-memory product target because driver/backend overhead is excluded. `graphics_memory.resident_measurement` is nullable and, when present, records `{provider, scope, sampling_interval_ms, peak_bytes, artifact_sha256, artifact_path}` from a reviewed acceptance-machine harness. `product_target_proven` is true only when that measurement covers the game process and relevant resident graphics allocations and its peak is below the threshold. Until such evidence exists, overall `passed` must be false with `resident_graphics_memory_unproven` unless Product records an explicit estimate-substitution approval ID in `estimate_substitution_approval_id`; that approval permits the ledger `< 2,097,152,000` gate but does not set `product_target_proven` true. No approval currently exists. This is an explicit pending-approval Design Divergence, not a silent substitution.

### Save size and exact restoration

Save size is filesystem metadata length of the atomically renamed zstd slot. Pass is `< 50,000,000 bytes` after the heavy-defacement criteria. `changed_voxels` and `changed_bricks` are recorded so a suspiciously tiny workload is evident. Exact restoration passes only if all delta-coordinate `Voxel` bytes match the pre-save snapshot, all sampled unchanged points equal base, identity matches, and no derived mesh/dressing bytes occur in the decoded file.

### Machine profile

Mandatory non-null fields are OS name/version/architecture, CPU model, logical cores, total physical memory, GPU adapter name/vendor/device class, wgpu backend, build git commit, Cargo profile, and resolution. Driver string/version is recorded when exposed and otherwise explicitly null with `driver_metadata_available: false`. Unified/discrete classification and M4/3060 acceptance label are explicit. A stable profile ID is SHA-256 over normalized hardware/OS/backend fields, excluding run timestamp.

## JSON report contract

The sole serializable type is `BenchmarkReport` in [data-model.md](data-model.md). Every JSON object contains these top-level keys, even when a failure makes a value null:

```text
schema, timestamp_utc, scenario, passed, failure_reasons,
baseline_status, build, world, assets, machine, resolution,
cold_start_ms, frame_rate, frame_time_ms, graphics_memory,
mutation_latency, save, coverage, streaming
```

`schema` is the literal `moria-product-one-benchmark`; `timestamp_utc` is RFC 3339 UTC; `failure_reasons` is a sorted array (empty only when `passed` is true); and enums serialize as documented lowercase-kebab-case strings. `frame_rate` is `{sample_count:u64, measured_seconds:f64, arithmetic_fps:f64, one_percent_low_fps:f64}`. `frame_time_ms` and each latency distribution are `{min,p50,p95,p99,max}` with finite `f64` values; frame distributions use the same keys with integer `u64` values. `graphics_memory.application_ledger` is `{peak_bytes:u64,end_bytes:u64,categories:object,untracked_driver_overhead:true}` and its optional resident object is defined above. `save` always exists with `{attempted,completed,size_bytes,changed_voxels,changed_bricks,round_trip}`; round trip is `{passed,delta_voxels_compared,base_samples_compared,identity_match,derived_bytes_found}`. `streaming.object_index` is `{validation_ms:f64,build_ms:f64,retained_bytes:u64,retained_byte_categories:object,placement_records:u32,dependency_grid_entries:u32,sample_grid_entries:u32,max_dependency_cell_entries:u16,max_sample_cell_entries:u8,max_horizon_tree_members_per_cell:u16,max_edit_candidates:u16,max_edit_affected_objects:u8,max_dependency_bricks:u16,dependency_coordinate_allocation_bytes:u64}` and is mandatory for every completed scenario.

For a completed flythrough, build/world/assets/machine/resolution/cold-start/frame/graphics/coverage/streaming are non-null, `mutation_latency` is null, and `save` has `attempted:false`, `completed:false`, `size_bytes:0`, count fields `0`, and `round_trip:null`. For a completed carve storm, those common fields plus mutation latency and every save field are non-null and `attempted:true`; round trip is mandatory. For an early runtime failure, every top-level key is still written, unavailable `Option` values are null, `save` truthfully records how far it got, `passed:false`, and `failure_reasons` names every missing mandatory field. A report with missing keys, fabricated zero metrics, non-finite numbers, `passed:true` with null scenario-required data, or a workload-minimum failure is rejected by `validate_report` and exits 1. Argument errors before scenario selection write no report and exit 2. This schema label identifies benchmark data shape but is not save migration/versioning.

A concise human summary is printed to stdout after the JSON is safely written. Comparison tools consume JSON, key by profile ID/resolution/scenario/world digest, and show changed metrics without declaring cross-machine regressions.

## Acceptance matrix

| Requirement | Scenario/evidence | Pass condition |
|---|---|---|
| Forest/index feasibility before breadth | F1 on checked-in manifest, M4 | All area/count/species/spacing/canopy/route/disjointness/index/startup contracts pass together |
| Complete carve feasibility before breadth | F2 signature + maximum-candidate stress trials, M4 Metal 1440p | Full production barrier including active Horizon repartition <=2 frames, max frame <=33.3 ms, discovery/eligibility <=1.0 ms, query-cost contract passes |
| 60 fps, 3060-class 1440p | Flythrough + carve storm on designated Linux machine | FPS/p95 rules at 2560x1440; first baseline provisional then reviewed/verified |
| 60 fps, M4 Mac Mini 32 GB | Flythrough + carve storm at 1920x1080 and 2560x1440 | FPS/p95 rules at each recorded resolution |
| Surface update <= two frames | Every carve-storm edit plus zero-tick/cutoff harness cases | Max submitted-visible and commit-visible frame latency <= 2 |
| Representative carve no hitch | First 3 m hillside dig | Max operation-window frame <= 33.3 ms and clear traversal path |
| Walkable under five seconds | Both scenario startups and demo smoke run | Process-to-control-ready < 5,000 ms |
| Graphics memory below ~2 GB | Peak of each headed scenario | Not proven by current plan; requires resident measurement < 2,000 MiB or Product-approved estimate substitution |
| Idle wilderness sparse | F1 + streaming/startup telemetry + unit/property tests | No voxel arrays/meshes for untouched inactive uniform/procedural bricks; both object-index tables together <=16 MiB, within startup/candidate caps, and no retained dependency-coordinate arrays |
| Delta save below 50 MB | Heavy carve-storm final save | Workload minimum met and file < 50,000,000 bytes |
| Exact reload | Carve-storm second-world comparison | Every delta voxel byte exact; unchanged samples equal base |
| Curated content/continuous route | Curator checks, flythrough coverage, manual playthrough | All feature tags covered; no teleport/level transition; solid-collision route passes |
| Material truth/debug operations | Protocol tests + manual/benchmark carve/place | Queries, collision, mesh, dressing share revision; all access public API |
| Portability/32-bit constraint | Shader smoke on Metal/Vulkan, code review/check tests | No platform-only path or 64-bit GPU atomic; checked u32 GPU fields |

## Non-automated evidence

Performance numbers cannot establish attractiveness, material readability, camera feel, or lack of visible cracks. Release review therefore attaches the visual checklist from [rendering.md](rendering.md) and the six milestone captures. The captures include seed/config digest and camera/operation metadata. Human review does not replace numerical acceptance, and numerical acceptance does not waive visual review.
