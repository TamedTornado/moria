# Issue 116 — Encode and decode the deterministic delta format

References: `docs/tdd/data-model.md §Edit delta set and save file`.

## Properties that must hold

- For every canonical delta map, uncompressed bytes exactly follow magic, little-endian identity/counts, lexicographic bricks/local indices, 4-byte voxels and SHA-256, then zstd level 3; repeated encodes are byte-identical.
- For every successful decode, all four voxel bytes round-trip and only non-base deltas exist; derived/player/camera/time/benchmark data is absent.

## Entity configurations to test

- Empty, one voxel, 4096 voxels/brick, multiple negative/positive brick coords, all material/density/state/flags byte values allowed by contract.
- Corrupt each field: magic, seed/digest, brick count/coord, zero/4097 voxel count, unsorted/duplicate index, invalid material, checksum, truncated/corrupt zstd.

## Edge cases and type boundaries

- Every malformed/identity/bounds/order/count/material/checksum/compression input returns its typed LoadError and no partial map.

## Error paths

- Checked u32/u16 size overflow fails encoding/decoding before allocation or output.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

