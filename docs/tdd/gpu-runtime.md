# Bevy and GPU runtime

The GPU is the normal owner of detailed canonical matter. This document
defines how that ownership is implemented without creating a privileged
consumer path or confusing submission with completion.

## Bevy ownership and schedules

### TECH-031 — Single Bevy renderer device

Implements: REQ-005, REQ-007, REQ-018, REQ-024, REQ-039

`MoriaPlugin` installs focused `CanonicalPlugin`, `QueryPlugin`,
`PersistencePlugin`, `ParticipantPlugin`, `PresentationPlugin`, and
`TelemetryPlugin` implementations. The main-world side owns public queues,
receipts, content tasks, and root metadata. Device-bound buffers, bind groups,
layouts, pipelines, pools, submissions, and mapping state live only in
`RenderApp`.

The adapter obtains Bevy's `RenderDevice` and `RenderQueue`; it never requests
a second wgpu device. `ExtractSchedule` copies only bounded request descriptors,
immutable canonical bytes, and root-generation deltas. Large content moves
through owned staging permits, not cloned ECS structures.

Plugin finish creates one `Arc<RenderCompletionBridge>` and inserts a clone
into both worlds. It is a mutex-protected, preallocated fixed ring, not an ECS
event channel: extraction remains one-way, while render completions return
through this explicitly shared transport. Each admitted GPU job must reserve
one ring cell before extraction and carries
`(JobId, WorldId, DeviceGeneration, attempt_nonce)`. The compiled maximum is 32
job cells, at least the sum of the configured canonical, query, checkpoint,
materialization, genesis, and presentation in-flight slots, plus two dedicated
generation/shutdown control cells. Materialization shares the query/readback
slot pool. Exhausted cells apply admission backpressure; the
render thread never drops or overwrites a completion.

GPU root and participant tokens carried by an envelope are internal
generation-tagged IDs into render-world ownership tables, not wgpu handles.
The main-world bundle may copy those IDs for pinning and later extraction, but
only `RenderApp` resolves them to device objects; they never enter public IDs,
canonical bytes, replay, or persistence.

Device-independent plugin and ABI descriptors initialize in the main world.
All recovery-sensitive GPU resources initialize through Bevy 0.19
`RenderStartup`, so a new render device builds a new device generation.

### TECH-032 — Render schedule ordering

Implements: REQ-005, REQ-013, REQ-033, REQ-040

Moria defines render-system sets with explicit edges:

```text
Main First:
  MoriaCollectRenderCompletions
    -> MoriaPublishCanonical
    -> MoriaFinalizeOtherReceipts

Main PostUpdate:
  MoriaCoordinateRequests

ExtractSchedule:
  ExtractMoriaRequests

Render schedule / root RenderGraph:
  MoriaPrepareResources
    -> MoriaEncodeCanonical
    -> MoriaSubmitCanonical
    -> MoriaDriveCompletion
    -> MoriaPrepareDerived
    -> MoriaEncodePresentation
```

Canonical camera-independent compute dispatches run in an explicitly ordered
root `RenderGraph` subgraph after preparation and before presentation consumes
new roots. Query dispatches against already committed roots may run alongside
derived work but never interleave with candidate publication. CPU callback and
mapping progress is polled in `MoriaDriveCompletion`.

`MoriaDriveCompletion` moves a terminal envelope into its already reserved
bridge cell only after mapping and decode. On the next main-world `First`,
`MoriaCollectRenderCompletions` drains at most the configured 32 cells into a
fixed `JobId` table. The exclusive `MoriaPublishCanonical` system accepts
exactly one envelope for the sole pending canonical attempt, revalidates world,
attempt nonce, source frontier, device generation, mapped diagnostic counts,
participant products, and root hash, then performs TECH-013 step 12 as one
`Arc<FrontierBundle>` swap. That same critical section updates receipt state,
rollback deque, replay log, participant commitments/tokens, revision metadata,
and canonical observations before any later main-world system can read them.
Noncanonical job completions are finalized only afterward.

Duplicate envelopes are an invariant failure and terminally fail the
generation; an unknown or already-aborted `JobId` is acknowledged only for
lifetime cleanup. An old-generation envelope may release its reserved cell and
render resources but cannot publish. The main world never waits while holding
the bridge mutex, and callback timing cannot select a tick outcome.

Presentation may lag by frames. Extraction never waits for it. Rollback
suppresses intermediate replay-derived work and emits one final dirty-root
delta. No system set, graph node, callback, or frame boundary is a canonical
tick boundary; the coordinator's confirmed publication is.

## Physical representation

### TECH-033 — Buffer and page layout

Implements: REQ-004, REQ-010, REQ-018, REQ-033

Durable device buffers are paged so one binding never exceeds the granted
`max_storage_buffer_binding_size`:

| Pool | Record | Baseline page |
| --- | --- | ---: |
| Dense brick | 2,048-byte `CellWire[512]` | 32 MiB / 16,384 bricks |
| Uniform brick | 16-byte aligned record | 8 MiB |
| Radix node | 1,024-byte record with 16 child handles/hashes | 32 MiB / 32,768 nodes |
| Volume metadata | 256-byte record | 8 MiB |
| Resident base directory | 32-byte bucket | 16 MiB |
| Inputs/outcomes | versioned wire records | 8 MiB per in-flight slot |
| Scan/sort scratch | phase-specific | 32 MiB per in-flight slot |
| Readback | `MAP_READ | COPY_DST` only | 16 MiB per slot |

Page sizes are ceilings; initialization chooses the lower of configured budget
and actual device limits. Storage binding offsets are multiples of
`min_storage_buffer_offset_alignment` and each effective range covers only its
page. A separate page table resolves logical handles; a shader never assumes
one monolithic allocation.

Canonical work buffers use `STORAGE | COPY_SRC` (plus `COPY_DST` where
initialization requires). Readback always copies into a distinct
`MAP_READ | COPY_DST` buffer. No buffer is mapped while submitted for GPU use.

The resident base directory is a bounded open-addressed cache with explicit
`EMPTY`, `RESERVED`, `OCCUPIED`, and `TOMBSTONE` states and maximum 32 probes.
Reservation, payload write, duplicate validation, and publication are separate
ordered dispatches. This directory is noncanonical: a miss changes readiness,
never truth. Canonical scars use the immutable radix tree rather than a mutable
hash table.

### TECH-034 — Host/WGSL ABI

Implements: REQ-007, REQ-021, REQ-028, REQ-036, REQ-039

Every wire type has:

- an explicit Rust encoder/decoder rather than transmute;
- a WGSL struct using only `u32`/`i32` scalar words;
- a table of field offsets, alignment, size, array stride, and binding range;
- compile-time Rust size/offset assertions where a staging mirror exists;
- a shader fixture that writes each field and padding word for exact readback;
- zero initialization of padding, counters, flags, and unused output slots.

Rust `bool`, enums, pointers, `usize`, `isize`, and implicit struct padding are
forbidden in GPU ABI. `CellWire` pairs are packed/unpacked through `u32`.
`vec3` is not used in durable records. Runtime arrays appear only as the last
member and are bound to an exact logical record count; `arrayLength` is not
used to infer allocation capacity.

Dispatch count is computed with checked `count / width + (count % width != 0)`.
Every over-dispatched invocation checks logical count. Workgroup dimensions,
invocation product, workgroup memory, buffer range, and indirect `[x,y,z]`
records are validated against granted limits before encoding.

## Portable canonical kernels

### TECH-035 — Kernel synchronization and deterministic output

Implements: REQ-007, REQ-028, REQ-033, REQ-036, REQ-039

Canonical WGSL uses the portable 32-bit baseline. Atomics may claim
noncanonical work slots or set failure flags; no atomic winner supplies stable
identity, output order, revision, or hash input. Multiword records follow:

```text
reserve -> ordered payload initialization -> validate -> ordered publish
```

Cross-workgroup phases use separate dispatches on the same queue. Barriers are
in uniform control flow and all inactive lanes contribute identity values.
There is no spin wait, “last workgroup,” subgroup-width assumption, 64-bit CAS,
or device-scope publication assumption.

Variable canonical output uses fixed slots or stable
mark/scan/scatter. Atomic append is permitted only for bounded noncanonical
diagnostics and reports overflow. The portable scan hierarchy and radix sort
are specified in TECH-015. Indirect dispatch records are exactly three packed
`u32` values, four-byte aligned, range checked, and generated in a distinct
phase; if `INDIRECT` is unsupported the equivalent host-known bounded direct
dispatch is used without changing canonical results.

Each shader operation is wrapped in balanced validation error scopes. Shader
parse, Naga validation, module/layout creation, pipeline creation, encoding,
submission, mapping, decoding, and semantic comparison are distinct error
layers.

### TECH-036 — Bounded resource policy

Implements: REQ-004, REQ-005, REQ-018, REQ-021, REQ-022

`ResourceBudgets` is immutable after genesis and contains byte/count ceilings.
Baseline defaults and compiled maxima are:

| Resource | Default | Compiled maximum | Overload behavior |
| --- | ---: | ---: | --- |
| pending canonical ticks | 1 | 1 | reject and return batch |
| tick inputs | 4,096 | 4,096 | reject batch |
| tick encoded bytes | 8 MiB | 8 MiB | reject batch |
| bricks per command | 64 | 64 | canonical command failure |
| changed bricks per tick | 16,384 | 16,384 | `NoAdvance(CanonicalBudget)` |
| query queue | 256 | 1,024 | reject request |
| interest records | 4,096 | 16,384 | reject update |
| observation records/world | 8,192 | 65,536 | explicit gap |
| in-flight canonical pool | 1 | 1 | backpressure |
| in-flight query/materialization/readback slots | 3 | 8 | reject or wait via permit |
| in-flight presentation slots | 3 | 8 | coalesce newest dirty revision |
| participant effects/tick | 4,096 | 4,096 | `NoAdvance` |
| participant tokens+snapshots/frontier | declared sum | 64 MiB | reject genesis / `NoAdvance` |
| render completion bridge | 32 cells | 32 cells | reserve before extraction |
| rollback frontiers | 32 | budget-derived | reject genesis/tick pressure |

One in-flight permit owns all input, scratch, output, diagnostic, and staging
resources for that job. A permit returns only after the last queue completion,
mapped views are dropped, buffers are unmapped, results are decoded/discarded,
and any staging-belt chunk has been recalled.

Canonical resource classifications use declared logical limits, not physical
allocation race or current free-list order. If the adapter cannot allocate the
declared baseline plus 20 frontiers, genesis fails. Noncanonical work applies
priority, delay, coalescing, retirement, or rejection as shown; it never evicts
pinned truth or unsaved scars.

### TECH-037 — Completion, mapping, and generation safety

Implements: REQ-005, REQ-015, REQ-021

Internal job state distinguishes `Encoded`, `Submitted(SubmissionIndex)`,
`GpuComplete`, `MapPending`, `Mapped`, `Decoded`, and `TerminalFailure`.
`Queue::submit` is not completion. Native progress is driven through Bevy/wgpu
polling without blocking the app thread. A successful mapping callback is
required before bytes are read; views drop before `unmap`.

Every job, root, buffer handle, callback, receipt bridge, and derived artifact
carries `DeviceGeneration`. Device loss marks that generation terminal;
callbacks from it can release lifetime state but cannot publish results.
Consumer cancellation after submission discards delivery only. Old
application handles fail `StaleGeneration`.

Pool telemetry records capacity, bytes, high-water marks, oldest age,
submission-to-complete, complete-to-map, decode time, and cancellation point.
Timeouts are diagnostics only: wall-clock expiry may mark the world
environmentally failed, but cannot select or publish a canonical result.

The bridge reserves one cell for every extracted job until the main world
acknowledges its terminal envelope. Shutdown closes new reservations, asks the
render world to terminalize every extracted job, and drains all reserved cells
before removing either bridge clone. Device loss writes one dedicated
`GenerationLost` control record and terminalizes each job into its own reserved
cell. If fixed-ring accounting is violated, the world fails closed; no
completion is silently replaced.

## Device loss and recovery

### TECH-038 — Fail-closed device recovery

Implements: REQ-001, REQ-005, REQ-014, REQ-015, REQ-021

On device loss Moria:

1. closes admission and marks every candidate in the old generation
   `FailedNoAdvance(DeviceLost)`;
2. preserves the last confirmed root identity, tick log, checkpoint identity,
   and participant commitments on the host, but makes GPU queries unavailable;
3. applies every GPU participant's TECH-029 failure policy: any `FailWorld`
   participant makes the world `Failed`; otherwise the world enters
   `RecoveringParticipant` and waits for an explicit `request_recovery`;
4. reserves one bounded recovery attempt, asks Bevy `RenderStartup` to
   construct a new generation, and verifies that the replacement adapter
   matches a current qualified tuple;
5. restores the newest compatible durable checkpoint—including its
   participant snapshot and exact replay-record blobs—and replays the
   confirmed in-memory suffix, bounded by `recovery_replay_cap` (default 256
   ticks);
6. compares every retained expected hash, participant commitment, and RNG
   commitment;
7. republishes the same frontier and returns to `Ready`, or applies
   `NoAdvanceExplicitRetry` by remaining `RecoveringParticipant` with an
   explicit failed recovery receipt.

The confirmed tick log stores inputs and bounded canonical outcome/participant
records, not a voxel mirror. If there is no compatible checkpoint/genesis
source, replay exceeds its bound, a participant cannot restore, the tuple is
unqualified, or a hash diverges, that recovery attempt fails explicitly.
Moria never loops or schedules a timer retry; the consumer may request another
bounded attempt or shut down. The last durable checkpoint remains usable;
Moria never fabricates empty matter or publishes a different state as
recovery.

Device loss during checkpoint readback leaves the manifest uncommitted and the
root pinned until failure cleanup. Device-bound objects are never retained
across generations.

## Portability and qualification

### TECH-039 — Native portability baseline

Implements: REQ-005, REQ-007, REQ-026, REQ-039, REQ-043

First-class backend families are:

- macOS physical GPU through Metal;
- Linux physical GPU through Vulkan;
- Windows physical GPU through DX12.

Qualification is adapter/driver/OS/backend/contract-specific, not merely
family-specific. The baseline requires compute shaders, 32-bit integer
atomics, storage buffers, copy/map readback, at least 128 MiB effective storage
binding range, at least 256 MiB buffer allocation, and sufficient workgroup
limits for 128 invocations. Actual features, limits, downlevel flags, adapter
identity, and fallback status are recorded.

Subgroups, timestamp queries, native 64-bit shader integers, large binding
ranges, and indirect dispatch are optional measured branches with baseline-
equivalent public semantics. Software/fallback adapters may run diagnostics
but cannot satisfy a physical-GPU claim. WebGPU/WebAssembly, WebGL/GLES, and
standalone second-device operation are excluded current targets.

### TECH-040 — Authority and candidate modes

Implements: REQ-008, REQ-021, REQ-023, REQ-026, REQ-039

`QualificationPolicy` has two modes:

- `RequireQualified(EvidenceDigest)`: genesis succeeds only when the
  digest-sealed qualification manifest exactly matches runtime tuple and all
  canonical contract digests. This is the only authority mode.
- `Candidate { diagnostics }`: runs the identical public API and kernels to
  produce evidence, but every frontier is labeled `UNQUALIFIED_CANDIDATE` and
  cannot be exported as an authoritative checkpoint or a passing conformance
  claim.

`CandidateDiagnostics` is a normal public configuration type available to any
external consumer. It is bounded to one optional
`FaultOnce { tick, command_order, stage }`; v1 has the single stage
`AfterBrickConstructionBeforePublication`. The selected ordinary matter
command passes normal admission and construction, after which the production
diagnostic/status record is set to `InjectedCandidateFailure` before
TECH-013 validation step 9. The coordinator then follows the ordinary
`FailedNoAdvance` cleanup path. The fault plan cannot write cells, roots,
outcomes, or buffers, cannot target authority mode, is not a canonical input,
and is recorded in candidate evidence. Authority configuration rejects
nonempty diagnostics. This seam is qualification control, not a mutation
bypass or a self-reported correctness result.

There is no automatic CPU, alternate GPU, or relaxed-shader fallback in an
authority world. A driver, OS, adapter, wgpu/Naga, canonical shader, encoding,
arithmetic, hashing, transition, or participant contract change invalidates
the matching manifest until qualification reruns. Presentation-only changes
do not, unless they change shared bindings or scheduling relevant to canonical
work.
