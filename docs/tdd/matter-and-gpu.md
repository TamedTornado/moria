# Matter representation and GPU execution

## 1. Authoritative matter model

The addressable unit is a cubic cell. `MatterCell` is a stable 32-bit logical
value:

```text
bits  0..15  MaterialId table slot (0 = empty)
bits 16..23  coverage (0 = empty, 1..255 = material)
bits 24..31  reserved, must be zero in contract version 1
```

The persisted form stores the stable 128-bit `MaterialId`; the table slot is a
running-world encoding only. A source or patch with zero material/nonzero
coverage, nonzero material/zero coverage, unknown material, or reserved bits
is invalid.

Coverage is authoritative shape information. Cell sampling returns exact
material and coverage. Discrete region occupancy treats a cell as occupied at
the registered material threshold. Point/surface queries trilinearly interpolate
the eight neighboring cell coverages, using empty outside a volume domain, and
select the highest-contributing occupied material with stable `MaterialId`
tie-breaking. Presentation may smooth or preserve faces but cannot alter these
facts.

Public world transformations run in `f64` on CPU. Before GPU dispatch, a
world-space query is transformed into each candidate volume's local space and
encoded as signed brick coordinates plus brick-local `f32` offsets; large
absolute world coordinates are never converted directly to `f32`. GPU contact
results use the same split representation and are transformed back to `f64`.
Rendering uses camera-relative high/low translation components. Conformance
tests require cell selection to match the integer CPU oracle exactly and
continuous contact values within `1e-5 * cell_size` plus two `f64` ULPs.

## 2. Sparse layout

The fixed brick edge is 8 cells (512 cells). This is a format/ABI constant for
contract version 1, not consumer configuration. A brick descriptor is:

- `Uniform(MatterCell)`, stored inline in the sparse tree; or
- `Dense(PageIndex)`, referencing a 2 KiB immutable page in the GPU page pool.

Each volume has a 15-level, 4-way-per-axis radix tree over signed brick
coordinates biased into 30-bit unsigned order. A node has 64 child
descriptors; each level consumes two bits from each axis. Empty subtrees and
repeated uniform subtrees collapse to one descriptor. Only nodes on paths to
nonuniform or scarred bricks allocate. The root descriptor and page-table
generation form a `SnapshotRoot`.

Dense pages include a 64-bit content hash and summary (`occupied_count`,
material-slot bloom, min/max coverage). Summaries accelerate rejection but a
positive query always examines authoritative cells. Hash collisions never
establish equality: compaction compares bytes before sharing pages.

Uniform descriptors and immutable dense pages make snapshots cheap. COW pages
are reclaimed only after GPU fences, active queries, presentation jobs,
extension leases, scar capture, and checkpoint cuts release their generation.

## 3. Content materialization

For each requested brick, `BaseContentSource::load_bricks` receives a sorted,
deduplicated bounded batch and cancellation token. It asynchronously returns
one `VerifiedBrick` per request:

```rust
pub enum BrickPayload {
    Uniform(PersistedMatterCell),
    Dense(Box<[PersistedMatterCell; 512]>),
}
pub struct VerifiedBrick {
    pub coordinate: BrickCoord,
    pub payload: BrickPayload,
    pub merkle_proof: BaseProof,
}
```

The runtime validates count, coordinates, material IDs, coverage, domain,
Merkle proof, and configured byte limit before upload. Scars are then overlaid
on the GPU. A batch is installed only when every brick validates; invalid
content fails the affected region and does not become truth.

Source calls run on Bevy's async compute pool or a consumer implementation's
own async runtime. Moria never calls a source on the render thread. Temporary
CPU bytes are charged to `max_content_staging_bytes` and dropped after upload
and validation.

## 4. Mutation transaction and atomic publication

Commands for one volume execute and publish in admission order. Different
volumes may execute concurrently. The transaction protocol is:

1. **Reserve:** admission calculates worst-case touched bricks/pages, decoded
   patch bytes, output, scar bytes, and one revision. All capacity is reserved.
2. **Prepare:** required cold bricks materialize at revision `R`; the runtime
   rechecks the revision precondition immediately before execution.
3. **Stage:** a compute pass reads root `R`, allocates only reserved COW pages,
   applies the command into an unpublished root, and emits changed-cell runs.
4. **Validate:** GPU status asserts bounds, allocation count, patch
   consumption, and changed-run capacity. A forced validation fault used by
   tests terminates here.
5. **Capture:** changed final values and their old-value hashes are copied to a
   bounded CPU scar buffer and CRC-checked. This sparse buffer is required for
   device recovery and persistence; it is not a full mirror.
6. **Publish:** if any cells changed, one final compute dispatch writes
   `{root_index, R+1}` to the volume selector. Queue ordering makes all page and
   tree writes happen-before the selector write. A no-op keeps root and `R`.
7. **Confirm:** after the submission fence completes, CPU metadata installs
   the same root/revision, queues one observation, releases unused reservation,
   and completes the receipt.

Queries and derived jobs bind the selector captured when dispatched. They can
see old root `R` or new root `R+1`, never staging pages. A failed command never
executes step 6. Device loss before confirmation discards the device and
rebuilds from the last CPU-confirmed revision plus confirmed sparse scars; the
unconfirmed command fails with no logical commit even if the lost device may
briefly have executed its selector write.

There is one publication lane per volume, which deliberately trades same-volume
commit throughput for simple atomic semantics. Stage kernels still parallelize
over touched bricks. This can later become optimistic multi-version admission
without changing the facade.

Placement changes use the same per-volume lane. They stage an immutable
placement record and publish `{placement, R+1}` in the volume metadata table.
Matter and placement commands are totally ordered within a volume. They are
never combined implicitly.

## 5. GPU query and extension access

Normal CPU consumers receive bounded readback. GPU consumers use opaque leases:

- `GpuSnapshotLease`: read-only bind group for selected snapshot page tables,
  pages, material table, volume metadata, and declared bounds.
- `GpuResultLease`: read-only result buffer produced by a Moria query.
- `GpuProposalLease`: write-only, Moria-allocated proposal buffer with a fixed
  record count and byte capacity.

Leases are render-world values valid for one declared frame interval and one
captured revision vector. They do not expose `wgpu::Buffer`, offsets, allocator
handles, or mutable bindings. The consumer registers a WGSL pipeline callback
against versioned bind-group layouts; Moria schedules it in a bounded render
graph node. Snapshot bindings have storage-read access only. Proposal bindings
have storage-write access only and are a different allocation.

The proposal ABI contains generic final-value patch runs or placement requests,
producer correlation keys, and declared bounds. It contains no force, damage,
health, gravity, or other behavior field. Overflow sets a batch flag and
rejects the entire batch. Moria validates proposal syntax and bounds on GPU,
reads back only a bounded summary, reserves ordinary command capacity, assigns
receipts, and executes each accepted proposal through the same transaction
lane. Proposals are not committed effects and cannot name internal page
addresses.

Each extension registration declares maximum input bricks, proposal count,
proposal bytes, dispatch dimensions, and execution frequency. Moria may defer
or reject a lease under pressure. Shader validation rejects writable aliasing,
undeclared bindings, ABI mismatch, or dispatch dimensions beyond the declared
limit. wgpu validation and device isolation remain the final safety boundary;
a plug-in device error fails its lease and world GPU work is recovered.

## 6. GPU capability floor and portability

Startup requires:

- compute shaders and storage buffers;
- at least 8 storage buffers per compute stage;
- 256-byte storage-offset alignment or better;
- 32-bit integer atomics;
- maximum compute workgroup size of at least 256 invocations;
- maximum storage buffer binding size large enough for the configured largest
  page-pool segment; and
- timestamp queries only when performance evidence is requested.

The design does not require 64-bit atomics, subgroup operations, sparse GPU
resources, mesh shaders, indirect-count extensions, bindless textures, or
backend-specific shader languages. Revision halves are written/read under one
seqlock-style 32-bit generation word in GPU structs; host results reconstruct
the `u64` and retry if the generation changed.

Page pools are segmented so no binding exceeds the adapter limit. Adapter
probing derives segment count and validates `MoriaLimits`; it never silently
shrinks consumer limits. Unsupported configuration fails startup with required
and observed values.

## 7. Resource limits and admission

`MoriaLimits` is mandatory and contains at least:

| Limit | Charged unit | Exhaustion behavior |
| --- | --- | --- |
| `max_worlds`, `max_volumes_per_world`, `max_materials` | registered items | configuration/create rejection |
| `max_interests`, `max_interest_bricks` | tracked sources/bricks | interest rejection |
| `authoritative_gpu_bytes` | tree, page, metadata bytes | retire eligible or defer/reject |
| `derived_gpu_bytes` | mesh/dressing bytes | stale/absent presentation |
| `max_content_staging_bytes` | source payload bytes | materialization pending/fails by policy |
| `max_mutation_cells`, `max_mutation_bricks` | per command | rejection |
| `max_patch_bytes` | encoded per command | rejection |
| `max_inflight_commands`, `max_transaction_bytes` | queue/reservations | rejection |
| `max_query_cells`, `max_query_result_bytes` | per query | reject or explicit partial |
| `max_inflight_queries`, `max_readback_bytes` | queue/reservations | rejection |
| `max_observation_records`, `max_subscriptions` | ring records | explicit gap/rejection |
| `max_receipt_records` | retained receipts | explicit receipt gap |
| `max_dirty_scar_bytes` | confirmed sparse changes | reject mutations; never discard |
| `max_checkpoint_bytes_inflight` | checkpoint staging | checkpoint rejection |
| `max_gpu_extension_*` | leases/proposals/bytes | lease/proposal rejection |

All arithmetic is checked. Reservations occur before `Admitted`; a later
internal estimate overrun is `InternalInvariant`, fails before publication,
and is a test failure. Priorities choose among otherwise eligible work but
never steal a reservation from admitted work. The API provides named
`MoriaLimits::small_validation()` and `desktop_reference()` constructors, but
the consumer must insert the chosen value explicitly and telemetry records
every field.

Retirement order is: unreferenced derived artifacts, ready clean regions by
lowest priority/oldest use, then nothing. Dirty, in-flight, leased, or
checkpoint-covered generations are pinned. If this cannot satisfy a request,
the request remains pending or is rejected according to its policy.
