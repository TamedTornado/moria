# Moria technical design

Status: implementation-ready

Binding product contract: [`../design-document.md`](../design-document.md)

Supporting authority: [`../product-vision.md`](../product-vision.md) and
[`../product-design-decisions.md`](../product-design-decisions.md)

## 1. Purpose and authority

This TDD selects an implementation for the approved Moria product design. The
product design wins if this document is ambiguous. Product behavior is not
inferred from seed documents or the historical implementation record.

Moria will be a Rust/Bevy crate family whose authoritative sparse matter is
resident in GPU buffers while it is ready. CPU state contains orchestration
metadata, consumer-supplied base payloads in transit, and sparse durable scars;
it is not a full authoritative mirror. Every consumer, including repository
validation programs, uses the same facade.

The detailed contracts are split as follows:

- [`public-contract.md`](public-contract.md): public Rust concepts, requests,
  results, error taxonomy, and revision semantics.
- [`matter-and-gpu.md`](matter-and-gpu.md): coordinates, sparse encoding,
  GPU ownership, atomic publication, GPU extension ABI, and resource control.
- [`runtime.md`](runtime.md): scheduling, region lifecycle, observations,
  shutdown, and failure recovery.
- [`queries-and-presentation.md`](queries-and-presentation.md): inspection,
  collision, meshing, dressing, and freshness.
- [`persistence.md`](persistence.md): verified base lineage, scar format,
  checkpoint cuts, restore, and durability.
- [`validation.md`](validation.md): test layers, acceptance scenarios,
  evidence schemas, performance gates, and portability matrix.

## 2. Selected architecture

```text
consumer / behavior plug-in / validation executable
                         |
                 moria public facade
                         |
       admission + receipts + observations + budgets
          /              |                 \
 content source     runtime scheduler     persistence sink
                          |
                   GPU work dispatcher
          /               |                  \
 sparse matter       truth queries       derived presentation
 and COW commits     and collision        and dressing
```

The architecture has five ownership rules:

1. `moria` owns the public facade, IDs, request/result types, and Bevy plugin.
2. `moria-core` owns lifecycle, admission, revisions, queue reservations,
   observation history, and checkpoint coordination.
3. `moria-gpu` owns authoritative ready-region pages, page tables, mutation
   transactions, queries, collision kernels, and derived GPU artifacts.
4. `moria-persist` owns the versioned checkpoint codec and transactional sink
   protocol. A consumer owns the actual storage service.
5. Consumers own base content algorithms, behavior policy and state, and the
   decision to display stale, diagnostic, or absent presentation.

No internal crate exposes an authoritative page, mutable GPU buffer, or
storage handle through the public facade. The only GPU-oriented extension is a
bounded, read-only observation lease plus a Moria-owned proposal buffer,
specified in `matter-and-gpu.md`.

## 3. Workspace and module structure

The implementation will create this workspace:

```text
.
├── .cargo/config.toml             # aliases only; no machine-specific flags
├── rust-toolchain.toml            # Rust 1.97.1, rustfmt, clippy
├── crates/
│   ├── moria-contract/            # semver public data types, no Bevy renderer
│   │   └── src/{id,math,material,volume,interest,command,query,
│   │            observation,persistence,telemetry,error}.rs
│   ├── moria-core/                # private orchestration implementation
│   │   └── src/{world,admission,scheduler,lifecycle,revision,
│   │            receipt,observation,budget,shutdown}.rs
│   ├── moria-gpu/                 # private wgpu implementation and WGSL
│   │   ├── src/{device,page_pool,page_table,materialize,mutation,
│   │   │        query,collision,mesh,dressing,extension,recovery}.rs
│   │   └── shaders/*.wgsl
│   ├── moria-persist/             # private portable codec and sink protocol
│   │   └── src/{format,manifest,scar,checkpoint,restore,sink}.rs
│   ├── moria/                     # only supported consumer dependency
│   │   └── src/{lib,plugin,config,facade,bevy_events}.rs
│   ├── moria-testkit/             # dev-only public-boundary fixtures/oracles
│   └── moria-tools/               # shader/layout/schema validation binary
├── validation/
│   └── moria-validation/          # ordinary consumer; headless + visual
└── xtask/                          # repository gate/evidence orchestration
```

`moria` re-exports the stable surface of `moria-contract`; external consumers
do not depend on internal crates. `moria-testkit` may depend only on `moria`
and independent oracle libraries, never `moria-core` or `moria-gpu`. The
validation executable has the same restriction. Unit tests inside private
crates may inspect their own internals, but no acceptance proof may do so.

Dependency direction is acyclic:

- `moria-contract` has no dependency on another Moria crate.
- `moria-core` depends on `moria-contract`.
- `moria-gpu` depends on `moria-contract` and selected Bevy render crates, not
  on `moria-core`.
- `moria-persist` depends on `moria-contract`, not on core or GPU.
- `moria` depends on all four and owns the adapter implementations.
- `moria-testkit` and `moria-validation` depend only on `moria`.

`moria-core` defines private ports implemented by adapter wrappers in `moria`;
it must not import Bevy render types. `moria` performs Bevy schedule
integration, constructs the GPU/persistence implementations, and connects them
to those ports.

## 4. Toolchain and dependency policy

- Pin Rust `1.97.1`; the workspace MSRV is `1.95.0`, matching Bevy 0.19.
- Pin Bevy crates to `=0.19.0`. Use Bevy's wgpu `29.0.3`; do not introduce a
  second wgpu major.
- WGSL is the only checked-in shader source. Runtime-generated shaders,
  backend-specific shader forks, and unsafe vendor extensions are forbidden.
- `Cargo.lock` is committed. Direct dependency versions use exact versions in
  workspace dependencies; updates are deliberate review changes.
- Production crates deny `unsafe_code`. A narrowly isolated unsafe block is
  allowed only in `moria-gpu::abi` when a safe alternative is unavailable,
  with a `# Safety` contract and Miri-covered host-side tests.
- New runtime dependencies require a reason in the pull request and must not
  introduce generation, physics, damage, gameplay, or storage-service policy.

Native tier-1 targets are Linux/Vulkan, Windows/DX12, and macOS/Metal through
wgpu. A GPU satisfying the limits in `matter-and-gpu.md` is required; startup
fails explicitly rather than installing a CPU world. Web/wasm is not a current
target. A deterministic CPU reference exists only in `moria-testkit` as an
oracle and cannot be selected as the production authority.

## 5. Cross-cutting invariants

These invariants are enforceable assertions and acceptance-test obligations:

1. A ready region's current root and revision name one immutable GPU snapshot.
2. A matter command targets exactly one volume and one bounded local region.
   Its root publication is one atomic visibility event.
3. A public query captures committed root/revision pairs at dispatch and never
   reads transaction staging pages.
4. Unknown, cold, failed, and budget-blocked matter are distinct from empty.
5. Presentation, dressing, debug geometry, and CPU oracle data are never inputs
   to truth queries, collision, mutation, or persistence.
6. A committed matter or placement change has a durable sparse scar capture
   reserved before it can publish; dirty capture pins the required data.
7. A dynamic placement changes the local-to-world isometry only. It never
   resamples local matter.
8. Overlap results retain every matching volume; no implicit composition or
   contact policy chooses a winner.
9. Every queue, output, history, staging allocation, and resident pool has a
   configured finite limit and an explicit exhaustion outcome.
10. Device loss, source failure, persistence failure, and observation lag do
    not become empty matter or successful completion.

## 6. Product traceability

| Approved design capability | Technical realization | Required evidence |
| --- | --- | --- |
| One public consumer contract (§2.2, §4) | `moria` facade, contract-only testkit and validation crate | V01 |
| Volume-general static and dynamic matter (§3, §6) | finite local cell domains, double-precision isometric placement, per-volume revisions | V02, V08 |
| Cheap sparse scale (§2.4, §5.3) | 8³ uniform/dense bricks, radix page table, budgeted residency and retirement | V03 |
| Explicit asynchronous work (§2.5, §4) | admitted receipts, query tickets, checkpoint tickets, terminal state machines | V01, V09 |
| Atomic bounded mutation (§4.4, D1) | per-volume serialized COW transaction and one root-selector publication | V04 |
| Truth inspection and collision (§4.3, §6) | revision-captured compute queries against matter pages | V05 |
| Bounded observations and gap recovery (§4.5) | sequence-numbered per-subscription rings and bounded snapshots | V06 |
| GPU-oriented behavior seam (§2.5, §4.5) | read-only snapshot bind group plus Moria-owned proposal buffer and ordinary admission | V07 |
| Derived organic/crisp views and dressing (§4.6) | revision-tagged surface-nets/greedy hybrid and anchored instance sets | V05 |
| Base plus scar persistence (§4.7) | Merkle-verified base root, sparse final-value scars, transactional checkpoint | V08 |
| Honest pressure, failure, telemetry (§5.3, §8, §9) | stable errors, reservations, lifecycle reasons, evidence counters | V09 |

The `Vxx` scenario definitions are in `validation.md`. No current technical
module implements a product-excluded generator, physics engine, damage model,
game controller, gameplay object, or ship/station validation target.

## 7. Intended `AGENTS.md`

The repository root `AGENTS.md` created with the workspace must state the
following rules exactly enough that an implementation agent can execute them
without interpretation.

### Formatting and documentation

- Run `cargo fmt --all -- --check`; committed Rust uses default rustfmt.
- Markdown uses one sentence per line where practical, ATX headings, fenced
  code blocks with language tags, and relative links for repository files.
- Every public item has rustdoc. Public fallible functions document `# Errors`;
  public panics are forbidden. Unsafe items additionally document `# Safety`.
- WGSL entry points and host ABI structs name the shared layout version.

### Naming and code rules

- Types and traits are `UpperCamelCase`; functions, modules, fields, and
  shader entry points are `snake_case`; constants are `SCREAMING_SNAKE_CASE`.
- ID newtypes end in `Id`, revisions end in `Revision`, asynchronous handles
  end in `Ticket` or `Receipt`, and terminal results end in `Outcome`.
- Coordinates include their space in the type (`LocalCell`, `LocalPoint`,
  `WorldPoint`); untyped `[i32; 3]` and `Vec3` at public boundaries are banned.
- Internal crates remain private implementation details. Acceptance and
  validation code imports only `moria`.
- No direct buffer mapping, page-table access, or feature-gated privileged
  facade is added for examples, tests, diagnostics, or behavior plug-ins.
- No unbounded channel or collection is used for runtime work. Allocation is
  charged to a named `MoriaLimits` field before admission.
- Production code contains no generator, motion response, gravity, damage,
  health, fracture, gameplay, or consumer persistence policy.

### Required commands

From the repository root:

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
cargo run -p moria-tools -- shaders check
cargo run -p moria-tools -- schemas check
cargo run -p moria-validation -- headless --output target/moria-evidence/headless.json
cargo run -p xtask -- ci
```

`cargo run -p xtask -- ci` runs all preceding non-visual commands and fails if
an evidence report is missing, malformed, or says a required scenario was not
demonstrated. It is the merge gate. Build and development commands are:

```sh
cargo build --workspace --all-targets --all-features
cargo run -p moria-validation --features visual -- visual
```

The visual command is a diagnostic consumer, not a gameplay deliverable or a
substitute for headless contract evidence.

### Change obligations

- A public contract change updates `public-contract.md`, golden API tests, and
  at least one black-box scenario.
- A host/shader ABI change increments its ABI version and updates shader layout
  tests on all tier-1 adapters available in CI.
- A checkpoint format change adds a new reader fixture; existing format
  readers are never silently reinterpreted.
- An optimization includes before/after evidence with adapter, driver, OS,
  CPU, GPU, limits, and scenario configuration. Correctness gates run first.
- Tests that inject post-admission failure, device loss, observation gaps, or
  corrupt persistence remain fail-closed and deterministic.

## 8. Delivery order

Implementation should proceed by risk, not by demo appearance:

1. contract types, limits, CPU oracle, and lifecycle state machines;
2. GPU capability probe, sparse pages, verified materialization, and sampling;
3. COW atomic mutation and scar capture;
4. bounded query/collision primitives and observation recovery;
5. checkpoint/restore and device recovery;
6. hybrid presentation and dressing;
7. GPU extension proposal path, portability qualification, and optimization.

Each stage lands only with its corresponding black-box failure tests. This
ordering does not weaken the complete product contract.

## Open Human Questions

None. The approved documents leave substantial engineering discretion, but no
remaining choice requires product authority. Algorithm and threshold choices
in this TDD are reversible behind the approved public contract.
