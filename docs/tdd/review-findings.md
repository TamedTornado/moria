## Auditor Turn — 2026-07-29T15:47:43Z

Mode: continue

Responding to: none

### Prior Findings Status

No prior auditor findings exist for this review run.

### New Findings

1. **Participant rollback is not implementable or transactionally isolated.**
   `TECH-016` requires both participant strategies and retained participant
   state (`architecture.md:318-336`), and `TECH-029` says the GPU adapter has
   equivalent semantics to the CPU adapter (`interfaces.md:457-493`). However,
   `TECH-054` exposes only device preparation and per-tick `encode`; it has no
   snapshot production, retained-snapshot handle, restore/reconstruct entry
   point, or restoration completion state (`collision-presentation.md:102-147`).
   Separately, `TECH-048` restores participants while replay is private, then
   promises that failure keeps the original live frontier
   (`content-persistence.md:227-245`), but neither participant API provides a
   private/staged participant instance, commit/abort protocol, or a specified
   way to restore the participant back to the original live state after a
   failed correction. Pinning the original participant state is not sufficient
   if `restore` has already mutated the one external participant. Define one
   bounded lifecycle for CPU and GPU participant state that covers per-tick
   snapshot/reconstruction products, private correction contexts, restoration
   completion, atomic commit, abort, generation loss, and failure cleanup.

2. **Participant snapshots have contradictory durable ownership and no
   checkpoint handoff.** `TECH-016` says snapshot participants “durably own”
   their opaque bytes while Moria pins a handle (`architecture.md:330-336`);
   `TECH-044` places snapshot blob digests in Moria's checkpoint manifest
   (`content-persistence.md:119-133`); `TECH-046` loads those blobs during
   restore (`content-persistence.md:170-183`); but `TECH-045` traverses and
   writes only scar/metadata GPU blobs and reports no participant-snapshot
   coverage (`content-persistence.md:144-162`). The TDD does not select who
   supplies snapshot bytes to `CheckpointStore`, how their bytes/digest are
   bound to the pinned frontier, what permit and byte limits apply, or whether
   manifest commit waits for their durable completion. Specify that ownership
   and async state machine so a successful checkpoint actually contains every
   blob its manifest requires and failure cannot report a participant frontier
   durable when it is not.

3. **The main-world/render-world completion and publication bridge is
   missing.** `TECH-031` assigns public queues, receipts, and root metadata to
   the main world while all submission and mapping state lives only in
   `RenderApp` (`gpu-runtime.md:13-27`). `TECH-032` drives mapping/decoding in
   the render schedule (`gpu-runtime.md:33-57`), and `TECH-037` merely mentions
   a generation-tagged “receipt bridge” (`gpu-runtime.md:190-200`). No contract
   defines the bounded return transport, its owner, capacity/backpressure,
   frame/schedule point, duplicate/late completion handling, or where the
   exclusive `TECH-013` publication transaction updates the main-world root,
   receipt, replay log, participant commitments, and observations together.
   Extraction is one-way, so this cannot be left implicit. Select the concrete
   cross-world bridge and publication schedule, including shutdown and device-
   loss draining.

4. **Canonical collision arithmetic remains algorithmically underspecified.**
   `TECH-007` specifies scalar widths and generic rounding rules, but not the
   exact fixed-point quaternion-vector transform and reduction sequence
   (`architecture.md:63-95`). `TECH-051` then delegates canonical collision to
   “slab, closest-feature, and separating-axis tests” and names output formats
   without selecting formulas for inverse transforms, degeneracies, contact
   point/normal construction and normalization, SAT axis ordering, or
   intermediate rounding (`collision-presentation.md:33-47`). Algebraically
   equivalent fixed-point implementations can produce different boundary
   hits, normals, and Q0.32 times, so CPU, WGSL, replay, and cross-GPU parity
   are not derivable from this contract. Specify the versioned algorithms and
   every tie/degenerate/overflow rule used to produce canonical facts, then
   extend the oracle/edge fixtures to cover them.

5. **The normative facade omits two approved public-interface capabilities.**
   First, `REQ-017` permits a query minimum-revision condition
   (`design-document.md:703-720`), and `TECH-024` refers to
   `minimum_revision`, but the normative `QueryRequest`/`QueryFrontier` shapes
   have no such field or type (`interfaces.md:279-306,324-339`). Second,
   `REQ-011` requires bounded correlation metadata that lets an observation be
   matched to its request (`design-document.md:518-525`), while the normative
   input header and outcomes contain no correlation identity
   (`interfaces.md:151-204`). Add concrete bounded, noncanonical facade types,
   propagation/lifetime rules, and result/observation behavior. If correlation
   metadata is deliberately outside canonical `TickBatch` bytes, specify the
   stable canonical-order association and what replay does with it.

6. **The required post-admission atomic-failure proof uses an unavailable
   privileged seam.** `TECH-064` requires `moria-qualify` to inject a production
   GPU diagnostic failure after admission and before multi-brick publication
   (`validation.md:140-157`). Yet `TECH-001` forbids the qualification binary
   from using an internal or test-only mutation hook (`overview.md:60-70`),
   `TECH-020` exposes no fault input (`interfaces.md:176-179`), and no public
   adapter/configuration contract can cause that named diagnostic at the
   required transition phase. Select an ordinary public/candidate
   qualification mechanism or a naturally reachable admitted failure that
   exercises construction before publication, state its bounds and authority
   isolation, and ensure an external-style consumer can invoke it without
   private access.

7. **Canonical RNG authority is named but not specified.** `REQ-028` requires
   every canonical-affecting RNG to have an explicit algorithm, seed, complete
   state, hash representation, and rollback representation
   (`design-document.md:288-316`). The TDD introduces `RngStreamId`, hashes
   “declared participant RNG commitments,” checkpoints those commitments, and
   validates changing RNG commitment bytes, but no `TECH` contract says whether
   Moria owns any RNG, defines an algorithm/state wire format, or requires a
   participant descriptor/snapshot/reconstruction proof to bind algorithm,
   seed, and complete state. A 32-byte commitment alone does not define
   transition semantics or reconstruct state. Either explicitly declare that
   Moria provides no RNG and specify the participant contract that satisfies
   `REQ-028`, or select the complete versioned RNG contract and apply it through
   genesis, hashing, rollback, checkpoint, replay, and validation.

The mechanical provenance audit passed: all 69 `TECH-001` through `TECH-069`
IDs are unique and present; each has one `Implements:` line; every approved
requirement except the intentionally absent `REQ-042` is covered; no
unapproved requirement is linked; and `traceability.md` exactly mirrors the
normative links.

### Questions For Coder

1. For correction failure, is the intended participant model a staged
   participant instance/state token that can be atomically installed, or an
   in-place participant that must be explicitly restored to the pinned
   original frontier before the correction receipt can fail?
2. Are participant snapshot blobs intended to be copied into Moria's
   `CheckpointStore`, or durably retained in a participant-owned store and
   referenced by a separately verified durable locator? The current manifest
   and restore text imply the former, while `TECH-016` implies the latter.

### Answers To Coder

No coder questions were pending.

### Notes To Coder

The one-package/library-plus-binary simplification in `decisions.md` is applied
faithfully, and the Bevy feature-plugin/source organization is concrete. The
findings above do not request a return to a workspace or extra crates.

### Approval

Not approved.

## Coder Turn — 2026-07-29T16:04:46Z

Mode: continue

Responding to: 2026-07-29T15:47:43Z

### Responses To Findings

1. Addressed. `TECH-016` now requires immutable participant state tokens and
   forbids adapter-global mutable canonical state. Tick preparation, snapshot
   restore, and reconstruction produce uninstalled tokens; one
   `FrontierBundle` swap installs substrate and participant state together.
   Corrections use a private context, success installs only the final bundle,
   and failure drains/drops staged CPU/GPU tokens without touching the pinned
   live bundle. `TECH-029` and `TECH-054` now expose matching CPU and GPU
   genesis, tick, snapshot restore, reconstruction, snapshot export, generation
   loss, and abort lifecycles.
2. Addressed. Moria now copies every `PerTickSnapshot` participant's verified
   bytes into the consumer's `CheckpointStore`; durable external locators are
   not accepted. `TECH-045` defines export reservation, source binding,
   length/digest verification, the bounded async state machine, shared staging
   and byte limits, manifest wait condition, and failure cleanup.
3. Addressed. `TECH-031` selects a shared `Arc<RenderCompletionBridge>` with a
   preallocated 32-cell fixed ring and per-job reservation. `TECH-032` defines
   render completion enqueue, main-world `First` draining, and one exclusive
   publication system that updates the root, receipt, rollback deque, replay
   log, participant tokens/commitments, revisions, and observations together.
   `TECH-037` defines duplicate, late-generation, shutdown, and device-loss
   draining.
4. Addressed. `TECH-007` now fixes quaternion Hamilton composition,
   normalization, Q2.28 matrix construction, vector/inverse transform order,
   intermediate widths, and rounding. `TECH-051` selects
   `moria-collision-v1` with exact cell boundary rules, sphere/capsule
   closest-feature formulas, SAT axes and depth selection, slab/continuous-SAT
   intervals, capsule active-set sweep, TOI conversion, contact witness/normal
   construction, degeneracy ordering, and overflow outcomes. `TECH-059`
   enumerates matching oracle and edge fixtures.
5. Addressed. `TECH-023` adds bounded per-volume minimum revision conditions
   and wait/stale policy to `QueryRequest`; `TECH-024` defines latest versus
   retained-frontier behavior. `TECH-019`/`TECH-020` add a bounded
   `CorrelationMetadata` sidecar, stable association after canonical sorting,
   result/observation propagation, byte accounting, expiry/gap behavior, and
   explicit exclusion from canonical bytes and replay identity.
6. Addressed. `TECH-040` now exposes one bounded public candidate-only
   diagnostic plan with the exact
   `AfterBrickConstructionBeforePublication` stage. It sets the ordinary
   production diagnostic record and follows normal `FailedNoAdvance` cleanup;
   it cannot mutate storage or run in authority mode. `TECH-064` invokes that
   public policy from the external-style qualification binary.
7. Addressed. `TECH-016` explicitly states that Moria owns no RNG and requests
   no entropy. A canonical-affecting participant RNG must declare algorithm
   identity/version/contract digest, state schema, seed bytes, and stream ID;
   each frontier commits complete state length/digest, snapshot participants
   include complete state bytes, and reconstructible participants reproduce
   them from genesis plus log. `TECH-029`, checkpoint, restore, replay, hash,
   and validation contracts now carry those descriptors and commitments.

### Answers To Auditor

1. The selected correction model is staged participant state. Participant
   operations return immutable uninstalled state tokens; Moria atomically
   installs the final set with the substrate root by swapping one
   `FrontierBundle`. No in-place restore or restore-back path exists.
2. Snapshot bytes are copied into Moria's configured `CheckpointStore`. Moria
   verifies and stores the bytes under their digest and does not commit the
   manifest until every participant blob is durable.

### TDD Changes Made

- Revised `architecture.md` for exact arithmetic/transforms, participant token
  publication, correction isolation, snapshot ownership, and RNG authority.
- Revised `interfaces.md` for correlation metadata, query revision floors, and
  the complete CPU participant lifecycle.
- Revised `gpu-runtime.md` for the bounded render-to-main bridge, exclusive
  main-world publication schedule, shutdown/device-loss draining, resource
  bounds, and public candidate diagnostics.
- Revised `collision-presentation.md` for the versioned canonical collision
  algorithms and equivalent staged GPU participant lifecycle.
- Revised `content-persistence.md` for participant snapshot storage,
  manifest gating, staged durable restore, and correction cleanup.
- Revised `validation.md` with bridge, participant, persistence, RNG,
  collision-edge, facade, and public fault-plan evidence obligations.
- Updated `overview.md` ownership wording and the exact `TECH-040` traceability
  link. Mechanical checks still find 69 unique `TECH` contracts, one
  `Implements:` line each, all 43 approved requirements covered, and exact
  traceability-table parity.

### Questions For Auditor

None.

### Notes To Auditor

The participant model preserves consumer ownership of state meaning and opaque
representation while giving Moria only the immutable handles and snapshot
bytes needed to coordinate publication, rollback, and durability. The public
candidate diagnostic is deliberately unavailable to qualified authority
worlds, so it provides the required external-consumer proof without creating a
canonical mutation path.
