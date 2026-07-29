# Content, persistence, replay, and reclamation

Base content belongs to the consumer. Moria makes its identity and bounded
materialization explicit, stores only scars and continuation state, and fails
restore unless exact reconstruction is provable.

## Base content

### TECH-041 — Exact base-authority contract

Implements: REQ-004, REQ-008, REQ-014, REQ-020, REQ-021

Every genesis volume selects one:

```rust
pub enum BaseAuthority {
    Uniform {
        cell: CellWire,
        identity: ContentDigest,
    },
    Reconstructable {
        lineage: ContentLineage,
        manifest_root: ContentDigest,
        source: BaseContentSourceId,
    },
    Bundled {
        lineage: ContentLineage,
        manifest_root: ContentDigest,
        store: ContentBlobStoreId,
    },
}
```

`lineage` is a semantic family label; `manifest_root` is the exact identity.
Matching lineage alone is never sufficient. The manifest is a canonical
Merkle radix tree from `(base authority id, local brick coordinate)` to a
content digest or uniform cell. A consumer-owned generator may implement
`BaseContentSource`, but its algorithm is opaque and not shipped by Moria.

```rust
pub trait BaseContentSource: Send + Sync + 'static {
    fn descriptor(&self) -> BaseSourceDescriptor;
    fn request(
        &self,
        request: BaseBrickRequest,
        completion: BaseBrickCompletion,
    );
}
```

A request is bounded, names the expected manifest subtree/brick digest, and
returns canonical brick bytes or a typed unavailable/failure result. Moria
validates domain, cell invariants, exact byte length, and digest before
residency. Wrong bytes fail the region. A source may retry only after an
explicit consumer retry; timing never substitutes content.

If a source cannot promise reconstructability, the consumer must use
`Bundled`, whose store contains every base blob referenced by the manifest.

### TECH-042 — Materialization and scar overlay

Implements: REQ-001, REQ-004, REQ-009, REQ-014, REQ-016, REQ-018

Materialization pins an immutable root, resolves each requested brick's base
digest, obtains and verifies its canonical base payload, then overlays at most
one scar leaf from that root. The resulting complete brick enters the
noncanonical resident cache keyed by `(world root, volume, brick, source
digest)`. It becomes `Ready` only after GPU upload and directory publication
complete.

Scar leaves always contain a complete replacement brick or uniform cell; they
never depend on edit order during restore. Mutation compares each new complete
brick with the verified base payload. Equal bricks remove the scar key;
different bricks replace it. This keeps untouched/homogeneous space cheap and
makes scar restore order-independent.

A committed dirty scar remains reachable through canonical roots regardless of
resident-cache interest. Eviction may remove a rematerialized cache brick but
not the scar leaf, rollback node, dirty journal reference, or base identity.

## Durable checkpoint format

### TECH-043 — Checkpoint store and atomic commit

Implements: REQ-005, REQ-014, REQ-015, REQ-021

Persistence is store-neutral:

```rust
pub trait CheckpointStore: Send + Sync + 'static {
    fn put_blob(&self, digest: BlobDigest, bytes: OwnedBytes, done: StoreSink);
    fn get_blob(&self, digest: BlobDigest, limits: BlobLimits, done: LoadSink);
    fn commit_manifest(
        &self,
        key: CheckpointKey,
        manifest: OwnedBytes,
        done: CommitSink,
    );
}
```

Calls are asynchronous callbacks with explicit byte limits. Blob keys are
BLAKE3 digests of uncompressed canonical bytes; zstd is a storage encoding
whose version/options are recorded and whose decode has a maximum output.
Deduplication is optional and cannot change semantics.

The native reference store writes content-addressed blobs first, verifies
length/digest, fsyncs them, writes a manifest to a unique temporary name,
fsyncs it, atomically renames to the checkpoint key, and fsyncs the parent
directory. Only a committed manifest makes a checkpoint visible. A crash or
failure before rename leaves ignorable orphan blobs, never a partial
checkpoint. Store adapters must document equivalent atomicity or report
`UnsupportedAtomicCommit`.

### TECH-044 — Versioned checkpoint manifest

Implements: REQ-001, REQ-014, REQ-017, REQ-028, REQ-032

`moria-checkpoint-v1` is a canonical manifest containing:

- product, canonical encoding, arithmetic, hash, transition, and persistence
  schema digests;
- world/genesis ID, exact base lineage and manifest roots, material registry;
- confirmed and durable tick, world root hash, per-volume revisions;
- live and retired volume identities, domains, kinds, placements, and scar
  root hashes;
- content-addressed scar radix nodes and brick blobs;
- `next_volume_serial`, simulation-domain normalized intervals, and declared
  participant RNG commitments;
- participant IDs/contracts/input schemas/strategies/commitments and snapshot
  blob digests where applicable;
- replay prefix/suffix digests sufficient for the configured recovery point;
- completeness counts, total uncompressed bytes, and manifest checksum.

Derived meshes, dressing, resident base-cache entries, physical slots, Bevy
entities, telemetry, timings, and receipt IDs are forbidden. Unknown required
fields/version tags fail restore. The decoder enforces configured node, blob,
depth, and byte bounds before allocation.

### TECH-045 — Checkpoint snapshot isolation

Implements: REQ-005, REQ-014, REQ-017, REQ-018

A checkpoint request names an exact confirmed `(tick, root_hash)` and a maximum
readback/store budget. Admission pins that immutable root and participant
frontier. New ticks may confirm concurrently; they remain dirty relative to
the checkpoint.

Checkpoint traversal follows scar and metadata nodes only. GPU blobs are copied
through bounded staging slots in lexicographic logical-key order, mapped,
decoded, digest-verified, and handed to the store. It never scans untouched
base cells or serializes derived state. Completion reports exactly:

```text
checkpoint key, durable tick/root, per-volume revisions,
participant commitments, scar/node/blob counts, compressed/uncompressed bytes
```

Failure keeps the root and all later dirty truth live, reports whether orphan
blobs may exist, and publishes no manifest. Cancelling before a submitted
readback stops new batches; submitted batches drain before permits/root pins
release.

## Restore

### TECH-046 — Durable restore protocol

Implements: REQ-008, REQ-014, REQ-021, REQ-029, REQ-035

Restore is world construction, not a post-genesis mutation:

1. load and bounded-decode the manifest;
2. verify all contract versions and manifest digest;
3. match material, base lineage **and exact manifest roots**, volume sources,
   qualification tuple, and participant registrations;
4. load every referenced scar/node/snapshot blob, enforce bounds, decompress,
   and verify its uncompressed digest;
5. rebuild new-generation GPU pages and radix roots from logical keys;
6. ask snapshot participants to restore and reconstructible participants to
   prove their declared starting commitment;
7. recompute every root bottom-up and compare the saved world root;
8. publish the restored frontier and readiness context, then permit the next
   tick.

The live world remains absent until all steps pass. Corruption, missing IDs or
blobs, unsupported contract, wrong lineage/root, content-source inability,
participant mismatch, resource exhaustion, unqualified backend, or hash
disagreement fails the entire restore. Migration/rebase is a separate
consumer-authored tool that produces a new genesis identity; Moria does not
guess or mutate an old checkpoint in place.

## Replay and rollback

### TECH-047 — Replay record and divergence artifact

Implements: REQ-032, REQ-034, REQ-035, REQ-038, REQ-043

`moria-replay-v1` is an appendable sequence:

```text
header {
  genesis bytes/digest, contract digests, participant descriptors,
  qualification identity, starting frontier
}
tick record {
  tick, sealed TickBatch bytes/digest, canonical outcome bytes/digest,
  participant commitments, expected world root hash
}
```

Records are length-prefixed, checksummed, bounded, and committed only after the
tick confirms. Presentation and timing are excluded. Replay feeds the same
`submit_tick` path with a replay permit; no internal state mutation or expected-
hash override exists.

At the first mismatch, replay stops before publishing the divergent candidate
and writes `moria-divergence-v1` containing genesis/contract/fixture digests,
backend qualification context, earliest tick, input prefix through that tick,
expected/actual root, outcome and participant commitments, changed logical
keys, and bounded canonical leaf/node byte comparisons. The artifact never
calls a self-reported pass authoritative; an independent tool compares bytes.

### TECH-048 — Rollback correction transaction

Implements: REQ-029, REQ-030, REQ-035, REQ-037, REQ-040, REQ-043

`request_correction(target, replacement_batches)` accepts only a retained
confirmed frontier and a complete contiguous input sequence from
`target.tick + 1` through the desired corrected present. It reserves replay
bytes, output roots, participant resources, and pins the original live and
target roots before starting.

Moria installs the target in a private replay context, restores each
participant strategy, and sends replacement batches through the ordinary
transition. Intermediate roots remain private and do not emit consumer
observations or presentation work. Expected hashes, when supplied, are checked
at each tick. Failure discards private roots and keeps the original live
frontier. Success atomically replaces the live frontier with the final
corrected root, emits one correction observation plus canonical outcome range,
and schedules only the final accumulated dirty regions for derived rebuild.

Original frontiers and participant state remain pinned until success/failure
and all GPU readers complete. Target outside the window, missing state,
resource bound, participant failure, content mismatch, or divergence is
terminal for that correction and never advances the world.

### TECH-049 — Replay/log and checkpoint bounds

Implements: REQ-015, REQ-018, REQ-021, REQ-029, REQ-032

The in-memory confirmed log retains at least the rollback window and at most
`max_log_ticks` (default 256), `max_log_bytes` (default 256 MiB), and the newest
durable checkpoint suffix. The consumer must attach a replay sink or checkpoint
before bounds prevent safe continuation. At the boundary, `reserve_tick`
returns `PersistenceBackpressure`; Moria never silently drops the only recovery
record.

Checkpoint traversal defaults to 16 MiB mapped bytes and 64 MiB store writes
in flight, with three staging slots. A manifest may reference at most the
configured scar nodes/bricks and 4 GiB uncompressed data in v1; lower consumer
limits are honored. Counts/offsets use checked `u64`, while individual wire
sequences remain `u32` bounded.

Rollback capacity is tick- and byte-bounded. A tick whose worst-case COW state
would exceed the canonical budget receives `NoAdvance(CanonicalBudget)`;
runtime pressure does not evict the required 20 confirmed frontiers.

## Reclamation and dirty truth

### TECH-050 — Dirty tracking and safe eviction

Implements: REQ-004, REQ-014, REQ-015, REQ-018

Each confirmed root carries a persistent dirty-key set relative to the newest
durable checkpoint. It is updated by stable merge with changed scar and
metadata keys. Checkpoint success advances the durable root and computes the
remaining dirty set from later confirmed roots; it does not clear later keys.

A canonical node/brick is reclaimable only when absent from:

- the live root;
- every retained rollback frontier;
- the newest durable/recovery root still being read;
- active replay/correction/query/checkpoint root pins;
- participant artifact leases;
- submitted GPU work.

A resident base-cache brick additionally requires no interest/admitted-use
pin. Presentation can be discarded independently. Unpersisted scars are not
lost when their detailed resident brick retires because their immutable scar
leaf remains in all required roots. Shutdown reports every dirty root if final
persistence fails.
