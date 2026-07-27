# Technical Decision Record

These are engineering decisions made under the approved product design. They
are not human product decisions and may be revised by a later TDD with
equivalent public semantics and new evidence.

## T1. One package with a Bevy adapter module

**Decision.** Start with one `moria` library package. The facade and Bevy
adapter are modules in that package; examples are ordinary consumers.

**Reason.** There is one deliverable and one renderer integration. A workspace
would add dependency and release coordination without providing a required
compile or packaging boundary.

**Revisit when.** A standalone compute library, tool binary, or separately
versioned adapter becomes an actual deliverable.

## T2. Fixed 8³ bricks and compact scalar samples

**Decision.** V1 uses 8×8×8 bricks and a four-byte material sample.

**Reason.** A 2 KiB detailed brick is small enough for copy-on-write mutation,
bounded readback, and fine-grained interest while retaining useful GPU
coalescing. Coverage supports non-cubic surfaces without promoting meshes to
truth.

**Rejected.** A dense world, camera-only clipmap, and hardware sparse textures
all weaken volume-general bounded residency or portable compute support.

## T3. GPU hash pages with MVCC revision chains

**Decision.** A bounded open-addressed hash table maps logical brick keys to a
version chain. Each new page version is tagged with its proposed volume
revision; a single revision-gate write publishes a prepared command.

**Reason.** Irregular sparse volumes favor hashing. Revision filtering lets
readers ignore prepared pages until one atomic publication and lets prior
readers finish without global copies.

**Rejected.** In-place multi-brick writes expose partial state. Double
buffering the entire page table makes mutation cost scale with resident world
size. A camera clipmap privileges presentation interest.

## T4. One revision stream per volume

**Decision.** Matter, placement, creation readiness, and retirement facts use a
single monotonic revision stream per volume. A world observation sequence
orders delivery but is not a cross-volume truth revision.

**Reason.** Queries can report the exact revisions observed without promising
unsupported atomicity between independent volumes.

## T5. Runtime-neutral receipts with Bevy-driven progress

**Decision.** Facade handles are `Send + Sync`; accepted work returns a typed,
`Send` receipt that implements `Future` and also supports nonblocking polling.
Bounded owner queues and render schedules drive progress. No async runtime is
part of the public contract.

**Reason.** Games may use Tokio, another executor, or Bevy tasks. Moria must not
force one or confuse queue admission with completion.

## T6. Content source executes on bounded workers

**Decision.** Base content is a synchronous, batch-oriented `Send + Sync`
callback executed by Moria's bounded worker pool.

**Reason.** Authored I/O and consumer generation stay outside render schedules,
while a synchronous callback avoids imposing a consumer async runtime.

## T7. GPU behavior uses copied packets and candidate effects

**Decision.** GPU extensions receive bounded, Moria-produced inspection packets
in extension-owned buffers and write fixed-schema candidate effects. They
never bind the page table or brick pool. Candidate effects are validated and
published through normal admission.

**Reason.** This preserves a GPU-to-GPU path without giving a behavior engine
storage ownership or a privileged mutation route.

**Rejected.** Direct read/write buffer leases are faster to prototype but
invalidate ownership, bounds, atomicity, and recovery contracts.

## T8. Sparse full-brick scars

**Decision.** A scar stores the complete committed 8³ sample payload for every
brick changed relative to the registered base, plus persistent volume identity
and placement. Optional compaction may remove a scar proven byte-equal to base.

**Reason.** Full changed bricks give deterministic restore and simple checksums
without serializing untouched matter. Cell-level compression can be added
inside a versioned chunk codec after measurement.

## T9. Dual contouring for presentation; cell occupancy for collision

**Decision.** GPU dual contouring derives per-brick surfaces. Material
presentation class selects smooth or feature-preserving constraints.
Collision tests occupied cells and coverage, independent of the mesh.

**Reason.** One technique can produce coherent organic and crisp constructed
surfaces, while separating collision preserves truth when meshing is stale or
failed.

## T10. Native renderer integration only

**Decision.** V1 qualifies Vulkan, Metal, and DX12 by capabilities, not backend
name. WebGPU, WebGL/GLES, and standalone-device constructors are absent.

**Reason.** The approved design excludes web as a current target, and Bevy owns
the renderer device in the supported path. Unsupported targets fail clearly
instead of receiving weaker semantics.
