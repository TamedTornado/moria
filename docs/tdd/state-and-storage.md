# State, Sparse Storage, and Atomic Publication

## Coordinate model

Each volume has a finite half-open local `CellAabb`. No axis is privileged.
Cells are addressed by checked `i32` triples. Brick coordinate and within-brick
coordinate use Euclidean division, so negative cells map consistently:

```text
brick = cell.div_euclid(8)
local = cell.rem_euclid(8)     // 0..7 on every axis
linear = local.x + 8 * (local.y + 8 * local.z)
```

All size, product, byte-offset, and world-to-local conversions use checked
arithmetic. A query point exactly on a cell boundary follows half-open
ownership. A point exactly on a volume-domain maximum is outside.

`RigidPlacement` maps local metric coordinates to world coordinates. The cell
at integer coordinate `c` occupies local
`[c * cell_size, (c + 1) * cell_size)`. World-space collision transforms the
query into volume local space; normal results transform back with the rigid
rotation.

Large-world rebasing is a consumer/Bevy camera concern. A placement translation
must be representable as finite `f32`; Moria does not silently lose or clamp
precision.

## Material sample and occupancy

The portable host/WGSL layout is:

```rust
#[repr(C)]
pub struct MaterialSample {
    material_le: u16,
    coverage: u8,
    flags: u8,
}
```

`material == 0` is canonical empty and requires `coverage == 0`. Consumers may
register exactly 65,535 nonempty materials using IDs 1..=65535; canonical empty
is an additional reserved sample, so the complete runtime ID space contains
65,536 values. A nonempty material may have coverage 0..=255. V1 occupancy is
true only at coverage 128..=255. Coverage is the scalar field used by surface
derivation; it is not opacity or health.

V1 flags must be zero in content, commands, checkpoint data, and extension
effects. Reserving the byte allows a later contract-version migration without
accidentally interpreting current data.

One cell has one material ID. Interfaces between two occupied materials affect
surface attributes but are not an empty boundary. Presentation selects the
material of the higher-coverage endpoint with a stable ID tie-break.

## Brick representations

Every logical brick resolves to one of:

- `ImplicitEmpty` for out-of-domain or canonical empty base;
- `Homogeneous(sample)` stored entirely in the page entry;
- `Detailed { segment, slot, generation }` referencing 512 samples;
- `Unavailable(reason)` in control metadata only; it is never a GPU material
  value.

Detailed slots are 2,048 bytes. GPU allocation is segmented so one binding
never exceeds the adapter limit. A segment has an immutable capacity and a
generation-tagged free list owned by storage.

Base content can be implicit/homogeneous without allocating a slot. A mutation
of one cell in a homogeneous brick allocates one detailed copy-on-write slot.
When a committed detailed brick becomes homogeneous, compaction may publish a
homogeneous page version and retire the slot.

## Page table

The page key is `(VolumeId index, BrickCoord)`. The GPU table uses fixed-
capacity open addressing with robin-hood insertion built by bounded prepare
work. Each key points to the newest immutable `PageVersion`:

```text
PageVersion {
    visible_at: VolumeRevision,
    representation: homogeneous | detailed slot,
    occupancy_summary: empty | full | mixed,
    previous: optional version index,
    scar_state: clean | dirty | durable(checkpoint generation),
}
```

Lookup for snapshot revision `R` follows `previous` until
`visible_at <= R`. The configured `max_versions_per_brick` is a pressure
threshold, not a correctness truncation rule. Reaching it delays admission and
schedules reclamation; if no version can be reclaimed, admission fails with
`BudgetExhausted::PinnedVersions`.

Hash-table load may not exceed 70%. Admission reserves a new key/version entry
before content/mutation work. Probe count is capped by table capacity and
reported; failure is explicit. Resizing is an offline copy into a newly
reserved table followed by a world-directory generation swap after all
readers of the old table complete. It never mutates the active table in place.

## Revisions and snapshots

Each volume starts at revision 1 when its directory entry is committed.
Matter and placement changes allocate `current + 1` only after all resources
are reserved. There are no speculative public revisions and no reuse.

A GPU snapshot consists of:

```text
SnapshotToken {
    device_generation,
    directory_generation,
    [(volume_runtime_index, committed_revision, placement)],
    last_submission_using_snapshot,
}
```

World-scope snapshot acquisition sorts volumes by stable key, then reads their
revision/placement gates in one ordered compute pass. The result is immutable
for that query/derivation. It does not imply a simultaneous cross-volume
transaction; it records exactly what was captured.

CPU result metadata is published only after its readback maps and decodes. A
snapshot pin ends after the GPU submission that consumed it completes; CPU
results contain copied values and do not pin GPU versions.

## Matter mutation transaction

For a command proposed at revision `N + 1`:

1. Revalidate the handle, current revision/precondition, domain, material IDs,
   and required readiness.
2. Enumerate unique affected bricks and exact maximum output bytes without
   dispatch.
3. Reserve all new detailed slots, page versions, hash keys, scar capacity,
   observation metadata, and submission/completion records. If any reservation
   fails, no GPU write begins.
4. For each affected brick, copy the version visible at `N` into an
   unreferenced slot or synthesize it from homogeneous/base input.
5. Apply the complete patch to only those unreferenced slots. Compute exact
   occupancy summaries and scar records.
6. Validate output sentinels, material IDs, target coverage, page links, slot
   generations, counts, and a transaction-wide error flag.
7. Insert prepared page versions tagged `N + 1`. Readers holding revision `N`
   ignore them and follow previous links.
8. A separate publication dispatch, ordered after all prepare/validate
   dispatches in the same command buffer, reads the transaction-wide status,
   atomically compares the volume gate against `N`, and writes `N + 1`. This
   does not rely on cross-workgroup barriers. A failed validation or compare
   sets the transaction failure flag and does not publish.
9. Queue completion copies the small status block to staging. On successful GPU
   completion, the control plane resolves the receipt and appends the
   observation. On compare failure, an ordered cleanup pass restores every
   affected hash head only if it still names this transaction, then releases
   prepared versions/slots. The volume queue cannot prepare its next command
   until that cleanup completes.
10. Dirty scar versions remain pinned until represented by a durable
    checkpoint or superseded by another retained dirty version.

The publication write is the linearization point. Queries encoded before it
see `N`; queries encoded after it see `N + 1`. Command submissions for a volume
are serialized, so a normal command cannot race the compare; the compare is
still required to catch stale extension work and invariant defects.

No ordinary error can occur after the publication gate while leaving the world
usable but the receipt failed. If queue/device execution becomes unknowable,
the device generation becomes unavailable and recovery is required.

## Placement publication

Placement is a versioned volume-directory record. Move reserves a directory
version and publishes its revision/placement gate together in one compute
write after validation. It does not touch page entries.

A query snapshot always couples placement with the same volume revision.
Collision and presentation installation use that pair. Existing presentation
geometry may be rigidly transformed to a new placement without remeshing, but
its entity tag advances only after the placement receipt commits.

## Occupancy hierarchy

Every detailed brick stores an exact 512-bit occupancy mask plus
`empty/full/mixed`. A coarser GPU hash map stores OR summaries over 8×8×8
brick groups for ready regions. It is an acceleration structure:

- `empty` must be exact;
- stale summaries may return nonempty and cause extra traversal;
- a summary may never produce a false empty;
- a query falling outside current summary coverage traverses pages or reports
  unavailable, never empty.

Mutation writes brick masks as part of the prepared version and updates coarse
summaries after publication. Until the update is installed, affected parents
are conservatively nonempty. Collision correctness therefore does not wait for
coarse compaction.

## Residency and eviction

Interest is converted to unique brick requirements with capability bits and
maximum accepted brick count. A brick is not evictable while any of these hold:

- an interest lease requires its authoritative capability;
- an admitted command/query references it;
- a GPU snapshot pin can resolve to its slot;
- a presentation job, scheduled behavior tick, or asynchronous extension job
  has not reached GPU completion;
- it has a dirty scar not retained elsewhere or durably checkpointed;
- it is being materialized, compacted, or retired.

Eviction proceeds:

1. mark the page/version non-admissible for new snapshots;
2. wait for its last GPU reader submission;
3. ensure base plus durable/retained scar can reconstruct it;
4. publish a cold lifecycle transition;
5. remove reclaimable page versions and return slots after completion.

Withdrawal of interest only makes this process eligible. Least-recently-used
eligible bricks within priority bands are selected; scars and active operations
override recency.

## Default and hard resource limits

Every value is configurable downward/upward within adapter and integer limits.
The validated config records effective values. These defaults are engineering
starting points, not universal performance promises.

| Limit | Default | Hard v1 request maximum |
| --- | ---: | ---: |
| Consumer-registered nonempty materials | 4,096 | 65,535 (plus reserved empty ID 0) |
| Material metadata per registration / aggregate | 4 KiB / 16 MiB | 1 MiB / 1 GiB |
| Concurrently live volumes | 1,024 | 65,535 |
| Lifetime volume records (live + tombstones) | 4,096 | 65,535; each owns one exact 1..=96-byte UTF-8 name |
| Active interest leases | 64 | 4,096 |
| Bricks per interest | 4,096 | 65,536 |
| Detailed resident bricks | 32,768 (64 MiB samples) | adapter/config bound |
| Page keys | 131,072 | adapter/config bound, <=70% occupied |
| Page versions | 262,144 | adapter/config bound |
| Versions per brick before pressure | 8 | 64 |
| Dirty scar bricks | 32,768 | configured and persistence-bound |
| Command records / payload bytes | 1,024 / 64 MiB | configured |
| Cells / bricks per matter command | 32,768 / 512 | fixed v1 maxima |
| Patch payload | 16 MiB | fixed v1 maximum |
| Query records / reserved result bytes | 256 / 32 MiB | configured |
| Cells per region read | 262,144 | fixed v1 maximum |
| Candidate bricks / cells per collision traversal | 8,192 / 65,536 | fixed v1 maxima |
| Hits per trace/overlap/sweep | 4,096 | fixed v1 maximum |
| World-scope volumes per query | 256 | fixed v1 maximum |
| Observation ring facts / payload bytes | 4,096 / 32 MiB | configured; payload includes one 128-byte append-time filter envelope per fact |
| Subscribers / volumes per filter | 64 / 256 | configured |
| In-flight staging maps / bytes | 8 / 32 MiB | configured |
| Content requests / bricks per request / response bytes | 64 / 512 / 32 MiB | configured; count and worst-case response bytes plus the exact-length Moria-owned sink are atomically reserved before callback invocation; source identity is borrowed and the only by-value error diagnostic is fixed inline, so no variable result ownership crosses the port |
| Persistence requests / staged bytes | 8 / 64 MiB | configured |
| Extraction records / bytes per frame | 2,048 / 32 MiB | configured |
| Presentation jobs | 1,024 | configured |
| Presentation artifacts / dirty records | 16,384 / 16,384 | configured |
| Vertices / indices per brick artifact | 2,048 / 12,288 | fixed v1 maximum |
| Dressing styles / device instances | 256 / 1,048,576 | configured |
| Scheduled behavior engines / order edges | 16 / 64 | configured and builder-validated DAG |
| Scheduled behavior view volumes / bricks / cells | 256 / 8,192 / 262,144 | configured sum of isolated per-participant views from one pinned frontier |
| Scheduled behavior CPU / GPU view bytes | 8 MiB / 32 MiB | configured aggregate export pools; each GPU participant binding also fits the adapter limit |
| Scheduled behavior input records / host bytes / GPU ingress bytes | 16 / 4 MiB / 4 MiB | configured opaque per-participant current input; complete descriptor maxima are reserved before planning and GPU bytes use ordered read-only uploads |
| Scheduled behavior CPU collision calls / contacts / bytes | 128 / 4,096 / 320 KiB | configured aggregate calls plus one reusable exact 80-byte contact-slot sink |
| Scheduled behavior handoff maps / bytes | 4 / 24 MiB | configured Moria-owned host/device/staging transport; payload meaning remains consumer-owned |
| Scheduled behavior proposals / payload / affected cells / affected bricks / directory effects / conflict checks / feedback | 1,024 / 64 MiB / 262,144 / 4,096 / 16 / 1,048,576 / 1 MiB | configured and wholly reserved/bounded before adapters run |
| Scheduled behavior GPU buffers / live buffer bytes / pipelines / bind groups / WGSL bytes | 256 / 256 MiB / 64 / 256 / 4 MiB | configured aggregate opaque factory resources; buffer bytes use a 64 MiB minimum and 1 GiB/adapter-max clamp, while WGSL is charged before parse |
| Scheduled GPU adapter dispatches / workgroups | 256 / 1,048,576 | configured counted-encoder limits per tick |
| Asynchronous GPU extension jobs | 64 | configured |
| Asynchronous GPU extension registrations / registry bytes | 32 / 4 MiB | configured; 1 MiB WGSL + 128-byte entry point per registration |
| Candidate effects per extension job | 256 | fixed v1 maximum; batch-reserved before dispatch |

The config must be capable of servicing one maximum legal operation for each
enabled capability. For example, enabling patch mutations with a command byte
budget or extraction batch below one maximum patch is a configuration error,
not a runtime deadlock. `live_volumes` bounds concurrent directories;
`volume_records` bounds every stable key accepted for the world lifetime, and
retirement converts rather than frees that record. Presentation invalidation
uses the bounded dirty-record pool; when it cannot retain individual keys it
atomically raises the marker reserved for that live-volume slot and later
enumerates only bounded active-interest artifacts. Configuration reserves all
`live_volumes` markers in addition to at least `presentation_jobs` other dirty
records, so simultaneous commits in every live volume lose precision but never
lose an eventual rebuild obligation. The field-level public schema, adapter-clamp
rules, and exact cross-limit relationships are normative in
[public-api.md](public-api.md#configuration-schema); this table summarizes
storage-facing capacity rather than defining a second configuration shape.

## Pressure policy

Policy is selected for command, query, checkpoint, scheduled behavior tick,
and asynchronous extension queues as `Reject` or `WaitForPermit`. It never
changes an already called `try_` method;
it determines whether the corresponding non-try `reserve_*` future waits or
immediately returns `Full`. Interest has no payload queue and rejects
synchronously at its lease limit. Interest and presentation scheduling may
defer lower priority work. Authoritative operations are never coalesced across
command IDs. Presentation rebuilds for superseded revisions may be coalesced,
and telemetry records each skipped target.

When allocation pressure occurs:

1. reclaim completed extraction/staging/version resources;
2. compact homogeneous/reverted bricks;
3. retire eligible presentation/dressing;
4. evict eligible authoritative bricks;
5. defer background materialization;
6. reject the requesting admission with exact limiting pool.

Behavior/extension registration and volume-key lifetime exhaustion are not
transient allocation pressure: registration returns its exact record/byte
capacity error, and a world that has consumed `volume_records` rejects every
new stable volume key even after older volumes retire. Telemetry exposes
current, high-water, limit, and rejection/coalescing counts for every pool in
the public `ResourceLimits`, including extraction, presentation
dirty/artifact/instance, behavior input/view/collision/handoff/proposal/feedback/
opaque-resource/live-factory-buffer-byte/WGSL, and extension-registry
resources. Factory buffer bytes remain charged after logical handle drop while
a bind group or in-flight submission depends on them; capacity returns only
after dependency drop and last-use completion. Device-loss recreation begins
only after the terminal generation's aggregate byte charge reaches zero.

Moria never blocks a render schedule waiting for capacity.

## Device loss and recovery state

All device-bound handles contain `DeviceGeneration`. On loss:

- the generation is marked terminal before processing callbacks;
- pending-but-unsubmitted work returns to its control queue;
- submitted receipts fail `DeviceLost` and never later succeed;
- mapped/staging callbacks from the old generation are discarded;
- no world query returns a fact until recovery rematerializes its scope;
- an active scheduled behavior tick without confirmed revision-gate completion
  reports typed no-publication device loss; a confirmed publication remains
  published; late adapter callbacks, mapped handoffs, and feedback from the old
  generation are quarantined; old-generation consumer-input uploads are also
  quarantined and cannot reach an adapter; the first recovered GPU tick
  receives typed `UnavailablePreviousGeneration` feedback;
- CPU adapter state remains consumer-owned; GPU adapter resources/state are
  invalid and must be recreated by that adapter before it reports ready;
- dirty scar data already durably stored is loaded normally;
- retained dirty scar data that existed only on the lost device cannot be
  claimed recovered.

An applied mutation is intentionally volatile until a requested checkpoint
includes it. This keeps mutation completion separate from persistence
completion, as required by the public receipts. Telemetry labels dirty GPU-only
bytes and revisions. If device loss occurs with any committed revision newer
than its durable checkpoint frontier, recovery fails the world with
`UnrecoverableDirtyState`; it does not reconstruct an older base and pretend
success. If all committed scars are durable, recovery may rebuild the same
truth and return to ready.

## Integer and generation exhaustion

Operation IDs, revisions, table generations, slot generations, and observation
sequences use checked increments. Exhaustion closes the narrowest affected
scope:

- slot generation exhaustion permanently retires that slot;
- operation/observation exhaustion closes the world to new admissions;
- volume revision exhaustion fails that volume;
- device generation exhaustion requires process restart.

No counter wraps, and no physical identity becomes a public stable key.
