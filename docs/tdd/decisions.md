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

**Outcome contract.** Every tick that enters the behavior family's `Preparing`
stage returns a closed completed report, including a GPU-input preflight abort
before planning. Tick-wide aborts give every discarded valid proposal a typed
cause and `revision_changed = false`; a post-publication report-hook failure is
reported without undoing or relabeling publication.

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
behavior seam. It serializes a consumer-triggered tick around an ordinary
command frontier, drains pre-frontier work, and pins one committed substrate
view through behavior publication or terminal failure.
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

---

## T31. Scheduled ticks own bounded participant-addressed consumer ingress

**Decision.** Each behavior descriptor declares `None | Optional | Required`
opaque current input and a maximum of at most 1 MiB. One tick request may
carry one exact byte slice per participant. Its permit reserves every declared
record, host byte, and GPU upload/device maximum before any planner runs.
Planners and CPU callbacks borrow the same immutable bytes. GPU participants
receive those bytes through ordered, completion-confirmed upload to read-only
group-0 binding 5.

**Failure contract.** Unknown/stale participants, duplicates, input supplied
to `None`, missing required input, and capacity overflow reject synchronously
with the request unchanged. Cancellation releases ingress before planning.
Upload failure or device loss is terminal before any adapter executes. Moria
does not interpret a timestep, force, body, control, damage, or any other
vocabulary.

**Reason.** Adapter-to-adapter handoffs cannot provide current consumer input
to the first participant. A participant-addressed ingress removes dummy
predecessors and shared-state side channels while keeping bytes, allocation,
GPU visibility, and failure within the existing tick admission boundary.

**Rejected.** Unbounded request blobs, a fake source adapter, undocumented
global state, raw GPU upload handles, and input readback on the authority path.

## T32. Scheduled v1 has controlled GPU adaptation, no create/split, and synchronous CPU execution

**Decision.** The GPU trait is for a purpose-built or substantially adapted,
Moria-conforming adapter using only the restricted factory, fixed six-binding
group-0 ABI, counted encoder, and Moria-owned submission. It is not a drop-in
interface for arbitrary pre-existing engine resources or command submission.
Scheduled effects remain fill, patch, move, and retire; create and atomic
volume splitting are unavailable. CPU planners/adapters execute synchronously
on the Bevy main thread while the frontier is held.

**Consequences.** A fracture/debris-shaped adapter can edit, move, or retire
existing volumes, but newly created independent volumes require later ordinary
control-plane commands and cannot be claimed as part of the tick transaction.
A slow CPU adapter stalls the main-world update; P10 is a fixed feasibility
gate, not a latency promise for arbitrary consumer code. `StorageRead` factory
buffers include `COPY_DST` so their documented staging initialization is legal
while shader access remains read-only.

**Rejected.** Passing `BaseContentSource` through the scheduled wire ABI,
inventing a worker architecture, accepting raw external GPU resources, and
creating a staging-copy destination without `COPY_DST`.

---

## T33. Scheduled GPU input preflight precedes all consumer code

**Decision.** After the captured command frontier drains, a behavior tick
atomically enters `Preparing` by uploading and confirming every GPU
participant's current input before invoking any access planner. This transition
is the cancellation point of no return. A non-device upload failure identifies
the first failed participant in stable order as
`Skipped(ConsumerInputUpload)` and marks every other participant
`NotRun(InputPreflightAborted { failed_engine })`; device loss marks every
participant `NotRun(DeviceLost)`. No planner, adapter, or report hook runs on
either preflight-abort path.

**Report and ABI consequence.** Because preflight has entered `Preparing`, its
receipt returns `BehaviorTickCompleted` with empty snapshot, proposal, and
published vectors rather than a generic receipt error. Every participant's
publication is `DiscardedByTick` and notification is `NotApplicable`.
Scheduled ABI v1 adds execution tag 3 (`not-run`) and failure tag 13
(`input-preflight-aborted`), with the failed engine in field A. P10 fixes exact
patch-run geometry, bytes, affected resources, conflict outcome, revision
vector, and its two distinct registered oracle samples as packed values
`0x00FF0001` and `0x00FF0002`, so the selected mixed publication path has one
reproducible gate.

**Reason.** Consumer planners are mutable adapter code. Uploading after planning
cannot satisfy the protected fail-before-execution requirement, and reusing an
own-failure `Skipped` outcome for unaffected participants would misreport what
ran.

**Rejected.** Planning before upload confirmation, invoking report hooks on a
preflight abort, attributing another participant's upload failure to every
participant, or leaving P10 effect kinds and mutation scale harness-defined.

---

## Human review entry — bounded behavior ingress and scheduled boundary

### Verbatim feedback

```text
TamedTornado (COMMENTED):
The TDD is close, but one contract gap and several smaller defects need another revision before approval.

## Human decision: purpose-built GPU adapters are sufficient

Moria does **not** need to import arbitrary pre-existing GPU physics engines, raw external GPU resources, or an engine-owned command/submission model. We expect a GPU behavior engine to be written for Moria or substantially adapted to Moria's restricted factory, fixed group-0 ABI, and counted encoder. Preserve that controlled boundary.

Please make the prose precise: this supports an independently implemented, Moria-conforming GPU behavior adapter. Do not imply drop-in integration with an arbitrary existing GPU engine, and do not weaken the no-raw-device/resource/encoder guarantees.

## Required: bounded consumer-to-adapter input on every tick

The architecture says consumers may provide consumer-owned stimuli before requesting a behavior tick, but the callable contract does not provide that route:

- `BehaviorTickRequest` contains only `correlation`;
- `BehaviorPlanContext` contains only tick and correlation;
- the GPU tick context exposes the substrate view, prior feedback, adapter handoffs, and encoder, but no current consumer input; and
- existing handoffs only connect registered adapters.

The first CPU or GPU adapter therefore has no defined, admitted route for timestep/step parameters, forces, impulses, runtime body changes, control input, damage events, or other consumer-owned stimuli. Requiring a fake predecessor adapter or an undocumented shared-state side channel is not acceptable.

Add a generic bounded ingress contract:

1. Each participant may declare a maximum opaque per-tick consumer-input capacity.
2. `BehaviorTickRequest` supplies per-participant opaque inputs.
3. `BehaviorTickPermit` reserves all input records/bytes before any planner or adapter runs.
4. CPU participants receive a borrowed immutable input slice.
5. GPU participants receive the same bytes through an ordered upload and read-only binding. Reusing the existing incoming-handoff transport with a reserved consumer-ingress source is acceptable if the ABI and ownership remain unambiguous.
6. Moria must not interpret time, forces, damage, bodies, or any other input vocabulary.
7. Missing required input, unknown participant, duplicate input, over-capacity input, cancellation, upload failure, and device loss must have closed fail-before-execution outcomes.
8. Validation must vary input across ticks and prove that both the first CPU adapter and the first GPU adapter consume it without a dummy participant, hidden allocation, raw GPU access, or authority-path readback.

## Scheduled create/split limitation

Scheduled v1 currently supports fill, patch, move, and retire but excludes create. That means fracture can remove or edit matter, but cannot atomically turn one volume into newly created independently moving volumes.

Do not silently promise that scheduled adapters can propose “any substrate changes.” Unless a small bounded data-only create effect fits the existing volume/content/persistence authority cleanly, explicitly narrow the v1 contract: scheduled volume creation/splitting is not atomic and remains a later ordinary control-plane operation. State the consequence for fracture/debris adapters and add a validation assertion so implementation cannot accidentally overclaim it. Do not transport a Rust `BaseContentSource` through the scheduled ABI merely to make the wording pass.

## Concrete contract repairs

1. `BehaviorGpuBufferUsage::StorageRead` maps to `STORAGE | COPY_SRC`, while `initialize_buffer` says it initializes through a staging copy. A copy destination requires `COPY_DST`. Correct the usage or define a distinct legal creation-time initialization mechanism, and test the backend usage flags.
2. `behavior-scheduling.md` says the stable view contains “authorized bounded material records,” but the documented CPU/GPU ABI contains volume and cell/sample records, not a material-definition table. Correct the wording or define the missing table. Consumer-owned behavior properties must remain outside Moria.
3. Repair the truncated sentence in `decisions.md` ending with “around an ordinary-”.
4. The CPU adapter currently executes directly from the main-world coordinator and can stall the Bevy main thread while the frontier is held. Make that v1 execution/latency choice explicit and add a CPU or mixed-adapter feasibility scenario. Do not invent a worker architecture solely to avoid documenting the chosen behavior.

Update architecture, public API, lifecycle, decisions, ABI text, configuration bounds, and validation consistently. The reviewer should specifically try to disprove bounded first-participant ingress and verify that the purpose-built GPU adapter boundary remains restricted.
```

### Technical decision and clarification

T31 adds one optional or required opaque input record per registered
participant. `BehaviorTickRequest` owns exact participant-addressed slices;
`BehaviorTickPermit` reserves the checked sum of all descriptor record/host
byte and GPU ingress maxima before planning. The planner and CPU callback
borrow the same immutable bytes. GPU participants receive them only through
ordered, completion-confirmed upload to the new read-only scheduled group-0
binding 5. Structural input errors reject without a tick; cancellation,
upload failure, and device loss have closed no-adapter-execution outcomes.
Moria assigns no behavior vocabulary to these bytes.

T32 preserves the controlled GPU authority boundary as the human directed.
The supported case is an independently implemented adapter written for or
substantially adapted to Moria's restricted factory, fixed six-binding group-0
ABI, counted encoder, and Moria-owned submission. No raw external GPU
resource/device/encoder or arbitrary engine-owned submission path was added.

Scheduled v1 remains fill, patch, move, and retire only. Fracture/debris-shaped
adapters cannot atomically create split volumes; later ordinary create commands
have separate sources, admission, receipts, and revisions. The CPU callback is
explicitly synchronous on the Bevy main thread while the frontier is held, and
P10 is the blocking fixed CPU/mixed feasibility workload. `StorageRead` now
includes `COPY_DST` for legal staging initialization; the stable-view wording
now names only volume and cell sample/occupancy records; and the prior
truncated decision sentence is complete.

### Unresolved question

None. The human feedback resolves the consequential GPU integration boundary,
and the remaining choices are fully specified engineering contracts.

---

## Human review entry — fracture, multi-fidelity simulation, and adapter egress

### Verbatim feedback

```text
The TDD is much stronger, but our follow-up architecture review exposed three required capabilities that need to be designed before approval. These are substrate/adapter contracts, not permission to add physics, damage, weapon, or gameplay semantics to Moria.

## 1. Required: atomic GPU-resident fracture into child volumes

Scheduled behavior must be able to split existing authoritative matter into new independently placed Moria volumes without a CPU readback or later host-authored `VolumeCommand::Create`.

This is **not** arbitrary scheduled creation and must not transport a Rust `BaseContentSource` through the GPU ABI. The source matter already exists inside Moria. Define a bounded, data-only operation that extracts GPU-discovered components from one pinned source volume into child volumes.

The contract must cover:

- pre-reservation of every possible child volume, directory entry, page/cell transfer, identifier, proposal record, and byte before execution;
- an all-or-none publication in which source removal and child creation become visible atomically, with no duplicated, lost, or temporarily ownerless matter;
- child local frames, initial placements, inherited cell size/material facts, persistence/provenance, and later rematerialization;
- proposal-local temporary piece handles and a GPU-visible mapping to final `VolumeId`s so the physics adapter can associate its new bodies without authority-path CPU readback;
- cancellation, validation failure, allocation failure, device loss, and complete resource reclamation;
- adapter-owned policy for which connected components become persistent Moria volumes versus transient debris or visual effects. Moria bounds and executes the choice but does not define “significant debris.”

Validation must demonstrate a GPU-discovered fracture that atomically converts one existing volume into multiple independently moving child volumes, preserves exact matter ownership, and returns usable child identities to GPU-owned adapter state.

## 2. Required: CPU-defined activity regions with multi-fidelity world simulation

The CPU/game layer defines one or more physics activity regions; the GPU does **not** choose where important gameplay regions are. Region definitions arrive through current-tick consumer input.

However, leaving every activity region must not mean that an object freezes or disappears. The intended model is one persistent world simulation with:

- coarse motion/world simulation outside activity regions;
- full collision, constraints, damage, and rigid-body physics inside them;
- a transition halo that permits continuous promotion into and demotion out of full physics;
- GPU-side classification of persistent bodies against the CPU-supplied regions, without reading the body list back to the CPU;
- continued coarse movement of ships and other significant objects outside the bubble, plus coarse remote destruction/debris outcomes where applicable.

Multiple disconnected regions must be supported. Overlapping regions must form a deterministic union: no voxel record or body may be processed twice because two player bubbles overlap. A projectile or other body crossing between regions must remain one continuously owned body; geographic regions are not separate behavior adapters or separate physics worlds requiring an accidental cross-adapter migration protocol.

Select and document a bounded placement-authority/update mechanism for coarse objects. Do not assume that publishing one ordinary scheduled move proposal for every coarse object on every full-physics tick will scale without proving it, and do not silently make Moria placements stale. Likewise, assess whether the fixed maximum-dispatch model is sufficient after GPU active-list compaction; if it is, prove the resource/performance bound, and if it is not, define the smallest controlled dispatch mechanism required.

Validation must include disconnected and overlapping CPU-defined regions, one-time processing in their overlap, continuous boundary crossing, promotion/demotion without transform or velocity discontinuity, and continued coarse motion outside every full-physics region.

## 3. Required: bounded opaque GPU-to-CPU adapter egress

GPU behavior adapters need an optional, bounded GPU-to-CPU egress channel for adapter-owned events and query outcomes. This is generic transport, not a Moria gameplay-event model.

The adapter owns the byte/record schema and may use it for facts such as significant collisions, projectile or beam impacts, destruction, scoring inputs, audio cues, or other consumer-defined results. Moria must not define or interpret those concepts.

The transport contract must provide:

- an optional per-adapter declared maximum record/byte capacity, with zero meaning no CPU egress;
- reservation and accounting before execution;
- an adapter-written initialized prefix and asynchronous ordered staging readback;
- tick and correlation identity sufficient for the owning consumer to decode the result;
- exact delivery or an explicit overflow/failure outcome—never silent truncation;
- defined cancellation, shutdown, device-loss, mapping/readback-failure, and resource-reuse behavior;
- no raw device, queue, mapped authority resource, or full solver-state readback;
- no requirement to route GPU-to-GPU data through the CPU; existing handoffs remain the appropriate path for GPU consumers.

Publication authority must not depend on CPU interpretation of these bytes. Specify the ordering between publication, egress availability, receipts, and buffer reuse so consumers can distinguish “no events” from “events unavailable or lost.”

Validation must round-trip an adapter-defined record layout that Moria does not understand, prove zero-event behavior, exercise exact capacity and overflow, and cover cancellation/readback failure/device loss without leaking or silently dropping required evidence.

## Cross-cutting revision

Update the public API, behavior scheduling, GPU ABI, lifecycle, resource limits, persistence/identity rules, receipts, decisions, and validation consistently. The adversarial reviewer should specifically try to disprove:

- atomic matter ownership during fracture;
- child-ID feedback without CPU authority-path readback;
- continued coarse simulation outside CPU-defined regions;
- overlap deduplication and boundary continuity;
- honest opaque-event delivery and overflow behavior; and
- the absence of new physics, damage, weapon, or gameplay semantics in Moria itself.
```

### Technical decision and clarification

T34 supersedes T32's no-create/no-split limitation only for one closed
source-bound operation. `ExtractComponents` redistributes occupied samples
from one pinned source into pre-reserved dynamic child identities under a
single `WorldDirectoryEpoch` root gate. It cannot invent samples, attach a
consumer content source, or perform arbitrary scheduled creation.
Proposal-local piece handles occupy a descriptor-bounded dense range; the
canonical reservation subrange contains the complete proposal-slot by
piece-handle map to final `VolumeId`s before the adapter dispatch. The adapter
selects which labeled
pieces become persistent children or explicit removal; Moria does not define
connectivity significance or transient-debris policy.

T35 selects one persistent adapter/body table for all CPU-authored activity
regions. Region bytes remain opaque current input. The adapter classifies each
body once against the deterministic region union, compacts mutually exclusive
full/halo/coarse lists, keeps coarse simulation running outside every region,
and publishes changed poses through one GPU placement stream and directory
epoch rather than per-object host move commands. The fixed-dispatch baseline
is retained with a declared 65,536-body, 16-dispatch/8,192-workgroup bound
and blocking P11 evidence.

T36 adds optional fixed-stride opaque CPU egress in a dedicated effect-buffer
subrange. Every enabled adapter declares exact record/byte maxima; tick
admission reserves device, staging, host, map, and receipt capacity before
execution. Publication may complete before asynchronous egress mapping. Zero
records is a successful empty result, while overflow, mapping, decode,
shutdown, cancellation, and device loss are explicit terminal results with no
truncated prefix.
GPU-to-GPU handoffs remain on the existing GPU path.

Scheduled ABI v2 therefore retains exactly six group-0 bindings and is the
only scheduled ABI accepted by the initial implementation. The proposal,
child-reservation, and egress sections are independently bounded within
binding 1. The new types remain generic substrate records. Moria does not
acquire a timestep, region, velocity, physics, collision-response, damage,
weapon, debris, scoring, audio, or gameplay-event model.

### Unresolved question

None. The human feedback explicitly authorizes these substrate/adapter
contracts while retaining the approved behavior-policy boundary. All remaining
choices are specified engineering mechanisms and validation obligations.

---

## T34. Source-bound component extraction uses one directory-epoch gate

**Decision.** Scheduled ABI v2 pre-reserves every possible child identity,
directory/lifetime entry, transfer/page/brick/scar/provenance record, proposal,
receipt, and byte before GPU execution. An extract-components proposal labels
samples from exactly one pinned source. The per-proposal child maximum defines
legal handles `1..=maximum`; registration checks the proposal-count product,
and the adapter sees that complete dense map before dispatch. Moria builds a
new source and children in unreferenced storage and installs the complete
directory root with one checked epoch gate.
Each proposal derives candidate stable keys from a deterministic preflight
nonce, dense slot, and bounded collision-retry salt; only published keys
become lifetime records.

**Frame and persistence.** A child's origin is the lexicographically smallest
assigned source cell; axes/cell size/material samples are inherited and its
initial placement preserves every transferred world-space cell box.
Persisted derived provenance and a complete sparse derived base replace a
consumer content source. Uncheckpointed derived content remains dirty and
subject to the existing unrecoverable-device-loss rule.

**Reason.** A later ordinary create cannot provide atomic ownership or a GPU
child identity. Arbitrary scheduled create would require behavior-authored
content and a source object. Source-bound transfer plus a directory root gives
the required capability without widening that authority.

**Rejected.** CPU component readback, host-authored follow-up create, copying
source cells into both parent and child, publishing directory entries
individually, implicit deletion of unassigned cells, and GPU transport of
`BaseContentSource`.

## T35. Persistent multi-fidelity adapters use opaque CPU regions and a placement stream

**Decision.** The CPU supplies region definitions through ordinary opaque
participant input. One GPU adapter owns one persistent body table across all
regions, classifies every body once into mutually exclusive adapter-owned
full/halo/coarse lists, continues coarse work outside all regions, and compacts
changed Moria placements into one bounded placement stream. The stream
validates on GPU and publishes one alternate directory root while each updated
volume advances its revision. A behavior-only `VolumeRecords` scope exports
the required placement/revision records without widening the ordinary
256-volume query filter or exporting unused cells.

**Dispatch.** The portable baseline keeps fixed maximum dispatch:
65,536 bodies at width 128. The proof adapter executes exactly 11 dispatches
and at most 3,604 workgroups against declared maxima 16/8,192. P11 at empty,
1%, 50%, and 100% active lists is blocking.

**Reason.** Geographic adapters/worlds would require accidental state
migration and duplicate overlapping-region processing. Ordinary move
admission per coarse object would force host enumeration and queue overhead.
One table, deterministic union, halo, and compact directory update preserve
continuous ownership and fresh Moria placements.

**Rejected.** GPU-selected activity regions, freezing objects outside all
regions, one adapter per bubble, duplicate body records in overlaps, silent
stale placement, an unproven per-object host command loop, and raw indirect
dispatch buffers.

## T36. Adapter egress is bounded opaque transport independent of publication

**Decision.** A GPU adapter may declare zero egress or one fixed-stride schema
and exact record/byte maximum. Moria initializes a dedicated egress subrange in
Scheduled ABI v2 binding 1. The adapter reserves records with the supplied
atomic helper; Moria
validates the complete required count, copies only a valid initialized prefix,
maps asynchronously, and returns a tick/participant/correlation/schema-bearing
receipt in adapter tick order.

**Outcome and lifetime.** Zero records is ready-empty. Exact capacity succeeds.
One-over or an overflow flag fails with no delivered prefix. Publication does
not depend on CPU decoding and may be reported while egress is pending.
Working, staging, and host bytes release only after their respective GPU-use,
map/view-drop/unmap, and result-drop milestones. Cancellation, shutdown,
mapping, decoding, and device loss are distinct terminal outcomes.

**Reason.** Adapters need generic CPU-visible facts without teaching Moria an
event vocabulary or reading back full solver state. A separately bounded
receipt makes no-events distinguishable from unavailable/lost events and keeps
GPU handoffs on the GPU path.

**Rejected.** Unbounded append, silent truncation, interpreting adapter
records, tying publication authority to CPU event decoding, returning mapped
authority resources, and routing GPU-to-GPU consumers through CPU egress.

---

## Human review entry — simplest sufficient adapter design

### Verbatim feedback

```text
Is this as simple as it can be while still satisfying the requirements? If yes, leave the TDD unchanged. If no, revise the TDD to make it the simplest sufficient design.
```

### Technical decision and clarification

The TDD was not yet as simple as it could be.
The three required adapter capabilities remain unchanged, but the focused
adapter document is now a delta over the already selected scheduled-tick,
copy-on-write, resource-pool, receipt, observation, and persistence contracts.
It no longer restates whole scheduling, storage, lifecycle, persistence, and
validation subsystems or makes the proof adapter's region schema,
classification vocabulary, and exact scan/dispatch decomposition part of
Moria's architecture.

The smallest sufficient substrate additions are:

1. one source-bound extraction proposal with pre-reserved final child
   identities and one atomic directory publication gate;
2. one bounded placement-stream proposal using the same directory gate; and
3. one optional fixed-stride egress range using the existing asynchronous
   staging/readback lifecycle.

T34-T36 remain authoritative for ownership, atomicity, boundedness,
persistence, and failure behavior.
Their fixed workloads and implementation counts are qualification fixtures,
not additional public product behavior or a mandate that consumer adapters use
the proof implementation.

### Unresolved question

None.
The review calls for engineering simplification and does not require a new
human product or authority decision.

## T37. Adapter capabilities reuse existing substrate machinery

**Decision.** Component extraction, bulk placement, and opaque egress are
extensions of one scheduled tick.
They reuse its permit, participant effect allocation, copy-on-write
transactions, configured pools, receipts, observations, and device lifecycle.
There is no separate fracture service, activity-region subsystem,
multi-fidelity scheduler, event model, or egress runtime.
The activity-region and fidelity proof remains adapter-owned opaque data and
code.

**Reason.** The public guarantees require atomic ownership transfer, fresh
placements, bounded readback, and explicit failure.
They do not require Moria to duplicate the surrounding scheduling and storage
architecture or standardize the proof adapter's internal algorithm.

**Rejected.** Removing any of the three required hooks; weakening
pre-reservation, atomic publication, coarse-outside-region proof, or egress
failure honesty; retaining duplicate normative descriptions; and promoting
proof-only region or simulation concepts into Moria.

## T38. Directory allocator closure is an operational and durable capability state

**Decision.** Directory-epoch allocation starts at one, never wraps or reuses,
and closes permanently after consuming `u64::MAX` or failing a checked
multi-root range reservation. Closure is a sticky bit independent of the
current root epoch. The current root, queries, matter mutation, ordinary
single-volume movement, observations, checkpoints, non-root scheduled work,
and shutdown remain usable; root publication and both new interest declaration
and existing-interest update are closed by the exact public admission matrix.
Existing interest inspection and withdrawal remain usable, so closure freezes
rather than leaks the already admitted residency set.

Checkpoint format v2 stores closure as
`DIRECTORY_ALLOCATOR_CLOSED` independently of `directory_epoch`. Restore with
that flag succeeds into `WorldState::DirectoryEpochExhausted`, even for a
lower epoch left by failed range reservation. Device recovery retains the same
bit and returns to the exhausted state rather than `Ready`.

**Reason.** Numeric epoch alone cannot reconstruct a failed range reservation
that closes below `u64::MAX`. Treating exhaustion as `Failed` would discard
truthful current-root read/checkpoint capability, while treating it as `Ready`
would reopen an allocator whose ordering domain is exhausted.

**Rejected.** Epoch wrap/reuse; inferring closure only from
`epoch == u64::MAX`; reopening after restore or recovery; failing the entire
world; and leaving permit/admission behavior implementation-defined.
