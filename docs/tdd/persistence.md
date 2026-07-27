# Persistence and reconstruction

## 1. Persistence ownership

Moria owns a portable checkpoint object and a transactional `CheckpointSink`
protocol. The consumer owns filesystem, database, cloud, encryption, backup,
retention, and external behavior/game state. Derived meshes, dressing, GPU
addresses, queue state, observations, and receipts are never checkpoint truth.

A checkpoint contains:

- format/product contract versions and integrity metadata;
- material stable IDs and definition fingerprints;
- volume stable IDs, kinds, domains, cell sizes, source IDs, and base lineage;
- topology and per-volume revision cut;
- current dynamic placements;
- sparse final-value scars relative to the verified base; and
- volume creation/retirement records needed to reconstruct the cut.

## 2. Exact base lineage

`BaseLineage` is:

```rust
pub struct BaseLineage {
    pub namespace: [u8; 16],
    pub version: u64,
    pub domain_root: [u8; 32],
    pub brick_edge: u8, // must be 8 for format v1
}
```

`domain_root` is a BLAKE3 Merkle root over every brick in the finite volume
domain. Leaves hash canonical stable material IDs and coverage in brick-major
order. Internal nodes include coordinates and use homogeneous subtree hashes,
so a vast uniform domain does not require one stored leaf per brick.

Every `VerifiedBrick` supplies a proof to this root. Therefore Moria establishes
that materialized base bytes belong to the exact registered base, rather than
trusting an arbitrary lineage label. A generator or authored importer remains
consumer code; it must expose the same immutable root and proofs. Alternatively
the consumer may provide an immutable, content-addressed base artifact through
the source, but it still uses the root/proof protocol.

Restore requires exact equality of namespace, version, root, domain, cell size,
and source contract. A new root is a new lineage even if the consumer considers
the content similar. Migration/rebase is an external tool that produces a new
checkpoint; Moria never guesses.

## 3. Scar model

A confirmed mutation yields sorted, nonoverlapping cell runs containing final
stable material IDs and coverage. `ScarIndex` stores one entry per changed
brick:

```text
brick coordinate
base brick hash
base brick Merkle proof
latest included volume revision
canonical run stream
scar payload hash
```

Successive changes coalesce to final values. Values equal to verified base are
removed. An empty entry is deleted. This keeps cost proportional to outstanding
differences rather than command history or untouched volume. Coalescing checks
the base brick hash to prevent applying a scar to different base bytes.

Scars remain CPU-resident sparse state after commit and are also uploaded when
a region rematerializes. `max_dirty_scar_bytes` bounds them. Mutation admission
reserves worst-case growth; if no space can be freed by a successful
checkpoint, the mutation is rejected before admission.

Volume create/retire and placement scars are small typed records, not encoded
as matter patches. External behavior state is absent.

## 4. Checkpoint cut and protocol

`request_checkpoint` names a world and either:

- `LatestConfirmed`, captured atomically at admission; or
- an explicit topology/revision vector not newer than confirmed state.

Only one checkpoint per world is assembled at a time. Later mutations may
continue if dirty-scar capacity remains. MVCC scar generations preserve the
requested cut; later changes are not silently included.

An explicit older cut is accepted only when every named root, placement, and
scar generation is still retained by an existing snapshot/checkpoint pin.
Moria does not reverse newer changes to synthesize history. An unretained or
topologically inconsistent vector is rejected as `RevisionUnavailable`; the
latest-confirmed cut remains available.

The sink protocol is:

```text
begin(checkpoint_id, expected_max_bytes)
  -> write_chunk(index, bytes, blake3)
  -> prepare(manifest_bytes, manifest_blake3)
  -> commit
```

Before `commit`, any error invokes `abort`; an aborted object is not a
checkpoint. `commit` must provide all-or-nothing publication and return a
durability token. Filesystem sinks implement this with a sibling temporary
file, file sync, atomic rename, and parent-directory sync. Services may use a
transaction or immutable object plus atomic pointer.

Checkpoint completion returns the exact durable revision vector, byte counts,
object hash, and sink durability token. Dirty generations at or below that cut
become clean only after commit. Newer scars remain dirty. Sink failure retains
all dirty generations and reports retryability.

## 5. Portable format

Format v1 is a chunked little-endian binary container:

```text
header: "MORIA\0CP", format u16, contract u16, manifest length, chunk count
manifest: canonical CBOR (sorted integer keys, no floats)
chunks: [kind u16, uncompressed_len u64, compressed_len u64,
         blake3[32], zstd payload]
footer: blake3 of header + manifest + chunk descriptors + chunk hashes
```

Coordinates are signed 32-bit integers, sizes/revisions are fixed-width
integers, UUID-like IDs are 16 network-order bytes, and placements are
canonical IEEE-754 `f64` bit patterns after finite/normalized validation.
Unknown required manifest keys or chunk kinds fail restore; explicitly marked
optional keys may be skipped. Limits are checked before allocation or
decompression. A chunk's declared expansion must fit the configured restore
budget.

The codec never serializes Rust layouts. Golden fixtures created on little- and
big-endian emulators, corruption/property tests, and independent manifest
decoding protect portability. Format readers are additive: writer v2 does not
change v1 interpretation.

## 6. Restore

Restore is a startup mode and publishes no partial world:

1. read and authenticate header, manifest, chunks, and footer under byte limits;
2. validate supported format/contract and unique IDs;
3. match every material fingerprint;
4. obtain each required content source and match exact `BaseLineage`;
5. validate volume definitions, every scarred brick's stored Merkle proof,
   scars, placement, and revision monotonicity;
6. ask each source to prove a bounded deterministic probe selected from its
   domain root, then materialize only the startup-interest bricks and overlay
   their scars;
7. create CPU metadata and those bounded GPU roots in a private restoring
   world; and
8. publish `Ready` with restored topology/revision vector only after all
   required collaborators and GPU state succeed.

Cold unscarred bricks remain sparse and materialize later with Merkle proofs.
The source contract ensures they belong to the same exact base root. Missing
source/material/volume, corrupt or incomplete bytes, base mismatch, unsupported
contract, invalid placement, budget overflow, or unverified reconstruction
fails the whole restore. The prior running world, if any, is not replaced.

Presentation and dressing start absent and rebuild from restored truth.
Consumers restore their behavior/game state after Moria reports its restored
revision context; cross-system transaction policy is consumer-owned.
