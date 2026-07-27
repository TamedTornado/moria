# Moria Technical Design Overview

## Status and authority

This is the implementation contract for the approved
[`docs/design-document.md`](../design-document.md). The approved product design
is binding if this TDD is ever ambiguous. Supporting scope decisions come from
[`docs/product-vision.md`](../product-vision.md) and
[`docs/product-design-decisions.md`](../product-design-decisions.md). This TDD
selects engineering mechanisms; it does not add product scope.

The initial implementation target is a native Rust library integrated with
Bevy 0.19.0 and its renderer-owned wgpu 29.0.3 device. Linux/Vulkan,
macOS/Metal, and Windows/DX12 are first-class backend families after physical
qualification. Web, WebGL, GLES, a standalone renderer, and a shipped game are
not current targets.

## Outcome

Moria is one Cargo library package named `moria`. It exposes opaque world
handles, bounded commands and queries, receipts, observations, persistence
ports, telemetry, and a Bevy plugin. Its authoritative ready representation is
a sparse GPU material volume. CPU state owns configuration, identities,
admission, lifecycle metadata, bounded transports, and durable checkpoint I/O;
it does not keep a full voxel mirror.

Material detail is divided into fixed 8×8×8-cell bricks. Empty and uniform
bricks are encoded directly in a sparse page entry. Non-uniform bricks occupy
slots in a configured GPU pool. A per-volume revision gate publishes prepared
copy-on-write page versions atomically. Readers acquire a committed revision
before resolving page versions, so they see all or none of a multi-brick
mutation. Old versions and slots are reclaimed only after earlier GPU readers
have completed.

Every consumer, including examples and validation harnesses, uses the public
facade. The facade never returns Moria's page table, brick pool, mesh buffers,
or device. A first-class behavior coordinator pins one committed substrate
view, invokes independently implemented CPU and GPU adapters in a validated
declared order, and composes bounded proposed effects before the tick
publication boundary. CPU adapters receive a borrowed tick view instead of an
ordinary query receipt. GPU adapters encode on the renderer-owned device
against a read-only exported view and fixed effect target without mandatory
CPU readback on the authority path. The bounded WGSL inspection/effect job
remains a separate asynchronous tool API.

## Document map

| File | Contract owned |
| --- | --- |
| [architecture.md](architecture.md) | Components, module ownership, schedules, portability, and dependency direction |
| [public-api.md](public-api.md) | Consumer types, inputs, outputs, invariants, errors, scheduled adapter API, and asynchronous extension boundary |
| [state-and-storage.md](state-and-storage.md) | Coordinates, material encoding, sparse GPU layout, atomic publication, revisions, and resource bounds |
| [lifecycles.md](lifecycles.md) | Startup, interest, commands, queries, observations, shutdown, and device loss |
| [behavior-scheduling.md](behavior-scheduling.md) | Scheduled CPU/GPU behavior hooks, ordering, synchronization, composition, state ownership, and tick publication |
| [collision-and-presentation.md](collision-and-presentation.md) | Matter-derived collision, surface generation, dressing, and stale-view rules |
| [persistence.md](persistence.md) | Scar model, checkpoint format, restore, durability, and base reconstruction |
| [validation.md](validation.md) | Automated, real-GPU, portability, performance, and human evidence obligations |
| [decisions.md](decisions.md) | Consequential technical decisions and rejected alternatives |

## Binding invariants

1. A material sample in a committed volume revision is the only occupancy
   authority. Meshes, dressing, debug geometry, acceleration summaries, CPU
   staging data, and checkpoint encodings are derived or transport forms.
2. No public method exposes internal buffers or grants unbounded work.
3. Admission and completion are distinct. Accepted work has a receipt, and
   every accepted receipt reaches exactly one terminal outcome.
4. One matter command targets one volume and commits every targeted sample at
   one new volume revision, or changes no committed sample.
5. A placement change advances the same per-volume revision sequence as a
   matter edit. No operation combines independent volumes atomically.
6. Unknown, cold, failed, stale, or device-lost matter is never reported as
   empty.
7. Collision is computed from material samples and the committed placement,
   never from the render mesh.
8. A derived artifact is installed only if its source volume revision and
   placement revision still match. A stale artifact may remain visible only
   under consumer policy and is labeled stale.
9. Dirty scars pin enough state to reconstruct truth until a checkpoint store
   has durably committed them. Budget pressure cannot silently discard them.
10. Dropping a receipt does not cancel accepted work. For cancellable
    operations, explicit cancellation linearizes before the transition to
    `Preparing`; `Preparing` and later are too late. Startup and shutdown
    receipts are noncancellable.
11. Device loss terminates the old device generation. Late callbacks from that
    generation cannot publish success. Recovery returns readiness only if base
    content plus durable/retained scars reconstruct every committed revision;
    loss of a GPU-only dirty scar is terminal and is never hidden as rollback.
12. External behavior owns all behavioral vocabulary and working state. Moria
    schedules only bounded stable-view access and substrate-effect proposals;
    no adapter can mutate authority or bypass validation, composition,
    revision, receipt, or publication rules.
13. One scheduled behavior tick pins one committed view for every participant.
    Post-frontier commands cannot interleave before behavior publication, and
    adapter ordering never exposes an earlier proposal as committed matter to
    a later adapter.

## Selected implementation baseline

- Rust edition 2024 with `rust-version = "1.95.0"`.
- Bevy `=0.19.0`; the integration uses Bevy's render device, render queue,
  `RenderStartup`, extraction, render schedules, and root render graph.
- One package and one public library. There is no Cargo workspace until a
  separate deliverable or strict compile boundary actually exists.
- Native multithreaded operation. Public futures are runtime-neutral and do not
  require Tokio. WebAssembly is rejected at compile time for the GPU feature.
- `MaterialId` is a runtime `u16`; zero is canonical empty. Durable material
  identity is a consumer-supplied UUID.
- `VolumeId`, `WorldId`, command IDs, query IDs, interest IDs, and subscriber
  IDs are opaque generational handles. Durable volume identity is a UUID.
- Per-volume revisions are nonzero monotonic `u64` values. Exhaustion makes
  that volume terminally failed rather than wrapping.
- Logical cell and brick coordinates use checked signed `i32` triples.
  Placement is a validated rigid transform: finite `f32` translation plus a
  normalized quaternion. Scale and shear are not placement operations.
- The portable cell format is four bytes:
  `{ material: u16, coverage: u8, flags: u8 }`. Occupancy is
  `material != EMPTY && coverage >= 128`. Flags are format-reserved in v1 and
  must be zero.
- Fixed 8³ bricks are a v1 format constant. Changing this constant requires a
  persistence contract version and migration.
- Sparse lookup uses a bounded GPU hash page table and per-key version chains.
  Mutation uses copy-on-write slots and one revision-gate publication.
- Scheduled behavior uses one active tick per world, a builder-validated
  stable-key order DAG, canonical bounded CPU/GPU view exports, whole-proposal
  conflict policies, and at most one behavior publication revision per
  affected volume.
- Organic and constructed surfaces use GPU dual contouring with
  material-selected feature treatment. Collision continues to use occupied
  material cells, not that contour.
- Checkpoints store stable identities, source lineage plus an exact
  reconstruction fingerprint, placements, and sparse full-brick scars. They
  never store meshes or untouched base bricks.

## Intended repository shape

```text
.
├── AGENTS.md
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── assets/
│   └── shaders/
├── benches/
│   └── substrate.rs
├── examples/
│   ├── contract_harness.rs
│   └── visual_harness.rs
├── src/
│   ├── lib.rs
│   ├── bevy/
│   ├── behavior/
│   ├── collision/
│   ├── command/
│   ├── config/
│   ├── content/
│   ├── gpu/
│   ├── identity/
│   ├── interest/
│   ├── material/
│   ├── observation/
│   ├── persistence/
│   ├── presentation/
│   ├── query/
│   ├── storage/
│   ├── telemetry/
│   └── volume/
└── tests/
    ├── contract/
    ├── gpu/
    ├── persistence/
    └── support/
```

`src/lib.rs` is a facade and export list. Feature modules own their types,
systems, messages, schedules, and tests. Do not create top-level catch-all
`types.rs`, `systems.rs`, `components.rs`, or `utils.rs` files. A feature may
use private submodules with those names only when the feature boundary is
already clear.

`assets/` stays at repository root for Bevy `AssetServer` compatibility.
Shaders that define persistence- or storage-visible layout constants have a
Rust mirror and layout assertion tests.

## Intended `AGENTS.md`

The implementation agent must create `AGENTS.md` at the repository root with
the following rules. These are exact repository instructions, not suggestions.

### Formatting and documentation

- Run `cargo fmt --all` after Rust edits. `cargo fmt --all -- --check` is the
  formatting gate.
- Markdown uses one sentence per line where practical, fenced blocks with a
  language, and no trailing whitespace.
- Public items require rustdoc that states bounds, revision semantics,
  cancellation behavior, and errors. Unsafe code requires a `// SAFETY:`
  argument adjacent to each unsafe block.
- `#![deny(unsafe_op_in_unsafe_fn)]` is mandatory. Unsafe code is allowed only
  in the `gpu` module for verified byte-layout interop; prefer `bytemuck`
  derives.

### Exact checks

Run the ordinary local gate in this order:

```sh
cargo fmt --all -- --check
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo doc --no-deps --all-features
```

Real-GPU correctness is an explicit additional gate on a machine with a
qualified physical adapter:

```sh
cargo test --test gpu --all-features -- --ignored --test-threads=1
```

Contract evidence, build, benchmark, and development commands are:

```sh
cargo run --example contract_harness --features validation -- --scenario all --report target/evidence/contracts.json
cargo build --all-targets --all-features
cargo bench --bench substrate --features validation
cargo run --example visual_harness --features validation
```

The ordinary test suite must not open a window or require a GPU. GPU tests are
ignored only when they execute a real adapter; shader validation and host
contract tests remain in the ordinary suite.

### Module and dependency rules

- Dependency direction is `identity/material/config -> content/volume ->
  storage`, then `storage -> collision -> query`, while `command` and
  `interest` depend directly on storage. `behavior` depends on the public
  access values, collision kernel, command transaction builder, and storage
  snapshot interface; none of those lower layers depends on adapters.
  `query`, `command`, `interest`, and `behavior` feed
  `presentation/persistence/observation/telemetry -> bevy`.
  `collision` is a private lower-level fact kernel: it must not import public
  query descriptors, partial-result policy, codecs, or receipts. Lower layers
  must not import Bevy ECS, cameras, windows, presentation, or consumer
  behavior concepts.
- Only `bevy` and `gpu` may use Bevy render APIs. `material` may name the
  public Bevy asset-handle types selected by `SurfaceDescriptor` and
  `DressingDescriptor`, but may not access renderer state. Only `bevy`
  registers schedules or ECS-facing plugins.
- Do not create a second wgpu device in the Bevy path. Device-bound resources
  live in the render world and are recreated from `RenderStartup`.
- Keep backend/runtime types out of the main facade. The scheduled GPU adapter
  API is isolated under `moria::bevy::behavior` and deliberately versioned to
  Bevy 0.19/wgpu 29. It receives a counted Moria-controlled encoder wrapper,
  read-only exported view, and write-only proposal/feedback targets, never the
  raw encoder, render queue, or authoritative storage handles. The
  asynchronous WGSL facility exposes Moria descriptors and opaque handles, not
  `wgpu::Buffer`.
- Channels, staging pools, page tables, mesh outputs, and per-request payloads
  must be bounded by `MoriaConfig`. No unbounded channel or implicit allocation
  policy is allowed.
- Invoke a base-content callback only after atomically reserving both its worker
  slot and worst-case output bytes. Construct the exact-length Moria-owned
  output sink inside that permit before invocation; callback writes may borrow
  or copy only fixed-size values and may not return result ownership.
  `descriptor()` must remain a borrow, and callback failure diagnostics must
  remain fixed inline values; no variable owned descriptor or error allocation
  may cross the port. Keep the sink charged until install or failure cleanup
  and drop.
- Retained observation facts carry their fixed append-time filter envelope.
  GPU delta reads freeze an independent closed
  `Empty | Retained { oldest, head }` frontier plus cursor/status and never
  mutate the ordinary subscriber cursor or skip a gap/unsupported fact. Polling
  must not reconstruct historical world bounds from the current volume
  directory, and gap snapshots must preserve typed retired pinned members.
- Extraction records/bytes, live and lifetime volume records, presentation
  artifact/dirty/job/mesh/instance pools, dressing registrations, scheduled
  behavior registrations/order edges/view records/proposals/feedback, and
  asynchronous GPU extension registrations/WGSL bytes are separate named
  bounds. Do not make one pool silently own another.
- A command/query type owns its payload until admission succeeds. Queue-full or
  closed errors return the payload unchanged.
- Public query/interest/result/dressing records, scheduled Behavior ABI v1,
  and asynchronous Extension ABI v1 layouts in `public-api.md` and
  `behavior-scheduling.md` are normative. Do not replace closed variants,
  mandatory revision binding, or fixed offsets with implementation-defined
  blobs.
- No consumer, example, test harness, or feature may inspect storage internals.
  `tests/support` builds worlds exclusively through public APIs.
- No physics, damage, health, resistance, bond, fracture, gravity, force,
  generation recipe, player, camera policy, gameplay content, or
  world-specific axis assumption belongs in `src/`. Behavior modules may name
  only generic access, schedule, proposal, outcome, and lifecycle concepts.
- New dependencies require a short justification in the commit message and
  must use default features only when each default is required. Keep
  `Cargo.lock` committed.

### Naming and test rules

- Public opaque IDs end in `Id`; asynchronous accepted operations end in
  `Receipt`; immutable committed views end in `Snapshot`; lifecycle enums end
  in `State`; configured hard limits end in `Limit`.
- Systems use verb phrases (`admit_commands`, `dispatch_queries`); resources
  and components use noun phrases. Do not encode schedule order in function
  names.
- Every state transition and error variant needs a unit or headless-app test.
  Atomic publication, stale preconditions, observation gaps, restore mismatch,
  output overflow, queue pressure, behavior tick ordering/composition,
  adapter-state nonownership, and device loss require adversarial tests.
- Test-only fault injection is feature-gated under `test-support` and can fail
  only public production stages. It may not expose a bypass or alternate truth
  path.
- Golden persistence fixtures live under `tests/fixtures/` and are never
  rewritten by a passing test. Updating a fixture requires an explicit
  migration test.

## Traceability

| Approved design capability | Technical owner | Required evidence |
| --- | --- | --- |
| Configure one facade | `config`, `bevy`, [public-api.md](public-api.md) | Startup validation matrix and external-style harness |
| Sparse volume and deep 3D content | `storage`, `content` | Homogeneous-page and deep-volume GPU/CPU oracle tests |
| Bounded interest/lifecycle | `interest`, `volume` | Headless transition tests and bounded-residency scenario |
| Inspection and collision truth | `query`, `collision` | Exact query oracle; truth-versus-view scenario |
| Atomic mutation | `command`, `storage` | Forced post-admission failure and concurrent snapshot tests |
| Dynamic volumes | `volume`, `query` | Move/edit/query/checkpoint/restore scenario |
| Bounded observation | `observation` | Overflow-to-gap and resnapshot tests |
| Derived presentation/dressing | `presentation` | Revision install checks and diagnostic visual capture |
| Persistence scars | `persistence` | Semantic round trip and incompatible-base failures |
| Scheduled external behavior seam | `behavior`, `bevy`, `gpu`, `command` | Conventional CPU physics, GPU-resident physics, and CPU/GPU damage-and-bond adversarial adapters |
| Asynchronous inspection/effect jobs | `gpu`, `query`, `command` | Bounded WGSL packet/effect tool scenario |
| Telemetry and failure honesty | `telemetry`, all owners | Schema invariants and deliberate failure suite |

## Open Human Questions

None. The approved documents provide enough authority for implementation; all
remaining choices in this TDD are ordinary reversible engineering decisions.

## Completion definition for implementation

Implementation is contract-complete only when every required automated claim
in [validation.md](validation.md) passes, the public contract harness produces
a fail-closed evidence report, at least one physical adapter in each claimed
native backend family passes real-GPU parity and device-loss qualification, and
the presentation fixture has a recorded human visual decision. Architecture
feasibility gates P1–P9 are blocking physical-adapter receipts for each claimed
backend family; failure blocks the affected storage, mutation/query, collision,
materialization, presentation, scheduled behavior, asynchronous extension, or
checkpoint selection until the design is revised or passes. Correctness and
performance statuses remain separate, and neither can turn the other's failure
into a pass. P6 requires
both its 27-artifact local latency fixture and its 13,824-artifact
maximum-command fair-drain fixture; the local receipt alone is incomplete.
The local fixture is the eight-corner-cell patch whose halo union is exactly
27 artifacts, not an impossible single-cell fan-out.
