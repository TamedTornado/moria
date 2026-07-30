# Issue 471 — Page GPU storage and publish resident directory entries

References: `gpu-runtime.md` TECH-033; issue M-061.

## Properties

- Every buffer page/effective binding fits granted allocation and storage-binding limits; offsets meet `min_storage_buffer_offset_alignment`.
- Working buffers use required STORAGE/COPY usages; CPU readback uses a distinct `MAP_READ|COPY_DST` staging buffer and never overlaps GPU access while mapped.
- Directory records transition `EMPTY/TOMBSTONE -> RESERVED -> OCCUPIED` through separate ordered reserve/write/dedup/validate/publish phases; lookup consumes only OCCUPIED and probes at most 32.

## Entity configurations

- Exercise baseline/max-lowered pages, exact last record/range, near-full/full tables, collision-heavy keys, duplicate contenders, tombstone-heavy chains, deletion reachability, and probe 32/33.
- Compare logical lookups across different page/slot layouts.

## Edge and error paths

- Reject misalignment, oversized effective range, allocation/binding overflow, bad page-table handle, mapped-buffer reuse, probe exhaustion, table full, duplicate publication, or generation mismatch.
- Failure after reservation but before publication leaves no visible partial record and reclaims/repairs RESERVED through the declared phase.
- Directory miss changes readiness only; it cannot be hashed or interpreted as empty canonical matter.
