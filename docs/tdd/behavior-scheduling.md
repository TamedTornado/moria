# Scheduled External Behavior

## Purpose and authority boundary

This contract is the first-class behavior-engine seam required by the approved
design.
It is distinct from the asynchronous query API and the bounded WGSL job
facility.
A scheduled adapter participates inside a Moria substrate tick against one
stable committed view and returns proposed substrate effects before that
tick's publication boundary.
The GPU surface supports an independently implemented, Moria-conforming
behavior adapter that is purpose-built or substantially adapted to the
restricted factory, fixed group-0 ABI, counted encoder, and Moria-owned
submission lifecycle. It is not a drop-in import seam for an arbitrary
pre-existing GPU engine, its raw external resources, or its engine-owned
command/submission model.

Moria owns tick admission, view construction, ordering, synchronization,
proposal capacity, validation, conflict resolution, authoritative publication,
receipts, observations, and device lifecycle notification.
An adapter owns its vocabulary, stimuli, algorithms, working state, and CPU
resources used to compute its proposals. GPU state participating through this
seam is consumer-owned in meaning but must be allocated through Moria's
restricted factory.
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
OwnedRequest
  -> ValidatingConsumerInputs
  -> Queued
  -> WaitingForFrontier(F)
  -> UploadingGpuInputs
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
authorized bounded volume records and cell sample/occupancy records. It
contains no material-definition table and no consumer-owned behavior
properties. Each participant receives a distinct
filtered export `S_i` containing only the volumes and cells admitted by that
participant's plan. All `S_i` records refer to the same pinned revisions in
`S`; “same view” means the same commit frontier, not permission to inspect
another participant's scopes.
Moria pins the referenced page versions and placements from
`ExportingStableView` through publication or terminal tick failure.
An earlier adapter's proposals never alter any later `S_i`.
Adapter order can coordinate consumer-owned stimuli or working state, but it
does not create an uncommitted alternate Moria view.

Each participant may declare one opaque per-tick consumer input with a fixed
maximum and `None | Optional | Required` policy. The accepted tick owns at
most one exact byte slice per participant. Moria does not parse or classify
those bytes as time, steps, forces, impulses, bodies, controls, damage, or any
other vocabulary. This is a consumer-to-participant ingress, not an adapter
edge: the first participant in the DAG receives it without a dummy
predecessor. The participant's planner and CPU callback borrow the same
immutable slice; a GPU participant receives those bytes through the ordered
binding-5 upload defined below.

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
- a `None | Optional | Required` opaque consumer-input policy and maximum
  bytes;
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
The planner also borrows its participant's current consumer-input bytes, so
input-dependent access planning needs no shared-state side channel.
The paired presence boolean distinguishes an omitted optional input from a
present empty slice; required input is validated as present before planning.
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

Before `Planning`, submission validates the entire input set against the
registered table and the already-held tick permit. Unknown or stale
participants, duplicates, input supplied to `None`, missing `Required` input,
per-participant overflow, or aggregate record/byte overflow rejects the
request synchronously and returns it unchanged. No tick ID is assigned and no
planner or adapter runs. Accepted slices are immutable and charged until their
last CPU borrow or GPU upload use. After the command frontier drains, the
coordinator atomically transitions the tick from cancellable
`WaitingForFrontier` to `Preparing`, whose first family-specific stage is
`UploadingGpuInputs`. Cancellation that wins before that transition drops the
request and releases host, staging, device, and record permits before its
terminal result is visible. Once upload preflight starts, cancellation is too
late even though no consumer code has run; Moria must retain each submitted
range through completion and produce the report defined below.

The exported canonical record is:

```text
BehaviorVolumeRecordV1 {
    runtime_volume_id: { low: u32, high: u32 },
    revision: { low: u32, high: u32 },
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
The tick context contains that participant's immutable current consumer input.
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

V1 invokes both `plan_tick` and `run_tick` synchronously on the Bevy main
thread. The committed frontier and post-frontier command barrier remain held
for the complete callback. Moria supplies no preemption, deadline, or worker
offload: a slow or blocked CPU adapter stalls that main-world update and
extends the behavior tick. The consumer must keep its callback within its own
frame/update budget; the mixed feasibility gate measures Moria overhead and a
fixed proof adapter but is not a latency guarantee for arbitrary consumer
algorithms. Moving CPU adapter execution to workers would require a later
threading/state-ownership contract and is not implied by this TDD.

The effect sink is preallocated from the tick's aggregate proposal reservation.
It accepts fixed fill, patch, move, and retire values and borrows dense samples
or run records long enough to copy them into already reserved storage.
It assigns `Exact` for the addressed member of `S`.
Scheduled v1 excludes create because its consumer-owned Rust content source is
a control-plane registration, not a stable-view-derived CPU/GPU value;
consumers retain the ordinary create command.
Therefore a scheduled fracture/debris-shaped adapter can remove, patch, move,
or retire existing volumes but cannot atomically split one volume into newly
created independently moving volumes. Creation is a later ordinary
control-plane operation behind the tick frontier, with its own admission,
receipt, source registration, and revision ordering. No Rust
`BaseContentSource` or source descriptor is transported through the scheduled
effect sink or GPU ABI.
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
An engine must implement or substantially adapt an adapter to this contract.
An arbitrary engine that requires its own device, raw buffers, command encoder,
submission queue, or nonconforming group-0 layout is not compatible without
that adaptation.

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
sizes and also charges the world-wide effective
`behavior_gpu_buffer_bytes` pool shared by every adapter. Builder registration
requires the checked sum of descriptor maxima to fit the requested desired
pool; startup repeats the check against the adapter-clamped effective pool
before any adapter creates device state.

Each buffer creation atomically reserves adapter bytes, aggregate bytes, and
one handle before invoking the renderer. Logical capacity failure rejects
without a backend allocation; renderer OOM releases the reservation and
registers no handle. Dropping an adapter handle only prevents new use.
Referenced bind groups and the last GPU submission keep the bytes charged
until their dependencies and use complete. Device-loss recovery destroys and
uncharges the terminal generation before replacement resources are admitted.
Moria exposes aggregate current/high-water/limit/rejection telemetry in
`BehaviorGpuBufferBytes` and per-adapter usage in
`BehaviorResourceReport`.

Separate buffer/pipeline/bind-group counts, generation, and destruction after
last use remain registry-enforced. Backend-private pipeline allocation size is
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
- the immutable current opaque consumer input uploaded into the dedicated
  ingress binding;
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

Consumer ingress is independent of handoffs. Before `Planning`, Moria
records every GPU participant input as a staging copy into its dedicated
tick-local device range, validates the complete header/range set, and submits
the uploads in participant order on the renderer queue. It waits
asynchronously for every completion before invoking the first planner; no Bevy
schedule blocks on that wait. The corresponding read-only binding remains
valid through that participant's later dispatch. Moria preflights every upload
before any consumer planner, CPU callback, report hook, or GPU dispatch starts.
The first non-device upload error in stable participant order produces
`NoPublication(PreparationFailure)` and records `ConsumerInputUpload` for that
addressed participant; every other participant is explicitly not run because
of that tick-global preflight failure. Device loss produces the existing
`NoPublication(DeviceLost)` and marks every participant not run. Neither path
invokes consumer code.
For a CPU-only tick, the empty GPU upload set confirms synchronously at the
same `Preparing` boundary and planning may follow in the same main-world
update.
No input is recovered through readback, and an adapter cannot mutate
or retain Moria's ingress allocation after the consuming submission.

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
Moria supplies bind group 0 with exactly six storage bindings:

| Binding | Access | Contents |
| ---: | --- | --- |
| 0 | read-only | this participant's packed stable-view header, volume records, then cell records |
| 1 | read-write | one effect header, fixed proposal records, then payload bytes |
| 2 | read-only | packed incoming opaque-handoff table and payloads, or a valid empty header |
| 3 | read-write | packed outgoing opaque-handoff table and zero-initialized payload capacities |
| 4 | read-only | the participant's prior-feedback header and records, or a typed no-prior-feedback header |
| 5 | read-only | current opaque consumer-input header and payload, or a valid absent optional/none header |

The adapter may use bind groups 1 and above for its own resources.
Moria supplies the bind-group layout and bind group; it never supplies the
underlying authoritative buffers.
The view, effect, handoff, and consumer-input allocations are tick-local and charged to
`behavior_gpu_view_bytes`, `behavior_proposal_bytes`,
`behavior_handoff_bytes`, and `behavior_gpu_input_bytes`, respectively.
Accepted host input ownership is charged separately to
`behavior_input_records` and `behavior_input_bytes`. Prior/current feedback uses a Moria-owned
double-buffered per-participant allocation charged to
`behavior_feedback_bytes`.
GPU ingress charges exactly
`2 * align4(64 + descriptor.maximum_consumer_input_bytes)` before planning:
one staging and one device range. Both are retained until upload completion;
the staging range then returns, while the device range remains through the
participant's consuming submission.
All words are little-endian, offsets are four-byte aligned, and Rust/WGSL
layout assertions are mandatory. WGSL has no Scheduled ABI `u64` field.
Every logical 64-bit ID, revision, tick, or generation is the exact
`ScheduledU64LeV1 { low:u32, high:u32 }` wire value: the low/significance word
comes first, both words are compared for equality, and zero means
`low == 0 && high == 0`. Host packing is `low = value as u32`,
`high = (value >> 32) as u32`; unpacking is
`u64::from(low) | (u64::from(high) << 32)`. WGSL declares each pair as two
separate `u32` members. Headers use these exact byte offsets:

| Record | Exact fields |
| --- | --- |
| `BehaviorViewHeaderV1` (64 bytes) | `0 magic:u32`, `4 version:u32`, `8 tick_low:u32`, `12 tick_high:u32`, `16 generation_low:u32`, `20 generation_high:u32`, `24 engine:u32`, `28 volume_count:u32`, `32 cell_count:u32`, `36 volume_offset:u32`, `40 cell_offset:u32`, `44 total_bytes:u32`, `48 reserved:[u32;4]` |
| `BehaviorEffectHeaderV1` (64 bytes) | `0 magic:u32`, `4 version:u32`, `8 engine:u32`, `12 proposal_capacity:u32`, `16 payload_capacity:u32`, `20 output_proposal_count:u32`, `24 output_payload_bytes:u32`, `28 reserved0:u32`, `32 tick_low:u32`, `36 tick_high:u32`, `40 generation_low:u32`, `44 generation_high:u32`, `48 reserved:[u32;4]` |
| `BehaviorHandoffHeaderV1` (64 bytes) | `0 magic:u32`, `4 version:u32`, `8 direction:u32`, `12 engine:u32`, `16 edge_count:u32`, `20 descriptor_offset:u32`, `24 payload_offset:u32`, `28 total_bytes:u32`, `32 tick_low:u32`, `36 tick_high:u32`, `40 generation_low:u32`, `44 generation_high:u32`, `48 reserved:[u32;4]` |
| `BehaviorFeedbackHeaderV1` (64 bytes) | `0 magic:u32`, `4 version:u32`, `8 availability:u32`, `12 engine:u32`, `16 source_tick_low:u32`, `20 source_tick_high:u32`, `24 generation_low:u32`, `28 generation_high:u32`, `32 participant_count:u32`, `36 proposal_count:u32`, `40 total_bytes:u32`, `44 reserved:[u32;5]` |
| `BehaviorInputHeaderV1` (64 bytes) | `0 magic:u32`, `4 version:u32`, `8 engine:u32`, `12 presence:u32`, `16 payload_bytes:u32`, `20 total_bytes:u32`, `24 tick_low:u32`, `28 tick_high:u32`, `32 generation_low:u32`, `36 generation_high:u32`, `40 reserved:[u32;6]` |

The 112-byte volume record has offsets `0 volume_low:u32`,
`4 volume_high:u32`, `8 revision_low:u32`, `12 revision_high:u32`,
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
| 8 | exact expected revision low `u32` |
| 12 | exact expected revision high `u32` |
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

The 64-byte input header contains magic `MORI`, ABI version 1, participant
engine ID, closed presence (`Absent=0`, `Present=1`), payload byte count,
total byte count, tick ID, device generation, and zero reserved words.
`Absent` requires zero payload bytes and `total_bytes == 64`; it is invalid for
a `Required` participant. `Present` permits an empty payload, sets
`total_bytes` to `align4(64 + payload_bytes)`, and stores the exact current
consumer bytes starting at offset 64 with zero alignment padding. The payload
must not exceed the participant descriptor maximum or its effective binding
range. The shader sees no schema or type tag beyond these transport fields.

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
reserved words. `Ready` has `participant_count == 1`; its total bytes are
`64 + 64 + 48 * proposal_count`. `NoneYet` and
`UnavailablePreviousGeneration` have zero source tick, zero counts, and
`total_bytes == 64`; the current generation pair remains present so absence
cannot be confused with an old ready record. A ready header is followed by one
fixed 64-byte participant/terminal-decision record and one fixed 48-byte
record per proposal. Scheduled ABI v1 feedback deliberately contains no
snapshot vector and reserves no bytes for one. `proposal_count` is the number
of indexed proposal outcomes retained from this participant's prior dispatch,
and every proposal record repeats its original zero-based proposal index. A
GPU adapter correlates those indices with proposal/snapshot state it retained
in its own factory-created resources from that dispatch; Moria neither owns
nor repeats that consumer state in feedback.
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
`16 status:u32`, and `20 reserved:[u32;3]`. The exact 64-byte feedback
participant record is:

| Byte | Field |
| ---: | --- |
| 0 | `execution:u32` |
| 4 | `execution_failure:u32` |
| 8 | `publication:u32` |
| 12 | `notification:u32` |
| 16 | `tick_disposition:u32` |
| 20 | `flags:u32` |
| 24 | `failed_hook_count:u32` |
| 28 | `abort_cause:u32` |
| 32 | `cause_engine_a_or_zero:u32` |
| 36 | `cause_proposal_a_or_zero:u32` |
| 40 | `cause_engine_b_or_zero:u32` |
| 44 | `cause_proposal_b_or_zero:u32` |
| 48 | `cause_transition_stage_or_zero:u32` |
| 52 | `reserved:u32` |
| 56 | `cause_generation_low_or_zero:u32` |
| 60 | `cause_generation_high_or_zero:u32` |

`flags` defines exactly bit 0 as the tick-wide `revision_changed`, bit 1 as
this participant notification's `publication_was_complete`, and bit 2 as this
participant publication's `revision_changed`; bits 3..31 are zero. Bit 1 is
set only with
`notification == FailedAfterTerminalDecision`. `failed_hook_count` is the
tick-wide number of report hooks that returned an error or panicked after the
terminal decision; it is nonzero for
`PublishedWithNotificationFailure` and may also record notification failures
after `NoPublication` without changing that abort disposition.

Participant publication is derived before the record is encoded:

- `DiscardedByTick { cause }` is used for every participant when the tick
  disposition is `NoPublication { cause }`; publication tag 3 carries the same
  cause through the participant record and flag bit 2 is clear.
- `NoSelectedEffect` is used on a published terminal path when no proposal
  from that participant survived validation and conflict composition into the
  preparation set; publication tag 2 and flag bit 2 are clear.
- `Published { revision_changed }` is used on a published terminal path when
  at least one proposal from that participant entered preparation.
  `revision_changed` is true exactly when at least one such proposal belongs to
  a volume listed in the tick's published revision vector; publication tag 1
  stores that boolean in flag bit 2. If all of the participant's selected
  volumes fail preparation, the result is therefore
  `Published { revision_changed: false }`, even when another participant's
  independent volume publishes and makes tick-wide flag bit 0 true.

Publication tag 1 is invalid unless the participant had a selected proposal;
tags 2 and 3 are invalid with flag bit 2 set. Thus bit 0 and bit 2 may differ
and neither is inferred from the other.

`abort_cause == ParticipantAbort` stores the engine in A.
`ConflictFailTick` stores the earlier engine/proposal in A and the later pair
in B. `TransitionFailure` stores predecessor in A, successor in B, and the
closed transition stage. `DeviceLost` stores only its generation pair.
`PreparationFailure` has no tick-cause payload. The execution mapping below may
independently use A for a not-run preflight reason. Every field selected by
neither the abort-cause nor execution mapping is zero; a nonzero unused field
is invalid. `Published` and
`PublishedWithNotificationFailure` require `abort_cause == 0`;
`NoPublication` requires one nonzero closed abort cause and a clear
tick-wide and participant revision-changed bit. This mapping is lossless for
`BehaviorTickDisposition`, `BehaviorTickAbortCause`,
`BehaviorParticipantPublication`, and `BehaviorNotificationOutcome`; it
exposes the `BehaviorParticipantExecution` variant plus its closed failure
category, not variable Rust failure payloads such as unavailable-region
vectors or diagnostics. Proposal-specific terminal data remains in the
following records. This is the only intentional reduction: the terminal tick
disposition and abort cause are never reduced.

The exact 48-byte feedback proposal record is `0 status:u32`,
`4 reason:u32`, `8 proposal_index:u32`,
`12 related_engine_or_zero:u32`, `16 related_proposal_or_zero:u32`,
`20 reserved:u32`, `24 command_id_low_or_zero:u32`,
`28 command_id_high_or_zero:u32`, `32 revision_low_or_zero:u32`,
`36 revision_high_or_zero:u32`, `40 related_volume_low_or_zero:u32`, and
`44 related_volume_high_or_zero:u32`. `OverlapsEarlier` and
`ReplacedByLater` use the related engine/proposal pair;
`PreparationFailed` uses the related volume pair; `TickAborted` refers to the
complete participant-record abort cause. All unrelated fields are zero.

Execution has one additional closed mapping for tick-global input preflight.
The participant whose upload fails uses
`Skipped { ConsumerInputUpload }`. Every other participant uses
`NotRun { InputPreflightAborted { failed_engine } }`; its record has execution
tag 3, execution-failure tag 13, and stores `failed_engine` in A. A
device-loss preflight gives every participant
`NotRun { DeviceLost { generation } }`; execution is 3, execution failure is
9, and the generation pair is the same pair required by the tick abort cause.
For execution 1, execution failure and execution-specific fields are zero.
For execution 2, the failure is the participant's own failure. For execution
3, only failure tag 9 or 13 is legal. Failure tag 13 requires
`abort_cause == PreparationFailure`, nonzero A, and zero B/stage/generation;
tag 9 requires `abort_cause == DeviceLost`, the matching nonzero generation,
and zero A/B/stage. This execution-specific use of A is independent of the
tick-abort payload mapping; the failed participant's tag-12 record retains
zero A because the participant is itself the addressed failure.

ABI tag values are stable: handoff direction incoming `0`, outgoing `1`;
handoff status empty `0`, ready `1`, failed `2`; feedback availability
`NoneYet=0`, `Ready=1`, `UnavailablePreviousGeneration=2`; participant
execution completed `1`, skipped `2`, not-run `3`; participant publication published `1`,
no-selected-effect `2`, discarded-by-tick `3`; notification delivered `1`,
not-applicable `2`, failed-after-terminal-decision `3`; tick disposition
published `1`, no-publication `2`,
published-with-notification-failure `3`; proposal status admitted matter `1`,
admitted volume `2`, rejected `3`; proposal reasons none `0`,
overlap `1`, replaced `2`, invalid `3`, participant-failed `4`,
preparation-failed `5`, and tick-aborted `6`. Failure and tick-abort category
tags are: none `0`; planning `1`, unavailable `2`, access-limit `3`,
effect-limit `4`, invalid-proposal `5`, panicked `6`, GPU-validation `7`,
transition `8`, device-lost `9`, not-ready-generation `10`, shutdown `11`,
consumer-input-upload `12`, input-preflight-aborted `13`;
abort cause participant-abort `1`, conflict-fail-tick `2`,
transition-failure `3`, device-lost `4`, and preparation-failure `5`.
Transition stages are none `0`, CPU-write `1`, upload `2`, GPU-validate `3`,
GPU-copy `4`, map `5`, and decode `6`. These constants are shared by Rust/WGSL
layout tests. Unknown tags, a both-zero pair where a nonzero value is
required, treating a one-zero/one-nonzero pair as an absence sentinel, or
nonzero unused fields are validation failures, never forwarded to an adapter.

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

At admission, before the tick may transition to
`Preparing/UploadingGpuInputs`, Moria atomically reserves:

- one input record and the declared maximum host bytes for every input-capable
  participant, plus each GPU participant's 64-byte header, aligned device
  payload range, staging range, and upload completion record;
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
Unused input capacity is released after request validation. Accepted GPU host
and staging bytes remain charged through confirmed preflight upload, and each
device ingress range remains charged through its consuming submission.
Accepted CPU host bytes remain charged through the participant's planner and
CPU callback. On preflight failure, all unsubmitted input is released
immediately and submitted ranges are released only after completion or
old-generation quarantine. Unused proposal and payload capacity is released
after proposal validation.

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

Consumer-input structural failures occur synchronously before admission and
therefore have no tick outcome. Cancellation that wins the
`WaitingForFrontier -> Preparing/UploadingGpuInputs` race resolves through the
ordinary cancellation error with no consumer execution. GPU ingress
upload/validation completes before `Planning`; a closed
`ConsumerInputUpload` participant failure produces
`NoPublication(PreparationFailure)`, while device loss produces
`NoPublication(DeviceLost)`. Neither path invokes any planner, adapter, or
report hook for that tick. Input preflight is
tick-global and deliberately does not apply `SkipParticipant`, because doing
so would let another adapter execute after only a partial ingress set was
confirmed.

Every admitted tick that wins the transition to
`Preparing/UploadingGpuInputs` resolves its receipt with a
`BehaviorTickCompleted`, including an input-preflight or later no-publication
abort; execution failure is not collapsed into the generic operation error and
therefore cannot hide participant/proposal outcomes. An input-preflight abort
has an empty snapshot, empty proposal vector, empty published vector,
`NotApplicable` notification for every participant, and does not invoke
`on_tick_report`. The upload-failed participant is
`Skipped { ConsumerInputUpload }`; all unaffected participants are
`NotRun { InputPreflightAborted { failed_engine } }`. A device-loss preflight
marks every participant `NotRun { DeviceLost { generation } }`. All receive
`DiscardedByTick` with the tick cause. For an ordinary upload error Moria
writes matching current-generation double-buffered GPU feedback for a possible
next tick. Device loss quarantines that storage, so the Rust receipt remains
the complete outcome and the next generation receives
`UnavailablePreviousGeneration`. The closed disposition is:

- `Published`: publication processing completed; `revision_changed` says
  whether any volume advanced;
- `NoPublication(ParticipantAbort | ConflictFailTick | TransitionFailure |
  DeviceLost | PreparationFailure)`: no behavior proposal published and
  `revision_changed` is false; or
- `PublishedWithNotificationFailure`: publication already completed and one or
  more post-publication CPU report hooks failed.

Each participant separately records whether it executed, was skipped by its
own failure, was not run because tick-global ingress preflight failed, or
executed/was ready but was discarded by a later tick-wide abort.
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
  `UploadingGpuInputs`, which is the behavior family's `Preparing` boundary; a
  tick that entered input preflight drains to a complete preflight report or
  proceeds through planning and later stages.
  Moria calls adapter shutdown hooks after the last report, but does not save
  or discard consumer state on the consumer's behalf.

The CPU report contains tick ID, the participant's stable snapshot revisions,
participant status, proposal selection/rejection, assigned command IDs,
terminal revision when known, and failure category. GPU feedback contains the
same terminal participant/proposal facts except for the snapshot vector: it
uses the retained prior proposal index as the correlation key, and the adapter
owns any snapshot/proposal state needed to interpret that key. Neither form
contains behavior-specific payload.

## Adversarial integration cases

A conventional CPU physics adapter may keep bodies, velocities, forces,
joints, solver caches, and policy in its own Rust state.
As the first and only participant it receives changing opaque current input in
its planner and tick context, plans bounded body regions, receives a direct
borrowed tick view, runs its solver without a query receipt, updates its state,
and proposes movement or matter edits through the exact-capacity sink.

A GPU-resident physics adapter may keep all of those concepts in its own
factory-created opaque renderer-device resources.
As the first and only participant it reads changing opaque current input from
group-0 binding 5 plus the exported stable material/occupancy bind group, runs
its pipelines on Moria's ordered encoder, updates its own buffers, and writes
proposal records.
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
If its consumer wants new debris volumes, the scheduled adapter cannot create
or atomically split them. It may propose edits to existing matter and the
consumer may later submit ordinary create commands, which are independently
admitted and cannot be described as part of the scheduled tick transaction.

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
6. absence of physics, damage, bond, or gameplay fields in Moria types;
7. bounded changing consumer input reaches the first CPU and first GPU
   participant without a dummy predecessor, shared-state side channel,
   allocation beyond the ingress permit, raw GPU access, or authority-path
   readback; and
8. scheduled fracture/debris claims do not include atomic volume creation or
   split.

Any counterexample blocks the behavior-hook architecture claim.
