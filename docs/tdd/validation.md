# Validation, evidence, and implementation gates

## 1. Validation posture

Acceptance is black-box and fail-closed. `moria-testkit` supplies content
sources, a CPU mathematical oracle, fault injection collaborators, and report
types while depending only on `moria`. It does not expose product internals.
Private unit tests supplement but never replace public-boundary scenarios.

Every scenario produces canonical JSON with:

- schema and product contract versions;
- git commit and dirty state;
- OS, architecture, Rust, Bevy, adapter, backend, driver, CPU, and GPU;
- complete `MoriaLimits` and scenario parameters;
- assertions with `passed | failed | not_demonstrated`;
- captured revision/sequence vectors and stable errors;
- peak/steady GPU authoritative and derived bytes;
- CPU staging/readback/scar bytes and queue high-water marks; and
- latency distributions with sample count, p50, p95, p99, and max.

Required evidence passes only when every required assertion says `passed`.
Missing adapters, timestamps, captures, or samples are `not_demonstrated`, not
zero or pass.

## 2. Required scenarios

### V01 — Public boundary and asynchronous lifecycle

An external-style Bevy app registers materials/volumes/sources/limits, starts,
declares/updates/withdraws interest, queries, mutates, observes, checkpoints,
restores, and shuts down using only `moria`. It asserts synchronous rejection
versus admitted terminal failure, receipt monotonicity, no hidden entities or
buffers, and finite queue telemetry.

### V02 — Deep and volume-general matter

A test source supplies non-heightmap bands, internal voids, diagonal
constructed surfaces, and a deep enclosed structure in a rotated volume.
Samples and edits at minimum/middle/maximum depth match the independent oracle.
The scene contains no assumed up axis. A dynamic volume retains ID/local cells
while moving and while overlapping a static volume; results retain both IDs.

### V03 — Sparse scale and lifecycle

A finite domain at least 1 TiB in hypothetical dense 32-bit cells is represented
mostly by homogeneous Merkle subtrees. Moving disjoint interest windows proves
that authoritative residency stays within configured bytes, clean regions
retire, cold query is unavailable rather than empty, and a scarred retired
region rematerializes correctly. Report bytes per ready/nonuniform brick and
lifecycle transitions.

### V04 — Atomic mutation

Remove, place, and patch cross material and brick boundaries. Assert one changed
command advances exactly one revision and all query/collision results switch
from old to new snapshot without a mixed set. Inject a GPU validation failure
after every staged page is written but before publication. Assert no target
changes, no revision advances, no matter observation emits, and the receipt
fails. Exercise stale preconditions and no-op commands.

### V05 — Truth versus presentation

For one captured revision, compare samples, region occupancy, ray, overlap, and
sweep with presentation current, evicted, deliberately stale, failed, and
rebuilt. Truth results must be byte-equivalent. Golden visual/geometry tests
cover organic, crisp, mixed, cut, placed, and brick seam surfaces. Dressing
loses/regenerates anchors after support changes. Mutation capture completes
only when presentation reaches the mutation revision.

### V06 — Observation gap and recovery

Use a ring smaller than the emitted event burst. Assert a gap names the last
trustworthy sequence/revisions, no later events leak before recovery, an
unrelated snapshot token is rejected, and a bounded snapshot/resume produces a
state equal to direct queries.

### V07 — Behavior extension parity

An optional neutral behavior plug-in reads a bounded GPU snapshot and proposes
a generic patch and placement. Assert it has no writable truth binding, overflow
rejects the entire proposal batch, proposals receive ordinary receipts, forced
plug-in failure changes no truth, and outcomes equal CPU-submitted commands.
Report GPU-to-GPU bytes, CPU summary/readback bytes, and proposal-to-commit
latency. A CPU-only variant proves semantics but is labeled
`gpu_path_not_demonstrated`.

### V08 — Persistence and dynamic volume

Edit a static volume; edit and move a dynamic volume; checkpoint cut R; commit
a later mutation R+1; restore R; and verify exact IDs, placements, material,
collision, and dirty coverage. Assert the checkpoint excludes R+1 and derived
bytes. Retire/rematerialize scars. Corrupt every container section, substitute
material and base roots, remove a source, return an invalid proof, and inject
sink failures before prepare/commit. All fail without partial publication or
scar loss.

### V09 — Failure and pressure matrix

Exercise invalid configuration/bounds/content, source unavailability, queue and
every named budget limit, query output overflow, indeterminate sweep,
presentation failure, device loss before publication and after confirmed
commit, receipt gap, revision exhaustion through a test-only narrow counter,
persistence failure, and shutdown with dirt. Each result's code, retryability,
revision-change flag, lifecycle, and telemetry are checked.

## 3. Test layers

- Pure unit/property tests: coordinate math, checked bounds, canonical patch
  decoding, sparse tree operations, Merkle proofs, scar coalescing, state
  machines, format parser, and error mapping.
- Shader conformance: host/WGSL sizes, alignments, offsets, limits, randomized
  oracle comparisons, staging-overflow guards, and backend compiler validation.
- Integration tests: manual-frame Bevy runtime with real headless GPU and
  injected content/sink/device collaborators.
- Black-box acceptance: V01–V09 from the validation executable.
- Golden format tests: permanent v1 checkpoints, manifests, corruption corpus,
  and endianness fixtures.
- Visual tests: deterministic mesh buffers and image captures for presentation;
  image changes require explicit review but do not supersede truth assertions.

Property tests use recorded seeds on failure. GPU comparisons use exact integer
matter values; floating contact values use documented absolute/relative
tolerances derived from cell size and run on CPU and every available tier-1
backend.

## 4. Performance and resource gates

Correctness scenarios must pass before performance is interpreted. The initial
reference gates are regression gates, not universal product promises:

- no authoritative GPU or CPU byte growth after 20 cycles returning to the
  same interest set, beyond 1% allocator slack;
- no full-volume allocation or readback in V03;
- CPU readback for GPU extension V07 is limited to result headers, receipt
  summaries, and configured scar capture, never observed input matter;
- mutation-to-confirm, query, presentation lag, and checkpoint throughput have
  stored baselines per adapter class; a greater than 15% p95 regression with at
  least 200 samples fails performance CI unless the baseline is deliberately
  revised with evidence;
- all configured pools remain at or below limits, with reserved plus used
  reconciled exactly after quiescence.

No fixed FPS, scene size, or one-machine graphics-memory target defines product
completion. Evidence always states its configuration.

## 5. Portability matrix

Required release qualification:

| Platform | Backend | Gate |
| --- | --- | --- |
| Linux x86_64 | Vulkan software adapter | all headless correctness in CI |
| Linux x86_64 discrete GPU | Vulkan | V01–V09 and performance evidence |
| Windows x86_64 | DX12 | V01–V09 |
| macOS arm64 | Metal | V01–V09 |

Linux software GPU validates semantics but cannot demonstrate GPU performance.
At least one real GPU runs mutation, presentation, and extension performance
before a release claim. Unsupported adapters must fail capability probing with
actionable observed/required limits. Web is neither built nor claimed.

## 6. Definition of implementation-ready completion

The implementation is complete only when:

1. root `AGENTS.md` contains the rules and commands from `overview.md`;
2. the public API has rustdoc and compile-pass/compile-fail contract tests;
3. every V01–V09 assertion is present and required platform status is honest;
4. host/shader ABI and checkpoint v1 have golden fixtures;
5. all repository commands in `overview.md` pass;
6. tier-1 portability results are attached to a versioned evidence manifest;
7. no validation crate uses private dependencies or feature-gated access; and
8. unresolved `not_demonstrated` evidence blocks the associated capability
   claim.
