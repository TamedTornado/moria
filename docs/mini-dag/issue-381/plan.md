# Issue 381 implementation plan

Status: ready for implementation.

This tranche removes the current forest product contract and repairs the first
review frontier. It does not design `prove-substrate`, a compact replacement
generator, a new whole-product TDD, player/controller work, character or
animation assets, or any change to PR #365.

## Observed baseline

- `assets/config/curated_manifest.ron` is a 6.1 MB globally enumerated
  population.
- `moria-curate` exists only to generate/check that population and emit
  `prove-forest`; its feature-gated facade builds the same forest.
- `moria-bench` requires `--forest-proof` for `feasibility-mutation`, and
  `MutationFeasibilityReport` stores `forest_report_sha256`.
- GitHub issue #66 is the open `M-044` assisted gate. Its direct dependents in
  `docs/issues.json` are `M-071`, `M-077`, `M-081`, `M-180`, and `V-11`;
  `M-081` then blocks multiple generic substrate issues.
- PR #363 is already closed without merge and has an operator disposition
  comment. PR #365 is open at head
  `1985b9578d960ea5f27300050ff23fb07f0b3418` and must remain unchanged.
- The unmodified tree passed `cargo test` on 2026-07-23. In particular, all 17
  `object_index_contract` tests, all 7
  `object_shapes_and_ownership_contract` tests, query-probe tests, and atomic
  benchmark evidence-output tests passed. Treat these as preservation
  characterizations.

## Boundary decisions

1. Delete the forest-only curation executable and feature rather than changing
   `generate`/`check` into an undesigned replacement pipeline. Remove
   `crates/moria-curate` from the workspace, remove the `curation` Cargo
   feature/facade/generator, and retain `curation/model.rs` as the existing
   generic generated-metadata and registered-object model.
2. Delete the checked-in manifest and its runtime asset identity. Do not
   replace it with a different population or persistence format in this issue.
3. Remove ecology requirements from configuration, but retain the generic
   index dimensions/caps, edit-candidate and exact-affected-object caps,
   dependency-brick cap, and retained-byte cap with their current values.
4. Retain `SpeciesId`, `ObjectKind::{TreeA, TreeB}`, and
   `VoxelObjectShape::Tree { canopy_radii_q8 }` as optional object/shape
   metadata. Their analytic sampling, bounds, ownership, and overlap tests are
   substrate-generic; this issue removes canopy/species *acceptance
   requirements*, not the ability to use a tree-shaped bounded fixture.
5. Remove tree-only Horizon membership from the immutable object index
   (`HorizonCellKey`, `horizon_tree_ids`, and its tree-member cap/evidence
   field). Preserve the generic 32 m dependency table, 4 m sample table,
   bounded/exact edit attribution, and generic streaming reconciliation
   lifecycle. Do not replace tree aggregation with a newly designed object
   aggregation API here.
6. Remove forest species/understory/card assets from the required runtime
   inventory and their per-file contract tests. Keep generic prop assets,
   shared-handle behavior, and the portable instanced vegetation shader where
   it remains useful to generic repeated-object presentation.
7. Keep `feasibility-mutation` as a standalone scenario for now, but remove
   its `--forest-proof` argument and `forest_report_sha256` field. This does
   not make it a replacement product gate.

## Red/Green TDD path

### Red: add removal contracts first

Add `crates/moria-world/tests/forest_surface_removed.rs` with repository-level
contract tests:

- `checked_in_forest_population_and_commands_are_absent` asserts that
  `assets/config/curated_manifest.ron` and the `crates/moria-curate` workspace
  member do not exist, and that active contributor/CLI sources contain neither
  `prove-forest` nor `--forest-proof`.
- `active_tracker_supersedes_the_forest_gate` parses `docs/issues.json`, asserts
  that `M-044` has the `superseded` label and a #380/#381 disposition, and
  asserts that no non-superseded node depends on `M-044` or `M-081`.
- `active_runtime_inventory_has_no_forest_contract_assets` asserts the runtime
  asset paths exclude the curated manifest, birch/pine/bush/grass population
  assets, and tree Horizon cards while retained prop paths remain declared.

These tests fail on the current tree because the manifest/crate/commands and
asset declarations exist, #66 is active, and the dependency edges remain.

Extend existing focused tests before implementation:

- In `crates/moria-world/tests/config_contract.rs`, assert that serialized
  defaults and the checked-in RON omit the retired biome/species/density/
  canopy/forest-timing fields, and that RON containing a retired field is
  rejected by `deny_unknown_fields`. This fails while the fields are valid.
- In `crates/moria-bench/src/cli.rs`, replace
  `requires_a_forest_proof_only_for_feasibility_mutation` with tests proving
  `feasibility-mutation` parses without a proof and rejects `--forest-proof` as
  unknown. Both expectations fail before the CLI change.
- In `crates/moria-world/tests/asset_declarations.rs`, assert the reduced
  inventory exactly and move the shared-handle regression in
  `asset_validation.rs` from `BirchNear` to retained `Boulder`/`Rock` handles.

Do not weaken the already-green generic characterizations. Run these before
the removal and record their green baseline:

```sh
cargo test -p moria-world --test object_index_contract
cargo test -p moria-world --test object_shapes_and_ownership_contract
cargo test -p moria-world --test curation_model_contract
cargo test -p moria-bench capture::output
cargo test -p moria-bench --test query_probe
```

### Green: minimum removal

1. Remove the forest data/tool boundary:

   - Delete `assets/config/curated_manifest.ron` and
     `crates/moria-world/tests/assets/config_curated_manifest_ron_placeholder.rs`.
   - Remove that explicit test target from `crates/moria-world/Cargo.toml`.
   - Delete `crates/moria-curate/`, remove it from root `Cargo.toml`, and
     regenerate `Cargo.lock`.
   - Delete `crates/moria-world/src/curation/{facade.rs,generate.rs}` and
     `crates/moria-world/tests/curation_facade.rs`; remove the feature and
     feature-gated exports from `crates/moria-world/{Cargo.toml,src/lib.rs}` and
     `curation/mod.rs`.
   - Keep `curation/model.rs`, `curation_model_contract.rs`, stable
     `ObjectId`, canonical ordering, stamp validation, and generic manifest
     validation.

2. Remove forest-only generation/config enforcement:

   - Delete the unused `generation/biome.rs` public forest classifier and its
     exports from `generation/mod.rs`/`lib.rs`; it currently serves only the
     forest curation surface and does not affect scalar terrain generation.
   - Remove the `BiomeConfig` block and `RegionConfig::biome`.
   - Reduce `ObjectGenConfig` to the existing generic spatial-index,
     edit-attribution, dependency-brick, and retained-memory limits. Remove
     global per-hectare counts, route clearance, birch/pine/bush/canopy ranges,
     and their defaults/validators.
   - Remove grass density/understory and tree-Horizon cap fields from
     `RenderingConfig`, and remove
     `forest_object_validation_max_ms` /
     `forest_object_index_build_max_ms` from `BenchmarkConfig`.
   - Make the matching deletions in
     `assets/config/{product_one_region.ron,presentation.ron}`,
     `config_validation.rs`, `config_contract.rs`, and the two config asset
     fixtures. Preserve fixed index cell sizes and all generic caps unchanged.

3. Remove forest proof/report coupling while preserving evidence integrity:

   - Remove `ForestFeasibilityReport`, `WorstEditTargetEvidence`,
     `FOREST_SCHEMA`, forest validators, and exports from
     `telemetry/{reports.rs,mod.rs}`; delete/replace the forest-only
     `tests/telemetry_reports.rs`.
   - Remove `forest_report_sha256` and only its validator from
     `MutationFeasibilityReport`. Keep build/world/manifest digests,
     `MachineProfile`, `passed == failure_reasons.is_empty()`, sorted reasons,
     finite metrics, workload reconciliation, and canonical JSON behavior.
   - Remove `max_horizon_tree_members_per_cell` from `ObjectIndexEvidence` and
     update the benchmark schema/output fixture. Keep every generic index
     accounting field.
   - In `moria-bench/src/cli.rs`, remove `forest_proof` from `BenchmarkArgs`
     and remove its parsing/scenario coupling. Rename “inactive forest” query
     comments/test vocabulary in `scenarios/query_probe.rs`,
     `tests/query_probe.rs`, and active docs to “inactive region”; do not change
     the workload counts or limits.

4. Remove forest-only runtime asset contracts:

   - Remove `AssetId` variants/declarations for `CuratedManifest`, all
     birch/pine LODs, bush LODs, `GrassCluster`, and `TreeHorizonCards`; update
     `ASSET_COUNT`, validation inventory counts, declaration tests, and
     `WorldRenderAssets` tests.
   - Delete the corresponding explicit test targets/files in
     `crates/moria-world/Cargo.toml` and `tests/assets/`.
   - Remove the matching files under `assets/vegetation/`. Keep
     `assets/shaders/vegetation.wgsl` and its generic 32-bit instancing
     contract.
   - Remove the tree-Horizon-card load assertion from the terrain-normal asset
     test. Preserve generic shared handles by asserting repeated retained prop
     instances clone identical handles.

5. Repair active documentation and the local DAG:

   - Update `README.md` and `AGENTS.md`: remove curation/gate commands and
     claims; retain the five requested Rust quality commands and ordinary
     benchmark commands.
   - In `docs/tdd/{overview,api,config,data-model,systems,rendering,assets,benchmarks,implementation-plan}.md`,
     delete ecology acceptance rules and forest-proof coupling. Preserve
     generic object/index/overlap/ownership/accounting/evidence text. Mark the
     old F1/F2 forest sequencing superseded by #380/#381 and explicitly defer
     a replacement acceptance design.
   - Add a prominent #380/#381 supersession note to active
     `docs/design-document.md` and `docs/engineering-evidence.md`, and remove
     current-evidence claims about the checked-in forest/proof. Historical
     `docs/interview-record.md`, `docs/pm-runs/**`, seed records,
     `docs/issue-review.md`, and `docs/tdd-review.md` remain history; add only a
     status banner where needed to prevent them being read as current
     authority.
   - Mark forest-only test specs (`issue-60`, `124`, `139`, `143`, `219`,
     `228`, `326`, `330`, `334`, `339`, `344`, and `347`) superseded in their
     forest portions. Update generic specs `issue-49`, `52`, `69`, `81`, `341`,
     `343`, and `352` to remove forest vocabulary/coupling while retaining
     their substrate properties.
   - In `docs/issues.json`, label and annotate obsolete forest-only nodes,
     including `M-017`, `M-042`, `M-043`, `M-044`, `M-081`, `M-099`, `M-100`,
     `M-129`, and `M-161`, as superseded by #380/#381. Mixed nodes must retain
     their generic clauses but mark forest clauses superseded/needs-replan; do
     not invent replacement deliverables.
   - Remove `M-044` from `M-071`, `M-077`, `M-180`, and `V-11`. Supersede
     `M-081` and remove it as a prerequisite from generic substrate nodes
     `M-082`, `M-083`, `M-084`, `M-085`, `M-091`, `M-101`, `M-107`, `M-180`,
     and `V-26`. Do not silently make forest-only visualization nodes runnable;
     mark them superseded instead. Keep `depends_on` arrays and textual
     “Depends on”/“Inputs” sections synchronized.

6. Record recovery disposition without porting PR #363:

   - Add `docs/recovery/pr-363-disposition.md`, pinned to PR #363 head
     `956063dc55da63e8b3e57571925793a3ab2554e4`.
   - Inventory every logical hunk in the PR’s changed files as
     `later-adaptation`, `forest-specific/rejected`, `generated/discarded`, or
     `already-present/no-port`.
   - Later-adaptation candidates must include: broad-candidate versus exact-hit
     accounting and cap regressions; inclusion of every registered object kind
     in dependency queries; independent changed-mask dependency attribution;
     stable first overlap witnesses; complete retained-byte/allocation
     accounting; atomic non-overwriting failure artifacts; truthful
     pass/failure validation; and measured build/machine/digest identity.
   - Record the review defects alongside those candidates: the PR’s stress
     search used incomplete/AABB substitutes at several revisions, fabricated
     or incomplete machine/failure evidence, weakened the two-function facade,
     and mixed unrelated changes. Therefore none is copied in this issue unless
     an existing generic preservation test breaks after removal.
   - Classify the expanded manifest, area/density/species/canopy/spacing/
     understory/forest-route logic, and forest report fields as rejected or
     discarded.

### Refactor and verification

- Remove dead imports, dependencies, feature flags, test targets, constants,
  and orphan asset declarations. Use generic names only for retained generic
  behavior; do not introduce new cell sizes, formats, generators, or proof
  schemas.
- Run a negative scan over active code/data/docs. `prove-forest`,
  `--forest-proof`, `ForestFeasibilityReport`, `forest_report_sha256`, retired
  config field names, and the curated-manifest asset path must be absent.
  Historical records may mention them only behind an explicit superseded
  status.
- Re-run the generic characterization tests above, then:

```sh
cargo fmt --all -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --all-targets
```

No headed or release acceptance run is required: this issue removes a false
gate and does not create its replacement.

## Tracker and recovery operations

These external state changes cannot be made red with a deterministic unit test;
verify them with read-only `gh ... --json` checks immediately before and after:

1. Confirm PR #363 is `CLOSED`, `mergedAt` is null, and post a link to
   `docs/recovery/pr-363-disposition.md` if the full hunk inventory is not
   already linked.
2. Comment on and close issue #344 as superseded only after its intended
   recovery hunks are represented in that disposition.
3. Comment on issue #66 that its forest gate is superseded by #380/#381, then
   close it. Update the live dependency text for #73, #103, #111, #229, and
   #70 consistently with the local DAG; #111 itself is superseded rather than
   treated as a replacement gate.
4. Re-read PR #365 and assert it is still open with the same head OID, body,
   comments, labels, and review state captured before implementation. Do not
   comment on, edit, close, rebase, or fetch/write through that PR.

## Acceptance criteria

- `assets/config/curated_manifest.ron`, the `moria-curate` forest pipeline,
  `prove-forest`, and `--forest-proof` are absent from active product paths.
- Active config, validators, reports, tests, assets, and docs do not require
  forest area, species ratios, canopy bins/ranges, tree/bush/prop population
  density, understory, global tree spacing, or forest-route clearance.
- Optional tree-shaped analytic fixtures may remain, but no species/canopy
  value is a product pass condition.
- The existing 32 m/4 m object index, sorted/deduplicated bounded queries,
  exact affected-object cap/oracles, overlap safety, ownership, mutation
  reconciliation, resource accounting, digest/machine identity, truthful
  failure state, and atomic evidence-output tests still pass.
- #66 and the paired forest gate are superseded; no generic substrate issue or
  first human-review path depends on `M-044`/`M-081`.
- PR #363 remains closed/unmerged with a complete disposition; #344 is closed
  as superseded after that record; PR #365 is byte-for-byte and
  tracker-state unchanged.
- All five required Rust quality commands pass.

## Risks

- Over-deletion: tree/canopy shape math and tests are generic overlap/ownership
  behavior and must not be removed with the ecology validators.
- False DAG repair: removing only the JSON dependency array while leaving body
  metadata or live GitHub dependency text will keep the operator graph blocked.
- Accidental architecture: replacing the deleted manifest/proof in this issue
  violates the completion boundary.
- Recovery contamination: do not cherry-pick PR #363; its branch includes
  forest logic and unrelated changes. Port only later through a separately
  reviewed generic issue.
- PR #365 mutation: snapshot its remote state before any tracker work and
  compare afterward.
