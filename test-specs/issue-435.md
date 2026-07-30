# Issue 435 — Implement the native atomic checkpoint store

References: `content-persistence.md` TECH-043 native reference-store protocol; issue M-040.

## Boundary contract

- Blob keys are digests of uncompressed canonical bytes; manifest keys are opaque `CheckpointKey`s. `commit_manifest` provides atomic whole-value visibility.
- Successful commit sequence is blobs verified/fsynced, unique temporary manifest written/fsynced, atomic rename, then parent-directory fsync.

## Multi-system and crash scenarios

- Interrupt before/after each blob write/fsync, temp creation/write/fsync, rename, and directory fsync. After restart, `load_manifest` returns the complete committed manifest or `NotFound`, never partial bytes.
- Leave orphan blobs/temp files at every precommit crash point and prove they do not make a checkpoint visible or alter a later valid commit.
- Recommit the same key and same manifest digest idempotently; a different digest for an existing key returns `Corrupt` and preserves visible truth.

## Failure propagation

- Permission, capacity, unavailable filesystem, short write, fsync failure, rename failure, and checksum/digest mismatch produce the exact store failure and no successful manifest completion.
- Concurrent commits to one key may yield one exact winner or identical idempotent success; no mixed manifest is observable.
- Reads during commit observe old complete value/`NotFound` until atomic rename, then new complete value.
- If platform atomicity cannot meet the contract, descriptor/operation reports `UnsupportedAtomicCommit`.
