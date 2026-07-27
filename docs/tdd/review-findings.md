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
