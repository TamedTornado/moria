# Validation and Evidence

## 1. Evidence contract

Validation is an ordinary consumer of `moria`. Each scenario emits a
machine-readable JSON record:

```rust
pub enum EvidenceStatus { Pass, Fail, NotDemonstrated }

pub struct EvidenceReport {
    pub schema_version: u32,
    pub scenario: String,
    pub status: EvidenceStatus,
    pub contract_ids: Vec<String>,
    pub context: MachineContext,
    pub configuration_digest: Digest,
    pub assertions: Vec<AssertionEvidence>,
    pub measurements: Vec<Measurement>,
    pub artifacts: Vec<ArtifactDigest>,
    pub diagnostics: Vec<String>,
}
```

Missing adapter, missing capture, omitted assertion, overflowed telemetry, or
incomplete scenario is `NotDemonstrated` and makes the conformance process exit
nonzero. Only a report with all required assertions `Pass` may pass.

`moria-conformance` depends on `moria` only through its public crate exports.
Compile graph and deny-list checks prevent features exposing internals.

## 2. Test layers

### Unit and property tests

- coordinate/bounds checked arithmetic and transforms;
- packed-cell validation and occupancy thresholds;
- canonical brick/scar encoding;
- revision overflow/precondition behavior;
- state-machine transition tables;
- observation gap/snapshot sequence model;
- persistence envelope corruption and compatibility;
- Merkle roots/proofs over empty, homogeneous, edge, and mixed bricks;
- host/WGSL layout equality;
- deterministic scheduling keys and pressure scoring; and
- error code/scope/retry/commit-effect completeness.

Property tests compare GPU query/mutation/collision outputs with the private
reference model for randomized bounded volumes, rotations, overlaps, patches,
and injected failures. Seeds are recorded in failure output.

### Compile-fail/public-boundary tests

`trybuild` fixtures prove consumers cannot import private storage/render
modules, construct IDs illegally, confuse coordinate types, retain GPU
snapshot lifetimes, or access internal buffer/page types.

### GPU semantic suite

Runs baseline production shaders on a real wgpu adapter and compares exact
integer truth/results to the reference model. It exercises empty,
homogeneous, mixed, edge, deep negative/positive coordinates, pool-boundary,
and transaction-boundary cases.

The `fault-injection` crate feature used here is part of documented public
configuration and is available to any external consumer. It selects a named
operation/stage failure but grants no storage access and does not bypass
admission, so the conformance binary remains an ordinary consumer.

### Scenario suite

Longer scenarios exercise cross-capability outcomes and emit evidence reports.
They are enumerated below.

## 3. Required scenarios

### C01 Public boundary

An external-style crate installs `MoriaPlugin`, starts a world with fixture
source, declares interest, waits for ready, samples/traces/overlaps, mutates,
observes, checkpoints, shuts down, and restores. Static analysis and compile
tests prove it imports no private path.

Pass evidence: every action used public tickets/leases; revision/correlation
facts agree; restored facts match.

### C02 Truth versus view

For one revision, run sample, occupancy, trace, overlap, and sweep while
presentation is absent, current, forcibly stale, failed through fault
injection, destroyed, and rebuilt.

Pass evidence: matter/collision facts and revision are identical; rebuilt mesh
matches current truth; no presentation allocation is read by collision.

### C03 Atomic mutation

Exercise remove, fill/replace, patch, stamp, predicate no-op, material
boundary, deep internal edit, and multi-brick edit. Inject GPU validation
failure after all transaction pages are staged and scar capacity is reserved.

Pass evidence: success advances exactly once and converges across query,
collision, scar, observation, and presentation; injected failure changes no
cell/revision/scar and emits no intermediate change event. Capture before,
stale, and current presentation for success.

### C04 Deep volume

Fixture source provides voids, material bands, and enclosed structures through
at least 1,024 cells on each axis, including negative local coordinates.
Interest/query/edit paths operate near both domain extremes and internal voids.

Pass evidence: region facts match source proofs and reference model; no
height/up-axis API or heightmap shortcut participates.

### C05 Sparse lifecycle and pressure

Use the `4096^3` homogeneous domain and working set in
`resources-and-portability.md`. Move/change/withdraw multiple non-camera
interest sources; force presentation then authoritative pressure; retain dirty
scars across retirement and rematerialization.

Pass evidence: required lifecycle transitions occur, live-byte thresholds pass,
pinned work is not evicted, unknown is never empty, pressure decisions are
reported, and restored retired regions contain edits.

### C06 Persistence

Edit a static volume; edit and move a dynamic volume; start a checkpoint;
commit a later mutation while it writes; restore the checkpoint.

Pass evidence: checkpoint names only the earlier cut, later revision remains
dirty, restored IDs/placements/material/revisions equal the cut, derived data
is absent/rebuilt, and byte threshold passes.

Negative variants corrupt every envelope layer, remove material/source/base
snapshot, change lineage only, change Merkle root under same lineage, return a
bad proof, and use unsupported major/contract/layout versions. Every variant
must fail before world publication.

### C07 Dynamic volumes and overlap

Query and collide with a dynamic volume, move it with an admitted placement,
edit in local coordinates, overlap two volumes, checkpoint, and restore.

Pass evidence: identity is stable, placement/matter share the one revision
sequence, local matter is not resampled on move, all overlapping identities
are returned, and no response/motion policy executes.

### C08 Observation gap

Create a capacity-two subscription and commit enough changes to overflow it.
Request a bounded snapshot while another commit occurs, resume with token, and
continue.

Pass evidence: a gap is unavoidable/explicit, no post-gap event leaks before
resume, snapshot revision and next sequence close the race, and consumer
reconstructs the same final facts as direct queries.

### C09 Failures and shutdown

Exercise invalid startup, bounds/domain overflow, cold `RequireReady` query,
source transient/permanent/invalid content, stale revision, budget/queue
exhaustion, presentation failure, persistence failure, extension failure,
receipt expiry, and dirty shutdown.

Pass evidence: error category/scope/retry/effect are correct; no case becomes
empty or partial success; dirty shutdown cannot report stopped. Device-loss
fault injection must fail unpublished work with no committed effect, preserve
already published scar/revision state, and rebuild before readiness.

### C10 Behavior extension

A minimal CPU plug-in observes a change, queries matter, and submits a command.
Removing it removes the effect. A GPU plug-in requests a bounded snapshot,
computes patch proposals without CPU matter readback, and receives ordinary
admission/receipt results. Inject overflow and malformed output.

Pass evidence: material schema has no behavior vocabulary; GPU path accesses
only snapshot/sink; bytes/readback are reported; effects are bounded and
atomic; bad exchanges change nothing except previously admitted commands.

### C11 Presentation and dressing

Render smooth and constructed fixture volumes, material boundaries, edits and
cut faces. Attach dressing and separately create a matter-backed assembly.

Pass evidence: surface border checks are crack-free, status revision is honest,
dressing follows/removes with support, assembly appears in matter/collision,
and dressing does not.

## 4. Portability matrix

Before release:

| Lane | Required evidence |
| --- | --- |
| Linux Vulkan physical | C01–C11 plus performance/sparse thresholds |
| Linux Vulkan software | C01–C10 semantics; timing excluded |
| Windows DX12 physical | C01–C11 semantic and visual parity |
| macOS Metal physical | C01–C11 semantic and visual parity |

Optimized shader variants run parity tests against baseline on every backend
where selected. If a backend lacks an optional feature, baseline must still
pass. One backend passing cannot qualify another.

## 5. Performance procedure

Performance scenarios:

1. pin CPU affinity/power policy when supported and report it;
2. record build profile, adapter, driver, backend, display state, and budgets;
3. perform 30 warm-up iterations;
4. collect 1,000 raw CPU and GPU timestamp samples;
5. report p50/p95/p99/max, queue delay, readback/GPU-copy bytes, pool high
   water, and failures;
6. compare with thresholds in `resources-and-portability.md`; and
7. fail the report if timestamp queries, context, samples, or correctness
   prerequisites are missing.

Optimization claims require a before/after report on identical fixtures and
configuration. Faster presentation cannot compensate for incorrect truth.

## 6. Persistence compatibility fixtures

For every released format minor, commit small golden checkpoint fixtures for:

- empty/homogeneous/mixed base identities;
- static edited volume;
- edited/moved dynamic volume;
- retirement record; and
- unknown optional chunk.

Tests validate supported read/write behavior and byte-stable canonical output.
Corrupt fixtures are generated in tests, not accepted as goldens. A format
major change retains at least the previous major's explicit
`UnsupportedVersion` fixture.

## 7. Human visual evidence

`moria-lab` provides only fixture scenarios, not a game. A reviewer capture
must include an overlay showing scenario ID, adapter/backend, world/volume
revision, presentation status/revision, query completeness, and configuration
digest. Required C03/C11 captures are stored only when a later delivery plan
requests them. A plausible image without matching machine evidence is
`NotDemonstrated`.

## 8. Release gate

The release gate is exactly:

```sh
cargo run -p xtask -- check
cargo run -p moria-conformance -- \
  --suite contract \
  --adapter auto \
  --output target/evidence/contract.json
cargo run -p moria-conformance -- \
  --suite performance \
  --adapter auto \
  --output target/evidence/performance.json
```

CI additionally runs the platform matrix. The local gate may establish only
the local backend. Contract or performance report status other than `Pass`
fails its command.
