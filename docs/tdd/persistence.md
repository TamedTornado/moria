# Persistence and Restore

## 1. Persistence boundary

Moria owns the checkpoint format and reconstruction protocol. The consumer owns
the durable medium through:

```rust
pub trait CheckpointStore: Send + Sync + 'static {
    fn begin(&self, meta: CheckpointMeta)
        -> BoxFuture<'static, Result<Box<dyn CheckpointWriter>, StoreError>>;
    fn open(&self, id: CheckpointId)
        -> BoxFuture<'static, Result<Box<dyn CheckpointReader>, StoreError>>;
}

pub trait CheckpointWriter: Send {
    fn write_chunk(&mut self, chunk: CheckpointChunk)
        -> BoxFuture<'_, Result<(), StoreError>>;
    fn commit(self: Box<Self>, manifest_digest: Digest)
        -> BoxFuture<'static, Result<DurabilityProof, StoreError>>;
    fn abort(self: Box<Self>) -> BoxFuture<'static, Result<(), StoreError>>;
}

pub trait CheckpointReader: Send {
    fn read_next(
        &mut self,
        max_bytes: u32,
    ) -> BoxFuture<'_, Result<Option<StoredChunk>, StoreError>>;
}
```

`read_next` is an ordered stream beginning with the envelope header and ending
at the footer; it must never return a chunk larger than `max_bytes`. Premature
EOF, duplicate data, or bytes after the footer are corruption. Chunks are
bounded. `commit` must provide all-or-nothing visibility and a bounded
durability proof meaningful for the store. Moria does not treat that opaque
proof as an integrity shortcut: it always reopens the committed checkpoint,
streams the complete envelope, and verifies every chunk plus the
header/manifest/footer lengths and digests before publishing `Durable`.
Filesystem, object-store, database, and consumer-specific implementations sit
outside Moria; conformance includes memory and crash-injecting filesystem
stores.

`WorldSpec::persistence` is exactly one of:

- `MemoryOnly`, which retains dirty scars and tombstones in bounded process
  memory and rejects checkpoint requests; or
- `Checkpointed { store, retirement }`, where `retirement` is
  `RetainInMemory` or `RequireCheckpoint`.

Neither mode permits silent loss. `MemoryOnly` makes no crash-durability claim.
The selected mode, store capability limits, and scar/catalog reservations are
validated before world publication.

The corresponding public DTOs are:

```rust
pub enum PersistencePolicy {
    MemoryOnly,
    Checkpointed {
        store: Arc<dyn CheckpointStore>,
        retirement: RetirementDurability,
    },
}

pub struct CheckpointRequest {
    pub selection: CheckpointSelection,
    pub correlation: BoundedBytes<256>,
}

pub enum CheckpointSelection {
    LatestCommitted,
    Exact(RevisionSet),
}

pub struct CheckpointResult {
    pub checkpoint: CheckpointId,
    pub durable: RevisionSet,
    pub manifest_digest: Digest,
    pub durability_proof_digest: Digest,
}
```

The durability-proof digest is audit evidence, not a substitute for Moria's
read-after-commit verification.

Moria persists no presentation mesh, dressing, occupancy hierarchy, query
cache, receipt, telemetry history, external behavior state, or consumer
metadata beyond the explicitly opaque bounded material/world metadata. The
canonical material table, including its bounded opaque metadata bytes, is part
of the manifest so restore can prove that stable IDs retain the same registered
meaning; Moria never interprets those opaque bytes.

## 2. Exact base identity

A lineage label alone is insufficient. Each finite volume base is committed by:

```rust
pub struct BaseIdentity {
    pub lineage: LineageId,
    pub cell_encoding: CellEncodingVersion,
    pub domain: CellBounds,
    pub brick_edge: u8,             // 8
    pub merkle_root: Digest,
}
```

The root is a canonical sparse Merkle tree over every aligned leaf brick in
lexicographic coordinate order. Leaf hashes cover volume ID, domain edge mask,
canonical brick representation, and encoding version. Empty and homogeneous
subtrees have precomputed level-specific hashes, so a sparse or procedural
consumer does not need to store every leaf merely to represent homogeneous
space.

Every `BaseRegion` returns leaf data plus a multiproof to the registered root.
Moria verifies it before accepting any base cell as authority. Thus restore
establishes exact equality with the saved base identity rather than trusting a
matching lineage string.

Consumers with non-reconstructable base content must durably retain an external
content-addressed base snapshot capable of returning these proven leaves.
`BaseDescriptor::availability` is either:

- `Reconstructable`: the source promises future proof-bearing reconstruction;
  or
- `ExternalSnapshot { snapshot_id, durability }`: the consumer's source/store
  promises a durable exact base.

Startup rejects `Ephemeral` sources when persistence is required. A checkpoint
records the full `BaseIdentity` and external snapshot reference, but does not
copy the whole untouched base into Moria's scar stream. Loss of the external
snapshot is an explicit restore failure.

Changing source code, recipe, or authored data without changing the Merkle root
is safe because it reconstructs identical cells. A different root, even under
the same lineage, requires explicit consumer migration/rebase outside normal
restore.

## 3. Scar model

The durable material scar for a changed leaf is a canonical delta against its
proven base leaf:

```rust
pub enum BrickScar {
    Homogeneous(PackedCell),
    Runs(Vec<CellRun>), // sorted, nonoverlapping changed indices only
}
```

`Runs` records exact replacement cells at local indices. A scar stores no entry
equal to base. If every resulting cell is equal, the scar disappears. If all
result cells are identical, canonical encoding uses `Homogeneous`; otherwise
runs merge adjacent equal replacements. This keeps repeated edits bounded by
changed cells, not command history.

The CPU scar index is keyed by `(VolumeId, BrickCoord)` and stores the latest
canonical scar plus:

- base leaf digest;
- first/last revision changed;
- dirty checkpoint generation; and
- checksum.

Placement, creation, and retirement scars are canonical state records, not an
unbounded event journal. Volume records store persistent identity, kind,
domain, cell size, base identity/reference, current placement, current
revision, and retired state. External behavior state is never included.

The scar index may be compacted asynchronously, but the old version remains
until the new checksum is verified. CPU scar memory is charged against its own
budget. An admitted edit reserves its worst-case new scar bytes; exhaustion
rejects admission before staging.

## 4. Checkpoint cut and format

Checkpoint request selects a world and either:

- `LatestCommitted`: runtime captures current catalog/volume revisions at the
  cut; or
- an exact current `RevisionSet`: rejected if any named revision is not the
  current committed state.

An exact checkpoint set must contain the current catalog revision and every
active volume exactly once; checkpoints are whole-world cuts, not partial
volume saves. The cut also includes every catalog tombstone and current
creation/retirement record. Catalog snapshot capture and immutable scar
reference capture occur in one scheduler transaction ordered before or after
each command publication.

Capturing the cut briefly snapshots immutable `Arc` references to catalog,
volume state, and scar versions. Later commits use copy-on-write scar entries
and remain dirty for the next generation. Checkpoint encoding and I/O do not
block later mutations.

Envelope:

```text
Header
  magic "MORIA\0CP"
  format major/minor
  product contract ID
  cell/shader layout IDs
  world ID and configuration digest
Manifest chunk
  canonical material-definition table and bounded opaque metadata
  catalog revision and sorted volume records
  exact per-volume durable revision coverage
  base identities and external snapshot references
Scar chunks
  sorted volume ID, brick coordinate, canonical scar records
Footer
  ordered chunk digests, manifest digest, total lengths
Store durability proof (outside hashed envelope)
```

The configuration digest covers every canonical material definition, cell
encoding, volume immutable field, and registered base identity. Restore
requires the supplied material table to match it byte-for-byte; changing a
material definition is an explicit migration, as required by `public-api.md`.
The digest excludes runtime budgets, queue capacities, adapter/driver,
telemetry, presentation asset registrations, and stale-view policy, which may
change on restore. Presentation registrations are validated separately for
requested presentation but never gate restoration of matter truth.

Integers use fixed little-endian encoding. Postcard payload schema is wrapped
in explicit chunk type/version/length/digest fields. Chunks are independently
bounded and checksummed with BLAKE3; the footer commits their order. Unknown
required chunk types fail restore. Unknown optional telemetry metadata may be
ignored only when explicitly flagged optional.

Format major changes are incompatible without an explicit migration tool.
Minor changes may add optional fields/chunks and require forward/backward
golden tests. The checkpoint records both format and approved product contract
IDs; no version mismatch is guessed through.

A checkpoint is `Durable` only after:

1. all chunks write successfully;
2. store atomic commit succeeds;
3. store returns a verifiable durability proof; and
4. Moria reopens/reads the manifest/footer or the store proof cryptographically
   commits their digest.

Completion returns exactly the captured `RevisionSet`. Dirty flags clear only
for scar versions included in that cut; later versions remain dirty.

## 5. Restore protocol

Restore accepts a `RestoreSpec` containing the checkpoint ID, expected world
ID, current material table, a source registry keyed by every saved active
`VolumeId`, checkpoint store, budgets, persistence policy, and presentation
registrations. Retired tombstones require no live source. It operates into a
new configuring world and is atomic at world visibility:

```rust
pub struct RestoreSpec {
    pub checkpoint: CheckpointId,
    pub expected_world: WorldId,
    pub materials: Vec<MaterialDefinition>,
    pub sources: BTreeMap<VolumeId, Arc<dyn BaseContentSource>>,
    pub store: Arc<dyn CheckpointStore>,
    pub persistence: PersistencePolicy,
    pub budgets: WorldBudgets,
    pub presentation: PresentationRegistrations,
    pub gpu_extensions: Vec<GpuExtensionRegistration>,
}
```

The `store` reads the selected existing checkpoint. The restored world's
`persistence` policy controls later writes and may name the same or a different
store. All vectors/maps are bounded by the limits reserved during restore.
Without the `gpu-extension` feature, the extension field is absent and saved
matter remains fully restorable.

1. Open header/footer and validate magic, lengths, digests, format, product
   contract, and configuration compatibility.
2. Validate all identities, sorted uniqueness, coordinate arithmetic, material
   references, and volume/source availability without creating visible state.
3. Compare each configured source's complete `BaseIdentity`, including Merkle
   root, encoding, and domain. Verify required external snapshot durability.
4. Stream bounded scar chunks into a private restore index, checking base leaf
   digests, canonical ordering, replacement cells, and aggregate budgets.
5. Validate registry records, revisions, placements, retirement state, and
   catalog revision.
6. Stream through every scarred brick in batches bounded by source-staging
   limits, request its base leaf and verify its Merkle proof, apply its scar in
   the validation path, and validate the resulting checksum. The total accepted
   scar index was already reserved against the configured aggregate CPU budget
   in step 4; no unbounded validation set is resident at once.
7. Install private runtime metadata and keep regions cold. Materialization
   later combines proven base plus restored scars.
8. Atomically publish the world as `Ready`, return its restored revision
   context, and emit one restored world observation.

Any failure drops private restore state and leaves no addressable partial
world. Restore does not automatically migrate, rebase, substitute missing
materials, create sources, or reinterpret external consumer state.

Corrupt chunks, incomplete store commits, unsupported contract/layout, missing
material/source/volume, mismatched root/lineage/domain, unavailable external
snapshot, proof failure, invalid scar, and budget excess are distinct stable
errors.

## 6. Eviction and shutdown durability

Region eviction needs no checkpoint when every dirty scar is safely retained in
the CPU scar index; rematerialization uses base plus that scar. Scar-index
eviction to durable storage uses an internal checkpoint fragment only if the
configured store supports it, and cannot clear dirty world revision coverage
until a full manifest checkpoint commits.

The default shutdown policy requires a checkpoint for dirty revisions.
Persistence failure retains in-memory scars and prevents successful destructive
shutdown. Only the explicit two-step discard contract in `runtime.md` may
authorize their loss.

## 7. Migration and rebase

Moria exposes inspection helpers that decode and validate old supported
checkpoints, but normal restore accepts only declared-compatible formats and
identical base roots. A separate consumer-invoked tool may:

- migrate envelope/schema without changing truth, or
- rebase scars by materializing old-base-plus-scar and explicitly resolving
  differences onto a new base.

Rebase must emit a new checkpoint, new base identity, evidence report, and
conflict list. It never occurs during application startup and is not part of
the substrate's automatic policy.
