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
- GitHub issue #66 is the open `M-044` assisted gate and #111 is the open
  paired `M-081` gate. The local graph and live issue bodies encode edges
  twice: manifest IDs in `depends_on`/“Depends on”/“Inputs” and numeric issue
  IDs in the generated footer. Live incoming #66 edges are #70, #73, #103,
  #111, and #229. Live incoming #111 edges are #112, #113–#117, #119, #122,
  #139, #143, and #229. In addition, #113–#117 depend on #112, the
  verification issue whose only subject is #111.
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
   (`horizon_tree_ids` and its tree-member cap/evidence field), but preserve
   the generic streaming partition lifecycle. `HorizonCellKey` currently also
   keys `HorizonLifecycle` and its stale-token, pin/evict, and reactivation
   tests in `streaming/lifecycle.rs`; relocate that key beside the lifecycle
   (and make it crate-private if no public consumer remains) rather than
   deleting it. Remove only its tree-index export/use. Preserve the generic
   32 m dependency table, 4 m sample table, bounded/exact edit attribution,
   and all existing lifecycle tests. Do not replace tree aggregation or the
   64 m lifecycle partition with a newly designed API or cell size here.
6. Remove forest species/understory/card assets from the required runtime
   inventory and their per-file contract tests. Keep generic prop assets,
   shared-handle behavior, and the portable instanced vegetation shader where
   it remains useful to generic repeated-object presentation.
7. Keep `feasibility-mutation` as a standalone scenario for now, but remove
   its `--forest-proof` argument and `forest_report_sha256` field. This does
   not make it a replacement product gate.

## Red/Green TDD path

### Red: independent test-author handoff

The Red suite must be authored by a separate adversarial test agent/session,
not by the implementation agent. That test author receives this plan and the
current `master` behavior, writes only the tests and test-fixture expectation
changes below, runs them against the unmodified product code, and records the
expected failures. The test-only handoff must be reviewed/frozen before the
implementation agent starts Green. The implementation agent may delete
retired forest-only tests as specified below, but may not weaken or rewrite the
new removal/preservation expectations; a genuinely incorrect Red expectation
goes back to the independent test author for revision.

Add `crates/moria-world/tests/forest_surface_removed.rs` with repository-level
contract tests:

- `checked_in_forest_population_and_commands_are_absent` asserts that
  `assets/config/curated_manifest.ron` and the `crates/moria-curate` workspace
  member do not exist, and that active contributor/CLI sources contain neither
  `prove-forest` nor `--forest-proof`.
- `active_tracker_supersedes_the_forest_gate` parses `docs/issues.json`, asserts
  that `M-044` has the `superseded` label and a #380/#381 disposition, and
  asserts that `M-081` and `V-26` are superseded and that no non-superseded
  node depends on `M-044`, `M-081`, or `V-26`.
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
- In `crates/moria-world/tests/telemetry_reports.rs`, replace the retired
  forest-report fixture with a valid `MutationFeasibilityReport` fixture that
  has no `forest_report_sha256`. Assert canonical JSON omits that field while
  the retained validators still reject an inconsistent pass flag, a non-finite
  metric, and the wrong machine identity. This is Red because the current
  struct/serialized schema requires the forest digest; the minimum Green change
  is removing only that field and validator while keeping the shared checks.
- In `crates/moria-world/tests/asset_declarations.rs`, assert the reduced
  inventory exactly and move the shared-handle regression in
  `asset_validation.rs` from `BirchNear` to retained `Boulder`/`Rock` handles.
- Record the existing `streaming/lifecycle.rs` `HorizonLifecycle` tests as
  green preservation characterizations. They are not forest-membership tests
  and the implementation agent must keep them intact; only the key import may
  change during the Green relocation.

Do not weaken the already-green generic characterizations. Run these before
the removal and record their green baseline:

```sh
cargo test -p moria-world --test object_index_contract
cargo test -p moria-world --test object_shapes_and_ownership_contract
cargo test -p moria-world --test curation_model_contract
cargo test -p moria-world streaming::lifecycle
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
   - In `objects/index.rs`, remove `HORIZON_CELL_METERS`, the tree-only
     `horizon_counts` capacity check, `horizon_tree_ids`, `horizon_key`, and
     only the Horizon-membership assertions from their object-index contract
     cases. Split combined tests where necessary so sorted placement queries,
     capacity failures, stable overlap witnesses, and out-of-region behavior
     remain covered. Move the small `HorizonCellKey` value type to
     `streaming/lifecycle.rs` so `HorizonLifecycle` retains its current key,
     token, revision, pin, eviction, and reactivation semantics. Remove the
     former public exports from `objects/mod.rs` and `lib.rs`; do not remove or
     redesign `HorizonLifecycle`.

3. Remove forest proof/report coupling while preserving evidence integrity:

   - Remove `ForestFeasibilityReport`, `WorstEditTargetEvidence`,
     `FOREST_SCHEMA`, forest validators, and exports from
     `telemetry/{reports.rs,mod.rs}`. Retain the independent test author's
     mutation-report cases in `tests/telemetry_reports.rs`; do not delete the
     test target with the forest fixture.
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
   - Remove those retired paths and checksums from
     `assets/manifests/{asset_licenses.ron,asset_budgets.ron}` and update their
     placeholder contract fixtures; keep all retained asset license/budget
     entries unchanged.
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
     `M-129`, species/understory/card acquisition `M-130`–`M-139`,
     `M-161`, and their wire-in nodes `M-162`–`M-171`, as superseded by
     #380/#381. Also supersede `V-26`, whose only subject is `M-081`. Clear
     active `depends_on` data from the superseded gate/verification/asset
     nodes; historical predecessors may be named only in their supersession
     prose. Mixed nodes such as `M-086`, `M-119`, and `M-180` must retain their
     generic clauses but mark forest clauses superseded/needs-replan; do not
     invent replacement deliverables.
   - Repair every local incoming gate edge, not only the direct-ID list:
     remove `M-044` from `M-071`, `M-077`, `M-081`, `M-180`, and `V-11`;
     remove `M-081` from `M-082`, `M-083`, `M-084`, `M-085`, `M-091`, `M-099`,
     `M-100`, `M-101`, `M-107`, and `M-180`; and remove superseded `V-26` from
     `M-082`, `M-084`, `M-085`, `M-091`, and `M-101`. `M-081`, `M-099`,
     `M-100`, and `V-26` remain superseded rather than becoming runnable.
     Remove obsolete `M-129` ordering edges from retained license/budget nodes
     `M-148`–`M-151`, and remove `M-161`–`M-171` from mixed final audit
     `M-180`.
     Keep each `depends_on` array and body “Depends on”/“Inputs” sections
     synchronized.

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
   close it. Supersede and close #111 and its single-purpose verification #112.
   Repair both forms of every live incoming edge:

   - remove #66/`M-044` from #70, #73, #103, #111, and #229;
   - remove #111/`M-081` from #112, #113–#117, #119, #122, #139, #143, and
     #229; and
   - remove #112/`V-26` from #113–#117.

   Update both the prose `**Depends on:**`/Inputs entries and generated numeric
   dependency footers. Mark forest-only #139 and #143 superseded rather than
   making them runnable; mixed issues retain their generic clauses and are
   marked needs-replan where removing the forest clause leaves unresolved
   scope. Re-list all open issue bodies after the edits and fail verification
   if any active dependency section/footer still names `M-044`, `M-081`,
   `V-26`, #66, #111, or #112. Narrative supersession references in #380/#381
   are allowed.
4. Supersede and close forest-only #139, #143, species/understory/card
   acquisition #178–#187, generated-manifest acquisition #196, their wire-in
   issues #210–#219, and generated-manifest wire-in #228. Mark mixed #124,
   #168, and #229 needs-replan in their forest portions. Remove #196 from the
   retained license/budget issues #197–#200, and remove retired #210–#219 and
   #228 from #229, in both prose and numeric footers. Verify no open
   acquisition/wire-in issue still requires a deleted forest asset or curated
   manifest.
5. Re-read PR #365 and assert it is still open with the same head OID, body,
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
- `HorizonLifecycle` keeps its existing token/revision, pin/evict,
  previous-presentation, and reactivation tests green using the relocated
  `HorizonCellKey`; only tree membership in the object index is removed.
- #66/#111 and single-purpose verification #112 are superseded; no active
  local or live dependency metadata (including numeric footers) points to
  `M-044`/#66, `M-081`/#111, or `V-26`/#112, and no generic substrate issue or
  first human-review path remains transitively blocked by them.
- Open forest-only asset acquisition/wire-in issues and their curated-manifest
  counterparts are superseded; retained license/budget work and the mixed final
  audit no longer depend on those retired nodes.
- PR #363 remains closed/unmerged with a complete disposition; #344 is closed
  as superseded after that record; PR #365 is byte-for-byte and
  tracker-state unchanged.
- All five required Rust quality commands pass.

## Risks

- Over-deletion: tree/canopy shape math and tests are generic overlap/ownership
  behavior and must not be removed with the ecology validators.
  `HorizonCellKey` must move with the generic streaming lifecycle if its
  tree-index API is removed.
- False DAG repair: removing only the JSON dependency array, only manifest-ID
  references, or only direct dependents leaves body metadata, numeric live
  footers, and the #112 verification hop blocking the operator graph.
- Accidental architecture: replacing the deleted manifest/proof in this issue
  violates the completion boundary.
- Recovery contamination: do not cherry-pick PR #363; its branch includes
  forest logic and unrelated changes. Port only later through a separately
  reviewed generic issue.
- PR #365 mutation: snapshot its remote state before any tracker work and
  compare afterward.
