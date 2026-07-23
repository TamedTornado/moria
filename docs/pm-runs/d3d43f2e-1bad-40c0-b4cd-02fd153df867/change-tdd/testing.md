# Verification and acceptance plan

## Test strategy

Tests extend the repository's existing infrastructure. No new framework or
window-based logic harness is introduced.

- Pure unit/property tests cover coordinate conversion, canonical digests,
  generation, ID packing, per-cell indexes, mutation kernels, delta encoding,
  report validation, and CLI parsing.
- Integration tests compile and drive the public facade from outside
  `moria-world`.
- Headless Bevy slices use `MinimalPlugins`, seed only required resources and
  messages, and use `moria_world::testing::run_fixed_ticks` for exact
  `FixedUpdate` counts.
- The independent `testing::conformance::DenseWorld` is extended for small
  bounded cells. Oracle code may share scalar public value types but may not
  call production generator/index/mutation/persistence helpers.
- No logic test sleeps, starts `DefaultPlugins`, opens a window, or asserts
  wall-clock timing.
- Rendering/GPU correctness uses headed benchmark evidence and F2 human review.

When issues are executed, test-writing and implementation should be assigned to
separate sessions/agents. Test specifications are reviewed before the
implementation issue is considered complete.

## Focused automated contracts

### Definition, coordinates, and identity

Add tests under `crates/moria-world/tests/` for:

- valid aligned finite bounds at negative/positive coordinates;
- empty, inverted, non-brick-aligned, non-cell-aligned, conversion-overflow,
  and more-than-48-bit-cell bounds rejection;
- every bounds face/corner and one unit outside for Q8/voxel/brick/cell
  conversion;
- canonical digest golden vectors, including signed integers, strings, enum
  discriminants, empty/nonempty vectors, and stamp bytes;
- RON whitespace/comment/field-layout changes not altering identity after
  parse, while every authoritative value change does;
- presentation, machine, benchmark, and diagnostic input not changing identity;
- schema/generator version or digest mismatch failing before readiness; and
- unknown fields, unsorted/duplicate anchors, over-cap fixture data, or
  out-of-context shapes failing with stable typed errors.

Golden digest fixtures are small checked-in text values. Updating one requires
an explicit schema/generator version review; implementation code must not
rewrite the expectation automatically.

### Bounded generation and stable IDs

Property generators select small aligned worlds and request-cell subsets.
Assert:

- same identity/cell produces byte-identical `GeneratedCell` and result digest;
- random request permutations, duplicates, task completion permutations, and
  parallel partitions produce the same per-cell/output digest;
- no request generates owner cells outside the clipped one-cell context;
- adjacent faces/edges/corners return identical material samples;
- cross-cell overlapping procedural candidates select the same lower canonical
  candidate from either request order, while authored conflicts fail
  definition validation;
- objects crossing a core boundary appear once, with the owner-cell ID, from
  either adjacent query;
- release/evict/reactivate reproduces the same material/object/index bytes;
- retained generated-cell count is bounded by the deduplicated core/context
  union, never world cell count;
- every accepted candidate ordinal packs to the documented nonzero ID and all
  IDs are unique; overflow is rejected before generation; and
- edit deltas survive generated-cell eviction and overlay regenerated base
  exactly.

An instrumented generator counter is test-only/public proof telemetry, not a
second generation path.

### Index and exact oracle

For random small cell populations and boundary-crossing shapes, compare
production output to brute force for:

- point sample membership;
- AABB placement membership;
- dependency membership for changed voxel masks;
- exact object/object and object/stamp overlap conflict/witness; and
- exact affected object IDs for sphere/box edits.

Run insertion, cell-request, and candidate-order permutations. Both sides must
produce sorted unique IDs. Add cap tests at exact maximum and one over for
object candidates, dependency/sample cell membership, dependency bricks,
affected objects, retained bytes, and context extent. The production result
must error rather than truncate.

### Public reads and streaming

Retain current query boundary tests, changing only fixed bounds and removed
route assumptions. Add headless plugin tests that:

- open from compact definition and become ready without a complete population;
- sample inactive cells without retaining the whole world;
- activate/release exact cell sets and verify core/context counts;
- replace/remove focus sources by ID;
- discard stale generation results after release/re-request;
- evict generated and materialized detail while retaining deltas;
- page diagnostics without exposing private entities/store handles; and
- assert query output is identical before/after cache activation.

### Mutation and reconciliation

Use controlled fixed ticks. Assert:

- admission validates opened bounds and preserves existing duplicate/queue/work
  limit behavior;
- accepted reservations commit only in `FixedUpdate`;
- one batch is atomic and query-visible before its message;
- changed-brick and affected-object vectors equal the independent oracle;
- batch indices/revisions/order and terminal digests are deterministic;
- no-op uses the selected zero-batch terminal without revision change;
- objects outside the affected set retain fingerprints and revisions;
- affected derived items install a current-revision result or tombstone;
- stale task results do not acknowledge the barrier;
- primary readiness cannot complete terminal reconciliation;
- exact expected/acknowledged key equality is required; and
- an edit near a cell face invalidates all and only dependencies owned by
  relevant context cells.

### Persistence

Use temporary directories and small dense worlds:

- identical sorted deltas produce byte-identical compressed files;
- input mutation order does not change bytes;
- saved cells are exactly current values unequal to regenerated base;
- decoded bricks/cells are strictly ordered and derived data is absent;
- save snapshots one revision while later commits continue independently;
- atomic replacement preserves the old slot on injected write/sync failure;
- truncated, oversized, invalid-zstd, unknown-field, checksum, out-of-bounds,
  unsupported version/generator, wrong-world, duplicate/unsorted, invalid
  material, and base-no-op files fail without truth/revision change;
- successful load swaps on exactly one fixed tick, blocks edits during load,
  and reconciles before completion; and
- save/reopen/load reproduces every delta coordinate and deterministic
  unedited samples/query results.

### Reports, CLI, and output

For substrate proof and benchmark documents:

- each missing required field/stage, wrong order, duplicate stage, nonfinite
  timing, inconsistent total, mismatched digest, invalid machine availability,
  false passed flag, or unsorted/duplicate failure reason is rejected;
- `NotStarted`, `Partial`, `Failed`, and `Passed` round-trip distinctly;
- correctness digests ignore machine/timing/timestamp but change for any
  deterministic result/workload change;
- compare-proofs accepts matching correctness on distinct Linux/macOS profiles
  and rejects same-profile, commit/world/fixture/version/stage/digest mismatch;
- runtime failures after output acceptance write complete valid non-pass JSON
  and exit 1;
- argument errors write nothing and exit 2;
- an existing output is never overwritten;
- temp files are flushed and atomically renamed; and
- generated stress parameters reconcile exactly with generated cell/object
  totals and never alter `assets/`.

## F1: cross-platform headless substrate proof

F1 is blocked until the ordinary gate passes and the active docs/commands no
longer require the global manifest. Use the exact commands in
[overview.md](overview.md) on the same git commit and input digests.

The fixture contains at least:

- two adjacent core cells sharing faces/edges/corners;
- one cell requested only as context;
- repeated activation, duplicate requests, release, eviction, and reactivation;
- surface and underground material samples;
- neutral objects wholly inside and crossing a core boundary;
- one authored neutral sparse stamp;
- bounded point/AABB/dependency broad-phase probes;
- one net-changing edit intersecting an object dependency;
- one edit that affects no object;
- one net no-op;
- save, fresh open, and load; and
- retained-state/accounting checkpoints before activation, at peak, and after
  release.

Each required stage in `SubstrateProofReport` must pass. Blocking conditions
are functional, deterministic, bounded, and integrity conditions:

- repeated and cross-host correctness digests match;
- generation never leaves the declared core/context union;
- stable IDs and sorted material/query results match after permutations and
  reactivation;
- production broad phase and mutation attribution equal the independent oracle;
- unaffected fingerprints are unchanged and every affected item reconciles;
- save/reload reproduces exact authoritative and query-visible truth;
- retained category totals reconcile and released state falls to the expected
  pinned/delta baseline; and
- evidence self-validation finds no missing/contradictory stage.

Elapsed time and measured memory are mandatory finite machine-labeled evidence,
but no numeric timing or named-machine threshold controls F1. Deterministic
configured capacity violations remain functional failures.

## Generated population stress

The canonical acceptance workload is the command in `overview.md` with 512
cells and 256 requested candidates per cell. Those values identify one
repeatable workload; they do not describe shipped content.

The report must reconcile:

- requested, accepted, rejected, and duplicate candidate counts;
- generated core/context cells;
- object/index entries and retained-byte categories;
- query/edit candidate maxima;
- generation/index/query elapsed distributions; and
- complete machine/build/world/workload identity.

There is no universal time threshold. Failure means a correctness, configured
capacity, accounting, output-integrity, or completion failure. Performance is
reported for later profile-specific review. Generated details and reports stay
under `target/`; a test asserts `git status --short -- assets` is unchanged by
the scenario.

## Headed benchmarks

`diagnostic-streaming` must exercise:

- camera/inspection focus movement across at least four noncanonical generated
  cell paths selected from workload parameters;
- core/context activation, LOD transitions, release, and eviction;
- surface and underground presentation;
- shared repeated-object handles and visibility management;
- bounded diagnostic pages/material overlay; and
- accounting before, peak, and returned steady state.

`mutation-reconciliation` must exercise:

- one object-affecting edit and one object-independent edit through
  `WorldEditWrite`;
- exact batch/affected IDs and unchanged unaffected fingerprints;
- terrain/seam/object install or explicit removal acknowledgements;
- visible authoritative-vs-derived pending/current states;
- terminal barrier equality;
- save and fresh-world reload; and
- complete frame, queue, CPU/GPU ledger, machine, and proof linkage evidence.

Reports fail on missing work, fallback assets, stale installation, queue
overflow, unreconciled items, invalid accounting, or missing identity. Frame
times and resource values are evidence only until Product approves a hardware
profile threshold.

## F2 headed human review

The reviewer runs the exact viewer command in `overview.md` on macOS and
records:

- build/world/fixture digests and complete machine identity;
- visible three-dimensional surface and underground geometry;
- visible bounded activation and eviction while moving the free camera;
- an authoritative dig/place result in material/voxel diagnostic view;
- the neutral registered object entering pending/invalidated state after the
  relevant edit and returning as rebuilt or current-revision tombstoned;
- no stale derived result after reconciliation;
- successful save/load observation; and
- confirmation that no player/avatar/locomotion/animation/curated route is
  required.

F2 passes only with all observations and an explicit reviewer disposition.
Screenshots/video may support the record but do not replace the checklist.
Resolution, timings, and scene attractiveness do not define correctness.

## Full quality and acceptance command order

From the repository root:

```sh
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test
cargo run -p moria-curate -- check
cargo build --all-targets
```

Then run F1 on Linux and macOS and compare the reports, followed by the three
release benchmark commands and F2 viewer command in `overview.md`.

No implementation issue is complete with a failing focused test or ordinary
gate. F1 failure blocks headed breadth. Any F1/F2 failure requires a reviewed
TDD revision with the immutable artifact, failing stage, measured cause,
redesign, and replacement proof. Workload/exactness/evidence requirements
cannot be reduced to turn a failure green.
