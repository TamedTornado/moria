# Change analysis: substrate-scope correction

## Analysis basis

This analysis uses the repository at commit `e6c8a1e` (the current
`origin/master` in this worktree) as the source of truth for implemented
behavior and conventions. The only committed change source is
`docs/product-scope-authority.md`; it was read in full. That authority was
approved on 2026-07-23 and explicitly supersedes conflicting active product,
design, TDD, planning, benchmark, test, asset, issue, and contributor material.

The repository also contains the Git objects for recovery PR heads #363
(`956063d`) and #365 (`1985b95`). Their commit histories and diffs from their
respective merge bases were inspected locally. This is enough to identify
candidate attachment areas, but it is not a line-by-line disposition of every
recovery hunk.

In the sections below, **Fact** describes the present repository or approved
authority. **Recommendation** identifies where the requested correction should
attach without prescribing an implementation design.

## Current product shape

### Product boundary already stated at the top level

**Fact:** `README.md` now describes Moria as a reusable voxel-world substrate,
not a game layer. It says that player controllers, characters, skeletal
animation, and game-specific presentation are outside the accepted boundary.
The approved authority agrees and gives the substrate these responsibilities:

- deterministic procedural three-dimensional material truth;
- sparse brick storage, activation, streaming, and eviction;
- terrain and registered-object meshing;
- bounded spatial queries and collision primitives;
- exact mutation dependency attribution and rebuilding;
- delta persistence and deterministic reload;
- telemetry, resource accounting, benchmarks, and evidence integrity; and
- a thin headed diagnostic viewer.

### What is actually implemented

**Fact:** The repository is an active, incomplete Rust/Bevy substrate. The
generic library contracts are substantially further along than the runnable
products:

- `moria-world` exposes fixed-width coordinate/material types, deterministic
  base terrain evaluation, sparse brick/delta storage internals, bounded
  sampling/ray/capsule/diagnostic query types, registered-object shapes and
  spatial indexes, mutation admission, streaming focus/planning lifecycle
  primitives, lifecycle state, asset validation, and telemetry/report schemas.
- `MoriaWorldPlugin` currently installs lifecycle, edit-admission, focus, and
  runtime telemetry resources; focus/edit messages; an `Update` focus-message
  system; a `PostUpdate` frame counter; and the Basis KTX2 loader when an
  `AssetServer` exists. It does not yet open a generated world, install
  `WorldReadState`, execute accepted edits, run streaming planners, mesh
  terrain/objects, persist deltas, or reconcile renderer work.
- `moria-demo/src/main.rs` only starts `DefaultPlugins` and
  `MoriaWorldPlugin`. There is no existing player/controller/camera module to
  remove. The current executable is therefore a headed shell, not yet the
  walkable demo described by the old design.
- `moria-bench/src/main.rs` parses a scenario, starts a headed app, and then
  deliberately writes a valid `failed_before_start` report because complete
  scenario evidence is not captured. The query-cost probe and evidence schema
  have real code and tests, but the runner does not implement flythrough,
  mutation, or feasibility scenarios end to end.
- `moria-curate` is operational for `generate`, `check`, and `prove-forest`.
  It derives and validates a complete forest-oriented `CuratedManifest`.
  `assets/config/curated_manifest.ron` is approximately 6.1 MB in the current
  checkout and is the checked-in global population that the new authority says
  must be removed rather than recreated.

**Fact:** `README.md` accurately separates implemented contracts from
unaccepted F1/F2, graphics-memory, visual, and release claims. Its remaining
forest/index curation wording and `prove-forest` command are now stale under the
new authority.

## Requested change summary

**Fact:** `docs/product-scope-authority.md` requests a correction of the
existing product, not a new product. It requires:

1. removing ecology, canonical forest, player, character, animation, curated
   traversal, and universally binding named-machine requirements;
2. preserving substrate-generic work such as stable registered-object IDs,
   deterministic bounded generation, compact broad-phase indexing, an
   independent exact oracle, exact mutation attribution, presentation overlap
   safety, invalidation/reconciliation, authored-versus-derived ownership,
   bounded accounting, and truthful evidence;
3. replacing the global enumerated population with a world seed, compact
   procedural parameters, generator/schema versions and digests, explicitly
   authored bounded anchors, and small reviewable fixtures;
4. making runtime generation deterministic and bounded by region or cell, with
   stable identities, without enumerating the complete world first;
5. replacing `prove-forest` with a substrate-oriented `prove-substrate`;
6. defining F1 as cross-platform headless substrate correctness/resource/
   evidence validation and F2 as headed human inspection of terrain,
   underground geometry, streaming, edits, diagnostic voxel views, and object
   invalidation/rebuilding;
7. inventorying and correcting active documents, code, tests, assets, issues,
   and recovery work before implementation;
8. auditing PR #363 without merging its generated forest population, and
   auditing PR #365 for generic evidence integrity while removing its
   forest/third-person/universal-machine assumptions; and
9. retaining conflicting historical artifacts as history while clearly
   marking them superseded.

**Recommendation:** The immediate change should remain a documentation,
contract, TDD, issue, and test-spec correction. The approved sequence explicitly
forbids compact-data-path implementation before reviewed analysis, design, TDD,
issue decomposition, and test specifications exist.

## Existing technology stack and commands

### Stack

**Fact:** `Cargo.toml`, package manifests, and `Cargo.lock` establish:

| Area | Current repository fact | Source |
|---|---|---|
| Language/tooling | Rust edition 2024 in a Cargo resolver-3 workspace; committed `Cargo.lock` | root `Cargo.toml`, `Cargo.lock` |
| Engine | Bevy 0.19 | root `Cargo.toml` |
| Packages | `moria-world`, `moria-demo`, `moria-bench`, `moria-curate` | root `Cargo.toml` |
| Headless/library Bevy features | Default features off; shared dependency enables `bevy_state`; `moria-world` adds asset/image/render/KTX2 support | root and `moria-world` manifests |
| Headed features | `moria-demo` and `moria-bench` enable Bevy `3d` | package manifests |
| Serialization/content | Serde, RON, JSON, SHA-256, zstd | workspace/package manifests |
| Asset validation | `basisu`, `ktx2`, and `naga` WGSL parsing | `moria-world/Cargo.toml` |
| Property tests | `proptest` | `moria-world/Cargo.toml` |
| Machine telemetry | `sysinfo` | workspace and bench manifests |
| Safety lint | `unsafe_code = "forbid"` workspace-wide | root `Cargo.toml` |

**Fact:** The four-crate workspace is a deliberate and still valid exception to
the XP preference for a single Bevy binary: this repository already needs a
reusable library boundary, two executables, and generation/validation tooling.
No package-manager layer other than Cargo is present.

**Fact:** Root `assets/` follows Bevy's standard lookup layout. `.cargo/config.toml`
sets `jobs = 1` because linking headed and headless Bevy test targets in
parallel exceeded the constrained verification runner's memory.

### Commands and quality gates

**Fact:** `AGENTS.md` defines these current repository-root commands:

```sh
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --all-targets
cargo run -p moria-demo
cargo run --release -p moria-bench -- --scenario flythrough --output target/bench/flythrough.json
cargo run --release -p moria-bench -- --scenario mutation-workloads --output target/bench/mutation-workloads.json
cargo run -p moria-curate -- generate
cargo run -p moria-curate -- check
cargo run --release -p moria-curate -- prove-forest --output target/feasibility/forest.json
cargo run --release -p moria-bench -- --scenario feasibility-mutation --resolution 2560x1440 --forest-proof target/feasibility/forest.json --output target/feasibility/mutation.json
```

Normal development uses the dev profile. The root manifest uses
`opt-level = 1` for project dev code, `opt-level = 3` for dependencies, and
native release `lto = "thin"` with `codegen-units = 1`.

**Fact:** `AGENTS.md` says CI runs format, check, clippy with warnings denied,
tests, and `moria-curate check`; headed GPU/platform acceptance is separate.
No `.github` workflow files are present in this checkout, so the automation
implementation cannot be independently confirmed from repository files.

**Fact:** The `prove-forest`, `--forest-proof`, named-M4 blocking gate, and
`moria-curate check` contract over the global manifest conflict with the new
authority. The first four Rust quality checks and ordinary build command remain
applicable. The future status of `moria-curate generate/check` depends on the
compact fixture architecture.

**Fact:** No Rust quality gate was run for this analysis because only a
run-scoped Markdown document was added. The current code baseline was inspected,
not modified.

## Existing conventions and data flow

### Package and module boundaries

**Fact:** `moria-world/src/lib.rs` is the public facade. It explicitly re-exports
consumer contracts while keeping `curation`, `mutation`, `query`, `storage`,
`streaming`, and lifecycle implementations private. Other packages import
through `moria_world::{...}`. Tests in `moria-curate`, `moria-demo`, and
`moria-world` enforce this consumer boundary.

**Fact:** Code is organized by feature responsibility:

```text
moria-world/src/
  generation/   identity, biome classification, deterministic terrain
  storage/      coordinates, voxels, bricks, sparse store/deltas
  query/        public reads, rays, capsules, diagnostics
  mutation/     public edit protocol and admission
  streaming/    focus messages, desired-band planner, tokened lifecycles
  objects/      shapes, overlap validation, immutable spatial index
  presentation/ asset declarations, loaders, validation
  telemetry/    runtime counters and serializable evidence
  terrain/      presentation ownership
  curation/     manifest model/generation/facade
  testing/      conformance support
```

This matches both repository guidance and XP feature-plugin organization.
There are no repository-wide `systems`, `components`, `resources`, `types`, or
`utils` buckets.

### Naming and API patterns

**Fact:** Repository guidance requires `snake_case` files/modules and systems,
`UpperCamelCase` public types/messages/plugins/sets, `Plugin` suffixes,
action/completed naming for public commands/results, and fixed-width public
coordinates/IDs. Public messages and system parameters follow this pattern:
`SetFocusSource`, `RemoveFocusSource`, `WorldEditCommand`, `EditAccepted`,
`WorldRead`, and `WorldEditWrite`.

**Fact:** The authoritative mutation API is `WorldEditWrite::submit`; read APIs
are exposed by `WorldRead`. `WorldStore` is crate-private. The demo and
benchmark may not query arbitrary world entities, construct authoritative
voxel components, or import private source modules.

### Authoritative state and current data flow

**Fact:** `WorldIdentity` combines a `u64` seed, a 32-byte parameters digest,
and `WorldBounds`. Terrain evaluation is fixed-point/integer and callable by
coordinate; call-order independence is tested.

**Fact:** The current coordinate implementation is not yet general: it
hard-codes the old 1 km × 1 km × 256 m Product One region in
`storage/coordinates.rs`. The same bounds are repeated in mutation admission
and object indexing. This is a key attachment point for a reviewed compact
bounded-region/cell contract.

**Fact:** `WorldStore` composes current truth as deterministic base plus sparse,
sorted per-brick deltas. It uses:

- a private `CuratedBaseTruth` for manifest features and placements;
- a `HashMap` of active brick records;
- a `BTreeMap` of brick deltas for stable persisted/current differences; and
- a monotonically increasing revision.

Mutation commits first collect changes into a `BTreeMap`, remove net no-ops,
and then update revisioned sparse state. This is compatible with the
authority's deterministic ordering and delta-persistence goals, but current
base installation still expects globally supplied manifest objects/features.

**Fact:** Current `WorldReadState` owns the store, material registry, water
bodies, traversal route, active bands, diagnostic statuses, render chunks, and
a diagnostic generation. `WorldRead` returns `NotReady` until
`WorldLifecycle` is ready and that private state exists. Query APIs enforce
bounded ray, capsule, sweep, hit, and diagnostic-page limits.

**Fact:** `WorldEditWrite::submit` currently performs only synchronous bounded
admission. Accepted commands are reserved in a fixed-capacity queue and emit
one `EditAccepted`; no scheduler currently consumes those reservations.

**Fact:** Streaming focus input is message-based and indexed by stable source
ID. The planner coalesces desired bricks with `BTreeMap`, applies band
hysteresis, and sorts work by priority/distance/brick. Tokened chunk and
Horizon lifecycles reject stale results. These are generic extension points,
although `Horizon` currently contains tree-specific partition assumptions.

### Scheduling and state

**Fact:** Repository rules place deterministic simulation, collision,
cooldowns, and edit commits in `FixedUpdate`; raw input, cameras, UI, task
polling, and render installation stay in frame schedules. Raw input edges must
be converted to durable intent first.

**Fact:** The current plugin has no `FixedUpdate` systems. It only applies
focus messages in `Update` and increments telemetry in `PostUpdate`. Therefore
the requested scope correction does not need to migrate existing player
simulation; future substrate mutation and collision wiring must attach to the
documented fixed/frame split.

**Fact:** `WorldLifecycle` is a singleton `Resource`, intentionally not a Bevy
`State`, so consumers can integrate it with their own state machine. Per-object
or per-view values are modeled as components when they exist; singleton
configuration, queues, assets, and world services are resources. Repository
guidance prohibits using one type as both.

### Tests

**Fact:** A source scan finds 224 `#[test]`/`#[proptest]` annotations across the
four crates (199 world, 20 bench, 4 curate, 1 demo), matching the README's
reported total. Patterns include:

- pure deterministic generation/storage/index tests;
- property tests using `proptest`;
- public-facade integration tests in each consumer crate;
- headless Bevy `App` tests with `MinimalPlugins`;
- controlled `FixedUpdate` advancement via
  `moria_world::testing::run_fixed_ticks`;
- message/resource/world-state assertions without sleeps or windows;
- strict asset/config placeholder tests;
- report-schema, failure-state, and atomic-output tests; and
- a public-query-only benchmark probe.

Rendering is left to headed/manual evidence. This matches the repository and XP
guidance.

### Assets and user-facing presentation

**Fact:** The asset declaration table fixes 30 Product One IDs and paths. It
contains species-specific birch/pine LOD assets, forest horizon cards, grass
and bush assets, and `player/explorer.glb` with dedicated skeleton/animation
tests. `presentation.ron` contains player/camera/light tuning and forest
timings; `input.ron` contains player movement/orbit bindings.

**Fact:** These declarations are contracts, not active player UI: the demo has
no current menus, HUD, controller, or camera implementation. The valid retained
UI concept is a thin diagnostic viewer/free camera. Existing raw-voxel,
brick-boundary, streaming-band, dig/place, save/load, and inspection concepts
are substrate diagnostics when decoupled from player/game semantics.

## Relevant modules and extension points

### Compact generation and bounded fixtures

**Fact:** `generation/identity.rs` and `generation/terrain.rs` already provide
the generic seed/digest/bounds and deterministic coordinate-evaluation base.
`curation/model.rs` and `curation/generate.rs` instead model and generate one
global curated world with species, tree/canopy shapes, route tags, a ruin, and
an enumerated object vector.

**Recommendation:** Attach the compact generation contract at the
`generation`/`curation` boundary, preserving deterministic evaluators and
identity types while replacing assumptions that world opening must consume one
complete `CuratedManifest`. The reviewed design must decide which curation
types remain as small bounded fixture/anchor formats and which become
historical-only.

### Registered objects and exact broad phase

**Fact:** `objects/index.rs` builds dependency, sample, and Horizon indexes;
reports retained bytes; bounds candidate and affected counts; and exposes
sorted ID queries. `objects/validation.rs` and tests provide exact
shape-disjointness/oracle behavior. `objects/shapes.rs` has analytic object
shapes, but tree/canopy names are embedded in public types and tests.

**Recommendation:** Preserve the stable-ID, bounded-index, exact-oracle,
dependency-attribution, retained-memory, and overlap-safety capabilities.
Generalize population ownership and terminology so an index can be built or
queried for a bounded generated cell/fixture without requiring a shipped
forest population. Tree-shaped stress fixtures may remain optional data, not
product invariants.

### Sparse truth, query, mutation, and persistence

**Fact:** `storage/store.rs`, `query/read.rs`, `mutation/api.rs`, and
`streaming/*` are the natural runtime seams. They already enforce private truth,
public reads/writes, sparse deltas, fixed capacities, sorted results, focus
messages, and stale-result rejection. Persistence is described in the TDD but
is not implemented in the current source tree.

**Recommendation:** The corrected design should connect bounded generation to
`WorldStore`/`WorldReadState` installation and make exact affected-region and
affected-object reconciliation flow through the existing public APIs. Do not
add a second privileged viewer or benchmark data path.

### Evidence and benchmark tooling

**Fact:** `telemetry/reports.rs` currently exports
`ForestFeasibilityReport` and a mutation report bound to
`forest_report_sha256`, Metal, 2560×1440, and an M4 acceptance label.
`moria-bench/cli.rs` accepts `--forest-proof`, rejects non-curated seeds, and
offers forest-shaped feasibility scenarios. The final benchmark schema already
has useful generic machinery: complete nullable failure reports, sorted failure
reasons/maps, machine profiles, allocation accounting, atomic temp-write/
flush/rename, and nonzero exit on unproven evidence.

**Recommendation:** Replace the proof/report/CLI identity at these seams, while
retaining immutable evidence, explicit failure states, digest binding, complete
machine identity, workload reconciliation, and stage accounting. Cross-platform
F1 correctness should not reject an artifact merely because it was not created
on one named machine; machine-specific timings should remain labeled evidence.

### Diagnostic viewer

**Fact:** `moria-demo` has no gameplay implementation, so its correction point
is its future composition rather than a large deletion.

**Recommendation:** Keep `main.rs` thin and add any viewer behavior as
domain-focused plugins beside diagnostics/presentation ownership. A free
camera, streaming focus, voxel/material inspection, edits, and object
invalidation visualization can consume `WorldRead`, `WorldEditWrite`, and focus
messages without introducing player, locomotion, or animation types.

## Likely affected files and areas

The following is an attachment inventory, not an implementation file list.

| Area | Current conflict or preserved capability | Likely action in later design/implementation |
|---|---|---|
| `docs/product-scope-authority.md` | Approved authority | Keep authoritative |
| `README.md` | Boundary is corrected; current-evidence and commands still say forest | Generalize curation/proof/status wording |
| `AGENTS.md` | Hard-blocks work on forest F1/F2 and one named M4 | Replace with reviewed substrate gates while preserving cargo/style/schedule rules |
| `docs/design-document.md`, `docs/seeds/*` | Walkable world, ecology, player, curated route | Supersede or rewrite active contracts; retain history labels |
| `docs/tdd/*.md`, `docs/tdd-review.md` | Forest/player/M4 assumptions pervade data, API, systems, states, rendering, assets, config, benchmarks, and sequence | Correct and review before code |
| `docs/issues.json`, `docs/issue-review.md` | 227-item old manifest includes forest, player, asset, route, and named-machine work | Record dispositions; this planning flow must write only the run-scoped `issues.json`, never replace `docs/issues.json` |
| `test-specs/*.md` | At least 31 files contain obvious forest/player/M4 terms; some still describe generic recovery/query work | Classify each as keep/generalize/supersede; do not discard generic tests by filename alone |
| `assets/config/curated_manifest.ron` | Large global enumerated population | Remove from active product data; do not recreate PR #363's larger form |
| `assets/config/product_one_region.ron` | Fixed old region and ecological density/species/canopy parameters | Generalize to compact generator/schema/fixture parameters |
| `assets/config/presentation.ron`, `assets/config/input.ron` | Player/camera/light/forest and gameplay bindings | Reduce to viewer/diagnostic concerns or supersede |
| `assets/player/`, species-specific vegetation assets and matching tests | Character/animation and ecological product contracts | Remove player contract; retain tree-like assets only if explicitly reclassified as optional bounded fixtures |
| `crates/moria-world/src/config*.rs` | Public `PlayerConfig`, forest biome/object and forest timing fields | Generalize schemas/validation and public exports |
| `generation/biome.rs` | Public `BiomeId::Forest`/`Meadow` | Remove ecology as a product requirement or make it fixture-local if retained |
| `curation/*` | Global manifest, species, route, complete population | Primary compact-generation/bounded-fixture correction point |
| `storage/coordinates.rs`, `mutation/admission.rs`, `objects/index.rs` | Old region bounds duplicated as constants | Consolidate only after the reviewed bounds/cell contract is explicit |
| `storage/store.rs`, `query/read.rs` | Generic sparse truth, but installs global curated features/objects/route | Adapt world-opening/base lookup to bounded generation |
| `objects/*`, `terrain/ownership.rs` | Valuable IDs/index/oracle/ownership; tree/Horizon terminology | Preserve algorithms and exactness, generalize presentation categories |
| `presentation/asset_ids.rs`, validation, Cargo test entries | Fixed 30-asset inventory including explorer and species assets | Rebuild active inventory around substrate/viewer fixtures |
| `telemetry/reports.rs` and tests | Strong evidence types bound to forest/M4 | Generalize proof schema and machine semantics |
| `moria-curate` | Functional global generator/check/prove-forest CLI | Replace active proof path with `prove-substrate`; retain small-fixture generation only if design needs it |
| `moria-bench/cli.rs`, capture schema/output, scenarios/tests | `--forest-proof`, curated seed, route coverage, M4/3060 acceptance assumptions | Bind to substrate proof/workload parameters; preserve truthful atomic reporting |
| `moria-demo` | Empty headed shell | Compose thin diagnostic viewer through the public facade |

## Recovery PR attachment analysis

### PR #363

**Fact:** The local PR head contains 12 commits, changes 15 files, and expands
`assets/config/curated_manifest.ron` by hundreds of thousands of lines. It adds
or modifies forest generation, curation reports, a stress-target selector,
object-index logic, and tests.

**Fact:** The global generated population and forest density/species/canopy/
route contracts are explicitly rejected by the new authority.

**Recommendation:** Discard the generated manifest and forest contract
enforcement. Review separately for salvage:

- deterministic report construction and canonical output;
- independent stress-target/oracle logic, if generalized to parameterized
  bounded fixtures;
- exact edit-candidate/index checks;
- retained-byte and dependency-allocation accounting; and
- deterministic failure detail.

Port approved hunks onto clean current `master`; do not merge or rebase the PR
wholesale.

### PR #365

**Fact:** The local PR head contains 22 commits and changes six telemetry/
benchmark files, adding extensive report validators and tests. It strengthens
atomic output, explicit partial/failed evidence, trusted identity/digest
binding, workload and stage reconciliation, renderer barrier accounting, and
cross-field validation. It also hard-codes forest counts/species/canopy, F1
forest linkage, M4/Metal identity, Product One route tags, and named M4/3060
acceptance behavior.

**Recommendation:** This PR is the strongest source of generic evidence
protections, but it must be adapted rather than copied as a schema. Preserve
truthful pass/failure serialization, immutable digest-bound identity,
stage/workload reconciliation, machine identification, and atomic output.
Remove forest/third-person/named-machine correctness assumptions and bind the
result to the reviewed `prove-substrate` contract.

**Unknown:** The repository does not contain a completed recovery-epic #325
disposition ledger. Closing that epic requires a separate per-item
merged/adapted/superseded record.

## Applicable XP stack guidance

The domain guidance mostly reinforces established repository conventions:

- **Bevy scaffold:** keep the existing workspace because there are concrete
  library, tool, demo/viewer, and benchmark deliverables. Keep root `assets/`
  and the committed lockfile.
- **Plugin organization and Rust modules:** keep `main.rs`/`lib.rs` thin and
  attach behavior to cohesive feature modules/plugins. Do not create new
  top-level item-kind buckets.
- **Cargo profiles:** current dev/dependency/release settings already match the
  guidance. Do not use release as the normal edit loop.
- **Schedules:** deterministic mutation/collision rules belong in
  `FixedUpdate`; free camera, diagnostics, UI, task polling, and render
  installation belong in frame schedules. Explicit ordering/sets are required
  when a result feeds another stage.
- **Fixed/headless tests:** continue `MinimalPlugins`, controlled fixed ticks,
  pure/property tests, public-facade integration tests, and no sleep/window for
  logic. Use headed human review for rendering.
- **Many-entity rendering:** optional tree-shaped/object stress fixtures should
  share mesh/material handles and use culling/LOD/visibility. Custom GPU paths
  need representative diagnostic evidence first. Do not let render
  representation become authoritative truth.
- **Resources vs components:** preserve singleton resources for world services,
  queues, config, and shared assets; use components for per-view/per-object
  values. Do not derive the same type as both.
- **Action input:** the old gamepad/player action layer is no longer required.
  If the diagnostic viewer supports multiple physical inputs, semantic
  diagnostic actions should still be separated from raw buttons, and frame
  edges must not be consumed directly by fixed-step edits.
- **Collision:** existing bounded voxel queries are the established source of
  truth. Generic collision detection should remain separate from consumer
  response; no player-response layer belongs in the substrate.

Repository rules are more specific than XP guidance for public facades,
fixed-width coordinates, deterministic ordering, `WorldRead`/`WorldEditWrite`,
32-bit GPU fields, and derived-presentation ownership; those rules should
continue to win unless the reviewed correction explicitly changes them.

## Risks, unknowns, and questions before implementation

### Contract questions

1. **World extent:** Is the substrate still a finite configurable
   `WorldBounds`, or must coordinates support an effectively unbounded
   cell-addressed world? Current public coordinate conversion and several
   private modules hard-code one finite region.
2. **Generation unit:** What exact bounded request unit is authoritative
   (brick, generation cell, region, or more than one), and what overlap/halo is
   required so independently generated neighboring units agree?
3. **Stable object identity:** How is an `ObjectId` derived from seed,
   generator/schema version, cell coordinate, and local candidate so identity
   is independent of activation order and duplicate-free across cells?
4. **Compact inputs:** Which values enter the parameters digest, how are
   generator/schema versions serialized, and how are authored bounded anchors
   distinguished from reproducible stress-output parameters?
5. **Fixture scope:** Which current ruin/tree/prop assets and shape types remain
   as small bounded fixtures, and which are removed from active product
   contracts? “Permitted” does not mean “required.”
6. **Object indexing lifetime:** Is the immutable index per generated cell,
   per active neighborhood, or another bounded unit? The current implementation
   owns one index over the complete manifest.
7. **Save compatibility:** May the unimplemented TDD save format be replaced
   freely, and should any already-generated delta artifacts be considered
   historical? The current source has sparse deltas but no public save/load
   implementation.

### Acceptance questions

8. **`prove-substrate` schema:** Exact report fields, workload parameters,
   oracle construction, retained-memory boundary, digest inputs, and failure
   immutability rules still need reviewed definition.
9. **F1 portability:** Which operating systems/architectures form the required
   cross-platform correctness matrix, and which measurements are informative
   rather than blocking on each machine?
10. **F2 review:** The authority names visual subjects but not exact scenes,
    capture metadata, resolutions, or pass-record format. These must be fixed
    without reintroducing a curated traversal/game experience.
11. **Performance policy:** Existing numeric M4/Metal thresholds are
    superseded as universal correctness gates. The corrected TDD must state
    which performance/resource limits remain product targets, how machine
    identity is recorded, and whether any hardware-specific acceptance job is
    still required.
12. **Stress populations:** Population counts are workload parameters, but
    their ranges, deterministic generation method, storage location (normally
    `target/`), and proof matrix are unspecified.

### Migration and planning risks

13. **Active versus historical documents:** The repository needs an explicit
    list of which design/TDD/review/issue/test-spec artifacts are rewritten and
    which are frozen with a superseded marker. Leaving both unlabeled would
    preserve contradictory authority.
14. **Issue DAG contamination:** `docs/issues.json` has 227 old-flow entries,
    including forest, route, player, animation, and named-machine work.
    Dependency edges into generic query/index/storage work mean bulk deletion
    would lose valid capabilities. The run-scoped issue manifest must be built
    from per-item dispositions.
15. **Recovery branch age:** PR #365 has an older merge base than current
    `master`, while PR #363 is based on the prior master commit. Porting must
    preserve current fixes and tests rather than treating either head as a new
    baseline.
16. **Public API churn:** `SpeciesId`, `BiomeId::Forest`, tree/canopy object
    shapes, route types, `ForestFeasibilityReport`, and several Product One
    config types are public exports. No compatibility policy is stated for
    removing or renaming these pre-release APIs.
17. **Integration gap:** Generic primitives are tested in isolation, but the
    world plugin does not yet connect generation, reads, streaming, edit
    execution, meshing, persistence, or proof capture. Passing existing unit
    tests must not be mistaken for an integrated substrate proof.
18. **Manifest removal sequencing:** `moria-curate check`, asset declarations,
    Cargo test targets, README commands, and tests currently require the global
    manifest. Removing the file before the reviewed replacement contracts and
    their dependent edits land would break the ordinary gate.

## Overall attachment conclusion

**Fact:** This is not a greenfield redesign. The durable center is the
`moria-world` public facade plus deterministic generation primitives, sparse
truth/deltas, bounded queries, exact object indexing/oracles, tokened
streaming/reconciliation, and truthful evidence machinery.

**Recommendation:** Correct the authoritative documents and issue/test DAG
first. Then attach compact bounded generation behind the existing
`WorldIdentity`/`WorldRead`/`WorldEditWrite` boundary; generalize the
curation/object/evidence layers; and compose the headed executable as a thin
diagnostic viewer. The global forest manifest, player/animation asset contract,
curated route, and named-machine correctness gate should not be allowed to
remain prerequisites for the preserved substrate capabilities.
