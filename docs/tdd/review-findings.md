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
