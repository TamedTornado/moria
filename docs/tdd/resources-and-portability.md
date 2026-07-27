# Resource Bounds, Portability, and Telemetry

## 1. Bound philosophy

Every allocation is attributable to configuration, an admitted operation, a
live interest, a retained result/subscription, a scar, or derived
presentation. Admission reserves worst-case capacity before work starts.
Arithmetic uses checked `u64`/`usize` conversions and rejects values that do
not fit both host and shader representations.

Defaults are safe development values, not claims of an unbounded world.
Consumers may raise them only within adapter and hard format limits.

## 2. Hard contract limits

These limits are encoded into validation and, where applicable, wire formats:

| Item | Hard maximum |
| --- | ---: |
| Active worlds per plugin | 8 |
| Registered materials per world | 65,535 plus reserved empty |
| Active volumes per world | 4,096 |
| Cells on one volume axis | 2,097,152 (`2^21`) |
| Live interests per world | 4,096 |
| Regions covered by one interest | 4,096 |
| Cells targeted by one matter command | 262,144 |
| Bricks touched by one matter command | 4,096 |
| Decompressed patch/stamp bytes | 8 MiB |
| Cells returned by one region query | 262,144 |
| Facts returned by one query | 65,536 |
| Convex query planes | 32 |
| Trace/sweep hits | 4,096 |
| Live receipts/query tickets per world | 65,536 |
| Observation subscriptions per world | 256 |
| Events in one subscription | 65,536 |
| Correlation metadata | 256 bytes |
| Public diagnostic string | 1,024 bytes |
| GPU extension snapshot cells | 1,048,576 |
| GPU effects in one exchange | 262,144 |
| One persistence chunk | 8 MiB |
| One world checkpoint | Configured, default 4 GiB; streaming only |

Requests outside hard maxima are rejected with the supported maximum and no
allocation. Lower per-world configured maxima are reported the same way.

## 3. Default budgets

Per plugin defaults:

| Pool | Default |
| --- | ---: |
| Authoritative GPU descriptors/bricks/occupancy | 512 MiB |
| GPU transaction staging | 128 MiB |
| GPU query/result/readback | 64 MiB |
| GPU presentation/dressing | 512 MiB |
| CPU source/upload staging | 128 MiB |
| CPU scar index | 256 MiB |
| CPU receipts/observations/metadata | 128 MiB |
| Concurrent source requests | 16 |
| Concurrent mutation transactions | 8 |
| Concurrent query dispatches | 64 |
| Concurrent presentation builds | 16 |
| Receipt retention | 60 seconds or explicit acknowledgment |
| Idle subscription expiration | 10 minutes |

Budgets are separate so presentation pressure cannot consume transaction
guarantees and a readback burst cannot consume scar safety. GPU buffers grow
geometrically inside the budget and are never resized while referenced.
Allocated bytes, live bytes, high water, fragmentation, and rejected
reservations are reported separately.

Startup validates that the configured total fits adapter-reported buffer and
binding limits. It does not assume all reported video memory is exclusively
available. Runtime allocation failure remains possible and is explicit.

## 4. Pressure and eviction scoring

An unpinned region's eviction score is deterministic:

```text
(effective_priority ascending,
 last_interest_frame ascending,
 presentation_before_authority,
 VolumeId,
 RegionCoord)
```

Frames are monotonic runtime counters, not wall time. Dirty scars do not make
authority unevictable once canonical CPU scar storage is reserved, but scar
storage itself is never evicted without durability. An admitted request's
reservation is not stolen by later higher priority work.

Telemetry records the exact pool, requested/reserved bytes, selected pressure
action, affected scope, and effective priority.

## 5. Supported platforms

Current supported targets are 64-bit native desktop:

- Linux x86_64/aarch64 with Vulkan 1.2;
- Windows x86_64 with Direct3D 12; and
- macOS arm64/x86_64 with Metal through Bevy/wgpu.

Web/wasm, mobile, 32-bit hosts, OpenGL, and networked/distributed authority are
not supported targets. Unsupported targets fail compile via target gating or
startup with `UnsupportedPlatform`; they are not silently routed to a CPU
world.

Required GPU capabilities:

- compute shaders and storage buffers;
- at least eight storage buffers per compute stage;
- 256 MiB maximum storage-buffer binding size or a validated segmented layout;
- indirect draw for presentation;
- map-read buffers for bounded results; and
- integer atomics required by the compaction kernels.

Optional capabilities:

- timestamp queries for GPU timings;
- subgroup operations for measured optimized kernels; and
- multi-draw indirect for presentation batching.

No semantic path depends on optional capabilities. WGSL baseline kernels avoid
subgroups, vendor intrinsics, float atomics, sparse hardware resources, and
backend-specific memory layouts. Optimized variants must pass byte/fact parity
against baseline before selection. Adapter/backend/driver selection and shader
variant are included in every evidence report.

## 6. Headless and CI strategy

Headless conformance still creates a wgpu device and runs production GPU
kernels. On Linux CI, a Vulkan software adapter may prove semantics but cannot
prove performance. A no-adapter environment reports required GPU scenarios as
`not_demonstrated` and exits nonzero.

Unit/property tests use a private deterministic CPU reference model to compute
expected facts; it is an oracle, not a selectable runtime fallback. At least
one Vulkan, Metal, and DX12 lane must pass the portable contract suite before a
release. Performance qualification uses timestamp-capable physical adapters
and reports the exact machine; no software-adapter timing is accepted.

## 7. Performance acceptance profiles

Correctness suites must pass before performance results count. The initial
`interactive` profile uses default budgets, a ready working set of 64 mixed
bricks, commands of at most 32,768 cells, queries returning at most 16,384
facts, and a timestamp-capable native physical adapter.

After 30 warm-up iterations, 1,000 measured operations must meet:

| Measure | Acceptance |
| --- | ---: |
| Ready mutation admission-to-commit p95 | ≤ 50 ms |
| Ready sample/trace/overlap query p95 | ≤ 50 ms |
| Commit-to-current presentation p95 | ≤ 150 ms |
| Dynamic placement commit p95 | ≤ 33 ms |
| GPU extension snapshot-to-admission-results p95 | ≤ 50 ms |
| Any measured operation p99 | ≤ 4 × its p95 limit |
| Budget overrun or unknown-as-empty | zero |

These thresholds qualify the declared profile, not every machine or arbitrary
request size. Reports include CPU/GPU/OS/driver, resolution, Bevy/Moria
revision, adapter backend, budgets, request distributions, shader variants,
raw samples, and failures. A machine that misses timing remains semantically
supported but does not qualify for the interactive-performance claim.

Sparse-scale acceptance uses a `4096^3`-cell homogeneous base with eight live
regions, then introduces 32 mixed bricks and scars. Live authoritative bytes
must grow with live descriptors/mixed bricks and stay below 64 MiB excluding
preallocated free capacity; withdrawing interest returns live detail below
8 MiB while scars remain reconstructable. A raw dense allocation of the domain
is an automatic failure.

Checkpoint acceptance for 32 changed bricks must serialize less than 2 MiB plus
fixed manifest overhead and must not scale with untouched domain cells.

## 8. Stable telemetry schema

Telemetry is versioned independently from persistence. Required records:

- context: product/format/layout/config hashes, platform, adapter, driver;
- catalog: world/volume/material counts and revisions;
- lifecycle: regions per state/capability and transition durations;
- residency: allocated/live/high-water bytes by authoritative, transaction,
  query, presentation, staging, and scar pool;
- interest: count, bounds/region counts, priority, readiness lag;
- commands: admitted/rejected/pending/applied/failed/conflicted/cancelled,
  target cells/bricks, stage timings;
- queries: kind, requested/covered size, result facts, completeness,
  availability, timing/readback bytes;
- presentation: state counts, geometry/anchor bytes, revision lag/build time;
- persistence: dirty bytes/revisions, checkpoint stage timings and coverage,
  restore errors;
- observations: queue fill, delivery lag, gaps, snapshots;
- extensions: snapshot/effect bytes and records, GPU-to-GPU transfer,
  readback, admission outcomes, timings; and
- pressure: pool, reservation, delay/retire/reject action.

High-cardinality IDs and bounds are emitted only for explicit diagnostic detail
requests. Default telemetry aggregates and bounds memory. Metrics do not expose
cell payloads or internal indices.

## 9. Diagnostic visualization

`moria-lab` may render raw cell samples, brick/region boundaries, lifecycle,
occupancy summaries, revisions, interest, and pool pressure by issuing public
diagnostic queries. The product crate may provide DTOs and presentation helpers
for those results but not internal storage access. Diagnostic displays carry
revision/completeness watermarks and visibly distinguish unavailable from
empty.
