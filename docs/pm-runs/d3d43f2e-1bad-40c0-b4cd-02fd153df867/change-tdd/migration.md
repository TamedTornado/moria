# Repository, data, API, and recovery migration

## Migration principle

The scope correction is a coordinated replacement, not deletion by keyword.
Each artifact/change is classified as **keep**, **generalize**, **remove**, or
**historical/superseded**. Generic capability is retained even when the current
fixture/name is contaminated. Active references must switch atomically enough
that the ordinary gate never points at a deleted global manifest.

[Design R15-R18, AC1-AC2, AC15-AC17]

## Active artifact disposition

| Area | Disposition | Required result |
|---|---|---|
| `docs/product-scope-authority.md` | Keep | Remains top-level authority. |
| This run's reviewed design/TDD/issues/test specs | Keep | Become the correction plan and trace source. |
| `README.md`, `AGENTS.md` | Generalize | Substrate vocabulary, corrected commands, Linux/macOS F1, headed F2; remove forest/M4 blocking text. |
| `docs/design-document.md`, `docs/seeds/*`, existing `docs/tdd/*`, their reviews | Historical/superseded or rewritten | A prominent first-screen marker names the authority and corrected active replacement. If rewritten, Git history retains old content. |
| `docs/issues.json`, `docs/issue-review.md`, old `test-specs/*` | Historical/superseded per item | Do not replace `docs/issues.json` in this run; run-scoped issue planning records keep/generalize/remove edges. |
| `assets/config/curated_manifest.ron` | Remove | No active or regenerated complete population. |
| `assets/config/product_one_region.ron` | Remove after replacement lands | `substrate_world.ron` is canonical compact input. |
| `assets/config/input.ron`, `presentation.ron` | Generalize | Diagnostic action/viewer settings only; no player/ecology fields. |
| `assets/player/explorer.glb` and animation/skeleton declarations/tests | Remove | No player/character asset contract. |
| Species vegetation/horizon asset contracts | Remove by default | Retain only assets explicitly reclassified as neutral repeated fixture visuals; rename declarations and remove ecological acceptance. |
| Terrain/material/raw-voxel shaders and KTX2 loader | Keep/generalize | Substrate presentation, validation, and shared handles remain. |
| `assets/stamps/ruin_p1.ron` | Generalize or remove | A small neutral `diagnostic_arch.ron` may reuse geometry after renaming/review; no ruin/route special ID. |
| `moria-world` generation/storage/query/mutation/streaming/objects | Generalize | Apply contracts in this TDD; preserve generic exactness and facade. |
| `moria-curate` manifest generator | Replace | `check`, `prove-substrate`, `compare-proofs`; no global generation. |
| `moria-bench` report/output machinery | Generalize | Preserve truthful status, validation, atomic output; replace scenarios/identity. |
| `moria-demo` | Generalize | Empty shell becomes thin diagnostic viewer; no game layer. |

Repository searches for `forest`, `species`, `canopy`, `player`, `explorer`,
`route`, `prove-forest`, `forest-proof`, `M4`, `Metal`, and Product One are
review aids, not automatic deletion rules. Occurrences may remain in clearly
marked history, migration records, rejected CLI tests, or authority rationale.
No active command, dependency, pass condition, public generic type, or product
asset requirement may retain them.

## Public API migration map

| Removed/current contract | Replacement |
|---|---|
| `RegionConfig`, fixed Product One bounds | `WorldDefinition`, validated configurable `WorldBounds` |
| `CuratedManifest` complete population | Pure bounded `GeneratedCell` plus explicit small `BoundedFixture` |
| world-local `ObjectId(u64)` | `ObjectId { world_digest, local }`, with collision-free packed local identity |
| `SpeciesId` | None |
| `ObjectKind::{TreeA,TreeB,Bush,Boulder,Stump,Rock,Ruin}` | Neutral `RegisteredObjectKind` and fixture presentation mapping |
| `VoxelObjectShape::{Tree,Bush,Boulder,Stump,Rock,SparseStamp}` | Generic `RegisteredObjectShape::{Ellipsoid,Column,SparseStamp}` |
| `RuinPoi`, special ID zero | Ordinary authored anchor/object; zero remains invalid |
| `CuratedRoute`, `RouteTag`, `TraversalRoute`, `WorldRead::route` | None; proof/viewer use parameterized request/camera paths |
| `BiomeId::{Forest,Meadow}` | Generic procedural/material classification without ecology names |
| `FocusPurpose::Traversal` | `Camera`, `Inspection`, `Mutation`, or `Proof` |
| `ForestFeasibilityReport` | `SubstrateProofReport` |
| forest/Horizon report fields | Per-cell object/index/accounting and generic derived-item fields |
| `PlayerConfig`, player actions | Viewer-local free-camera and `DiagnosticAction` |
| `moria-curate generate` | Removed |
| `moria-curate prove-forest` | `moria-curate prove-substrate` |
| `--forest-proof`, curated seed restriction | Generic `--proof` and definition/workload identity |
| flythrough/feasibility-mutation route scenarios | Diagnostic streaming, mutation reconciliation, generated object stress |

No deprecated aliases are required at version `0.1.0`. Compile failures in
consumer tests are intentional migration signals. Generic query/edit types keep
their names and behavior to limit unnecessary churn.

## Save and data compatibility

The global manifest and recovery PR #363 generated population are discarded;
they are not converted into anchors or a save.

There is no accepted implemented public save format in the baseline, so
`moria-substrate-delta` version 1 is the first supported format. Historical
prototype files fail as `UnsupportedFormat`. A save is accepted only for an
exact `WorldIdentity`; there is no best-effort generator migration. Future
schema/generator migrations require a new reviewed format version and explicit
converter or rejection policy.

The compact definition, smoke fixture, and stamp are small checked-in inputs.
Stress populations, generated cell dumps, proof reports, benchmark reports,
and F2 review records remain under `target/` and are not committed as product
truth.

## Dependency-ordered landing plan

### Phase 0: authority and inventory

- Add superseded markers or corrected active links before old documents can be
  mistaken as authority.
- Produce the run-scoped issue/test-spec DAG and per-artifact disposition.
- Update contributor commands only when replacement `moria-curate check` is
  ready in the same landing series.

Exit: no implementation issue depends on forest/player/M4 acceptance, and every
old issue/test spec has a disposition.

### Phase 1: compact types beside old data

- Add definition/identity/cell/object-ID types and tests.
- Add `substrate_world.ron`, bounded smoke fixture, and neutral stamp.
- Generalize coordinate conversion to opened bounds.
- Keep old manifest command temporarily only until new `check` passes in CI;
  do not add or regenerate population data.

Exit: canonical compact inputs validate; pure generation/ID/index oracle tests
pass; current gate is not broken.

### Phase 2: runtime and persistence

- Adapt `WorldStore`/`WorldReadState` to generated cells plus deltas.
- Integrate open, activation, edit execution, exact attribution,
  reconciliation, save/load, and telemetry behind the facade.
- Adapt external consumer tests.

Exit: headless public-facade integration and save/reload tests pass without the
global manifest.

### Phase 3: cutover

- Replace `moria-curate` commands/reports and `moria-bench` schemas/CLI.
- Remove active manifest/Product One config/player and contaminated asset test
  targets.
- Update README, AGENTS, active TDD/design, commands, and CI gate.
- Remove obsolete public exports in one API migration commit or reviewed
  series.

Exit: ordinary gate passes from a clean checkout; active artifact search has no
unclassified conflicting requirement.

### Phase 4: F1 and headed work

- Produce Linux/macOS F1 artifacts and compare correctness digests.
- Only after F1 passes, finish viewer/headed presentation breadth and benchmark
  scenarios.
- Run stress/headed measurement evidence and F2 human review.

Exit: Design AC1-AC14 pass with immutable evidence.

### Phase 5: recovery and closure

- Port reviewed generic recovery hunks onto current master.
- Record every recovery item as merged, adapted, or superseded.
- Close epic #325 only after ledger review and remote-master clean gate/F2
  evidence.

Exit: Design AC15-AC17 pass.

## Recovery PR #363

Do not merge or rebase wholesale. The disposition ledger works at commit/hunk
or cohesive behavior level and records source commit, target issue/commit,
classification, rationale, and tests.

- **Discard:** generated global manifest and ecology/forest count, species,
  canopy, spacing, route, and density enforcement.
- **Review for adaptation:** canonical deterministic report construction,
  independent bounded stress-target/oracle logic, exact edit candidates,
  retained-byte/dependency-allocation accounting, and stable failure detail.
- **Adaptation rule:** rewritten code uses compact world/workload identity,
  per-cell inputs, generic object terminology, and current facade/tests. Copying
  a forest-shaped schema and renaming fields is insufficient.

## Recovery PR #365

Do not merge or rebase wholesale.

- **Preserve/adapt:** immutable evidence identity, explicit not-started/partial/
  failed/pass states, digest binding, workload/stage reconciliation, atomic
  output, stable failure reasons, and nonzero exit on unproven evidence.
- **Remove:** forest counts/species/canopy/route, third-person stages,
  M4/Metal/1440p as functional identity, universal named-machine thresholds,
  and forest-proof linkage.
- **Validation rule:** machine identity is mandatory evidence but cannot make
  otherwise identical functional proof fail because it is not one named host.

Every adapted change receives current focused tests plus the ordinary gate.
Recovery artifacts never override newer master fixes.

## Rollback and failure handling

Before cutover, phases 1-2 are additive and can be reverted by normal commits
while the old gate remains intact. After cutover, rollback is a normal revert
to the previous complete contract; do not restore only the global manifest or
old command without its matching code/tests/docs.

F1/F2 failures preserve reports at their original paths and stop dependent
breadth. The reviewed TDD revision must identify the failing stage and
redesign. It may change internals behind public deterministic contracts, but
may not:

- expand the context ring without revising bounded-work/accounting contracts;
- truncate exact query/affected sets;
- omit reconciliation or save/reload stages;
- relabel missing/partial evidence as pass;
- reduce the recorded canonical stress workload merely to pass; or
- restore forest/player/named-machine product requirements.

## Completion audit

Before declaring migration complete:

1. run the ordinary command gate in `overview.md`;
2. confirm `assets/config/curated_manifest.ron` and player assets/contracts are
   absent from active product data;
3. confirm `moria-curate check` creates no population artifact;
4. inspect active term-search results and classify every remaining occurrence;
5. verify external consumer crates use only the corrected facade;
6. validate Linux/macOS F1 reports and their matching correctness digest;
7. validate benchmark non-pass/pass integrity and F2 record;
8. verify remote master from a clean checkout; and
9. review the complete #363/#365/#325 disposition ledger.
