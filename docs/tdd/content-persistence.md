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
- `next_volume_serial` and simulation-domain normalized intervals;
- participant IDs/contracts/input schemas/strategies/commitments, complete RNG
  descriptors and current RNG-state commitments, and snapshot blob digests
  where applicable;
- for every reconstructible participant, its required inclusive replay tick
  range and a sorted list of content-addressed
  `moria-checkpoint-replay-v1` chunk descriptors `{first_tick, last_tick,
  record_count, uncompressed_bytes, blob_digest}` whose exact bytes cover the
  union of all such ranges without gaps or overlap;
- replay prefix/suffix digests binding that covered range to the public
  `moria-replay-v1` sequence;
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

Canonical substrate traversal follows scar and metadata nodes only; participant
snapshots use the separate export path below. GPU blobs are copied
through bounded staging slots in lexicographic logical-key order, mapped,
decoded, digest-verified, and handed to the store. It never scans untouched
base cells or serializes derived state. Completion reports exactly:

```text
checkpoint key, durable tick/root, per-volume revisions,
participant commitments, scar/node/blob and participant-snapshot-blob counts,
replay-record/chunk counts, compressed/uncompressed bytes
```

The pinned participant frontier is part of traversal. For every
`PerTickSnapshot` participant in `ParticipantId` order, checkpoint admission
reserves its declared `max_bytes` against both the participant-export pool and
the checkpoint's total byte budget. Moria invokes the public CPU/GPU snapshot
export operation on the exact pinned state token. Export state is:

```text
Pinned -> ExportReserved -> Exporting -> BytesVerified
       -> BlobPutPending -> BlobDurable -> ManifestReferenced
       \-> Failed
```

The export result is bound to participant ID/contract, pinned tick/root,
participant commitment, snapshot length/digest, and device generation where
applicable. Moria checks all fields, exact length, and BLAKE3 digest before
calling `CheckpointStore::put_blob`; the store completion must confirm the
same digest. At most three aggregate checkpoint staging slots and 64 MiB of
store writes, including scar, participant, and replay bytes, are in flight.
Snapshot participants cannot provide an external locator instead of bytes.
Reconstructible participants contribute descriptor, commitment, and required
replay range but no snapshot blob. Moria takes the union of those ranges,
pins every exact confirmed `moria-replay-v1` tick record, and rejects
checkpoint admission if any record is absent or a participant's range exceeds
its declared `max_replay_ticks`. Chunks contain at most 64 consecutive records
and at most 8 MiB uncompressed; an oversized individual record is one
single-record chunk only if it fits both the checkpoint request and
`max_log_bytes`, otherwise admission fails. Chunk encoding retains the exact
length-prefixed tick-record bytes and adds only the
`moria-checkpoint-replay-v1` version, first/last tick, record count, and
checksum. It never reconstructs records from digests.

Replay chunks use the same bounded durable state machine:

```text
LogPinned -> BytesReserved -> ChunkEncoded -> BytesVerified
          -> BlobPutPending -> BlobDurable -> ManifestReferenced
          \-> Failed
```

Encoding and storage reserve the chunk's declared uncompressed bytes before
work starts. Moria verifies tick continuity, each embedded record checksum and
digest, the chunk length, and its BLAKE3 blob digest before
`CheckpointStore::put_blob`; store completion must confirm that digest.

`commit_manifest` is not called until every scar/node/metadata,
participant-snapshot, and required replay chunk has reached `BlobDurable`. The
manifest references exactly those verified digests. Any participant export,
generation, mapping, replay gap/encode, store, size, or digest failure enters
`Failed`, publishes no manifest, and cannot report that participant frontier
durable. Submitted exports and puts drain for lifetime safety; the root,
participant tokens, and log records remain pinned until all reservations are
released.

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
4. load every referenced scar/node/snapshot/replay-chunk blob, enforce bounds,
   decompress, and verify its uncompressed digest; validate each replay
   chunk's tick interval, embedded record checksum/digest, exact continuity,
   and prefix/suffix digest against the manifest before exposing a
   `ParticipantReplayLease`;
5. rebuild new-generation GPU pages and radix roots from logical keys;
6. ask snapshot participants to create staged tokens from verified snapshot
   bytes and reconstructible participants to create staged tokens from only
   the restored canonical frontier plus their manifest-declared replay range,
   while proving every reproduced intermediate commitment and the saved final
   participant/RNG-state commitments;
7. recompute every root bottom-up and compare the saved world root;
8. publish the restored frontier and readiness context, then permit the next
   tick.

The restored GPU root and all participant tokens remain in one private genesis
bundle until all steps pass; adapter calls never mutate a live singleton.
Publishing that bundle is one pointer swap. Corruption, missing IDs, replay
gaps, missing blobs, unsupported contract, wrong lineage/root, content-source
inability, participant mismatch, resource exhaustion, unqualified backend, or
hash disagreement fails the entire restore. Migration/rebase is a separate
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

Moria creates a private replay context from the target root and target
participant tokens. Snapshot adapters return new staged tokens from the pinned
target snapshot; reconstructible adapters return new staged tokens after
replaying the bounded prefix. Each replacement tick consumes only the prior
private token and returns the next one through the ordinary transition.
Intermediate roots and participant tokens remain private and do not emit consumer
observations or presentation work. Expected hashes, when supplied, are checked
at each tick. Failure discards private roots and keeps the original live
frontier. Success atomically replaces the live frontier with the final
corrected `FrontierBundle`, emits one correction observation plus canonical
outcome range, and schedules only the final accumulated dirty regions for
derived rebuild.

Original frontiers and participant tokens remain pinned until success/failure
and all GPU readers complete. On failure, staged CPU tokens drop immediately
after callback closure and staged GPU tokens enter generation-tagged retire
queues until their last submission completes; no participant restore-back call
is needed or permitted. Target outside the window, missing state,
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
in flight, with three staging slots shared by scar, participant, and replay
blobs.
The sum of declared snapshot maxima must fit the configured checkpoint byte
budget and the 64 MiB per-frontier compiled maximum; otherwise genesis or
checkpoint admission fails before export. A manifest may reference at most the
configured scar nodes/bricks and 4 GiB uncompressed data in v1; lower consumer
limits are honored. Counts/offsets use checked `u64`, while individual wire
sequences remain `u32` bounded.

The union of reconstructible participant ranges must also fit
`max_log_ticks`, `max_log_bytes`, the checkpoint request's total byte budget,
the 4 GiB manifest maximum, and the three-slot/64 MiB in-flight limits.
Replay-record and replay-chunk bytes count in checkpoint progress, completion,
telemetry, manifest completeness totals, and recovery-anchor accounting. A
checkpoint is a recovery anchor for a reconstructible participant only after
every required chunk and then the manifest are durable. Once that anchor is
visible, older in-memory records may be released only if they are not required
by rollback, correction, another participant range, or an attached replay
sink.

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
