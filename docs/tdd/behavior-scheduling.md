# Scheduled External Behavior

## Purpose and authority boundary

This contract is the first-class behavior-engine seam required by the approved
design.
It is distinct from the asynchronous query API and the bounded WGSL job
facility.
A scheduled adapter participates inside a Moria substrate tick against one
stable committed view and returns proposed substrate effects before that
tick's publication boundary.

Moria owns tick admission, view construction, ordering, synchronization,
proposal capacity, validation, conflict resolution, authoritative publication,
receipts, observations, and device lifecycle notification.
An adapter owns its vocabulary, stimuli, algorithms, working state, and any
CPU or GPU resources used to compute its proposals.
Moria never stores or interprets bodies, velocities, forces, joints, damage,
health, resistance, bonds, fracture, gravity, players, or gameplay policy.

Adapters are trusted in-process Rust code but are not storage authorities.
They receive only the authorized exported view and proposal targets described
below.
They never receive Moria page tables, brick slots, scar buffers, revision
gates, render meshes, or a queue on which they can publish Moria work.

## Substrate tick and publication point

A behavior tick is consumer-triggered; Moria does not choose a simulation
frequency or tie it to a camera or wall clock.
At most one behavior tick is active per world in v1.
The accepted request establishes an ordinary-command frontier `F`.
Commands admitted before `F` drain in their normal per-volume order.
Commands admitted after `F`, later checkpoint-frontier capture, and later
volume retirement publication wait behind the active behavior tick.

The tick then follows this state machine:

```text
Queued
  -> WaitingForFrontier(F)
  -> Planning
  -> WaitingForMatter
  -> ExportingStableView(S)
  -> RunningAdapters(S)
  -> ValidatingProposals(S)
  -> ResolvingComposition(S)
  -> PublishingEffects
  -> Reporting
  -> Complete
```

`S` is the coordinator's pinned union of
`(VolumeId, VolumeRevision, RigidPlacement, cell_size, local_domain)` plus the
authorized bounded material records. Each participant receives a distinct
filtered export `S_i` containing only the volumes and cells admitted by that
participant's plan. All `S_i` records refer to the same pinned revisions in
`S`; “same view” means the same commit frontier, not permission to inspect
another participant's scopes.
Moria pins the referenced page versions and placements from
`ExportingStableView` through publication or terminal tick failure.
An earlier adapter's proposals never alter any later `S_i`.
Adapter order can coordinate consumer-owned stimuli or working state, but it
does not create an uncommitted alternate Moria view.

After every selected proposal is prepared, Moria publishes at most one new
revision per affected volume.
All selected matter and placement effects for one volume are one prepared
transaction: they become visible together at that revision or none of them
do.
Different volumes retain the product's independent-publication rule.
Observations are appended before the tick and proposal receipts are awakened.
Only after publication or terminal failure does Moria release `S` and allow
post-`F` authoritative commands to prepare.

Scheduled effects therefore cannot become stale because an ordinary Moria
command interleaved with the tick.
A proposal that names a snapshot member other than the one Moria supplied, or
whose GPU record changes its captured revision, is invalid and is rejected
before publication.
The ordinary asynchronous command path retains its normal stale-precondition
behavior.

## Registration, planning, and access

Behavior adapters register on `MoriaBuilder` before validation.
Each descriptor has:

- a stable `BehaviorEngineKey` and bounded debug name;
- `Cpu` or `Gpu` execution;
- a maximum access envelope and maximum per-tick scopes, volumes, bricks,
  cells, proposals, proposal payload bytes, affected cells/bricks, and
  directory effects;
- zero or more `runs_after` keys;
- a participant failure policy;
- a conflict policy applied when this adapter's later proposal overlaps an
  earlier selected proposal; and
- a readiness policy of `RequireReady` or `Materialize`.

Builder validation resolves every dependency key, rejects self-edges and
cycles, and computes one deterministic topological order.
Unconstrained peers are ordered by stable key bytes.
There is no built-in physics, damage, or other named phase.
An adapter that wants to run after another adapter declares that edge.

At `Planning`, Moria calls each registration's separate host-side
`BehaviorAccessPlanner::plan_tick` in topological order.
Planning fills a Moria-owned exact-capacity `BehaviorAccessSink` inside the
descriptor envelope; it may read the adapter's own host state but cannot
inspect Moria matter.
This permits a CPU adapter and planner to share CPU-owned body state, while a
GPU registration can use a constant conservative planner and keep its detailed
working set GPU-resident.
A fully GPU-resident engine may register a stable maximum envelope and let its
own GPU state discard irrelevant exported records; it is not required to
read back its body list to plan a narrower scope.

Moria resolves each participant's world scopes to a sorted
volume-and-local-brick set at the
frontier placements.
Unlike a long-lived interest/subscription, this resolution is new for every
tick: `allowed_volumes = All` may include volumes live at this tick even if
they were created after adapter registration, but the descriptor and planned
`maximum_volumes` are rechecked before admission.
If the actual set exceeds any declared or configured volume/brick/cell bound,
planning fails; it is never clipped.
With `Materialize`, Moria holds internal authoritative interest until the view
is ready.
With `RequireReady`, cold or failed members produce an unavailable participant
outcome.
Unknown matter is never exported as empty.

The planner and adapter return only fixed-layout `BehaviorAdapterError` values
with an inline diagnostic. No variable scope collection, error string, or
other owned allocation crosses a callback return; incomplete/over-capacity
sink use poisons that participant before view construction.

The exported canonical record is:

```text
BehaviorVolumeRecordV1 {
    runtime_volume_id: u64,
    revision: u64,
    stable_key: [u8; 16],
    translation: [f32; 4],
    rotation_xyzw: [f32; 4],
    cell_size: f32,
    flags: u32,             // v1 zero
    local_domain_min: [i32; 4],
    local_domain_max: [i32; 4], // exclusive
    reserved: [u32; 2],     // v1 zero
}

BehaviorCellRecordV1 {
    volume_index: u32,
    local_x: i32,
    local_y: i32,
    local_z: i32,
    sample: u32,       // material:u16 | coverage:u8 | flags:u8
    occupied: u32,
}
```

Records are sorted by volume stable key and then Z/Y/X cell coordinate.
The view contains every cell in each accepted scope, including empty cells,
so absence of a record is never overloaded as empty matter.
`cell_size` is finite and positive and the half-open domain matches the
registered volume definition. These fields let a late-created volume with a
different metric be interpreted without registration-time hidden metadata.
The descriptor and request byte/count limits bound each `S_i` and their
aggregate before any CPU allocation or GPU copy. No CPU iterator, lookup, or
GPU binding can address a record outside its participant's `S_i`.

## CPU adapter execution

`CpuBehaviorEngine::run_tick` is invoked directly by Moria's behavior
coordinator when the stable CPU view is ready.
It receives a borrowed `CpuBehaviorView`, a borrowed tick context, and a
Moria-owned exact-capacity `BehaviorEffectSink`.
The view offers iteration, exact sample lookup, occupied-cell iteration, and
the same bounded shape/trace/sweep helpers used by public collision queries.
Those helpers run over this immutable exported view and debit the adapter's
declared traversal authorization and per-tick call budget.
They do not consult presentation and do not create an ordinary query receipt.
The coordinator lends one exact-capacity collision sink from the tick
reservation. A call clears and reuses that sink and returns only a borrow
valid until the next call; no `Vec` or other result ownership crosses into the
adapter. A callback may copy facts into its own memory, but repeated calls
cannot increase Moria-owned allocation. Ignored errors poison the sink,
overflow returns no partial contacts, and call/traversal counters are charged
even when the adapter ignores the returned error.

If the authoritative representation is GPU-resident, Moria schedules and
owns the bounded staging readback needed to construct the CPU view.
The adapter is not required to submit or poll a query, and the tick cannot
advance past it until the callback returns or fails.
This readback is a cost of selecting a CPU adapter, not a second authoritative
CPU mirror or a persistent collision cache.

The effect sink is preallocated from the tick's aggregate proposal reservation.
It accepts fixed fill/move/retire values and borrows dense samples or run
records long enough to copy them into already reserved storage.
It assigns `Exact` for the addressed member of `S`.
Scheduled v1 excludes create because its consumer-owned Rust content source is
a control-plane registration, not a stable-view-derived CPU/GPU value;
consumers retain the ordinary create command.
The sink rejects over-capacity records or bytes immediately and poisons only
that participant batch.
The callback cannot return a capacity-bearing effect collection or source
object.

Moria catches an unwinding planner, transition, CPU adapter, report, recovery,
or shutdown hook at its boundary, drops any poisoned sink, and records
`BehaviorEngineFailure::Panicked` when the tick is still active.
It never holds a world lock while calling adapter code.

## GPU adapter execution

`GpuBehaviorEngine` lives in the Bevy render world and uses the renderer-owned
Bevy 0.19/wgpu 29 device generation.
The integration API is deliberately version-coupled and isolated under
`moria::bevy::behavior`; no wgpu type enters the core facade.

On device startup or recovery, Moria calls `create_device_state` with a
restricted `BehaviorGpuResourceFactory`.
The factory exposes only generation-bound opaque behavior buffer, pipeline,
layout, and bind-group handles. It accepts borrowed WGSL and closed buffer/
binding descriptors, validates the scheduled group-0 interface, creates the
resource with Bevy's renderer-owned device internally, and registers its byte
charge and last-use submission. Cumulative borrowed WGSL is charged before
parse to the descriptor/configured scheduled-WGSL limits and retained only as
the backend pipeline requires. A bounded `initialize_buffer` operation copies
borrowed bytes through Moria staging and delays readiness until completion.
It exposes neither `RenderDevice`,
`wgpu::Device`, `RenderQueue`, raw `wgpu` handles, nor encoder creation.
Every resource usable by `BehaviorGpuEncoder` must come from this factory;
external raw handles are not accepted. Moria therefore enforces
`maximum_owned_gpu_bytes` as the sum of requested live buffer allocation
sizes, separate buffer/pipeline/bind-group counts, generation, and destruction
after last use from the registry. Backend-private pipeline allocation size is
not knowable and is bounded by count rather than falsely included in byte
telemetry. `BehaviorResourceReport` is a read-only
registry snapshot/telemetry record computed by Moria, not a trusted adapter
claim. In-process application code may use Bevy elsewhere, but no such resource
can enter this scheduled seam or acquire Moria authority.

For a tick, Moria copies that participant's `S_i` into a private read-only
exported GPU view allocation and
provides:

- the immutable `BehaviorGpuViewV1` bind group and record counts;
- a write-only fixed-schema `BehaviorGpuEffectTargetV1`;
- read-only incoming and write-only outgoing opaque handoff targets;
- the prior terminal feedback for this participant as read-only input;
- the current `DeviceGeneration`; and
- a counted `BehaviorGpuEncoder` controlled by the coordinator.

`encode_tick` may dispatch factory-owned pipelines with the exported view and
factory-owned bind groups, or copy between registered behavior buffers,
through that wrapper.
It cannot access a raw encoder, submit, exceed declared dispatch/workgroup/copy
budgets, remap Moria ranges, or bind Moria's authoritative page, brick, scar,
revision, or mesh buffers.
Consecutive GPU adapters are encoded in declared order in one command stream.
Moria inserts ordered passes and bounded handoff copies between them when an
ordering edge declares an opaque payload.

The GPU effect ABI uses fixed records for fill, run patch, move, and retire.
Each record addresses a supplied snapshot index; Moria supplies the exact
revision and rejects any modified revision word.
Payload ranges, counts, coordinates, material IDs, flags, and reserved words
are validated by Moria GPU kernels.
Validation, conflict resolution, copy-on-write preparation, and publication
remain on the renderer-owned GPU path.
No CPU readback is required to admit, validate, compose, or publish a GPU
adapter's effects.
Small outcome metadata may be read back later to complete CPU-visible receipts
and telemetry, but that readback is not on the tick's authority path.

### Scheduled GPU ABI v1

The scheduled GPU ABI is deliberately smaller than exposing Moria storage.
Moria supplies bind group 0 with exactly five storage bindings:

| Binding | Access | Contents |
| ---: | --- | --- |
| 0 | read-only | this participant's packed stable-view header, volume records, then cell records |
| 1 | read-write | one effect header, fixed proposal records, then payload bytes |
| 2 | read-only | packed incoming opaque-handoff table and payloads, or a valid empty header |
| 3 | read-write | packed outgoing opaque-handoff table and zero-initialized payload capacities |
| 4 | read-only | the participant's prior-feedback header and records, or a typed no-prior-feedback header |

The adapter may use bind groups 1 and above for its own resources.
Moria supplies the bind-group layout and bind group; it never supplies the
underlying authoritative buffers.
The view, effect, and handoff allocations are tick-local and charged to
`behavior_gpu_view_bytes`, `behavior_proposal_bytes`, and
`behavior_handoff_bytes`. Prior/current feedback uses a Moria-owned
double-buffered per-participant allocation charged to
`behavior_feedback_bytes`.
All words are little-endian, offsets are four-byte aligned, and Rust/WGSL
layout assertions are mandatory. Headers use these exact byte offsets:

| Record | Exact fields |
| --- | --- |
| `BehaviorViewHeaderV1` (64 bytes) | `0 magic:u32`, `4 version:u32`, `8 tick:u64`, `16 generation:u64`, `24 engine:u32`, `28 volume_count:u32`, `32 cell_count:u32`, `36 volume_offset:u32`, `40 cell_offset:u32`, `44 total_bytes:u32`, `48 reserved:[u32;4]` |
| `BehaviorEffectHeaderV1` (64 bytes) | `0 magic:u32`, `4 version:u32`, `8 engine:u32`, `12 proposal_capacity:u32`, `16 payload_capacity:u32`, `20 output_proposal_count:u32`, `24 output_payload_bytes:u32`, `28 reserved0:u32`, `32 tick:u64`, `40 generation:u64`, `48 reserved:[u32;4]` |
| `BehaviorHandoffHeaderV1` (64 bytes) | `0 magic:u32`, `4 version:u32`, `8 direction:u32`, `12 engine:u32`, `16 edge_count:u32`, `20 descriptor_offset:u32`, `24 payload_offset:u32`, `28 total_bytes:u32`, `32 tick:u64`, `40 generation:u64`, `48 reserved:[u32;4]` |
| `BehaviorFeedbackHeaderV1` (64 bytes) | `0 magic:u32`, `4 version:u32`, `8 availability:u32`, `12 engine:u32`, `16 source_tick:u64`, `24 generation:u64`, `32 participant_count:u32`, `36 proposal_count:u32`, `40 total_bytes:u32`, `44 reserved:[u32;5]` |

The 112-byte volume record has offsets `0 volume:u64`, `8 revision:u64`,
`16 key:[u8;16]`, `32 translation:[f32;4]`,
`48 rotation_xyzw:[f32;4]`, `64 cell_size:f32`, `68 flags:u32`,
`72 local_domain_min:[i32;4]`, `88 local_domain_max:[i32;4]`, and
`104 reserved:[u32;2]`. The 24-byte cell record has offsets
`0 volume_index:u32`, `4 x:i32`, `8 y:i32`, `12 z:i32`,
`16 sample:u32`, and `20 occupied:u32`. Array strides equal these record
sizes; no implicit `vec3` layout is used.

The 64-byte view header contains magic `MORB`, ABI version 1, tick ID, device
generation, participant engine ID, volume count, cell count, volume-record
offset, cell-record offset, total bytes, and zero-reserved words.
Volume records and cell records have the exact field order and sizes in
[public-api.md](public-api.md#scheduled-behavior-engine-hook).

The 64-byte effect header contains magic `MORE`, ABI version 1, participant
engine ID, proposal capacity, payload capacity, output proposal count, output
payload byte count, tick ID, device generation, and zero-reserved words.
Moria initializes output counts to zero.
Each proposal slot is 128 bytes:

| Byte | Field |
| ---: | --- |
| 0 | kind: unused `0`, fill `1`, patch-runs `2`, move `3`, retire `4` |
| 4 | snapshot index `u32` |
| 8 | exact expected revision `u64` |
| 16 | flags, v1 zero |
| 20 | payload offset |
| 24 | payload byte length |
| 28 | reserved zero |
| 32 | 16-byte correlation |
| 48, 52, 56 | target minimum XYZ |
| 60, 64, 68 | target maximum XYZ |
| 72 | packed material sample |
| 76 | reserved zero |
| 80 | placement translation `[f32; 4]` |
| 96 | placement quaternion `[f32; 4]` |
| 112..127 | reserved zero |

Fill uses target/sample and no payload.
Patch uses the asynchronous extension's 20-byte canonical run record and
X-fastest indexing.
Move uses only placement.
Retire uses only snapshot index/revision/correlation.
The adapter copies the revision from the indexed view record; Moria compares it
to the still-pinned revision and rejects the complete participant batch on any
mismatch.
Create is not representable in scheduled ABI v1 because its required
consumer-owned Rust content-source object cannot cross a scheduled effect
record; a consumer may submit the ordinary `VolumeCommand::Create`.

The 64-byte handoff header uses magic `MORH`, ABI version 1, direction,
participant engine ID, edge count, descriptor offset, payload offset, total
bytes, tick ID, device generation, and zero reserved words. Each 32-byte edge
descriptor contains the peer engine ID, payload offset/capacity, written byte
count, closed status, and zero reserved words. Moria initializes outgoing
payloads to zero; `written_bytes <= capacity` and all reserved/header words are
validated before transfer. Payload bytes are opaque to Moria.

The 64-byte feedback header uses magic `MORF`, ABI version 1, closed availability
(`NoneYet`, `Ready`, or `UnavailablePreviousGeneration`), engine ID, source
tick ID, device generation, participant/proposal counts, total bytes, and zero
reserved words. It is followed by one fixed 32-byte participant record and one
fixed 32-byte record per proposal. Records contain only closed
status/failure/rejection tags, tick/engine/proposal indices, assigned command
ID or zero, and published revision or zero.
After the terminal publication/no-publication decision and all CPU report hooks
return or panic, Moria finalizes the current feedback slot, including any
post-decision notification failure. That slot becomes binding 4 on the
participant's next tick and remains pinned until that submission completes;
only then may the older slot be cleared/reused. The first tick receives
`NoneYet`. Device loss quarantines both old-generation slots and the first tick
after recreation receives `UnavailablePreviousGeneration`, never stale
feedback presented as ready.

The exact 32-byte handoff descriptor offsets are `0 peer_engine:u32`,
`4 payload_offset:u32`, `8 capacity:u32`, `12 written_bytes:u32`,
`16 status:u32`, and `20 reserved:[u32;3]`. The exact 32-byte feedback
participant record is `0 status:u32`, `4 failure:u32`, `8 engine:u32`,
`12 flags:u32`, `16 revision_changed:u32`, `20 cause_engine_or_zero:u32`,
`24 cause_proposal_or_zero:u32`, and `28 reserved:u32`. The exact
32-byte feedback proposal record is `0 status:u32`, `4 reason:u32`,
`8 proposal_index:u32`, `12 reserved:u32`, `16 command_id_or_zero:u64`, and
`24 revision_or_zero:u64`.

ABI tag values are stable: handoff direction incoming `0`, outgoing `1`;
handoff status empty `0`, ready `1`, failed `2`; feedback availability
`NoneYet=0`, `Ready=1`, `UnavailablePreviousGeneration=2`; participant status
completed `1`, skipped `2`, discarded-by-tick `3`; proposal status admitted
matter `1`, admitted volume `2`, rejected `3`; proposal reasons none `0`,
overlap `1`, replaced `2`, invalid `3`, participant-failed `4`,
preparation-failed `5`, and tick-aborted `6`. Failure and tick-abort category
tags are: none `0`; planning `1`, unavailable `2`, access-limit `3`,
effect-limit `4`, invalid-proposal `5`, panicked `6`, GPU-validation `7`,
transition `8`, device-lost `9`, not-ready-generation `10`, shutdown `11`;
tick participant-abort `32`, conflict-fail-tick `33`, transition-failure `34`,
device-lost `35`, and preparation-failure `36`. These constants are shared by
Rust/WGSL layout tests. Unknown tags are validation failures, never forwarded
to an adapter.

GPU adapters may retain their own factory-created GPU working state across
ticks.
Moria neither allocates that state from `extension_state_bytes` nor returns it
as a Moria state lease.
Its semantics are consumer-owned, while allocation is enforced and measured by
the restricted factory registry. Moria does not serialize or interpret the
bytes.

An ordering edge between CPU and GPU adapters is legal. An edge may declare one
opaque handoff capacity; an edge with no handoff is ordering-only.
Before any adapter runs, Moria reserves every edge's host/device/staging
storage and map slot from `behavior_handoff_bytes` and
`behavior_handoff_maps`. CPU callbacks receive borrowed incoming slices and
exact-capacity outgoing writers. GPU shaders receive bindings 2 and 3.

- CPU-to-CPU moves the initialized prefix between Moria-owned host slots.
- CPU-to-GPU validates the written prefix, uploads it to the successor's
  incoming device allocation in queue order, and exposes it only after the
  upload.
- GPU-to-GPU validates the outgoing header on GPU and copies the bounded prefix
  to the successor input before its dispatch.
- GPU-to-CPU copies the bounded prefix to a dedicated staging slot, waits for
  queue/map completion, drops the mapped view before unmap, and invokes the
  successor with a borrow into canonical host storage.

No adapter-owned pointer/buffer crosses processors. The coordinator never
interprets payload bytes. A transition failure is attributed to the successor
and applies that successor's `BehaviorFailurePolicy`; the predecessor remains
honestly executed. Cancellation is already too late after `Planning`, so the
coordinator completes or fails the transfer and releases its reservation.
Device loss terminates every old-generation transfer and no mapped/uploaded
late completion reaches a successor. These transitions are explicit telemetry
events.
A GPU-only chain performs no mandatory CPU readback.

## Proposal admission and composition

Before the tick enters `RunningAdapters`, Moria atomically reserves:

- every participant's declared maximum proposal records and payload bytes;
- the declared aggregate affected cells, bricks, moves, and retires;
- ordinary command completion/observation records for that aggregate maximum;
- worst-case copy-on-write page, brick, scar, and directory transaction
  records for the declared effects; and
- fixed per-proposal outcome records.
- the maximum reusable CPU collision sink, aggregate collision-call counters,
  every declared handoff's host/device/staging bytes and required map slots;
  and
- current/prior feedback slots for every GPU participant.

If the reservation cannot be made, the tick remains queued or fails according
to `behavior_ticks` overload policy; no adapter runs with partial capacity.
Unused proposal and payload capacity is released after validation.

Each participant output is all-or-none at validation.
An invalid record admits no proposal from that participant.
After validation, proposals are considered in adapter order and then proposal
index order.
Two matter proposals overlap when their declared target AABBs intersect in one
volume; a sparse run patch therefore conflicts conservatively across its whole
declared patch bounds.
Two placement/retire proposals overlap when they address the same volume.
A retirement overlaps every proposal for that volume.
Matter and one placement proposal for the same volume do not overlap and may
share the volume's tick revision.
Ordinary create commands remain behind the tick frontier and therefore cannot
interleave with scheduled composition.

The later adapter's declared `BehaviorConflictPolicy` selects one generic
whole-proposal outcome:

- `RejectLater`: reject the entire later proposal;
- `ReplaceEarlier`: reject the entire earlier proposal and select the entire
  later proposal; or
- `FailTick`: fail the tick before any behavior effect publishes.

Partial proposal application is forbidden.
Replacement never cuts a patch into winning and losing cells.
Within one adapter, overlap is always a validation error because its output
order is not an implicit policy language.
This contract provides deterministic composition without assigning physics,
damage, or another semantic order.

The portable GPU resolver stable-sorts fixed proposal summaries by volume,
minimum X, adapter order, and proposal index, then performs a per-volume sweep
over X-active summaries with Y/Z AABB tests.
Move/retire summaries use the closed same-volume rules above.
Every tested pair debits `behavior_conflict_checks`; exhausting it fails the
tick before publication with `BudgetExhausted(BehaviorConflictChecks)`.
The resolver never launches an unbounded pairwise pass or silently assumes
sparse patch runs do not conflict.

Selected proposals receive ordinary `CommandId`s and typed child receipts.
Rejected proposals receive no command ID and a
`BehaviorProposalRejection` in the tick report.
All selected proposals for one volume share the new revision produced by that
volume's tick transaction.
If preparation or publication for one volume fails, every selected proposal
for that volume receives `PreparationFailed { volume }` with no volume revision
change; other volumes remain independent and the tick disposition is still
`Published` if its terminal publication pass completed.

## Failure and adapter-owned state

`BehaviorFailurePolicy` is either:

- `AbortTick`: any planning, availability, CPU callback, GPU validation, or
  device failure discards every participant proposal and publishes no behavior
  effect; or
- `SkipParticipant`: discard that participant's proposals, record its failure,
  and continue with other participants.

If several failures occur and any failed participant selected `AbortTick`, the
tick aborts.
Moria never retries an adapter callback or reruns a shader automatically.

Every admitted tick that enters `Planning` resolves its receipt with a
`BehaviorTickCompleted`, including a no-publication abort; execution failure is
not collapsed into the generic operation error and therefore cannot hide
participant/proposal outcomes. The closed disposition is:

- `Published`: publication processing completed; `revision_changed` says
  whether any volume advanced;
- `NoPublication(ParticipantAbort | ConflictFailTick | TransitionFailure |
  DeviceLost | PreparationFailure)`: no behavior proposal published and
  `revision_changed` is false; or
- `PublishedWithNotificationFailure`: publication already completed and one or
  more post-publication CPU report hooks failed.

Each participant separately records whether it executed, was skipped by its
own failure, or executed/was ready but was discarded by a tick-wide abort.
Every otherwise valid proposal discarded by `AbortTick` or `FailTick` receives
  `TickAborted` with the closed cause. Report hooks run only after the terminal
  publication/no-publication decision. A panic is recorded as
  `FailedAfterTerminalDecision { publication_was_complete }`, does not invoke
  `AbortTick`, does not change proposal receipts, and cannot retroactively
  claim that no effect published. Published ticks with such a failure use
  `PublishedWithNotificationFailure`; no-publication ticks retain their
  original abort cause.

Adapter state is explicitly outside Moria authority:

- **Checkpointing:** Moria checkpoints only substrate revisions and scars.
  The consumer must checkpoint CPU/GPU behavior state in its own store and
  coordinate its frontier with the `BehaviorTickReceipt`.
- **Restore:** restored substrate truth does not imply restored adapter state.
  Each adapter must reconstruct or load its state and report ready before a
  tick including it is admitted.
- **Rejected, conflicted, or tick-aborted proposals:** Moria does not roll back
  adapter state. CPU adapters receive `on_tick_report`; GPU adapters receive
  the bounded prior-feedback binding on their next ordered dispatch.
  The adapter decides whether to roll back, recompute, or accept divergence.
- **Stale revisions:** scheduled proposals are bound to pinned `S`.
  Malformed revision references fail the participant.
  Ordinary later submissions use normal conflict receipts.
- **Device loss:** CPU state remains consumer-owned and untouched.
  A tick without a confirmed revision-gate completion completes as typed
  `NoPublication(DeviceLost)`; a tick whose publication was already confirmed
  retains its published outcome and ordinary dirty-state recovery rules.
  Late old-generation callbacks can never change either result.
  GPU adapter resources and state become invalid; Moria calls
  `on_device_lost`, recreates only its exported-view/effect resources, and
  calls `create_device_state` for the new generation.
  The adapter must restore or reinitialize its own GPU state and report ready.
- **Recovery:** Moria resumes scheduled ticks only after authoritative matter
  and every included adapter report ready for the current generation.
  Adapter recovery failure is an adapter failure, not recovered Moria truth.
- **Shutdown:** new ticks are rejected.
  A queued/waiting tick may be cancelled only before its atomic transition to
  `Planning`, which is the behavior family's `Preparing` boundary; a tick that
  entered planning drains to report or terminal device failure.
  Moria calls adapter shutdown hooks after the last report, but does not save
  or discard consumer state on the consumer's behalf.

The CPU report and GPU feedback contain tick ID, stable snapshot revisions,
participant status, proposal selection/rejection, assigned command IDs,
terminal revision when known, and failure category.
They contain no behavior-specific payload.

## Adversarial integration cases

A conventional CPU physics adapter may keep bodies, velocities, forces,
joints, solver caches, and policy in its own Rust state.
It plans bounded body regions, receives a direct borrowed tick view, runs its
solver without a query receipt, updates its state, and proposes movement or
matter edits through the exact-capacity sink.

A GPU-resident physics adapter may keep all of those concepts in its own
factory-created opaque renderer-device resources.
It reads the exported stable material/occupancy bind group, runs its pipelines
on Moria's ordered encoder, updates its own buffers, and writes proposal
records.
Moria validates and publishes them without reading material or solver state
back to the CPU.

A CPU or GPU damage-and-bond adapter follows the same contract.
It owns accumulation, bond strength, impacts, fracture, and crumbling rules;
Moria sees only the access request, material view, and proposed fill/patch,
move, or retire effects.
If it consumes impact data produced by a physics adapter, both adapters share
an ordering edge with an opaque bounded handoff;
Moria transports bytes but neither defines nor interprets the impact
vocabulary.

An independent reviewer must implement or mock the CPU physics, GPU physics,
CPU damage-and-bond, and GPU damage-and-bond variants through the public
adapter traits and attempt to disprove:

1. CPU participation without an ordinary query/receipt loop;
2. GPU participation and effect publication without material/effect CPU
   readback on the authority path;
3. adapter-owned state across success, rejection, checkpoint, restore, device
   loss, recovery, and shutdown;
4. deterministic multi-adapter ordering and whole-proposal conflict outcomes;
5. inability to obtain a render device, raw resource/encoder, submit
   independently, read another participant's export, or bind/mutate
   authoritative Moria storage; and
6. absence of physics, damage, bond, or gameplay fields in Moria types.

Any counterexample blocks the behavior-hook architecture claim.
