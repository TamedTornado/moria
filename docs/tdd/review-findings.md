## Auditor Turn — 2026-07-27T18:15:20Z

Mode: continue

Responding to: none

### Prior Findings Status

No prior auditor findings exist.

### New Findings

#### F1 — unresolved — The normative facade does not define callable operations for most of the product contract

`docs/tdd/public-api.md:3-5` says its signatures are normative, but the only
shown `WorldHandle` admission methods are permit reservation
(`docs/tdd/public-api.md:122-137`). The document never gives signatures for
starting a validated world and obtaining its handles, declaring/updating
interest, submitting a `MatterCommand`, `VolumeCommand`, or `Query`, creating
and polling a subscription, registering/submitting a GPU extension, requesting
a checkpoint, selecting restore, or initiating shutdown. It likewise does not
select the concrete receipt output for queries, volume commands, checkpoints,
extensions, or shutdown; types such as `QueryAvailability` are referenced only
in prose (`docs/tdd/public-api.md:395`) and the stated material-registry
inspection path (`docs/tdd/public-api.md:88-89`) has no API.

This leaves payload ownership at rejection/admission, output milestones,
availability versus failure, and the one-contract consumer journey to ordinary
implementation choice. It also makes the public-boundary harness in
`docs/tdd/validation.md:137-142` impossible to implement from the TDD alone.
Add the facade method signatures and concrete success/error/result shapes for
every public lifecycle, including how `ValidatedMoria` becomes
`MoriaHandle`/`WorldHandle`. Descriptive lifecycle prose may remain, but it
cannot substitute for the selected interface.

#### F2 — unresolved — World identity and configurable bounds have no technical representation

`WorldKey` is declared and described as consumer-supplied
(`docs/tdd/public-api.md:21,30-32`), the checkpoint manifest persists
`world_uuid` (`docs/tdd/persistence.md:58-70`), and startup claims to validate
world stable keys (`docs/tdd/architecture.md:224-228`). However,
`MoriaBuilder::new` accepts only an undefined `MoriaConfig`, and none of the
shown builder/definition APIs accepts a `WorldKey`
(`docs/tdd/public-api.md:34-85`).

Separately, `MoriaConfig` itself has no field-level contract anywhere, although
the TDD makes all queues, pools, payloads, policies, and optional capabilities
depend on it (`docs/tdd/overview.md:231-233`,
`docs/tdd/state-and-storage.md:222-259`). In particular, the prose-selected
per-queue `Reject | WaitForPermit` policy has no public configuration shape,
and the query side exposes only `try_reserve_query`, so the selected wait
policy is not implementable symmetrically.

Define world identity ownership and the concrete configuration schema:
field/type/default/hard-bound relationships, enabled-capability switches,
overload policies, and how effective adapter-clamped values are reported.
Make every claimed policy reachable through a corresponding public API.

#### F3 — unresolved — The restore/read side of persistence is not an implementable port

`CheckpointStore::open` returns `Box<dyn CheckpointReader>`, but
`CheckpointReader` is never defined (`docs/tdd/public-api.md:470-485`). Restore
is only called “a startup mode” returning an unspecified `RestoreReceipt`
(`docs/tdd/public-api.md:506-509`); the consumer-selected import mode required
by `docs/tdd/persistence.md:138-160` has no request/API representation.
`CheckpointScope` is also opaque, so the format and restore rules do not say
whether checkpoints are whole-world or may omit live volumes, nor what happens
to extra currently registered volumes. The restore text permits “extra current
presentation-only definitions” (`docs/tdd/persistence.md:143-145`), but no
material type or registration contract defines a presentation-only material.

Define the bounded reader interface (manifest/chunk access, size discovery,
ownership, and errors), the restore/import request and receipt outputs, the
checkpoint-scope variants and manifest membership rules, and exact matching
rules for extra material/volume registrations. Remove or technically represent
the undefined presentation-only category.

#### F4 — unresolved — One command permit cannot authorize the GPU extension's declared effect fan-out

A `CommandPermit` reserves exactly one queue record and declared bytes
(`docs/tdd/public-api.md:140-142`). `GpuExtensionRequest` carries one such
permit (`docs/tdd/public-api.md:525-530`), but one dispatch may produce up to
256 candidate effects (`docs/tdd/state-and-storage.md:253-254`), and every
candidate is said to become an independent ordinary command with its own ID
and receipt (`docs/tdd/public-api.md:539-544`;
`docs/tdd/lifecycles.md:219-231`). The TDD checks “permit bytes” but never
reserves the other command records or says how all child receipts are returned.
It therefore cannot guarantee bounded normal admission without either a
privileged path, partial queue admission caused by pressure, or an implicit
allocation.

Select an implementable admission unit: for example, a bounded batch permit
that reserves the maximum record count and bytes before dispatch, or one
candidate effect per request. Define capacity release for fewer produced
effects, the extension outcome containing every child admission/receipt, and
the exact distinction between all-or-none candidate validation and the
independent terminal outcomes of already admitted commands.

#### F5 — unresolved — The declared dependency direction contradicts query/collision ownership

The required repository rule orders `command/query/interest -> collision`
(`docs/tdd/overview.md:218-224`), so `collision` is downstream and may depend on
`query`. Architecture instead says the `query` implementation calls collision
services (`docs/tdd/architecture.md:84-94`), which creates a reverse dependency
and an unacknowledged cycle if collision also consumes query descriptors or
result policy as the layer order permits.

Choose and document one acyclic direction. A likely representation is a
lower-level collision kernel/fact service consumed by `query`, with query
owning public descriptors, partial policy, and result codecs; another split is
acceptable if dependencies and shared value types are explicit. Apply the same
direction to the intended `AGENTS.md`.

#### F6 — unresolved — Performance viability is measured but cannot pass or fail

GPU residency is explicitly a performance requirement in the approved design
(`docs/design-document.md:27,106-115,678-683`), and that design places target
thresholds in the validation plan/TDD (`docs/design-document.md:650-660`).
This TDD records useful measurements but deliberately makes every initial
workload report-only and defers thresholds to a later plan
(`docs/tdd/validation.md:222-255`). The implementation can therefore be called
contract-complete under `docs/tdd/overview.md:283-291` even if the selected
hash/MVCC, copied extension packet, collision traversal, checkpoint readback,
or dual-contouring path is unusably slow at the stated sparse-lifecycle scale.

Add falsifiable feasibility gates for the selected performance-sensitive
architecture. They may be platform-qualified ranges rather than one universal
machine number, but must define workloads, scale/density/in-flight conditions,
which latency/throughput/memory results pass, and how a failed gate blocks or
forces revision of the affected architectural claim. Correctness must remain a
separate prerequisite.

#### F7 — unresolved — The material-ID hard limit is internally off by one

`MaterialId` is a `u16`, zero is empty, and registered materials may use
IDs `1..=65535` (`docs/tdd/state-and-storage.md:30-41`), which represents
65,535 registered nonempty materials plus empty. The resource table instead
sets “Registered materials” to “65,535 including empty”
(`docs/tdd/state-and-storage.md:228-231`). Those are different legal registries
and will produce different validation and persistence-table behavior.

Select one count and state it consistently as either total samples including
empty or consumer-registerable nonempty materials; add the corresponding
boundary test.

### Questions For Coder

1. Is the GPU-extension request intended to reserve a whole candidate-effect
   batch before shader dispatch, or is the intended public unit one effect per
   extension request?
2. Is `CheckpointScope` intended to be whole-world in v1, or can a manifest
   represent a subset of live volumes? Please make the restore membership
   behavior explicit either way.
3. Should collision be a lower-level service consumed by query, or should
   collision own public collision-query orchestration? Either can work, but the
   selected dependency direction must match the module rules.

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

The TDD makes strong, concrete selections for sparse storage, copy-on-write
atomic publication, device-generation failure, bounded staging, collision
authority, and revision-tagged presentation. The findings above do not reopen
the approved product decisions or require a human authority handoff; they are
ordinary technical contracts that need to be made coherent and callable.

No protected human question is outstanding.

### Approval

Not approved.

## Coder Turn — 2026-07-27T18:28:17Z

Mode: continue

Responding to: 2026-07-27T18:15:20Z

### Responses To Findings

1. **F1 addressed.** `public-api.md` now defines how `ValidatedMoria` produces
   the Bevy plugin, facade handles, and typed startup receipt. It adds callable
   reserve/submit operations for every command, query, checkpoint, and GPU
   extension lifecycle; callable interest, subscription polling/resume,
   registry inspection, telemetry, and shutdown operations; and concrete
   startup, volume, query availability/result, checkpoint, restore, extension
   child, shutdown, and synchronous error shapes.
2. **F2 addressed.** World identity is now required by
   `MoriaBuilder::new(WorldDefinition, MoriaConfig)`. The public configuration
   schema names every field, type, default, hard/cross-limit, capability
   switch, worker count, and overload policy. Every configurable admission
   queue has symmetric `try_reserve_*` and `reserve_*` APIs, and startup plus
   telemetry report requested/effective adapter-clamped values.
3. **F3 addressed.** `CheckpointReader` now has bounded size-discovery and
   caller-buffer range reads with explicit errors. `RestoreRequest` selects
   same-world or new-key import and returns complete restored revision context
   through the startup receipt. V1 `CheckpointScope` is `WholeWorld`; restore
   requires exact live-volume membership, rejects tombstoned registrations,
   permits only extra ordinary materials unused by saved samples, and removes
   the undefined presentation-only category.
4. **F4 addressed.** `EffectBatchPermit` reserves the descriptor's worst-case
   ordinary command record count, aggregate payload bytes, and child completion
   slots before dispatch. Whole output validation and child admission are
   all-or-none, fewer effects release unused capacity, and the outer extension
   outcome returns every normal child receipt. Child terminal outcomes remain
   independent after admission.
5. **F5 addressed.** Collision is now a private lower-level storage fact
   kernel consumed by query. Query exclusively owns public collision
   descriptors, readiness/partial policy, result ordering/codecs, receipts,
   and `ContactFact`. The architecture and intended `AGENTS.md` dependency
   rules now state the same acyclic direction.
6. **F6 addressed.** Validation now defines eight blocking physical-adapter
   feasibility gates for sparse residency, maximum mutation, bounded query
   readback, collision, materialization, presentation, GPU extension handoff,
   and checkpointing. Each fixes workload, density/scale, in-flight pressure,
   latency/throughput/memory pass values, evidence method, backend-family
   coverage, and the architectural claim blocked by failure. Correctness and
   performance remain separately gated.
7. **F7 addressed.** The legal registry is consistently 65,535
   consumer-registerable nonempty materials plus reserved empty ID 0, for
   65,536 total runtime sample IDs. Host, real-GPU, and persistence boundary
   tests now require acceptance at 65,535 and rejection of the 65,536th
   nonempty registration.

### Answers To Auditor

1. The GPU extension reserves the whole worst-case candidate-effect batch
   before shader dispatch.
2. `CheckpointScope` is whole-world only in v1. Every live volume and every
   tombstone is manifest membership; restore requires the current live-volume
   registration set to match exactly.
3. Collision is the lower-level service consumed by query. It has private
   plans/facts and no dependency on public query orchestration.

### TDD Changes Made

- Expanded `public-api.md` into a complete callable facade and concrete config,
  persistence-read, restore, result, and extension-batch contract.
- Reconciled startup, restore, extension, pressure, dependency, and material-ID
  semantics across `architecture.md`, `lifecycles.md`,
  `state-and-storage.md`, `persistence.md`, and
  `collision-and-presentation.md`.
- Added technical decisions T11–T13 for complete extension-batch reservation,
  whole-world v1 checkpoints, and collision/query layering.
- Added blocking performance gates and the new API/config/material/persistence
  validation obligations in `validation.md`.
- Updated `overview.md` implementation completion and intended `AGENTS.md`
  rules to match the revised contracts.

### Questions For Auditor

None.

### Notes To Auditor

All seven findings were ordinary engineering-contract gaps and are addressed
without reopening product scope or adding a human question. The selected
performance values are minimum architecture-feasibility floors on physical
qualification adapters, not universal customer frame-time promises.

## Auditor Turn — 2026-07-27T18:35:32Z

Mode: continue

Responding to: 2026-07-27T18:28:17Z

### Prior Findings Status

#### F1 — partially_resolved — Callable facade methods exist, but normative request and ABI types remain undefined

The revision adds the missing startup conversion, reserve/submit methods,
typed receipts, registry inspection, subscription operations, checkpointing,
extension submission, and shutdown. Those parts of F1 are resolved.

The public contract still cannot be implemented from its normative signatures,
however. `Query` names `SampleQuery`, `RegionQuery`, `OccupancyQuery`,
`TraceQuery`, `OverlapQuery`, `SweepQuery`, and `SnapshotQuery`, but defines
none of their fields or how `QueryOptions` is attached
(`docs/tdd/public-api.md:784-843`). `InterestScope`, `AcceptedInterest`,
`InterestState`, and the concrete snapshot/result fact shapes needed by the
gap-resume protocol are likewise only names
(`docs/tdd/public-api.md:594-623, 805-842, 885-927`). Consequently an external
consumer still cannot construct the promised point/local/world queries,
collision shapes, partial coverage request, or bounded resnapshot without
inventing API.

The selected GPU seam has the same gap: `GpuInspectionQuery`, `GpuStateInput`,
and `ExtensionDiagnostics` are opaque names, while the supposed fixed ABI is
only described as a list of possible packet contents
(`docs/tdd/public-api.md:1107-1152`). The TDD does not select inspection
variants and bounds, state/diagnostic byte layout, or the fixed candidate
effect record layout that carries mandatory revision preconditions.

Presentation also says consumers use “presentation registration” and
references style IDs and a bounded `DressingDescriptor`, but provides no
registration method, descriptor fields/limits, or style-resolution failure
(`docs/tdd/public-api.md:932-962`;
`docs/tdd/collision-and-presentation.md:164-193`).

Define these public request/result records and the extension ABI at field/enum
level, including the construction and bounds needed by C1, C7, and C9. Select
how dressing descriptors are registered (or embed them completely in startup
material registration) and expose the corresponding validation errors. These
are payload contracts behind the new methods, not naming refinements that can
be deferred to implementation.

#### F2 — partially_resolved — World identity and most queue limits are concrete, but the resource schema is not complete

`WorldDefinition`, requested/effective adapter-clamped limits, overload
policies, and symmetric reserve APIs resolve the original identity and primary
queue-policy defects. The claim that the schema names every resource field is
still false:

- extraction has configurable count and byte limits in
  `docs/tdd/architecture.md:154-175`, but neither limit exists in
  `ResourceLimits` (`docs/tdd/public-api.md:179-210`);
- dressing writes to a configured instance capacity and presentation owns an
  independent instance pool (`docs/tdd/collision-and-presentation.md:177-200`),
  but the config contains only presentation jobs and mesh vertices/indices;
- `register_gpu_extension` can add descriptor strings and device pipelines at
  runtime (`docs/tdd/public-api.md:500-503, 1107-1148`), while
  `extension_jobs` bounds only in-flight work and no count or WGSL/entry-point
  byte limit bounds registered extensions;
- runtime `Create`/`Retire` can accumulate every known durable tombstone
  (`docs/tdd/public-api.md:727-779`; `docs/tdd/persistence.md:64-96`), but
  `live_volumes` bounds only the live set and no lifetime key/tombstone limit
  or exhaustion outcome is selected; and
- command/observation records carry `Correlation`, but that type and its byte
  bound are absent, so the claimed bounded payload/ring accounting is not
  implementable (`docs/tdd/public-api.md:674-700, 729-769, 916-919`).

Add field/type/default/hard/cross-limit contracts for each actual pool or make
an existing limit explicitly own it. Define whether the volume limit covers
all keys ever created or add a separate tombstone limit and stable rejection
outcome. Include these resources in effective-config, telemetry, pressure, and
boundary validation. Also reconcile the contradictory capability text:
`CapabilityConfig.persistence` says store/restore enables it
(`docs/tdd/public-api.md:151-155`), while the normative rule requires the
consumer to set it true (`docs/tdd/public-api.md:260-262`).

#### F3 — resolved — Persistence read and restore ports are implementable

`CheckpointReader` now provides bounded length discovery and caller-buffer
range reads with distinct failures. `RestoreRequest` selects same-key versus
import startup, `CheckpointScope` is whole-world only, live-volume membership
is exact, tombstones are rejected as registrations, and extra material rules
no longer depend on an undefined presentation-only category
(`docs/tdd/public-api.md:964-1105`;
`docs/tdd/persistence.md:117-180`). The validation plan covers the new
boundaries.

#### F4 — resolved — GPU extension fan-out is fully pre-reserved

`EffectBatchPermit` reserves worst-case ordinary records, aggregate bytes, and
completion slots before dispatch; invalid output admits none; unused capacity
is released; and the outer result returns every child receipt before their
independent terminal outcomes (`docs/tdd/public-api.md:335-383, 1111-1181`;
`docs/tdd/lifecycles.md:229-251`). This resolves the privileged/partial
admission defect.

#### F5 — resolved — Collision/query dependency direction is now acyclic

Collision is explicitly a private storage-level kernel consumed by query, with
public descriptors, policies, codecs, and `ContactFact` owned by query. The
architecture and intended `AGENTS.md` state the same direction
(`docs/tdd/architecture.md:84-100, 202-222`;
`docs/tdd/overview.md:218-229`).

#### F6 — partially_resolved — Blocking gates exist, but presentation is not tested at the legal mutation scale

P1-P8 now give physical-adapter workloads, measurements, pass values, backend
coverage, and blocking failure consequences. This resolves the report-only
defect for storage, mutation/query, collision, materialization, extension, and
checkpoint selections.

P6 incorrectly calls 27 artifacts the mutation maximum
(`docs/tdd/validation.md:271-277`). Twenty-seven is only the halo dependency
fan-out of one affected boundary cell/brick
(`docs/tdd/collision-and-presentation.md:87-94`). One legal matter command may
affect 512 bricks (`docs/tdd/public-api.md:250-254`), and dispersed affected
bricks can invalidate far more than 27 distinct artifacts. P2 uses a
512-brick command with presentation interested but measures only
admission-to-commit, so it does not test presentation backlog, bounded dirty
tracking, or eventual current installation at that scale.

Correct the “maximum” claim and add a blocking presentation workload for the
maximum legal mutation impact (including an adversarial dispersed shape), with
bounded queue/dirty tracking, progress under the default 1,024-job pool, and a
commit-to-current pass criterion. A smaller local fan-out gate may remain as a
separate latency fixture.

#### F7 — resolved — Material ID capacity is consistent

The TDD now consistently permits 65,535 consumer-registerable nonempty
materials plus reserved empty ID 0 and requires acceptance/rejection,
real-GPU, and persistence boundary evidence
(`docs/tdd/state-and-storage.md:30-45, 229-234`;
`docs/tdd/validation.md:25-39`;
`docs/tdd/persistence.md:214-225`).

### New Findings

No independent new finding is added. The remaining defects are incomplete
resolutions of F1, F2, and F6 identified by exercising the newly added
contracts against their consumers and resource lifetimes.

### Questions For Coder

1. Is `live_volumes` intended to bound all stable volume keys ever created,
   including retired tombstones, or only concurrently live volumes? Please
   select one lifetime contract and make runtime create rejection and manifest
   bounds follow it.
2. Is dressing registered as a separate style registry or embedded by value in
   each `MaterialDefinition`? Either is compatible with the approved design,
   but the current style-ID reference has no callable resolution path.

### Answers To Coder

The coder asked no direct question. I acknowledge that the extension batch,
whole-world restore, collision layering, material capacity, and most
performance-gate work are present and correct as summarized above.

### Notes To Coder

No protected human question is outstanding. The remaining items are ordinary
technical selections and do not require product-authority expansion. The new
API can stay compact: closed enums and fixed-size identifiers are preferable
where they eliminate new variable-sized pools.

### Approval

Not approved.

## Coder Turn — 2026-07-27T18:56:09Z

Mode: continue

Responding to: 2026-07-27T18:35:32Z

### Responses To Findings

1. **F1 addressed.** `public-api.md` now defines every query request with its
   attached options, closed local/world scopes, collision shapes, partial
   policy, interest scope/acceptance/readiness, concrete result facts,
   coverage, lifecycle snapshots, and an opaque race-closing gap-resume token.
   Surface inputs are embedded in materials, while a callable builder-time
   dressing registry owns complete bounded descriptors and typed asset/style
   failures. Extension ABI v1 now fixes the inspection variants, state lease,
   diagnostics, packet/snapshot/inspection layouts, 128-byte candidate record,
   20-byte patch run, and mandatory exact revision preconditions. The revision
   also makes adjacent registry, content-source, operation-error, correlation,
   and telemetry payloads field-level rather than leaving new opaque names.
2. **F2 addressed.** `ResourceLimits` now independently bounds per-frame
   extraction records/bytes, concurrent live volumes, lifetime
   live-plus-tombstone records, presentation artifacts, dirty invalidations,
   dressing styles/instances, extension registrations, descriptor bytes,
   packets, and retained GPU state. Correlation is a fixed 16-byte value with
   no variable allocation. Each pool has defaults, hard/cross-limits,
   effective-config representation, pressure outcomes, telemetry
   `ResourceKind`/`ResourceUsage`, and boundary evidence. Persistence now
   explicitly requires the consumer-set capability flag; store/restore calls
   do not mutate it.
3. **F6 addressed.** P6 now has two blocking sub-workloads. P6a preserves the
   27-artifact local latency fixture. P6b exercises one legal 32,768-cell,
   512-brick dispersed mutation that invalidates exactly 13,824 interested
   artifacts, drains them fairly through the default 1,024 jobs, stays inside
   the 16,384 artifact/dirty records and default mesh pools, proves periodic
   progress, and requires every artifact to become current at the committed
   revision within two seconds. Either sub-workload failing blocks the v1
   presentation selection.

### Answers To Auditor

1. `live_volumes` bounds only concurrently live directory entries.
   `volume_records` is a separate permanent lifetime bound for every accepted
   stable key, including retained tombstones. Retirement releases a live slot
   but not a lifetime record; create and restore have distinct stable capacity
   failures, and whole-world manifest counts cannot exceed either applicable
   bound.
2. Dressing uses a separate builder-time style registry.
   `MoriaBuilder::register_dressing_style` registers a stable style key,
   exact bounded material-key filter, mesh/material handles, density,
   coverage, scale/orientation, and per-artifact instance maximum.
   `MaterialDefinition` embeds its surface descriptor by value.

### TDD Changes Made

- Expanded `public-api.md` with complete query, interest, result, snapshot,
  observation, dressing, correlation, error, content-source, telemetry, and
  Extension ABI v1 contracts.
- Added every missing resource field and reconciled its ownership,
  cross-limit, capability behavior, effective reporting, pressure, telemetry,
  persistence, and exhaustion semantics across the TDD.
- Defined bounded exact presentation invalidation, volume-dirty coalescing,
  fair scheduling, instance/style ownership, and typed asset failure.
- Added technical decisions T14–T16 for lifetime volume records, builder-time
  dressing registration, and the closed GPU extension ABI.
- Strengthened automated, real-GPU, public-boundary, failure, portability, and
  P6 feasibility evidence for the revised contracts.

### Questions For Auditor

None.

### Notes To Auditor

The 13,824-artifact workload is intentionally sparse in geometry
(96–128 vertices and at most 768 indices per artifact) so all affected current
artifacts fit the default global mesh pools. This isolates invalidation,
bounded backlog, fair progress, and current-revision installation at the
maximum legal mutation fan-out instead of turning P6b into an unrelated
maximum-output-memory test.

## Auditor Turn — 2026-07-27T19:07:24Z

Mode: continue

Responding to: 2026-07-27T18:56:09Z

### Prior Findings Status

#### F1 — partially_resolved — Most payloads are concrete, but bounded collision and failure contracts are still incomplete

The revision does define the query/interest/snapshot/result records, dressing
registration, correlation value, extension inspection variants, state lease,
diagnostics, and fixed ABI layouts claimed by the coder. Those portions are
resolved.

Three normative gaps remain:

- `QueryOptions` bounds returned results but contains no candidate-cell,
  candidate-brick, or traversal-work bound for shape occupancy, trace, overlap,
  or sweep (`docs/tdd/public-api.md:1253-1349`). The fixed limits list names
  only 4,096 collision hits (`docs/tdd/public-api.md:406-412`), while the
  collision design says an unspecified candidate bound is computed and enforced
  (`docs/tdd/collision-and-presentation.md:38-40`) and P4 invents a
  65,536-candidate-cell authorization that no public request can express
  (`docs/tdd/validation.md:296`). A finite shape/segment can still cover an
  enormous number of cells with a small hit budget, so result bytes do not
  bound compute work.
- Result-cap truncation is not representable honestly. The TDD permits a
  collision result to exceed its hit cap when partial coverage was requested
  (`docs/tdd/collision-and-presentation.md:97-100`;
  `docs/tdd/public-api.md:1597-1602`), but `QueryCompleteness::PartialRequested`
  can report only brick coverage and `UnavailableRegion` reasons
  (`docs/tdd/public-api.md:1392-1433`). It has no required-hit count,
  result-limit reason, continuation, or indication that an inspected brick's
  contacts were truncated. A fully inspected region can therefore return an
  apparently complete coverage mask with missing ordered hits.
- The typed failure surface still contradicts the lifecycle contract.
  `RegionLifecycleState::Failed` retains only retryability
  (`docs/tdd/public-api.md:1527-1540`), although the approved design and TDD
  require consumer-visible cause plus retryability
  (`docs/design-document.md:410-423`; `docs/tdd/lifecycles.md:31-41`).
  Startup similarly returns `Receipt<StartupApplied>` through the generic
  `OperationErrorKind`, which has no renderer-unavailable, unsupported-
  capabilities report, startup stage, or aggregated causes
  (`docs/tdd/public-api.md:85-107,669-703`), while architecture names an
  undefined `StartupError::RendererUnavailable`, promises a complete scoped
  startup cause, and promises a deterministic unsupported-capabilities report
  (`docs/tdd/architecture.md:137-141,277-283,317-320`).

Define an explicit collision traversal authorization and supported bounds,
select whether hit overflow always fails or add truthful truncation metadata,
and make startup/region failure shapes carry the promised machine-actionable
causes.

#### F2 — partially_resolved — New named limits exist, but several retained or fallback allocations are still not bounded coherently

The extraction, volume-lifetime, presentation, dressing, and extension fields
added in this revision resolve the concrete omissions cited in the prior turn.
The capability-flag contradiction is also resolved.

The revised resource model still has these defects:

- Exact presentation invalidations collapse to one marker *per volume*
  (`docs/tdd/collision-and-presentation.md:117-124`), but the only cross-limit is
  `presentation_dirty_records >= presentation_jobs`; it need not cover
  `live_volumes` (`docs/tdd/public-api.md:397-400`). Two or more volumes may
  publish mutations concurrently after the dirty pool has collapsed. With no
  free record for each volume and no world-level fallback, the claim that truth
  publication never waits and no eventual-current obligation is lost is not
  implementable for every legal configuration.
- `max_material_metadata_bytes` is explicitly a per-registration maximum
  (`docs/tdd/public-api.md:127,323,380`), but `MaterialMetadataBytes` is also
  presented as one `ResourceKind`/`EffectiveLimit` and the telemetry contract
  treats every kind as a pool with `used`, `high_water`, and `effective`
  (`docs/tdd/public-api.md:294-303,1705-1745,2263-2279`). There is no aggregate
  retained metadata-byte limit, and treating the per-record maximum as the pool
  limit makes ordinary multiple registrations exceed their reported effective
  capacity.
- The observation ring is bounded only by fact count
  (`docs/tdd/public-api.md:337,391`), although a retained checkpoint fact owns a
  revision vector of up to `volume_records` entries
  (`docs/tdd/public-api.md:1685-1688,1792-1793`). No observation payload-byte
  pool or fixed indirection bounds the aggregate retained allocation.
- `BaseBrickRequest.bricks` is documented as no larger than a “content batch
  bound,” but no such count limit or selected batching rule exists
  (`docs/tdd/public-api.md:1039-1053`). `content_requests` and response bytes do
  not by themselves select the request-record count used by the callback or the
  P5 four-batch workload.

Add the missing aggregate/count bounds and their defaults, pressure behavior,
telemetry, and boundary tests, or select fixed-size/indirected representations.
For presentation, either enforce enough dirty records for every possible
per-volume marker or define a further bounded world-level fallback.

#### F3 — resolved — Persistence read, scope, and restore membership remain implementable

The bounded reader, whole-world scope, restore/import request, exact live-volume
membership, tombstone handling, and extra-material rule remain internally
consistent (`docs/tdd/public-api.md:1857-2000`;
`docs/tdd/persistence.md:125-188`).

#### F4 — resolved — Extension effect fan-out remains fully pre-reserved

The complete worst-case child record, payload, and receipt batch is reserved
before dispatch, unused capacity is released, and child receipts are returned
without creating a privileged admission path
(`docs/tdd/public-api.md:572-607,2186-2237`).

#### F5 — resolved — Collision/query dependency direction remains acyclic

The architecture, module ownership, and intended `AGENTS.md` consistently make
collision a private storage-level kernel consumed by query
(`docs/tdd/architecture.md:77-100,208-222`;
`docs/tdd/overview.md:218-229`).

#### F6 — partially_resolved — Maximum-command presentation is gated, but the local gate's claimed workload is impossible

P6b now exercises the 512-brick/13,824-artifact fair-drain case with bounded
jobs and pools, which resolves the missing maximum-command workload.

P6a says one boundary-cell mutation invalidates exactly the maximum 27
halo-dependent artifacts (`docs/tdd/validation.md:298`). With an 8³ artifact
core and a one-cell halo (`docs/tdd/collision-and-presentation.md:104-110`), one
discrete cell can belong to at most two artifact read domains per axis, hence
at most `2^3 = 8` artifacts. Twenty-seven is a valid union bound for edits
touching appropriately selected cells on both sides of a brick, not for one
cell. Correct the source-field statement and make P6a either an eight-artifact
single-corner-cell fixture or a specified multi-cell one-brick edit whose union
really is all 27 neighbors. Keep P6b unchanged.

#### F7 — resolved — Material ID capacity remains consistent

The registry, GPU, and persistence contracts consistently permit 65,535
nonempty registrations plus reserved empty ID zero and require the correct
boundary evidence.

### New Findings

#### F8 — unresolved — Cancellation has two incompatible linearization points

The binding overview says explicit cancellation is guaranteed before GPU
submission (`docs/tdd/overview.md:77-78`), and
`ShutdownPolicy::CancelUnsubmitted` says shutdown cancels all unsubmitted work
(`docs/tdd/lifecycles.md:275-283`). The command lifecycle instead permits
cancellation only while queued/waiting and before preparation reserves GPU
resources (`docs/tdd/lifecycles.md:82-101`), leaving a `Preparing` but not yet
`Submitted` operation outside both stated rules. The public
`CancelRequest::Requested` result does not distinguish acceptance from a
too-late request; only eventual terminal status is described
(`docs/tdd/public-api.md:624-649,735-738`).

Select one exact cancellation linearization point for every operation family
and shutdown. If preparation is the point of no return, rename the terminal
outcome and shutdown policy accordingly and expose a typed too-late result. If
submission is the point, define cleanup/release for cancellation during
preparation. Add state-machine tests at each boundary.

#### F9 — unresolved — Long-lived world filters have no semantics when volume membership or placement changes

`BoundedVolumeFilter::All { max_volumes }` is used by world interest and
subscriptions, and a world interest also has a fixed `max_bricks`
(`docs/tdd/public-api.md:932-960,1604-1613`). The TDD defines atomic interest
replacement only when the consumer calls `update`; it does not say whether an
accepted `All` filter is a pinned resolved set or dynamically includes later
`VolumeCommand::Create` volumes. It likewise does not say how a world-bounds
interest changes when a dynamic volume moves into or out of the bound. Dynamic
reevaluation can exceed the already accepted `max_volumes`/`max_bricks`, while
pinning silently changes the meaning of `All` and world-bounds interest. The
same ambiguity affects whether a long-lived `All` observation subscription
sees later-created volumes.

Select snapshot-versus-live membership for each long-lived filter. If live,
define bounded reevaluation, overflow/failure state, observations, and recovery
after capacity returns; if pinned, expose the resolved IDs and state explicitly
that create/move does not expand the lease/subscription. Validate create,
retire, and move across accepted world scopes.

### Questions For Coder

1. Is partial collision intended to omit only unavailable spatial regions, or
   may it also truncate hits from a region that was fully inspected? The result
   type currently supports only the former.
2. For dirty-presentation pressure across several volumes, do you prefer the
   cross-limit `presentation_dirty_records >= live_volumes`, or a single
   additional world-dirty epoch fallback?
3. Should `BoundedVolumeFilter::All` on a long-lived interest/subscription be a
   snapshot of accepted volume IDs or a live set that reacts to create/retire
   and movement?

### Answers To Coder

The coder asked no direct question. I acknowledge that the claimed query/ABI
record definitions, separate lifetime volume capacity, builder-time dressing
registry, extension batch reservation, and P6b maximum-command workload are
present in the TDD.

### Notes To Coder

No protected human question is outstanding. These are ordinary interface,
resource, lifecycle, and validation selections under the approved authority.
The P6b sparse-output choice is reasonable and does fit the default mesh pools;
the remaining P6 issue is only the impossible single-cell P6a fixture.

### Approval

Not approved.

## Coder Turn — 2026-07-27T19:22:45Z

Mode: continue

Responding to: 2026-07-27T19:07:24Z

### Responses To Findings

1. **F1 addressed.** Collision-capable queries now carry an explicit
   `TraversalAuthorization` bounded to 8,192 candidate bricks and 65,536
   candidate cells, independently of result bytes and hit count. Conservative
   transformed work is checked before admission. Partial results may omit only
   explicitly unavailable spatial regions; hit-cap overflow always fails with
   `OutputOverflow` and returns no truncated result. Region failure now carries
   a closed cause, retryability, device generation, and bounded diagnostic.
   Startup failure is a staged, aggregated `StartupFailure` with distinct
   renderer absence, unsupported feature/limit report, shader, persistence,
   restore, allocation, and invariant causes.
2. **F2 addressed.** Presentation dirty storage is partitioned so one marker is
   permanently reserved for every live-volume slot, with the enforced
   cross-limit `presentation_dirty_records >= live_volumes +
   presentation_jobs`; simultaneous volume commits cannot consume each other's
   fallback. `material_metadata_bytes` now bounds aggregate retained metadata
   separately from the per-registration maximum.
   `observation_payload_bytes` independently bounds ring payload allocation,
   guarantees one maximum checkpoint fact, and evicts only whole facts.
   `content_bricks_per_request` fixes callback batch count, response
   cross-limits, deterministic partitioning, telemetry, and pressure behavior.
3. **F6 addressed.** P6a is now a sparse patch of the eight corner cells in one
   8³ brick. Their halo-dependency union is exactly the 3×3×3 set of 27
   artifacts. The TDD also states that one cell can affect at most eight
   artifacts. P6b is unchanged.
4. **F8 addressed.** The exact cancellation linearization point is the atomic
   transition from `Queued | WaitingForMatter` to `Preparing` for every
   cancellable family. `Accepted` installs terminal
   `CancelledBeforePreparation` before returning; `TooLate { stage }` exposes
   a lost race; startup/shutdown receipts are noncancellable.
   `ShutdownPolicy::CancelNotPrepared` uses the same transition and drains
   `Preparing` and later work. State-machine evidence covers both sides of the
   boundary and every later stage.
5. **F9 addressed.** Long-lived `All` filters are snapshots. Interest accepts
   and exposes pinned volume IDs, placements/revisions, and exact local brick
   sets; create/retire/move never expands or recomputes them, and `update` is
   the refresh path. Subscriptions expose pinned accepted IDs, exclude later
   creates, and use world bounds only as an event predicate over those IDs;
   move facts match old or new bounds, and resubscription refreshes membership.
   Gap snapshots and resume tokens bind the resolved membership.

### Answers To Auditor

1. Partial collision omits only unavailable spatial regions. It never
   truncates hits in an inspected region; hit overflow fails under both partial
   policies.
2. Dirty presentation uses the cross-limit plus a physically partitioned,
   per-live-volume marker reservation. This is stronger than only requiring
   `presentation_dirty_records >= live_volumes` because exact keys and jobs
   cannot consume the marker partition.
3. `BoundedVolumeFilter::All` is a snapshot for both long-lived interests and
   subscriptions. Accepted resolved IDs are public; interests also freeze
   local bricks. Explicit update/resubscription is required to observe new
   membership.

### TDD Changes Made

- Expanded `public-api.md` with traversal budgets, typed startup/region
  failures, truthful partial semantics, complete aggregate/count resource
  limits, preparation-based cancellation results, and accepted resolved
  interest/subscription records.
- Reconciled collision, invalidation, lifecycle, startup, storage pressure,
  telemetry, extension lifecycle records, and shutdown semantics across
  `architecture.md`, `collision-and-presentation.md`, `lifecycles.md`, and
  `state-and-storage.md`.
- Added decisions T17–T20 for collision work/truncation, cancellation,
  snapshot long-lived filters, and retained allocation bounds.
- Strengthened host, state-machine, real-GPU, public-scenario, failure, P5, and
  P6 validation fixtures for the selected contracts.

### Questions For Auditor

None.

### Notes To Auditor

No human-authority question is open. Snapshot filter membership was selected
because it preserves already admitted volume/brick bounds; the accepted
records make that choice visible rather than silently redefining `All`.

## Auditor Turn — 2026-07-27T19:29:24Z

Mode: continue

Responding to: 2026-07-27T19:22:45Z

### Prior Findings Status

#### F1 — resolved — Collision authorization, partial-result honesty, and typed startup/region failure are now coherent

Every collision-bearing public query now requires an explicit
`TraversalAuthorization`; admission checked-sums conservative candidate brick
and cell work against both request and fixed maxima
(`docs/tdd/public-api.md:1370-1381,1695-1704`). Collision output overflow fails
under both partial policies, while partial success can omit only named
unavailable regions (`docs/tdd/public-api.md:1750-1759`;
`docs/tdd/collision-and-presentation.md:91-103`).

`RegionFailure` now retains a closed machine-actionable cause, retryability,
generation, and bounded diagnostic (`docs/tdd/public-api.md:1657-1681`).
Startup failures distinguish renderer lookup, capability aggregation, device,
persistence, restore, and directory stages through the public receipt error
(`docs/tdd/public-api.md:714-777,834-841`;
`docs/tdd/architecture.md:137-143,284-294`). This resolves the prior finding.

#### F2 — partially_resolved — The missing named limits exist, but content-response byte admission is still unspecified

The revision correctly adds separate aggregate material-metadata and
observation-payload pools, a fixed `content_bricks_per_request`, and the
partitioned per-live-volume presentation fallback
(`docs/tdd/public-api.md:321-362,381-410,450-469`;
`docs/tdd/collision-and-presentation.md:117-142`). Their cross-limits,
telemetry kinds, and boundary tests resolve the previously cited metadata,
observation, batching-count, and simultaneous-presentation omissions.

The content callback path is not yet implementably bounded by its aggregate
byte pool. `content_requests` permits multiple callbacks in flight and
`content_response_bytes` is one aggregate `ResourceKind`, while each callback
may return an owned batch of up to `content_bricks_per_request` detailed boxes
(`docs/tdd/public-api.md:345-347,400,467-469,1133-1175`). The request carries
`maximum_encoded_bytes`, but the TDD never states that Moria reserves that many
bytes from `content_response_bytes` before invoking consumer code, how callback
admission behaves when count capacity exists but byte capacity does not, or
when that reservation is released (`docs/tdd/public-api.md:1156-1169,
1197-1209`). Rejecting an oversized result after the callback returns does not
bound the simultaneous host allocations already created by several callbacks.

Define the content-worker admission protocol: reserve an exact or conservative
response-byte permit before invocation, set `maximum_encoded_bytes` from that
permit, defer/reject batches when either count or bytes is unavailable, and
release bytes at a named installation/failure/cancellation milestone. Add a
pressure test in which count slots remain but response-byte capacity is
exhausted.

#### F3 — resolved — Persistence read, restore scope, and membership remain implementable

The bounded reader, whole-world checkpoint/import request, exact live-volume
membership, tombstone handling, and extra-material rule remain coherent
(`docs/tdd/public-api.md:2047-2190`; `docs/tdd/persistence.md:125-188`).

#### F4 — resolved — Extension effect fan-out remains fully pre-reserved

Worst-case child records, aggregate payload, and completion slots are reserved
before dispatch; invalid output admits none and unused capacity is released
(`docs/tdd/public-api.md:597-618,2388-2432`).

#### F5 — resolved — Collision/query dependency direction remains acyclic

Architecture, ownership, and intended repository instructions consistently
place the private collision kernel below public query orchestration
(`docs/tdd/architecture.md:84-98,210-232`;
`docs/tdd/overview.md:221-229`).

#### F6 — resolved — Both presentation feasibility fixtures are now physically possible

P6a now mutates the eight corner cells of one brick, whose halo union is the
declared 27 artifacts; P6b retains the 512-brick/13,824-artifact legal-command
fair-drain case (`docs/tdd/validation.md:319-324`;
`docs/tdd/collision-and-presentation.md:111-117`).

#### F7 — resolved — Material ID capacity remains consistent

The public registry, GPU sample format, and persistence boundary consistently
allow IDs 1 through 65,535 plus reserved empty ID zero
(`docs/tdd/state-and-storage.md:42-51`;
`docs/tdd/persistence.md:233-236`).

#### F8 — resolved — Cancellation now has one observable linearization point

All cancellable families use the atomic
`Queued | WaitingForMatter -> Preparing` race; a winning cancellation installs
`CancelledBeforePreparation`, a loss returns `TooLate { stage }`, and startup
and shutdown are noncancellable (`docs/tdd/public-api.md:661-676,811-826`;
`docs/tdd/lifecycles.md:100-121`). `CancelNotPrepared` uses the same boundary
and drains later stages (`docs/tdd/lifecycles.md:301-315`). The state-machine
matrix exercises both sides and every later stage
(`docs/tdd/validation.md:58-64`).

#### F9 — partially_resolved — Snapshot membership is selected, but retained observation filtering and retired-member resnapshot are not representable

The revision explicitly makes `All` a snapshot for interests and
subscriptions, exposes accepted IDs, freezes interest bricks, and requires
update/resubscription to refresh membership
(`docs/tdd/public-api.md:1051-1063,1101-1115,1925-1945,1961-1975`). That
resolves the original snapshot-versus-live ambiguity.

Two adjacent contracts still prevent the selected subscription behavior from
being implemented:

- Filtering occurs when a subscriber polls the shared ring, potentially long
  after the fact was appended (`docs/tdd/lifecycles.md:168-177`). Yet
  world-bounds predicates are defined at the fact revision and move predicates
  must match either the prior or new placed domain
  (`docs/tdd/public-api.md:1969-1975`). The retained public facts do not contain
  sufficient geometry: `MatterCommitted` retains local affected bounds but no
  placement, lifecycle and presentation facts retain local bounds/bricks, and
  `VolumeMoved` retains only the new placement
  (`docs/tdd/public-api.md:1797-1840`). Current directory state cannot
  reconstruct a prior placement after later moves. No bounded internal ring
  envelope, revision pin, or append-time match representation is selected.
- Retirement invalidates the runtime handle
  (`docs/tdd/public-api.md:1335-1341`), but a subscription-gap snapshot promises
  the exact pinned membership and revisions while `WorldSnapshot` can represent
  only live `VolumeStateSnapshot` records; it has no retired/tombstone member
  record carrying the terminal revision
  (`docs/tdd/public-api.md:1632-1655,1724-1731`). If the retirement observation
  itself was overwritten, the resnapshot contract does not tell the consumer
  how that pinned member is represented rather than silently absent.

Select a bounded retained filter representation—such as per-fact prior/new
world bounds or a charged internal filter envelope—and include it in
`observation_payload_bytes`. Define a subscription-gap snapshot record for a
pinned member retired during the gap, including stable identity and terminal
revision (or explicitly define an equally unambiguous typed absence). Then test
polling after multiple moves have reclaimed older directory versions and gap
recovery after the retirement fact has been overwritten.

### New Findings

No new finding family beyond the unresolved resource and long-lived-filter
contracts above.

### Questions For Coder

1. Will observation facts retain revision-time world filter bounds (including
   both prior and new move bounds), or will the ring use another bounded
   charged filter envelope?
2. What typed record will a subscription-gap snapshot return for a pinned
   volume whose retirement fact was lost?
3. Will each content callback reserve its worst-case response bytes before
   invocation, or will batching shrink to the currently available byte permit?

### Answers To Coder

The coder asked no direct question. I acknowledge and verified the stated
choices: partial collision omits only unavailable regions, presentation uses
partitioned per-volume fallback markers, and long-lived `All` membership is a
snapshot. The remaining F9 issue is not the membership choice; it is the
retained data needed to execute that choice after moves, retirement, and a
gap.

### Notes To Coder

No protected human decision is open. The remaining corrections are ordinary
bounded transport and observation-state design. The newly added validation
language for create/retire/move is directionally correct, but the production
record shapes must first contain the state those tests are meant to assert.

### Approval

Not approved.

## Coder Turn — 2026-07-27T19:35:21Z

Mode: continue

Responding to: 2026-07-27T19:29:24Z

### Responses To Findings

1. **F2 addressed.** Before invoking a content source, the scheduler now
   atomically acquires both one callback-count slot and a conservative
   worst-case `content_response_bytes` permit for the already formed batch.
   `maximum_encoded_bytes` is that permit. If either resource is unavailable,
   the batch remains queued, consumer code is not invoked, no partial resource
   hold remains, and deferred pressure is observable. A valid smaller response
   releases its unused reservation after validation; the rest is released only
   after install/failure cleanup and drop. The response charge includes bounded
   per-result ownership/control storage plus homogeneous or detailed sample
   bytes. Validation now includes the case where count slots remain free but
   aggregate response bytes are exhausted.
2. **F9 addressed.** Each ring fact now retains a private fixed 128-byte
   `FilterEnvelopeV1`, charged to `observation_payload_bytes`, with append-time
   local/world geometry. Move envelopes retain both prior and new placed-domain
   bounds; retirement retains the final live extent. Polling filters only
   against this immutable envelope, so reclaimed directory versions cannot
   change historical matches. `VolumeStateSnapshot` now has typed `Live` and
   `Retired` states. A subscription-gap snapshot returns exactly one record per
   pinned member; a lost retirement fact therefore produces a retired record
   with the accepted historical ID, stable key, and terminal revision instead
   of silent absence.

### Answers To Auditor

1. The ring uses a separate bounded, charged filter envelope. It retains
   revision-time local/world bounds, including both prior and new world bounds
   for a move, without enlarging the public observation fact payload.
2. The gap snapshot returns
   `VolumeStateSnapshot { volume, key, state:
   SnapshotVolumeState::Retired { terminal_revision } }`. The historical
   runtime ID identifies the accepted member but remains stale for operations;
   the stable key is durable identity.
3. Each callback reserves the already formed batch's full conservative
   worst-case response bytes before invocation. Batching does not shrink to
   currently available bytes; an unavailable dual permit defers the batch
   without invoking the source or holding only one resource.

### TDD Changes Made

- Specified atomic count-plus-byte admission, pressure behavior, exact response
  charging, cancellation, validation, installation, drop, and release
  milestones for `BaseContentSource`.
- Added the fixed append-time observation filter envelope, its payload-pool
  accounting/cross-limit, old/new move semantics, and poll-time matching rules.
- Added typed live/retired gap snapshot membership and made complete volume
  state mandatory for `SubscriptionGap` snapshots.
- Reconciled module ownership, lifecycle, resource tables, technical decisions,
  host/state-machine tests, and the C7 public scenario with those contracts.

### Questions For Auditor

None.

### Notes To Auditor

No human-authority question is open. The private filter envelope is
intentionally fixed-size and co-retained with its fact: it preserves
revision-time spatial filtering without pinning historical directory versions
or turning placement history into a second public truth channel.

## Auditor Turn — 2026-07-27T19:41:46Z

Mode: continue

Responding to: 2026-07-27T19:35:21Z

### Prior Findings Status

#### F1 — resolved — Collision work, partial results, and typed failures remain coherent

Collision-bearing queries retain separate conservative brick/cell traversal
authorization, hit overflow fails without a truncated result, and startup plus
region failures remain closed and actionable
(`docs/tdd/public-api.md:714-777,1695-1704,1789-1806`;
`docs/tdd/collision-and-presentation.md:26-103`).

#### F2 — partially_resolved — Callback admission now reserves bytes first, but the permitted returned allocation can still exceed that reservation

The scheduler now atomically reserves the callback slot and
`256 + 2,080 * request.bricks.len()` response bytes before invocation, defers
without a partial hold, and releases ownership only after drop
(`docs/tdd/public-api.md:469-476,1204-1239`). This resolves the missing
count-plus-byte admission protocol.

The response validation nevertheless permits
`BaseBrickBatch.bricks.capacity()` up to the configured
`content_bricks_per_request`, while the permit and exact charge use the
current request/result length (`docs/tdd/public-api.md:1173-1176,1209-1211,
1224-1230`). For a one-brick batch under the 4,096-brick configuration, a
source may therefore return a vector allocation sized for 4,096 enum slots
while being charged for one 32-byte control record. The returned
`SourceDescriptor` also contains `ContentLineage.opaque: Vec<u8>`, despite the
claim that no other variable payload may be returned
(`docs/tdd/public-api.md:1150-1161,1173-1176,1227-1229`). Length validation
after return cannot make those simultaneously live allocations fit the
pre-invocation permit.

Make the charged ownership representation exact: require/canonicalize result
capacity to the returned length (or charge actual capacity before shrinking),
and either remove the echoed variable descriptor from the response, copy it
into fixed/canonical bounded storage, or include its actual retained allocation
in the conservative permit. Add a short-batch/max-capacity adversarial callback
test and assert aggregate live returned allocation never exceeds
`content_response_bytes`.

#### F3 — resolved — Persistence read, restore scope, and membership remain implementable

The bounded reader, atomic store publication, exact live membership, retained
tombstones, and stable-key restore rules remain explicit
(`docs/tdd/public-api.md:2121-2264`; `docs/tdd/persistence.md:125-188`).

#### F4 — resolved — Extension effect fan-out remains fully pre-reserved

The complete candidate child record, payload, and completion capacity is
reserved before dispatch, and invalid output admits no child
(`docs/tdd/public-api.md:597-618,2481-2510`).

#### F5 — resolved — Collision/query dependency direction remains acyclic

The private collision kernel remains below query orchestration in ownership,
dependencies, and intended repository instructions
(`docs/tdd/architecture.md:86-100,215-232`;
`docs/tdd/overview.md:221-229`).

#### F6 — resolved — Presentation feasibility covers both local and maximum-command fan-out

P6a uses the physically valid eight-corner-cell/27-artifact union, and P6b
covers the 512-brick/13,824-artifact fair drain
(`docs/tdd/validation.md:319-345`;
`docs/tdd/collision-and-presentation.md:106-142`).

#### F7 — resolved — Material ID capacity remains consistent

The registry, sample format, and persistence tests consistently represent
65,535 nonempty IDs plus reserved empty zero
(`docs/tdd/state-and-storage.md:29-55`;
`docs/tdd/persistence.md:222-236`).

#### F8 — resolved — Cancellation retains one observable point of no return

All cancellable operation families race on entry to `Preparing`, later stages
are typed `TooLate`, and shutdown applies the same boundary
(`docs/tdd/public-api.md:661-676,811-826`;
`docs/tdd/lifecycles.md:87-121,307-325`).

#### F9 — resolved — Historical filtering and retired-member gap recovery are now representable

Each retained fact now has a fixed, byte-charged append-time envelope with
old/new move geometry, polling uses that envelope rather than reclaimed
directory history, and a gap snapshot returns one typed live or retired record
for every pinned member
(`docs/tdd/public-api.md:1609-1655,1765-1785,2011-2055`;
`docs/tdd/lifecycles.md:168-202`). The state-machine and C7 fixtures exercise
both reclaimed move history and an overwritten retirement fact
(`docs/tdd/validation.md:68-79,231-241`).

### New Findings

#### F10 — unresolved — Runtime volume definitions can leave an unbounded retained host allocation

Every other public debug name shown in the facade has a 96-byte bound, but
`VolumeDefinition.debug_name` is an unconstrained `String`
(`docs/tdd/public-api.md:101-103,122-126,155-158,169-178`). A definition may
arrive during startup or through `VolumeCommand::Create`; once applied, its
record outlives the command-payload permit, and the name is also part of the
persistence volume record (`docs/tdd/public-api.md:1320-1386`;
`docs/tdd/persistence.md:85-98`). `live_volumes` bounds record count but no
named pool bounds these retained bytes. The 64 MiB manifest cap only makes a
later checkpoint fail; it does not bound live host ownership.

Give volume names a fixed byte maximum (the existing 1..=96 convention is the
obvious choice), validate it for builder and runtime create paths before
admission, and require retained/canonical ownership not to preserve arbitrary
input capacity. Add maximum/one-over tests for both creation paths and a
maximum-count manifest-size check.

#### F11 — unresolved — GPU observation deltas have no gap, filtering, or cursor contract

`GpuInspectionQuery::ObservationDeltas` accepts a subscriber and arbitrary
`after` sequence, but the TDD does not say whether it advances that
subscriber's CPU cursor, reads independently, or validates `after` against the
cursor (`docs/tdd/public-api.md:2282-2305`). The ordinary subscriber must enter
`NeedsSnapshot` after overwrite and cannot deliver later facts until bounded
snapshot/resume (`docs/tdd/lifecycles.md:168-202`). The GPU packet has only
fact records: its header has no retained/gap status, trustworthy revision
frontier, or resume token, and facts that do not fit 64 bytes are merely said
to be “rejected” without defining the failed sequence or terminal result
(`docs/tdd/public-api.md:2399-2435`). Consequently an implementation cannot
tell a shader whether deltas are complete, whether history was lost, or
whether skipping an unsupported fact would create silent event loss.

Select one concrete model. Define subscriber ownership/cursor effects,
append-time envelope filtering, legal `after` values, retained-head capture,
maximum-record overflow, unsupported-fact behavior, and an explicit
gap/needs-snapshot outcome that cannot be mistaken for an empty delta packet.
Bind any resume to the ordinary snapshot protocol or state explicitly that the
GPU view is nonadvancing and cannot resume it. Extend the ABI and real-GPU/C9
evidence with filtered retained deltas, an overwritten sequence, an
unsupported large fact, and recovery without silent loss.

### Questions For Coder

1. Will content results be canonicalized to `capacity == len` before they can
   be retained, or will the permit charge actual allocation capacity?
2. Is `ObservationDeltas` intended to consume the ordinary subscriber cursor,
   or to be a nonadvancing retained-ring read tied only to that subscriber's
   accepted filter?

### Answers To Coder

The coder asked no direct question. I verified the claimed atomic
content-count/byte acquisition, fixed observation envelope, old/new move
filtering, and typed retired gap records in the actual TDD. F9 is resolved.

### Notes To Coder

No protected human question is open. F2 is now a narrow allocation-accounting
correction rather than a missing admission design. F10 and F11 are ordinary
bounded-host and extension-interface selections under the approved authority;
they do not require new product behavior.

### Approval

Not approved.

## Coder Turn — 2026-07-27T19:54:25Z

Mode: continue

Responding to: 2026-07-27T19:41:46Z

### Responses To Findings

1. **F2 addressed.** `BaseBrickBatch` now returns an exact
   `Box<[BaseBrickResult]>`, so a valid short batch cannot retain spare vector
   capacity. `ContentLineage.opaque` is also an exact boxed slice, and the
   response no longer echoes `SourceDescriptor`; Moria compares and drops a
   bounded temporary descriptor before invoking the callback. The existing
   pre-invocation permit therefore covers all valid returned ownership, and
   validation includes the short-result/max-source-capacity adversarial case.
2. **F10 addressed.** Every `VolumeDefinition.debug_name` is limited to
   1..=96 UTF-8 bytes for builder and runtime-create paths. Accepted names are
   canonicalized into exact `Box<str>` directory/tombstone ownership, so
   command input capacity is not retained. Persistence sizing and tests cover
   96/97-byte boundaries and a maximum-record-count, maximum-name no-scar
   manifest under the independent 64 MiB cap.
3. **F11 addressed.** `ObservationDeltas` is now a nonadvancing retained-ring
   read tied to the subscriber's accepted snapshot/filter and explicit initial
   cursor floor. Capture freezes oldest/head and filters with append-time
   envelopes. The ABI and public result distinguish `Complete`,
   `MoreAvailable`, `NeedsSnapshot`, and `UnsupportedFact`; gaps and matching
   unsupported facts emit zero records, forbid effects, and never skip a
   sequence. A bounded non-resuming `SubscriptionState` snapshot reconciles
   state before the caller restarts after its head, without mutating or
   resuming the CPU cursor.

### Answers To Auditor

1. Content results are canonical exact allocations before Moria can retain
   them: the response type is `Box<[BaseBrickResult]>`, not a capacity-bearing
   `Vec`, and it carries no echoed variable descriptor.
2. `ObservationDeltas` does not consume or advance the ordinary subscriber
   cursor. It is an independently paged retained-ring read that reuses only
   that subscriber's accepted membership, kinds, spatial filter, and
   append-time envelopes.
3. `BoundedVolumeFilter::All` on a long-lived interest or subscription is a
   snapshot of all volumes live at acceptance, not a live-growing set.
   Interest `update` or resubscription is required to include later-created
   volumes; retirement never substitutes another member.

### TDD Changes Made

- Replaced capacity-bearing content result/lineage ownership and removed the
  variable response descriptor from `public-api.md`; added exact allocation
  validation and decision T23.
- Bounded and canonicalized retained volume names across startup, runtime
  creation, tombstones, persistence, tests, and decision T24.
- Added nonadvancing GPU delta cursor semantics, complete 128-byte supported
  fact layouts, explicit packet status/frontier/cursor fields, unsupported-fact
  boundaries, subscription-state snapshot recovery, render extraction
  ownership, lifecycle rules, real-GPU/C9 evidence, and decision T25.

### Questions For Auditor

None.

### Notes To Auditor

No human-authority question is open. `All` remains the previously selected
snapshot model. The GPU delta seam deliberately does not compete with CPU
polling: an empty complete packet is distinguishable from capacity paging,
history loss, and a matching fact that ABI v1 cannot represent.

## Auditor Turn — 2026-07-27T20:01:28Z

Mode: continue

Responding to: 2026-07-27T19:54:25Z

### Prior Findings Status

#### F1 — resolved — Public payload, traversal, partial-result, and failure contracts remain concrete

Every query family has a field-level request/result contract, collision work is
separately authorized, hit overflow is terminal rather than silently
truncated, and startup/region failures retain actionable causes
(`docs/tdd/public-api.md:1406-1838`;
`docs/tdd/collision-and-presentation.md:26-103`).

#### F2 — partially_resolved — Valid callback results are exact, but invalid returned ownership can still exceed the pre-reserved pool

The response no longer echoes `SourceDescriptor`, lineage uses an exact boxed
slice, and a valid result slice has no spare vector capacity. Those corrections
resolve the previously demonstrated short-result/capacity case.

The permit is nevertheless sized from the request length as
`256 + 2,080 * bricks.len()`, while the public callback can return an
arbitrary-length `Box<[BaseBrickResult]>`; only after return does Moria reject
an omitted, duplicate, overlength, or excess-byte batch
(`docs/tdd/public-api.md:1177-1185,1211-1238`). For a one-brick request, a
misbehaving source can return an exact 4,096-element boxed slice, each element
owning a detailed 2 KiB box. That allocation has already crossed into Moria's
ownership before validation and can coexist with other callbacks despite only
one-brick response bytes having been reserved. Exact capacity therefore bounds
valid retained results, but it does not make the claim that *every concurrently
returned response* fits `content_response_bytes` true
(`docs/tdd/public-api.md:1240-1247`). The added validation fixture exercises
spare capacity converted to one result, not an invalid overlength transfer
(`docs/tdd/validation.md:52-56`).

Make the ownership crossing itself bounded, for example by giving the callback
a Moria-owned exact-length output sink/preallocated slots, using an opaque
permit-bound batch builder that cannot produce more than the request, or
selecting an equivalent fixed-count interface. Invalid material IDs and other
contents can still fail normally, but a source must not be able to transfer
more owned result storage than was reserved. Add concurrent overlength and
oversized-detailed-output adversarial cases and assert the process-visible
Moria-owned response high-water remains within
`content_response_bytes`.

#### F3 — resolved — Persistence read, restore scope, and membership remain implementable

Reader sizing/range semantics, whole-world manifests, stable identity, exact
live-volume membership, and base-lineage/fingerprint validation remain
explicit (`docs/tdd/public-api.md:2156-2299`;
`docs/tdd/persistence.md:1-225`).

#### F4 — resolved — Extension effect fan-out remains fully pre-reserved

The shader's worst-case child record, payload, and completion capacity is
reserved before packet capture, and invalid candidate output admits no child
(`docs/tdd/public-api.md:608-638,2584-2625`).

#### F5 — resolved — Collision/query ownership remains acyclic

Collision remains a private storage-level fact kernel consumed by public query
orchestration, consistently in architecture and intended repository rules
(`docs/tdd/architecture.md:77-100,215-232`;
`docs/tdd/overview.md:218-229`).

#### F6 — resolved — Presentation feasibility covers physical local and maximum-command workloads

P6a exercises the valid eight-corner-cell/27-artifact union and P6b exercises
the 512-brick/13,824-artifact bounded fair drain
(`docs/tdd/validation.md:359-368`).

#### F7 — resolved — Material ID capacity remains consistent

The runtime and persistence contracts consistently permit 65,535 registered
nonempty IDs plus reserved empty ID zero
(`docs/tdd/state-and-storage.md:29-55`;
`docs/tdd/persistence.md:227-247`).

#### F8 — resolved — Cancellation retains one observable preparation boundary

Queued/waiting cancellation and the transition to `Preparing` form one atomic
race for every cancellable operation, while startup and shutdown remain
noncancellable (`docs/tdd/public-api.md:822-837`;
`docs/tdd/lifecycles.md:87-121`).

#### F9 — resolved — Snapshot membership, historical filtering, and retired gap records remain representable

Accepted filters pin membership, each fact retains its charged append-time
geometry, and gap snapshots return one typed live or retired record per pinned
member (`docs/tdd/public-api.md:1783-1810,2041-2089`;
`docs/tdd/lifecycles.md:168-212`).

#### F10 — resolved — Retained volume names are bounded and canonical

Both builder and runtime create paths validate 1..=96 UTF-8 bytes, accepted
directory/tombstone ownership is exact, and persistence sizing/tests cover the
boundary and maximum record count (`docs/tdd/public-api.md:169-183,1382-1401`;
`docs/tdd/persistence.md:93-106,227-247`).

#### F11 — partially_resolved — Delta cursor/gap semantics are concrete, but an empty observation history has no representable frontier

The revision does select a coherent nonadvancing model: it freezes the
subscriber's accepted filter and ring bounds, distinguishes
`Complete | MoreAvailable | NeedsSnapshot | UnsupportedFact`, forbids effects
at blocked boundaries, and keeps the CPU cursor unchanged
(`docs/tdd/public-api.md:2335-2465,2500-2549`;
`docs/tdd/lifecycles.md:203-212`). Those parts of F11 are resolved.

`ObservationSequence` is nonzero, however, while
`GpuObservationDeltaOutcome.oldest_retained` and `captured_head` and
`WorldSnapshot.observation_head` are mandatory nonoptional values
(`docs/tdd/public-api.md:40-42,1682-1691,2369-2375`). The same contract
explicitly uses `None` for a subscriber whose lower bound precedes the first
world sequence (`docs/tdd/public-api.md:2071-2075`), and no rule requires
startup to append an initial observation. A newly started world can therefore
accept a subscription and request an empty complete GPU delta or an explicit
region/`SubscriptionState` snapshot before any fact exists, but neither public
result can encode that no head/oldest sequence exists. ABI v1 reserves zero
only as the documented `None` encoding for the cursor; it gives no empty-ring
meaning to the mandatory oldest/head words
(`docs/tdd/public-api.md:2500-2522`). The current “empty complete page” test can
run after unrelated facts exist and does not close this case.

Represent the absent frontier explicitly—such as optional oldest/head fields
with zero as their ABI `None`, or a closed empty/nonempty frontier type—and
apply the same rule to snapshots. Define paging legality from `after = None`
when the capture is empty. Add host, ABI, real-GPU, and public-boundary cases
that subscribe and read/snapshot before the first observation and distinguish
that valid empty state from overwrite.

### New Findings

No new finding family is added. The remaining defects are incomplete
resolutions of F2's bounded callback ownership and F11's complete delta/frontier
state model.

### Questions For Coder

1. Will the content port switch to a Moria-owned exact-length output sink (or
   another type-level bounded builder), or is a different mechanism intended
   to prevent an invalid callback from transferring more ownership than its
   pre-reserved response bytes?
2. Should an observation frontier be optional before sequence 1, or do you
   intend startup to append a mandatory initial fact? If the latter, specify
   that fact, its filtering behavior, and its ordering before any subscription
   can be accepted.

### Answers To Coder

The coder asked no direct question. I acknowledge and verified the stated
answers: valid content results now have exact boxed ownership,
`VolumeDefinition.debug_name` is bounded/canonical on both creation paths,
`ObservationDeltas` is nonadvancing, and long-lived `All` remains snapshot
membership.

### Notes To Coder

No protected human question is open. Both remaining issues are ordinary
interface edge contracts. F2 does not require treating consumer-internal
temporary allocations as a Moria pool; it requires bounding the owned value
that the callback transfers back across the port. F11 should preserve the
selected nonadvancing model while making the pre-sequence-1 state explicit.

### Approval

Not approved.

## Coder Turn — 2026-07-27T20:10:23Z

Mode: continue

Responding to: 2026-07-27T20:01:28Z

### Responses To Findings

1. **F2 addressed.** `BaseContentSource::load_bricks` no longer returns owned
   result storage. Moria atomically reserves the callback slot and worst-case
   bytes, constructs an opaque exact-length `BaseBrickOutput` inside that
   permit, and lends it mutably to the callback. Homogeneous writes copy one
   sample; detailed writes borrow exactly 512 samples into a fixed slot.
   Overlength, invalid, ignored-error, incomplete, panic, and cancellation
   paths cannot transfer extra ownership and poison/drop the bounded sink
   before permit release. Concurrent adversarial validation now checks that
   Moria-owned output high-water never exceeds `content_response_bytes`.
2. **F11 addressed.** The public contract now has the closed
   `ObservationFrontier::Empty | Retained { oldest, head }` type.
   `WorldSnapshot` and `GpuObservationDeltaOutcome` use it. A pre-sequence-1
   delta read from `after = None` returns `Complete`, zero records, an empty
   frontier, and a `None` cursor; it is not a gap. ABI v1 encodes the empty
   frontier as zero oldest/head words and rejects a half-present pair. Host,
   ABI, real-GPU, C7, and C9 evidence covers empty subscription/snapshot/read,
   transition to sequence one, overwrite, paging, unsupported facts, and
   recovery without advancing the CPU cursor.

### Answers To Auditor

1. The content port uses a Moria-owned exact-length output sink. The callback
   returns only `Result<(), ContentError>` and can copy or borrow fixed-size
   sample values into pre-reserved slots; no result collection or detailed box
   crosses ownership.
2. Observation frontier is optional before sequence one through the closed
   `ObservationFrontier::Empty` variant. Startup does not invent a mandatory
   initial fact. Once sequence one is appended, the frontier remains
   `Retained` and never returns to `Empty`.
3. `BoundedVolumeFilter::All` remains a snapshot for long-lived interests and
   subscriptions: it pins all volumes live at acceptance. Explicit interest
   update or resubscription is required to include later-created volumes.

### TDD Changes Made

- Replaced capacity-bearing callback results with the permit-backed
  `BaseBrickOutput` API and reconciled ownership, lifecycle, persistence,
  resource, repository-rule, decision, and validation text.
- Added the common empty/nonempty observation-frontier contract to snapshots,
  GPU delta outcomes, subscription starts, paging legality, lifecycle, Bevy
  extraction, and Extension ABI v1.
- Added adversarial content-sink bounds and pre-first-observation host,
  shader-layout, real-GPU, and public-contract evidence obligations.

### Questions For Auditor

None.

### Notes To Auditor

No human-authority question is open. The sink deliberately keeps the complete
worst-case permit through installation instead of shrinking homogeneous
results; this makes the ownership bound structural and leaves compaction as a
later measured internal optimization. The empty frontier preserves the
existing nonadvancing GPU-delta model and does not change snapshot membership
or ordinary gap recovery.

## Auditor Turn — 2026-07-27T20:15:32Z

Mode: continue

Responding to: 2026-07-27T20:10:23Z

### Prior Findings Status

#### F1 — resolved — Public operations, bounded traversal, truthful partial results, and typed failures remain concrete

The facade still defines every material, mutation, volume, query, observation,
persistence, extension, telemetry, and shutdown operation. Collision work is
separately authorized and hit overflow remains terminal rather than silently
truncated (`docs/tdd/public-api.md:872-1039,1450-1872`;
`docs/tdd/collision-and-presentation.md:18-103`).

#### F2 — partially_resolved — Brick-result transfer is structurally bounded, but the content port still has two variable owned return paths outside the permit

The new `BaseBrickOutput` resolves the defect demonstrated in the prior turn.
Moria constructs one exact fixed slot per requested brick before invocation;
homogeneous writes copy one sample, detailed writes borrow exactly 512 samples,
overlength/incomplete/ignored-error writes poison the batch, and no brick-result
collection crosses ownership (`docs/tdd/public-api.md:1194-1209,1243-1277,
1285-1291`). The concurrent sink high-water fixture verifies that selected
mechanism (`docs/tdd/validation.md:48-58`).

The same public content port nevertheless still transfers unbounded owned
values in two other ways:

- `BaseContentSource::descriptor()` returns a `SourceDescriptor` by value, and
  that descriptor contains consumer-constructed
  `ContentLineage.opaque: Box<[u8]>` (`docs/tdd/public-api.md:1161-1182`).
  The TDD checks/canonicalizes the returned descriptor only after ownership has
  crossed, and repeats that return immediately before every callback
  (`docs/tdd/public-api.md:1278-1283`). A misbehaving source can therefore
  return a many-megabyte exact box for a one-brick request even though only the
  fixed 256-byte callback allowance was reserved.
- `load_bricks` returns `Result<(), ContentError>`, where
  `ContentError.diagnostic` is an unconstrained owned `String`
  (`docs/tdd/public-api.md:1163-1168,1223-1234`). A valid source-error path may
  therefore transfer arbitrary owned capacity before Moria turns it into the
  bounded 512-byte `RegionFailure` diagnostic. This is not consumer-internal
  temporary allocation once the `ContentError` has been returned.

These paths contradict the statements that consumer code can only copy or
borrow fixed-size values and that result ownership cannot cross the port
(`docs/tdd/architecture.md:54-62`;
`docs/tdd/state-and-storage.md:253-255`). They also leave the claimed
process-visible Moria-owned high-water assertion incomplete.

Make both ownership crossings bounded by construction. For example,
`descriptor()` can borrow an already registration-validated descriptor or
return a fixed inline lineage representation, and callback failures can use a
closed/fixed-capacity diagnostic record or write into a Moria-owned diagnostic
sink. Post-return length validation or truncation alone does not solve the
ownership interval. Extend the adversarial content-port test with oversized
descriptor and error-diagnostic attempts and prove all ownership received from
concurrent source calls remains within the charged allowance.

#### F3 — resolved — Persistence read, restore, and whole-world membership remain implementable

The reader uses caller-owned bounded destinations; manifests and chunks have
preallocation size checks; restore retains exact live membership, tombstones,
lineage, and fingerprint validation (`docs/tdd/public-api.md:2205-2346`;
`docs/tdd/persistence.md:1-225`).

#### F4 — resolved — Extension effect fan-out remains fully pre-reserved

Worst-case child records, payload bytes, and completion slots are reserved
before dispatch; invalid output admits no child and a smaller valid output
releases unused capacity (`docs/tdd/public-api.md:608-638,2631-2678`).

#### F5 — resolved — Collision/query ownership remains acyclic

The private collision fact kernel remains below the public query orchestrator
in module ownership, dependency policy, and intended repository rules
(`docs/tdd/architecture.md:75-100,225-247`;
`docs/tdd/overview.md:221-229`).

#### F6 — resolved — Presentation viability covers valid local and legal-command fan-out

P6a retains the eight-corner-cell/27-artifact fixture, while P6b exercises the
512-brick/13,824-artifact bounded fair drain with explicit pool and latency
gates (`docs/tdd/validation.md:361-373`).

#### F7 — resolved — Material ID capacity remains consistent

The runtime sample, registry, persistence count, and boundary evidence all
permit 65,535 nonempty IDs plus reserved empty ID zero
(`docs/tdd/state-and-storage.md:29-55`;
`docs/tdd/persistence.md:228-247`).

#### F8 — resolved — Cancellation retains one preparation boundary

Every cancellable family races cancellation against entry to `Preparing`;
later requests return `TooLate`, and shutdown uses the same boundary
(`docs/tdd/public-api.md:822-837`;
`docs/tdd/lifecycles.md:87-121,321-339`).

#### F9 — resolved — Snapshot membership, historical filtering, and retired gap members remain representable

Accepted long-lived filters pin membership, retained facts carry charged
append-time geometry, and gap/state snapshots return one typed live or retired
record per pinned member (`docs/tdd/public-api.md:1125-1153,1827-1858,
2069-2128`; `docs/tdd/lifecycles.md:168-216`).

#### F10 — resolved — Retained volume names remain bounded and canonical

Both creation paths validate 1..=96 UTF-8 bytes and retain exact boxed names;
tombstones and persistence preserve that bound
(`docs/tdd/public-api.md:169-183,1382-1404`;
`docs/tdd/persistence.md:93-106,239-247`).

#### F11 — resolved — Empty and retained observation frontiers are now complete across host and GPU contracts

`ObservationFrontier::Empty | Retained { oldest, head }` represents the
pre-sequence-1 state without synthesizing a fact
(`docs/tdd/public-api.md:40-63`). Snapshots and GPU delta outcomes share that
type; `after = None` over an empty frontier returns zero-record `Complete` with
a `None` cursor, then observes sequence one after the first append
(`docs/tdd/public-api.md:1726-1735,1848-1858,2413-2433,2485-2522`). ABI v1
encodes only the empty pair as zero/zero, rejects half-present pairs, and keeps
status distinct from overwrite (`docs/tdd/public-api.md:2577-2587`). Host,
layout, real-GPU, C7, and C9 evidence explicitly cover the transition
(`docs/tdd/validation.md:96-103,146-150,194-205,264-274,299-314`).

### New Findings

No new finding family is added. The remaining defect is the incomplete closure
of F2: the permit-backed brick sink is correct, but all owned values returned
through the same content source boundary are not yet structurally bounded.

### Questions For Coder

1. Will `descriptor()` become a borrow/fixed representation, or will another
   type-level mechanism prevent an invalid source from returning an oversized
   owned lineage before validation?
2. Will source failure diagnostics use fixed inline storage or a
   Moria-provided bounded sink so `ContentError` cannot transfer an unreserved
   `String`?

### Answers To Coder

The coder asked no direct question. I acknowledge and verified both stated
resolutions: `BaseBrickOutput` structurally bounds brick-result transfer, and
`ObservationFrontier` fully distinguishes the valid pre-first-fact state from
gap, paging, and unsupported-fact outcomes.

### Notes To Coder

No protected human question is open. Consumer-internal temporary memory remains
outside Moria accounting as previously noted; this finding concerns only owned
values after the source returns them into Moria. The descriptor and error
record are small interface changes and do not require changing content
semantics, sink sizing, or the selected nonadvancing observation model.

### Approval

Not approved.

## Coder Turn — 2026-07-27T20:19:47Z

Mode: continue

Responding to: 2026-07-27T20:15:32Z

### Responses To Findings

1. **F2 addressed.** `BaseContentSource::descriptor()` now returns an
   immutable borrow tied to the source, so even an invalid oversized lineage
   allocation remains consumer-owned during registration validation and never
   crosses the port. Moria copies only an accepted at-most-256-byte lineage
   into exact canonical world ownership. `ContentError` now carries a
   192-byte inline UTF-8 `ContentDiagnostic` instead of `String`; its private
   fixed representation rejects 193-byte construction rather than truncating.
   `repr(u8)` tags, `repr(C)` records, and a compile-time assertion make the
   complete by-value error exactly 195 bytes, inside the existing 256-byte
   callback allowance. Thus brick output, descriptor identity, and failure
   diagnostics are all structurally bounded at the ownership crossing.
2. **F11 remains resolved.** This revision does not alter the selected closed
   empty/nonempty observation frontier or nonadvancing GPU delta semantics.

### Answers To Auditor

1. `descriptor()` is now a borrow: `fn descriptor(&self) ->
   &SourceDescriptor`. The source retains ownership of its allocation; Moria
   validates through the borrow and copies only a valid bounded descriptor.
2. Source failures use fixed inline storage. `ContentError` can return only a
   closed kind, `Retryability`, and `ContentDiagnostic` containing at most 192
   UTF-8 bytes; no `String` or other variable owned diagnostic can cross.
3. `BoundedVolumeFilter::All` on a long-lived interest or subscription is a
   snapshot of the live volume IDs at acceptance. Create, retire, and movement
   do not expand or substitute membership; explicit interest update or
   resubscription takes a new bounded snapshot.

### TDD Changes Made

- Revised the normative content-source API in `public-api.md` to use borrowed
  immutable descriptors and an exact fixed-layout error/diagnostic record,
  including ownership, validation, canonical-copy, size, and permit rules.
- Reconciled the content ownership boundary in `architecture.md`,
  `overview.md`, and `state-and-storage.md`.
- Added adversarial validation for oversized borrowed descriptors, 192/193
  byte diagnostics, concurrent fixed source errors, canonical lineage copies,
  and aggregate response high-water.
- Added technical decision T26 for borrowed source identity and inline errors.

### Questions For Auditor

None.

### Notes To Auditor

Consumer-internal memory, including storage behind the borrowed descriptor,
remains outside Moria accounting and ownership. The public port can now receive
only pre-reserved sink writes and the exact 195-byte error value; every
variable source-owned allocation stays behind a borrow. No human-authority
question is open.

## Auditor Turn — 2026-07-27T20:23:35Z

Mode: approved

Responding to: 2026-07-27T20:19:47Z

### Prior Findings Status

#### F1 — resolved — The public contract remains callable, bounded, and explicit

The facade still selects concrete operations and normative request/result
types for configuration, interest, content, commands, queries, observations,
presentation, persistence, GPU extensions, telemetry, and shutdown. Collision
traversal has separate candidate authorization, and output overflow cannot be
reported as a partial hit result (`docs/tdd/public-api.md:875-1159,
1342-1926`; `docs/tdd/collision-and-presentation.md:18-103`).

#### F2 — resolved — Every content-port ownership crossing is now structurally bounded

The earlier exact output sink remains permit-backed and Moria-owned. The
remaining descriptor and error paths are now closed as claimed:
`descriptor()` returns `&SourceDescriptor`, Moria copies only accepted bounded
lineage into canonical ownership, and callback failure returns a 195-byte
fixed-layout `ContentError` containing at most 192 UTF-8 bytes
(`docs/tdd/public-api.md:1161-1260,1270-1339`). The 256-byte fixed charge covers
that complete error and batch state, and the callback cannot return a
capacity-bearing descriptor, diagnostic, or brick result
(`docs/tdd/public-api.md:489-499,1288-1331`;
`docs/tdd/architecture.md:54-63`).

The adversarial evidence now covers an oversized borrowed descriptor without
ownership transfer, exact canonical copying at 256 lineage bytes, 192/193-byte
diagnostic boundaries, concurrent failures, and aggregate sink/error
high-water under `content_response_bytes`
(`docs/tdd/validation.md:48-68`). This resolves the final concrete defect.

#### F3 — resolved — Persistence read, restore, and whole-world identity remain implementable

Caller-owned bounded read buffers, size discovery/revalidation, exact
live-volume membership, tombstones, lineage, fingerprint checks, and semantic
restore evidence remain explicit (`docs/tdd/public-api.md:2244-2387`;
`docs/tdd/persistence.md:1-254`).

#### F4 — resolved — GPU extension fan-out remains pre-reserved and nonprivileged

The effect-batch permit reserves worst-case ordinary command records, bytes,
and child completions before dispatch. Whole output validation admits every
child or none, after which children use ordinary independent receipts
(`docs/tdd/public-api.md:608-638,2389-2734`).

#### F5 — resolved — Dependency direction remains acyclic

The storage-level collision kernel produces private facts for the public query
orchestrator and does not import query policy, codecs, or receipts
(`docs/tdd/architecture.md:77-104,227-270`;
`docs/tdd/overview.md:221-229`).

#### F6 — resolved — Performance viability remains falsifiable at local and maximum legal scale

P6a uses the physically valid eight-corner/27-artifact fixture, while P6b
requires bounded fair draining of all 13,824 artifacts from a legal
512-brick command. Both are blocking on every claimed backend family
(`docs/tdd/validation.md:361-373,399-423`).

#### F7 — resolved — Material capacity remains internally consistent

Empty ID zero is additional to the permitted 65,535 nonempty registrations in
runtime layout, configuration, persistence, and boundary tests
(`docs/tdd/state-and-storage.md:29-55`;
`docs/tdd/persistence.md:228-247`).

#### F8 — resolved — Cancellation has one testable point of no return

All cancellable operation families race cancellation against the transition
from queued/waiting to `Preparing`; preparation and every later stage are too
late, including during shutdown (`docs/tdd/public-api.md:822-837`;
`docs/tdd/lifecycles.md:87-121,321-339`).

#### F9 — resolved — Long-lived membership and historical observation are explicit

`All` snapshots the live IDs at acceptance for both interest and subscription;
interest also freezes its brick set, retained facts carry append-time filter
geometry, and gap snapshots preserve one live or retired record per pinned
member (`docs/tdd/public-api.md:1125-1153,1827-1858,2069-2128`;
`docs/tdd/lifecycles.md:55-67,168-216`). The coder's clarification that only
update or resubscription refreshes membership matches the normative TDD.

#### F10 — resolved — Retained volume names remain canonical and bounded

Both creation paths enforce 1..=96 UTF-8 bytes and retain only exact boxed
directory/tombstone ownership; persistence encodes the canonical length
(`docs/tdd/public-api.md:1382-1404`;
`docs/tdd/persistence.md:93-106,239-247`).

#### F11 — resolved — Empty observation history is representable throughout the contract

The closed `Empty | Retained` frontier is shared by host snapshots, GPU delta
outcomes, paging/gap rules, and ABI v1. The pre-first-fact and sequence-one
transition has host, layout, real-GPU, and public-contract evidence
(`docs/tdd/public-api.md:40-63,1848-1858,2485-2522,2577-2587`;
`docs/tdd/validation.md:96-103,146-150,194-205,264-274,299-314`).

### New Findings

None. The complete TDD selects concrete ownership, publication, lifecycle,
failure, bounded-resource, persistence, portability, and validation mechanisms
without adding product behavior or a privileged consumer path.

### Questions For Coder

None.

### Answers To Coder

The coder asked no direct question. I verified the claimed borrowed descriptor,
fixed inline error record, canonical copy, and adversarial evidence against the
actual TDD. I also confirm the stated snapshot semantics for long-lived `All`
filters are consistent across the public API, lifecycle, decisions, and tests.

### Notes To Coder

No protected human question is open. T26 records the final ownership choice
without changing the approved content semantics. The TDD is implementation-
ready subject to its own blocking physical-adapter, performance, and human
visual evidence gates.

### Approval

Approved.

## Coder Turn — human-review revision

Mode: continue

Responding to: TamedTornado (COMMENTED)

### Responses To Human Review

1. **Scheduled hook added.** A consumer-triggered substrate tick now captures
   an ordinary-command frontier, drains it, pins one committed
   material/placement view, runs registered adapters, validates and composes
   their proposals, publishes at most one new revision per affected volume,
   and only then releases post-frontier work.
2. **CPU and GPU are first-class.** CPU adapters receive a borrowed stable tick
   view and exact-capacity effect sink directly from the coordinator, without
   submitting or polling a query receipt. GPU adapters own their pipelines and
   state on Bevy's renderer device, encode against a read-only exported view
   through a Moria-controlled encoder, and reach validation/composition/
   publication without material or candidate-effect CPU readback.
3. **Bounds and authority remain Moria-owned.** Builder descriptors cap access,
   view records/bytes, proposals, payloads, external GPU-resource reporting,
   and ordering edges. Tick admission pre-reserves proposal, transaction,
   completion, observation, and feedback resources. Adapters receive neither
   authoritative storage nor queue submission.
4. **Ordering and conflict are explicit.** `runs_after` produces a
   builder-validated DAG with stable-key tie order. Every participant sees the
   same view. Overlap selects a whole-proposal `RejectLater`,
   `ReplaceEarlier`, or `FailTick` outcome; no named physics/damage phase or
   partial proposal exists.
5. **State ownership is complete.** CPU/GPU behavior state is excluded from
   checkpoints and Moria recovery. Reports/GPU feedback describe rejected
   effects without rollback. Device loss quarantines the tick and invalidates
   adapter GPU resources; the adapter recreates its state and reports ready.
   Shutdown drains an already-running tick and calls adapter lifecycle hooks
   without saving or discarding consumer state.
6. **Asynchronous APIs retained and reframed.** Ordinary queries remain
   unchanged. The copied-packet WGSL inspection/effect job remains available
   for tools and nonscheduled consumers but is explicitly not the general
   behavior-engine hook.

### Concrete TDD Evidence

- `behavior-scheduling.md` owns the tick state machine, stable-view format,
  CPU/GPU execution, synchronization, admission, composition, failure,
  persistence, device loss, recovery, shutdown, and adversarial cases.
- `public-api.md` defines builder registrations, CPU/GPU adapter traits,
  behavior access/envelope types, tick permits/receipts, participant/proposal
  outcomes, resource limits, and the renderer-coupled GPU context boundary.
- `architecture.md` places behavior export/run/validate/compose/publish in the
  render schedule and keeps the adapter dependency below Bevy wiring but above
  storage/collision/command services.
- `lifecycles.md` makes post-frontier command exclusion, tick transitions,
  failure policy, publication, and shutdown behavior explicit.
- `persistence.md` excludes adapter state and specifies consumer coordination
  and restore readiness.
- `state-and-storage.md` names the view/proposal/feedback bounds and device-loss
  pins.
- `validation.md` adds host/state-machine/real-GPU evidence, contract scenario
  C9 for conventional CPU physics-shaped, GPU-resident physics-shaped, and
  CPU/GPU damage-and-bond-shaped adapters, retains the WGSL facility as C10,
  and adds blocking scheduled-GPU feasibility gate P7.
- `decisions.md` preserves every prior technical entry, adds T27, and records
  the complete supplied human feedback separately from the resulting
  technical interpretation.

### Questions For Reviewer

None.

### Notes To Reviewer

The proof adapters are adversarial harness integrations only. No physics,
damage, health, resistance, bond, fracture, gravity, force, player, or gameplay
field was added to Moria's data model.

## Auditor Turn — 2026-07-27T21:37:00Z

Mode: continue

Responding to: none

### Prior Findings Status

#### F1–F11 — resolved — The previously approved contract remains closed

The post-approval revision does not regress the earlier fixes for callable
command/query/collision APIs, bounded content ownership, persistence identity,
extension fan-out, dependency direction, presentation fan-out, material-ID
capacity, cancellation, pinned observation membership, bounded names, or the
empty observation frontier. Those contracts remain present in their previously
approved forms. The findings below concern the newly added scheduled behavior
surface and do not reopen those resolutions.

### New Findings

#### F12 — unresolved — The GPU device context directly defeats the promised controlled-encoder boundary

The TDD says `BehaviorGpuDeviceContext` contains Bevy's `RenderDevice`, while
also saying an adapter cannot obtain a raw command encoder and that C9 must
fail if it can (`docs/tdd/behavior-scheduling.md:203-225`;
`docs/tdd/public-api.md:2826-2905`; `docs/tdd/validation.md:359-362`).
In the selected Bevy 0.19 API, `RenderDevice` is `Clone`, exposes
`create_command_encoder`, and exposes the underlying device through
`wgpu_device`; this is the documented public surface:
https://docs.rs/bevy/0.19.0/bevy/render/renderer/struct.RenderDevice.html.
A conforming adapter handed that value can therefore obtain exactly the raw
encoder the TDD and intended `AGENTS.md` prohibit. It can also retain the
device and allocate resources after the one `BehaviorResourceReport`, making
the claimed per-tick check of `maximum_owned_gpu_bytes` unenforceable from the
specified API (`docs/tdd/behavior-scheduling.md:312-318`;
`docs/tdd/public-api.md:2519,2647-2655,2847-2855`).

Required correction: expose a deliberately restricted device/resource factory
that supplies the Moria group-0 layout and the needed buffer/pipeline/bind-group
creation operations without exposing `RenderDevice`, `wgpu::Device`, or raw
encoder creation. Define resource registration and whether resource reporting
is enforced or trusted telemetry. Then make C9 compile an external-style
adapter that attempts every forbidden acquisition through the actual public
surface.

#### F13 — unresolved — One aggregate GPU view widens every adapter's authorized access

Access is declared and planned per adapter, and the contract says an adapter
may inspect only its authorized stable view
(`docs/tdd/behavior-scheduling.md:80-126`;
`docs/tdd/public-api.md:2529-2549`). The implementation selection instead
exports one aggregate `S`, supplies the packed view directly to GPU shaders,
and explicitly has multiple GPU adapters share it
(`docs/tdd/behavior-scheduling.md:211-225,242-257`;
`docs/tdd/validation.md:343-362,461`). A GPU shader can read every record in
that binding; unlike the CPU helper, no per-adapter method can reject reads
outside that adapter's planned scopes. Adapter A can therefore inspect records
admitted only for adapter B without debiting A's access bounds, so C9's
access-bound adversary succeeds.

Required correction: distinguish “same committed revision set” from “same
visible record set.” Export a per-participant filtered view, or explicitly
authorize and charge the full aggregate union against every participant before
running it. Add a two-adapter test with disjoint scopes proving neither CPU nor
GPU adapter can inspect the other's records while both still refer to the same
pinned revisions.

#### F14 — unresolved — The scheduled GPU view lacks the metric data needed to consume collision truth

`BehaviorVolumeRecordV1` carries identity, revision, translation, and rotation,
but omits `cell_size` and the finite local domain
(`docs/tdd/behavior-scheduling.md:133-152`;
`docs/tdd/public-api.md:2702-2719`). Authoritative occupied geometry is a cell
box scaled by each volume's `cell_size`
(`docs/tdd/state-and-storage.md:19-22`). A GPU physics-shaped adapter cannot
turn exported integer coordinates into the world-space occupied boxes it is
required to consume. It cannot safely preload this information at registration
because an `All` planner may include a volume created later
(`docs/tdd/behavior-scheduling.md:114-121`). The CPU helper can retain hidden
metadata internally; the closed GPU ABI cannot.

Required correction: add all volume metadata required to interpret the exported
material/collision field—at minimum finite positive cell size, and domain
bounds if boundary semantics are part of the exported collision view—to the
versioned host/WGSL record with exact layout and validation. Exercise unequal
cell sizes and a post-registration created volume in the GPU adapter proof.

#### F15 — unresolved — Mixed CPU/GPU ordering has no implementable transfer contract

The architecture promises that a processor edge performs a bounded map or
upload (`docs/tdd/architecture.md:212-219`), and C9 requires adapters to share
consumer-owned impact stimuli through CPU memory and GPU buffers
(`docs/tdd/validation.md:350-356`). The callable API provides only
`predecessor_complete(engine, processor)`, GPU dispatch, and GPU-to-GPU buffer
copy (`docs/tdd/public-api.md:2662-2674,2854-2873`). It defines no transition
buffer registration, CPU upload sink, post-submit map/readback callback, byte
reservation, mapping lifetime, or transition error. Calling the hook after
queue completion does not make adapter-owned GPU bytes available to a CPU
successor, and a CPU predecessor has no portable way to upload newly produced
bytes to a GPU successor.

Required correction: select a concrete bounded cross-processor handoff. Define
who registers/owns staging and device buffers, who reserves bytes/maps, when
copy/map/unmap/upload occurs, what the successor receives, how cancellation and
device loss terminate it, and how errors apply the participant failure policy.
The payload can remain opaque to Moria. Add both GPU-to-CPU and CPU-to-GPU C9
cases; if mixed-processor state transfer is intentionally unsupported, remove
the contrary scheduling and validation claims.

#### F16 — unresolved — GPU reconciliation and tick-wide failure outcomes are not representable

The TDD promises that a GPU adapter consumes feedback on a later tick, but the
only specified tick context exposes a write-only current effect/feedback target
and no prior-feedback view or adapter callback
(`docs/tdd/behavior-scheduling.md:303-310,415-418`;
`docs/tdd/public-api.md:2830-2845,2892-2920`). Buffer lifetime, previous record
count, and the point at which a later dispatch may read prior feedback are not
defined. Thus C9 cannot prove GPU reconciliation after the adapter has changed
its own state.

The Rust outcome model is also incomplete for the selected policies.
`FailTick` and an `AbortTick` participant discard otherwise valid proposals,
but `BehaviorParticipantOutcome` has only `Completed` or participant-local
`Skipped`, and `BehaviorProposalRejection` has no tick-aborted or fail-tick
reason (`docs/tdd/public-api.md:2560-2614`;
`docs/tdd/behavior-scheduling.md:360-405`). A completed adapter whose proposal
is discarded because another adapter aborts cannot receive an honest typed
report. In addition, `on_tick_report` runs after publication, yet a panic in
that report hook is generically recorded as `Panicked` while the stated
`AbortTick` guarantee says an abort publishes no effect
(`docs/tdd/behavior-scheduling.md:191-194,395-405`).

Required correction: define a retained/read-only prior-feedback input (or an
ordered copy into adapter-owned state) with bounded lifetime and
device-generation rules. Add closed participant/proposal/tick outcomes for
composition failure, abort caused by another participant, and post-publication
notification failure. State whether the tick receipt is applied or failed,
whether `revision_changed` is true, and what every CPU/GPU adapter observes.
Post-publication hook failure must not be retroactively described as a
no-publication abort.

#### F17 — unresolved — CPU collision helper output bypasses the tick's reservation model

`CpuBehaviorView::collision` returns an owned
`BehaviorCollisionFacts { contacts: Vec<ContactFact> }`
(`docs/tdd/public-api.md:2721-2764`). The tick reservation lists view,
proposal, transaction, completion, observation, and fixed outcome resources,
but no collision-result records/bytes or call bound
(`docs/tdd/behavior-scheduling.md:331-344`). `max_contacts` bounds one call,
but a callback can make repeated calls and retain every returned vector; Moria
allocates and transfers those capacity-bearing results despite the adjacent
claim that callback crossings are structurally bounded
(`docs/tdd/behavior-scheduling.md:128-131`;
`docs/tdd/validation.md:44-50`).

Required correction: use a caller/Moria-provided exact-capacity collision sink
or reserve an aggregate per-tick result scratch budget and define reuse so no
owned `Vec` crosses the callback. Add repeated-call, ignored-error, maximum-hit,
and result-overflow tests with allocation/high-water evidence.

#### F18 — unresolved — The new scheduled ABI lacks its required negative and layout evidence

Scheduled ABI v1 introduces two headers, packed volume/cell records, 128-byte
proposal records, payload runs, and 32-byte feedback records, with mandatory
Rust/WGSL layout assertions (`docs/tdd/behavior-scheduling.md:242-310`).
The shader-validation matrix still names exact offsets and malformed fields
only for Extension ABI v1; it does not require scheduled-header/record parity,
bad magic/version/reserved fields, undersized effective binding ranges,
count/offset/payload overflow, invalid snapshot indices, modified revisions, or
feedback layout rejection (`docs/tdd/validation.md:157-176`). The general
“fixed proposal validation” phrase in the real-GPU suite is not a concrete
layered evidence contract.

Required correction: add host/WGSL size/offset/stride assertions and explicit
negative fixtures for every scheduled view/effect/feedback field and failure
layer, then run positive exact-integer readback parity on every claimed backend
family. Include access-isolation and old-generation feedback quarantine.

#### F19 — unresolved — P7's fixed workload is not a legal default packed view

P7 says gates run at defaults and requires a 16 MiB stable view
(`docs/tdd/validation.md:443-461`). At defaults the aggregate view permits
262,144 cells and 256 volumes
(`docs/tdd/public-api.md:467-471`). The specified packed records are 24 bytes
per cell and 64 bytes per volume, plus a 64-byte header
(`docs/tdd/public-api.md:2702-2719`;
`docs/tdd/behavior-scheduling.md:261-265`), for at most 6,307,904 logical
bytes, not 16 MiB. If “16 MiB” means allocation capacity rather than logical
records, the gate does not state the copied/initialized workload and cannot be
reproduced.

Required correction: state exact legal volume/cell counts and exact packed
bytes for P7, or change the default limits/workload consistently. Distinguish
allocated capacity from bytes exported and timed.

### Questions For Coder

1. Will the GPU initialization context replace `RenderDevice` with a restricted
   resource factory, or is the no-raw-encoder/C9 claim being intentionally
   relaxed?
2. Is each participant meant to see only its planned scopes, or is the complete
   aggregate union intentionally authorized and charged to every adapter?
3. What concrete public operation maps GPU-owned stimuli for a CPU successor
   and uploads CPU-owned stimuli for a GPU successor?
4. Through what read-only handle or callback does a GPU adapter consume the
   previous tick's feedback before that storage is overwritten or released?
5. What typed outcome does a valid proposal receive when another participant's
   `AbortTick` policy or a `FailTick` conflict discards it?

### Answers To Coder

The coder asked no direct question. I acknowledge that the revision preserves
the requested authority boundary at the conceptual level: external engines own
their vocabulary/state, Moria owns publication, and CPU/GPU adapters are both
named first-class cases. The findings above are about places where the selected
callable types and evidence do not yet realize those stated semantics.

### Notes To Coder

No protected human question is open. These are ordinary technical-design
responsibilities created by the selected scheduled-adapter architecture. The
restricted-device, per-adapter-view, mixed-transition, and feedback choices may
change implementation shape, but none requires adding physics/damage semantics
or weakening the approved behavior seam.

### Approval

Not approved.

## Coder Turn — 2026-07-27T21:55:15Z

Mode: continue

Responding to: 2026-07-27T21:37:00Z

### Responses To Findings

1. **F12 addressed.** GPU device initialization now exposes only a restricted
   resource factory and generation-bound opaque handles. It exposes no
   `RenderDevice`, raw wgpu resource, queue, or encoder constructor. Buffer
   bytes, resource counts, WGSL bytes, generation, and last use are enforced by
   Moria's registry; `BehaviorResourceReport` is computed telemetry. C9 adds
   external-crate compile-fail attempts against every forbidden acquisition.
2. **F13 addressed.** One commit frontier `S` produces a separately filtered
   `S_i` for each participant. CPU iterators and GPU binding 0 contain only
   that participant's records; config charges the sum of isolated exports.
   Disjoint-scope host/GPU tests prove isolation at the same pinned revisions.
3. **F14 addressed.** `BehaviorVolumeRecordV1` is now an exact 112-byte
   host/WGSL record with finite positive cell size and half-open local domain.
   Tests cover unequal metrics and a volume created after registration.
4. **F15 addressed.** Declared ordering edges may carry one opaque bounded
   handoff. The tick pre-reserves Moria-owned host/device/staging bytes and map
   slots. CPU-to-CPU copy, CPU-to-GPU upload, GPU-to-GPU copy, and GPU-to-CPU
   completion/map/decode/unmap have explicit ownership, visibility, failure,
   cancellation, device-loss, and release rules. C9 covers both mixed
   directions.
5. **F16 addressed.** GPU feedback uses two Moria-owned slots per participant;
   the prior terminal slot is read-only binding 4 on the next tick and is
   pinned through its consuming submission. First-use and generation-loss
   absence are typed. `BehaviorTickCompleted` now represents published,
   no-publication, and post-publication-notification-failure outcomes with
   `revision_changed`, participant states, and `TickAborted` proposal reasons.
6. **F17 addressed.** CPU collision fills one reusable exact-capacity sink and
   returns a borrow tied to it. Repeated calls debit aggregate call/traversal
   budgets without transferring a `Vec`; ignored-error, maximum/overflow, and
   high-water tests prove nonpartial bounded results.
7. **F18 addressed.** Scheduled ABI v1 fixes offsets/strides for all four
   64-byte headers, volume/cell/proposal/run/handoff/feedback records.
   Validation now requires Rust/WGSL parity, field-specific negative fixtures
   at the expected layer, exact-integer real-GPU parity on every claimed
   backend, access isolation, and old-generation feedback quarantine.
8. **F19 addressed.** P7 now uses two legal disjoint default-limit exports.
   Each has 128 volumes and 256 full bricks/131,072 cells, or
   `64 + 128×112 + 131,072×24 = 3,160,128` initialized bytes. Timed aggregate
   logical bytes are exactly 6,320,256; the 32 MiB allocation capacity is
   reported separately.

### Answers To Auditor

1. The no-raw-device/encoder claim is retained through the restricted factory.
2. Every participant sees only its own planned scopes at the shared frontier.
3. Mixed transfer uses the declared Moria-owned opaque handoff upload/map port.
4. Previous GPU feedback is read through pinned group-0 binding 4 and
   `BehaviorPriorFeedback`.
5. A valid proposal discarded by `AbortTick` or `FailTick` receives
   `TickAborted`; its participant is `DiscardedByTick`, the tick is
   `NoPublication`, and `revision_changed` is false.

### TDD Changes Made

- Reworked `behavior-scheduling.md` and `public-api.md` with the selected
  scheduling, resource, ABI, ownership, handoff, feedback, collision, and
  outcome types.
- Reconciled configuration, render scheduling, multi-update progress, device
  loss, persistence exclusion, intended `AGENTS.md`, and resource summaries
  across the remaining TDD.
- Added T28 plus host, state-machine, shader, real-GPU, C9, and corrected P7
  evidence.

### Questions For Auditor

None.

### Notes To Auditor

No human-authority question is open. Backend-private pipeline allocation size
is bounded by pipeline count rather than invented byte telemetry. Opaque
handoff semantics and durable adapter state remain consumer-owned; no behavior
vocabulary moved into Moria.

## Auditor Turn — 2026-07-27T22:07:01Z

Mode: continue

Responding to: 2026-07-27T21:55:15Z

### Prior Findings Status

#### F12 — resolved — The scheduled GPU surface no longer exposes a raw renderer device or trusted byte report

`BehaviorGpuDeviceContext` now contains only a restricted factory and device
generation, while the callable encoder accepts only opaque factory-created
handles (`docs/tdd/public-api.md:3019-3150,3183-3204`).
The TDD explicitly excludes `RenderDevice`, `wgpu::Device`, queues, raw
resources, and encoder construction, and makes `BehaviorResourceReport`
registry-computed rather than adapter-supplied
(`docs/tdd/behavior-scheduling.md:223-244`).
The external compile-fail/adversarial requirements now attempt every forbidden
acquisition (`docs/tdd/validation.md:44-50,419-424`).
The distinct aggregate byte-budget defect is recorded as F20 below rather than
leaving this authority finding open.

#### F13 — resolved — Participants receive isolated exports from one pinned frontier

The state machine now defines union `S` only as the coordinator's pin set and
gives each participant a filtered `S_i`
(`docs/tdd/behavior-scheduling.md:52-62`).
The public and GPU contracts deny addressing records outside that participant's
export (`docs/tdd/behavior-scheduling.md:164-172,246-266`;
`docs/tdd/public-api.md:3197-3204`).
The aggregate count/byte rules charge the sum of isolated views, and validation
uses disjoint CPU/GPU scopes at the same pinned revisions
(`docs/tdd/public-api.md:480-482`; `docs/tdd/validation.md:51-55,262-275`).

#### F14 — resolved — Scheduled volume records carry the metric and domain needed to interpret collision truth

The exact 112-byte record now includes finite positive `cell_size` and
half-open local-domain bounds
(`docs/tdd/behavior-scheduling.md:139-170,311-318`;
`docs/tdd/public-api.md:2825-2846,2968-2973`).
Host and real-GPU evidence includes unequal metrics and a volume created after
registration (`docs/tdd/validation.md:51-55,262-275`).

#### F15 — resolved — Mixed CPU/GPU order edges have an owned, bounded transfer protocol

Each declared edge may carry one bounded opaque payload; startup reserves the
host/device/staging representations and mixed-edge map slots
(`docs/tdd/public-api.md:2568-2575,2766-2769,2948-2966`).
CPU-to-CPU, CPU-to-GPU, GPU-to-GPU, and GPU-to-CPU milestones, borrows,
validation, map/unmap, cancellation, loss, and failure attribution are selected
concretely (`docs/tdd/behavior-scheduling.md:418-443`).
C9 exercises both mixed directions and forbids smuggling raw harness buffers
through the seam (`docs/tdd/validation.md:403-410`).

#### F16 — partially_resolved — Feedback lifetime and CPU outcomes are concrete, but the GPU wire record cannot represent the promised terminal outcome

The double-buffered feedback lifetime, next-tick binding, first-use state, and
old-generation quarantine are now explicit
(`docs/tdd/behavior-scheduling.md:370-384`;
`docs/tdd/public-api.md:3125-3129,3222-3231`).
The Rust outcome model also closes publication, tick-abort, proposal discard,
and post-publication notification cases
(`docs/tdd/public-api.md:2606-2691,2748-2760`).

The fixed GPU record does not carry that same closed information.
`BehaviorTickAbortCause::ConflictFailTick` needs two engine/proposal pairs,
`TransitionFailure` needs predecessor, successor, and stage, and
`PublishedWithNotificationFailure` needs a terminal disposition and failed-hook
count (`docs/tdd/public-api.md:2613-2635`).
The scheduled feedback ABI provides only `status`, `failure`, undefined
`flags`, `revision_changed`, and one `cause_engine_or_zero` /
`cause_proposal_or_zero` pair, with no disposition or transition-stage field
(`docs/tdd/behavior-scheduling.md:386-407`).
Nevertheless the TDD says the slot is finalized after notification failure and
that GPU adapters reconcile from this feedback
(`docs/tdd/behavior-scheduling.md:377-384,540-563`).
An implementation agent cannot encode a third participant's discarded
proposal after a two-party `ConflictFailTick`, distinguish publication from
`PublishedWithNotificationFailure`, or reconstruct a transition failure from
the specified fields.

Required correction: either extend the versioned header/records so every
promised GPU-visible terminal disposition and closed cause has an exact field
mapping, or explicitly select a smaller GPU feedback contract and define the
lossless information it does expose. Define every `flags` bit and add golden
host/WGSL cases for participant abort, conflict fail-tick, transition failure,
preparation failure, device loss, and post-publication notification failure.

#### F17 — resolved — CPU behavior collision output is structurally reused and bounded

`CpuBehaviorView::collision` fills a Moria-owned exact-capacity sink and returns
only a borrow valid until reuse (`docs/tdd/public-api.md:2864-2892,2975-2986`).
The aggregate call counter, traversal debit, poison behavior, nonpartial
overflow, and 320 KiB contact pool are explicit
(`docs/tdd/public-api.md:483`; `docs/tdd/behavior-scheduling.md:180-191`).
Validation covers repeated calls, ignored errors, maximum/overflow, and
allocation high-water (`docs/tdd/validation.md:71-74`).

#### F18 — partially_resolved — Scheduled ABI fixtures are named, but the normative WGSL field types are not legal WGSL

The revision adds the requested size/offset/stride matrix and field-specific
negative fixtures (`docs/tdd/validation.md:208-219`) and requires exact integer
readback on the claimed backends (`docs/tdd/validation.md:262-275`).
However, the normative Scheduled ABI declares `tick:u64`,
`generation:u64`, `volume:u64`, `revision:u64`, command IDs, and proposal
revisions as `u64` fields while requiring direct Rust/WGSL layout parity
(`docs/tdd/behavior-scheduling.md:300-318,330-348,386-408`).
WGSL has no concrete 64-bit integer type; its concrete integer scalar types are
`i32` and `u32`, as the current WGSL specification states:
https://gpuweb.github.io/gpuweb/wgsl/#scalar-types.
The older Extension ABI already demonstrates the portable representation by
naming low/high `u32` words for device generation and operation ID
(`docs/tdd/public-api.md:3462-3474`), but Scheduled ABI v1 never selects such a
wire representation or comparison rule.

Required correction: define every logical 64-bit scheduled field as an exact
little-endian low/high `u32` pair (or another legal closed wire type), preserve
or update all offsets and sizes consistently, specify host pack/unpack and
two-word equality/zero rules, and make the scheduled Rust/WGSL golden and
negative fixtures assert those actual declarations.

#### F19 — resolved — P7 now states a legal default packed workload

P7 uses two isolated exports whose summed volume/cell counts exactly consume
the default aggregate maxima and whose arithmetic is correct:
`64 + 128×112 + 131,072×24 = 3,160,128` bytes per participant and
`6,320,256` initialized/copied bytes in aggregate
(`docs/tdd/validation.md:523`; `docs/tdd/public-api.md:480-482`).
It separately reports the 32 MiB allocation capacity.

### New Findings

#### F20 — unresolved — Factory-owned GPU buffer bytes have no aggregate world budget

The restricted factory makes adapter buffers Moria-created, Moria-registered
device allocations and enforces each descriptor's
`maximum_owned_gpu_bytes` (`docs/tdd/behavior-scheduling.md:223-244`;
`docs/tdd/public-api.md:3032-3099,3190-3195`).
But `ResourceLimits` has aggregate counts for behavior buffers, pipelines, and
bind groups plus aggregate WGSL bytes, and no aggregate byte field for the
buffers themselves (`docs/tdd/public-api.md:400-435,478-490`).
The storage summary repeats counts/WGSL only
(`docs/tdd/state-and-storage.md:262-269`) while claiming telemetry exposes
current, high-water, and limit for every behavior opaque-resource pool
(`docs/tdd/state-and-storage.md:311-318`).

Consequently, 16 default adapters may each declare an arbitrarily large `u64`
maximum, pass every stated aggregate cross-limit, and allocate until the
renderer reports OOM. Per-adapter registries prevent one adapter from exceeding
its own declaration, but they do not provide a world memory budget,
backpressure rule, startup feasibility check, or aggregate byte telemetry.
P7's two 32 MiB states happen to be finite but are not checked against any
configured aggregate factory-state byte pool
(`docs/tdd/validation.md:523`).

Required correction: add a configured, adapter-negotiated aggregate live
factory-buffer byte limit with default/minimum/hard/adapter relationships;
require the sum of descriptor maxima to fit it at builder/startup validation;
charge allocation until handle dependencies and last GPU use permit release;
surface current/high-water/limit/rejection telemetry; and test multi-adapter
admission, drop-after-last-use, recreation, and OOM-before-allocation behavior.
Backend-private pipeline memory may remain count-bounded as already justified.

### Questions For Coder

1. Is next-tick GPU feedback intended to reproduce the complete Rust terminal
   cause/disposition, or only a reduced rejection/failure category? Whichever
   is intended needs an exact record mapping.
2. Will Scheduled ABI v1 encode each logical 64-bit value as low/high `u32`
   words, matching the existing Extension ABI convention?
3. What configured aggregate byte limit is meant to cap all live
   factory-created behavior buffers across adapters?

### Answers To Coder

The coder asked no direct question.
I acknowledge all five prior answers: the restricted factory, per-participant
views, Moria-owned mixed handoffs, pinned prior-feedback binding, and typed
tick-abort outcome are present in the actual TDD.
The remaining F16 issue is not the absence of those concepts; it is that the
selected fixed GPU record cannot encode the outcome the prose and Rust types
promise.

### Notes To Coder

No protected human question is open.
F16/F18/F20 are ordinary wire-format and resource-admission decisions within
the already selected T28 architecture.
The view, metric/domain, handoff, collision-sink, and P7 corrections should be
retained.
Only `docs/tdd/review-findings.md` was modified in this auditor turn.

### Approval

Not approved.

## Coder Turn — 2026-07-27T22:18:33Z

Mode: continue

Responding to: 2026-07-27T22:07:01Z

### Responses To Findings

1. **F16 addressed.** Next-tick GPU feedback now preserves the complete Rust
   terminal disposition and every `BehaviorTickAbortCause`. The participant
   record is 64 bytes and carries execution/failure category, publication,
   notification, tick disposition, exactly defined flags, failed-hook count,
   both engine/proposal pairs for `ConflictFailTick`, both engines plus stage
   for `TransitionFailure`, and a device-generation pair. Proposal feedback is
   48 bytes and retains related engine/proposal or volume identity where its
   closed rejection variant requires it. Variable Rust failure payloads remain
   intentionally CPU-report-only; the GPU record exposes their category, but
   terminal disposition/cause is lossless. Golden host/WGSL cases cover all
   requested terminal paths.
2. **F18 addressed.** Scheduled ABI v1 now defines
   `ScheduledU64LeV1 { low: u32, high: u32 }` as its only logical 64-bit wire
   representation. Every header, volume, proposal, feedback command/revision,
   tick, generation, and device-loss field names the two legal WGSL words at
   exact offsets. Host pack/unpack, equality, zero, byte order, boundary
   values, exact absence-sentinel handling, and the absence of WGSL `u64`
   declarations are normative validation obligations. Existing record sizes
   remain unchanged except for the deliberately expanded feedback records.
3. **F20 addressed.** `ResourceLimits` now includes the adapter-negotiated
   aggregate `behavior_gpu_buffer_bytes` pool: 256 MiB desired, 64 MiB
   minimum, and `min(1 GiB, adapter max_buffer_size)` hard/effective bound.
   Builder registration checked-sums descriptor maxima against desired
   capacity; startup repeats the check against effective capacity before
   device-state creation. Each allocation atomically reserves adapter bytes,
   aggregate bytes, and a handle before renderer allocation. Charges survive
   handle drop through bind-group dependencies and last GPU use, and terminal
   generation teardown reaches zero before recreation. Logical exhaustion
   invokes no backend allocation; renderer OOM releases all permits and
   registers no handle. Public resource telemetry reports aggregate current,
   high-water, limit, and rejections. P7's two 32 MiB states are now explicitly
   checked as a 64 MiB charge against the 256 MiB default.

### Answers To Auditor

1. GPU feedback reproduces the complete Rust terminal tick
   disposition/cause. Only variable participant failure detail such as region
   vectors or diagnostic text is reduced to its closed category; this
   reduction is explicit and does not affect reconciliation of publication,
   abort, conflict, transition, device-loss, preparation, or notification
   outcomes.
2. Yes. Every Scheduled ABI v1 logical 64-bit value is a low/high `u32` pair
   matching the Extension ABI convention, with the low significance word
   first.
3. The aggregate cap is
   `ResourceLimits::behavior_gpu_buffer_bytes`, defaulting to 256 MiB desired
   and 64 MiB minimum, with a 1 GiB hard ceiling and adapter
   `max_buffer_size` clamp.

### TDD Changes Made

- Revised `behavior-scheduling.md` and `public-api.md` with the portable
  two-word integer ABI, complete 64-byte terminal participant feedback,
  complete 48-byte proposal feedback, tag/flag/zero rules, and exact outcome
  mappings.
- Added the aggregate factory-buffer byte field, resource kind, registration
  and startup cross-limits, allocation/release/device-generation lifecycle,
  typed capacity/OOM failures, and telemetry contract.
- Reconciled architecture, lifecycle, storage/resource summaries, intended
  repository rules, decision T29, scheduled correctness evidence, C9, and P7.
- Added host/WGSL golden and negative fixtures plus real-GPU requirements for
  low/high fields, all terminal outcomes, multi-adapter byte admission,
  dependency-delayed release, recreation, and no-backend-call logical OOM.

### Questions For Auditor

None.

### Notes To Auditor

No human-authority question is open. F12–F15, F17, and F19 remain unchanged.
Backend-private pipeline allocation remains count-bounded. The new aggregate
byte pool covers factory-created buffers only and does not claim unavailable
pipeline-memory telemetry or expand adapter authority.

## Auditor Turn — 2026-07-27T22:26:14Z

Mode: continue

Responding to: 2026-07-27T22:18:33Z

### Prior Findings Status

#### F12–F15, F17, and F19 — resolved — The retained corrections remain present

The restricted factory/encoder boundary, isolated participant exports, complete
volume metric/domain, bounded mixed-processor handoffs, reusable CPU collision
sink, and legal P7 packed workload remain unchanged and coherent in the current
TDD. I found no regression in those contracts.

#### F16 — resolved — The fixed feedback record now carries every terminal tick disposition and abort cause

The 64-byte participant record has explicit execution, publication,
notification, tick-disposition, flags, failed-hook count, two engine/proposal
pairs, transition stage, and device-generation words
(`docs/tdd/behavior-scheduling.md:417-437`). The following mapping assigns every
`BehaviorTickAbortCause`, defines every flag bit and unused-zero rule, and
distinguishes published, no-publication, and post-publication notification
failure (`docs/tdd/behavior-scheduling.md:439-496`). Golden cases exercise the
six requested terminal paths and exact Rust/WGSL round trips
(`docs/tdd/validation.md:238-244`). The distinct nonterminal feedback
inconsistency is recorded as F21 below.

#### F18 — resolved — Scheduled ABI v1 now selects a legal portable representation for every logical 64-bit value

`ScheduledU64LeV1` fixes low/high word order, packing, unpacking, equality, zero,
and little-endian representation (`docs/tdd/public-api.md:2836-2848,3003-3011`).
Every scheduled header, volume revision/ID, proposal revision, command ID, and
generation uses explicit `u32` words at fixed offsets
(`docs/tdd/behavior-scheduling.md:318-374,465-470`). Validation checks boundary
values, actual declarations, offsets, and the absence of Scheduled ABI WGSL
`u64` scalars (`docs/tdd/validation.md:219-237`).

#### F20 — resolved — Factory buffer memory now has aggregate admission, lifetime, pressure, and evidence contracts

`ResourceLimits::behavior_gpu_buffer_bytes` and
`ResourceKind::BehaviorGpuBufferBytes` provide the configured and reported pool
(`docs/tdd/public-api.md:424,490,2212-2214`). Registration and startup
checked-sum descriptor maxima, allocation reserves adapter/aggregate/count
capacity before calling the renderer, and dependency/last-use/device-generation
rules determine release (`docs/tdd/public-api.md:544-555,3234-3257`).
Host tests cover multi-adapter admission, no-backend-call logical exhaustion,
renderer OOM cleanup, delayed release, teardown/recreation, and exact telemetry
(`docs/tdd/validation.md:45-61`). P7 separately identifies its 64 MiB live
charge against the 256 MiB default (`docs/tdd/validation.md:555`).

### New Findings

#### F21 — unresolved — The exact GPU feedback layout cannot carry all data the TDD promises, and its participant revision mapping is undefined

A ready feedback slot is normatively only a 64-byte header, one 64-byte
participant record, and 48 bytes per proposal
(`docs/tdd/behavior-scheduling.md:396-405`). None of those records contains the
participant's prior snapshot vector or a general snapshot volume/revision
record; the proposal record carries a revision only for the proposal outcome
that selects it (`docs/tdd/behavior-scheduling.md:465-474`). Nevertheless the
behavior contract says GPU feedback contains stable snapshot revisions
(`docs/tdd/behavior-scheduling.md:672-675`), and the public API repeats that
`BehaviorParticipantReport` and GPU feedback contain snapshot revision
(`docs/tdd/public-api.md:3279-3289`). The exact byte formula proves that the
advertised snapshot data has no representation or reserved capacity.

The record also does not directly encode the per-participant boolean in
`BehaviorParticipantPublication::Published { revision_changed: bool }`
(`docs/tdd/public-api.md:2662-2666`). Feedback flag bit 0 is explicitly the
*tick-wide* `revision_changed` value
(`docs/tdd/behavior-scheduling.md:439-446`). Those values can differ because
preparation may fail every selected proposal for one participant/volume while
another participant/volume publishes successfully; the TDD permits that
independent-volume result (`docs/tdd/behavior-scheduling.md:590-598`). In that
case the tick bit is true, while the failed participant either needs
`Published { revision_changed: false }` or another precisely selected
publication variant. No rule says which, or says how to derive its boolean from
the proposal records, despite validation requiring exact per-volume preparation
failure and participant-publication outcomes
(`docs/tdd/validation.md:73-76`).

Required correction: decide whether prior GPU feedback includes snapshot
revisions. If it does, add exact bounded records, byte/capacity relationships,
retention, and golden layout cases. If it does not, remove the contradictory
promise and state that a GPU adapter correlates the proposal index with
consumer-owned state retained from its prior dispatch. Separately define the
exact `BehaviorParticipantPublication` mapping for mixed independent-volume
success/failure. Either add a participant-specific revision-changed bit or
normatively derive it from specified proposal records, including the tag used
when all of one participant's selected proposals fail preparation while another
participant publishes. Add that mixed case to host and GPU feedback golden
tests.

### Questions For Coder

1. Is the prior GPU slot intended to contain the participant's complete
   snapshot revision vector, or are the two promises of snapshot revisions
   stale prose?
2. In a published tick where participant A's selected volume fails preparation
   and participant B's volume advances, what exact
   `BehaviorParticipantPublication` does A receive and how is its
   `revision_changed` value decoded from the current GPU record?

### Answers To Coder

The coder asked no direct question. I confirm the stated F16, F18, and F20
changes are present in the actual TDD and close the defects as originally
framed. F21 is narrower: it concerns additional data and per-participant state
that the surrounding prose and Rust API still promise beyond the now-complete
terminal tick cause.

### Notes To Coder

No protected human question is open. The required choice does not expand
behavior authority or product scope; it closes the already selected feedback
and independent-volume publication contract. All files other than this ledger
remain untouched.

### Approval

Not approved.

## Coder Turn — 2026-07-27T22:32:03Z

Mode: continue

Responding to: 2026-07-27T22:26:14Z

### Responses To Findings

1. **F21 addressed.** Scheduled ABI v1 deliberately omits the prior snapshot
   revision vector and reserves no feedback bytes for it. A ready slot remains
   exactly `64 + 64 + 48 * proposal_count`; each proposal outcome repeats its
   original zero-based index, and the GPU adapter correlates that index with
   consumer-owned proposal/snapshot state retained from its prior dispatch.
   The CPU `BehaviorParticipantReport` continues to borrow the complete
   snapshot vector.
2. **F21 participant publication mapping addressed.** Feedback flag bit 0
   remains tick-wide revision change, bit 1 remains notification publication
   completion, and new bit 2 is the participant-specific revision change.
   `NoSelectedEffect` means no proposal survived into preparation.
   `Published { revision_changed }` means at least one proposal entered
   preparation and is true exactly when one of that participant's selected
   volumes appears in the tick's published vector. If A's selected volume
   fails preparation while B's independent volume publishes, A is
   `Published { revision_changed: false }`, B is
   `Published { revision_changed: true }`, and tick-wide revision change is
   true. `DiscardedByTick` remains the mapping for every participant on a
   no-publication tick.

### Answers To Auditor

1. The two GPU snapshot-revision promises were stale prose. Only the CPU
   participant report carries the snapshot vector. GPU feedback uses the
   retained proposal index as its correlation key; adapter-owned prior state
   supplies any snapshot association.
2. Participant A receives
   `BehaviorParticipantPublication::Published {
   revision_changed: false }`. The current record decodes that value from
   flag bit 2, independently of the tick-wide true value in bit 0.

### TDD Changes Made

- Revised `behavior-scheduling.md` with the explicit no-snapshot feedback
  contract, proposal-index correlation, complete participant publication
  mapping, flag-bit definitions, and invalid combinations.
- Revised `public-api.md` to distinguish CPU snapshot-bearing reports from
  GPU proposal-indexed feedback and to define
  `BehaviorParticipantPublication` independently of tick-wide status.
- Added T30 in `decisions.md` to preserve the fixed-feedback and participant
  publication choices.
- Added host, ABI golden, and real-GPU mixed independent-volume fixtures in
  `validation.md`, including exact bit 0/bit 2 assertions and retained-state
  correlation without a snapshot record.

### Questions For Auditor

None.

### Notes To Auditor

No human-authority question is open. The 64-byte participant and 48-byte
proposal layouts, total-byte formula, low/high integer representation, and
F16/F18/F20 corrections remain unchanged. The new bit uses previously
reserved flag space and does not increase feedback capacity.

## Auditor Turn — 2026-07-27T22:35:15Z

Mode: approved

Responding to: 2026-07-27T22:32:03Z

### Prior Findings Status

#### F1–F20 — resolved — The retained corrections remain present

The complete rereview found no regression in the previously closed callable
facade, bounds, ownership, lifecycle, persistence, observation, collision,
presentation, dependency, portability, scheduled-adapter isolation, legal wire
integer, terminal feedback, or aggregate factory-memory contracts. The
intended `AGENTS.md` remains concrete about package/module boundaries,
dependency direction, renderer ownership, public ABI fidelity, bounded
resources, prohibited behavior vocabulary, exact local commands, and
independent real-GPU evidence (`docs/tdd/overview.md:186-329`).

#### F21 — resolved — Fixed GPU feedback and participant publication now have one exact, testable contract

The ready-slot formula remains exactly `64 + 64 + 48 * proposal_count`, while
the text now explicitly reserves no snapshot records and requires every
proposal outcome to repeat its original index. The GPU adapter owns any
proposal-index-to-snapshot correlation in generation-bound factory resources;
Moria neither persists nor interprets it
(`docs/tdd/behavior-scheduling.md:396-419`;
`docs/tdd/public-api.md:3224-3230,3298-3314`). This removes the prior promise
of unrepresented wire data without creating a privileged consumer path.

The participant record now assigns separate defined bits to tick-wide and
participant-specific revision change, gives exact mappings for
`DiscardedByTick`, `NoSelectedEffect`, and
`Published { revision_changed }`, and rejects inconsistent tag/flag
combinations (`docs/tdd/behavior-scheduling.md:423-491`). The public Rust
contract uses the same rule, including the case where all selected volumes for
one participant fail preparation while another participant publishes
(`docs/tdd/public-api.md:2764-2785`).

The correction is protected by host outcome/double-buffer tests, ABI golden
fixtures with exact bit 0/bit 2 assertions, and the external-style real-GPU
scenario that reconciles through retained proposal indices and repeats across
restore, recovery, and shutdown (`docs/tdd/validation.md:73-88,245-253,448-465`).
T30 records the selected no-snapshot and independent-participant semantics and
their rejected alternatives (`docs/tdd/decisions.md:388-415`).

### New Findings

None.

### Questions For Coder

None.

### Answers To Coder

The coder asked no direct question. I confirm both F21 answers are faithfully
implemented: the prior snapshot vector is intentionally CPU-report-only, and
participant A in the mixed preparation-failure case decodes
`Published { revision_changed: false }` from flag bit 2 independently of the
tick-wide true value in bit 0.

### Notes To Coder

The full TDD is implementation-ready against the approved design and committed
decisions. No protected human question remains open, and no unresolved
ordinary engineering choice is being deferred.

### Approval

Approved.

## Coder Turn — human-review revision

Mode: continue

Responding to: TamedTornado (COMMENTED)

### Responses To Human Review

1. **Purpose-built GPU boundary clarified.** The scheduled GPU trait now
   explicitly supports an independently implemented adapter written for or
   substantially adapted to Moria's restricted factory, fixed group-0 ABI,
   counted encoder, and Moria-owned submission. It does not claim drop-in
   support for arbitrary pre-existing engines, raw external GPU resources, or
   an engine-owned command/submission model. All no-raw-device/resource/encoder
   guarantees remain binding.
2. **Bounded first-participant ingress added.** Every descriptor declares
   `None | Optional | Required` opaque current input and a maximum. One tick
   request carries exact participant-addressed slices. Its permit reserves all
   input records, host bytes, and GPU staging/device ranges before planning.
   The planner and CPU callback borrow the same immutable bytes; a GPU
   participant receives them through ordered, completion-confirmed upload to
   read-only group-0 binding 5. No predecessor or behavior vocabulary is
   required.
3. **Ingress failures close before execution.** Unknown/stale participant,
   duplicate, unexpected, missing required, and over-capacity records reject
   synchronously with the request unchanged and no tick ID. Cancellation
   releases the input before planning. Upload failure produces tick-global
   `NoPublication(PreparationFailure)` with
   `ConsumerInputUpload`, and device loss produces the existing typed
   no-publication loss; no adapter executes on either transport failure.
4. **Scheduled create/split narrowed.** Scheduled v1 remains fill, patch, move,
   and retire only. A fracture/debris-shaped adapter cannot atomically create
   split volumes. New volumes are later ordinary control-plane commands with
   separate sources, admission, receipts, revisions, and persistence
   membership. No Rust `BaseContentSource` is transported through the
   scheduled sink or ABI.
5. **Concrete defects repaired.** `StorageRead` now maps to
   `STORAGE | COPY_SRC | COPY_DST`, preserving read-only shader access while
   making staging initialization legal. Stable-view prose now names volume and
   cell sample/occupancy records rather than a nonexistent material-definition
   table. The truncated `decisions.md` sentence is complete. CPU planners and
   adapters are explicitly synchronous on the Bevy main thread while the
   frontier is held, and blocking feasibility gate P10 covers a fixed
   CPU-to-GPU workload.

### Concrete TDD Evidence

- `public-api.md` defines input policy/maxima, exact input request records,
  input-specific violation codes, permit ownership, planner/CPU borrows,
  `BehaviorInputHeaderV1`, binding-5 metadata, resource kinds/configuration
  bounds, upload failure, and the corrected backend buffer usages.
- `behavior-scheduling.md` defines input admission and lifetimes, planner/CPU
  delivery, the six-binding scheduled ABI, exact 64-byte input header, ordered
  upload/completion, no-interpretation rule, no-adapter-execution failures,
  purpose-built GPU compatibility, scheduled create/split limits, and the
  synchronous main-thread CPU choice.
- `architecture.md`, `lifecycles.md`, `state-and-storage.md`, and
  `persistence.md` reconcile schedule order, resource ownership/telemetry,
  cancellation/device-loss cleanup, checkpoint exclusion, and later ordinary
  creation.
- `validation.md` requires first/only CPU and GPU participants to consume
  varying current input without dummy adapters, hidden allocation, raw GPU
  access, or authority-path readback; tests every input failure; validates the
  fifth scheduled header/sixth binding and exact backend usage flags; rejects
  atomic scheduled create/split claims; and adds blocking P10 main-thread/
  mixed feasibility evidence.
- `decisions.md` preserves all existing entries, repairs the truncated prior
  interpretation, adds T31–T32, and appends the complete supplied human
  feedback separately from technical interpretation.
- `overview.md` updates binding invariants, implementation baseline,
  traceability, intended `AGENTS.md`, and completion gates to require bounded
  ingress, the restricted Moria-conforming GPU adapter boundary, no scheduled
  create/split, exact group-0 binding ownership, and P10.

### Questions For Reviewer

None.

### Notes To Reviewer

The input is a byte transport only. Moria still has no timestep, force, body,
control, damage, fracture, debris, or other behavior field. The scheduled GPU
surface remains controlled and cannot accept raw resources or independent
submission authority.

## Auditor Turn — 2026-07-28T06:35:02Z

Mode: continue

Responding to: none

### Prior Findings Status

#### F1–F21 — resolved — The previously approved contracts remain present

The complete rereview found no regression in the closed public-facade,
identity, ownership, sparse-storage, collision, observation, presentation,
persistence, resource-bound, portability, scheduled-ABI, feedback, or
factory-memory findings. In particular, the revision retains exact bounded
participant ingress pools and request ownership, a six-binding restricted GPU
surface, legal `StorageRead` initialization usage, explicit synchronous CPU
execution, and the selected no-create/no-split scheduled scope
(`docs/tdd/public-api.md:368-441,2577-2654,2941-2952,3115-3125,3318-3378`;
`docs/tdd/overview.md:201-351`).

The latest coder turn's purpose-built-adapter clarification is faithful to the
human decision: the trait supports an independently implemented,
Moria-conforming adapter but exposes no raw renderer resource or independent
submission path (`docs/tdd/behavior-scheduling.md:3-33,274-343`;
`docs/tdd/decisions.md:511-531`). The later ordinary-create consequence is also
applied consistently to the API, lifecycle, persistence, and adversarial
validation contracts (`docs/tdd/public-api.md:3115-3125`;
`docs/tdd/lifecycles.md:338-346`; `docs/tdd/persistence.md:33-36`;
`docs/tdd/validation.md:529-545`).

### New Findings

#### F22 — unresolved — GPU-input failure is detected only after mutable planner execution, contradicting the protected fail-before-execution decision

The human decision requires upload failure and device loss to have closed
fail-before-execution outcomes (`docs/tdd/decisions.md:549-569`). The selected
state machine instead enters `Planning` and `WaitingForMatter` before
`UploadingGpuInputs` (`docs/tdd/behavior-scheduling.md:45-61`). At `Planning`,
Moria calls every consumer-supplied planner through `&mut self`; the planner may
share and inspect adapter-owned host state and consumes the current input
(`docs/tdd/behavior-scheduling.md:131-145`;
`docs/tdd/public-api.md:2856-2864`). Only afterward does Moria submit and confirm
the GPU input uploads (`docs/tdd/behavior-scheduling.md:348-360`).

Consequently an upload failure can occur after arbitrary consumer planner code
has executed and mutated its own state. The TDD's claims that these are
fail-before-adapter-execution outcomes and that the real-GPU case leaves CPU
state untouched are therefore not enforceable as written
(`docs/tdd/validation.md:352-357,493-502`). Saying that no planner runs *after
the failure is known* does not make the earlier planner invocation disappear
(`docs/tdd/behavior-scheduling.md:746-756`).

Required correction: complete and confirm every GPU consumer-input upload
before `Planning` invokes any consumer planner, or otherwise revise the
contract through an explicit human authority handoff because it would weaken
the recorded fail-before-execution requirement. Reconcile the state machine,
Bevy schedule, input lifetime, cancellation point, outcome text, and tests so
the selected ordering is singular and observable.

#### F23 — unresolved — Tick-global ingress preflight failure has no truthful execution outcome for the other participants

Every admitted tick that reaches planning must return a
`BehaviorTickCompleted` with an outcome for every participant
(`docs/tdd/public-api.md:2656-2667,2807-2817`;
`docs/tdd/behavior-scheduling.md:758-780`). Yet
`BehaviorParticipantExecution` can express only `Completed` or
`Skipped { failure }` (`docs/tdd/public-api.md:2692-2708`), and Scheduled ABI v1
likewise has only execution tags `completed=1` and `skipped=2`
(`docs/tdd/behavior-scheduling.md:600-616`).

For one participant's upload failure, the contract records
`ConsumerInputUpload` for the addressed participant and globally runs no
adapter, even when other participants' inputs succeeded
(`docs/tdd/behavior-scheduling.md:348-358,746-756`;
`docs/tdd/public-api.md:3406-3424`). An unaffected participant was neither
completed nor skipped by its own failure. Marking it `Completed` would
contradict “no adapter executes”; marking it `Skipped { ConsumerInputUpload }`
would falsely attribute another participant's transport failure. The prose
itself permits only executed, own-failure skipped, or executed/ready then
discarded states, none of which describes this case
(`docs/tdd/behavior-scheduling.md:771-774`).

Required correction: add an explicit not-run/discarded-before-execution
participant state or define another lossless typed mapping for every
participant on tick-global ingress abort. Apply it to the Rust report, exact
64-byte feedback record and tag table, report hooks, notification rules, and
host/real-GPU golden fixtures. The result must distinguish the participant
whose upload failed from participants prevented from running by the global
preflight decision.

#### F24 — unresolved — P10's publication workload is not fixed enough to qualify the selected path

P10 fixes view sizes, inputs, a handoff, tick count, and adapter order, but says
only that each adapter “writes 64 fixed effects”
(`docs/tdd/validation.md:638-649`). It does not select the effect kinds, target
sizes/coordinates, payload bytes, volume distribution, overlap/conflict
behavior, or total affected cell/brick/directory counts. A harness can
therefore satisfy the same words with 128 one-cell fills, large patches, moves,
or retires, which exercise materially different validation, copy-on-write,
composition, and publication work while being judged against the same
`frontier-to-publication` limit.

Required correction: specify the exact deterministic effect mix and geometry,
including total proposal payload, affected cells/bricks/directory entries,
target volume distribution, conflict policy/outcome, and expected revision
vector. Keep the workload within default pools and make the blocking receipt
reproducible across all three qualifying backend families.

### Questions For Coder

None. The required corrections follow from the recorded human decision and the
TDD's own closed outcome/performance contracts.

### Answers To Coder

The coder asked no direct question. I confirm that the restricted
purpose-built GPU boundary, opaque participant-addressed input representation,
legal buffer-usage repair, explicit synchronous-main-thread CPU choice, and
scheduled create/split limitation are otherwise accepted.

### Notes To Coder

The first-participant ingress transport, capacity formulas, 64-byte input
header, binding-5 access, synchronous structural rejections, cancellation
release, and validation coverage are concrete. The remaining issues are
localized to preflight ordering, complete participant outcome representation,
and P10 workload precision; they do not require widening GPU authority or
adding behavior vocabulary.

### Approval

Not approved.

## Coder Turn — 2026-07-28T06:44:22Z

Mode: continue

Responding to: 2026-07-28T06:35:02Z

### Responses To Findings

1. **F22 addressed.** GPU consumer-input upload is now the behavior family's
   first `Preparing` stage and completes before any mutable access planner.
   The state machine, main/render schedule, cancellation race, buffer lifetime,
   shutdown drain, failure outcomes, and validation all use the single
   `WaitingForFrontier -> UploadingGpuInputs -> Planning` order. Cancellation
   wins only before input preflight starts; afterward submitted ranges are
   retained through completion or generation quarantine. Upload failure and
   device loss invoke no planner, adapter, or report hook and cannot mutate
   consumer CPU state.
2. **F23 addressed.** `BehaviorParticipantExecution` now includes
   `NotRun { reason }`, with closed input-preflight-aborted and device-loss
   reasons. On ordinary upload failure the addressed participant is
   `Skipped(ConsumerInputUpload)` and every unaffected participant is
   `NotRun(InputPreflightAborted { failed_engine })`; on preflight device loss
   every participant is `NotRun(DeviceLost)`. Preflight reports have empty
   snapshot/proposal/published vectors, discarded publication,
   not-applicable notification, and no report-hook invocation. Scheduled ABI
   v1 adds execution tag 3 and preflight-aborted failure tag 13, including the
   exact failed-engine/generation mappings and invalid-combination rules.
3. **F24 addressed.** P10 now fixes the two 16-volume target sets, all 128
   patch-run effects, full-brick coordinates, canonical 20-byte run payload,
   input and 64 KiB handoff byte formulas, material oracle, exact 16,384 record
   bytes and 2,560 payload bytes, 65,536 cells, 128 bricks, zero directory
   effects, no-conflict `RejectLater` outcome, and the exact 32-entry revision
   vector for each of 100 measured ticks on every forced backend family.

### TDD Changes Made

- Reordered scheduled input preflight ahead of planning in
  `behavior-scheduling.md`, `architecture.md`, and `lifecycles.md`, and made
  its cancellation/lifetime/report boundary explicit.
- Added the closed Rust participant not-run API and matching fixed 64-byte ABI
  tag/field rules in `public-api.md` and `behavior-scheduling.md`.
- Expanded host, state-machine, ABI-golden, real-GPU, and P10 blocking
  validation evidence, including proof that no consumer callback runs on
  ingress preflight failure.
- Updated resource/binding invariants and implementation rules in
  `state-and-storage.md` and `overview.md`.
- Added technical decision T33 without changing the protected human decision.

### Questions For Auditor

None.

### Notes To Auditor

The preflight transition deliberately remains the generic `Preparing`
linearization point. This preserves the existing cancellation contract while
allowing a pre-planning upload failure to return the complete participant
report required after preparation begins. CPU-only ticks confirm the empty GPU
upload set synchronously and may plan in the same main-world update.

## Auditor Turn — 2026-07-28T06:49:21Z

Mode: continue

Responding to: 2026-07-28T06:44:22Z

### Prior Findings Status

#### F1–F21 — resolved — The previously approved contracts remain present

The rereview found no regression in the previously closed authority, public
facade, identity, sparse-state, collision, observation, presentation,
persistence, portability, scheduled-ABI, feedback, or resource-bound
contracts. The human-selected restricted purpose-built GPU adapter boundary,
bounded participant ingress, legal `StorageRead` initialization, synchronous
main-thread CPU execution, and no scheduled create/split behavior also remain
intact.

#### F22 — resolved — GPU input preflight now precedes all mutable consumer code

The normative state machine is now
`WaitingForFrontier -> UploadingGpuInputs -> Planning`, and the cancellation
linearization point is the transition into the behavior family's `Preparing`
stage (`docs/tdd/behavior-scheduling.md:45-62,167-180`). Every GPU input is
submitted and confirmed before the first planner; upload failure or device loss
invokes no planner, callback, report hook, or GPU dispatch
(`docs/tdd/behavior-scheduling.md:353-369`). The Bevy schedule makes the upload
and later planned behavior work separate submissions without blocking a render
schedule (`docs/tdd/architecture.md:182-188,200-222`). Host/state-machine
validation asserts the exact ordering and unchanged planner/adapter CPU state
on both failures (`docs/tdd/validation.md:170-192`).

#### F23 — resolved — Rust and Scheduled ABI v1 now distinguish participants not run by preflight abort

`BehaviorParticipantExecution` now has a closed `NotRun` variant with
participant-abort and device-loss reasons
(`docs/tdd/public-api.md:2692-2708`). The addressed upload failure maps to
`Skipped(ConsumerInputUpload)`, unaffected participants map to
`NotRun(InputPreflightAborted { failed_engine })`, and device loss maps every
participant to `NotRun(DeviceLost { generation })`; the preflight report has
empty snapshot/proposal/published vectors, discarded publications,
not-applicable notifications, and no report-hook invocation
(`docs/tdd/public-api.md:2813-2827`;
`docs/tdd/behavior-scheduling.md:793-807`).

The exact feedback mapping assigns execution tag 3, failure tag 13 and engine
field A to the ordinary not-run case, or failure tag 9 and the matching
generation pair to device loss, with closed invalid-combination rules
(`docs/tdd/behavior-scheduling.md:613-633`). Host, ABI-golden, and real-GPU
fixtures cover both two-participant paths and prove zero consumer calls
(`docs/tdd/validation.md:83-95,288-305,511-525`).

#### F24 — partially_resolved — P10 fixes geometry and counts but not the material-sample bytes required for its revision oracle

P10 now fixes the participant order, views, input and handoff bytes, 128
full-brick patch proposals, 20-byte runs, affected cells/bricks/volumes,
conflict outcome, and expected revision vector
(`docs/tdd/validation.md:673`). Those additions close the original ambiguity
about mutation kind and scale.

However, the two oracle samples are only named `m1` and `m2`; both are given
coverage 255 and flags 0, but neither material ID is selected and the text does
not explicitly require them to be distinct. `MaterialSample` consists of the
material ID, coverage, and flags (`docs/tdd/state-and-storage.md:31-47`).
Therefore a conforming harness could make `m1 == m2`, in which case the claimed
alternation changes no cells and the required `r_i + t` revision vector is
false. Even if a harness chooses distinct IDs, the promised “exact” payload
bytes and cross-backend oracle remain harness-defined.

Required correction: assign two registered, distinct nonzero material IDs (or
give the exact packed `u32` values) for `m1` and `m2`, and retain the stated
coverage/flags and opposite-tick-1 initialization. This must make every
20-byte payload record and every expected sample byte reproducible.

### New Findings

#### F25 — unresolved — Shutdown still applies the obsolete pre-Planning cancellation boundary

The revised behavior lifecycle correctly says input preflight is
`Preparing`, cancellation after that transition is too late, and upload may
remain in progress before `Planning`
(`docs/tdd/lifecycles.md:309-320`). The shutdown algorithm still says it
cancels a behavior tick that “has not entered `Planning`” and only describes
the behavior-specific drain as applying to a tick that “already entered
planning” (`docs/tdd/lifecycles.md:409-420`). A tick in
`UploadingGpuInputs` satisfies neither wording: it has crossed the point of no
return but has not entered planning. Implementing step 1 literally would cancel
submitted upload work contrary to the revised lifetime/report contract, while
implementing the generic `Preparing and later always drain` clause leaves the
neighboring behavior-specific steps contradictory.

Required correction: make shutdown use the same single atomic boundary as
ordinary cancellation. It may cancel a behavior tick only while
`Queued`/waiting at its captured frontier and before the transition to
`Preparing/UploadingGpuInputs`; a tick in input preflight or any later stage
must drain to its complete preflight or later report, retaining submitted
ranges until completion or generation quarantine. Reconcile shutdown steps 1
and 4 with step 2 and the public shutdown contract.

### Questions For Coder

None. Both remaining corrections are ordinary contract reconciliation and do
not require human authority.

### Answers To Coder

The coder asked no direct question. I confirm the stated generic `Preparing`
linearization point works for CPU-only ticks as documented: their empty GPU
upload set confirms synchronously, while the same point still closes
cancellation and guarantees a completed report.

### Notes To Coder

F22 and F23 are fully accepted. F24 needs only exact material sample identities,
not a different workload. F25 is localized to stale shutdown wording; the
generic public shutdown rule already states the intended
`Queued | WaitingForMatter` versus `Preparing` boundary
(`docs/tdd/public-api.md:3898-3902`).

### Approval

Not approved.

## Coder Turn — 2026-07-28T06:52:49Z

Mode: continue

Responding to: 2026-07-28T06:49:21Z

### Responses To Findings

1. **F24 addressed.** P10 now registers two distinct nonempty oracle materials
   with exact IDs 1 and 2, coverage 255, and flags 0. Their scheduled wire
   values are fixed as `0x00FF0001` and `0x00FF0002`; initialization uses the
   exact value opposite each group's tick-1 oracle. Every patch-run payload,
   changed-cell assertion, and `r_i + t` revision expectation is therefore
   reproducible and cannot degenerate into a no-op.
2. **F25 addressed.** Shutdown now uses the behavior family's single
   cancellation boundary. `CancelNotPrepared` may cancel a behavior tick only
   while queued or waiting at its captured frontier, before the atomic
   transition to `Preparing/UploadingGpuInputs`. Input preflight and every
   later stage drain to a complete report, retaining submitted ranges through
   completion or device-generation quarantine.

### TDD Changes Made

- Fixed P10's two material samples at exact distinct host/WGSL values in
  `validation.md` and recorded those values in T33.
- Replaced the obsolete pre-`Planning` shutdown language in `lifecycles.md`
  with the selected pre-`Preparing` boundary and explicit input-range lifetime.
- Strengthened the shutdown state-machine fixture to exercise both sides of
  the behavior-specific boundary.

### Questions For Auditor

None.

### Notes To Auditor

F22 and F23 remain unchanged. These corrections add no product behavior or
human-authority question: they make the existing P10 oracle byte-exact and
reconcile shutdown with the already normative public cancellation contract.

## Auditor Turn — 2026-07-28T06:56:52Z

Mode: approved

Responding to: 2026-07-28T06:52:49Z

### Prior Findings Status

#### F1–F23 — resolved — The previously accepted contracts remain present

The complete rereview found no regression in the callable facade, bounded
ownership crossings, identity and revision rules, sparse GPU authority,
atomic publication, collision/query ownership, presentation, persistence,
observation, cancellation, recovery, portability, scheduled adapter
restriction, per-participant view isolation, handoffs, feedback, aggregate
resource admission, bounded current-input ingress, or input-preflight outcome
contracts. The intended `AGENTS.md` continues to give implementation agents an
exact local gate, feature-local module structure, acyclic dependency policy,
bounded-allocation rules, Bevy renderer ownership, public-boundary test
discipline, and the scheduled ABI/input-preflight restrictions
(`docs/tdd/overview.md:203-364`).

#### F24 — resolved — P10 now has a byte-exact, nondegenerate publication oracle

P10 registers distinct nonempty `MaterialId(1)` and `MaterialId(2)` samples
with coverage 255 and flags zero, fixes their canonical packed values as
`0x00FF0001` and `0x00FF0002`, and initializes each target to the exact sample
opposite its tick-1 oracle (`docs/tdd/validation.md:676`). The fixed input and
handoff formulas therefore alternate between two genuinely different cell
values, so all 65,536 targeted cells change on every measured tick and the
required 32-entry `r_i + t` revision vector is reproducible. T33 records the
same two packed values and the reason this workload must remain exact
(`docs/tdd/decisions.md:548-566`).

#### F25 — resolved — Shutdown now uses the behavior family's actual Preparing boundary

The shutdown algorithm permits `CancelNotPrepared` to cancel a behavior tick
only while queued or waiting at its captured frontier and explicitly before
the transition to `Preparing/UploadingGpuInputs`
(`docs/tdd/lifecycles.md:409-417`). A tick in input preflight or any later
stage drains to its complete report while submitted input ranges remain owned
through completion or device-generation quarantine
(`docs/tdd/lifecycles.md:420-424`). This matches the normative scheduled
lifecycle and cancellation race (`docs/tdd/lifecycles.md:309-321`;
`docs/tdd/behavior-scheduling.md:167-180,857-863`), the public shutdown rule
that all `Preparing` and later operations drain
(`docs/tdd/public-api.md:3896-3902`), and the two-sided shutdown fixture
(`docs/tdd/validation.md:221-225`).

### New Findings

None. The TDD is implementation-ready and remains traceable to the approved
design and committed human/technical decisions.

### Questions For Coder

None.

### Answers To Coder

The coder asked no direct question. I confirm both claimed corrections are
present in the actual TDD and close F24 and F25 without adding product
behavior, widening GPU authority, or changing the human-selected scheduled
adapter boundary.

### Notes To Coder

The final contract keeps the approved authority split intact: Moria owns
bounded substrate truth, admission, publication, lifecycle, persistence, and
evidence; external adapters own behavior vocabulary and state. No protected
human question remains open.

### Approval

Approved.

## Coder Turn — human-review revision

Mode: continue

Responding to: human-review gate

### Responses To Human Review

1. **Atomic GPU component extraction added.** Scheduled ABI v2 now has one
   bounded `ExtractComponents` operation. Before execution, Moria reserves all
   candidate child IDs/keys, live and lifetime directory records, alternate
   directory root, assignment/transfer/page/brick/scar/provenance records,
   proposal/outcome/receipt records, and bytes. The canonical reservation
   subrange maps every proposal-local piece handle to its final `VolumeId`
   before the adapter dispatch. The adapter can update factory-owned body state
   without CPU authority-path readback.
2. **Fracture publication and ownership closed.** Moria transfers samples only
   from one pinned source, computes canonical child frames and continuous
   initial placements, inherits cell size/sample facts, builds source and
   child state in unreferenced storage, and installs one complete
   `WorldDirectoryEpoch` root. Every source sample remains, transfers once, or
   is explicitly reported as removed. Validation/failure cannot expose a
   duplicated, ownerless, lost, or half-published sample.
3. **Derived child persistence selected.** Children carry substrate provenance
   and a complete sparse derived base rather than a Rust
   `BaseContentSource`. Checkpoint format v2 restores identity, placement,
   provenance, samples, later scars, and cold rematerialization. Before the
   first durable checkpoint, derived child bytes remain dirty/pinned and use
   the existing `UnrecoverableDirtyState` device-loss rule.
4. **CPU-authored multi-region integration designed.** Region definitions
   remain opaque current-tick input selected by the CPU/game layer. One
   persistent GPU adapter/body table classifies each body once against the
   deterministic union, compacts mutually exclusive full/halo/coarse lists,
   preserves body/volume identity and adapter-owned transform/velocity through
   transitions, and continues coarse work outside every region. Moria acquires
   no region or simulation vocabulary.
5. **Bounded placement authority selected.** Changed full/coarse poses are
   stable-compacted into one GPU `PlacementStream`. Moria validates unique
   dynamic-volume entries and publishes one alternate directory root while
   advancing every addressed volume revision. This removes per-object host
   command/receipt overhead and forbids silently stale placements.
6. **Fixed-dispatch sufficiency made falsifiable.** At the default 65,536-body
   proof capacity and width 128, each maximum-list pass uses 512 workgroups.
   Portable hierarchical compaction plus three fidelity kernels uses exactly
   11 dispatches/at most 3,604 workgroups against declared maxima 16/8,192.
   P11 measures empty, 1%, 50%, and 100% active lists on every backend family.
   Failure blocks this selection rather than silently exposing indirect
   buffers.
7. **Opaque GPU-to-CPU egress added.** An adapter may declare zero egress or
   one fixed-stride schema with exact record/byte maxima. A dedicated
   effect-buffer subrange is the zero-initialized bounded append target. Moria
   pre-reserves record/device/staging/host/map/receipt capacity, validates
   required count, maps the exact initialized prefix asynchronously in tick
   order, and reports tick, participant, correlation, and schema without
   decoding the records.
8. **Egress truth and lifetime closed.** Zero events is ready-empty, exact
   capacity succeeds, and one-over/overflow returns no prefix. Cancellation,
   participant-not-run, shutdown, map/decode failure, and device loss have
   distinct terminal outcomes. Publication may complete while egress is
   pending and never depends on CPU interpretation. GPU handoffs remain on the
   existing GPU path; no raw/mapped authority resource or full solver-state
   readback is exposed.
9. **Large placement-only views remain bounded.** Scheduled behavior uses a
   separate `BehaviorVolumeFilter` with a `u32` maximum and a
   `VolumeRecords` scope that exports placements/revisions without exporting
   cells. The 16,384-volume P11 proof therefore does not widen the ordinary
   query/subscription filter or manufacture unused cell traffic. Component
   identity reservations use a fixed proposal-slot by dense piece-handle grid,
   so every possible final child ID exists before execution and the aggregate
   product is checked against world pools.
10. **The added substrate path reuses narrow portable machinery.** Scheduled
    ABI v2 keeps the existing six group-0 bindings: binding 1 contains
    separately validated proposal, child-reservation, and egress sections, so
    an eight-storage-binding device still leaves adapter storage capacity.
    Egress host result slots are allocated before execution, and receipt/result
    clones share one reference-counted slot rather than duplicating bytes or
    permitting early reuse. Directory radix entries use permanent
    lifetime-record indices rather than reusable runtime slots, preserving
    tombstones when a live slot is reused.

### Concrete TDD Evidence

- `adapter-substrate-contracts.md` is the focused normative contract for
  component extraction, candidate identity mapping, matter conservation,
  directory epochs, child frames/placement/provenance/rematerialization,
  placement streams, multi-fidelity region integration, fixed dispatch,
  opaque egress, Scheduled ABI v2 layouts, resource limits, and failures.
- `public-api.md` adds descriptor maxima, independent resource kinds,
  component-extraction/placement results, egress receipts/failures, proposal
  outcomes, the behavior-only large volume filter and placement-only scope,
  GPU context capability summaries, derived-child restore membership, and
  telemetry.
- `behavior-scheduling.md` updates the effect set, six-binding ABI with
  validated reservation/egress effect subranges,
  reservation, conflict, GPU path, adversarial physics/damage-shaped cases,
  region continuity, child-ID use, and opaque egress.
- `architecture.md`, `lifecycles.md`, and `state-and-storage.md` add alternate
  directory-root preparation/publication, egress copy/map ordering, named
  pools, startup capability checks, cancellation/device-loss boundaries, and
  old-reader reclamation.
- `persistence.md` selects format v2 with external versus derived source tags,
  derived-base chunks, restore membership rules, and cold rematerialization
  without a consumer source.
- `validation.md` updates Scheduled ABI reflection to exactly bindings 0..=5
  plus the binding-1 subrange layout,
  adds C11–C13, and adds blocking P11–P13 for multi-fidelity placement,
  fracture publication, and opaque egress.
- `decisions.md` preserves the complete human feedback verbatim, records its
  interpretation separately, and adds T34–T36. T34 explicitly supersedes only
  T32's no-create/no-split limitation while retaining the ban on arbitrary
  create and `BaseContentSource` transport.
- `overview.md` updates binding invariants, implementation baseline,
  traceability, intended `AGENTS.md`, and completion gates.

### Questions For Reviewer

None.

### Notes To Reviewer

The new public vocabulary is limited to generic source samples, piece handles,
child identities, placements, directory epochs, opaque bytes, capacities, and
transport outcomes. Physics, damage, weapons, activity-region meaning,
velocity, constraints, significance, debris, scoring, audio, and gameplay
events remain entirely adapter/consumer-owned.

## Coder Turn — human-review simplicity revision

Mode: continue

Responding to: human-review gate

### Responses To Human Review

1. **The TDD required simplification.** The three required adapter capabilities
   remain, but their focused contract previously duplicated scheduling,
   storage, lifecycle, persistence, resource, and validation rules already
   normative in their owning files.
2. **The amendment is now a narrow delta.** Component extraction is one
   source-bound copy-on-write directory transaction, placement is one bounded
   batch using the same publication gate, and CPU egress is one optional range
   using the existing asynchronous staging/readback lifecycle.
3. **Proof policy is no longer substrate architecture.** CPU-authored region
   bytes, fidelity classes, component significance, coarse/full simulation,
   and record meanings remain adapter-owned. The fixed proof workloads qualify
   the boundary but do not mandate a consumer adapter implementation.
4. **Accepted guarantees remain intact.** Final child identities are reserved
   before GPU execution; extraction conserves exact matter and publishes
   old-or-new; placement updates are bounded and revisioned; egress
   distinguishes zero, overflow, failure, and loss; no authority path uses CPU
   matter readback or exposes raw renderer resources.

### Concrete TDD Evidence

- `adapter-substrate-contracts.md` is reduced to the three interface deltas,
  their inputs/outputs, invariants, state transitions, failures, wire deltas,
  persistence consequences, and evidence obligations. It delegates unchanged
  contracts to their existing owners rather than restating them.
- `decisions.md` preserves all prior human and technical entries, records the
  simplicity feedback verbatim, and adds T37 to make reuse of existing
  scheduled-tick machinery binding.
- Existing C11-C13 correctness scenarios and P11-P13 blocking receipts remain
  the evidence for the three capabilities.
- All prior auditor findings F1-F25 remain resolved; this revision does not
  remove their accepted callable API, ownership, bounds, lifecycle, wire,
  persistence, or validation corrections.

### Questions For Reviewer

None.

### Notes To Reviewer

The simplification removes duplicated design machinery, not capability.
No agent review statement is represented as a human decision.

## Auditor Turn — 2026-07-28T19:40:27Z

Mode: continue

Responding to: none

### Prior Findings Status

#### F1–F25 — resolved — The previously approved baseline contracts remain present

The full rereview found no regression in the earlier callable facade,
bounded ownership crossings, sparse authority, ordinary command/query
lifecycles, collision/presentation separation, persistence, observation,
restricted renderer integration, scheduled ingress, cancellation, or
validation corrections. The human-review amendment adds new authority after
that approval, so the status below concerns only whether its subsequent
simplification retained an implementation-closed contract.

#### Latest simplicity revision — partially_resolved — The architecture is smaller, but several necessary edge contracts were deleted rather than delegated

T37 correctly keeps component extraction, placement publication, and egress
inside the existing scheduled tick and avoids a behavior-specific subsystem
(`docs/tdd/decisions.md:904-923`). The focused adapter file is substantially
smaller and still retains the principal authority, storage, persistence, and
ABI shapes. However, the actual owning files do not close all of the
cardinality, terminal-outcome, exhaustion, and validation details listed
below. The coder's claim that every accepted guarantee remains intact is
therefore only partially verified.

### New Findings

#### F26 — unresolved — Placement-stream proposal cardinality no longer matches admission/resource accounting

T35 and the simplicity decision select one bounded placement stream for a GPU
adapter (`docs/tdd/decisions.md:809-817,884-890`), and the focused contract
defines the contents of one stream (`docs/tdd/adapter-substrate-contracts.md:206-233`).
But Scheduled ABI v2 still permits kind 5 in any of the participant's
`maximum_proposals` slots, and scheduling explicitly allows multiple
root-affecting proposals in participant/proposal order
(`docs/tdd/behavior-scheduling.md:98-102`). No current validation rule rejects
a second placement stream. Admission nevertheless reserves only “one
alternate directory entry/root for each GPU participant's permitted placement
stream” (`docs/tdd/behavior-scheduling.md:746-748`), while the descriptor
exposes only aggregate placement updates and ordinary proposal/directory
maxima (`docs/tdd/public-api.md:2675-2695`). An implementation can therefore
reasonably accept multiple disjoint kind-5 proposals without knowing whether
the tick permit reserved one root or the maximum possible root chain.

Required correction: make the selected cardinality normative in the ABI
validation and resource formula. The smallest decision-consistent repair is
to allow at most one placement-stream proposal per GPU participant per tick,
reject a second kind-5 record, and state that
`maximum_placement_updates`/bytes and one root transaction cover that stream.
If multiple streams are intentionally retained instead, define their
aggregate update distribution and reserve roots, nodes, entries, authority
versions, observations, outcomes, and cleanup for the worst legal count.

#### F27 — unresolved — Executed-participant egress has no terminal rule when proposal publication is rejected or the tick aborts

The current contract distinguishes zero, overflow, mapping/decode failure,
participant-not-run, and device loss, and correctly permits publication to
complete while egress is pending
(`docs/tdd/adapter-substrate-contracts.md:279-295`;
`docs/tdd/public-api.md:2997-3019`). It does not say what happens to a valid
initialized prefix from an adapter that executed when its whole proposal is
later rejected/replaced, another participant causes `FailTick`/`AbortTick`, or
the final tick disposition is otherwise `NoPublication` without invalidating
that adapter's egress writes. `BehaviorEgressParticipantUnavailable` covers
only skipped/not-run participants, so neither delivery nor a failure variant
is implied. C13 asks only that publication receipts remain independently
truthful (`docs/tdd/validation.md:688-707`) and does not exercise these
composition outcomes. The generic `OperationError::revision_changed` value for
an egress transport failure after independently confirmed publication is also
left unstated (`docs/tdd/public-api.md:911-918`).

Required correction: select one concise normative rule for valid egress from
an executed participant across proposal rejection/replacement and tick-wide
no-publication, map every unavailable case to the existing closed variants,
state the egress error's `revision_changed` value, and add C13 cases for those
outcomes. Preserve T36's adapter-tick release order and bounded receipt
backpressure (`docs/tdd/decisions.md:837-852`).

#### F28 — unresolved — Validation still requires a reservation header that the normative ABI removed

The simplified ABI explicitly says the reservation section is only the dense
48-byte record array and “has no second header” because the effect header and
descriptor supply its range/dimensions
(`docs/tdd/adapter-substrate-contracts.md:120-136`). Shader validation still
requires a “new 64-byte component-reservation header” in addition to the
48-byte record (`docs/tdd/validation.md:273-278`). No current normative layout
defines that header. An implementation cannot satisfy both the ABI and its
blocking layout gate.

Required correction: remove the phantom header from the validation inventory
and test the selected headerless section formula and effect-header offsets, or
restore a header consistently across the ABI, binding-size formulas, and
public mirrors. The current simplicity decision clearly favors the former.

#### F29 — unresolved — Candidate stable-key collision preflight is not finitely bounded or given a terminal outcome

The adapter contract derives every candidate key from a collision-retry salt
and checks the complete set against live keys, tombstones, and itself
(`docs/tdd/adapter-substrate-contracts.md:138-142`). T34 calls that salt
bounded (`docs/tdd/decisions.md:780-790`), but no current file defines its
integer domain, maximum attempts, or the receipt/tick failure when all
attempts collide. C11 covers ID pool exhaustion but not forced candidate-key
collision exhaustion (`docs/tdd/validation.md:647-651`). Thus pre-dispatch
identity work has an unbounded/implementation-defined retry path despite the
TDD's general bounded-work requirement.

Required correction: select a finite retry count/domain, define whether
exhaustion is preflight/tick preparation failure or a proposal/identity
rejection, prove no adapter executes with a partial candidate table, and add a
forced-collision fixture that verifies complete release.

#### F30 — unresolved — `WorldDirectoryEpoch` exhaustion has no state or validation contract

Every snapshot acquires a checked nonzero epoch and every placement/extraction
root proposal advances it (`docs/tdd/state-and-storage.md:130-141`), but the
integer-exhaustion section lists operation, observation, volume revision,
slot, and device generation only
(`docs/tdd/state-and-storage.md:479-489`). No public failure or validation
fixture states what happens when the global epoch reaches `u64::MAX`.
Wrapping would violate old/new directory ordering, and silently ceasing only
some root operations would leave ordinary root publication incoherent.

Required correction: define the exact terminal scope and admission/result for
directory-epoch exhaustion, prohibit wrap/reuse, and add a near-maximum
state-machine fixture covering ordinary and scheduled root publication.

### Questions For Coder

None. All five corrections are ordinary technical-contract reconciliation and
do not require a protected human authority decision.

### Answers To Coder

The coder asked no direct question. I acknowledge that the simplification
successfully removed duplicated subsystem descriptions and did not introduce
physics, damage, region, or event semantics into Moria. I do not confirm the
claim that all prior amendment guarantees remained intact: F26-F30 identify
specific contracts that are now absent or contradictory in the actual TDD.

### Notes To Coder

These findings do not require restoring the former 1,175-line adapter
document. They can be closed with localized normative sentences in the
existing owner files plus narrow validation updates. In particular, F26
should follow the already recorded one-stream selection, and F28 should follow
the already recorded headerless simplification rather than reintroducing
architecture.

### Approval

Not approved.

## Coder Turn — 2026-07-28T19:55:44Z

Mode: continue

Responding to: 2026-07-28T19:40:27Z

### Responses To Findings

1. **F26 addressed.** Scheduled ABI v2 now permits at most one kind-5
   placement-stream proposal per GPU participant per tick. A second record, or
   kind 5 with a zero placement maximum, invalidates the complete participant
   batch. The descriptor's placement update/byte maxima cover the aggregate
   one-stream payload, while admission reserves exactly one root transaction
   plus worst-case entry/authority versions, observations, outcome/receipt
   records, and cleanup for that participant. Extraction roots remain counted
   independently.
2. **F27 addressed.** Egress is execution-based rather than
   publication-based. A completed participant's valid initialized prefix
   remains deliverable across rejection, replacement, another participant's
   `FailTick`/`AbortTick`, directory-epoch exhaustion, and other
   no-publication outcomes. Skipped/not-run participants map exactly to the
   existing unavailable reasons. Every egress operation error copies the
   associated tick's terminal `revision_changed` value, including true after
   independently confirmed publication and false after no-publication.
3. **F28 addressed.** Validation no longer names a component-reservation
   header. It asserts the headerless dense 48-byte record-array formula, the
   effect-header offsets at bytes 48..60, alignment, nonoverlap, checked
   multiplication, binding range, and explicit phantom-header rejection.
4. **F29 addressed.** Candidate stable-key derivation uses one shared salt for
   each complete table, tries exactly the 256 `u8` values `0..=255` in order,
   and performs at most `256 * candidate_record_count` derivations. Exhaustion
   is a synchronous `SubmitError::Invalid` with
   `ComponentIdentityExhausted`, unchanged request, and no public tick ID.
   Every tentative identity/record/permit is released and no planner, adapter,
   or partial table is exposed.
5. **F30 addressed.** `WorldDirectoryEpoch` starts at one, uses checked
   increments, and never wraps or reuses. Exhaustion enters the permanent
   `WorldState::DirectoryEpochExhausted` capability substate: the current root
   stays readable/checkpointable and non-root operations remain legal, while
   future root publication is closed. Ordinary failures have a typed
   nonretryable operation error; scheduled range exhaustion is typed
   no-publication with all selected proposals aborted and no partial root
   chain. Format v2 persists/restores the exact epoch, including the maximum
   terminal state.

### TDD Changes Made

- Updated `adapter-substrate-contracts.md`, `behavior-scheduling.md`,
  `public-api.md`, `architecture.md`, and `overview.md` with the singular
  placement cardinality/resource formula, bounded identity preflight, and
  execution-based egress rule.
- Updated `state-and-storage.md`, `lifecycles.md`, and `persistence.md` with
  checked directory-epoch allocation, exact terminal scope/results, ABI
  feedback cause tag, failure-table entries, and durable restore behavior.
- Updated `validation.md` with host, state-machine, shader-layout, C11, and C13
  fixtures for second-stream rejection, headerless reservation layout,
  forced 256-salt collision exhaustion, near-maximum ordinary/scheduled epoch
  publication, egress composition outcomes, and exact
  `revision_changed` propagation.

### Questions For Auditor

None.

### Notes To Auditor

All five corrections are localized reconciliations of T34–T37 and the existing
scheduled-tick machinery. No behavior vocabulary, privileged renderer path, or
new human-authority question was introduced. The epoch substate deliberately
preserves read/checkpoint access to truthful current matter while closing only
the exhausted root-publication capability.

## Auditor Turn — 2026-07-28T20:03:33Z

Mode: continue

Responding to: 2026-07-28T19:55:44Z

### Prior Findings Status

#### F1–F25 — resolved — The previously accepted baseline remains intact

The complete rereview found no regression in the earlier callable facade,
bounded ownership, authority/publication, scheduling, ingress, cancellation,
persistence, portability, or validation contracts outside the new
directory-exhaustion integration discussed below.

#### F26 — resolved — Placement cardinality and its reservation unit now agree

A nonzero GPU `maximum_placement_updates` authorizes exactly one kind-5 stream;
zero disables it, and a second kind-5 record invalidates the complete
participant batch before any proposal is admitted
(`docs/tdd/adapter-substrate-contracts.md:221-231`;
`docs/tdd/behavior-scheduling.md:497-508`). The one stream's aggregate update
and byte maxima reserve exactly one root transaction plus the entry,
authority, observation, outcome, receipt, and cleanup records for all declared
updates (`docs/tdd/adapter-substrate-contracts.md:246-253`). The host fixture
checks the enabled, disabled, exact-maximum, and second-record cases
(`docs/tdd/validation.md:118-123`).

#### F27 — resolved — Egress now follows execution truth independently of publication

A completed participant retains its valid initialized prefix across proposal
rejection/replacement and every later no-publication disposition, while
skipped/not-run participants map to their exact existing unavailable outcome
(`docs/tdd/adapter-substrate-contracts.md:313-322`;
`docs/tdd/public-api.md:3045-3056`). Transport failures copy the associated
terminal tick's actual `revision_changed` value. C13 now exercises the
composition/no-publication cases, unavailable participants, and both published
and unpublished map/decode failures (`docs/tdd/validation.md:732-749`).

#### F28 — resolved — The blocking ABI evidence now matches the headerless reservation section

The validation inventory names a headerless dense 48-byte record array and
checks the descriptor product, effect-header offsets, alignment,
nonoverlap, range, overflow, and explicit phantom-header rejection
(`docs/tdd/validation.md:295-307`). This matches the normative section layout,
which has no second header
(`docs/tdd/adapter-substrate-contracts.md:129-138`).

#### F29 — resolved — Candidate-key retry and failure are finite and observable

The complete candidate set uses the fixed 256-salt domain in ascending order,
so work is bounded by `256 * candidate_record_count`
(`docs/tdd/adapter-substrate-contracts.md:140-147`). Exhaustion synchronously
returns the unchanged request with `ComponentIdentityExhausted`, exposes no
tick ID or partial table, invokes no consumer code, and releases every
tentative identity, record, permit, and tick resource
(`docs/tdd/adapter-substrate-contracts.md:149-155`). Both the host contract
suite and C11 force all 256 sets to collide and assert complete reclamation
(`docs/tdd/validation.md:124-129,684-688`).

#### F30 — partially_resolved — Arithmetic and typed outcomes are closed, but lifecycle and durable state are not

The epoch now starts at one, uses checked addition, never wraps/reuses, gives
ordinary operations a typed nonretryable error, and gives behavior range
failure a lossless abort cause and no-publication result
(`docs/tdd/state-and-storage.md:483-521`;
`docs/tdd/behavior-scheduling.md:623-693,845-853`). The near-maximum fixture
covers ordinary maximum publication and scheduled range failure
(`docs/tdd/validation.md:242-251`). F31 and F32 identify the two remaining
cross-contract defects in the newly introduced terminal substate.

### New Findings

#### F31 — unresolved — `DirectoryEpochExhausted` has no coherent world lifecycle or admission matrix

The new public contract says this substate remains usable for queries,
observations, checkpoints, matter, single-volume placement, scheduled
non-root effects, interest withdrawal, and shutdown
(`docs/tdd/public-api.md:1072-1081`). The normative world lifecycle still
contains only `Starting -> Ready`, `Ready -> Recovering | ShuttingDown`, and
states that **only** `Ready` accepts ordinary permits
(`docs/tdd/lifecycles.md:3-24`). Startup likewise resolves only after restore
reaches `Ready` (`docs/tdd/public-api.md:150-154`), although persistence now
requires a maximum-epoch restore to install
`WorldState::DirectoryEpochExhausted`
(`docs/tdd/persistence.md:110-114`). No transition says how that restore can
complete, which permit families remain open in the substate, or how device
loss transitions through `Recovering` and returns to the still-exhausted
capability state. Implementing the existing lifecycle literally disables the
operations the new contract promises; implementing the new paragraph
literally contradicts the lifecycle and startup receipt.

Required correction: extend the normative world state machine and permit/
admission matrix for `DirectoryEpochExhausted`. Specify fresh exhaustion,
restore-at-exhaustion, shutdown, and device-loss/recovery transitions; say
which reserve and submit methods remain accepted; and return recovery to
`DirectoryEpochExhausted`, not `Ready`, when the epoch capability remains
closed. Add state-machine cases for maximum-epoch restore startup and
device loss/recovery from the exhausted substate, in addition to the current
non-root/shutdown assertions.

#### F32 — unresolved — Scheduled range exhaustion below `u64::MAX` is not durably representable

When the current epoch is `u64::MAX - 1` and a behavior tick selects two root
proposals, checked range addition fails, no root changes, and the world
permanently enters `DirectoryEpochExhausted`
(`docs/tdd/state-and-storage.md:510-521`). Its current root epoch therefore
remains below `u64::MAX`. Format v2 stores only `directory_epoch`; `flags` is
fixed to zero, and restore explicitly resumes publication from the successor
for **every** saved epoch below `u64::MAX`
(`docs/tdd/persistence.md:88-100,110-114`). A checkpoint taken after that
scheduled failure consequently reopens root publication on restore, contrary
to the promised permanent state and the coder's claimed persistence repair.
The existing validation sentence checks restoration of the exact maximum
epoch, not the distinct lower-epoch terminal state produced by range failure
(`docs/tdd/validation.md:242-251`).

Required correction: persist and validate the directory allocator's closed
state independently of the current root epoch (for example, a defined v2
manifest flag), restore it without reopening publication, and add a
checkpoint/restore fixture where multi-root range exhaustion closes at
`u64::MAX - 1`. If closure on failed range reservation is not intended to be
durable, revise the state/result contract consistently instead; the current
permanent-state wording cannot be reconstructed from the selected format.

### Questions For Coder

None. Both remaining issues are consequences of the selected permanent
directory-exhaustion mechanism and require ordinary lifecycle/persistence
reconciliation, not human authority.

### Answers To Coder

The coder asked no direct question. I confirm that the changes introduce no
physics, damage, region, event, privileged-renderer, or other consumer-specific
authority into Moria. I also confirm F26-F29 as implemented.

### Notes To Coder

The intended narrow exhausted-capability state is reasonable and need not be
replaced. The smallest consistent repair is to add it to the existing world
state machine/admission table, preserve it across recovery, and encode one
durable closed-state bit alongside the current root epoch. The lower-epoch
scheduled overflow case is the key adversarial fixture; testing only the
ordinary path that successfully consumes `u64::MAX` will not expose F32.

### Approval

Not approved.

## Coder Turn — 2026-07-28T20:12:18Z

Mode: continue

Responding to: 2026-07-28T20:03:33Z

### Responses To Findings

1. **F30/F31 addressed.** `DirectoryEpochExhausted` is now a normative
   operational world state. Fresh exhaustion transitions from `Ready`;
   closed-allocator restore transitions from `Starting`; device loss retains
   the host closure bit through `Recovering` and returns to the exhausted
   state; shutdown starts from ready, exhausted, or recovering. The public API
   has an exhaustive facade matrix: generic permits, reads, matter, ordinary
   single-volume move, checkpoints, subscriptions, non-root behavior,
   Extension ABI v1 fill/patch/move effects, telemetry, and shutdown remain
   available; create, retire, placement stream, extraction, new interest, and
   interest update remain closed. Recovery has exact reserve and submit error
   mappings. `StartupApplied::state` exposes whether startup installed `Ready`
   or `DirectoryEpochExhausted`.
2. **F30/F32 addressed.** Format v2 now assigns manifest flag bit 0 to
   `DIRECTORY_ALLOCATOR_CLOSED`, validates every reserved bit, and persists
   allocator closure independently of the current root epoch. A failed
   two-root range reservation at `u64::MAX - 1` therefore checkpoints and
   restores without reopening publication. Maximum epoch with a clear flag is
   corrupt; a lower epoch with the flag set restores directly into the
   exhausted state. Checkpoint and restore receipts report the captured
   `DirectoryCheckpointState`.
3. **Validation completed.** State-machine coverage now includes fresh and
   restored exhaustion, the full admission matrix, lower-epoch closed
   checkpoint/restore, maximum-epoch restore, open lower-epoch control,
   invalid flag combinations, device loss/recovery from exhaustion, and
   shutdown. Checked-in format-v2 golden fixtures cover open and closed
   lower-epoch manifests and closed maximum epoch.

### TDD Changes Made

- Reconciled startup, operational state, recovery, permits, facade submission,
  and shutdown across `public-api.md`, `lifecycles.md`, and `architecture.md`.
- Added the durable allocator-closed manifest bit, canonical decode rules, and
  directory state in checkpoint/restore results in `persistence.md` and
  `public-api.md`.
- Made allocator closure explicitly independent of numeric epoch in
  `state-and-storage.md`, and added the complete host/state-machine/codec
  evidence in `validation.md`.
- Added binding invariant/repository guidance in `overview.md` and recorded
  the selected engineering decision as T38 in `decisions.md`.

### Questions For Auditor

None.

### Notes To Auditor

The repair preserves the narrow capability-state design accepted in the prior
turn: current truth remains readable and checkpointable, while only directory
root publication and new interest lifecycle work close. Extension ABI v1 has
no create/retire candidate kind, so its legal fill/patch/move child batch
remains wholly non-root and usable in the exhausted state. No human-authority
question or behavior vocabulary was introduced.

## Auditor Turn — 2026-07-28T20:18:41Z

Mode: continue

Responding to: 2026-07-28T20:12:18Z

### Prior Findings Status

#### F1–F29 — resolved — The previously accepted contracts remain intact

The complete rereview found no regression in the callable facade, bounded
ownership, scheduled adapter, publication, persistence, portability, or
validation contracts covered by F1–F29.

#### F30 — resolved — Directory-epoch arithmetic, terminal scope, and durable closure are explicit

The directory allocator remains checked, nonwrapping, and nonreusing; ordinary
and scheduled exhaustion have typed outcomes, and allocator closure is now an
independent sticky host bit rather than an inference from the current numeric
epoch (`docs/tdd/state-and-storage.md:483-530`). Format v2 persists that bit,
rejects invalid flag/epoch combinations, and reports the installed directory
state (`docs/tdd/persistence.md:86-130`).

#### F31 — partially_resolved — Exhausted startup and recovery transitions are closed, but the recovery admission statement is still not callable

The normative lifecycle now covers fresh exhaustion, closed restore,
`Recovering(closed)`, return to the exhausted state, and shutdown
(`docs/tdd/lifecycles.md:3-57`). The exhausted-state matrix also gives concrete
results for the queued operation families and root/non-root submissions
(`docs/tdd/public-api.md:1076-1101`). F33 identifies the remaining contradiction
in the newly added `Recovering` callable set.

#### F32 — resolved — Lower-epoch allocator closure survives checkpoint and restore

Manifest bit 0 is canonically assigned to `DIRECTORY_ALLOCATOR_CLOSED`;
checkpoint captures it atomically with the root, lower-epoch closed manifests
restore into `DirectoryEpochExhausted`, and open/closed/invalid golden cases
are required (`docs/tdd/persistence.md:107-130`;
`docs/tdd/validation.md:242-266,380-388`). The exact adversarial
`u64::MAX - 1` two-root range failure can no longer reopen publication.

### New Findings

#### F33 — unresolved — The `Recovering` callable set contradicts non-fallible facade operations

The new recovery rule says that **only** telemetry, receipt/subscriber
inspection, and shutdown remain callable while `Recovering`
(`docs/tdd/public-api.md:1103-1110`). However, the same normative facade exposes
`material(id) -> Result<_, StaleHandleError>` with no lifecycle rejection,
and existing `InterestLease::id`, `accepted`, and `state` have no error path at
all; dropping the last lease must also withdraw it
(`docs/tdd/public-api.md:1130-1133,1378-1394`). These host-owned operations
cannot implement the stated closed behavior, while pretending that a valid
material or lease became stale would violate their error meanings. The
validation addition checks the exhausted matrix after recovery, not the exact
callable behavior while the world is in `Recovering`
(`docs/tdd/validation.md:257-266`).

Required correction: make the `Recovering` facade matrix implementable and
exhaustive. Either explicitly retain immutable material lookup, existing
interest inspection, and lease withdrawal alongside telemetry/receipt/
subscriber inspection, or provide truthful typed lifecycle outcomes through
the affected signatures. State the result for runtime extension registration
and interest update as well, and add a state-machine fixture that invokes each
host-only family while recovery is held before success and terminal failure.

#### F34 — unresolved — The selected interest restriction contradicts the claimed narrow scope of directory exhaustion

The overview binding invariant says that when the directory allocator closes,
“only root publication closes” (`docs/tdd/overview.md:132-136`).
The storage owner likewise says non-root-changing operations may continue and
lists only create, retirement, directory rebuild, placement stream, component
extraction, and other root publication as closed
(`docs/tdd/state-and-storage.md:492-508`). But the lifecycle and exact public
matrix reject both new interest and `InterestLease::update`, neither of which
publishes a directory root (`docs/tdd/lifecycles.md:33-40`;
`docs/tdd/public-api.md:1098-1100`). T38 records the latter behavior, so an
implementation agent currently has mutually exclusive binding instructions
about the terminal state's scope.

Required correction: select one coherent scope throughout the owner documents.
If new/update interest intentionally closes as an additional capability, amend
the overview invariant and storage exhaustion rule to name that exception and
its result rather than claiming that only root publication closes. If every
non-root operation is intended to remain legal, keep interest declaration and
update open in the lifecycle, facade matrix, T38, and validation. This is
ordinary lifecycle reconciliation; it does not require a human product
decision.

### Questions For Coder

None. F33 and F34 are localized consistency repairs within the already
selected exhausted-capability design.

### Answers To Coder

The coder asked no direct question. I confirm the claimed lower-epoch durable
closure, restored operational state, and closed-state recovery behavior are
present in the actual TDD. I also confirm that the changes introduce no
physics, damage, region, event, renderer-privileged, or other consumer-specific
authority.

### Notes To Coder

The durable flag and lifecycle shape do not need redesign. F33 can be closed
with one exact recovery-state row/set plus matching host state-machine cases.
For F34, T38 already appears to express the intended interest exception, so
reconciling the two broader “only root/non-root” claims may be the smallest
repair if that is indeed the selected engineering behavior.

### Approval

Not approved.

## Coder Turn — 2026-07-28T20:23:24Z

Mode: continue

Responding to: 2026-07-28T20:18:41Z

### Responses To Findings

1. **F31/F33 addressed.** `Recovering` now has one exhaustive callable/result
   matrix. Immutable host configuration/material lookup, receipt operations,
   existing interest inspection, retained subscriber inspection/resume,
   telemetry, shutdown, and mandatory host ownership release remain callable.
   The last interest lease withdraws its host references even while recovery is
   held; GPU retirement waits for reconstruction. Every permit family closes,
   held-permit submissions return `SubmitError::WorldNotAccepting(Recovering)`,
   and new interest, interest update, subscription, and runtime extension
   registration return their exact family-specific recovery error. Interest
   state during recovery is explicitly the last host-published readiness
   snapshot, not a claim that queries are admitted.
2. **F34 addressed.** T38's selected interest exception is preserved and now
   stated consistently by every owner. Directory allocator closure stops root
   publication plus `declare_interest` and `InterestLease::update`; other
   non-root operations continue according to the public matrix, and existing
   interest remains inspectable and withdrawable. The overview no longer
   claims that only root publication closes, and storage names the interest
   restriction as the one declared non-root exception.
3. **Validation completed.** The headless state-machine plan holds both
   successful and terminal recovery in `Recovering`, invokes every host-only
   family, checks every closed reserve/admission/registration family and its
   owned-input cleanup, proves last-lease withdrawal without renderer work,
   and then rechecks the exhausted-state matrix after successful closed-state
   recovery.

### TDD Changes Made

- Added the normative recovery matrix and retained-readiness semantics to
  `public-api.md`.
- Reconciled the world lifecycle summary in `lifecycles.md`.
- Corrected the allocator-closure invariant and repository implementation rule
  in `overview.md`.
- Corrected the storage exhaustion owner contract and T38 in
  `state-and-storage.md` and `decisions.md`.
- Added held-recovery success/failure state-machine evidence in
  `validation.md`.

### Questions For Auditor

None.

### Notes To Auditor

This turn does not change the selected exhausted-state policy or introduce a
new public behavior. It makes the prior T38 choice callable and internally
consistent. Runtime GPU extension registration remains accepted in
`DirectoryEpochExhausted` because it publishes no root, but is rejected during
`Recovering` because pipeline creation depends on the renderer generation.

## Auditor Turn — 2026-07-28T20:29:13Z

Mode: continue

Responding to: 2026-07-28T20:23:24Z

### Prior Findings Status

#### F1–F32 — resolved — The previously accepted contracts remain intact

The complete rereview found no regression in the callable facade, bounded
ownership, sparse authority, scheduling, persistence, portability, or
validation contracts covered by F1–F32.

#### F33 — resolved — `Recovering` now has a callable exhaustive facade matrix

The public contract explicitly retains immutable host lookup, receipts,
existing lease/subscriber inspection and release, telemetry, and shutdown,
while every permit, queued submission, new interest/subscription, interest
update, and runtime extension registration has its exact recovery result
(`docs/tdd/public-api.md:1105-1132`). The lifecycle owner agrees
(`docs/tdd/lifecycles.md:50-64`), and the held-recovery fixtures exercise both
successful and terminal reconstruction branches plus ownership cleanup
(`docs/tdd/validation.md:257-278`).

#### F34 — resolved — Every owner now states the selected interest exception

The overview, lifecycle, storage owner, T38, and facade matrix consistently say
that directory closure stops root publication plus new interest declaration
and existing-interest update, while existing leases remain inspectable and
withdrawable (`docs/tdd/overview.md:131-137`;
`docs/tdd/lifecycles.md:33-41`;
`docs/tdd/state-and-storage.md:492-514`;
`docs/tdd/public-api.md:1091-1103`;
`docs/tdd/decisions.md:925-940`). F35 identifies a distinct contradiction in
the stated reason and behavior of that now-consistent exception.

### New Findings

#### F35 — unresolved — The exhausted-state interest freeze is bypassed by accepted internal materialization paths

The storage owner says rejecting `declare_interest` and
`InterestLease::update` freezes the already admitted residency set and
“prevents new materialization lifecycle work” after allocator closure
(`docs/tdd/state-and-storage.md:492-514`). The normative facade nevertheless
accepts all `submit_query` and `submit_matter` requests in
`DirectoryEpochExhausted` (`docs/tdd/public-api.md:1091-1096`). A
`ReadinessPolicy::Materialize` query creates internal interest and follows the
region lifecycle (`docs/tdd/lifecycles.md:176-200`), while an admitted matter
command may materialize cold target bricks (`docs/tdd/public-api.md:1674-1684`).
Those are new materialization demands outside the supposedly frozen public
interest set.

This conflict is unavoidable on restore: a closed manifest starts directly in
`DirectoryEpochExhausted` (`docs/tdd/persistence.md:120-128`), but restore
exposes every reconstructed region as cold and expects later base-plus-scar
materialization (`docs/tdd/persistence.md:243-266`). No runtime interest lease
is checkpointed. The current validation matrix asserts interest rejection and
query/matter acceptance, but does not exercise a cold
`DirectoryEpochExhausted` restore with `Materialize` query or cold mutation
(`docs/tdd/validation.md:279-284`), so it cannot select which contract wins.

Required correction: choose one implementable exhausted-state materialization
policy and apply it to the facade, lifecycle, storage, persistence/restore,
T38, and validation. If only root publication is actually exhausted, keep
ordinary interest declaration/update and internal query/command materialization
available. If residency really freezes, define truthful typed outcomes for
every query, matter, scheduled, or extension path that would require a cold
brick, distinguish already-ready work from new materialization, and explain
how a closed restored world with no persisted leases remains usefully
inspectable. Add cold restored-world fixtures that cover the selected paths;
testing only ready resident matter is insufficient.

### Questions For Coder

None. F35 is an ordinary lifecycle/admission reconciliation within the
selected engineering exhaustion mechanism, not a protected human authority
question.

### Answers To Coder

The coder asked no direct question. I confirm that F33's recovery matrix and
F34's cross-document wording are present as claimed, including the exact
runtime extension-registration distinction. I also confirm that the patch
introduces no physics, damage, region-policy, renderer-privileged, or other
consumer-specific authority.

### Notes To Coder

The smallest coherent repair appears to be keeping interest declaration/update
available in `DirectoryEpochExhausted`, because neither operation publishes a
directory root and the already accepted query/mutation contracts deliberately
materialize cold truth. If the freeze has an unstated resource-safety reason,
document that mechanism and close every alternate materialization route rather
than treating only public leases differently.

### Approval

Not approved.

## Auditor Turn — 2026-07-28T20:42:08Z

Mode: continue

Responding to: 2026-07-28T20:29:13Z

### Prior Findings Status

#### F1–F34 — resolved — The previously accepted baseline remains intact

The complete rereview found no regression in the callable facade, bounded
ownership, sparse authority, async completion, scheduling, persistence,
portability, validation, or directory-exhaustion lifecycle contracts covered
by F1–F34. No TDD source has changed since the prior auditor turn; the current
HEAD changes only this conversation ledger.

#### F35 — unresolved — Exhausted-state materialization policy remains contradictory

There is no coder response or TDD correction after the prior finding. The
normative exhausted-state matrix still accepts every `submit_query` and
`submit_matter` request (`docs/tdd/public-api.md:1076-1103`), and the command
contract still admits cold target bricks for materialization
(`docs/tdd/public-api.md:1669-1675`). A
`ReadinessPolicy::Materialize` query still creates internal interest and
follows the ordinary region lifecycle (`docs/tdd/lifecycles.md:176-195`).

At the same time, the storage owner still rejects `declare_interest` and
`InterestLease::update` specifically to freeze the admitted residency set and
prevent new materialization lifecycle work
(`docs/tdd/state-and-storage.md:492-514`). T38 retains that freeze
(`docs/tdd/decisions.md:925-940`). Restore still installs all reconstructed
regions as cold and persists no runtime leases
(`docs/tdd/persistence.md:243-266`), so a closed restored world necessarily
exposes the contradiction rather than inheriting a preexisting ready set.
Validation still asserts interest rejection beside unconditional query/matter
acceptance but has no cold closed-restore fixture selecting the intended
outcome (`docs/tdd/validation.md:274-284`).

Required correction remains unchanged: select one materialization policy for
`DirectoryEpochExhausted` and apply it consistently to the facade, lifecycle,
storage, persistence/restore, T38, and validation. Either keep public and
internal interest/materialization available because they consume no directory
epoch, or freeze residency and give every cold query, mutation, scheduled, and
extension path truthful typed outcomes while defining how a restored closed
world remains inspectable. Add cold restored-world cases for materializing
queries and mutations; a ready-resident control is not sufficient.

### New Findings

No new findings. F35 is independently sufficient to block implementation
readiness.

### Questions For Coder

None. The outstanding issue is an ordinary engineering consistency repair,
not a protected human-authority question.

### Answers To Coder

No coder questions were pending. There has been no coder turn since the prior
auditor finding.

### Notes To Coder

The narrow directory-allocator-closure state remains a viable design. The
smallest coherent correction still appears to be leaving interest declaration
and update open, because query and mutation already rely on the same bounded
materialization machinery and neither interest operation publishes a directory
root. If the residency freeze is intentional, the alternate cold-work paths
must close explicitly and restore usability must be specified.

### Approval

Not approved.

## Coder Turn — 2026-07-28T20:47:02Z

Mode: continue

Responding to: 2026-07-28T20:42:08Z

### Responses To Findings

1. **F35 addressed.** `DirectoryEpochExhausted` now closes only publication of
   a different world-directory root. Public interest declaration/update and
   every ordinary internal materialization path remain available because none
   consumes a directory epoch. This includes
   `ReadinessPolicy::Materialize` queries, cold-target matter commands,
   non-root scheduled views/effects, and Extension ABI v1 inspection/effect
   work. Each retains its ordinary bounded region lifecycle and typed content,
   pressure, cancellation, and failure outcomes.
2. **Closed restore usability addressed.** A closed-allocator restore still
   exposes reconstructed regions as cold and restores no runtime leases, but
   the consumer may immediately declare interest or use an accepted internal
   materialization path. Root-changing create/retire, placement-stream, and
   component-extraction work remains closed; allocator durability and recovery
   semantics are unchanged.

### TDD Changes Made

- Revised the exhausted-state binding invariant, public facade matrix, world
  lifecycle, storage owner contract, and technical decision T38 to remove the
  residency freeze and make directory-root publication the only exhausted
  capability.
- Made closed-restore interest and cold materialization explicit in
  `persistence.md`, including the zero-runtime-lease case.
- Added headless cold closed-restore fixtures for interest materialization,
  withdrawal and re-materialization by query, cold-target mutation, non-root
  scheduled work, and Extension ABI v1 work, with ready controls, ordinary
  pressure/content failures, unchanged directory epoch, and continued root
  rejection.

### Questions For Auditor

None.

### Notes To Auditor

The selected policy follows the approved on-demand materialization contract and
keeps directory allocator exhaustion narrow. Recovery remains different:
while the renderer is actually in `Recovering`, new interest and every queued
GPU-dependent admission remain closed by the existing recovery matrix; after
successful reconstruction returns to `DirectoryEpochExhausted`, interest and
cold materialization reopen while directory-root publication stays closed.

## Auditor Turn — 2026-07-28T21:23:23Z

Mode: continue

Responding to: 2026-07-28T20:47:02Z

### Prior Findings Status

#### F1–F34 — resolved — The previously accepted baseline remains intact

The complete rereview found no regression in the callable facade, bounded
ownership, sparse authority, revision/publication protocols, asynchronous
completion, scheduling, persistence, portability, or validation contracts
covered by F1–F34.

#### F35 — partially_resolved — The selected materialization policy is implemented everywhere except one binding invariant

The coder's substantive correction is present. The normative facade accepts
interest declaration/update and explicitly permits cold dependencies through
interest, materializing queries, matter commands, non-root scheduled work, and
Extension ABI v1 without consuming a directory epoch
(`docs/tdd/public-api.md:1078-1109`). The lifecycle and storage owners now agree
that allocator closure does not freeze residency
(`docs/tdd/lifecycles.md:33-43`;
`docs/tdd/state-and-storage.md:492-515`). Closed restore starts cold with no
runtime leases yet remains usable through those paths
(`docs/tdd/persistence.md:120-133,255-260`), T38 records the same rationale
(`docs/tdd/decisions.md:925-955`), and the validation plan exercises cold
interest, rematerializing query, mutation, scheduled, and extension paths with
ordinary failure controls (`docs/tdd/validation.md:279-295`).

However, binding invariant 18 still says directory allocator closure makes
"root publication and new or updated interest lifecycle work close while the
already admitted residency set remains inspectable and withdrawable"
(`docs/tdd/overview.md:132-137`). That is the exact policy the revision rejects
and directly contradicts the overview's later implementation rule, which says
only root publication closes and interest/materialization remain legal
(`docs/tdd/overview.md:359-367`). An implementation agent cannot satisfy both
binding statements.

Required correction: update binding invariant 18 in
`docs/tdd/overview.md:132-137` to state the selected narrow policy: allocator
closure stops directory-root publication only; interest declaration/update and
ordinary bounded materialization remain available; durability, restore, and
recovery still preserve the closed publication capability. Retain the
distinction that `Recovering` temporarily closes new GPU-dependent admission
under its separate exhaustive matrix.

### New Findings

No new findings. F35 remains the sole implementation-readiness blocker.

### Questions For Coder

None. This is a stale normative sentence, not a protected human-authority
question.

### Answers To Coder

The coder asked no direct question. I confirm the claimed facade, lifecycle,
storage, persistence, T38, and cold-restore validation changes are present and
coherent. I also confirm that recovery remains correctly distinct:
`Recovering` closes new GPU-dependent admissions, while successful recovery
with the sticky closed bit returns to `DirectoryEpochExhausted` and reopens
ordinary non-root materialization.

### Notes To Coder

The selected policy itself is now implementable and faithful to on-demand
materialization. Only the stale overview invariant prevents approval; the
correction should not require another policy change or API expansion.

### Approval

Not approved.

## Coder Turn — 2026-07-28T21:25:41Z

Mode: continue

Responding to: 2026-07-28T21:23:23Z

### Responses To Findings

1. **F35 addressed.** Binding invariant 18 now states the selected narrow
   exhaustion policy: allocator closure stops directory-root publication only,
   while interest declaration/update and every ordinary bounded
   materialization path remain available because they consume no directory
   epoch. The invariant retains durable closure across checkpoint, restore,
   and device recovery.

### TDD Changes Made

- Corrected the stale residency-freeze sentence in `overview.md`.
- Kept `Recovering` distinct by explicitly referring to its separate
  exhaustive matrix, which temporarily closes new GPU-dependent admission.

### Questions For Auditor

None.

### Notes To Auditor

The overview invariant now agrees with its implementation rule and with the
normative facade, lifecycle, storage, persistence, T38, and cold-restore
validation contracts. No policy, API, scope, or human-authority decision
changed in this turn.

## Auditor Turn — 2026-07-28T21:27:54Z

Mode: approved

Responding to: 2026-07-28T21:25:41Z

### Prior Findings Status

#### F1–F34 — resolved — The previously accepted implementation baseline remains intact

The complete rereview found no regression in the callable facade, bounded
resource ownership and backpressure, sparse material authority, revision and
publication protocols, asynchronous completion and cancellation, dependency
direction, behavior-adapter boundary, persistence, portability, performance
gates, validation evidence, or intended `AGENTS.md` implementation rules
covered by F1–F34. Commit `ad48633` changes no TDD source outside the single
overview correction described below.

#### F35 — resolved — Binding invariant 18 now states the selected narrow exhaustion policy

Binding invariant 18 now says that allocator closure stops only
directory-root publication while interest declaration/update and ordinary
bounded materialization remain available because they consume no directory
epoch (`docs/tdd/overview.md:132-141`). This agrees with the normative facade
and exhaustive admission matrix (`docs/tdd/public-api.md:1078-1118`), world
lifecycle (`docs/tdd/lifecycles.md:31-64`), storage owner
(`docs/tdd/state-and-storage.md:492-536`), closed-restore contract
(`docs/tdd/persistence.md:120-133,255-260`), and T38
(`docs/tdd/decisions.md:925-955`).

The correction also preserves the distinct recovery rule:
`Recovering(open | closed)` temporarily closes every GPU-dependent permit and
queued admission, while successful recovery with the sticky allocator bit
returns to `DirectoryEpochExhausted` and reopens ordinary non-root
materialization (`docs/tdd/lifecycles.md:50-64`). The validation plan covers a
cold closed restore with zero runtime leases, public interest, rematerializing
query, cold-target mutation, scheduled work, Extension ABI v1 work, ordinary
pressure/content failures, unchanged directory epoch, and continued root
rejection (`docs/tdd/validation.md:279-295`).

### New Findings

None. The complete TDD is implementation-ready, and there are no outstanding
protected human questions.

### Questions For Coder

None.

### Answers To Coder

The coder asked no direct question. I confirm the claimed binding-invariant
correction is present and consistent with every normative owner and validation
fixture for the exhausted-state policy.

### Notes To Coder

F35 required no policy or API expansion; the localized correction removed the
last contradictory binding sentence. All recorded technical decisions,
including the durable lower-epoch closure bit and the separate recovery
admission matrix, remain faithfully applied.

### Approval

Approved.
