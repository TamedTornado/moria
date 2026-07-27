# Technical Decision Record

These are engineering decisions made under the approved product design. They
are not human product decisions and may be revised by a later TDD with
equivalent public semantics and new evidence.

## T1. One package with a Bevy adapter module

**Decision.** Start with one `moria` library package. The facade and Bevy
adapter are modules in that package; examples are ordinary consumers.

**Reason.** There is one deliverable and one renderer integration. A workspace
would add dependency and release coordination without providing a required
compile or packaging boundary.

**Revisit when.** A standalone compute library, tool binary, or separately
versioned adapter becomes an actual deliverable.

## T2. Fixed 8³ bricks and compact scalar samples

**Decision.** V1 uses 8×8×8 bricks and a four-byte material sample.

**Reason.** A 2 KiB detailed brick is small enough for copy-on-write mutation,
bounded readback, and fine-grained interest while retaining useful GPU
coalescing. Coverage supports non-cubic surfaces without promoting meshes to
truth.

**Rejected.** A dense world, camera-only clipmap, and hardware sparse textures
all weaken volume-general bounded residency or portable compute support.

## T3. GPU hash pages with MVCC revision chains

**Decision.** A bounded open-addressed hash table maps logical brick keys to a
version chain. Each new page version is tagged with its proposed volume
revision; a single revision-gate write publishes a prepared command.

**Reason.** Irregular sparse volumes favor hashing. Revision filtering lets
readers ignore prepared pages until one atomic publication and lets prior
readers finish without global copies.

**Rejected.** In-place multi-brick writes expose partial state. Double
buffering the entire page table makes mutation cost scale with resident world
size. A camera clipmap privileges presentation interest.

## T4. One revision stream per volume

**Decision.** Matter, placement, creation readiness, and retirement facts use a
single monotonic revision stream per volume. A world observation sequence
orders delivery but is not a cross-volume truth revision.

**Reason.** Queries can report the exact revisions observed without promising
unsupported atomicity between independent volumes.

## T5. Runtime-neutral receipts with Bevy-driven progress

**Decision.** Facade handles are `Send + Sync`; accepted work returns a typed,
`Send` receipt that implements `Future` and also supports nonblocking polling.
Bounded owner queues and render schedules drive progress. No async runtime is
part of the public contract.

**Reason.** Games may use Tokio, another executor, or Bevy tasks. Moria must not
force one or confuse queue admission with completion.

## T6. Content source executes on bounded workers

**Decision.** Base content is a synchronous, batch-oriented `Send + Sync`
callback executed by Moria's bounded worker pool.

**Reason.** Authored I/O and consumer generation stay outside render schedules,
while a synchronous callback avoids imposing a consumer async runtime.

## T7. GPU behavior uses copied packets and candidate effects

**Decision.** GPU extensions receive bounded, Moria-produced inspection packets
in extension-owned buffers and write fixed-schema candidate effects. They
never bind the page table or brick pool. Candidate effects are validated and
published through normal admission.

**Reason.** This preserves a GPU-to-GPU path without giving a behavior engine
storage ownership or a privileged mutation route.

**Rejected.** Direct read/write buffer leases are faster to prototype but
invalidate ownership, bounds, atomicity, and recovery contracts.

## T8. Sparse full-brick scars

**Decision.** A scar stores the complete committed 8³ sample payload for every
brick changed relative to the registered base, plus persistent volume identity
and placement. Optional compaction may remove a scar proven byte-equal to base.

**Reason.** Full changed bricks give deterministic restore and simple checksums
without serializing untouched matter. Cell-level compression can be added
inside a versioned chunk codec after measurement.

## T9. Dual contouring for presentation; cell occupancy for collision

**Decision.** GPU dual contouring derives per-brick surfaces. Material
presentation class selects smooth or feature-preserving constraints.
Collision tests occupied cells and coverage, independent of the mesh.

**Reason.** One technique can produce coherent organic and crisp constructed
surfaces, while separating collision preserves truth when meshing is stale or
failed.

## T10. Native renderer integration only

**Decision.** V1 qualifies Vulkan, Metal, and DX12 by capabilities, not backend
name. WebGPU, WebGL/GLES, and standalone-device constructors are absent.

**Reason.** The approved design excludes web as a current target, and Bevy owns
the renderer device in the supported path. Unsupported targets fail clearly
instead of receiving weaker semantics.

## T11. Reserve the complete GPU-extension effect batch

**Decision.** An extension request reserves its worst-case child command
records, aggregate payload bytes, and receipt slots before shader dispatch.
Whole candidate output validation and child admission are all-or-none; admitted
children then complete independently.

**Reason.** One ordinary command permit cannot bound a shader that may emit
many effects. Pre-reservation preserves normal admission without a privileged
queue or pressure-dependent partial admission.

**Rejected.** One candidate per extension request needlessly multiplies packet
capture/dispatch overhead and weakens the intended GPU-oriented handoff shape.

## T12. Whole-world checkpoints in v1

**Decision.** `CheckpointScope` has only `WholeWorld`; restore requires exact
live-volume membership and permits extra current material registrations only
when no saved sample refers to them.

**Reason.** Whole-world membership gives unambiguous volume identity,
tombstone, placement, and dirty-frontier semantics. Partial-volume imports need
namespace/conflict policy that the approved contract does not require.

## T13. Collision kernel below public query orchestration

**Decision.** `collision` is a private lower-level fact kernel over storage.
`query` owns public descriptors, partial/readiness policy, result codecs, and
contact results.

**Reason.** This gives one acyclic direction and keeps collision truth reusable
without coupling storage traversal to receipt or public result policy.

## T14. Separate live-volume and lifetime-key capacities

**Decision.** `live_volumes` bounds concurrent directory entries, while
`volume_records` bounds every stable key accepted for the world's lifetime.
Retirement frees only the live slot and preserves one bounded tombstone record.

**Reason.** Reusing a retired stable key would make checkpoint membership and
restore ambiguous. A separate lifetime bound keeps key history and manifests
finite without preventing ordinary live-slot reuse.

## T15. Builder-time dressing registry

**Decision.** Surface inputs are embedded in `MaterialDefinition`; derived
dressing is registered separately on `MoriaBuilder` with a stable style key,
exact material-key filter, bounded descriptor, and Bevy asset handles.

**Reason.** One style can serve several materials and consumes an independent
instance pool. A callable registry removes unresolved style IDs while keeping
dressing presentation-only and outside persistence authority.

## T16. Closed GPU extension ABI v1

**Decision.** GPU extensions use fixed 32-bit packet/snapshot/inspection,
opaque-state, diagnostic, candidate, and patch-run layouts. Inspection and
effect kinds are closed; every effect carries an exact captured revision.

**Reason.** Named-but-opaque packet types cannot support external WGSL or
layout validation. A fixed bounded ABI preserves GPU-to-GPU inspection and
state while allowing Moria to validate and translate effects into ordinary
commands without exposing storage.

## T17. Collision work is separately authorized; partial never truncates hits

**Decision.** Every non-point collision query carries explicit candidate-brick
and candidate-cell limits within fixed v1 maxima. Partial coverage may omit
unavailable regions only; exceeding the hit cap always fails with
`OutputOverflow` and returns no facts.

**Reason.** Result bytes do not bound traversal through a huge sparse shape,
and a coverage mask cannot truthfully describe omitted contacts inside a brick
reported as inspected.

## T18. Preparation is the cancellation point of no return

**Decision.** Explicit cancellation and shutdown cancellation win only while an
operation is queued or waiting for matter. The atomic transition to
`Preparing` closes cancellation; later requests return `TooLate`, and
`CancelNotPrepared` drains preparation and later stages.

**Reason.** Preparation owns GPU and transaction reservations whose rollback
would otherwise require a second ambiguous cancellation protocol. One atomic
state race gives every operation family and shutdown the same testable rule.

## T19. Long-lived world filters snapshot membership

**Decision.** An accepted `All` interest or subscription pins the then-live
volume IDs. Interest additionally pins exact local bricks at captured
placements; subscription bounds remain an event predicate over the pinned IDs.
Create/retire/move never expands membership. Update or resubscribe refreshes it.

**Reason.** Live reevaluation could exceed already accepted volume/brick bounds
and would make `All` pressure failure part of every lease. Exposing the
resolved set makes the bounded snapshot semantics explicit.

## T20. Retained variable payloads have aggregate pools

**Decision.** Material metadata and observation payload bytes have independent
aggregate limits. Content callbacks have a concrete bricks-per-request limit,
and presentation reserves one dirty marker per live-volume slot in addition to
ordinary dirty records.

**Reason.** Per-record/count limits alone do not bound retained bytes or protect
eventual presentation progress when several legal producers are active.

## T21. Content callback bytes are admitted before consumer invocation

**Decision.** One content batch starts only after Moria atomically acquires a
callback slot and its exact worst-case response-byte permit. The batch is not
shrunk to current capacity. Returned ownership remains charged until validation,
copy/install or failure cleanup, and drop have completed.

**Reason.** Validating an oversized owned response only after return does not
bound concurrently live callback allocations. Atomic dual admission also
avoids count/byte hold-and-wait deadlocks.

## T22. Observation filters retain append-time geometry

**Decision.** Every ring fact carries a fixed, charged private envelope with
its revision-time local/world extents; move facts retain both old and new world
bounds. Subscription-gap snapshots return a typed live or retired state for
every pinned member, with retired members identified by stable key and terminal
revision.

**Reason.** Poll-time filtering cannot reconstruct historical placement after
directory-version reclamation, and an overwritten retirement fact must not
turn a pinned member into ambiguous absence.

## T23. Content callbacks return exact boxed ownership

**Decision.** A valid content response owns an exact
`Box<[BaseBrickResult]>`, lineage opaque bytes are exact boxed slices, and the
response does not echo a variable source descriptor.

**Reason.** Charging returned length while retaining spare vector capacity or
an echoed variable lineage does not bound simultaneous Moria-owned callback
allocations. Exact boxes make the pre-invocation permit equal to valid returned
ownership.

## T24. Volume names are canonical bounded directory data

**Decision.** Startup and runtime volume definitions accept 1..=96 UTF-8 name
bytes and retain an exact boxed copy in live records and tombstones.

**Reason.** Volume records outlive command permits and are persisted. A count
bound alone cannot bound unconstrained `String` capacity or name length.

## T25. GPU observation deltas are nonadvancing status-bearing reads

**Decision.** GPU delta inspection uses a caller-supplied cursor over one
subscriber's accepted filter without mutating its CPU cursor. The packet and
public result distinguish complete, paged, overwritten, and unsupported-fact
boundaries; blocked boundaries emit no effects and recover through a bounded
non-resuming subscription-state snapshot.

**Reason.** A fact-only packet cannot distinguish an empty delta from lost
history, and silently omitting a fact that does not fit the fixed ABI violates
the observation contract. Independent cursors avoid hidden competition with
ordinary CPU polling.
