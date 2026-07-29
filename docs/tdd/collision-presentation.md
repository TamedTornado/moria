# Collision, participant interop, and presentation

Collision is a canonical derivation of material truth. Presentation is a
discardable consumer of that truth. They share source roots but never
authority.

## Canonical occupancy and collision

### TECH-051 — Collision primitive set

Implements: REQ-010, REQ-019, REQ-028, REQ-044

`CollisionShapeQ` is the closed set:

```rust
pub enum CollisionShapeQ {
    Point { point: WorldPointQ },
    Aabb { center: WorldPointQ, half_extent: WorldVectorQ },
    Sphere { center: WorldPointQ, radius: Q23_8 },
    Capsule {
        a: WorldPointQ,
        b: WorldPointQ,
        radius: Q23_8,
    },
    OrientedBox {
        center: WorldPointQ,
        half_extent: WorldVectorQ,
        orientation: QuatQ14,
    },
}
```

Extents/radii are nonnegative, a capsule may degenerate to a sphere, and all
shapes must fit the request bounds. `Trace` uses a closed fixed-point segment.
`Sweep` linearly translates a shape without rotating it during the sweep;
consumers represent rotation as separate bounded steps. These primitives cover
point/region occupancy, traces, overlaps, and swept movement without exposing
meshes. More primitives require a new versioned collision contract.

Each occupied local cell is its exact closed-open cell box transformed by the
volume's canonical placement. Broad phase compares checked world AABBs.
Narrow phase transforms the consumer shape into volume-local space and uses
fixed-point slab, closest-feature, and separating-axis tests. Products and
squared distances use TECH-007's exact 64-bit semantics. Contact normal is a
canonical signed Q1.14 vector; ties select x, then y, then z, then negative
before positive face. Time of impact is unsigned Q0.32 in `[0,1]` with floor
rounding, conservatively choosing the earliest representable contact.

### TECH-052 — Sparse collision traversal and facts

Implements: REQ-001, REQ-010, REQ-017, REQ-019, REQ-021

Collision reads the pinned canonical root and verified resident base bricks,
never mesh, dressing, or presentation bounds. It traverses volumes in
`VolumeId` order, radix brick ranges in Morton-independent lexicographic
`(z,y,x)` order, then cells. Uniform empty nodes reject whole regions. Exact
brick occupancy masks are derived from `CellWire` and cached with their source
leaf hash; masks may accelerate but cannot report occupied matter empty.

Trace uses three-axis integer DDA with rational comparisons; sweep broad phase
uses the union of start/end bounds. Candidate facts occupy fixed input slots,
then stable mark/scan/scatter and sort establish TECH-024 order. A complete
fact is:

```text
tick, world root, volume/revision, local cell, material,
time_of_impact, contact point, normal, source leaf hash
```

`NoHit` is emitted only if every required brick was ready and inspected.
Missing/cold/corrupt truth is `Pending` or `Unavailable`. Canonical participant
collision is prepared against `State[t]`; absent or source-mismatched input
causes tick `NoAdvance`, never collision-disabled success.

### TECH-053 — Canonical collider artifact

Implements: REQ-005, REQ-006, REQ-019, REQ-030

A participant descriptor may reserve collider artifacts by bounded regions,
shape classes, maximum occupied cells, and bytes. Moria builds a canonical
artifact from a pinned state:

```text
header {
  contract, tick, world_root, request_digest,
  volume_count, record_count, complete
}
sorted volume records { id, revision, inverse placement, range }
sorted occupied spans { volume_id, brick_coord, z/y/x run, material, density }
artifact_hash
```

Spans are a deterministic lossless encoding of occupancy; capacity overflow is
an unavailable artifact, not truncation. The artifact hash binds all bytes and
source roots. CPU participants receive decoded immutable bytes. GPU
participants receive the same bytes in a read-only artifact buffer. This is a
bounded derived representation, not a second authority: it is accepted only
for its exact source hash and can always be rebuilt from matter.

## GPU behavior participant seam

### TECH-054 — Bounded GPU participant adapter

Implements: REQ-002, REQ-005, REQ-006, REQ-012, REQ-030, REQ-044

The deliberately Bevy/wgpu-coupled seam is:

```rust
pub trait BevyGpuParticipant: Send + Sync + 'static {
    fn descriptor(&self) -> GpuParticipantDescriptor;
    fn prepare_device(
        &self,
        device: &RenderDevice,
        contract: &GpuParticipantContract,
    ) -> Result<Box<dyn GpuParticipantDeviceState>, ParticipantError>;
}

pub trait GpuParticipantDeviceState: Send + Sync {
    fn encode(
        &mut self,
        pass: &mut wgpu::ComputePass<'_>,
        input: ParticipantGpuInput<'_>,
        effects: ParticipantEffectSink<'_>,
    ) -> Result<(), ParticipantError>;
}
```

`ParticipantGpuInput` exposes a Moria-created read-only bind group for the
bounded canonical collider artifact, immutable participant input bytes, exact
counts, tick/root/artifact hash, and the participant's own registered buffers.
It cannot return Moria's radix, brick, page-table, allocator, or mutable
buffers. `ParticipantEffectSink` is a Moria-owned fixed-slot buffer whose
schema is ordinary command wire plus one status/commitment record. The
participant cannot submit the encoder or increase capacity.

Moria invokes adapters in `ParticipantId` order in the canonical preparation
schedule, then validates effect tags, bounds, preconditions, unique local
sequences, unused-zero slots, overflow flags, and commitment before normal
transition processing. The output remains GPU-resident through validation and
application; only bounded canonical outcomes/commitment are read back for
receipt and replay. Thus a GPU behavior engine need not round-trip occupancy or
effects through CPU, while admission and publication remain identical.

Adapter pipeline/device creation failure, panic caught at the FFI/callback
boundary where possible, validation error, output overflow, or old generation
causes `NoAdvance`. There is no automatic CPU implementation swap. Device
state reconstructs in `RenderStartup`.

## Derived presentation

### TECH-055 — Hybrid surface derivation

Implements: REQ-001, REQ-003, REQ-013, REQ-020

Presentation is brick-local with a one-cell neighbor halo:

- `SmoothDensity` materials use surface nets over the signed Q8.8 density
  field. One vertex is produced per sign-changing cell by a deterministic
  integer average, then converted to float for rendering.
- `CrispCell` materials use greedy axis-aligned quads within a volume's local
  lattice. Merging depends only on material/presentation key and source face.
- Boundaries between styles emit the crisp material face plus a clipped smooth
  edge; duplicate faces resolve by stable material ID.

The baseline produces indexed Bevy meshes per `(volume, brick presentation
chunk, source revision)`. Dynamic volume vertices remain local and the Bevy
transform derives from canonical placement. Movement therefore changes the
transform, not material cells. Neighbor edits dirty the changed brick and its
26 halo neighbors so cuts and seams rebuild honestly.

Mesh vertex/index order and float values are noncanonical. Buffers and output
counts are bounded; overflow sets `PresentationState::Failed` for that chunk
and leaves matter/collision intact. Raw cell display is a diagnostic option,
not the normal surface contract.

### TECH-056 — Presentation lifecycle and revision installation

Implements: REQ-011, REQ-012, REQ-013, REQ-017, REQ-021, REQ-040

Each presentation chunk follows:

```text
Absent -> Building(source root/revision)
       -> Current(source root/revision)
       -> Stale(old source) -> Building(new source)
       -> Failed(source, reason)
```

A job pins its source root. Installation compares world, volume, brick,
revision, root hash, and device generation with the current requested source.
A mismatch discards the result; it cannot become current. Consumers configure
whether stale meshes remain visible, disappear, or are replaced by a
diagnostic marker. Collision never reads any choice.

Dirty revisions coalesce to the newest committed source when no job is
submitted; a submitted job drains and is then discarded if stale. Rollback
cancels/invalidates future-source jobs and enqueues only the final corrected
dirty union. `PresentationCurrent` observations emit only after GPU mesh upload
and Bevy entity installation for the named revision.

### TECH-057 — Revision-anchored dressing and assemblies

Implements: REQ-003, REQ-013, REQ-020

Derived dressing is generated from a presentation surface plus a consumer
presentation seed that is explicitly noncanonical. Every dressing instance
carries supporting `(volume, brick, material, source leaf hash, revision)`.
When support changes or disappears, the instance is removed or regenerated
before the chunk can report current. Dressing has no collision, occupancy,
persistence, query identity, or canonical hash contribution.

Anything requiring independent material identity or occupancy is not dressing.
It is registered by the consumer as an ordinary static/dynamic material volume
or included in base/patch content, and therefore obeys normal IDs, ticks,
mutation, collision, rollback, and persistence. Moria supplies no vegetation,
clutter, geology, or assembly recipe.

### TECH-058 — Presentation resource and failure isolation

Implements: REQ-007, REQ-013, REQ-018, REQ-021, REQ-022

Presentation has a separate byte budget and at most three in-flight jobs by
default. Under pressure it coalesces dirty work, retires out-of-interest
chunks, keeps stale views per policy, or marks a chunk failed. It cannot claim
canonical pool permits, evict pinned truth, block a tick on shader readiness,
or feed mesh counts/timing into simulation.

Presentation telemetry reports requested/current/stale/failed chunks,
truth-to-view revision lag, mesh/vertex/index bytes, queue age, rebuild cause,
overflow, and commit-to-current latency. Rendering correctness is human/visual
evidence; presentation failure injection must leave identical canonical hashes
and collision facts.
