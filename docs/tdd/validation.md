# Validation and evidence plan

Validation uses the same public facade as external consumers and layers
evidence so a host mock, compiled shader, rendered frame, or self-reported pass
cannot stand in for canonical GPU truth.

## Test architecture

### TECH-059 — CPU oracle and property tests

Implements: REQ-007, REQ-021, REQ-023, REQ-036, REQ-038

`moria-qualify` contains an independent, deliberately small CPU reference
model for canonical encoding, fixed-point arithmetic, cell edits, sorted tick
outcomes, collision primitives, radix commitments, and replay hashes. It shares
public wire definitions but no transition/storage implementation with Moria.
It is evidence only and cannot publish an authority world.

Required deterministic tests include:

- golden bytes and digests for every canonical record and hash domain;
- arithmetic edges for every operation, overflow, division, shift, ties-even
  rounding, quaternion normalization/composition/inverse, and the maximum-
  radius displacement proof;
- generated edit state machines with create/retire/move/erase/place/patch,
  overlaps, stale preconditions, revision exhaustion, and failed atomic
  commands;
- generated sparse maps compared after every edit, compaction, checkpoint
  round trip, and rollback;
- hash sensitivity for matter, placement, IDs/allocator, simulation domain,
  participant/RNG commitment, and insensitivity to derived cache mutations;
- collision properties: no fact targets empty matter, all returned facts carry
  the source revision, stable tie order, and CPU/GPU parity;
- decoder fuzzing for truncation, trailing bytes, invalid tags/counts, zip
  bombs, corrupt hashes, and allocation overflow.

Schedule-perturbation tests submit identical sealed bytes under randomized
producer threads, insertion orders, worker counts, completion notifications,
cache layouts, and physical-slot histories. Canonical outcomes/bytes/hashes
must remain identical.

### TECH-060 — Headless Bevy contract tests

Implements: REQ-002, REQ-005, REQ-012, REQ-015, REQ-016, REQ-023

Facade, admission, receipts, observations, lifecycle, shutdown, and failure
state machines are tested in small Bevy `App`s with `MinimalPlugins` and
deliberately selected Moria plugins. Tests seed only required resources, call
`app.update()` explicitly, and use controlled completion sinks; they never
open a window or depend on wall time.

Required slices cover every legal transition and invalid transition rejection,
permit drop/queue close, receipt drop after submission, cancellation lifetime,
gap and resnapshot, interest withdrawal with pins, checkpoint failure,
participant duplicate/late completion, shutdown with dirty truth, and a missing
`RenderApp`. The missing-renderer case reports `BackendUnavailable`; it is not
a GPU test and does not install a no-op canonical backend.

`moria-qualify` compiles as a separate binary crate and imports only public
`moria`. A lint/test fails if it enables a test-only facade feature or imports a
private module.

## Shader and real-GPU validation

### TECH-061 — Shader validation pipeline

Implements: REQ-007, REQ-021, REQ-023, REQ-036, REQ-039

`moria-qualify shaders validate` discovers every WGSL module referenced by the
crate, parses and validates it with the exact matching Naga version and
declared capabilities, verifies reflected bindings against ABI tables, and
runs negative fixtures at the expected layer.

Negative fixtures cover malformed/semantically invalid WGSL, mismatched
bindings, undersized effective ranges, invalid workgroup dimensions/storage,
nonuniform barriers, over-dispatch without bounds guard, scan/output/counter
overflow, invalid indirect offset/range/dimensions, unsupported optional
feature, map/decode length/alignment errors, duplicate sparse reservations,
failure after reservation, tombstone reachability, and stale generations.
Each passes only if its named layer returns its named error; crash, timeout,
unrelated error, or silent no-op fails.

Every pushed error scope is popped and its result recorded. Unexpected
uncaptured validation errors fail the run.

### TECH-062 — Real-GPU semantic parity

Implements: REQ-001, REQ-023, REQ-036, REQ-038, REQ-039

The real-GPU suite exercises public genesis, queues, participant adapter,
canonical transition, collision, readback, checkpoint, and replay. For each
fixture it initializes every byte, dispatches through the normal Bevy device,
copies ordinary output to the production staging path, maps/decodes, and
compares exact bytes and integer values with the CPU oracle.

Required sizes include empty, one item, partial tile, exact tile, multi-tile,
multi-level scan, maximum command capacity, and overflow. Sparse cases include
empty/near-full/full resident directories, collision-heavy keys, 32-probe
exhaustion, duplicate contenders, tombstone-heavy caches, allocation-counter
edges, failed construction, multi-brick atomic failure, old-root readers during
publication, delayed reclaim, and compaction preserving every live mapping.

The suite intentionally destroys/loses a test device where the backend harness
supports it. Outstanding work must fail, old callbacks must not publish,
device-bound state must reconstruct or reach the documented terminal failure,
and root semantics after successful replay must match.

Evidence records input/output digests and decoded comparisons. “Shader
compiled,” “queue submitted,” “frame rendered,” a mock, or a software adapter
cannot satisfy this contract.

### TECH-063 — Cross-backend qualification matrix

Implements: REQ-026, REQ-039, REQ-043

`moria-qualify qualify` requires physical runs for at least one declared tuple
in each claimed family: Metal, Vulkan, and DX12. Every retained row contains:

```text
fixture/contract/source digests; git commit; dirty flag;
OS/kernel; CPU; GPU vendor/device; driver; backend;
Bevy/wgpu/Naga/Rust versions; granted features/limits/downlevel flags;
fallback status; start/end UTC as evidence metadata;
canonical genesis/tick/outcome bytes and per-tick hashes;
readback digests; unexpected errors; PASS/FAIL/UNAVAILABLE
```

The retained conformance fixture is byte-identical across rows. An independent
`compare` subcommand reads artifacts and compares canonical records/hashes at
every tick; it does not trust row status. Any divergence marks the tuple
unqualified and emits TECH-047's earliest-divergence artifact.

`UNAVAILABLE` remains missing evidence. Local replay on one tuple qualifies
only local replay. Driver or relevant contract changes expire the row. A
qualification manifest references immutable evidence digests and is accepted
by authority mode only when every requested tuple row is current.

## Scenario acceptance

### TECH-064 — Public-boundary and truth scenarios

Implements: REQ-002, REQ-003, REQ-010, REQ-011, REQ-013, REQ-019, REQ-020, REQ-023, REQ-044

The permanent scenario suite uses consumer-supplied fixtures and the public
facade:

1. **Public boundary:** configure, verify genesis, interest, sample/region/
   trace/overlap/sweep, mutate, observe, checkpoint, restore, and shut down.
2. **Deep volume:** a static volume has voids, signed-density boundaries,
   material bands, and authored structures across all axes. Queries and edits
   operate at the deepest bounds; no height field exists in the fixture.
3. **Dynamic volume:** query and collide, rotate/translate by an admitted
   placement input, edit local matter, checkpoint, and restore the same ID.
4. **Atomic mutation:** inject a post-admission diagnostic failure into a
   multi-brick command before publication. Every targeted old cell/revision/
   observation remains unchanged. The failure seam is the same checked
   diagnostic flag used for production GPU failure, not a mutation bypass.
5. **Truth versus view:** remove presentation, make it stale, corrupt/discard
   derived buffers, and rebuild. Matter/collision/hash bytes remain identical;
   smooth and crisp surfaces show honest exposed cuts when current.
6. **Assembly/dressing:** an ordinary small volume supplies material-backed
   occupancy while dressing loses support and disappears without collision or
   persistence residue.

Visual captures are diagnostic evidence for smooth/crisp presentation and
dressing support. Humans review them, but exact pixels are not canonical and
cannot replace state/readback assertions.

### TECH-065 — Lifecycle, failure, and isolation scenarios

Implements: REQ-009, REQ-012, REQ-016, REQ-018, REQ-021, REQ-022, REQ-040

One machine-readable failure matrix injects every product-design condition:
cold query, invalid bounds, stale precondition, source unavailable/invalid,
closed tick, arithmetic/range exhaustion, canonical/query/pool pressure,
presentation failure, observation overwrite, store failure, corrupt/incomplete
checkpoint, wrong lineage/root, missing material/source, activation mismatch,
participant restore/divergence, rollback outside window, replay poison,
unqualified tuple, qualified divergence, and external participant failure.

Each row states expected layer/code, retryability, receipt terminal state,
committed tick/revision change, observation, and retained dirty state. The test
asserts unknown never becomes empty, no scar disappears, no internal storage
handle escapes, and no timing-selected tick confirms.

I/O, materialization, callback, and presentation schedules are permuted while
the same activation input bytes run. Simulation-domain normalized bytes and
hashes must match. Overlapping activity regions produce one union. Camera and
presentation interest changes alone produce no canonical byte change.

### TECH-066 — Persistence, snapshot, and participant scenarios

Implements: REQ-014, REQ-029, REQ-030, REQ-032, REQ-035, REQ-037, REQ-038

Persistence tests checkpoint edited static and edited/moved dynamic volumes
while a later tick commits. The checkpoint reports only its pinned frontier;
the later root remains dirty. Saved data contains scars/continuation but no
mesh and is materially smaller than a raw dump of the fixture's untouched
domain.

Restore compares canonical logical state, cell queries, placements, IDs,
allocator, simulation domain, participant commitments, and root hash. It then
rebuilds GPU/cache/presentation state and compares observable behavior. Wrong
lineage with the right label but wrong exact root must fail.

Rollback retains 32 ticks (therefore at least 20) while overlapping/disjoint
bricks, placements, IDs, participant RNG commitment bytes, and domain
membership change. It restores ticks 1, 5, 10, 20, and 32 without enumerating
untouched matter, then replays to identical per-tick outcomes/hashes. One
snapshot participant and one reconstructible participant exercise success,
capacity exhaustion, missing state, and divergence. Pin/reclaim counters prove
state is live exactly while reachable.

Poisoning each canonical influence category changes its appropriate leaf/root
at the first affected tick. Poisoning mesh/cache/telemetry does not.

## Performance and evidence

### TECH-067 — Fixed rollback performance fixture

Implements: REQ-007, REQ-022, REQ-041

Reference fixture `rollback-chain-v1` is frozen before measurements:

- 8 static sparse volumes with a combined 65,536 resident 8³ bricks;
- 64 dynamic volumes, each 8 bricks, with independent placements;
- 2 external participants: one GPU `PerTickSnapshot` participant with a
  2 MiB maximum snapshot and one
  `ReconstructibleFromCanonicalStateAndLog` participant with 32-tick capacity;
- a consumer-defined 64-link interaction dependency chain represented only in
  participant input/state (Moria contains no physics/constraint meaning);
- 256 changed bricks and at most 8,192 changed cells per tick, half overlapping
  prior-tick dirty regions;
- activation/deactivation of 32 brick regions and 16 placement changes per
  tick;
- rollback depths 1, 5, 10, 20, and 32;
- simulation-frame interval 16.667 ms.

After 30 warm-up corrections, each depth runs 120 corrections. Evidence reports
median, p95, maximum, CPU wall time, GPU timestamps when supported, mapping
cost, participant time, restored/replayed ticks, bytes retained/changed/hashed,
and complete restore-through-final-replay time. The **20-tick performance
tier** requires p95 complete correction at depth 20 no greater than 16.667 ms
on the named tuple. Correct slower tuples report their measured rollback-per-
16.667-ms curve and do not claim the tier.

GPU timestamps are measurement only, never synchronization. Runs without them
retain CPU wall-time evidence and mark GPU timing unavailable. Correctness must
pass before performance supports a claim.

### TECH-068 — Quality benchmark receipts

Implements: REQ-007, REQ-022, REQ-023, REQ-041

`moria-qualify benchmark` fixes fixture digest, budgets, warm-up, sample count,
interest route, mutation density, and adapter context before running. Separate
receipts report:

- mutation admission-to-confirm and confirm-to-current-presentation;
- point/region/trace/overlap/sweep response and bytes;
- lifecycle transition latency and residency high-water marks;
- authoritative, rollback, scratch, readback, and derived GPU bytes;
- checkpoint throughput/size and restore time;
- collision oracle agreement;
- CPU/GPU participant handoff, transfer/readback pressure, and commitment cost;
- incremental leaves/nodes/bytes hashed;
- replay ticks/second and correction-depth curve.

No universal latency is claimed beyond TECH-067's named tier. Each benchmark
has `PASS`, `FAIL`, or `NOT_DEMONSTRATED`; exceeding a configured resource
budget is always `FAIL`. Reports from differing machines remain comparable
because raw configuration/context and distributions are retained, not only a
summary score.

### TECH-069 — Evidence schema and completion gate

Implements: REQ-021, REQ-022, REQ-023, REQ-026, REQ-039, REQ-044

Evidence lives under a caller-selected directory and uses
`moria-evidence-v1` JSON manifests referencing immutable binary blobs by
BLAKE3. A run manifest includes command line, fixture/contract/source/commit
digests, dirty-worktree status, environment, adapter, limits, results,
measurements, expected/actual errors, artifact paths, and missing claims.
Canonical byte blobs use their native versioned formats rather than JSON
numbers.

The implementation-ready completion gate is:

1. the exact local commands in TECH-004 pass;
2. all CPU/headless/public-boundary/failure/persistence/rollback tests pass;
3. every WGSL module and negative fixture passes at its named layer;
4. real-GPU parity passes on each tuple being claimed;
5. cross-backend comparison passes for every claimed Metal/Vulkan/DX12 row;
6. device-loss behavior is evidenced as reconstructed or terminal;
7. visual fixtures are captured and human-reviewed only for presentation
   claims;
8. benchmark receipts report budgets and honest status;
9. no missing row is interpreted as pass.

Release automation must not emit an authority qualification manifest if the
worktree is dirty, contract/source digests differ, an unexpected GPU error
occurred, a required row is unavailable, or byte comparison diverges.
