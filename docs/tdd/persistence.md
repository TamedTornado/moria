# Single-Slot Persistence

## Scope and path

The one save records only absolute differences from the deterministic composite base. It does not store base voxels, columns, generated feature/object placement, meshes, dressing, water surfaces, active bands, player/camera state, fixed time, or benchmark state. There are no named slots, autosave history, cloud sync, save migration, or future-system payloads.

`PersistenceConfig` resolves one platform user-data path ending in `moria/product-one.delta.zst`. Tests and benchmarks inject an isolated path; production never writes into `assets/` or the repository. F5 requests an atomic save, F9 reloads this slot, and launch automatically applies a present compatible slot before `ControlReady`.

## Fixed Product One envelope

The uncompressed byte stream has one fixed schema:

```text
magic[16] = "MORIA-P1-DELTA\0"
seed: u64 little-endian
generation_fingerprint: [u8; 32]
material_fingerprint: [u8; 32]
persistence_fingerprint: [u8; 32]
truth_revision_at_snapshot: u32 little-endian
changed_brick_count: u32 little-endian
changed_voxel_count: u64 little-endian
payload_byte_count: u64 little-endian
payload: sorted brick records
payload_crc32: u32 little-endian
```

Each brick record stores signed brick coordinates, codec tag (`sparse` below 25% occupancy or `dense_rle` at/above 25%), record length, then sorted local-index/value entries or deterministic runs. Values are exactly three bytes `(material, density, state)`. All counts/lengths are bounds-checked before allocation. The complete stream is compressed as one Zstandard frame at `PersistenceConfig.zstd_level = 9`, checksum enabled, content-size field enabled, zero worker threads, and no dictionary. The payload CRC uses the standard CRC-32/ISO-HDLC polynomial exposed by `crc32fast`. These flags and the locked Zstandard crate version are part of the persistence fingerprint; there is no runtime codec negotiation.

The `P1` magic identifies this product’s only accepted schema; the implementation contains no version field, alternate decoder, migration dispatch, or upgrade writer. A mismatch is `UnsupportedSave` and requires deleting the slot or using the exact matching Product One build/preset. `generation_fingerprint` includes the compiled `BASE_SAMPLER_REVISION` plus both exact base-truth asset digests (`vegetation_templates.ron` and `ruin.ron`); together, generation/material fingerprints prevent deltas from being applied to a different procedural base, while `persistence_fingerprint` prevents a codec/layout mismatch. The revision is not duplicated as an envelope field because it is normatively the first member of `generation_fingerprint`. This is compatibility rejection, not save migration/versioning.

## Save transaction

1. On `SaveSingleSlot`, capture the current revision and clone/retain an immutable sorted delta snapshot under a short read lock.
2. Serialize and compress on the IO task pool while exploration and later edits continue.
3. Write to a sibling temporary file created with exclusive replacement intent, flush file contents, then atomically rename over the single slot; sync the containing directory where supported.
4. Emit `SaveCompleted` with snapshot revision and final compressed file size. Later in-memory edits remain unsaved and the UI reports the last saved revision.
5. On any serialization, space, flush, or rename error, remove the temporary file when possible, preserve the previous complete slot, and emit `Failed`.

No write is reported successful before durable replacement steps complete. Concurrent save requests return `Busy`; normal streaming/meshing is irrelevant because derived state is excluded.

## Load transaction

1. Read and decompress with an explicit maximum uncompressed-size/count limit derived from the finite region (`<= 16,384,000,000` possible voxel coordinates is never preallocated; individual declared lengths are validated incrementally).
2. Validate magic, seed, `generation_fingerprint`, `material_fingerprint`, and `persistence_fingerprint` against the boot-verified preset before decoding any brick record, sampling a base coordinate, or applying a delta; then validate coordinate bounds, codec tags, sorted uniqueness, known material IDs, density/state canonical rules, declared counts/length, CRC, and end-of-file.
3. Only after compatibility succeeds, sample the deterministic base for every entry and reject entries equal to base (nonminimal/corrupt) or invalid Product One state. Construct a complete replacement `EditDeltaStore` off-thread.
4. With player mutation suppressed, atomically replace the old store, increment the live truth revision, invalidate the union of old/new edited bricks and one-voxel-halo neighbours, and make collision queries see the new revision.
5. Keep/relocate the player according to `states.md`, rebuild the current visible derived artifacts through mutation-priority jobs while traversal remains suppressed, atomically reveal the new revision, and then emit `LoadCompleted`/resume control. Derived content is never loaded from disk.

A missing file at startup means an empty delta set. An explicit F9 with no file returns `NotFound` and leaves current truth unchanged. Any failed validation leaves the current truth and previous on-disk file untouched.

## Exactness and size contracts

For every coordinate, `load(save(seed, deltas), same seed/config).sample(coord)` must equal the pre-save current `VoxelValue` byte-for-byte. The sorted canonical encoder must produce identical uncompressed bytes for identical seed and generation/material/persistence fingerprints and deltas regardless of mutation order or map insertion order; the stored snapshot revision may be normalized out in the canonical payload hash test.

The acceptance save is the post-run delta set from the exact 512-operation carve storm in `benchmarks.md`. Its final compressed file must be below 50,000,000 bytes and must reload to the same canonical delta hash and sampled world. The report includes compressed bytes, uncompressed payload bytes, changed bricks, changed voxels, and compression ratio. This makes “heavily defaced” reproducible rather than subjective.

## Tests

- Empty, sparse, dense-run, cross-brick, object-backed, and boundary-coordinate round trips.
- Equal final worlds reached through different edit orders produce equal canonical payloads.
- Edits reverted exactly to base disappear from the save.
- Mixed solid/water/air dig fixtures prove water bytes never change, `changed_voxels` excludes fluid/empty samples, the canonical payload contains no water-coordinate delta, and reload preserves the generated water byte-for-byte.
- Wrong seed, generation/material/persistence fingerprint, unknown material, nonzero Product One state, duplicate/unsorted index, out-of-bounds coordinate, length overflow, truncation, bad CRC, and decompression-bomb declarations reject without changing current truth. A dedicated fixture changes only `BASE_SAMPLER_REVISION` while seed, config, registry, and asset bytes remain identical; it confirms `generation_fingerprint` changes and instruments the sampler/delta-store swap to prove rejection occurs with zero base-sampler calls and zero delta applications. A one-byte `vegetation_templates.ron` fixture and an equivalent ruin-byte fixture prove the same pre-sampling rejection for asset-only changes.
- Injected write/flush/rename failures retain the prior slot and never emit success.
- Headless save/load unloads and rebuilds caches, then asserts queries rather than meshes.
- The acceptance carve-storm fixture checks exact hash, `<50 MB`, and reload equivalence in release validation.
