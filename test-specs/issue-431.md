# Issue 431 — Implement persistence provider completion protocols

References: `content-persistence.md` TECH-043; issue M-039.

## Boundary contract

- External stores implement exact `put_blob`, `get_blob`, `load_manifest`, and `commit_manifest` signatures with non-clone Moria-owned `StoreSink`, `LoadSink`, `ManifestLoadSink`, and `CommitSink`.
- All callback cells/bytes/operation records are reserved before invocation. `get_blob` uses `expected_bytes: Some(n)`; only initial key lookup uses `None`.

## Multi-system scenarios

- Store/load exact known-length blobs and unknown-length framed manifests; verify identity echo, BLAKE3, cursor length, checksum, declared framing, and no trailing bytes.
- Commit a manifest only after blobs; later `load_manifest(key)` observes complete exact bytes or provider failure, never a partial/older value.
- Route checkpoint, restore, recovery, and shutdown checkpoint to the exact request store ID.

## Failure propagation and concurrency

- Short/long known loads, over-max manifest writes, empty/truncated/bad-checksum/trailing manifest, wrong digest/key/durable byte count, duplicate/drop/cancel/late completion, and provider failure terminate the owner without publication.
- Race cancellation with write/terminal completion; one terminal result wins and buffers remain pinned until active copying/provider use ends.
- Store failure never falls through to the configured default or another store. Retry requires a new owning operation and fresh sink.
- A descriptor lacking atomic manifest visibility or adequate maxima fails at configuration freeze.
