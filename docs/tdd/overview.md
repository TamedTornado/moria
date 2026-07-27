# Moria Technical Design

Status: implementation-ready

Product contract: [`../design-document.md`](../design-document.md)

Supporting authority: [`../product-vision.md`](../product-vision.md) and
[`../product-design-decisions.md`](../product-design-decisions.md)

## 1. Purpose and authority

This TDD turns the approved Moria product design into implementable contracts.
The approved design controls when this document conflicts with product intent.
The other files in this directory are normative parts of this TDD:

- [`public-api.md`](public-api.md) defines consumer-visible Rust types and
  operations.
- [`matter-and-storage.md`](matter-and-storage.md) defines coordinates,
  material truth, sparse GPU storage, atomic transactions, and collision.
- [`runtime.md`](runtime.md) defines scheduling, state machines, observations,
  failure handling, and shutdown.
- [`persistence.md`](persistence.md) defines base-content proof, scars,
  checkpoints, and restore.
- [`presentation-and-extensions.md`](presentation-and-extensions.md) defines
  derived surfaces, dressing, and the bounded GPU behavior seam.
- [`resources-and-portability.md`](resources-and-portability.md) defines hard
  bounds, budget behavior, supported targets, and telemetry.
- [`validation.md`](validation.md) defines automated evidence and acceptance.
- [`traceability.md`](traceability.md) maps every approved capability to its
  technical contract and proof.

Normative terms such as **must**, **must not**, **may**, and **should** have
their usual RFC 2119 meanings. Examples never weaken a normative statement.

## 2. Selected architecture

Moria is one publishable Rust crate, `moria`, integrated as a Bevy plugin. Its
public facade is deliberately smaller than its internal implementation:

```text
consumer Bevy App
  |
  | MoriaPlugin + MoriaClient (only supported consumer boundary)
  v
main-world facade
  |-- validates bounded requests and manages receipts/subscriptions
  |-- owns identities, revisions, lifecycle, CPU scar index, persistence I/O
  `-- sends immutable work packets across bounded channels
          |
          v
Bevy RenderApp / MoriaRenderPlugin
  |-- private sparse page table and immutable-base/current brick pools
  |-- private compute pipelines for materialization, mutation, query/collision
  |-- copy-on-write transaction pages and revision-tagged derived work
  `-- bounded result/readback rings
          |
          v
wgpu compute and render queues
```

Authoritative detailed cell data lives in GPU buffers after materialization.
CPU state contains configuration, volume and lifecycle metadata, compact
homogeneous descriptors, sparse scar records, and bounded staging/readback
data. It is not a cell-for-cell mirror. No public type exposes the page table,
brick pool, storage buffers, mutable bind groups, or internal ECS entities.

The main Bevy world is the sole public control plane. Render-world systems are
private implementation. Public methods are nonblocking: they either reject
before admission or return a typed receipt/ticket/lease. GPU completion is
reported later through polling and observations.

### 2.1 Technology baseline

- Rust `1.96.1`, edition 2024, pinned by `rust-toolchain.toml`.
- Bevy `0.19.0`, declared exactly and pinned in `Cargo.lock`; Moria uses
  individual Bevy crates and disables unneeded default features.
- WGSL compute/render shaders compiled by Bevy's matching render stack. Moria
  does not add a second independently versioned `wgpu` dependency.
- `serde` for public configuration and persistence DTOs, `thiserror` for error
  definitions, `blake3` for canonical digests, and `uuid` with random and serde
  support for persistent IDs.
- `postcard` payloads inside a versioned binary persistence envelope. The
  envelope is Moria-owned; a consumer-provided `CheckpointStore` owns actual
  durability.

The exact patch versions are committed once during workspace bootstrap and
updated only in an explicit dependency change. A Bevy minor upgrade is an
architectural migration because render-world APIs and shader bindings are not
stable across minors.

### 2.2 Why one publishable crate

One crate makes the supported boundary mechanically clear: external consumers
can only import `moria`. Internal modules can use `pub(crate)` without
accidentally creating a second semi-public storage API. Separate ordinary
consumer and tooling packages test that boundary without being shipped as
substrate APIs.

### 2.3 Internal ownership

| Owner | Owns | Must not own |
| --- | --- | --- |
| `facade` | Plugin, builders, `MoriaClient`, typed IDs, API DTOs | GPU handles or gameplay policy |
| `runtime` | Admission, queues, receipts, lifecycle, revisions, subscriptions | Cell storage or derived truth |
| `matter` | Cell semantics, bounds, source validation, scar calculation | Consumer generation algorithms |
| `gpu` | Page table, pools, transactions, compute dispatch, readback rings | Public API or persistent I/O |
| `collision` | Point/region/trace/shape query planning and facts | Motion integration or response |
| `presentation` | Revision-tagged mesh/dressing derivation and status | Collision authority |
| `persistence` | Canonical scars, checkpoint envelope, restore protocol | Derived meshes or consumer state |
| `telemetry` | Stable metrics/events and diagnostic snapshots | Raw storage escape hatches |

Dependencies point downward from `facade`/`runtime` into private domain
modules. `matter`, `collision`, `persistence`, and `presentation` may share
small internal value types but may not call the facade. `gpu` implements work
requested by `runtime`; it never performs consumer admission.

Cross-module calls use typed immutable packets:

- `runtime -> gpu`: reserved operation packet plus captured catalog/volume
  revisions;
- `gpu -> runtime`: bounded completion record plus an owned readback slice;
- `runtime <-> persistence`: immutable checkpoint cut or validated private
  restore state;
- `matter -> presentation/collision`: internal cell/occupancy value types, not
  storage handles; and
- `telemetry`: receives bounded facts from owners and cannot call mutation or
  query kernels.

Only `runtime` may publish a revision, receipt terminal state, lifecycle
transition, or observation. Only `gpu` may mutate private page-table bindings.
Only `persistence` may mark a revision durable.

## 3. Repository and module contract

The implementation will create this structure:

```text
Cargo.toml
Cargo.lock
rust-toolchain.toml
rustfmt.toml
clippy.toml
deny.toml
.cargo/config.toml
AGENTS.md
crates/
  moria/
    Cargo.toml
    src/
      lib.rs
      facade/
      runtime/
      matter/
      collision/
      gpu/
      presentation/
      persistence/
      telemetry/
    shaders/
  moria-conformance/
    Cargo.toml
    src/
    tests/
examples/
  moria-lab/
    Cargo.toml
    src/
tools/
  xtask/
    Cargo.toml
    src/
docs/
  tdd/
```

Workspace packages:

- `moria`: the only publishable product crate.
- `moria-conformance`: a non-publishable library/binary that depends only on
  `moria`'s public API. It owns deterministic fixture content sources,
  fault-injection stores, scenario orchestration, and evidence schemas.
- `moria-lab`: a non-publishable ordinary Bevy consumer for interactive and
  visual checks. It contains cameras, controls, fixture generation, and assets;
  none are Moria features.
- `xtask`: repository automation for shader checks, schema checks, conformance,
  and evidence collation.

`moria-conformance`, `moria-lab`, and `xtask` must never use `pub(crate)`
features, unsafe path dependencies into source modules, internal ECS queries,
or raw storage handles. A compile-fail test verifies that representative
private module paths are inaccessible.

Within `crates/moria/src`, every owner directory has a `mod.rs` that exports
only `pub(crate)` domain types to sibling owners; files below it are private by
default. Cross-owner DTOs live in `src/internal.rs` only when at least two
owners need the exact value and the ownership table above identifies the
writer. Public DTOs live under `facade` and are re-exported deliberately from
`lib.rs`; internal owners do not define look-alike public IDs.

### 3.1 Naming and dependency rules

- Rust files and modules use `snake_case`; types/traits use `UpperCamelCase`;
  methods and fields use `snake_case`; constants use `SCREAMING_SNAKE_CASE`.
- Persistent wire fields use descriptive names, never one-letter names.
- IDs are distinct newtypes (`WorldId`, `VolumeId`, `ReceiptId`, and so on);
  bare UUIDs or integers are not accepted across ownership boundaries.
- Coordinate spaces are present in names or types: `LocalCell`, `LocalPoint`,
  and `WorldPoint` are not interchangeable vectors.
- Public APIs contain no `Gpu`, `Buffer`, `Page`, `BrickIndex`, or Bevy
  `Entity` identifiers except the explicitly opaque GPU extension token types.
- Production dependencies are declared in `[workspace.dependencies]` with
  exact versions. Package manifests use `workspace = true`.
- New production dependencies require a license/security check via
  `cargo deny check`, a short rationale in the change, and no duplicate
  implementation already available through Bevy or `std`.
- `unsafe` is denied in `moria` by default. A narrowly scoped exception needs a
  module-level safety contract and an ADR; shaders are validated instead of
  relying on host `unsafe`.
- Public fallible operations return typed errors. Panics are bugs, not consumer
  error handling. Indexing untrusted input and unchecked numeric casts are
  forbidden.
- The optional `fault-injection` feature exposes a documented public
  configuration hook for selecting an operation ID and pre-commit failure
  stage. It never bypasses admission or exposes/mutates storage directly; any
  external consumer can enable the same hook. It exists only to prove public
  failure semantics and is disabled by default.
- Wall-clock time, hash-map iteration order, and thread scheduling must not
  affect committed truth or canonical persistence bytes.

## 4. Intended `AGENTS.md`

The repository-root `AGENTS.md` created with implementation must contain the
following operational contract. It may add explanatory prose but may not
weaken or rename these commands.

All commands below run from the repository root with the committed
`rust-toolchain.toml` and `Cargo.lock`. No command may download unpinned helper
scripts, depend on an uncommitted asset, or convert a missing GPU adapter into a
pass. Required developer tools are the pinned Rust components `rustfmt` and
`clippy`, plus `cargo-deny` at the exact version recorded in
`.cargo/config.toml`/the bootstrap instructions.

### Formatting

```sh
cargo fmt --all -- --check
```

Run `cargo fmt --all` to repair formatting. Markdown uses one sentence per line
where practical, fenced blocks with a language, no trailing whitespace, and
relative links for repository documents.

### Check

```sh
cargo check --workspace --all-targets --all-features
cargo run -p xtask -- shader-check
cargo run -p xtask -- schema-check
```

### Lint

```sh
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
```

Workspace lint policy sets `unsafe_code = "deny"`, `missing_docs = "warn"` for
the public crate, and denies `unwrap_used`, `expect_used`, lossy numeric casts,
and wildcard imports in production code. Tests may use `expect` with a message.

### Test

```sh
cargo test --workspace --all-targets --all-features
cargo run -p moria-conformance -- \
  --suite contract \
  --adapter auto \
  --output target/evidence/contract.json
```

The first command contains unit, property, compile-fail, persistence golden,
and shader-interface tests. The conformance command requires a wgpu compute
adapter; absence is a reported `not_demonstrated` result and a failing exit,
not a skipped pass.

### Release qualification

```sh
cargo run -p xtask -- check
cargo run -p moria-conformance -- \
  --suite contract \
  --adapter auto \
  --output target/evidence/contract.json
cargo run -p moria-conformance -- \
  --suite performance \
  --adapter auto \
  --output target/evidence/performance.json
cargo run -p xtask -- visual-review-check \
  --contract target/evidence/contract.json \
  --review target/evidence/visual-review.json
```

These commands implement the release gate in `validation.md`. The visual review
record is supplied by the human review workflow; automation validates it but
does not manufacture approval.

### Build and documentation

```sh
cargo build --workspace --all-targets --all-features
cargo doc --workspace --no-deps --all-features
```

### Development

```sh
cargo run -p moria-lab -- --scenario deep-volume
cargo run -p moria-conformance -- \
  --suite smoke \
  --adapter auto \
  --output target/evidence/smoke.json
cargo run -p xtask -- check
```

`xtask check` runs formatting, check, shader/schema checks, clippy, unit tests,
and `cargo deny`; it does not claim GPU conformance when no adapter is present.
All generated evidence stays under `target/evidence/` and is not committed
unless a later validation plan explicitly requests a curated record.

### Implementation constraints to state in `AGENTS.md`

1. Read `docs/design-document.md` and this TDD before changing public behavior.
2. Keep product behavior out of fixture/harness code and keep harness behavior
   out of `moria`.
3. Consumers, examples, and tests use only `moria` public exports.
4. Never expose or persist derived geometry as truth.
5. Every request is checked against configured bounds before allocation or
   dispatch; use checked integer arithmetic.
6. Matter mutations stage privately and commit once or not at all.
7. Unknown/cold/failed matter is never represented as empty.
8. Add failure-path and evidence assertions with every capability.
9. Persistence format changes require versioning, golden fixtures, and explicit
   compatibility behavior.
10. Any new public API or storage-layout change updates this TDD and the
    traceability matrix in the same change.

## 5. System invariants

The following invariants are release blockers:

1. One committed volume revision names one material-and-placement state.
2. Only successful commit changes a revision. Rejection, cancellation before
   commit, and failure do not.
3. One matter command targets one volume and is all-or-nothing across every
   affected cell. Cross-volume intent uses separate commands.
4. Public query/collision results are computed from authoritative matter and
   carry the exact revisions used.
5. A result never maps cold, pending, failed, out-of-domain, or overflowed
   matter to empty.
6. Derived presentation and dressing are disposable and revision-tagged.
7. An admitted operation pins every required region and transaction allocation
   until a terminal outcome.
8. Dirty scars remain in memory or durable storage through retirement and
   shutdown unless the consumer explicitly authorizes loss through the
   destructive discard API described in `runtime.md`.
9. Observation loss becomes a gap marker before later facts are delivered.
10. Persistence replays scars only when exact base-content identity can be
    established.
11. CPU state never grows in proportion to every cell of an untouched volume.
12. External GPU behavior observes bounded immutable snapshots and emits
    untrusted effect requests; it never receives authoritative storage.
13. A ready brick retains a proof-verified immutable base view until eviction,
    so canonical scars are computed without guessing or a CPU cell mirror.
14. A commit barrier prevents old revision metadata from being paired with
    post-commit GPU cells.
15. World-scope no-hit/completeness claims use one captured catalog/AABB-index
    snapshot.
16. Collision truth is the thresholded occupied-cell union; smooth or
    constructed presentation never changes it.

## 6. Delivery sequence

This ordering reduces architectural risk without changing product completion:

1. Establish typed domains, public facade, deterministic reference model, and
   bounded admission.
2. Establish private GPU page tables, homogeneous/mixed bricks,
   materialization, and revisioned sample/region queries.
3. Add copy-on-write mutation and fault-injected atomicity proof.
4. Add remaining collision queries and dynamic placements.
5. Add lifecycle pressure, observation gap recovery, scars, checkpoint, and
   restore.
6. Add presentation, dressing invalidation, and visual truth-versus-view proof.
7. Add bounded GPU extension snapshots/effects and full evidence telemetry.
8. Optimize only against passing semantic evidence; layout or kernel changes
   may not alter the facade contract.

Each stage merges only with fail-closed tests for the behavior it claims.

## 7. Open Human Questions

None.

All consequential product choices are resolved by the approved design and
product decision record. Brick size, encoding, API shape, persistence envelope,
rendering technique, supported native backends, resource defaults, and
validation thresholds are ordinary engineering choices selected in this TDD.
