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
- material registry accepts exactly 65,535 nonempty definitions plus reserved
  empty ID 0, rejects the 65,536th nonempty definition, and round-trips the
  maximum manifest count without counting empty as registered;
- rigid transform validation and domain half-open semantics;
- stable versus runtime identity, stale generational handles, and counter
  exhaustion;
- every configuration cross-limit and complete error aggregation;
- bounded permit ownership, queue close, byte accounting, and wakeups.
- `Reject` versus `WaitForPermit` behavior for every command, query,
  checkpoint, and extension reserve method, including waiter cancellation.
- exact construction/validation of every query, interest, snapshot, result,
  dressing, and fixed correlation type in the normative facade;
- scheduled behavior descriptor/access maxima, unresolved/cyclic ordering,
  deterministic topological ties, CPU/GPU trait mismatch, aggregate
  per-participant view/collision/handoff/proposal/transaction/conflict-check/
  feedback/dispatch/workgroup cross-limits, and exact effective-config
  telemetry;
- restricted GPU factory surface: an external crate compile-pass adapter can
  create and use only opaque registered buffers/layouts/bind groups/pipelines;
  compile-fail fixtures attempt `RenderDevice`, `wgpu::Device`, raw resource,
  queue, encoder, and submission acquisition through every public method.
  Registry tests prove per-adapter and aggregate live bytes/counts are enforced
  through create/drop/dependency/last GPU use rather than trusted from
  `BehaviorResourceReport`;
- aggregate factory-buffer admission at minimum/default/hard bounds: descriptor
  maxima are checked-summed at registration and against the adapter-clamped
  effective value at startup; multiple adapters fill the pool exactly and one
  byte over returns `BehaviorGpuBufferCapacity`. An instrumented backend proves
  a logical-capacity rejection occurs before renderer allocation, renderer OOM
  releases the permit and registers no handle, dropped buffers remain charged
  through bind-group and submission dependencies, terminal device-generation
  teardown reaches zero, and recreation can then reserve the same bytes.
  `BehaviorGpuBufferBytes` current/high-water/limit/rejected telemetry must
  match every transition;
- two adapters plan disjoint scopes at one commit frontier. Their CPU
  iterators/lookups and GPU bindings contain only their own records, attempts
  to name the other scope fail, and both reports retain the same pinned
  revisions. Include unequal finite positive cell sizes, finite half-open
  domains, and a volume created after adapter registration;
- mixed-processor handoff registration and capacity: CPU-to-GPU upload and
  GPU-to-CPU map/decode borrow carry exact opaque bytes, CPU/CPU and GPU/GPU
  variants preserve order, count/byte/map pressure fails before adapters run,
  uninitialized/duplicate CPU writes and oversized GPU `written_bytes` fail
  the successor policy, and cancellation/device loss releases or quarantines
  every slot;
- behavior tick outcome closure: `AbortTick`, `FailTick`, transition failure,
  per-volume preparation failure, zero-change success, and post-publication
  report panic each produce the exact disposition, `revision_changed`,
  participant publication/notification, and proposal rejection records.
  A post-publication panic never changes an admitted proposal receipt;
- prior GPU feedback double-buffering: first tick is `NoneYet`, the next
  dispatch reads the prior terminal feedback until its submission completes,
  slots never overwrite while pinned, and recovery returns
  `UnavailablePreviousGeneration` rather than old-generation records;
- CPU behavior collision sink: repeated calls, ignored errors, maximum hits,
  one-over output, traversal/call exhaustion, and a callback copying facts into
  its own memory. Moria reuses one exact contact allocation, returns no partial
  overflow result, and its contact/byte high-water never exceeds config;
- behavior effect sink ownership: dense/run inputs are borrowed and copied
  only into pre-reserved slots, over-capacity writes poison the participant,
  create is rejected as nonscheduled control-plane work, and no adapter-owned
  collection/source crosses the callback return;
- behavior planner/adapter callback ownership: access scopes fill the
  Moria-owned exact-capacity sink, errors use only the fixed inline diagnostic,
  and adversarial overlength plans/diagnostics cannot transfer owned storage;
- live-volume reuse versus permanent `volume_records` exhaustion, including
  bounded manifest tombstones;
- extraction, presentation artifact/dirty/instance, dressing registry, and
  extension registration/byte-pool defaults, boundaries, pressure facts,
  effective-config reflection, and telemetry high-water accounting.
- aggregate/per-record material metadata, observation fact/payload, content
  batch count/response bytes, and the reserved per-live-volume presentation
  marker partition at minimum/default/maximum cross-limits;
- content scheduling with callback-count capacity still free but response-byte
  capacity exhausted: the next callback is not invoked, holds no count slot,
  emits deferred pressure, and begins only after the first permit-backed sink
  is installed or dropped and its byte permit is released;
- content callbacks receive only a Moria-owned exact-length output sink:
  concurrently invoke one-brick callbacks that attempt a second result and
  detailed writes sourced from consumer buffers sized for
  `content_bricks_per_request`; the extra write poisons the batch, the detailed
  API borrows exactly 512 samples rather than accepting ownership, incomplete
  and ignored-write-error callbacks fail, and process-visible Moria-owned sink
  high-water never exceeds `content_response_bytes`;
- content-port ownership adversaries: a source whose borrowed descriptor owns
  an oversized lineage allocation is rejected at registration without
  transferring it; accepted 256-byte lineage is copied into exact canonical
  ownership; 192-byte source diagnostics round-trip, 193-byte construction is
  rejected before callback return, and concurrent source failures can return
  only the fixed inline error record. Instrumented allocators assert that
  source-owned descriptor/temporary allocations never enter the Moria response
  charge and that all Moria-owned sink/error high-water remains within
  `content_response_bytes`;
- volume debug names accept exactly 96 UTF-8 bytes and reject 97 bytes through
  both `register_volume` and runtime `VolumeCommand::Create`; accepted names
  with oversized input `String` capacity retain exact boxed bytes only;
- collision traversal authorization at 8,192 bricks/65,536 cells, rejection
  one above either bound, and hit overflow with both partial policies;
- deterministic startup cause aggregation and every `RegionFailureKind`;
- snapshot `All` membership and accepted resolved-scope encoding.

### State-machine tests

Use `MinimalPlugins` plus only the Moria control-plane plugin/test driver.
Explicit `app.update()` steps inject worker/GPU milestone completions.

- every legal world/region/presentation/checkpoint transition;
- invalid transition rejection;
- interest cloning/update/withdrawal and retirement pins;
- receipt cancellation winning immediately before the atomic
  `WaitingForMatter -> Preparing` transition, losing immediately after it,
  `TooLate` at every later stage, and noncancellable startup/shutdown receipts;
- per-volume FIFO and independent-volume concurrency;
- behavior command-frontier capture, one pinned view for every participant,
  isolated per-participant exports, direct CPU callback without query
  admission, post-frontier command blocking, CPU-to-GPU and GPU-to-CPU handoff
  milestones, whole-proposal reject/replace/fail
  composition, conservative sparse-patch AABB conflicts, bounded conflict-
  check exhaustion before publication, typed tick-abort discard outcomes,
  post-publication notification failure, prior-feedback rotation, per-volume
  tick atomicity, and independent-volume failure;
- query pending/materialize/minimum/exact revision behavior;
- observation cursor filtering after several dynamic moves and reclamation of
  their old directory versions, proving retained envelopes still apply
  revision-time old-or-new world bounds; overwrite gap and snapshot/resume race;
- interest/subscription `All` acceptance followed by create, retire, and
  dynamic move into/out of world bounds; prove pinned IDs/bricks never expand,
  subscription move predicates use old-or-new bounds, and update/resubscribe is
  the only membership refresh;
- retirement after subscription followed by overwrite of the retirement fact;
  the gap snapshot still returns exactly one typed retired member with its
  accepted historical ID, stable key, and terminal revision, and never silently
  omits or replaces it;
- nonadvancing GPU delta capture after several moves uses the same retained
  envelopes and leaves the CPU cursor byte-for-byte unchanged; maximum-record
  paging returns `MoreAvailable` with an exact continuation cursor, overwrite
  returns a zero-record `NeedsSnapshot`, and a matching checkpoint fact returns
  a zero-record `UnsupportedFact` at its sequence without skipping later facts;
  `SubscriptionState` snapshot recovery restarts after its frontier's optional
  head while never calling `resume_after`;
- before the first observation, `CurrentHead` subscription acceptance records
  `initial_after = None`, explicit and subscription-state snapshots return an
  `Empty` frontier, and a GPU delta read from `after = None` returns zero-record
  `Complete` with a `None` cursor; appending sequence one then makes the same
  cursor read that first fact rather than reporting overwrite;
- exact 27-key local and 13,824-key dispersed presentation invalidation,
  bounded 1,024-job draining, dirty-record coalescing fallback, fair eventual
  scheduling, superseded-target replacement, and simultaneous dirty commits in
  every live volume with each reserved fallback marker occupied;
- shutdown drain/`CancelNotPrepared`/checkpoint reports, including an
  operation already in `Preparing`;
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
- every Extension ABI v1 host/WGSL offset and size;
- malformed candidate kind/reserved word/offset/alignment/state ID, missing
  exact revision precondition, oversized WGSL/entry point/registry, and
  malformed candidate effect layout;
- observation-delta frontier ABI parity: both oldest/head words are zero only
  for `Empty`, both are nonzero for `Retained`, one-zero/one-nonzero is rejected,
  and pre-sequence-1 `Complete` has zero records and a zero cursor;
- every Scheduled ABI v1 host/WGSL size, alignment, offset, and stride:
  all four 64-byte headers, the 112-byte volume, 24-byte cell, 128-byte
  proposal, 20-byte patch run, 32-byte handoff descriptor, 64-byte feedback
  participant, and 48-byte feedback proposal;
- every logical 64-bit scheduled field is declared as adjacent low/high
  `u32` words at the documented offsets. Golden values
  `0`, `1`, `0x00000001_00000000`, and `u64::MAX` assert host pack/unpack,
  WGSL reconstruction/comparison, both-word zero tests, and little-endian
  bytes. No Scheduled ABI WGSL declaration or shader source contains a
  concrete `u64` scalar;
- scheduled negative fixtures at the expected validation layer for bad
  magic/version/direction/availability/status/failure tags, nonzero
  reserved/undefined-flags words, a zero pair where a nonzero value is
  required, or a decoder incorrectly treating a one-zero pair as absent,
  undersized effective binding ranges, nonmonotonic/overlapping offsets,
  record-count or byte arithmetic overflow, proposal/payload/handoff
  over-capacity, invalid snapshot/volume indices, changed revisions, invalid
  cell size/domain, cross-participant record access, stale device generation,
  malformed prior feedback, and old-generation handoff/feedback reuse.
- golden feedback fixtures map participant abort, two-party conflict fail-tick,
  transition failure with predecessor/successor/stage, preparation failure,
  device loss with a two-word generation, and
  published-with-notification-failure with exact failed-hook count. Each
  fixture round-trips to the corresponding Rust disposition, cause,
  participant publication/notification, proposal outcome, flags, and unused
  zero fields; changing any required/unused field fails at feedback validation.

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
   (`1e-5` local cell units for contact point/TOI), conservative candidate
   accounting at both fixed work maxima, and no hit truncation under either
   partial policy.
6. **Readback lifecycle:** multiple in-flight maps, cancellation, decode error,
   and staging reuse without mapping/submission overlap.
7. **Eviction/compaction:** edit, checkpoint, evict, rematerialize, compact,
   and compare exact public facts.
8. **Persistence:** checkpoint through real GPU readback, restore into a new
   renderer generation, and compare samples/collision.
9. **Presentation provenance:** current/stale/failure/overflow install behavior,
   halo seams, and discard of obsolete output.
10. **Scheduled behavior:** run a CPU adapter and two renderer-owned GPU
    adapters against the same pinned revisions but disjoint exported scopes;
    prove declared DAG order, direct CPU view delivery, unequal cell-size/domain
    interpretation including a post-registration volume, GPU view/effect
    publication with zero authority-path readback, CPU-to-GPU upload and
    GPU-to-CPU mapped handoff, fixed proposal validation, whole-proposal
    conflict/tick-abort outcomes, prior-feedback reconciliation,
    old-generation feedback quarantine,
    exact low/high-word integer readback parity for every scheduled
    header/record, all six terminal-feedback golden outcomes,
    restricted-factory per-adapter and aggregate buffer-byte enforcement,
    drop-after-last-use release, terminal-generation teardown/recreation, and
    logical OOM rejection before a backend allocation,
    one revision per affected volume, and post-frontier command exclusion.
    Induce participant failure, rejected effects, checkpoint/restore, shutdown,
    and device loss; prove Moria never owns adapter state, late old-generation
    work cannot publish, CPU state is untouched, and GPU adapters must recreate
    and report ready.
11. **Asynchronous GPU extension:** snapshot packet remains GPU-oriented; the
    worst-case effect batch is reserved before dispatch; fewer effects release
    unused capacity; every valid child receipt is returned; invalid/overflow
    output admits none; ABI v1 samples/occupancy/lifecycle/delta records, inline/
    previous opaque state, fixed diagnostics, and Fill/Patch/Move candidate
    layouts execute through the public path; retained filtered delta records
    use their 128-byte tagged layout; an overwritten sequence, a matching
    unsupported checkpoint fact, and a maximum-record page produce distinct
    header/public statuses; before any fact is appended, an
    `after = None` request and subscription-state snapshot return the exact
    empty frontier (zero oldest/head/cursor words), then sequence one is read
    from the same cursor; blocked statuses produce no effects; recovery through
    a bounded subscription-state snapshot does not advance the CPU cursor; and
    already admitted children have independent applied/conflict/failure
    outcomes.
12. **Device loss:** intentionally destroy/lose the device while operations are
    pending/submitted; every receipt terminates, late callbacks are ignored,
    every prior extension-state lease becomes stale, durable material state
    reconstructs, and volatile dirty material state fails honestly.

Exact parity is required for samples, material IDs, coordinates, occupancy
masks, revisions, indices, and checkpoint bytes. Only derived floating-point
positions/normals use stated tolerance.

## Public contract scenarios

`contract_harness --scenario all` runs deterministic scenarios and writes
`target/evidence/contracts.json`:

### C1. Public boundary

Create a builder with an explicit world key; register material/static/dynamic
volumes, a dressing style, a consumer base source, and ordered CPU/GPU
behavior adapters; consume
`ValidatedMoria` into its plugin, handles, and startup receipt; start, interest,
construct every query variant, inspect, mutate, observe,
checkpoint, restore/import, inspect the material registry, request a behavior
tick, run an asynchronous WGSL job, and shut down through
the callable methods in [public-api.md](public-api.md). Build the
example as if it were an external crate: it imports only `moria` exports.
Collision variants supply explicit traversal authorization; startup failure
uses the typed staged cause shape.

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

Before any fact exists, subscribe at `CurrentHead`, take explicit-region and
subscription-state snapshots, and require their frontier plus a GPU delta page
from `after = None` to report valid `Empty`, not a gap; after the first fact,
that same delta cursor observes sequence one. Then stall a bounded subscriber
past ring capacity, observe an explicit gap, take a
bounded snapshot, resume at its head, and receive later facts without claiming
the missing sequence. Repeat with `All`, then create, retire, and move volumes:
the accepted membership remains pinned and identical in the gap/snapshot; a
fresh subscription is required to include the new volume. Force several moves,
reclaim their old directory versions, and poll retained facts afterward;
revision-time world filtering must still use the retained old/new envelopes.
Then overwrite a pinned member's retirement fact and require the gap snapshot
to return its typed retired record with stable key and terminal revision.

### C8. Failure matrix

Exercise every approved failure: cold query, bad bounds, stale precondition,
temporary/invalid content, pressure, presentation failure, gap, persistence
failure, material mismatch, lineage mismatch, fingerprint mismatch, shutdown
with dirty state, and device loss where the host supports it. Presentation
failures include unresolved dressing material filters at validation, missing
surface/triplanar/dressing assets at runtime, instance overflow, and dirty/job
pressure without loss of the eventual-current obligation.
It also verifies renderer absence and a deliberately under-capable adapter
produce distinct staged startup causes, region failures preserve cause plus
retryability, observation byte pressure gaps rather than losing silently, and
simultaneous dirty commits in all live volumes retain one fallback marker each.

### C9. Scheduled behavior engines

Implement four external-style adversarial adapters:

1. a conventional CPU physics-shaped adapter owning bodies, velocities,
   forces, joints, solver state, and policy;
2. a GPU-resident physics-shaped adapter owning equivalent state in its own
   renderer-device buffers; and
3. a CPU damage-and-bond-shaped adapter owning impacts, accumulation, bond
   strength, breakage, crumbling, and fracture rules; and
4. a GPU-resident variant owning the same vocabulary in its own buffers.

These names describe harness-owned state only and must not appear in Moria
types. Each adapter plans bounded access at the same pinned commit frontier,
receives only its own filtered collision/material records, updates its own
state, and proposes only generic matter, move, or retire effects. Use two
disjoint scopes to prove neither CPU nor GPU can inspect the other's records
while their reports cite the same pinned revisions. Include unequal cell sizes
and a post-registration created volume. The CPU adapter must participate
without submitting/polling a query.
The GPU adapters must validate, compose, prepare, and publish without material,
solver-state, or candidate-effect CPU readback on the authority path.

Declare physics before damage through `runs_after`, then reverse the legal
order and prove Moria has no hard-coded phase. Transfer opaque impact stimuli
through a declared CPU-to-GPU edge and a declared GPU-to-CPU edge, verifying
upload/map lifetimes and bounds; do not smuggle raw harness buffer handles
through the seam. Exercise every conflict policy with
overlapping and nonoverlapping proposals and prove no partial proposal, same-
view leakage, or revision interleaving. Force rejection after adapter state
changes and prove the borrowed CPU report and next-tick read-only GPU feedback
let the adapter reconcile while Moria neither rolls state back nor checkpoints
it. Force another participant's `AbortTick`, a `FailTick` conflict, and a
post-publication CPU report panic; assert the exact tick/participant/proposal
outcomes and `revision_changed` in both the Rust report and next-tick GPU
feedback. The GPU assertion includes both engine/proposal pairs for
`FailTick`, predecessor/successor/stage for transition failure, the device
generation pair, preparation failure, disposition, defined flags, and exact
failed-hook count after publication. Repeat across checkpoint, restore
readiness, device loss/recovery, and shutdown.

The independent reviewer must compile an external-style adapter that attempts
every public route to obtain `RenderDevice`, `wgpu::Device`, a raw buffer/
pipeline/bind group, raw encoder, queue, or submission; import an externally
created resource into the behavior encoder; bind another participant's export;
bypass proposal validation; exceed access/collision/handoff/proposal/dispatch/
workgroup bounds; or introduce behavior-specific fields. Any success fails C9.

### C10. Asynchronous WGSL inspection/effect jobs

A minimal external descriptor observes bounded matter and requests one patch or
move. Its opaque state and reason remain outside Moria. CPU and GPU-oriented
variants produce the same public command/revision semantics; only the latter
can claim GPU handoff evidence. The GPU variant registers through the bounded
extension registry, uses each closed inspection variant, chains one opaque
state ID, decodes fixed diagnostics, and emits the exact ABI v1 Fill,
Patch-runs, and Move records with mandatory captured revisions. For
`ObservationDeltas`, it filters retained matter/move facts after old directory
versions are reclaimed, pages at `maximum_records`, and proves the CPU
subscriber cursor is unchanged. It first captures a newly started world and
proves the public/ABI frontier is empty, then reads sequence one from
`after = None`. It then overwrites the requested sequence and observes
`NeedsSnapshot`, matches a checkpoint fact and observes
`UnsupportedFact`, produces no candidate effects in either blocked state,
reconciles via `SnapshotScope::SubscriptionState`, and restarts after the
snapshot frontier's optional head without silent loss. Empty `Complete`,
`MoreAvailable`, `NeedsSnapshot`, and `UnsupportedFact` must be distinct in
both the shader header and the public outcome.

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
- scheduled behavior planning/export/CPU-map/GPU-dispatch/composition/
  publication latency, processor-transition count, view/proposal/feedback
  bytes, handoff upload/map bytes, and restricted-factory GPU bytes.

GPU timestamp queries are used only when supported. CPU wall time remains
recorded. Every run states warm-up count, sample count, workload dimensions,
density, mutation distribution, in-flight depth, adapter/driver, build profile,
and fallback status. Synchronous readback is never inserted into a production
hot path merely to time it.

### Architecture feasibility gates

The selected GPU hash/MVCC, bounded readback, collision, scheduled behavior,
asynchronous extension, checkpoint,
and dual-contouring architecture is not implementation-ready on measurement
alone. The following v1 gates are falsifiable. They are minimum feasibility
floors for the physical qualification adapter in **each** claimed backend
family, not customer frame-time promises. Software adapters cannot pass them.

Runs use the default effective limits, enabling persistence, behavior hooks,
or GPU extensions only for the gates that exercise them, an optimized
non-debug build, uncapped
Bevy updates, 10 warm-up iterations, at least 100 measured operations (or 10
complete passes for throughput routes), and three process runs. The reported
p95 is computed over all post-warm-up samples; every process run must meet the
bound. GPU time uses timestamps where available; otherwise a queue-completion
wall interval is reported and must meet the more lenient end-to-end bound.
Correctness for the same workload must pass first.

| Gate | Fixed workload and pressure | Required pass |
| --- | --- | --- |
| P1 sparse residency | C5 domain has at least 4 GiB dense sample size (64× the 32,768-brick detail pool); move 4,096-brick interest through 16 disjoint regions, with 4,096 dirty bricks and current presentation in the active region | No capacity exceeds effective config; authoritative device bytes are <=160 MiB at defaults; host-owned material payload/staging is <=96 MiB; after two full route cycles neither grows by >1%; no full-domain CPU or GPU allocation |
| P2 mutation publication | 8 in flight across 4 volumes; each command changes 32,768 cells spanning 512 mixed bricks; 50% patch and 50% fill; presentation interested but presentation completion is measured only by P6 | p95 admission-to-commit <=50 ms, GPU prepare/validate/publish <=12 ms, and >=20 committed commands/s sustained; every revision/atomicity check passes |
| P3 bounded region readback | 4 in-flight 262,144-cell region queries (1 MiB decoded samples each), detailed/mixed data, hot matter | p95 submit-to-decoded result <=50 ms, GPU query <=12 ms when timestamps exist, >=80 MiB/s decoded aggregate, staging stays within config |
| P4 collision traversal | 32 in-flight sweeps/traces, each authorizing 65,536 candidate cells and 256 hits across static plus rotated dynamic volumes | p95 submit-to-decoded facts <=33 ms and GPU traversal <=10 ms when timestamps exist; >=1,000 queries/s aggregate for 4,096-candidate-cell zero/one-hit control workload |
| P5 materialization | Precomputed in-memory source, 8,192 detailed mixed bricks (16 MiB), no scar, 512 bricks per source callback, four of the resulting 16 batches in flight | cold-to-ready throughput >=64 MiB/s and p95 per 4,096-brick interest <=300 ms; extraction never exceeds its byte/count limit |
| P6 presentation rebuild | **P6a local:** one sparse patch changes the eight corner cells of one 8³ brick. The union of their one-cell halo dependencies is exactly the 3×3×3 neighborhood (27 artifacts); each emits 1,024–2,048 vertices and 6,000–12,288 indices. **P6b legal-command scale:** four nonoverlapping presentation interests of at most the default 4,096 bricks cover 13,824 artifacts; one 32,768-cell mutation touches 512 pairwise halo-disjoint bricks and therefore invalidates exactly those 13,824 artifact keys. Each emits 96–128 vertices and <=768 indices, no dressing, so all outputs fit default mesh pools. Start with all artifacts current, use default 1,024 jobs, 16,384 artifact/dirty records, and sample queue/dirty/current counts every update. | **P6a:** p95 commit-to-all-current <=250 ms and GPU derivation total <=20 ms when timestamps exist. **P6b:** first newly current artifact <=250 ms; all 13,824 are current at the command revision <=2 s; submitted jobs never exceed 1,024; exact dirty high-water <=13,824 and allocated artifact/dirty records stay <=16,384; current count increases in every 250 ms interval after dispatch begins; no starvation, coalescing, overflow, eviction of an interested target, crack, or provenance failure. |
| P7 scheduled GPU behavior | Two ordered GPU adapters have disjoint filtered exports at one pinned frontier. Each export contains exactly 128 volume records and 256 full 8³ bricks = 131,072 cell records: `64 + 128×112 + 131,072×24 = 3,160,128` logical initialized bytes; aggregate logical export is exactly 6,320,256 bytes and the configured `behavior_gpu_view_bytes` allocation capacity remains 32 MiB. Together they propose exactly 1,024 valid effects with payload totaling <=1 MiB across four volumes; each declares and owns exactly 32 MiB of factory-registered harness state, so the checked descriptor sum and live charge are exactly 64 MiB against the default effective `behavior_gpu_buffer_bytes = 256 MiB`; no CPU adapter participates. | p95 stable-view-export-to-publication <=33 ms, adapter+validation+composition+publication GPU time <=16 ms when timestamps exist, time/export receipts report exactly 6,320,256 initialized/copied view bytes rather than allocation capacity, zero material/proposal readback before publication, bounded prior feedback only after publication, `BehaviorGpuBufferBytes` reports exactly 64 MiB live and no rejection, and every pool/factory-registry byte remains within declared limits |
| P8 checkpoint path | 8,192 dirty detailed scars (16 MiB raw), checkpoint concurrent with four mutation streams; in-memory durable test store so storage hardware is excluded | GPU-readback-plus-encode throughput >=64 MiB/s, mutation P2 p95 degrades by <=2×, staged bytes stay within config, and semantic restore parity passes |
| P9 asynchronous WGSL job | 8 MiB inspection packet, 256 structurally valid candidate effects whose 32,768 record bytes plus payload total <=65,472 bytes, 2 extension jobs in flight; effects touch four volumes | p95 packet-capture-to-all-child-admitted <=50 ms, extension GPU work <=16 ms when timestamps exist, candidate + 64-byte diagnostic readback <=64 KiB/job, and zero inspection-packet/material readback to CPU |

These floors deliberately span latency, throughput, memory, density, and
in-flight pressure. Qualification receipts may also publish stronger
platform-specific targets, but may not weaken these common floors without a
reviewed TDD decision and evidence that the approved GPU-residency performance
direction remains viable.

Failure is blocking and scoped:

- P1 failure rejects the bounded sparse-residency claim and requires pool/page
  layout revision.
- P2 or P3 failure rejects the selected MVCC/readback hot path.
- P4 failure rejects the collision traversal selection.
- P5 failure rejects the materialization batching selection.
- Either P6 sub-workload failing rejects dual contouring, invalidation
  tracking, queue granularity, or fair-drain scheduling as the v1 presentation
  baseline. P6a alone cannot qualify maximum-command presentation viability.
- P7 failure rejects the scheduled renderer-owned GPU behavior path.
- P8 failure rejects the selected checkpoint readback/encoding pipeline.
- P9 failure rejects the asynchronous copied-packet WGSL facility, not the
  scheduled behavior hook.

The affected architectural claim remains `fail`, not “report-only,” and the TDD
implementation cannot be called contract-complete until it is revised or the
gate passes on every claimed backend family. Performance never excuses a
correctness failure, and a correctness pass never substitutes for these
feasibility receipts.

## Portability qualification

At least one physical adapter for each claimed Linux/Vulkan, macOS/Metal, and
Windows/DX12 family runs:

- all real-GPU correctness tests;
- contract scenarios C1–C10 where platform capabilities allow loss injection;
- shader validation;
- a fixed report-only workload;
- architecture feasibility gates P1–P9;
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
- a P1–P9 pass missing its workload scale, in-flight pressure, percentile,
  throughput/memory result, or physical-adapter context.
- a P6 pass missing both local and 512-brick dispersed receipts, per-update
  dirty/job/current series, exact 13,824 invalidations, or the final
  command-revision equality for every interested artifact.

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
