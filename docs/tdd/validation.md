# Validation and Evidence Plan

## Principles

Validation is layered. Host-only tests prove deterministic contracts; real-GPU
tests prove the production compute/readback path; diagnostic rendering proves
derived presentation; physical-adapter receipts qualify portability and
performance. No layer claims evidence it did not execute.

All harnesses are ordinary crate consumers. `tests/support` may provide content
fixtures, clocks, and fault triggers through public `test-support` hooks, but it
cannot read storage or mutate truth directly.

The contract harness writes a versioned JSON report. A required check with
missing evidence is `not_demonstrated`, and the process exits nonzero. A
rendered frame, shader compilation, software adapter, or host oracle alone
cannot make GPU correctness pass.

## Automated host suite

The ordinary `cargo test --all-targets --all-features` suite runs without a
window or physical GPU and includes:

### Value and configuration tests

- negative-coordinate Euclidean brick mapping and overflow boundaries;
- material empty/coverage/flag invariants and host/WGSL byte layout;
- rigid transform validation and domain half-open semantics;
- stable versus runtime identity, stale generational handles, and counter
  exhaustion;
- every configuration cross-limit and complete error aggregation;
- bounded permit ownership, queue close, byte accounting, and wakeups.

### State-machine tests

Use `MinimalPlugins` plus only the Moria control-plane plugin/test driver.
Explicit `app.update()` steps inject worker/GPU milestone completions.

- every legal world/region/presentation/checkpoint transition;
- invalid transition rejection;
- interest cloning/update/withdrawal and retirement pins;
- receipt cancellation before versus after submission;
- per-volume FIFO and independent-volume concurrency;
- query pending/materialize/minimum/exact revision behavior;
- observation cursor filtering, overwrite gap, snapshot/resume race;
- shutdown drain/cancel/checkpoint reports;
- device-generation callback quarantine and recovery/terminal branches.

### CPU oracle and generated sequences

An independent dense oracle is limited to small finite volumes. It implements
material cells, rigid placement, queries, and occupied-cell collision without
calling storage or GPU modules.

Property/state-machine sequences generate registrations, interest changes,
fills, patches, moves, queries, checkpoint frontiers, eviction, and restore.
After every successful command:

- oracle and public query samples agree exactly;
- revision increases once and only once;
- failed commands preserve every sample and revision;
- no collision result targets empty/dead/retired volume matter;
- all query results identify retained revisions;
- scar/restored semantic state agrees.

Generated bounds stay small enough to enumerate, while invalid generation
deliberately covers overflow and caps.

### Shader validation

Naga/API validation runs for every WGSL module. Layout tests compare Rust
offsets/sizes/constants with shader declarations. Negative fixtures cover:

- invalid binding type/count or out-of-range group;
- descriptor count/range larger than the validated bound buffer;
- invalid workgroup/output capacity;
- page/slot offset overflow;
- bad transaction sentinel path;
- extension attempts to bind internal storage;
- malformed candidate effect layout.

Passing compilation is only validation evidence, not execution correctness.

### Persistence codec tests

Run every requirement in [persistence.md](persistence.md), fuzz decoder inputs
under bounded allocation, and keep checked-in v1 golden fixtures.

## Real-GPU correctness suite

`tests/gpu.rs` is ignored in ordinary environments and required on qualifying
hosts. It constructs a normal Bevy renderer path, never a second device or a
mock/no-op backend.

Every test records Bevy/wgpu/Moria versions, OS/architecture, backend, adapter,
driver, device type, features, limits, downlevel flags, software-fallback
status, and device generation.

Required tests:

1. **Base parity:** upload implicit, homogeneous, and detailed bricks and query
   them through normal staging readback against the CPU oracle.
2. **Atomic success:** mutate multiple cells/bricks while queries are queued
   before and after publication; each sees entirely old or entirely new state.
3. **Forced post-admission failure:** inject failure after copy-on-write
   preparation but before revision publication; readback, collision, revision,
   scar, and observations remain unchanged.
4. **Pressure:** exhaust every pool/queue with pins and prove bounded rejection,
   then release and recover capacity.
5. **Collision parity:** all shape/trace/sweep cases, overlapping and rotated
   dynamic volumes, exact integer IDs/revisions and documented float tolerance
   (`1e-5` local cell units for contact point/TOI).
6. **Readback lifecycle:** multiple in-flight maps, cancellation, decode error,
   and staging reuse without mapping/submission overlap.
7. **Eviction/compaction:** edit, checkpoint, evict, rematerialize, compact,
   and compare exact public facts.
8. **Persistence:** checkpoint through real GPU readback, restore into a new
   renderer generation, and compare samples/collision.
9. **Presentation provenance:** current/stale/failure/overflow install behavior,
   halo seams, and discard of obsolete output.
10. **GPU extension:** snapshot packet remains GPU-oriented, valid candidates
    use normal receipts, invalid/overflow output commits none, and stale
    preconditions conflict.
11. **Device loss:** intentionally destroy/lose the device while operations are
    pending/submitted; every receipt terminates, late callbacks are ignored,
    durable state reconstructs, and volatile dirty state fails honestly.

Exact parity is required for samples, material IDs, coordinates, occupancy
masks, revisions, indices, and checkpoint bytes. Only derived floating-point
positions/normals use stated tolerance.

## Public contract scenarios

`contract_harness --scenario all` runs deterministic scenarios and writes
`target/evidence/contracts.json`:

### C1. Public boundary

Register material/static/dynamic volumes and a consumer base source; start,
interest, inspect, mutate, observe, checkpoint, restore, and shut down through
public APIs. Build the example as if it were an external crate: it imports only
`moria` exports.

### C2. Truth versus view

For the same matter revision, capture sample/occupancy/collision results with
presentation absent, current, deliberately stale, failed by injected output
overflow, discarded, and rebuilt. Facts must match exactly.

### C3. Mutation honesty

Remove across an organic/constructed material boundary, patch deep internal
matter, place matter, and force a multi-brick failure. Record receipt stages,
one-step revisions, observations, collision results, cut-surface current
revision, and dressing replacement.

### C4. Deep volume

A harness-owned source returns voids, material bands, and structure throughout
all axes including negative coordinates. Query/edit reachable scopes at
multiple depths. The report includes domain dimensions and non-heightmap
occupancy variation; the generator is test content, not library code.

### C5. Sparse lifecycle

Use a finite domain whose dense bytes exceed the configured detail pool by at
least 64×. Move non-camera interest among disjoint regions, retain a dirty scar
through retirement, and prove detail/page/mesh/staging high-water marks remain
within config.

### C6. Dynamic volume

Query/collide, move, edit local matter, checkpoint, restore, and query/collide
again. Stable identity and exact placement/revision context must match.

### C7. Observation gap

Stall a bounded subscriber past ring capacity, observe an explicit gap, take a
bounded snapshot, resume at its head, and receive later facts without claiming
the missing sequence.

### C8. Failure matrix

Exercise every approved failure: cold query, bad bounds, stale precondition,
temporary/invalid content, pressure, presentation failure, gap, persistence
failure, material mismatch, lineage mismatch, fingerprint mismatch, shutdown
with dirty state, and device loss where the host supports it.

### C9. Behavior extension

A minimal external descriptor observes bounded matter and requests one patch or
move. Its opaque state and reason remain outside Moria. CPU and GPU-oriented
variants produce the same public command/revision semantics; only the latter
can claim GPU handoff evidence.

## Visual evidence

`visual_harness` is an adjacent diagnostic consumer, not a game and not a
completion requirement by itself. It uses public APIs and a harness-owned
content source to display:

- smooth organic and sharp constructed volumes together;
- deep internal void exposed by an edit;
- placed material and honest cut faces;
- a moved dynamic volume;
- revision/status overlay and stale-view toggle;
- revision-anchored dressing removal/regeneration;
- raw occupied-cell and volume-bound diagnostic modes.

A capture receipt records commit revision, presentation current revision,
adapter context, config, camera transform, and artifact hash. Human review
answers only:

1. Are organic surfaces coherent for the fixture?
2. Are constructed edges acceptably crisp?
3. Are edited/placed surfaces honest with no persistent cracks?
4. Does dressing visibly remain supported by matter?

The decision and reviewer identity/date live alongside the captured evidence.
A human rejection is not overridden by automated correctness.

## Performance methodology

Correctness scenarios must pass before performance is interpreted. Criterion
benchmarks and GPU harnesses record separately:

- command admission CPU time;
- admission-to-submit, submit-to-commit, and commit-to-current-presentation;
- query submission, GPU execution, map/readback, and decode;
- collision query latency by candidate cells/hits;
- materialization and retirement throughput;
- authoritative, scar, derived, staging, and peak allocated bytes;
- checkpoint GPU readback, encoding, store write, and restore;
- extension packet/effect GPU bytes and compact CPU readback bytes.

GPU timestamp queries are used only when supported. CPU wall time remains
recorded. Every run states warm-up count, sample count, workload dimensions,
density, mutation distribution, in-flight depth, adapter/driver, build profile,
and fallback status. Synchronous readback is never inserted into a production
hot path merely to time it.

Initial performance work uses report-only baselines. A threshold becomes a
release gate only through a later reviewed validation-plan change containing
physical receipts. One machine's result is not a universal product promise.

## Portability qualification

At least one physical adapter for each claimed Linux/Vulkan, macOS/Metal, and
Windows/DX12 family runs:

- all real-GPU correctness tests;
- contract scenarios C1–C9 where platform capabilities allow loss injection;
- shader validation;
- a fixed report-only workload;
- renderer recovery/device-loss test or an explicit `not_demonstrated` if the
  platform cannot inject it.

A backend family is not first-class until its receipt is checked in under
`evidence/qualification/<date>-<os>-<backend>.json` with a passing schema.
Software/lavapipe/SwiftShader runs are labeled and cannot substitute.

## Evidence schema and gates

Each claim record contains:

```text
claim_id
status = pass | fail | not_demonstrated
scenario_version
source_commit
config_digest
toolchain + dependency versions
machine + adapter context
started_at + duration
measurements
artifact digests
failure category/message
```

Cross-field validation rejects:

- `pass` with missing required artifact/measurement;
- performance claim from a software adapter;
- mutation visual pass whose presentation revision is behind commit revision;
- persistence pass whose durable frontier differs from requested frontier;
- GPU claim from host oracle/mock/validation-only execution;
- collision pass based only on mesh/render evidence;
- device recovery pass without terminal outcomes for all outstanding receipts.

The ordinary merge gate is the exact command list in
[overview.md](overview.md). GPU qualification and visual review are release
evidence gates and may run on dedicated machines, but missing required receipts
leave the affected claim `not_demonstrated`.

## Test authorship and change discipline

Implementation commits must not weaken a failing acceptance assertion to make
code pass. Changes to public invariants, evidence schema, tolerances, golden
fixtures, or required scenarios receive independent review and state which
approved design outcome remains equivalent.

Fault injection hooks can fail content load, slot reservation, transaction
validation, map/decode, presentation output, store chunk/manifest, observation
retention, and device generation. Hooks are deterministic counters keyed by
operation ID and use the production stage; they cannot directly edit a
revision, sample, or receipt.
