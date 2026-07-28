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
acceleration, mesh/dressing data, cameras, scheduled or asynchronous external
behavior state, generation code, or other consumer state.

CPU behavior objects, factory-created GPU solver resources, and the semantic
contents of stimuli remain consumer-owned. Moria-owned handoff staging and
prior-feedback transport slots, accepted per-tick input records, and GPU input
upload allocations are ephemeral coordination records and are never
checkpointed. A consumer that needs a coherent combined save
waits for a `BehaviorTickReceipt`, checkpoints the returned substrate
revisions through Moria, and checkpoints each adapter's own state through its
own store/protocol. Moria does not claim atomic durability across those stores.
On restore, substrate readiness does not enable scheduled ticks until every
included adapter independently reports its CPU state loaded or its current
device-generation GPU state recreated. Failure to restore adapter state is an
adapter readiness failure; Moria never synthesizes, rolls back, or imports it
into the scar manifest.

Scheduled v2 component extraction can create only children whose complete
matter comes from one pinned source.
Those children use Moria-owned extracted-base provenance and are part of the
tick's atomic directory publication.
Arbitrary later ordinary creations still require consumer content sources and
their own receipts.

V2 exposes only `CheckpointScope::WholeWorld`. A successful manifest contains
every live volume at the captured frontier and every known retirement
tombstone. There is no partial-volume checkpoint and therefore no ambiguity
about omitted live volumes during restore. `volume_records` bounds their
combined count for the lifetime of the world; retirement changes a live record
to a tombstone and never creates an unaccounted manifest entry.

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

Admission captures sorted `(VolumeKey, VolumeRevision, Placement)` entries for
the complete live-volume directory plus the tombstone set. That immutable
frontier `F` is the checkpoint truth:

- scar page versions visible at `F` are pinned;
- later commits allocate later versions and remain dirty;
- unchanged chunks may be reused by digest from an earlier checkpoint;
- completion returns exactly `F`, never the current revisions at completion.

Checkpoint readback copies dirty scar bricks through the ordinary staging pool.
It can batch and stream chunks, but in-flight bytes/maps remain bounded.
Mutation completion does not imply checkpoint durability; only atomic manifest
publication advances the durable frontier.

## Binary format v2

All integers are little-endian. Decoding uses checked offsets and declared
lengths before allocation. Rust struct memory layout and general-purpose serde
formats are not persistence formats.

### Manifest

```text
magic                  8 bytes = "MORIA\0\2\0"
format_version         u16 = 2
minimum_reader_version u16 = 2
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

`volume_count + tombstone_count` is checked before addition/allocation and must
not exceed the effective `volume_records`; `volume_count` must not exceed
`live_volumes`. Restore rejects an oversized manifest with `SizeLimit` before
reading record sections. The 64 MiB manifest cap is an additional byte bound,
not a substitute for these record-count bounds.

Material records sort by stable UUID and contain UUID, checkpoint runtime
material number, debug-name bytes, and a digest of the occupancy-relevant
definition. Restore maps UUIDs to current runtime IDs and requires the
occupancy-relevant definition digest to match. Presentation input may change
without a persistence migration.

Volume records sort by stable UUID and contain:

- UUID and debug-name bytes;
- domain min/max `i32` triples and `cell_size` IEEE-754 bits;
- static/dynamic tag;
- one source tag:
  `ExternalBase { lineage, reconstruction_fingerprint }` or
  `ExtractedBase { parent_volume_uuid, extraction_uuid,
  parent_revision, complete_nonempty_brick_refs }`;
- frontier revision;
- placement translation/quaternion IEEE-754 bits;
- sorted `(BrickCoord, ChunkDigest, record_index)` scar references.

An extracted base contains every full brick with any nonempty sample at child
creation; omitted bricks inside its finite domain are canonical empty.
Its extraction UUID is the exact `ComponentExtractionKey`, and parent revision
is the pinned source revision before the extraction publication.
Its later scar references are changes relative to that immutable base.
The derived-base records use the same bounded scar-chunk codecs and chunk
directory but a distinct record tag, so restore cannot confuse base with a
later edit.
Before the first durable checkpoint, these complete bricks remain dirty and
resident/pinned; they cannot be evicted as though a consumer source could
recreate them.

Volume debug names are canonical 1..=96-byte UTF-8 strings from the directory's
exact `Box<str>`; the encoder never serializes discarded `String` capacity.
Checked manifest sizing covers every fixed field, name byte, scar reference,
and section prefix before allocation or store I/O.

Tombstones sort by volume UUID. Chunk directory entries sort by digest and
contain encoded size, decoded record count, codec, and CRC32.

### Scar chunk

A chunk is at most 4 MiB encoded and 8 MiB decoded. It begins with its own
magic/version/count/length/CRC. Records sort by `(VolumeKey, brick z, y, x)` and
contain volume UUID, brick coordinate, source revision, encoding tag, and
payload.

V2 brick encoding tags are:

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

The reader first reports manifest/chunk lengths, then fills caller-owned slices
through the bounded range API in [public-api.md](public-api.md). Moria rejects a
manifest over 64 MiB, an encoded chunk over 4 MiB, a changing length, an
out-of-range read, or a short read before decoding. Only Moria's configured
persistence staging pool owns read buffers; a store cannot make Moria allocate
an implementation-selected blob or expose an unbounded stream.

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
2. Apply the explicit `RestoreWorldMode`: require the saved `WorldKey`, or
   import under the builder's selected new key while preserving subordinate
   identities.
3. Match every persisted material UUID and occupancy definition to a current
   registration. Missing persisted materials fail. Extra current materials
   are ordinary valid materials and are allowed only because no saved sample
   refers to them; there is no presentation-only material class.
4. Require the current live registration key set to equal exactly the
   manifest's `ExternalBase` live key set and reject registration of a
   tombstoned or `ExtractedBase` key. For each external volume, require equal
   finite domain/cell size/mode, lineage, and exact reconstruction fingerprint.
   For each extracted child, validate unique identity, parent provenance,
   canonical frame, exact generated diagnostic name, dynamic mode, material
   IDs, and complete derived-base references, then reserve its runtime/
   directory record without invoking a consumer source. Validate nested
   children in parent-before-child order; a
   parent may be live or a retained tombstone whose terminal revision is at
   least the recorded extraction parent revision. A missing/extra external
   volume, registered extracted key, missing/cyclic parent provenance, missing
   source, or matching lineage with a different fingerprint fails.
5. Verify every referenced chunk exists and its size/digest/CRC before any
   world directory is published.
6. Decode all scar records into bounded host batches, validate coordinates
   intersect the volume domain and sample material mappings, then stage them
   for lazy GPU materialization.
7. Publish external and reconstructed extracted-child directories at saved
   revisions/placements and expose regions as cold with known base/scars.
8. Resolve the restore receipt with the complete revision context.

Import mode assigns exactly the `WorldKey` in `RestoreWorldMode::ImportAs` but
does not relax material, exact volume membership, tombstone, lineage, or
fingerprint validation. Rebase/migration is a separate consumer tool outside
Moria v1; the library returns structured mismatch data to support one.

## Base plus scar materialization

For a cold restored brick:

1. if a later scar reference exists, decode and upload the scar as authoritative
   content at the restored revision;
2. otherwise, for an extracted child, read its immutable derived-base brick or
   use canonical empty when absent;
3. otherwise request the exact registered external base source;
4. finalize the permit-backed output sink and upload its
   homogeneous/detailed form;
5. publish region readiness only after GPU installation.

Moria does not need to rerun the base for scarred bricks. An extracted child
never invokes a consumer base source. The external fingerprint
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
- zero/65,535/65,536 nonempty material-registration boundaries and persisted
  material-table counts, proving empty remains additional and reserved;
- exact whole-world volume membership, extra-current material acceptance,
  extra/missing current volume rejection, and same-key versus import mode;
- acceptance at `live_volumes` and `volume_records`, live-slot reuse after
  retirement, permanent lifetime-key exhaustion, and manifest rejection when
  live-plus-tombstone counts exceed effective limits;
- builder/runtime volume-name acceptance at 96 UTF-8 bytes and rejection at
  97, exact retained name ownership, and a maximum-`volume_records` directory
  of 96-byte names whose checked no-scar manifest size remains below the 64 MiB
  v2 cap (additional scar references still obey the independent cap);
- extraction before the first checkpoint pins every nonempty child base brick,
  persists it once with parent/extraction-key/revision provenance, and restores the
  exact child samples without invoking a consumer base source;
- extraction followed by child edits persists later full-brick scars relative
  to the immutable extracted base; compaction never discards that base;
- missing/duplicate/cyclic extracted provenance, a registered extracted key,
  incomplete derived-base references, and corrupt extracted-base chunks fail
  before directory publication;
- reader oversize/changing length, short/range read, and caller-buffer bounds;
- chunk write failure, manifest failure, and incomplete transaction cleanup;
- device-derived state discarded and rebuilt after restore;
- semantic sample/query/collision equality before save and after restore;
- v2 golden fixture readability and explicit rejection of an unsupported v3
  fixture.
