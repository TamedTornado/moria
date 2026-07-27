# Architecture and Ownership

## Component model

Moria has a control plane and a render/compute plane. They communicate through
bounded, generation-tagged queues. Neither plane is an alternate source of
material truth.

```text
consumer threads / Bevy main world
        |
        | owned commands, queries, interests, checkpoint requests
        v
control plane: facade + admission + lifecycle + identities + receipts
        |                                      ^
        | bounded prepared work                | bounded completion metadata
        v                                      |
Bevy RenderApp / Moria render resources
        |
        +-- sparse authority: page table + brick/scar pools + revisions
        +-- collision/query compute
        +-- presentation derivation
        +-- staging/readback pool
        +-- GPU extension packet/effect jobs
        |
        v
renderer-owned RenderDevice and RenderQueue
```

Base-content workers and checkpoint-store workers are subordinate to the
control plane. They exchange bounded brick batches or chunk blobs; they cannot
publish truth.

## Module ownership

### `identity`

Owns opaque generational runtime IDs, stable UUID wrappers, monotonic operation
IDs, and stale-handle validation. It depends only on serialization-free core
types.

### `material`

Owns `MaterialKey`, runtime `MaterialId`, `MaterialSample`,
`MaterialDefinition`, occupancy rules, and presentation descriptors. It does
not own behavioral fields.

### `config`

Owns `MoriaConfig`, validated resource limits, backend capability
requirements, and overload policies. Validation returns all detected
configuration errors in deterministic field order.

### `content`

Owns the `BaseContentSource` port, content requests/results, reconstruction
lineage/fingerprint, base-brick validation, the bounded worker pool, and the
atomic callback-count/response-byte permit. It invokes consumer code only
after both resources are reserved and releases response bytes only after the
returned allocation is dropped. Consumer algorithms live behind this port.

### `volume`

Owns volume registration, finite local domain, rigid placement, static/dynamic
mode, committed revision metadata, and lifecycle summaries. It does not own
motion policy.

### `interest`

Owns interest leases, requested capabilities, priority, request coalescing, and
retirement eligibility. Camera entities have no special status.

### `storage`

Owns logical brick keys, page entries, version-chain rules, physical slot
allocation, dirty/scar metadata, occupancy summaries, snapshot pins, and
reclamation. It has no Bevy ECS or public consumer types beyond deliberately
exported value objects.

### `command`

Owns permits, validation, command payloads, per-volume FIFO ordering, receipts,
cancellation state, prepared mutation transactions, and terminal outcomes.
Only this module can request a material or placement commit.

### `query`

Owns bounded query descriptors, permits, snapshot acquisition, result codecs,
partial-result policy, public contact/result value types, and query receipts.
It translates public collision-bearing queries into private collision-kernel
plans and encodes the returned kernel facts; it never calls presentation.

### `collision`

Is a lower-level service consumed by `query`. It owns exact v1 occupancy
predicates, broad-phase brick traversal, private shape/trace/sweep plans, and
private kernel facts. It depends on `storage`, `material`, `volume`, and shared
coordinate values, but never imports `query`, its public descriptors, partial
policy, result codecs, or receipts. It produces facts, not public responses or
behavior.

### `observation`

Owns filtered subscriber cursors, the bounded world observation ring, explicit
gaps, resnapshot coordination, and the fixed append-time spatial filter
envelope retained with each fact. It never reconstructs historical filtering
geometry from current directory state. An observation records a committed fact
only.

### `presentation`

Owns surface and dressing derivation jobs, derived-resource budgets, source
revision tags, render entity installation, status, and failure/retry. It reads
committed matter and cannot alter it.

### `persistence`

Owns checkpoint frontiers, scar chunk encoding/decoding, integrity validation,
the `CheckpointStore` port, restore, and dirty coverage. It does not serialize
presentation or consumer behavior state.

### `telemetry`

Owns public aggregate snapshots and machine-readable evidence records.
Telemetry reads counters and summaries; it exposes no storage handle.

### `gpu`

Owns shader layouts, device-generation resources, dispatch encoding, staging
pools, validation error scopes, completion callbacks, extension packets, and
layout assertions. It is the only module allowed to turn storage transactions
into GPU work.

### `bevy`

Owns `MoriaPlugin`, main-world resources/messages, extraction, render-world
resources, schedule registration, startup/recovery, and render entity
installation. It is wiring, not domain authority.

## Bevy integration and schedule order

`MoriaPlugin` requires Bevy's render plugin. Its build step installs control
plane resources in the main world and a render sub-app plugin in `RenderApp`.
If `RenderApp` is missing, startup fails with
`OperationErrorKind::Startup(StartupFailure { stage:
RendererLookup, causes: [RendererUnavailable] })`; simulation-only headless
apps can use the host oracle/test support but cannot claim a running Moria
world.

Device-bound layouts, pipelines, pools, and fallback diagnostic assets are
created in `RenderStartup`. Every resource carries a monotonically increasing
`DeviceGeneration`. Recovery reruns startup and invalidates all prior handles.

Main-world order:

1. `First`: receive worker/store completions and renderer completion metadata.
2. `PreUpdate`: update receipts, emit observations, and apply lifecycle
   transitions proven by completions.
3. `Update`: consumer systems may reserve/submit/cancel work and update
   interests.
4. `PostUpdate`: validate admissions, freeze bounded extraction batches, and
   update presentation entities from already completed artifacts.
5. `Last`: snapshot telemetry and advance shutdown coordination.

Render-world order:

1. `ExtractSchedule`: copy only frozen descriptors, payload ranges, interest
   deltas, and IDs. It never copies resident volume content.
2. `RenderSystems::PrepareResources`: allocate or reserve slots/staging,
   materialize validated base batches, and prepare copy-on-write transactions.
3. `RenderSystems::Prepare`: build query, collision, presentation, and
   extension dispatch descriptors.
4. Root `RenderGraph`: execute camera-independent Moria compute in explicit
   order:
   `materialize -> prepare_mutation -> validate_mutation -> publish_revision
   -> query/collision -> extension_packet -> presentation`.
5. Renderer cleanup: register queue-completion callbacks, map submitted
   readbacks asynchronously, and retire resources whose last submission is
   complete.

Work that does not fit the extraction batch remains queued; it is not silently
dropped. `extraction_records` and `extraction_bytes` are the exact per-frame
freeze limits. The byte limit covers owned command/query descriptors and
payload ranges, extension job ranges, content batches, and completion metadata;
it never includes resident volume data. Both current/high-water counts,
deferred records/bytes, oldest deferred operation, and extraction-frame lag are
telemetry. Configuration must fit one maximum enabled patch or extension
input packet plus inline state, while larger queue contents drain across later
frames.

For GPU `ObservationDeltas`, the main-world observation owner freezes the
subscriber's accepted filter, retained oldest/head pair, status, cursor, and
matching fact-plus-envelope records as one extraction record. The CPU
subscriber cursor is neither read nor written. The render world only encodes
that immutable bounded capture into ABI v1 and cannot reinterpret a gap,
unsupported fact, or maximum-record boundary as an empty complete packet.

## Threading and progress

`MoriaHandle`, world handles, permits, and receipts are `Send + Sync`. Methods
never hold a Moria lock while invoking consumer code. Per-world mutable control
state is owned by the Bevy main thread; cross-thread submissions enter bounded
MPMC queues.

Base content and persistence each use a dedicated bounded native worker pool.
The default is two content workers and one persistence worker; configuration
allows 1–8 each. Worker callbacks receive cancellation tokens and owned data.
A callback panic is caught at the worker boundary and becomes
`ContentError::Panicked` or `PersistenceError::Panicked`; the worker slot is
replaced.

Progress requires:

- the Bevy app schedules to keep updating;
- the render app to submit and poll the renderer device;
- consumer executors to poll receipt futures only for wakeup delivery, not to
  drive GPU work.

`Receipt::try_status` is available for consumers that do not use async.
Blocking waits are deliberately absent from the public API.

## Dependency policy

The domain dependency direction is acyclic:

```text
identity/material/config
        -> content/volume
        -> storage
             |-> collision -> query
             |-> command
             `-> interest
        query/command/interest
             -> presentation/persistence/observation/telemetry
        -> bevy
```

`query` is the only public collision-query orchestrator. It converts public
shapes and result budgets to collision plans; collision returns private facts
that query turns into `ContactFact` values. `command` and `interest` do not
depend on query, and collision does not consume query descriptors. The `gpu`
module is a device implementation leaf used behind storage/collision/
presentation dispatch traits; it may import their POD plans but none of those
domain modules imports Bevy render types.

The package pins Bevy `=0.19.0`. It uses Bevy renderer wrappers and the
renderer-compatible wgpu version. Adding an independent wgpu version or
requesting an adapter/device in the Bevy path is forbidden.

Expected narrow dependencies are:

- `bevy` with explicitly selected application/render features;
- `bytemuck` for checked host/WGSL layouts;
- `uuid` for durable keys;
- `thiserror` for machine/actionable error types;
- `futures-channel`/`futures-core` for runtime-neutral receipts;
- `crossbeam-channel` for bounded ingress and worker queues;
- `blake3` for checkpoint integrity and reconstruction fingerprints;
- `crc32fast` for quick per-record corruption detection;
- `bitflags` for closed capability/flag sets;
- `serde` only for telemetry/evidence JSON, never as the persistence layout;
- `tracing` for diagnostics.

No dependency may introduce gameplay, physics, generation, or an async runtime
into the facade.

## Startup ownership and validation

World construction has two phases.

`MoriaBuilder::validate` is host-only and checks:

- all limits are nonzero and internally consistent;
- required world/material/volume/style debug names are 1..=96 UTF-8 bytes;
  retained volume names and content-lineage bytes canonicalize to exact boxes;
- the required consumer-supplied `WorldDefinition` has a nonnil stable key,
  and world/material/volume stable keys are unique in their scopes;
- material zero is not consumer-registerable;
- volume domains and coordinate conversions cannot overflow;
- placements are finite and rigid;
- every volume has a content source and exact reconstruction fingerprint;
- static/dynamic mode matches allowed placement command policy;
- persistence is configured if dirty-state retirement is enabled;
- observation fact/payload and staging limits can hold one maximum legal
  record/readback;
- aggregate material metadata bytes cover the per-registration maximum;
- content batching and response bytes can hold one maximum detailed source
  batch;
- live-volume and lifetime volume-record limits cover every initial
  registration, with lifetime records no smaller than live records;
- extraction can freeze one maximum enabled operation;
- presentation artifact/dirty/mesh/instance pools can represent their stated
  fixed maximum artifact, and dirty records reserve one marker for every live
  volume in addition to the job/exact-key partition; and
- extension registration count/bytes can hold every startup/runtime descriptor
  within its per-descriptor WGSL and entry-point caps.

`ValidatedMoria::into_bevy` returns the plugin, configured facade handles, and
typed startup receipt. Installing that plugin waits for GPU capability
negotiation, shader/pipeline creation, and initial volume-directory publication
or the selected restore. The world becomes `Ready` only after those stages
complete. The startup output contains the stable world identity, adapter
report, and every requested/effective config value. A failure tears down every
partially allocated resource and returns the complete scoped
`StartupFailure`. Renderer lookup, adapter qualification, device resources,
persistence open, restore read, and directory publication are distinct stages.
Adapter qualification aggregates every missing feature and unmet numeric
minimum in deterministic public records rather than returning the first error.

Required GPU capabilities are compute shaders, storage buffers, buffer-to-
buffer copies, at least four writable storage bindings for mutation, and the
configured binding/allocation limits. Optional timestamps and indirect
dispatch are enabled only when reported and have semantic fallbacks.
Adapter-negotiated capacities use `effective = min(desired, adapter_legal)`;
startup fails instead of clamping below the consumer's declared minimum or one
maximum legal enabled operation.

## Portability strategy

Capabilities and limits are queried and recorded; backend labels never stand
in for them. All buffers obey:

- allocation size `<= max_buffer_size`;
- each binding range `<= max_storage_buffer_binding_size`;
- dynamic offsets aligned to `min_storage_buffer_offset_alignment`;
- WGSL member alignment independently checked by Rust layout tests.

Pools larger than one storage binding are segmented. A page entry stores a
segment plus local slot; no shader binds an out-of-range slice. Readback always
copies from `STORAGE | COPY_SRC` working buffers to an unmapped
`MAP_READ | COPY_DST` staging buffer. Mapped views are dropped before unmap,
and a mapped buffer is never submitted for GPU use.

The first-class matrix is:

| OS | Backend family | Correctness claim |
| --- | --- | --- |
| Linux x86_64 | Vulkan | Required physical-adapter qualification |
| macOS arm64 | Metal | Required physical-adapter qualification |
| Windows x86_64 | DX12 | Required physical-adapter qualification |

Software adapters may run correctness smoke tests only and are labeled
fallback. They cannot qualify performance. An adapter below the minimum
configuration returns a deterministic `UnsupportedCapabilities` report listing
every unmet limit/feature and the maximum usable configuration where known.

## Failure containment

- A content failure affects the requested region and dependent operations; it
  does not fabricate empty matter.
- A command preparation or validation failure releases unreferenced slots and
  leaves the revision gate unchanged.
- A presentation failure affects only its artifact/status.
- A readback decode failure fails that query/checkpoint and quarantines the
  staging buffer.
- Queue validation errors are captured into the owning operation. Uncaptured
  device errors transition the world to `DeviceFailed`.
- Out-of-memory rejects or fails the operation that requested the allocation.
  Existing committed truth remains valid unless Bevy reports device loss.
- Device loss makes the entire old GPU truth generation unavailable. Recovery
  rejects new admissions, fails submitted operations with `DeviceLost`,
  requeues only work that had not reached submission, and rebuilds from
  registered base plus retained/durable scars. It emits readiness facts for the
  new generation only if every committed revision is reconstructable;
  otherwise the world terminates with `UnrecoverableDirtyState`.

## Security and trust boundary

Moria is an in-process library, not a hostile-code sandbox. It still validates
all public sizes, coordinates, material IDs, byte counts, shader packet
descriptors, checkpoint offsets, checksums, and decoded enum tags before
allocation or dispatch.

Consumer GPU extension WGSL is treated as untrusted input to the renderer:
Moria validates it with Naga, accepts only the fixed bind schema and one compute
entry point, prohibits internal buffer bindings, caps workgroups and output
records, and captures pipeline/dispatch errors. This limits accidental
corruption; it does not claim process isolation from malicious native code.
