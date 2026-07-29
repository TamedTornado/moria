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
  per-participant input record/host byte/GPU ingress byte/view/collision/
  handoff/proposal/transaction/conflict-check/feedback/dispatch/workgroup
  cross-limits, and exact effective-config telemetry;
- behavior tick permits atomically reserve every descriptor's maximum input
  records, host bytes, GPU header/payload/staging/device bytes, and all later
  tick resources before a planner runs. Exact-bound success and one-record/
  one-byte pressure failures must invoke zero planners/adapters until one
  complete permit is available;
- consumer-input structural closure: unknown/stale participant, duplicate
  record, input supplied to `None`, missing `Required` (including distinction
  from a present empty slice), per-participant overflow, and aggregate overflow
  each return the unchanged request with its specific `ViolationCode`, assign
  no tick ID, and invoke no planner, upload, or adapter. Cancellation releases
  every accepted input charge before returning
  `CancelledBeforePreparation`;
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
  tick-global input-preflight failure,
  per-volume preparation failure, zero-change success, and post-publication
  report panic each produce the exact disposition, `revision_changed`,
  participant publication/notification, and proposal rejection records.
  For a two-participant upload failure, the addressed participant is
  `Skipped(ConsumerInputUpload)`, the peer is
  `NotRun(InputPreflightAborted { failed_engine })`, both publications are
  discarded, both notifications are not applicable, all snapshot/proposal/
  published collections are empty, and no report hook runs.
  Include a mixed independent-volume case where participant A's only selected
  volume fails preparation while participant B's volume publishes: A is
  `Published { revision_changed: false }`, tick-wide `revision_changed` and B's
  participant value are true, and the two feedback flag bits differ exactly.
  A post-publication panic never changes an admitted proposal receipt;
- prior GPU feedback double-buffering: first tick is `NoneYet`, the next
  dispatch reads the prior terminal feedback until its submission completes,
  slots never overwrite while pinned, and recovery returns
  `UnavailablePreviousGeneration` rather than old-generation records. Assert
  the byte formula contains no snapshot records and that the adapter can
  reconcile every outcome by retaining its own prior
  proposal-index-to-snapshot mapping;
- CPU behavior collision sink: repeated calls, ignored errors, maximum hits,
  one-over output, traversal/call exhaustion, and a callback copying facts into
  its own memory. Moria reuses one exact contact allocation, returns no partial
  overflow result, and its contact/byte high-water never exceeds config;
- behavior effect sink ownership: dense/run inputs are borrowed and copied
  only into pre-reserved slots, over-capacity writes poison the participant,
  arbitrary create is rejected as nonscheduled control-plane work, no
  `BaseContentSource` crosses the callback return, and only the closed
  source-bound `ExtractComponents` operation can split existing matter. A
  later ordinary create has its own receipt and is excluded from the prior
  tick's published revisions;
- placement-stream cardinality and reservation: zero placement maximum rejects
  kind 5, one kind-5 proposal at the declared update/byte maximum reserves
  exactly one root transaction and all update/entry/authority/observation/
  outcome/receipt/cleanup records, and a second kind-5 record invalidates the
  complete participant batch without consuming a second root or admitting any
  proposal;
- component candidate-key preflight derives complete tables for salts
  `0..=255` in order. A collision-injection seam forces all 256 complete sets
  to collide and asserts synchronous
  `ComponentIdentityExhausted`, unchanged request, no public tick ID,
  planner, adapter, or partial table, and complete release of every tentative
  runtime ID, key, lifetime/live record, byte permit, and tick resource;
- behavior planner/adapter callback ownership: access scopes fill the
  Moria-owned exact-capacity sink, errors use only the fixed inline diagnostic,
  current input is the same borrowed immutable byte sequence in planner and
  CPU callback, and adversarial overlength plans/diagnostics cannot transfer
  owned storage;
- factory usage flags: `StorageRead` creates
  `STORAGE | COPY_SRC | COPY_DST` so staging initialization is legal while its
  shader binding remains read-only; every other usage maps to the documented
  exact backend flags and no usage exposes mapping or indirect dispatch;
- live-volume reuse versus permanent `volume_records` exhaustion, including
  bounded manifest tombstones;
- extraction, directory root/node/entry/authority versions, presentation
  artifact/dirty/instance, dressing registry, behavior egress maps/receipts/
  records/bytes, and extension registration/byte-pool defaults, boundaries,
  pressure facts, effective-config reflection, and telemetry high-water
  accounting;
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
  bounded input capture and confirmed GPU upload before planning, isolated
  per-participant exports,
  direct CPU callback without query
  admission, post-frontier command blocking, CPU-to-GPU and GPU-to-CPU handoff
  milestones, whole-proposal reject/replace/fail
  composition, conservative sparse-patch AABB conflicts, bounded conflict-
  check exhaustion before publication, typed tick-abort discard outcomes,
  post-publication notification failure, prior-feedback rotation, per-volume
  tick atomicity, and independent-volume failure;
- first-participant ingress without a dummy predecessor: vary payload bytes
  across consecutive single-participant CPU ticks and single-participant GPU
  ticks, prove planner/CPU borrow and GPU binding 5 see exactly the current
  bytes, and prove no prior handoff, shared state, hidden allocation, raw GPU
  access, or authority-path readback supplies them. Input upload validation,
  renderer upload failure, and device loss all terminate before any adapter
  execution; upload failure is tick-global even for `SkipParticipant`. Assert
  the exact `UploadingGpuInputs -> Planning` order and that no planner-owned or
  adapter-owned CPU state changes on either upload failure or device loss;
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
- shutdown drain/`CancelNotPrepared`/checkpoint reports, including a behavior
  tick cancelled while waiting at its captured frontier immediately before
  `Preparing`, and a tick shut down immediately after entering
  `Preparing/UploadingGpuInputs` that retains its submitted ranges and drains
  to a complete preflight-or-later report;
- device-generation callback quarantine and recovery/terminal branches.
- world-directory epoch exhaustion with an injected near-maximum allocator:
  an ordinary root-changing operation can publish `u64::MAX` exactly once and
  then leaves the current root readable/checkpointable while later
  root-changing submissions reject in `DirectoryEpochExhausted`; in a separate
  world at `u64::MAX - 1`, a scheduled tick with two selected root proposals
  whose range would overflow returns
  `NoPublication(DirectoryEpochExhausted)`, marks every otherwise selected
  proposal `TickAborted`, changes no revision/root, and never wraps or reuses
  an epoch. A checkpoint captured afterward has epoch `u64::MAX - 1` and
  `DIRECTORY_ALLOCATOR_CLOSED` set; same-key and import restore both succeed
  with `StartupApplied::state == DirectoryEpochExhausted`, report the closed
  `DirectoryCheckpointState`, reject root work, and never reopen publication.
  A control checkpoint at the same lower epoch with the flag clear restores
  `Ready` and may publish `u64::MAX` once. Maximum epoch with the flag clear,
  zero epoch, and every unknown flag bit fail decode as corrupt before root
  installation. Separately force device loss from the lower-epoch exhausted
  world with fully durable scars and hold recovery before reconstruction
  completes. In that held state assert every `try_reserve_*`/`reserve_*` is
  closed; every queued submission with a previously acquired permit returns
  `SubmitError::WorldNotAccepting(Recovering)` with owned inputs and capacity
  reclaimed; `declare_interest`/lease update, new subscription, and runtime
  extension registration return their family-specific
  `WorldNotAccepting(Recovering)` before allocating. In the same held state,
  invoke immutable material lookup; receipt ID/status/cancellation; existing
  interest ID/accepted/last-published state; existing subscriber
  ID/accepted/poll/resume; telemetry; shutdown-receipt inspection; handle
  clone/drop; and last-lease withdrawal. Assert their ordinary host results,
  that withdrawal decrements host references without renderer work, and that
  interest `Ready` is only a retained readiness snapshot rather than query
  admission. In a separate otherwise identical held-recovery world, call
  `shutdown` and assert it enters `ShuttingDown` under the once-only contract
  rather than returning a lifecycle rejection; that world is not reused for a
  recovery-completion branch. Run the non-shutdown matrix once before
  successful recovery and once before forced terminal recovery failure.
  Successful recovery passes through
  `Recovering(closed)` and returns to `DirectoryEpochExhausted`, not `Ready`;
  root rejection and the complete exhausted-state matrix remain identical
  afterward. Also test fresh exhaustion, maximum-epoch restore, lower-epoch
  closed restore, interest declare/update/withdrawal acceptance, every generic
  reserve family, matter/ordinary move/query/checkpoint/subscription/extension/
  non-root tick acceptance, extension all-or-none admission for its non-root
  fill/patch/move candidates, shutdown, and terminal recovery failure. For the
  lower-epoch closed restore, begin with zero runtime interest leases and every
  reconstructed region cold. Declare interest over one cold brick and require
  `Cold -> Requested -> Materializing -> Ready`; withdraw it, retire the brick
  lease and advance the region lifecycle back to `Cold`, then submit a
  `ReadinessPolicy::Materialize` query and require the exact restored
  sample/revision. Keep a second restored brick cold, submit a matter mutation
  targeting it, and require ordinary base-plus-scar materialization followed by
  one atomic committed revision. Repeat a cold dependency through a non-root
  scheduled view and an Extension ABI v1 inspection/effect batch. Every path
  must stay within ordinary interest, content, residency, and queue bounds,
  consume no directory epoch, and return its ordinary typed content/pressure
  failure when forced; the ready-resident controls must produce the same public
  truth. Root-changing work remains rejected throughout.

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
- every `DirectoryRootV2`, `DirectoryNodeV2`, and
  `DirectoryEntryVersionV2` plus `VolumeAuthorityVersionV2` host/WGSL offset,
  size, tag, null-index, low/high-word, finite placement/domain, and
  zero-reserved invariant;
- bad transaction sentinel path;
- extension attempts to bind internal storage;
- every Extension ABI v1 host/WGSL offset and size;
- malformed candidate kind/reserved word/offset/alignment/state ID, missing
  exact revision precondition, oversized WGSL/entry point/registry, and
  malformed candidate effect layout;
- observation-delta frontier ABI parity: both oldest/head words are zero only
  for `Empty`, both are nonzero for `Retained`, one-zero/one-nonzero is rejected,
  and pre-sequence-1 `Complete` has zero records and a zero cursor;
- every Scheduled ABI v2 host/WGSL size, alignment, offset, and stride:
  the five retained 64-byte headers, the headerless component-reservation
  section, 80-byte egress header, 48-byte reservation record, 32-byte component-piece,
  24-byte assignment, 64-byte placement update, the 112-byte volume, 24-byte cell, 128-byte
  proposal, 20-byte patch run, 32-byte handoff descriptor, 64-byte feedback
  participant, and 48-byte feedback proposal;
- the headerless reservation formula is exactly 48 bytes times
  `maximum_component_extraction_proposals` times
  `maximum_component_extraction_children` under checked arithmetic; the effect
  header's aligned `reservation_offset/reservation_bytes` and
  `egress_offset/egress_bytes` ranges are nonoverlapping and in binding range.
  A phantom-header offset, wrong product, overflow, misalignment, overlap, or
  out-of-range end fails at ABI validation;
- host `[u8; 16]` key/schema fields have the exact same 16 wire bytes as four
  WGSL `u32` words, and egress record strides reject nonmultiples of four;
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
  over-capacity, input presence/payload/total-byte mismatch, absent required
  input, present input above the participant/effective range, invalid
  snapshot/volume indices, changed revisions, invalid
  cell size/domain, cross-participant record access, stale device generation,
  a second kind-5 record or kind 5 with a zero placement maximum,
  malformed prior feedback, component-extraction duplicate/empty/unreserved
  assignments,
  egress stride/count/overflow mismatch, and old-generation
  handoff/feedback/identity/egress reuse.
- scheduled group-0 reflection requires exactly bindings 0..=5 with the
  documented access. Binding 5 executes exact byte parity for absent optional,
  present empty, one-byte, maximum-byte, and varying-across-ticks input; an
  extra/missing/writable input binding or direct use of a handoff as implicit
  ingress fails layout validation. Binding 1 reflection and golden bytes cover
  its proposal/payload, canonical child-reservation, and opaque-egress
  subranges, including offsets 48..60 in `BehaviorEffectHeaderV2`.
  Any overlapping/out-of-range section or shader modification of a
  reservation byte fails the complete participant output. A
  missing/extra/wrong-access binding fails;
- scheduled feedback golden cases use status 4 for a placement stream and
  status 5 for component extraction, retain the command ID and proposal index,
  require zero single-volume revision/volume fields, and reconcile the full
  revision result through the typed receipt plus tick published vector;
- golden feedback fixtures map participant abort, two-party conflict fail-tick,
  transition failure with predecessor/successor/stage, preparation failure,
  directory-epoch exhaustion as abort-cause tag 6 with zero cause payload,
  device loss with a two-word generation, and
  published-with-notification-failure with exact failed-hook count. Add the
  two-participant input-upload pre-execution fixture with addressed execution
  tag 2/failure category 12 and unaffected execution tag 3/failure category 13
  plus the failed engine in A, preparation-failure disposition, not-applicable
  notifications, zero snapshot/proposal/published records, and zero planner,
  report-hook, or adapter calls. The device-loss preflight fixture uses
  execution tag 3/failure category 9 and the exact generation for every
  participant. Add the
  mixed independent-volume fixture in which tick flag bit 0 is set while
  participant A's flag bit 2 is clear and participant B's bit 2 is set. Each
  fixture round-trips to the corresponding Rust disposition, cause,
  participant publication/notification, proposal outcome, flags, and unused
  zero fields; changing any required/unused field fails at feedback validation.

Passing compilation is only validation evidence, not execution correctness.

### Persistence codec tests

Run every requirement in [persistence.md](persistence.md), fuzz decoder inputs
under bounded allocation, and keep checked-in v2 golden fixtures. Golden
manifests include open epoch `u64::MAX - 1` with flags zero, closed epoch
`u64::MAX - 1` with exactly `DIRECTORY_ALLOCATOR_CLOSED`, and closed epoch
`u64::MAX`. Decode rejects maximum epoch with a clear flag and every reserved
flag bit. Re-encoding each accepted fixture is byte-identical, and restore
asserts the matching `DirectoryCheckpointState` and operational `WorldState`.

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
    prove declared DAG order, direct CPU view delivery, and bounded current
    consumer input reaching a first/only CPU adapter and a first/only GPU
    adapter while changing across ticks. The CPU planner/callback and GPU
    binding 5 must consume identical request bytes without a predecessor,
    dummy adapter, hidden allocation, raw resource, or readback. Also prove
    unequal cell-size/domain interpretation including a post-registration
    volume, GPU view/effect
    publication with zero authority-path readback, CPU-to-GPU upload and
    GPU-to-CPU mapped handoff, fixed proposal validation, whole-proposal
    conflict/tick-abort outcomes, prior-feedback reconciliation,
    old-generation feedback quarantine,
    exact low/high-word integer readback parity for every scheduled
    header/record and all documented terminal-feedback golden outcomes,
    restricted-factory per-adapter and aggregate buffer-byte enforcement,
    drop-after-last-use release, terminal-generation teardown/recreation, and
    logical OOM rejection before a backend allocation,
    one revision per affected volume, and post-frontier command exclusion.
    Induce missing/duplicate/unknown/oversized input, cancellation, input-upload
    failure, participant failure, rejected effects, checkpoint/restore,
    shutdown, and device loss; prove every input failure occurs before adapter
    execution, Moria never owns adapter state, late old-generation work/input
    cannot publish, CPU state is untouched, and GPU adapters must recreate and
    report ready. Attempt arbitrary scheduled create and source-object
    transport and prove both unrepresentable; exercise only the pre-reserved,
    source-bound component-extraction split. A later ordinary create is
    independently admitted and not part of the tick.
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
tick with explicit per-participant opaque inputs, run an asynchronous WGSL job,
and shut down through
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

The GPU variants are purpose-built or substantially adapted to be
Moria-conforming: they use only the restricted factory, fixed six-binding
group-0 ABI v2, and counted encoder. They are not presented as arbitrary
pre-existing engine binaries/resources or an engine-owned submission model.

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

Before testing multi-adapter order, run one CPU participant alone and one GPU
participant alone. Give each `Required` input, vary it across at least three
ticks (including a present empty payload and its declared maximum), and make
its proposal depend on those opaque bytes. Assert the CPU planner and callback
borrow the same bytes and the GPU shader reads the exact bytes through binding
5. There is no predecessor, handoff, dummy participant, shared-state lookup,
new allocation after the tick permit, raw GPU handle, or authority-path
readback. Then exercise missing required, unknown, duplicate, one-byte-over,
cancellation, upload failure, and device loss and prove their closed
fail-before-adapter-execution outcomes. In the mixed two-participant case,
fail the GPU upload after the CPU and GPU inputs are admitted but before any
planner call. Assert the upload-failed GPU participant is
`Skipped(ConsumerInputUpload)`, the CPU participant is
`NotRun(InputPreflightAborted { failed_engine })`, both are
`DiscardedByTick(PreparationFailure)` with `NotApplicable` notification, and
the tick has empty snapshot/proposal/published vectors. No CPU planner,
`run_tick`, or `on_tick_report` call may occur. The 64-byte golden records use
execution tags 2 and 3 respectively; the not-run record uses failure tag 13
and stores the failed engine in A. Repeat with device loss and require tag 3 /
failure tag 9 plus the exact generation for every participant.

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
feedback. Also force A's selected volume to fail preparation while B's
independent volume publishes; assert A is
`Published { revision_changed: false }`, B is
`Published { revision_changed: true }`, tick-wide revision change is true,
and the feedback flags encode all three values without inference. The adapter
must reconcile by prior proposal index from consumer-owned retained state;
the feedback binding contains no snapshot-vector record. The GPU assertion
includes both engine/proposal pairs for
`FailTick`, predecessor/successor/stage for transition failure, the device
generation pair, preparation failure, disposition, defined flags, and exact
failed-hook count after publication. Repeat across checkpoint, restore
readiness, device loss/recovery, and shutdown.

For the fracture/debris-shaped GPU variants, label at least three connected
pieces in one pinned source and publish at least two pre-reserved dynamic
children. Prove exact coordinate/sample conservation, tolerance-bounded
world-box continuity, and one directory-epoch visibility gate. Copy the
canonical child-reservation piece-handle/final-ID mappings into
factory-owned body state and publish without CPU authority-path readback.
Arbitrary child content and passing a Rust `BaseContentSource` through either
scheduled sink/ABI must remain impossible.

The independent reviewer must compile an external-style adapter that attempts
every public route to obtain `RenderDevice`, `wgpu::Device`, a raw buffer/
pipeline/bind group, raw encoder, queue, or submission; import an externally
created resource into the behavior encoder; bind another participant's export;
bypass proposal validation; exceed access/collision/handoff/proposal/dispatch/
workgroup bounds; or introduce behavior-specific fields. Any success fails C9.
It must also attempt to use an arbitrary external GPU buffer/device/command
model in place of the restricted adapter resources and to read current input
through an undocumented side channel; either success fails C9.

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

### C11. Atomic GPU component extraction

A GPU proof adapter labels three or more six-neighbor connected pieces in one
hot source volume, retains a nonempty source remainder, and publishes at least
two children. Before its dispatch, assert that every possible child ID/key,
directory/lifetime record, page/brick/scar/transfer/provenance record, proposal,
receipt, and byte is reserved. The shader copies the child-reservation
piece-handle-to-final-`VolumeId` subrange into its own factory-created body
state without modifying the canonical bytes.

An independent CPU oracle compares exact coordinate/packed-sample records and
world-space cell boxes within the specified f32 corner tolerance before and
after the one `WorldDirectoryEpoch` publication. Old-epoch
readers see only the original source; new readers see only the complete source
remainder and all children. This fixture selects zero explicit removals, so no
coordinate/sample record is duplicated, absent, or temporarily ownerless.
Query, collision, observation, presentation invalidation, receipt,
checkpoint, restore, and cold rematerialization agree on child identity,
placement, inherited cell size/material samples, provenance, and revision.
The extraction receipt contains only published children in ascending piece
handle order, while the tick's unique live revision vector is in ascending
runtime-ID order and omits a retired source.
Same-key and import restore preserve each derived child's extraction
nonce/reservation slot and stable key; the next extraction preflights a fresh
candidate set and cannot collide with a saved child key.

Fault injection covers every validation phase, live/lifetime ID exhaustion,
directory root/node/entry/authority-version exhaustion,
page/brick/scar/byte exhaustion, renderer OOM, unused reservations,
cancellation, shutdown, device loss before and after the gate, and delayed
old-reader reclamation. Every unpublished identity and resource returns to its
pool after last use. Passing a source object or inventing a sample fails.
Force every complete candidate table for salts `0..=255` to collide with a
live key, tombstone, or another candidate. Submission must return the unchanged
request with `ComponentIdentityExhausted`, expose no tick ID or partial table,
invoke no planner/adapter, and release every reserved identity, record, byte,
receipt, and cleanup slot.
Retire one published child, reuse its runtime live slot for an unrelated
volume, and prove the child's lifetime-index tombstone and new live mapping
remain distinct through query, checkpoint, and restore.
For a rejected proposal, next-tick GPU feedback makes the proof adapter remove
every provisional body association; none of the failed candidate `VolumeId`s
resolves in a later view or effect.

### C12. CPU-authored regions and persistent multi-fidelity adapter

The validation adapter declares required current input and owns a fixed
test-only region schema carried as opaque binding-5 bytes. Use at least two
disconnected regions, then overlap them. A GPU classification pass writes one
class per persistent body before mark/scan/scatter; assert a body in the
overlap appears exactly once. A per-cell diagnostic also proves every exported
cell of a matter-backed overlap body is visited exactly once. One
projectile-shaped body crosses full, halo, coarse, halo, and full
classifications without changing body/volume identity or discontinuously
changing the adapter-owned transform/velocity oracle.

Objects outside every region continue changing in the adapter's coarse pass.
Their Moria placements remain current through one compact placement stream,
with no host body enumeration and no ordinary move proposal per object.
Exercise a remote generic remove/extract proposal as a debris/destruction-
shaped outcome while verifying Moria owns no such meaning. Changed placement
records are unique, the placement receipt follows ascending snapshot index,
and the tick's unique runtime-ID-sorted directory epoch/revision vector is
exact.

Run empty, 1%, 50%, and 100% active compact lists at the declared 65,536-body
capacity. Every maximum-list pass dispatches 512 workgroups at width 128.
The complete adapter uses exactly 11 dispatches/at most 3,604 workgroups
against declared maxima 16/8,192. Overrun, duplicate classification, stale
placement, a separate adapter/world per region, or transform/velocity
discontinuity fails.

### C13. Opaque GPU-to-CPU adapter egress

Register an adapter-owned fixed record layout whose fields and meaning do not
appear in Moria. Round-trip its initialized bytes and schema ID exactly for
zero records, one record, multiple records/ticks, and exact capacity. Results
are delivered in tick order with the request correlation. Zero is a successful
empty result, not pending or unavailable.

One-over capacity must return exact overflow with no prefix; saturating-counter
overflow is a separate failure. Malformed header/stride/count/reserved fields,
cancellation before preparation, skipped and not-run participants, shutdown,
map failure, decode failure, and device loss each produce their closed result.
For a participant that completed with a valid prefix, separately exercise
`RejectLater`, `ReplaceEarlier`, another participant's `FailTick`, another
participant's `AbortTick`, and transition/preparation no-publication. The
same assertion applies to directory-epoch-exhaustion no-publication. The prefix
remains byte-exact and deliverable in every case even though proposal and tick
receipts report their independent rejection/disposition. By contrast, the
skipped and not-run fixtures return
`ParticipantUnavailable` with the exact existing execution reason and no
prefix.
Publication receipts remain independently truthful before egress
mapping/decoding. Inject map and decode failure after a tick that published and
assert `OperationError::revision_changed` equals that tick's true value; repeat
after no-publication and assert false. A published zero-change tick preserves
false. Device, working, staging, and host bytes are not reused
before their defined last-use/unmap/drop milestones, and dropping the receipt
still reclaims them. Clone a ready result and its byte handle, drop the
original receipt/result, and prove the one shared host slot remains charged
until the last clone drops without duplicating its byte charge. Attempts to
obtain a raw mapped view/authority buffer or route a GPU handoff through egress
fail.
Retain terminal zero-byte receipts through the configured receipt limit;
the next tick cannot execute until a receipt permit is available, and dropping
one permits progress without changing byte/map accounting.

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
  bytes, consumer-input host/upload bytes, handoff upload/map bytes,
  synchronous Bevy-main-thread CPU callback time, total frontier-held time, and
  restricted-factory GPU bytes.

GPU timestamp queries are used only when supported. CPU wall time remains
recorded. Every run states warm-up count, sample count, workload dimensions,
density, mutation distribution, in-flight depth, adapter/driver, build profile,
and fallback status. Synchronous readback is never inserted into a production
hot path merely to time it.

### Architecture feasibility gates

The selected GPU hash/MVCC, bounded readback, collision, scheduled behavior,
asynchronous extension, checkpoint,
and dual-contouring architecture is not implementation-ready on measurement
alone. The following gates are falsifiable. They are minimum feasibility
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
| P10 scheduled CPU/mixed behavior | Register distinct nonempty materials `m1 = MaterialId(1)` and `m2 = MaterialId(2)`. Their samples are respectively `{ material=1, coverage=255, flags=0 }` and `{ material=2, coverage=255, flags=0 }`, encoded in the canonical little-endian scheduled `sample:u32` field as `0x00FF0001` and `0x00FF0002`. Create 32 hot volumes, named `V0..V31` in ascending runtime-ID order, each with four full bricks at local cell bounds `[8b,0,0)..[8(b+1),8,8)` for `b=0..3`. The CPU adapter exclusively views `V0..V15`; the following GPU adapter exclusively views `V16..V31`. Thus each view has exactly 16 volumes, 64 bricks, and 32,768 cells and scans every record. For measured tick `t`, byte `j in 0..512` of CPU input is `(t+j) mod 256` and of independent GPU input is `(t+3j) mod 256`; neither input comes from a predecessor. CPU writes all 65,536 handoff bytes as `handoff[k] = cpu_input[k mod 512] XOR floor(k/512)`. GPU validates that complete pattern. Each adapter emits exactly 64 patch-run proposals in volume-major/brick-min-X order with `Correlation::NONE`, the supplied exact revision, and one proposal per brick. Each proposal has one canonical 20-byte run `{start_index=0, length=512, sample=oracle}` covering that full 8³ target. CPU chooses sample `m1` when `cpu_input[0] & 1 == 0`, else `m2`; GPU chooses `m1` when `(gpu_input[0] XOR handoff[0] XOR handoff[1]) & 1 == 0`, else `m2`. Initialize each group to the exact sample opposite its tick-1 oracle; both oracles then alternate between the two distinct packed values, so every target changes each tick. Across both adapters this is exactly 128 proposal records (16,384 bytes), 2,560 payload bytes, 65,536 affected cells, 128 affected bricks, 32 affected volumes, and zero directory effects. Use `RejectLater`; disjoint volume sets mean all 128 are admitted with no conflict/replacement. If starting revisions are `r_i`, measured tick `t in 1..=100` must publish exactly `[(V_i, r_i+t); i=0..31]` in runtime-ID order and no other revision. Run with default pools and no other workload on each forced backend family. | p95 synchronous main-thread CPU planner+callback <=4 ms; p95 total main-world Moria work for the callback update <=8 ms; p95 frontier-to-publication <=50 ms; every tick consumes the exact current input/handoff bytes, admits all 128 proposals, produces the exact oracle samples and 32-entry revision vector, and reports the stated record/payload/cell/brick/directory counts; input/handoff/view/proposal/transaction allocations stay within default config with no growth after warm-up. The qualification receipt separately reports CPU callback time and total frontier-held time. |
| P11 multi-fidelity placement | One GPU adapter owns 65,536 persistent records, of which 16,384 are distinct dynamic Moria volumes, and receives 32 CPU-authored AABB regions with equal-width halos. This gate sets `live_volumes=16,384`, `volume_records=32,768`, and `behavior_view_volumes=16,384`; other limits retain defaults. The descriptor uses `BehaviorVolumeFilter::All { maximum_volumes: 16,384 }` and the planner pushes one `VolumeRecords { maximum_volumes: 16,384 }` scope, exporting exactly 16,384 volume records and zero cell records without transferring a planner-owned list; this does not use the ordinary 256-volume query filter. Fixtures use disconnected, 50%-overlapping, and fully overlapping arrangements. Classification compaction, three full/halo/coarse proof kernels, and changed-placement compaction use workgroup width 128, exactly 11 adapter dispatches and at most 3,604 workgroups against declared maxima 16/8,192; each maximum-list pass has 512 workgroups. Measure empty, 1%, 50%, and 100% full/halo active lists; every body still runs either transition or coarse work and all 16,384 matter-backed bodies change pose/publish placement per measured tick. | p95 input-upload-to-directory-publication <=33 ms at 50% and <=50 ms at 100%; GPU time <=20 ms when timestamps exist; adapter dispatches/workgroups are exactly 11/at most 3,604 and declared charges are 16/8,192; one-time overlap classification, exact continuous boundary oracle, continued coarse changes, exactly 16,384 unique placement entries/revision advances, zero host body enumeration, zero exported cell records, and every placement/directory pool within its configured limit |
| P12 atomic component extraction | One 32,768-cell/512-brick source contains 64 disjoint connected pieces of 512 cells. Publish 63 dynamic children and retain one source piece. This gate sets `live_volumes=8,192`, `volume_records=8,192`, and both desired/minimum `detailed_bricks` and `dirty_scar_bricks` to 65,536; other limits retain defaults. Pre-reserve all default component-extraction capacity, use the binding-1 child-reservation IDs, transfer every sample, rebuild 64 directory entries, and hold one old-epoch reader through publication. Repeat 100 times with newly prepared sources; no presentation work is included. | p95 adapter-output-to-directory-gate <=50 ms and GPU transfer/validate/publish <=20 ms when timestamps exist; exact coordinate/sample conservation, world-box continuity within the specified f32 tolerance, 63 usable GPU child IDs, one old-or-new epoch, zero authority-path readback, and all unused reservations reclaimed after last use. Live/dirty authoritative counts must rise by exactly the published result; allocator capacity and nonauthority overhead may not grow >1% after warm-up. |
| P13 opaque egress | One GPU adapter declares a 64-byte unknown record, 16,384-record/1 MiB maximum, and emits 0, 1, 8,192, 16,384, then 16,385 required records in a repeating sequence. Two staging slots alternate while publication effects remain valid. | p95 GPU-complete-to-decoded ready <=16 ms for <=8,192 records and <=33 ms at exact capacity; byte-exact initialized prefix/schema/tick/correlation and increasing-tick delivery order; zero is ready-empty; exact capacity succeeds; one-over is explicit overflow with zero delivered bytes; device/staging/host high-water stays within config and returns to baseline after result drop |

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
- P10 failure rejects the selected synchronous-main-thread CPU/mixed scheduled
  execution path and requires revisiting callback placement, staging, or the
  selected workload; it cannot be waived by the GPU-only P7 result.
- P11 failure rejects fixed maximum dispatch or the compact placement-stream
  mechanism for persistent multi-fidelity integration.
- P12 failure rejects atomic GPU component extraction or its directory-epoch
  publication mechanism.
- P13 failure rejects the opaque CPU-egress transport or its selected staging/
  delivery bounds; publication correctness does not convert that failure into
  a pass.

The affected architectural claim remains `fail`, not “report-only,” and the TDD
implementation cannot be called contract-complete until it is revised or the
gate passes on every claimed backend family. Performance never excuses a
correctness failure, and a correctness pass never substitutes for these
feasibility receipts.

## Portability qualification

At least one physical adapter for each claimed Linux/Vulkan, macOS/Metal, and
Windows/DX12 family runs:

- all real-GPU correctness tests;
- contract scenarios C1–C13 where platform capabilities allow loss injection;
- shader validation;
- a fixed report-only workload;
- architecture feasibility gates P1–P13;
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
- a P1–P13 pass missing its workload scale, in-flight pressure, percentile,
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
