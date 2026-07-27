# Persistence and Restore

## Authority and scope

Persistence records enough substrate-owned state to reconstruct committed
material truth:

- checkpoint contract/version and integrity metadata;
- stable world, material, and volume identities;
- each volume's domain, cell size, mode, lineage, exact reconstruction
  fingerprint, committed revision, and committed placement;
- sparse full-brick scars for bricks changed relative to base;
- volume retirement tombstones.

It does not record runtime IDs, physical slots, page-table shape, occupancy
acceleration, mesh/dressing data, cameras, external behavior state, generation
code, or other consumer state.

## Scar authority

A scar brick is the complete 512-sample committed value of one logical brick.
It is stored only for a brick that has been changed through an admitted command
or imported by restore. A background compactor may ask the registered base
source for that brick and remove the scar only after byte-for-byte equality is
proven at the same reconstruction fingerprint.

Until that proof, a no-op or reverted command may conservatively leave a scar.
This affects storage cost, not truth. Scar state tracks the source volume
revision.

Placement is stored once per volume at the checkpoint frontier, not as an event
log. A retired volume has a tombstone with its key and terminal revision so
restore cannot accidentally resurrect it through current registration.

## Checkpoint frontier

Admission captures sorted `(VolumeKey, VolumeRevision, Placement)` entries.
That immutable frontier `F` is the checkpoint truth:

- scar page versions visible at `F` are pinned;
- later commits allocate later versions and remain dirty;
- unchanged chunks may be reused by digest from an earlier checkpoint;
- completion returns exactly `F`, never the current revisions at completion.

Checkpoint readback copies dirty scar bricks through the ordinary staging pool.
It can batch and stream chunks, but in-flight bytes/maps remain bounded.
Mutation completion does not imply checkpoint durability; only atomic manifest
publication advances the durable frontier.

## Binary format v1

All integers are little-endian. Decoding uses checked offsets and declared
lengths before allocation. Rust struct memory layout and general-purpose serde
formats are not persistence formats.

### Manifest

```text
magic                  8 bytes = "MORIA\0\1\0"
format_version         u16 = 1
minimum_reader_version u16 = 1
flags                  u32 = 0
world_uuid             16 bytes
checkpoint_uuid        16 bytes
material_count         u32
volume_count           u32
tombstone_count        u32
chunk_count            u32
sections...            length-prefixed, canonical order
manifest_blake3        32 bytes over every preceding byte
```

Material records sort by stable UUID and contain UUID, checkpoint runtime
material number, debug-name bytes, and a digest of the occupancy-relevant
definition. Restore maps UUIDs to current runtime IDs and requires the
occupancy-relevant definition digest to match. Presentation input may change
without a persistence migration.

Volume records sort by stable UUID and contain:

- UUID and debug-name bytes;
- domain min/max `i32` triples and `cell_size` IEEE-754 bits;
- static/dynamic tag;
- lineage bytes and reconstruction fingerprint;
- frontier revision;
- placement translation/quaternion IEEE-754 bits;
- sorted `(BrickCoord, ChunkDigest, record_index)` scar references.

Tombstones sort by volume UUID. Chunk directory entries sort by digest and
contain encoded size, decoded record count, codec, and CRC32.

### Scar chunk

A chunk is at most 4 MiB encoded and 8 MiB decoded. It begins with its own
magic/version/count/length/CRC. Records sort by `(VolumeKey, brick z, y, x)` and
contain volume UUID, brick coordinate, source revision, encoding tag, and
payload.

V1 encoding tags are:

- `Homogeneous`: one four-byte sample;
- `Raw`: exactly 2,048 sample bytes;
- `Rle`: X-fastest runs of `(u16 run_length, sample)` totaling 512.

The encoder deterministically selects the smallest representation, breaking
ties `Homogeneous`, then `Rle`, then `Raw`. The chunk digest is BLAKE3 over
encoded bytes. Duplicate record keys, invalid flags/material keys, unsorted
records, trailing bytes, size mismatch, checksum mismatch, and decompression
overflow are corruption.

## Store transaction

The writer contract is content-addressed chunks followed by one atomic manifest
publish. An implementation may deduplicate an existing verified chunk.

Filesystem layout:

```text
checkpoint-root/
  chunks/ab/<full-blake3-hex>
  manifests/<checkpoint-uuid>.moria
  tmp/<transaction-uuid>/
```

The included native store writes a chunk temp file in the target filesystem,
flushes and syncs it, renames it to the digest path, then syncs the parent.
After every chunk is durable it does the same for the manifest. Existing
digest files are length/checksum verified before reuse. Temporary/incomplete
files are ignored by readers and can be garbage-collected only when no live
writer references them.

Store backends must document whether parent-directory sync is supported. If a
platform cannot meet crash-atomic publication, the backend reports
`UnsupportedDurability`; it cannot claim a successful durable checkpoint.

## Restore validation

Restore is fail-closed and ordered:

1. Read and validate manifest size, magic, version, section lengths, canonical
   ordering, and digest.
2. Verify world identity or explicit consumer-selected new-world import mode.
3. Match every persisted material UUID and occupancy definition to a
   registration. Extra current presentation-only definitions are allowed;
   missing persisted materials fail.
4. Match every live volume UUID, finite domain/cell size/mode, lineage, and
   exact reconstruction fingerprint to its registered source. A missing source
   or matching lineage with different fingerprint fails.
5. Verify every referenced chunk exists and its size/digest/CRC before any
   world directory is published.
6. Decode all scar records into bounded host batches, validate coordinates
   intersect the volume domain and sample material mappings, then stage them
   for lazy GPU materialization.
7. Publish restored volume directories at saved revisions/placements and
   expose regions as cold with known scars.
8. Resolve the restore receipt with the complete revision context.

Import mode may assign a new world UUID but does not relax material, volume,
lineage, or fingerprint validation. Rebase/migration is a separate consumer
tool outside Moria v1; the library returns structured mismatch data to support
one.

## Base plus scar materialization

For a cold restored brick:

1. if a scar reference exists, decode and upload the scar as authoritative
   content at the restored revision;
2. otherwise request the exact registered base source;
3. validate the returned batch and upload its homogeneous/detailed form;
4. publish region readiness only after GPU installation.

Moria does not need to rerun the base for scarred bricks. The fingerprint
nevertheless remains mandatory because every unscarred brick relies on it.

## Dirty coverage and failure

Telemetry maintains per-volume:

- current committed revision;
- newest named-checkpoint durable revision;
- dirty brick count/bytes;
- pinned checkpoint bytes;
- last checkpoint error.

A failed checkpoint retains dirty truth and releases only redundant staging.
Budget pressure may block new mutations when dirty/scar capacity is full. It
cannot evict or silently authorize loss.

Corrupt checkpoint input does not mutate the store and publishes no restored
world. Unsupported newer format returns both saved and supported versions.
Decode, missing identity, base mismatch, and store I/O are separate error
categories.

## Persistence tests

Required fixtures and generated tests cover:

- byte-stable canonical encoding independent of hash-map insertion order;
- every codec and malformed length/tag/order/checksum case;
- save/restore of edited static and edited+moved dynamic volumes;
- checkpoint frontier while a later mutation remains dirty;
- homogeneous, detailed, negative-coordinate, and domain-edge scars;
- absent/mismatched materials, volume, lineage, and reconstruction fingerprint;
- chunk write failure, manifest failure, and incomplete transaction cleanup;
- device-derived state discarded and rebuilt after restore;
- semantic sample/query/collision equality before save and after restore;
- v1 golden fixture readability and explicit rejection of an unsupported v2
  fixture.
