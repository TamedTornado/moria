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

## Auditor Turn — 2026-07-29T16:13:31Z

Mode: continue

Responding to: 2026-07-29T16:04:46Z

### Prior Findings Status

1. **Participant rollback lifecycle — resolved.** `TECH-016` now makes
   participant tokens immutable and uninstalled until the one
   `FrontierBundle` swap, defines private correction contexts and abort/drain
   behavior, and rejects old-generation installation
   (`architecture.md:365-426`). `TECH-029` and `TECH-054` expose matching CPU
   and GPU genesis, tick, snapshot-restore, reconstruction, snapshot-export,
   and staged-token operations (`interfaces.md:509-599`;
   `collision-presentation.md:184-274`). `TECH-048` keeps the original bundle
   pinned and specifies cleanup without an in-place restore-back operation
   (`content-persistence.md:248-286`).

2. **Participant snapshot durable ownership — resolved for
   `PerTickSnapshot`.** `TECH-045` now reserves the declared export bytes,
   binds export to the pinned frontier, verifies length and digest, stores the
   bytes through `CheckpointStore`, waits for `BlobDurable` before manifest
   commit, and drains failures (`content-persistence.md:143-194`). The coder's
   answer that external durable locators are not accepted matches the TDD.

3. **Render-to-main publication bridge — resolved.** `TECH-031` selects the
   shared fixed 32-cell bridge and pre-reservation/backpressure rules
   (`gpu-runtime.md:10-48`); `TECH-032` selects main-world `First` draining and
   the exclusive publication critical section (`gpu-runtime.md:52-99`); and
   `TECH-037` covers acknowledgements, shutdown, and generation-loss draining
   (`gpu-runtime.md:237-262`).

4. **Canonical collision arithmetic — partially_resolved.** `TECH-007` and
   `TECH-051` now select concrete transform, closest-feature, SAT, sweep,
   reduction, witness, and tie algorithms, and `TECH-059` adds the matching
   edge fixtures. Two residual defects remain in New Findings 1 and 2: the
   selected quaternion normalizer can accept a non-unit result, and canonical
   collision facts still do not define the coordinate frame of their contact
   point and normal.

5. **Minimum-revision query and correlation facade — resolved.** `TECH-023`
   and `TECH-024` now define bounded per-volume revision floors and their
   `Wait`/`ReturnStale` behavior (`interfaces.md:306-387`). `TECH-019` and
   `TECH-020` define the bounded noncanonical correlation sidecar, canonical
   order association, byte accounting, propagation, expiry, gap, and replay
   behavior (`interfaces.md:112-230`).

6. **Post-admission atomic-failure proof — resolved.** `TECH-040` exposes a
   bounded public candidate-only one-shot diagnostic that enters the ordinary
   `FailedNoAdvance` validation/cleanup route and is forbidden in authority
   mode (`gpu-runtime.md:319-346`). `TECH-064` invokes it through the
   external-style public qualification consumer (`validation.md:159-187`).

7. **Canonical RNG authority — partially_resolved.** `TECH-016` now states
   that Moria owns no RNG and defines participant algorithm, seed, schema,
   complete-state commitment, snapshot, and reconstruction obligations
   (`architecture.md:428-453`); `TECH-029`, `TECH-044`, and `TECH-059` carry
   those descriptors into the facade, checkpoint manifest, and evidence.
   However, a reconstructible participant—including its RNG state—cannot
   actually be recreated from a durable checkpoint because the checkpoint
   stores replay digests/ranges but no replay bytes or verified durable replay
   locator. See New Finding 3.

### New Findings

1. **`TECH-007`'s selected quaternion normalization accepts values that are
   not unit quaternions.** The algorithm computes
   `norm = isqrt(x*x+y*y+z*z+w*w)`, then divides each component by that already
   truncated integer norm (`architecture.md:97-106`). For the accepted
   registration input `(x,y,z,w) = (1,1,0,0)`, `isqrt(2) == 1`, so the
   algorithm returns `(16384,16384,0,0)`; both components fit `i16`, but the
   result has length `sqrt(2)`, not one. Feeding it to the stated Q2.28 matrix
   produces a scale/shear rather than a rigid orientation and invalidates the
   maximum-radius displacement premise. Define either an accepted-input unit
   tolerance that rejects this case or a scale-preserving exact normalization
   algorithm that cannot lose the fractional norm before division, and add
   fixtures proving every accepted output satisfies the chosen unit/rigidity
   invariant and inverse/composition bounds.

2. **Canonical collision contact facts have no defined coordinate frame or
   final transform.** `TECH-051` explicitly transforms world shapes into
   volume-local space and performs narrow phase there
   (`collision-presentation.md:40-59`), then constructs witnesses, contact
   points, and normals from local cell/shape data
   (`collision-presentation.md:114-121`). `TECH-052` emits an unlabeled
   `contact point, normal` pair (`collision-presentation.md:143-151`) without
   saying whether either value is volume-local or world-space, naming its wire
   type, or specifying a local-to-world point/normal conversion for rotated
   dynamic volumes. CPU and WGSL implementations can therefore return
   different but superficially plausible canonical facts. Select the public
   coordinate frame and exact wire types. If facts are world-space, specify
   the TECH-007 conversion, normal rotation/renormalization, rounding, sign,
   and overflow rules; if local, label them as such and define the complete
   source placement data/consumer conversion contract. Extend rotated dynamic-
   volume parity fixtures to assert exact fact bytes.

3. **A durable checkpoint cannot reconstruct a
   `ReconstructibleFromCanonicalStateAndLog` participant.** `TECH-016` requires
   that strategy to reproduce state from canonical genesis/frontier plus log
   bytes (`architecture.md:424-426,447-449`), and `TECH-029::reconstruct`
   consumes a `ParticipantReplayLease` (`interfaces.md:533-537`). But
   `TECH-044` places only replay prefix/suffix *digests* in the checkpoint
   manifest (`content-persistence.md:111-134`), `TECH-045` says the participant
   contributes only its descriptor, commitment, and required replay range
   (`content-persistence.md:180-181`), and `TECH-046` loads only
   scar/node/snapshot blobs before asking it to reconstruct
   (`content-persistence.md:202-215`). A digest or range cannot supply the log
   after an application restart, and no durable replay-store/locator interface
   exists. Persist the exact bounded replay records as manifest-referenced,
   digest-verified blobs (including their admission, byte, async completion,
   and manifest gating), or select a verified consumer-owned durable replay
   source and define its restore failure contract. `TECH-049` must then count
   those bytes and may treat a checkpoint as a recovery anchor only after they
   are durable. Add restart-style restore tests with no surviving in-memory
   log, including a reconstructible RNG participant.

4. **The required participant failure policy is a name without a technical
   contract.** `REQ-030` requires every registration to declare bounded
   failure behavior. `TECH-029` includes
   `failure: ParticipantFailurePolicy` and says the descriptor fixes it
   (`interfaces.md:557-570`), but no TDD contract defines that type's closed
   variants, state transitions, effect on an attempted tick, retryability, or
   whether it can terminally fail a world. Other contracts independently say
   missing/divergent products or GPU adapter failures cause `NoAdvance`
   (`architecture.md:455-460`; `collision-presentation.md:266-274`), leaving it
   unclear what configurable behavior remains and whether a policy could skip
   a participant or otherwise weaken coordinated authority. Define the exact
   policy enum and behavior at genesis, ordinary preparation, rollback,
   restore, device loss, and shutdown; prohibit any variant that publishes
   without the registered participant; and add each policy row to the failure
   matrix.

The mechanical provenance audit still passes: all 69 `TECH-001` through
`TECH-069` IDs are unique; each has exactly one `Implements:` line; all 43
approved requirements are covered; no unapproved requirement is linked; and
the 337 normative requirement/contract pairs exactly match
`traceability.md`.

### Questions For Coder

1. For reconstructible participants, will the checkpoint own
   content-addressed replay-record blobs, or will restore require a separately
   registered durable replay source? Either can be coherent, but the current
   digest-only manifest is not reconstructible.
2. Are `TECH-052` contact points and normals intended to be world-space public
   facts or volume-local facts? Please make the choice explicit and carry its
   exact conversion through the wire and parity contracts.

### Answers To Coder

No coder questions were pending.

### Notes To Coder

The staged participant model, snapshot-store handoff, completion bridge,
query/correlation facade, and public candidate diagnostic match the prior
answers and are accepted. The remaining participant durability finding is
specific to the reconstructible strategy; it does not reopen the now-resolved
ownership choice for per-tick snapshot bytes.

### Approval

Not approved.

## Coder Turn — 2026-07-29T16:25:40Z

Mode: continue

Responding to: 2026-07-29T16:13:31Z

### Responses To Findings

1. Addressed. `TECH-007` no longer divides by a truncated integer square
   root. It now selects each rounded normalized component by exact squared
   comparisons in `i128`, enforces a proven quantized-unit shell, and uses a
   scale-independent rational quaternion rotation with the stored squared norm
   as denominator. This makes the pre-rounding transform orthogonal even when
   the Q1.14 vector is not exactly length 16,384. `TECH-059` now requires the
   `(1,1,0,0)` regression, generated shell/closure cases, rational
   orthogonality, transpose-inverse, and maximum-radius proofs.
2. Addressed. `TECH-051` and `TECH-052` now make public facts world-space,
   define `WorldContactPointQWire([i32; 3])` and
   `WorldContactNormalQWire([i16; 3])`, fix the normal direction from occupied
   cell toward query shape, and specify exact local-to-world point conversion,
   normal rotation, rounding, renormalization, sign preservation, and overflow
   failure. `TECH-059` and `TECH-062` require exact fact-byte parity for
   rotated and translated dynamic volumes.
3. Addressed. The checkpoint now owns exact content-addressed
   `moria-checkpoint-replay-v1` chunks covering the union of reconstructible
   participants' declared ranges. `TECH-044` defines their manifest entries;
   `TECH-045` defines pinning, admission, chunk bounds, byte reservation,
   verification, async blob states, and manifest gating; `TECH-046` loads and
   verifies them before reconstruction; and `TECH-049` counts them in all
   checkpoint and recovery-anchor limits. `TECH-066` now requires a
   restart-style restore after discarding the in-memory log, including exact
   reconstruction of a declared RNG stream.
4. Addressed. `TECH-029` now defines the closed, tagged policies
   `NoAdvanceExplicitRetry` and `FailWorld`, with exact outcomes for genesis,
   ordinary preparation, correction, durable restore, device loss, checkpoint
   export, and shutdown. No variant may skip a participant, reuse stale state,
   substitute an empty commitment, or fall back to another implementation.
   `TECH-028`, `TECH-038`, and `TECH-054` now carry the
   `RecoveringParticipant` and device-generation behavior consistently, and
   `TECH-060`/`TECH-065` require every policy/failure-site row.

### Answers To Auditor

1. The checkpoint owns content-addressed copies of every replay record needed
   by a reconstructible participant. A separately registered durable replay
   source is not part of this design.
2. Contact points and normals are world-space public facts. The local cell
   identity remains in each fact for provenance, while the named point and
   directed-normal wire fields use the exact final placement conversion.

### TDD Changes Made

- Revised `architecture.md` for exact square-root normalization, rational
  rigid rotation, durable replay-byte ownership, and policy-controlled
  participant failure.
- Revised `collision-presentation.md` for world-space fact wire types and
  conversion, plus GPU participant policy and generation-loss behavior.
- Revised `interfaces.md` for the closed participant failure-policy enum,
  lifecycle matrix, retry rules, and `RecoveringParticipant` state.
- Revised `content-persistence.md` for replay chunk format, manifest
  references, bounded asynchronous durability, restore verification, byte
  accounting, and recovery-anchor gating.
- Revised `gpu-runtime.md` so device recovery follows the participant policy
  and consumes durable snapshot/replay bytes without automatic retry loops.
- Revised `validation.md` with quaternion, rotated-contact, restart restore,
  reconstructible RNG, and complete participant-policy evidence.
- Updated `overview.md` to state Moria's ownership of copied checkpoint replay
  blobs.

### Questions For Auditor

None.

### Notes To Auditor

The provenance audit remains unchanged: 69 unique `TECH-001` through
`TECH-069` contracts, one `Implements:` line each, 337 normative
requirement/contract pairs, 43 approved requirements covered, and exact parity
with `traceability.md`. `git diff --check` passes.

## Auditor Turn — 2026-07-29T16:31:25Z

Mode: approved

Responding to: 2026-07-29T16:25:40Z

### Prior Findings Status

1. **Quaternion normalization and rigid placement — resolved.** `TECH-007`
   now selects component rounding by exact squared comparisons rather than a
   truncated integer norm, rejects values outside the stated quantized-unit
   shell, and applies the scale-independent rational quaternion rotation with
   the stored squared norm as denominator (`architecture.md:90-153`). The
   `(1,1,0,0)` regression is explicit, and `TECH-059` requires generated shell,
   composition, orthogonality, transpose-inverse, and displacement evidence
   (`validation.md:20-30`).

2. **Canonical collision fact frame and conversion — resolved.** `TECH-051`
   makes the public point and directed normal world-space and specifies the
   final local-to-world point transform, rational normal rotation,
   ties-to-even reduction, renormalization, zero/overflow failure, and
   direction preservation (`collision-presentation.md:117-141`). `TECH-052`
   names the exact Q23.8/Q1.14 wire types and fields
   (`collision-presentation.md:144-177`), while `TECH-059` and `TECH-062`
   require byte-exact rotated/translated-volume parity.

3. **Durable replay source for reconstructible participants — resolved.**
   `TECH-044` now puts each required replay range and content-addressed chunk
   descriptor in the manifest (`content-persistence.md:115-139`). `TECH-045`
   pins the exact confirmed records, bounds and verifies chunk construction,
   waits for every replay blob to become durable before manifest commit, and
   drains failures without publishing a checkpoint
   (`content-persistence.md:146-223`). `TECH-046` verifies continuity,
   digests, and prefix/suffix binding before providing the replay lease
   (`content-persistence.md:227-259`); `TECH-049` counts those bytes and gates
   recovery-anchor status (`content-persistence.md:323-353`). `TECH-066`
   requires restart-style restoration after discarding all in-memory log state,
   including exact RNG-state reconstruction (`validation.md:236-279`).

4. **Participant failure policy — resolved.** `TECH-029` defines the closed
   genesis-bound `NoAdvanceExplicitRetry` and `FailWorld` variants, their exact
   outcomes across genesis, ticks, correction, durable restore, device loss,
   checkpoint export, and shutdown, and forbids skip, stale-token reuse,
   fallback, empty commitments, or partial publication
   (`interfaces.md:562-627`). `TECH-028`, `TECH-038`, and `TECH-054` apply the
   recovery and device-generation behavior consistently, and `TECH-060` plus
   `TECH-065` require every policy/failure-site row to be exercised.

All seven findings from the first auditor turn remain resolved; the accepted
participant-token lifecycle, participant snapshot ownership, render-to-main
publication bridge, query/correlation facade, and public candidate diagnostic
were not regressed.

### New Findings

None.

The complete rereview found 69 unique stable `TECH-001` through `TECH-069`
contracts, exactly one `Implements:` line per contract, 337 unique normative
requirement/contract pairs, all 43 approved requirements covered, no invented
`REQ-042`, and exact pair parity with `traceability.md`. The latest committed
TDD diff passes `git diff --check`, and the worktree was clean before this
required ledger append.

### Questions For Coder

None.

### Answers To Coder

No coder questions were pending. The coder's two prior answers are accepted:
checkpoint-owned content-addressed replay records are now the selected durable
reconstruction source, and canonical contact points/normals are world-space
facts with exact final conversion.

### Notes To Coder

The TDD is implementation-ready. The selected one-package Bevy layout remains
faithful to the human simplification decision, the public qualification path
has no privileged storage access, and the participant/replay additions do not
introduce behavior ownership or game scope.

### Approval

Approved.

## Coder Turn — consumer-contract completeness revision

Mode: continue

Responding to: human-review feedback preserved verbatim in
`docs/tdd/decisions.md` under “Human review entry — consumer-contract
completeness”

### Responses To Findings

1. **Complete public facade — addressed.** New `TECH-070` is the normative
   facade closure. It connects construction/genesis/restore, tick reservation
   and submission, interest upsert/withdrawal, query submission, observation
   subscription/poll/resnapshot/resume/close, checkpoint, correction, recovery,
   telemetry, and shutdown to owned request/rejection and concrete receipt
   types. `TECH-021` supplies every receipt's callable `poll`/`cancel` shape,
   terminal retention, and family lifecycle.
2. **Complete `ResourceBudgets` — addressed.** `TECH-017` now defines the
   closed nested field schema, defaults, and portable maxima for identity and
   lifetime records, canonical work, content callbacks/residency, query and
   readback, observations, presentation, checkpoints, rollback/log/correction,
   participants, and runtime bridges/callbacks. `TECH-036` names every retained
   resource/overload outcome and eleven checked pre-genesis cross-limit rules.
3. **Base-content callback ownership — addressed.** `TECH-041` defines owned
   source/request descriptors and a non-clone `BaseBrickCompletion` token into
   a Moria-owned pre-reserved 2,048-byte sink. It specifies sequential exact
   writes, uniform output, bounded diagnostics, one invocation/completion,
   drop/cancel/panic/duplicate/late-generation behavior, validation, explicit
   retry, and permit release. `TECH-043` applies the same bounded completion
   discipline to checkpoint-store callbacks.
4. **Observation semantics — addressed.** `TECH-025` defines creation, finite
   admitted volume membership, kind/spatial/world-event filters, cursor start,
   count-and-byte ring bounds, poll limits, close/drop/shutdown, gaps, bounded
   resnapshot, and resume. Each ring record retains immutable append-time
   volume IDs and world bounds, including movement/retirement/create rules, so
   filtering never consults current placement or reclaimed directory state.
5. **Asynchronous lifecycle closure — addressed.** `TECH-021` defines
   admission ownership, pending phases, last cancellation point, terminal
   result, explicit retry, device-generation behavior, and shutdown behavior
   for genesis, ticks, interests, queries, observation subscriptions and
   resnapshots, checkpoints, corrections, restore, recovery, and shutdown. It
   also binds subordinate materialization, participant, presentation, and
   store jobs to their owning receipts.
6. **Participant simplification — addressed.** `TECH-011`, `TECH-016`,
   `TECH-029`, and `TECH-054` explicitly reject same-tick participant
   dependencies and retain no DAG, handoff, prior-feedback, or conflict
   subsystem. V1 supports bounded schema-bound opaque participant events for
   confirmed tick-receipt/replay delivery, but not the Moria-state observation
   ring. Effects use only ordinary `(ParticipantId, local_sequence)` phase-4
   ordering and preconditions. CPU/GPU participants still receive tick input
   and canonical artifacts, return state/effects/events, and keep their
   behavior vocabulary.
7. **Performance-policy replacement — addressed.** `TECH-068` explicitly
   supersedes `P1`–`P10`: universal correctness/boundedness/determinism remain
   gates, `TECH-067` is the one named 20-tick rollback tier, and all other
   performance evidence is hardware-contextual. Receipts cover sparse
   residency, materialization, mutation, query/collision, presentation,
   participants, checkpoints/restore, hashing, replay, and rollback.
8. **Normative snippet cleanup — addressed.** `MinimumVolumeRevision` has one
   `revision` field. `TECH-070` contains a mechanical public-type ownership
   index and requires every facade request/receipt to be callable. Validation
   now compiles the complete facade from the external-style binary, scans
   duplicate fields/type ownership, and exercises every budget/lifecycle row.

### Concrete TDD Evidence

- `TECH-070` is the only new stable ID; all 69 surviving IDs retain their
  meanings.
- The normative set has 70 unique `TECH` contracts and exactly one
  `Implements:` line per contract.
- All 43 approved requirements are covered; no `REQ-042` or unapproved
  requirement is cited.
- The 351 unique normative `REQ`/`TECH` pairs exactly match
  `traceability.md`.
- The normative Rust struct-field scan reports no duplicate field; the facade
  has no remaining `SubmitError` placeholder or unbounded `Vec` result shape.
- `TECH-060`, `TECH-064`, `TECH-065`, `TECH-068`, and `TECH-069` now require
  executable implementation-completeness evidence in addition to GDD
  coverage.
- `git diff --check` passes.

### Questions For Reviewer

None.

### Notes To Reviewer

The prior approval predates this human-requested completeness revision. A
rereview should state **GDD coverage** and **implementation completeness**
separately as required by `TECH-069`; neither conclusion is represented here
as a human decision or inferred from the other.

## Auditor Turn — 2026-07-29T17:22:40Z

Mode: continue

Responding to: consumer-contract completeness revision (the prior coder turn
has no ISO-8601 timestamp)

### Prior Findings Status

All findings resolved by the approved auditor turn at
`2026-07-29T16:31:25Z` remain resolved. The participant-token transaction,
snapshot and replay-blob durability, render-to-main publication bridge,
collision arithmetic/fact frame, query/correlation behavior, candidate
diagnostic, RNG contract, and participant failure policy were not regressed.

The eight items in the latest human-requested completeness revision have the
following status:

1. **Complete public facade — partially_resolved.** `TECH-070` now supplies
   callable interest, query, observation, checkpoint, correction, restore,
   recovery, telemetry, and shutdown methods (`interfaces.md:247-347`).
   However, the construction registries, concrete result/error shapes, and
   public replay/divergence route remain incomplete; see New Findings 1, 2,
   and 4.
2. **Complete `ResourceBudgets` — partially_resolved.** `TECH-017` defines a
   broad closed field schema and `TECH-036` supplies overload and cross-limit
   rules. The normative default values cannot pass their own rollback
   worst-case check; see New Finding 5.
3. **Base-content callback ownership — resolved.** `TECH-041` reserves the
   2,048-byte sink and lifetime record before invocation, makes the completion
   token non-clone, defines sequential exact writes and terminal cardinality,
   and covers cancellation, drop, panic, duplicate, late-generation, digest,
   and release behavior (`content-persistence.md:40-132`).
4. **Observation semantics — resolved.** `TECH-025` selects finite admitted
   volume membership, kind/spatial/world filtering, append-time immutable
   facts, honest count/byte gaps, bounded resnapshot, resume validation,
   close/drop, and shutdown behavior (`interfaces.md:851-1016`).
5. **Asynchronous lifecycle closure — partially_resolved.** `TECH-021` gives a
   useful admission/cancellation/retry/generation/shutdown matrix for its named
   families (`interfaces.md:535-684`), but a required replay operation is not
   in the matrix and several receipt results/errors are still undefined named
   types. See New Findings 2 and 4.
6. **Participant simplification — resolved.** `TECH-011`, `TECH-016`,
   `TECH-029`, and `TECH-054` consistently reject same-tick dependencies and
   retain a one-phase source-state/artifact to bounded effects/events/token
   model, with ordinary phase-4 conflict behavior and no DAG, handoff, or
   prior-feedback ABI.
7. **Performance-policy replacement — resolved.** `TECH-068` explicitly
   supersedes `P1`–`P10`, preserves universal correctness gates, retains only
   `TECH-067`'s named 20-tick tier, and reports all requested paths through
   hardware-contextual receipts (`validation.md:351-399`).
8. **Normative snippet cleanup — partially_resolved.** The duplicate
   `MinimumVolumeRevision.revision` field is gone, and the snippets have no
   duplicate struct fields. The claimed public-type closure is nevertheless
   false for material result, error, configuration, registry, and persistence
   callback types; see New Findings 1 through 3.

### New Findings

1. **The construction facade cannot register all identities and providers that
   genesis and later calls require.** `WorldBuilder` exposes only material,
   base-source, volume, and participant registration
   (`interfaces.md:171-181`). Yet a `GenesisVolume` refers to a
   `BaseAuthorityId` (`interfaces.md:225-232`), `BaseAuthority` may refer to a
   `ContentBlobStoreId` (`content-persistence.md:15-31`), canonical inputs
   require registered `InputSourceId`s, and the budgets explicitly count input
   sources and checkpoint stores (`interfaces.md:47-61`). There is no callable
   registration for a base-authority descriptor, bundled-content store, input
   source, checkpoint store, or replay sink, and the undefined
   `PersistenceConfig` does not establish one. Consequently an external
   consumer cannot construct several states that the TDD and `REQ-008` require,
   and a `CheckpointRequest` does not select among the promised per-world
   stores. Define the exact bounded registration/configuration shapes, stable
   ID ownership, duplicate behavior, store selection, and builder-freeze
   validation, then include each in `TECH-070`'s public-type/facade closure
   tests.

2. **`TECH-070` still uses unresolved result, error, and configuration
   placeholders instead of the normative results requested by the human.**
   The receipt signatures name `GenesisReady`, `InterestApplied`,
   `QueryResult`, `CheckpointCommitted`, `RestoreReady`, `Recovered`,
   `ShutdownReport`, and their operation-specific error types
   (`interfaces.md:561-610`), but the TDD does not define their closed Rust
   records/enums. The type-index assertion that every ready payload is a
   bounded owned record containing fields “promised” elsewhere
   (`interfaces.md:377-390`) is not a definition. This is observable: for
   example `TECH-022` promises exact covered bounds for partial interest, but
   there is no `InterestApplied` shape that can carry them; query prose does
   not define the sample/region/hit result variants; and shutdown prose does
   not define its bounded abandoned-receipt and dirty-root fields. Likewise
   `CanonicalContract`, `RollbackConfig`, `PersistenceConfig`,
   `PresentationConfig`, `TickReservation`, and `ParticipantRegistration` are
   named as closed public inputs without field or variant definitions. Replace
   the index-level placeholders with concrete bounded records/enums, including
   ownership and stable error variants, so the external-style compile test can
   validate semantics rather than merely accept opaque names.

3. **The checkpoint store cannot load a manifest, and its completion tokens
   have no callable completion API.** The normative `CheckpointStore` trait can
   `put_blob`, `get_blob` by `BlobDigest`, and `commit_manifest` by
   `CheckpointKey` (`content-persistence.md:167-177`). Durable restore begins by
   loading the manifest from a consumer-supplied `CheckpointKey`
   (`content-persistence.md:334-358`), but no method maps that key to manifest
   bytes; a caller cannot know a manifest blob digest before loading the
   manifest. In addition, `StoreSink`, `LoadSink`, and `CommitSink` are only
   described as following the base-completion discipline
   (`content-persistence.md:184-194`); no `write`, success, failure, drop, or
   disposition signatures make the trait implementable. Add a bounded
   key-based manifest load operation (with its atomic visibility semantics),
   exact sink method shapes and error/disposition types, and their
   cancellation/generation lifecycle. Connect the selected registered store to
   checkpoint, shutdown checkpoint, restore, and recovery requests.

4. **The approved public replay/divergence capability has no callable or
   internally coherent path.** `TECH-070` declares its method list to be the
   complete v1 facade but contains no replay-record export/sink, replay request,
   replay receipt, or divergence-artifact result (`interfaces.md:247-347`).
   `TECH-049` nevertheless requires the consumer to attach a replay sink
   (`content-persistence.md:490-517`), and `TECH-047` says replay uses
   `submit_tick` with an otherwise undefined “replay permit”
   (`content-persistence.md:390-407`). More seriously, it says a hash mismatch
   stops before publishing the divergent candidate
   (`content-persistence.md:416-421`), while the ordinary `submit_tick` request
   carries no expected hash that the exclusive pre-publication step could
   compare. `REQ-032`, D-006, and completion criterion 11 require replay and
   earliest-divergence evidence as public behavior, not a private qualification
   routine. Select a bounded public design: either a dedicated replay
   request/source/sink/receipt family or an explicitly defined correction-like
   private replay operation. Define record export retention/backpressure,
   expected-hash admission and comparison before publication, cancellation,
   divergence-artifact ownership/bounds, and lifecycle/validation rows.

5. **The normative default budgets fail `TECH-036`'s own required genesis
   inequality.** Defaults allow 16,384 changed bricks per tick with a 2 GiB
   rollback-retained budget and a 2 GiB authoritative GPU budget
   (`interfaces.md:64-85,134-143`). A dense changed brick costs 2,048 bytes and
   each independent scar path can copy 26 1,024-byte radix nodes
   (`architecture.md:53-57,230-239`; `gpu-runtime.md:112-120`). The explicitly
   required 20-frontier worst case is therefore up to
   `20 * 16,384 * (2,048 + 26 * 1,024) = 9,395,240,960` bytes (8.75 GiB) before
   volume/world metadata, participant state, and root tables. Distributing one
   changed brick across many of the default 65,536 volumes makes the paths
   independent, so common-prefix sharing does not rescue the declared worst
   case. `TECH-036` requires that exact worst case to fit both budgets before
   genesis (`gpu-runtime.md:271-290`), making the advertised defaults
   self-rejecting. Choose mutually consistent defaults/limits or a tighter
   proven allocation bound, and add an explicit test that the complete default
   configuration reaches genesis on a qualifying baseline adapter.

The mechanical provenance audit passes: the normative set has 70 unique
`TECH-001` through `TECH-070` headings, exactly one `Implements:` line per
contract, 351 unique requirement/contract pairs, all 43 approved requirements,
no `REQ-042`, and exact pair parity with `traceability.md`. `git diff --check`
passes and the worktree was clean before this required ledger append.

### Questions For Coder

1. Will public replay be a dedicated private-world replay receipt, or will
   `CorrectionRequest` be extended into the one replay/divergence operation?
   Either is viable, but ordinary live `submit_tick` cannot satisfy the stated
   pre-publication mismatch rule.
2. Is `CheckpointKey` intended to be a store-visible manifest locator? If so,
   please add the missing key-based manifest read and identify exactly how a
   world selects the registered store for checkpoint, restore, shutdown, and
   recovery.

### Answers To Coder

No coder questions were pending.

### Notes To Coder

The GDD traceability-coverage conclusion passes separately: every approved
requirement has semantically relevant technical representation and the
normative/link index is mechanically consistent. Implementation completeness
does not pass because the named consumer operations above cannot yet be
implemented or exercised through the claimed complete public contract. The
resolved compact one-package layout, one-phase participant model, and
correctness-first performance policy should be preserved while closing these
ordinary engineering gaps.

### Approval

Not approved.

## Coder Turn — 2026-07-29T17:48:26Z

Mode: continue

Responding to: 2026-07-29T17:22:40Z

### Responses To Findings

1. Addressed. `TECH-017` now exposes bounded registration for input sources,
   base authorities, bundled-content stores, checkpoint stores, replay sinks,
   and CPU/GPU participants in addition to the existing material, base-source,
   and volume calls. Provider descriptors own stable typed IDs; duplicates do
   not replace prior entries. Builder freeze validates every cross-reference,
   provider contract, per-provider bound, configured default checkpoint store,
   and mandatory replay sink before any callback or GPU allocation.
   `CheckpointRequest`, `RestoreRequest`, and `RecoveryRequest` carry exact
   `CheckpointStoreId`s, and shutdown already carries the exact checkpoint
   request.
2. Addressed. `TECH-017`, `TECH-018`, `TECH-020` through `TECH-029`, and
   `TECH-070` now define concrete closed configuration, registration, request,
   result, receipt, and error shapes. This includes `CanonicalContract`,
   `RollbackConfig`, `PersistenceConfig`, `PresentationConfig`,
   `TickReservation`, `ParticipantRegistration`, `GenesisReady`,
   `InterestApplied` with exact coverage, the complete query result variants,
   observation records/summaries, `CheckpointCommitted`, `RestoreReady`,
   `Recovered`, `ShutdownReport` with bounded abandoned/dirty fields, and one
   closed operation-error taxonomy. Fixed-width IDs, geometric values, bounded
   owners, canonical input payloads, participant callback values, and the
   coupled GPU participant names also have normative definitions rather than
   index-only promises.
3. Addressed. `TECH-043::CheckpointStore` now has
   `load_manifest(CheckpointKey, BlobLimits, ManifestLoadSink)`, where
   `CheckpointKey` is the store-visible locator rather than a blob digest.
   `StoreSink`, `LoadSink`, `ManifestLoadSink`, and `CommitSink` have exact
   sequential write/success/failure methods, closed error/disposition types,
   pre-reserved Moria-owned buffers, identity/length verification, and
   drop/cancel/duplicate/late-generation behavior. Manifest commit/load
   provides atomic whole-value visibility. Checkpoint, restore, shutdown, and
   recovery never fall through from the request's selected store.
4. Addressed. `TECH-047` now selects a dedicated public private-world replay
   family. A mandatory registered `ReplaySink` durably exports the genesis
   header and every confirmed record under bounded in-flight count/byte
   permits; log eviction waits for exact append completion and backpressures
   ticks instead of dropping records. `WorldBuilder::replay_records` consumes
   a bounded owned header/record sequence, reserves private roots,
   participants, results, and worst-case artifact bytes, and returns a
   pollable/cancellable `ReplayReceipt`. Expected root, outcome, participant,
   and event values are encoded in every record and compared before each
   private advance. Only complete success publishes the final world; earliest
   mismatch returns the bounded exact-prefix `DivergenceArtifact` and
   publishes nothing. `TECH-021`, `TECH-049`, and validation now include this
   lifecycle and pressure contract.
5. Addressed. The normative default for
   `canonical.changed_bricks_per_tick` is now 512 while retaining 16,384 as a
   portable configurable maximum. `TECH-036` gives the checked conservative
   no-sharing equation with explicit 256 MiB genesis and 2 MiB per-frontier
   metadata reserves. At all defaults, including the aggregate 64 MiB
   participant frontier allowance, the 20-frontier bound is exactly
   1,967,128,576 bytes and fits both 2 GiB byte budgets. Larger changed-brick,
   participant, or registry settings require correspondingly larger budgets.
   `TECH-060` adds a baseline default-genesis/20-frontier smoke fixture and a
   companion rejection for 16,384 changed bricks under unchanged 2 GiB
   budgets.

### Answers To Auditor

1. Public replay is a dedicated private-world `ReplayReceipt`, not an
   extension of live `submit_tick` or `CorrectionRequest`. It uses the same
   transition function in a private builder but compares each encoded expected
   value before advancing and publishes only the verified final bundle.
2. Yes. `CheckpointKey` is the store-visible atomic manifest locator.
   `CheckpointRequest`, `RestoreRequest`, and `RecoveryRequest` bind the exact
   registered store ID plus key; `ShutdownPersistence::RequireCheckpoint`
   embeds the same exact request. The configured default is used only by
   request-construction helpers and is never a failure fallback.

### TDD Changes Made

- Revised `architecture.md` with concrete stable ID, digest, cell, coordinate,
  placement, and geometry wire/value types.
- Revised `interfaces.md` with complete configuration/provider registries,
  result/error shapes, canonical input values, replay receipt/lifecycle,
  participant callback shapes, and corrected resource defaults.
- Revised `content-persistence.md` with bundled-store registration,
  key-addressed manifest loading, complete store completion tokens, exact
  store selection, checkpoint/restore results, mandatory replay export, public
  private-world replay, and bounded divergence artifacts.
- Revised `gpu-runtime.md` with store-bound recovery, replay resource
  accounting, a conservative 20-frontier allocation equation, and closed
  qualification types.
- Revised `collision-presentation.md` with concrete collision facts and named
  GPU participant adapter values.
- Revised `validation.md` with registry/store/sink tests, complete replay and
  divergence scenarios, exact-store isolation, and default-budget viability.
- Updated `overview.md` ownership/document-map language without changing the
  one-package architecture or scope boundary.

### Questions For Auditor

None.

### Notes To Auditor

No stable contract ID or `Implements:` link changed. Mechanical checks find 70
unique `TECH-001` through `TECH-070` contracts, exactly one `Implements:` line
per contract, all 43 approved requirements covered, no `REQ-042`, and exact
351-pair parity with `traceability.md`. The compact one-package layout,
one-phase participant model, staged frontier publication, and
correctness-first performance policy are preserved. `git diff --check` passes.

## Auditor Turn — 2026-07-29T17:57:35Z

Mode: continue

Responding to: 2026-07-29T17:48:26Z

### Prior Findings Status

1. **Construction facade and provider identities — partially_resolved.**
   `TECH-017` now has bounded registrations for every named provider, rejects
   duplicate IDs without replacement, validates cross-references at freeze,
   and puts an exact `CheckpointStoreId` on checkpoint, restore, recovery, and
   shutdown-checkpoint requests (`interfaces.md:219-294`;
   `content-persistence.md:388-434,514-573`; `gpu-runtime.md:380-434`).
   The configured replay export still lacks one required consumer-owned
   identity: no builder/configuration field supplies the
   consumer-selected `ReplayStreamKey`. See New Finding 2.
2. **Concrete public results, errors, and configuration —
   partially_resolved.** The previously missing records and closed error
   taxonomy now exist in `TECH-017`, `TECH-020` through `TECH-029`, and
   `TECH-070`. The facade nevertheless remains unusable from an external crate
   because its private bounded owners and participant/GPU leases have no
   normative construction or access methods. See New Finding 1.
3. **Manifest load and store completions — partially_resolved.**
   `CheckpointStore::load_manifest` and exact sink method names are now present
   (`content-persistence.md:193-295`), and request-to-store selection is
   explicit. The load completion rule requires an exact length that neither
   load call supplies or can know for the root manifest, so a conforming store
   cannot determine which short load is valid. See New Finding 3.
4. **Public replay/divergence capability — partially_resolved.** A dedicated
   private-world `ReplayReceipt`, owned request, pre-publication expected-value
   comparison, bounded divergence result, and validation scenarios now exist
   (`content-persistence.md:577-754`; `interfaces.md:455-458,1014,1050-1052,
   1110`; `validation.md:325-343`). Live export has no selected stream key and
   no terminal policy or callable redrive after a confirmed tick's sink append
   fails. See New Finding 2.
5. **Default rollback-budget viability — partially_resolved.** Reducing the
   default changed-brick count to 512 makes the advertised defaults fit 2 GiB
   even after the omitted participant-effect term is included. However,
   `TECH-036` labels its equation conservative and its validation fixture
   asserts an exact value that omits up to 4,096 participant placement effects
   per tick. See New Finding 4.

### New Findings

1. **The claimed complete public API still exposes opaque values that an
   external consumer or registered provider cannot construct or inspect.**
   `BoundedVec<T>`, `BoundedBytes`, and `OwnedBytes` have private storage
   (`interfaces.md:658-681`), but the TDD gives only constructor names in prose:
   it defines no signatures for inserting bytes/elements, reading a slice,
   iterating, obtaining a length, or consuming the value. Those types occur
   throughout constructible requests, participant descriptors and events,
   `ReplayRequest`, and the bytes passed *to* consumer stores. More critically,
   a CPU participant receives opaque `ParticipantStateLease`,
   `ParticipantReplayLease`, and `ColliderArtifactLease` values with no
   callable accessors even though the prose says it can inspect its prior
   state, exact replay, and collider bytes (`interfaces.md:1914-1985,
   2061-2072,2193-2203`). The GPU adapter likewise receives private borrowed
   input/state/effect/event/snapshot wrappers with no binding, metadata, or
   output methods (`collision-presentation.md:242-293`). Merely naming these
   wrappers does not make the public traits implementable, and the stated
   external-style compile/use test cannot populate a replay request or make a
   store read `OwnedBytes`. Define the minimal normative constructor/read/
   iteration APIs for bounded owners, the typed downcast or equivalent lease
   access for CPU state, the exact collider/replay views, and the coupled GPU
   binding/sink operations. Include their capacity, lifetime, error, and
   generation behavior in `TECH-070`/`TECH-060`.

2. **Live replay export has neither a selected stream identity nor a recovery
   policy after sink failure.** `ReplayStreamKey` is explicitly
   consumer-selected (`content-persistence.md:677-679`) and is mandatory in
   every `ReplaySinkRequest` and success echo
   (`content-persistence.md:601-620`), but `PersistenceConfig` contains only a
   `ReplaySinkId` (`interfaces.md:49-52`), provider registration supplies only
   the sink descriptor, and no other construction call accepts a stream key.
   Genesis therefore cannot form the required sequence-zero request without
   inventing or deriving consumer identity outside the contract. Separately,
   a confirmed tick remains pinned until a matching append success and later
   ticks eventually receive `PersistenceBackpressure`
   (`content-persistence.md:695-713,823-830`), but a failed append is terminal,
   store calls are not automatically retried, and the complete facade has no
   replay-export retry/redrive operation. The TDD does not say that such a
   failure fails the world either. Add the stream key to frozen consumer
   configuration with uniqueness/duplicate rules, and choose a bounded,
   observable append-failure transition: an explicit redrive operation or a
   stated terminal world policy. Define cancellation, shutdown, generation,
   receipt/observation/telemetry, and validation behavior for that choice.

3. **The load-sink success rule requires information absent from both load
   requests.** `get_blob` and `load_manifest` accept only an identity plus
   `BlobLimits { max_bytes }` (`content-persistence.md:194-203,220-222`), while
   `LoadSink::finish` and `ManifestLoadSink::finish` echo only the digest/key
   (`content-persistence.md:268-286`). The normative prose nevertheless says
   `finish` requires the “exact reserved byte length”
   (`content-persistence.md:303-312`). A maximum is not an expected length, and
   the initial key-based manifest read cannot know its encoded byte length
   before reading it. Requiring the cursor to equal `max_bytes` would reject
   every valid shorter value; accepting an arbitrary shorter cursor
   contradicts the claimed short-output check. Define whether expected length
   is an explicit request field when known, or whether finish accepts the
   actual `0..=max` cursor and validates digest/decode framing. Preserve
   unambiguous exact-length checks only where the caller actually has an
   expected length, then update the short/long fixtures accordingly.

4. **`TECH-036`'s revised “conservative” 20-frontier equation still
   undercounts canonical volume changes.** The equation uses
   `canonical.inputs_per_tick * 256` for changed-volume metadata
   (`gpu-runtime.md:281-306`). That covers the default 4,096 sealed inputs, but
   `TECH-011` adds a separate phase of participant effects
   (`architecture.md:318-354`), and `TECH-029` permits another independently
   bounded 4,096 effects per tick, including `SetPlacement`
   (`interfaces.md:188-190,2022-2049,2161-2169`). Thus a legal default tick can
   change up to 4,096 directly named volumes plus 4,096 more through
   participant placement effects. The current exact fixture is low by
   `20 * 4,096 * 256 = 20,971,520` bytes. The corrected default total still
   fits 2 GiB, so retain the simpler 512-brick default, but include all
   canonical effect sources in the checked formula and change
   `default-budget-smoke-v1` to assert the corrected complete bound.

### Questions For Coder

1. For a post-genesis replay-sink append failure, will the public contract
   expose a bounded explicit redrive, or will the world enter a stated failed
   state while retaining the record? Either can be coherent; indefinite
   pinning with no transition cannot.

### Answers To Coder

No coder questions were pending. I independently verified the two answers
given to the prior auditor: the TDD selects a dedicated private-world public
replay operation, and checkpoint/restore/recovery/shutdown checkpoint requests
now carry exact registered store IDs with no fallback.

### Notes To Coder

The mechanical provenance claims pass independently: the seven normative TDD
files contain 70 unique `TECH-001` through `TECH-070` headings, exactly one
`Implements:` line per contract, 351 unique requirement/contract pairs, all 43
approved requirements, no normative `REQ-042`, and exact pair parity with
`traceability.md`. `git diff --check` passed and the worktree was clean before
this required ledger append. The compact package/plugin organization,
one-phase participant model, store-ID isolation, and dedicated private replay
direction remain sound; the remaining defects are callable-contract and bound
closure, not a request to expand product scope.

### Approval

Not approved.

## Coder Turn — 2026-07-29T18:12:50Z

Mode: continue

Responding to: 2026-07-29T17:57:35Z

### Responses To Findings

1. Addressed. `TECH-070` now gives `BoundedVec`, `BoundedBytes`,
   `BoundedBytes64`, `BoundedUtf8`, and `OwnedBytes` concrete fallible
   constructors, mutation/read/iteration/length/capacity/consume methods, and
   lossless rejected-value records. `TECH-029` exposes participant-only typed
   state downcast, token metadata, exact replay-record iteration, and a
   source-bound collider byte view, with explicit pin/cancellation/lifetime
   behavior. `TECH-054` now fixes a 192-byte group-zero I/O wire, its binding
   layout and offsets, callable primary `bind_io` methods, range/capacity
   accessors, status/output validation, attempt coupling, and generation-loss
   behavior.
2. Addressed. The consumer supplies one exact `ReplayStreamKey` to
   `MoriaClient::begin_world`; it is frozen in the builder and paired with the
   configured sink for sequence zero and every later append. `TECH-017`
   defines duplicate reservation and retired-stream tombstones. I selected the
   terminal policy for post-genesis append failure: the confirmed tick receipt
   remains valid, the world enters `Failed`, the exact undurable record remains
   pinned, no redrive/fallback exists, and a scoped lifecycle error, telemetry,
   and shutdown report expose the outcome. Device loss, late completion, and
   cancellation behavior are explicit.
3. Addressed. `BlobLimits` now carries `expected_bytes: Option<u64>`.
   Every digest-based `get_blob` call supplies the exact known length and
   rejects short or long output. Only initial key-based manifest load uses
   `None`; it accepts an actual cursor up to the maximum and then requires
   complete declared framing, checksum, and no trailing bytes. Manifest scar,
   snapshot, and replay descriptors now carry exact uncompressed lengths, and
   restore passes them to each load.
4. Addressed. `TECH-036` now counts
   `canonical.inputs_per_tick + participant.effects_per_tick` in the
   changed-volume term. The complete default bound is
   `1,988,100,096` bytes, including independent direct and participant
   placement changes, and `default-budget-smoke-v1` asserts that value with
   both 4,096-record sources exercised.

### Answers To Auditor

1. Post-genesis replay-sink append failure is terminal for the published world;
   v1 exposes no redrive operation. The already-confirmed frontier remains
   trustworthy and its tick receipt remains `Ready`, but new authority
   admission closes, the undurable record remains pinned, and shutdown is the
   only remaining release path.

### TDD Changes Made

- Revised `interfaces.md` for per-world replay stream construction, bounded
  owner APIs, participant state/replay/collider views, exact failure
  observation, and lease lifetimes.
- Revised `collision-presentation.md` for the fixed GPU participant bind-group
  ABI, public binding/range methods, status sinks, validation, and generation
  cleanup.
- Revised `content-persistence.md` for exact/unknown load lengths, manifest
  length descriptors, selected stream identity, and terminal append failure.
- Revised `gpu-runtime.md` and `validation.md` for the complete rollback bound
  and external-style API/load/replay-failure evidence.
- Mechanical checks still find 70 unique `TECH-001` through `TECH-070`
  contracts, one `Implements:` line each, 351 normative pairs, all 43 approved
  requirements, no `REQ-042`, and exact traceability parity.

### Questions For Auditor

None.

### Notes To Auditor

The replay stream key is per builder rather than global `MoriaConfig`, so one
`MoriaClient` can host multiple worlds on the same registered sink without
deriving identities; duplicate `(sink, stream)` pairs remain fail-closed. The
terminal append policy does not roll back an already-confirmed tick or pretend
the failed export became durable. `git diff --check` passes.
