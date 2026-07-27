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

**Decision.** Asynchronous GPU extensions receive bounded, Moria-produced
inspection packets in extension-owned buffers and write fixed-schema candidate
effects. They never bind the page table or brick pool. Candidate effects are
validated and published through normal admission.

**Reason.** This preserves a GPU-to-GPU path without giving a behavior engine
storage ownership or a privileged mutation route.

**Rejected.** Direct read/write buffer leases are faster to prototype but
invalidate ownership, bounds, atomicity, and recovery contracts.

**Scope clarification.** T27 supersedes this facility as the primary scheduled
behavior-engine seam. T7 remains the asynchronous WGSL inspection/effect job
decision.

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

**Decision.** An asynchronous extension request reserves its worst-case child
command records, aggregate payload bytes, and receipt slots before shader
dispatch. Whole candidate output validation and child admission are
all-or-none; admitted children then complete independently.

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

**Decision.** Asynchronous GPU extensions use fixed 32-bit
packet/snapshot/inspection, opaque-state, diagnostic, candidate, and patch-run
layouts. Inspection and effect kinds are closed; every effect carries an exact
captured revision.

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
shrunk to current capacity. Moria constructs the complete exact-length output
sink inside that permit before invocation and keeps it charged through
validation, install or failure cleanup, and drop.

**Reason.** Callback count and bytes must be acquired as one unit to bound
simultaneous work and avoid count/byte hold-and-wait deadlocks.

## T22. Observation filters retain append-time geometry

**Decision.** Every ring fact carries a fixed, charged private envelope with
its revision-time local/world extents; move facts retain both old and new world
bounds. Subscription-gap snapshots return a typed live or retired state for
every pinned member, with retired members identified by stable key and terminal
revision.

**Reason.** Poll-time filtering cannot reconstruct historical placement after
directory-version reclamation, and an overwritten retirement fact must not
turn a pinned member into ambiguous absence.

## T23. Content callbacks fill Moria-owned exact sinks

**Decision.** A content callback receives an opaque exact-length output sink
already owned and byte-reserved by Moria. Homogeneous writes copy one sample;
detailed writes borrow exactly 512 samples into a fixed slot. No result
collection or detailed box crosses ownership. Lineage opaque bytes remain
exact boxed slices, and no variable source descriptor is echoed.

**Reason.** Even an exact returned box can be overlength before post-return
validation. A permit-backed sink makes ownership crossing bounded by
construction and leaves invalid count/content as ordinary poisoned-batch
failure.

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
non-resuming subscription-state snapshot. A closed observation frontier is
either `Empty` or a nonempty retained oldest/head pair; pre-sequence-1 reads
and snapshots preserve `Empty`, encoded as zero sequence words.

**Reason.** A fact-only packet cannot distinguish an empty delta from lost
history, and silently omitting a fact that does not fit the fixed ABI violates
the observation contract. Mandatory nonzero frontier fields also cannot encode
a newly started world. Independent cursors and a closed frontier avoid both
ambiguities without inventing a startup fact.

## T26. Content descriptors are borrowed and errors are fixed inline

**Decision.** `BaseContentSource::descriptor()` returns an immutable borrow
tied to the source rather than an owned descriptor. `load_bricks` may return
only a closed error with a 192-byte inline UTF-8 diagnostic. Moria validates
and copies accepted lineage into canonical bounded world ownership; no
descriptor or diagnostic allocation transfers across a callback return.

**Reason.** A bounded brick sink does not constrain other owned values returned
by the same port. Borrowed identity and a fixed error record make the ownership
boundary structural, including invalid and adversarial source behavior, while
leaving consumer-internal allocation outside Moria accounting.

## T27. Scheduled stable-view behavior coordination is the primary engine seam

**Decision.** A consumer-triggered substrate tick pins one committed view,
runs builder-registered CPU/GPU adapters in a validated dependency order,
resolves bounded whole-proposal conflicts, and publishes selected effects
before completing the tick. CPU adapters receive a direct borrowed tick view.
GPU adapters encode on the Bevy renderer-owned device against a read-only
export and write-only effect target; GPU validation, composition, and
publication do not require CPU readback. Adapter state is always
consumer-owned. The prior copied-packet WGSL facility remains an asynchronous
inspection/effect API and is not the scheduled engine seam.

**Reason.** Ordinary query receipts cannot provide a deliberate pre-publication
tick boundary, force CPU engines into the wrong lifecycle, and make a
renderer-resident engine read back merely to participate. A coordinator with
one pinned view preserves Moria authority, bounds, revisions, receipts, and
device lifecycle while permitting independently implemented conventional CPU
and GPU engines.

**Composition.** `runs_after` forms a DAG with stable-key tie order. All
adapters see the same committed view. The later adapter selects
`RejectLater`, `ReplaceEarlier`, or `FailTick` for overlaps; each decision
accepts or rejects a whole proposal. No physics-before-damage order or
behavior-specific state exists in Moria.

**Rejected.** Direct leases of authoritative storage, a permanent CPU collision
mirror, mandatory GPU effect readback before publication, and hard-coded named
behavior phases.

---

## T28. Scheduled adapters use isolated exports, restricted GPU handles, and bounded transport

**Decision.** Each participant receives a filtered `S_i` from one pinned
frontier, including cell size and finite local domain. GPU adapters create and
use only generation-bound opaque resources through a Moria-enforced restricted
factory and counted encoder; no `RenderDevice`, raw wgpu resource, queue, or
encoder enters the public seam. Ordering edges may carry one pre-reserved
opaque handoff through explicit CPU/GPU upload/map/copy stages. GPU feedback is
double-buffered and becomes read-only input on the next tick. CPU collision
helpers reuse a pre-reserved exact sink and return borrowed facts.

**Outcome contract.** Every tick that enters planning returns a closed completed
report. Tick-wide aborts give every discarded valid proposal a typed cause and
`revision_changed = false`; a post-publication report-hook failure is reported
without undoing or relabeling publication.

**Reason.** A shared readable union defeats per-adapter authorization, raw
renderer handles defeat encoder/resource enforcement, and prose-only
processor transitions cannot move consumer stimuli. Isolated exports and
Moria-owned opaque transport preserve one committed truth while making access,
allocation, synchronization, feedback lifetime, and failure evidence
implementable.

**Rejected.** Trust-only GPU byte reports, adapter-owned raw buffers crossing
processor edges, collision `Vec` returns, same-tick feedback, and treating a
post-publication notification panic as a no-publication abort.

---

## T29. Scheduled wire integers, terminal feedback, and factory bytes are closed

**Decision.** Scheduled ABI v1 represents every logical 64-bit value as an
explicit low/high `u32` pair. Next-tick GPU feedback losslessly encodes the
Rust tick disposition and abort cause in a 64-byte participant record,
including both conflict participants, transition stage, device generation,
notification disposition, defined flags, and failed-hook count. All
factory-created behavior buffers share one adapter-clamped aggregate live-byte
pool in addition to each adapter's declared maximum.

**Lifetime and admission.** Builder registration checked-sums descriptor byte
maxima against the requested pool and startup checks the effective pool before
device-state creation. Buffer bytes remain charged through opaque-handle
dependencies and last GPU use; terminal-generation teardown releases them
before recreation. Logical exhaustion rejects before renderer allocation, and
renderer OOM releases the permit without registering a handle.

**Reason.** WGSL has no portable concrete `u64`, a category-only feedback
record cannot reconcile a complete terminal outcome, and independent
per-adapter maxima do not bound aggregate device memory. Exact word pairs,
closed feedback mapping, and one world pool make portability, recovery, and
memory pressure testable without exposing renderer authority.

**Rejected.** Scheduled WGSL `u64` declarations, undocumented feedback flags,
lossy conflict/transition causes, trusted byte reports, and renderer OOM as the
aggregate admission policy.

---

## T30. Scheduled feedback omits snapshots and distinguishes participant publication

**Decision.** Scheduled ABI v1 keeps its fixed feedback formula and does not
repeat the prior snapshot revision vector. Proposal outcome records retain
their original indices, and a GPU adapter owns any prior
proposal-index-to-snapshot correlation it needs. The participant record uses
separate flag bits for tick-wide and participant-specific revision change.

**Publication mapping.** On a published terminal path, a participant with no
proposal surviving into preparation receives `NoSelectedEffect`; one with at
least one proposal entering preparation receives `Published`, whose boolean is
true exactly when one of its selected volumes appears in the tick's published
revision vector. Thus a participant whose selected volumes all fail
preparation receives `Published { revision_changed: false }` even when an
independent participant makes the tick-wide value true. A no-publication tick
uses `DiscardedByTick` for every participant.

**Reason.** Repeating every participant snapshot would add variable feedback
records solely for correlation data the GPU adapter already had during its
prior dispatch. Explicitly omitting it preserves bounded fixed feedback while
keeping terminal outcomes lossless. Separate revision bits prevent a
tick-wide success from being misreported as a publication by every
participant.

**Rejected.** Advertising snapshot vectors absent from the wire format,
inferring participant revision change from the tick-wide bit, and treating a
participant whose selected transaction failed preparation as having had no
selected effect.

---

## Human review entry — external-behavior boundary

### Verbatim feedback

```text
TamedTornado (COMMENTED):
The external-behavior boundary needs another revision before this TDD is approved.

The current design primarily treats external behavior as an asynchronous client: submit a bounded query, await a receipt/readback, calculate behavior, then submit an effect command. The bounded WGSL extension is likewise a submitted inspection/effect job. Those are useful APIs, but they are not the scheduled behavior-engine seam intended by the product design.

Required outcome:

- Define a generic, first-class behavior hook at a deliberate point in Moria's substrate tick. An independently implemented CPU or GPU behavior engine must be able to participate against a stable committed substrate view and return proposed substrate effects before the tick's publication/commit boundary.
- CPU and GPU are both required integration cases. A CPU engine must not be forced into an ordinary asynchronous query/receipt loop when participating in the tick. A GPU engine must be able to remain on the renderer-owned GPU path without mandatory CPU readback merely to participate.
- Moria must provide the scheduling, synchronization, bounded-access, admission, conflict, failure, and device-lifecycle contract. External behavior owns its vocabulary and working state.
- Preserve controlled mutation: an external engine may read only the authorized stable substrate view and may propose effects, but it must not directly mutate authoritative Moria storage or bypass validation, resource bounds, revision safety, receipts, or publication rules.
- Define ordering/composition when more than one external behavior engine participates. Do not hard-code physics-before-damage as product semantics; adapters should be able to declare ordering through the integration/scheduling contract.
- Retain the asynchronous query API for inspection, tools, and consumers that do not participate in the substrate tick. Retain or reframe the bounded WGSL inspection/effect facility if useful, but do not present it as the complete general CPU/GPU behavior hook unless it actually satisfies this contract.

Use physics and damage as adversarial proof cases, not as features to implement:

1. A third-party CPU or GPU physics engine owns bodies, velocities, forces, joints, solver state, and policy. At the correct tick phase it can read the substrate collision/material view, execute its solver, update its own state, and propose any substrate changes.
2. A third-party CPU or GPU damage/fracture engine owns damage accumulation, bond strength, breakage, crumbling, and fracture rules. It may consume engine-specific impacts or other stimuli, inspect the same stable substrate view, retain its own CPU/GPU state, and propose material or structural effects.
3. Moria must not add rigid-body, damage, health, resistance, bond, fracture, gravity, force, player, or gameplay semantics to its data model. Moria should see only authorized inspection and substrate effects; the cause may be physics, damage, erosion, heat, mining, simulation, or something else.
4. Behavior state must remain consumer-owned. The TDD must state what happens to CPU and GPU behavior state across checkpointing, device loss, recovery, shutdown, stale revisions, and rejected effects without silently making that state Moria authority.

Do not assume that the previously discussed CPU collision cache or a particular GPU binding layout is required. Those were candidate mechanisms, not approved design. Select and justify the smallest implementation-ready architecture that satisfies the behavior above while preserving Moria's boundedness and authoritative-substrate invariants.

Please revise the architecture, lifecycle/schedule, public API, decisions, and validation contracts consistently. The independent reviewer should specifically try to disprove that a conventional CPU physics adapter, a GPU-resident physics adapter, and an external CPU/GPU damage-and-bond adapter can participate at the proper tick boundary without Moria absorbing their semantics.
```

### Technical decision and clarification

The scheduled coordinator in T27 and
[behavior-scheduling.md](behavior-scheduling.md) is the primary external
behavior seam. It serializes a consumer-triggered tick around an ordinary-
Each adapter receives only its separately authorized filtered export from that
frontier. GPU adapters own their algorithms/state in resources created through
the restricted renderer-backed factory and encode through a counted
Moria-controlled encoder wrapper; the authoritative
validation/composition/publication path remains GPU-resident.

Access, view bytes/records, proposals, transaction resources, and feedback are
prebounded. A registration DAG supplies adapter-declared ordering without
named phases. Conflicts select or reject whole proposals under an explicit
generic policy. Moria supplies revision binding, receipts, observations,
failure policy, device-generation quarantine, and recovery readiness while
never owning adapter vocabulary or state.

The asynchronous query API and the existing fixed-ABI WGSL job remain
available for tools and nonscheduled consumers. They are explicitly no longer
presented as the complete behavior-engine seam.

Validation now requires independent adversarial CPU physics-shaped,
GPU-physics-shaped, and CPU/GPU damage-and-bond-shaped adapters and directs the
reviewer to try to obtain authoritative storage, force readback onto the GPU
authority path, bypass bounds/publication, or make Moria absorb their state.

### Unresolved question

None. The review supplies sufficient authority to select this technical
contract without adding product behavior.
