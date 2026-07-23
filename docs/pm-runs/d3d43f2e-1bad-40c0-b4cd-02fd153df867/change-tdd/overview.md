# Change TDD overview: substrate-scope correction

Status: **implementation plan proposed for review**

## Change boundary and authority

This TDD specifies the change from the repository at `4fe205c` to the
substrate-only product approved in `docs/product-scope-authority.md`. It is not
a replacement design for already-generic query, storage, mutation, streaming,
presentation-ownership, or evidence behavior.

Requirements in this TDD trace to:

- **[Authority]** `docs/product-scope-authority.md`;
- **[Design Rn/ACn]** the numbered requirements and acceptance criteria in this
  run's `change-design.md`;
- **[Analysis]** this run's repository analysis; and
- **[Repository]** code and contributor conventions inspected at `4fe205c`.

The authority wins conflicts with the existing `docs/tdd`, `AGENTS.md`, asset
contracts, issue set, or recovery branches. Until the active documents are
corrected, the run-scoped files in this directory describe the intended
replacement contract; they do not authorize weakening generic correctness
behavior.

## Existing architecture and conventions reused

The implementation keeps the existing Rust 2024, Bevy 0.19, Cargo resolver-3
workspace and its four intentional packages:

- `moria-world` remains the reusable library and sole owner of authoritative
  world truth.
- `moria-demo` becomes a thin headed diagnostic viewer.
- `moria-bench` remains the headed measurement consumer.
- `moria-curate` becomes the headless compact-input validator and
  `prove-substrate` runner.

This workspace is a justified exception to the XP single-binary starting point:
the repository already has a reusable library, separate viewer and benchmark
deliverables, and a validation tool. `Cargo.lock` and root `assets/` remain.
No crate, engine, serializer, ECS framework, package manager, web target, or
custom renderer is added by this change. Existing Cargo profiles remain:
project code at dev `opt-level = 1`, dependencies at dev `opt-level = 3`, and
native release with thin LTO and one codegen unit.

The following implemented patterns remain binding:

- `moria-world/src/lib.rs` is a thin explicit public facade. Consumer packages
  cannot import private modules, construct authoritative voxel components,
  expose `WorldStore`, or query arbitrary world entities.
- Consumers read through `WorldRead`, mutate through `WorldEditWrite`, and
  control activation through stable focus messages. The viewer, proof, and
  benchmark receive no privileged path.
- Authoritative coordinates and IDs use fixed-width integers. `usize` is only
  for local indexing; GPU-visible counters and indices are `u32`.
- Deterministic output is sorted before persistence or messages and does not
  depend on floating-point transcendental results, wall clock, task order,
  render LOD, or hash-map iteration order.
- Truth remains deterministic base generation plus sparse per-brick deltas.
  Derived terrain meshes, object meshes, water meshes, debug geometry, and
  other presentation are never persisted as authority.
- Bevy code remains grouped by feature plugin/domain. `main.rs` and `lib.rs`
  remain wiring/facade files; no repository-wide `systems`, `components`,
  `resources`, `types`, or `utils` bucket is introduced.
- Singleton configuration, queues, services, asset collections, and the opened
  world are resources. Per-view, per-object, per-render-chunk, and per-widget
  data are components. No type is both.
- Existing bounded synchronous query limits stay in force unless a focused
  implementation issue proves a generic correction is required. Removing
  player rationale does not silently remove the safety caps.
- Existing `MinimalPlugins`, `run_fixed_ticks`, dense conformance oracle,
  property-test, public-facade, report-validator, and atomic-output patterns are
  reused.

[Design R2, R6-R8; Analysis: Existing conventions; Repository]

## Selected implementation approach

### Compact identity and bounded generation

The corrected first release remains a finite, configurable `WorldBounds`; the
fixed Product One dimensions are removed. Bounds must align to the existing
4 m brick grid and the new 64 m generation-cell grid. This preserves current
bounded query semantics without preventing a later effectively unbounded
world. [Design open question 1, proposed default]

One authoritative `GenerationCellCoord` is a three-dimensional 64 m cube:
16 existing 4 m bricks on each axis. A request produces only its core cell.
Objects are owned by the cell containing their anchor. To include objects whose
shape/dependency crosses a core boundary, generation may inspect at most the
3 x 3 x 3 owner-cell neighborhood. Definition validation rejects any
registered object or authored anchor whose raw/dependency bounds require more
than this one-cell ownership context. Adjacent requests therefore agree without
generating the complete world. [Design R3-R5, AC3-AC4]

Each generated cell contains:

- deterministic procedural descriptors needed to evaluate its material truth;
- sorted registered objects owned by that cell;
- an owned per-cell sample/dependency index; and
- byte/accounting counters.

Cell results are pure values keyed by complete `WorldIdentity` and
`GenerationCellCoord`. Activation order, eviction, thread count, and repeated
requests cannot change the bytes or ordering. Core/context references are
counted separately; an unreferenced cell can be evicted and regenerated.
Sparse edit deltas outlive generated-cell eviction.

The exact data, digest, cell, stable-ID, index, and save contracts are in
[data-model.md](data-model.md).

### Runtime integration

`MoriaWorldPlugin` gains feature-local open, generation, streaming, edit,
persistence, and reconciliation plugins behind the existing facade. Opening a
world validates compact inputs, installs `WorldReadState`, and transitions the
existing `WorldLifecycle` resource. It does not load a complete object
population.

Frame schedules collect open/focus/input requests, poll tasks, install
revision-matched results, update the free diagnostic camera/UI, and advance
telemetry. `FixedUpdate` admits/schedules/commits edits, swaps loaded deltas,
and publishes deterministic invalidation. Explicit system sets order commit
before invalidation and invalidation before reconciliation scheduling. A task
result carries world/cell/revision/token identity; stale results are discarded.

Mutation messages expose sorted changed bricks and exact affected object IDs
per committed batch. Terminal reconciliation is reached only when every
expected derived item has a matching current-revision installed/removed
acknowledgement. Unaffected object fingerprints remain unchanged. Save/load
uses only the supported public request/result protocol and sparse delta truth.

The exact public and scheduling contracts are in [api.md](api.md).

### Generic object and presentation behavior

The current exact analytic shape, overlap, dependency, and broad-phase
algorithms are preserved, but `SpeciesId`, forest/canopy/Horizon names, route
semantics, and complete-manifest ownership are removed from the generic API.
Neutral registered-object kinds and optional authored sparse stamps are enough
to exercise object generation, overlap safety, invalidation, and rebuilding.

Each retained repeated visual kind shares mesh/material handles. Built-in Bevy
batching, culling, visibility ranges, and LOD are measured in representative
diagnostic scenes before any custom GPU path is proposed. Render
representations remain separate from simulation/truth.

[Design R5-R7, R13-R14; XP rendering guidance]

### Proof and viewer

`moria-curate check` validates only the compact definition, small bounded
fixture, stamps, and their canonical digests. `generate` and `prove-forest`
are removed; no command regenerates a complete population.

`moria-curate prove-substrate` is a headless release-mode F1 proof. It drives
the production public facade and production generator/index/edit/save paths,
while an independent bounded dense/brute-force oracle supplies expected
results. It emits immutable evidence with separate functional, accounting, and
measurement sections. Functional pass does not depend on machine name, GPU
backend, or resolution. Linux x86_64 and macOS arm64 reports for the same
commit/workload must have the same machine-independent correctness digest.

`moria-demo` composes a free camera, semantic diagnostic actions, streaming
focus, edits, material/voxel views, and reconciliation visualization. It
contains no player entity, locomotion, avatar, skeleton, animation, ecology
model, or curated traversal. F2 is a headed human checklist, not a timing gate.

[Design R9-R14, AC8-AC14]

## Design traceability

| Design requirement | TDD contract |
|---|---|
| R1 product identity | Active/historical disposition and term audit in `migration.md` |
| R2 shared consumer boundary | Facade, open, activation, edit, save, proof, viewer contracts in `api.md` |
| R3 compact versioned identity | Canonical definition/digests in `data-model.md` |
| R4 bounded deterministic generation | 64 m core, one-cell context, permutation/seam tests |
| R5 bounded fixtures | Compact definition anchors plus consumer-owned smoke fixture |
| R6 generic capabilities | Per-cell exact indexes/oracle, ownership, accounting, evidence |
| R7 exact mutation/reconciliation | Batch affected sets, fingerprints, revisioned barrier |
| R8 sparse save/reload | `moria-substrate-delta` v1 and atomic fixed-tick load swap |
| R9 `prove-substrate` | Twelve required F1 stages and exact CLI |
| R10 cross-platform F1 | Linux x86_64/macOS arm64 correctness-digest comparison |
| R11 evidence integrity | Four-state reports, digest/workload/machine binding, atomic output |
| R12 generated stress | Parameterized cell/candidate scenario under `target/` |
| R13 headed F2 | Surface/underground/stream/edit/view/object-rebuild checklist |
| R14 thin viewer | Free-camera diagnostic plugin using only public facade |
| R15 active artifact correction | Keep/generalize/remove/supersede ledger and cutover phases |
| R16 manifest removal | Coordinated compact-input gate replacement |
| R17 recovery PR audit | Per-change #363/#365 adaptation rules |
| R18 epic closure | Complete merged/adapted/superseded ledger before #325 closes |

## Expected file and module changes

This is the expected attachment map. Implementation issues may split a listed
module further within the same owning feature, but may not move behavior into
cross-repository item-kind buckets.

| Path | Change |
|---|---|
| `crates/moria-world/src/generation/identity.rs` | Add schema/generator/parameter/anchor digests and complete identity validation. |
| `crates/moria-world/src/generation/definition.rs` (new) | Compact `WorldDefinition`, canonical digest material, bounds and anchor validation. |
| `crates/moria-world/src/generation/cell.rs` (new) | 64 m cell coordinates, request/core/context rules, pure bounded generation result. |
| `crates/moria-world/src/generation/object_identity.rs` (new) | Collision-free packed object-ID derivation from owner-cell ordinal and local candidate slot. |
| `crates/moria-world/src/generation/{mod.rs,terrain.rs,biome.rs}` | Export corrected generic types; remove ecology names while preserving integer coordinate evaluation. |
| `crates/moria-world/src/storage/{coordinates.rs,store.rs}` | Replace Product One constants with validated opened-world bounds; compose per-cell base truth plus sparse deltas. |
| `crates/moria-world/src/curation/*` | Remove global manifest generation. Retain only bounded fixture/stamp validation behind the `curation` tool feature, or move cohesive neutral types into `generation::definition`. |
| `crates/moria-world/src/objects/{shapes.rs,index.rs,validation.rs,mod.rs}` | Generalize public terminology; make indexes owned per generation cell; preserve sorted exact oracle agreement and retained-byte accounting. |
| `crates/moria-world/src/query/{read.rs,sample.rs}` | Remove route/global-manifest state; read regenerated cells and deltas through current bounds. |
| `crates/moria-world/src/mutation/{execution.rs,reconciliation.rs}` (new), existing mutation files | Consume admitted reservations in `FixedUpdate`; expose exact affected sets and reconciliation lifecycle. |
| `crates/moria-world/src/persistence/` (new) | Versioned canonical sparse-delta snapshot, async I/O, validation, and fixed-tick atomic swap. |
| `crates/moria-world/src/streaming/*` | Plan generation-cell core/context ownership alongside existing brick bands; rename traversal/Horizon-only concepts to generic purposes. |
| `crates/moria-world/src/telemetry/{reports.rs,runtime.rs}` | Replace forest/M4 schemas with substrate proof, workload, cell/accounting, and complete machine evidence. |
| `crates/moria-world/src/config.rs`, `config_validation.rs` | Replace Product One/ecology/player schemas with world definition, diagnostic viewer, generic object, streaming, mutation, and benchmark settings. |
| `crates/moria-world/src/lib.rs` | Explicitly export the corrected facade and compose feature plugins; remove contaminated public exports. |
| `crates/moria-curate/src/` | Implement `check`, `prove-substrate`, and cross-host proof comparison; remove complete-manifest output. |
| `crates/moria-bench/src/cli.rs`, `capture/*`, `scenarios/*`, `main.rs` | Use generic workload/proof identity and corrected scenarios; retain truthful partial failure and atomic report output. |
| `crates/moria-demo/src/viewer/` (new), `main.rs` | Thin viewer plugin composition; semantic diagnostic input, free camera, overlays, and F2 scene. |
| `assets/config/substrate_world.ron` (new) | Small canonical compact world definition. |
| `assets/fixtures/substrate_smoke.ron` (new) | Bounded proof requests, references to reviewed neutral anchors, edits, and oracle samples. |
| `assets/stamps/diagnostic_arch.ron` (new or adapted) | Small neutral authored stamp used to prove anchor/object rebuilding. |
| `assets/config/curated_manifest.ron`, `assets/config/product_one_region.ron`, `assets/player/` | Remove from active product data; do not recreate. |
| Product One/forest/player asset declarations and placeholder test targets | Remove, or explicitly reclassify only a minimal neutral subset used by the viewer fixture. |
| `README.md`, `AGENTS.md`, `docs/design-document.md`, `docs/tdd/*`, reviews, issue/test-spec artifacts | Apply the disposition plan in [migration.md](migration.md); active commands and gates must no longer reference forest/player/M4 correctness. |

Cargo manifests change only to remove obsolete asset test targets or features
and register existing-package modules/tests. A new dependency requires a
separate TDD revision; the selected encoding uses existing Serde, JSON, SHA-256,
RON, and zstd support.

## Exact commands after this change

All commands run from the repository root.

### Ordinary implementation and CI gate

Run in this order:

```sh
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test
cargo run -p moria-curate -- check
cargo build --all-targets
```

The first four commands are the canonical Rust gate. `moria-curate check` is
the compact-input gate. `cargo build --all-targets` proves every
library/binary/test target builds. These commands use the dev profile.

### Diagnostic viewer development

```sh
cargo run -p moria-demo -- \
  --definition assets/config/substrate_world.ron \
  --fixture assets/fixtures/substrate_smoke.ron
```

### F1 headless substrate correctness

Run the first command on Linux x86_64 and the second on macOS arm64 from the
same commit and compact-input digests:

```sh
cargo run --release -p moria-curate -- prove-substrate \
  --definition assets/config/substrate_world.ron \
  --fixture assets/fixtures/substrate_smoke.ron \
  --output target/feasibility/substrate-linux-x86_64.json

cargo run --release -p moria-curate -- prove-substrate \
  --definition assets/config/substrate_world.ron \
  --fixture assets/fixtures/substrate_smoke.ron \
  --output target/feasibility/substrate-macos-arm64.json

cargo run -p moria-curate -- compare-proofs \
  --left target/feasibility/substrate-linux-x86_64.json \
  --right target/feasibility/substrate-macos-arm64.json
```

Release is used here for comparable measurement evidence, not because
functional results depend on optimization. A failure report remains immutable;
reruns use a new output path.

### Parameterized generated stress and headed measurements

```sh
cargo run --release -p moria-bench -- \
  --scenario generated-object-stress \
  --definition assets/config/substrate_world.ron \
  --cell-count 512 \
  --object-candidates-per-cell 256 \
  --output target/bench/generated-object-stress.json

cargo run --release -p moria-bench -- \
  --scenario diagnostic-streaming \
  --definition assets/config/substrate_world.ron \
  --fixture assets/fixtures/substrate_smoke.ron \
  --output target/bench/diagnostic-streaming.json

cargo run --release -p moria-bench -- \
  --scenario mutation-reconciliation \
  --definition assets/config/substrate_world.ron \
  --fixture assets/fixtures/substrate_smoke.ron \
  --proof target/feasibility/substrate-macos-arm64.json \
  --output target/bench/mutation-reconciliation.json
```

The numeric stress arguments identify that workload only; they are not
shipped-world counts or universal pass thresholds.

### F2 headed human inspection

On the reviewed macOS host:

```sh
cargo run --release -p moria-demo -- \
  --definition assets/config/substrate_world.ron \
  --fixture assets/fixtures/substrate_smoke.ron
```

The reviewer completes the repository's corrected F2 template into
`target/feasibility/f2-substrate-review.md`, recording commit, definition and
fixture digests, machine profile, each observation in Design AC12, and a
pass/fail disposition. The file is evidence output, not checked-in product
data.

## XP stack guidance applied

- The existing workspace, root assets, and committed lockfile are kept because
  separate deliverables and a strict public library boundary already exist.
- New behavior is placed in cohesive generation, persistence, mutation,
  viewer, proof, and benchmark feature modules/plugins. Entrypoints remain
  thin.
- Deterministic edits and load swaps run in `FixedUpdate`; frame input is
  converted to durable diagnostic intent before fixed ticks. Camera, UI, task
  polling, render installation, and visual interpolation stay frame-based.
- Logic uses pure/property tests and small `MinimalPlugins` apps with
  controlled fixed ticks. No logic test sleeps or opens a window. Rendering is
  checked by headed measurements and F2 human review.
- Repeated diagnostic objects share mesh/material handles and first use Bevy
  batching, culling, visibility ranges, and LOD. Custom rendering needs a
  measured TDD revision.
- Global world/config/queue/asset state stays in resources; cell/object/view/
  render instances stay in components or owned cell values.
- Viewer controls map physical input to semantic `DiagnosticAction`; raw
  keys/buttons do not enter mutation or world simulation.
- Generic collision detection remains a bounded query producing contacts.
  Consumer response/player locomotion is outside this change.
- The testing role should be assigned separately from implementation when
  issues are executed, preserving the XP adversarial-test expectation.

## Risks, compatibility, rollout, and unresolved decisions

### Compatibility and migration

The crate is pre-release (`0.1.0`). Generic behavior is preserved, but
contaminated public names are removed rather than maintained as deprecated
aliases: `CuratedManifest`, route/species/tree/Horizon-specific types,
`ForestFeasibilityReport`, player configuration/actions, and forest-proof CLI
flags have no compatibility promise. Downstream users migrate to compact
definitions, generation cells, neutral registered objects, substrate reports,
and diagnostic actions.

The unimplemented historical save contract and generated forest artifacts are
not load-compatible. The new sparse save schema fails explicitly on unsupported
schema/generator/world identity. No migration tool is required because the
current repository has no accepted public save implementation. The removed
global manifest is not archived elsewhere in active assets and must not be
recovered from PR #363.

The manifest, tests, Cargo target declarations, commands, and docs must change
in one dependency-ordered series so `moria-curate check` never points at a
missing contract. Detailed dispositions and recovery handling are in
[migration.md](migration.md).

### Technical risks

- A cell boundary bug could produce missing/duplicate objects. Ownership-cell,
  permutation, seam, and independent-oracle tests are blocking.
- Packed IDs depend on aligned finite bounds and cell-count/candidate caps.
  Validation must reject overflow before generation; it must never hash and
  hope collisions do not occur.
- Generalizing coordinates touches storage, admission, query, focus, and object
  indexes that currently duplicate Product One bounds. One shared validated
  conversion contract must replace every duplicate constant.
- Per-cell context references and sparse deltas have different lifetimes.
  Eviction tests must prove base/cache removal does not discard edits or alter
  regenerated output.
- Reconciliation can falsely pass if zero-work or stale render results are
  treated as current. Expected item keys, revision/token matching, and explicit
  removal acknowledgements are mandatory.
- Evidence migration can accidentally fabricate passes from absent data.
  Nullable partial reports, sorted failure reasons, cross-field validation,
  atomic output, and nonzero exit are retained from current/recovery work.
- Removing scenery assets may expose a thin F2 scene. The required neutral
  fixture must visibly exercise object rebuilding, but visual richness is not
  allowed to become ecology scope.

### Rollout and gates

Implementation is dependency ordered:

1. Correct active authority markers, commands, and the artifact disposition
   ledger; land compact schemas and small fixtures without deleting the old
   manifest gate prematurely.
2. Land shared bounds/identity/cell generation and per-cell object indexes with
   pure/oracle tests.
3. Integrate open/stream/edit/reconciliation/persistence behind the public
   facade and pass headless integration tests.
4. Replace curation/evidence CLIs, remove the global manifest and contaminated
   contracts, and pass the ordinary gate.
5. Produce matching Linux/macOS F1 correctness artifacts.
6. Only after F1 passes, finish the headed viewer/presentation slice, run
   parameterized stress and headed measurements, and complete F2.
7. Port only reviewed generic recovery changes and close epic #325 after its
   complete disposition ledger.

A failed F1 or F2 does not authorize lowering workload, exactness,
reconciliation, or evidence-integrity requirements. It requires a reviewed
change-TDD revision identifying the failed artifact, measured cause, redesign,
and replacement proof. This replaces the obsolete forest/M4 F1/F2 dependency
in `docs/tdd/implementation-plan.md`.

### Product decisions still open

No universal timing, graphics-memory, or per-profile performance threshold is
approved by the source authority. This TDD records timings and resources
truthfully and treats deterministic capacity, bounded-work, exactness, and
evidence consistency as blocking correctness. If Product wants performance to
block release on a named hardware profile, it must approve profile-specific
thresholds in a later change; implementation agents may not copy the old
M4/3060 values.

The exact supported Linux distribution/version and macOS version are also not
approved. F1 binds artifacts to complete observed OS/toolchain identities and
requires Linux x86_64 plus macOS arm64 as distinct stacks, but release support
claims beyond that matrix require Product/operations approval.
