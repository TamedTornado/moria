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
- The retired surface is referenced more broadly than the two proof flags:
  `moria-curate` is also present in the workspace/Cargo lockfile, contributor
  commands, active TDD, test specs, and tracker inventory; the curated-manifest
  path/stable ID/type vocabulary is present in runtime asset declarations,
  source/tests, active TDD, five test specs, and tracker nodes. `M-001` still
  claims four workspace packages and `M-003` still claims 30 runtime assets,
  although this removal leaves three packages and 19 declared runtime assets
  (17 content entries plus the two registry manifests).
- `moria-bench` requires `--forest-proof` for `feasibility-mutation`, and
  `MutationFeasibilityReport` stores `forest_report_sha256`.
- `RenderingConfig::{cluster_visibility_m,horizon_object_cell_size_m,
  horizon_derived_lod_m}` occur only in the config type/default, presentation
  RON and its fixture, config validation, and config documentation. No runtime
  renderer, streaming, object-index, or other substrate-generic consumer reads
  them.
- `MutationWorkloadEvidence::{horizon_partition_checked,
  horizon_excluded_base_cards,horizon_derived_records}` occur only in the
  report schema and the F2 documentation. No producer, validator, or
  substrate-generic accounting consumer reads them.
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
8. Remove all six inspected orphan fields named above. The three rendering
   fields describe only the retired tree-cluster/cell/derived-LOD contract, and
   the three workload fields describe only base-card partition evidence. Do
   not rename them, retain them as generic-looking settings, or replace them
   with a new aggregation. Keep `object_visibility_m`, generic workload stage
   timings/counts, changed-voxel/brick/batch counts, and renderer barrier
   counts.

## Red/Green TDD path

### Red: failing removal contracts

Use ordinary Red/Green TDD within the existing implementation node. Before
editing production code or deleting retired forest-only tests, add the
repository-level removal tests and focused fixture/schema expectation changes
below, run them against the current product code, and record the expected
failures. Then make the minimum Green changes in the following section. Keep
the new removal and preservation expectations intact unless repository
evidence shows an expectation itself is incorrect.

Add `crates/moria-world/tests/forest_surface_removed.rs` with repository-level
contract tests:

- `retired_curation_package_manifest_asset_and_commands_are_absent` asserts
  that `assets/config/curated_manifest.ron` and `crates/moria-curate/` do not
  exist; also asserts that `curation/{facade.rs,generate.rs}`,
  `tests/curation_facade.rs`, and the curated-manifest asset fixture are gone
  while `curation/model.rs` remains. Assert that the root workspace contains
  exactly the three retained packages. Scan `Cargo.toml`, `Cargo.lock`,
  `.cargo/**`, `README.md`,
  `AGENTS.md`, every `crates/*/Cargo.toml`, and all executable/test Rust under
  `crates/**` (excluding this enforcement file and the old-option rejection
  literals in the `moria-bench` `#[cfg(test)]` module). The scan must reject
  every active occurrence of `moria-curate`/`crates/moria-curate`, not merely
  `prove-forest`; it must also reject `prove-forest`, `--forest-proof`, the
  `curation` Cargo feature/test target, `config/curated_manifest.ron`,
  `moria.config.curated_manifest`, and `AssetId::CuratedManifest`. This catches
  stale `generate`/`check` package references, lockfile entries, facade tests,
  and runtime asset identity as well as ordinary command invocations. The
  focused `moria-bench` parser tests below prove the old option is rejected
  rather than merely undocumented.
- `retired_forest_schema_is_absent_from_executable_surfaces` recursively scans
  the exact roots `crates/moria-world/src`, `crates/moria-bench/src`,
  `crates/moria-demo/src`, `assets/config`, and `assets/manifests` for the
  retired report/config field identifiers. Its allowlist is limited to the
  `#[cfg(test)]` portion of `crates/moria-bench/src/cli.rs` that passes the old
  option to assert rejection and this enforcement test's own string literals.
  The production portion of `cli.rs` remains scanned. The identifiers include
  `ForestFeasibilityReport`, `forest_report_sha256`,
  `max_horizon_tree_members_per_cell`, `cluster_visibility_m`,
  `horizon_object_cell_size_m`, `horizon_derived_lod_m`,
  `horizon_partition_checked`, `horizon_excluded_base_cards`, and
  `horizon_derived_records`.
- `retired_curation_and_curated_manifest_references_are_dispositioned` audits
  every occurrence of `moria-curate`, `crates/moria-curate`,
  `prove-forest`, `--forest-proof`, `curated_manifest`,
  `curated-manifest`, `curated manifest`, and `CuratedManifest` across
  `docs/**/*.md`, `docs/issues.json`, and `test-specs/**/*.md`; it reports
  every unclassified path/line or JSON node. Allowed references are explicit
  and executable:

  - historical whole-file/prefix allowlists:
    `docs/interview-record.md`, `docs/issue-review.md`,
    `docs/tdd-review.md`, `docs/pm-runs/**`, and `docs/seeds/**`;
  - supersession references: a `docs/tdd/**`,
    `docs/design-document.md`, or `docs/engineering-evidence.md` Markdown
    paragraph containing any token must also contain `superseded` and #380 or
    #381; no command, asset path/stable ID, or checked-in global-manifest claim
    may remain normative. The wholly retired asset specs
    `test-specs/issue-{210..219}.md` may retain their bodies only with the
    required status line, while every other mixed spec must mark the
    containing command/manifest clause superseded. The generic model section
    in `docs/tdd/data-model.md` and `issue-336` are the only retained active
    exceptions: they may name the in-memory `CuratedManifest` model only after
    explicitly saying that the type is generic generated metadata and is not
    the deleted checked-in asset or an acceptance gate;
    `docs/issues.json` references are allowed only inside nodes whose parsed
    labels and body mark that forest clause or entire node superseded;
  - disposition whole-file allowlist:
    `docs/recovery/pr-363-disposition.md`;
  - enforcement-reference allowlists:
    `docs/mini-dag/issue-381/plan.md`,
    `crates/moria-world/tests/forest_surface_removed.rs`, and the
    `#[cfg(test)]` module of `crates/moria-bench/src/cli.rs`.

  The test also uses a narrow source allowlist for the retained generic
  `CuratedManifest` symbol: `curation/model.rs`, its `curation/mod.rs` and
  `lib.rs` re-exports, and `curation_model_contract.rs`. Those files may not
  contain the retired package name, checked-in path/stable ID, generator,
  facade, or proof vocabulary. This makes every curated-manifest reference
  visible to the test without deleting the generic identity/order/overlap
  model. This is not a scan for broad words such as “forest,” “tree,” or
  “Horizon.”
- `active_tracker_has_no_retired_curation_or_stale_inventory` parses
  `docs/issues.json` structurally. It asserts:

  - every node containing the retired package/command/path/stable-ID/type
    vocabulary is either explicitly superseded by #380/#381 or has that
    clause removed/reworded as generic metadata;
  - retired manifest/forest asset scaffolds `M-017`–`M-027`, curator/gate
    nodes `M-042`–`M-044` and `M-081`, acquisition nodes
    `M-129`–`M-139`, wire-in nodes `M-161`–`M-171`, and `V-26` are
    superseded, while mixed verification/integration nodes no longer list
    their removed products;
  - `M-001` names exactly the three retained workspace packages and no
    four-package/curator claim; `M-003`'s inventory cardinality equals the
    executable `ASSET_COUNT` (19), not 30, and its body does not promise
    immutable retired paths; registry content counts agree at 17;
  - no non-superseded node produces a deleted package/path/asset, depends on a
    superseded forest/curator/asset node, or retains `M-044`, `M-081`, `V-26`,
    `M-129`, or `M-161` in `depends_on`, body “Depends on,” “Inputs,”
    “Produces,” or acceptance inventory; and
  - every parsed `depends_on` array remains synchronized with the body
    dependency/input clauses after repair.
- `active_tree_asset_and_horizon_specs_are_dispositioned` reads the exact
  `test-specs/issue-{210..219}.md` set and requires the #380/#381 supersession
  status in every file. For the mixed spec list in Green step 5, any retained
  paragraph naming tree-Horizon membership, base cards, derived/tombstone
  records, or the retired F1 forest digest/command must carry the same explicit
  supersession marker. The test also asserts that `issue-67`, `issue-85`, and
  `issue-351` still contain their generic token/source-revision, stale-result,
  pin/evict, and reactivation lifecycle clauses; runtime preservation remains
  covered by the existing `streaming::lifecycle` tests.
- `active_runtime_inventory_has_no_forest_contract_assets` asserts the runtime
  asset paths exclude the curated manifest, birch/pine/bush/grass population
  assets, and tree Horizon cards while retained prop paths remain declared.
  It asserts the exact resulting `ASSET_COUNT == 19`, exact retained path set,
  and 17-entry license/budget content inventories so stale cardinalities cannot
  survive in code, fixtures, or registries.

These tests fail on the current tree because the manifest/crate/lockfile,
generate/check/proof references, curated-manifest runtime identity, retired
schema fields, and asset declarations exist; active documents/specs/tracker
nodes still treat the curator and checked-in manifest as current; `M-001` and
`M-003` advertise stale package/asset cardinalities; active asset/Horizon specs
lack supersession markers; #66 is active; and the dependency edges remain.

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
  has neither `forest_report_sha256` nor the three retired Horizon/base-card
  workload fields. Add these focused preservation cases:

  - `mutation_report_preserves_generic_identity_digests_and_truth_state`
    validates a passing report and round-trips its schema, release build/git
    identity, `WorldIdentity` seed/parameter digest/bounds, generic
    `manifest_sha256`, machine/profile digest, backend, and resolution. It
    asserts canonical JSON omits `forest_report_sha256`; rejects a malformed
    manifest digest, git identity, or M4 machine identity; rejects
    `passed:true` with a reason and `passed:false` without a reason; rejects
    unsorted/duplicate/empty reasons; and accepts `passed:false` with a sorted
    nonempty reason list. This preserves truthful failed evidence rather than
    requiring every valid report to be green.
  - `mutation_report_rejects_workload_and_barrier_misreconciliation` mutates
    the passing fixture independently to cover zero request count,
    `first_committed_frame < submitted_frame`,
    `final_reconciled_frame < first_committed_frame`, mismatched
    `barrier_expected_items`/`barrier_renderer_ready_items`, a missing required
    stage timing/count, and a missing/reordered workload role. Each must retain
    its current deterministic `Inconsistent` or `Missing` rejection.
  - `mutation_report_preserves_accounting_and_finite_measurements` round-trips
    exact `changed_voxels`, `changed_bricks`, `committed_batches`,
    `stage_timings_ms`, `stage_counts`, equal renderer-barrier counts, query
    sample counts, and observed-work maxima. It separately rejects a
    non-finite distribution, stage timing, throughput, runnable-wait, and
    frame metric. Deserializing JSON containing any of the four removed report
    fields must fail under `deny_unknown_fields`.

  These tests are Red because the current Rust fixtures cannot be constructed
  without the forest digest and three Horizon fields. The minimum Green change
  removes only those fields and their forest-digest validation; it does not
  relax header, role, stage, reconciliation, barrier, finite-metric, identity,
  digest, or canonical-serialization behavior and does not invent new
  accounting relationships.
- In `crates/moria-world/tests/asset_declarations.rs`, assert the reduced
  inventory exactly and move the shared-handle regression in
  `asset_validation.rs` from `BirchNear` to retained `Boulder`/`Rock` handles.
- Record the existing `streaming/lifecycle.rs` `HorizonLifecycle` tests as
  green preservation characterizations. They are not forest-membership tests;
  keep them intact, changing only the key import during the Green relocation.

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
   - Remove grass density/understory and all four tree-Horizon fields
     `cluster_visibility_m`, `horizon_object_cell_size_m`,
     `horizon_derived_lod_m`, and `max_horizon_tree_members_per_cell` from
     `RenderingConfig`. Remove their defaults and validators rather than
     moving or renaming them: repository inspection found no
     substrate-generic consumer. Also remove
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
     `telemetry/{reports.rs,mod.rs}`. Retain the new mutation-report cases in
     `tests/telemetry_reports.rs`; do not delete the test target with the
     forest fixture.
   - Remove `forest_report_sha256` and only its validator from
     `MutationFeasibilityReport`. Keep build/world/manifest digests,
     `MachineProfile`, `passed == failure_reasons.is_empty()`, sorted reasons,
     finite metrics, workload reconciliation, and canonical JSON behavior.
   - Remove `horizon_partition_checked`, `horizon_excluded_base_cards`, and
     `horizon_derived_records` from `MutationWorkloadEvidence` and from
     `docs/tdd/{data-model,implementation-plan}.md`. They have no existing
     producer or validator and encode only the retired tree-card partition;
     do not replace them with a new aggregate or counter. Keep workload roles,
     frame ordering, exact changed-voxel/brick/batch and stage accounting,
     equal expected/ready renderer-barrier counts, timing distributions, and
     query evidence.
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
     `ASSET_COUNT` from 30 to the exact 19 retained declarations, update
     `CONTENT_ASSET_COUNT` from 28 to the exact 17 retained license/budget
     entries, and update declaration tests and `WorldRenderAssets` tests.
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
     a replacement acceptance design. Remove all active `moria-curate` and
     checked-in curated-manifest asset/path/stable-ID claims. The one retained
     `CuratedManifest` type section in `data-model.md` must explicitly scope it
     to generic in-memory generated metadata and state that it is not a
     checked-in population or acceptance mechanism.
   - Add a prominent #380/#381 supersession note to active
     `docs/design-document.md` and `docs/engineering-evidence.md`, and remove
     current-evidence claims about the checked-in forest/proof. Historical
     `docs/interview-record.md`, `docs/pm-runs/**`, seed records,
     `docs/issue-review.md`, and `docs/tdd-review.md` remain history; add only a
     status banner where needed to prevent them being read as current
     authority.
   - Add an explicit `Superseded by #380/#381 for current product work` status
     to every retired tree/vegetation asset wire-in spec
     `test-specs/issue-{210..219}.md`; none remains an active asset contract.
     Also mark the forest-specific portions of `issue-60`, `124`, `139`, `143`,
     `228`, `326`, `330`, `344`, and `347` superseded. Where any of those
     files, plus `issue-107`, `issue-334`, `issue-336`, and `issue-339`, mix
     generic and forest requirements, retain the generic determinism, overlap,
     shared-handle, identity, config, metadata-ordering, or report validation
     clause and mark only the curator/checked-in-manifest/forest/F1 clause
     superseded. In `issue-336`, explicitly distinguish the retained in-memory
     generic metadata model from the deleted checked-in curated-manifest asset.
   - Mark the tree-membership, tree-card aggregation, base-card,
     derived/tombstone, and tree-Horizon evidence clauses superseded in
     `issue-54`, `67`, `77`, `85`, `88`, `89`, `93`, `99`, `105`, `131`,
     `133`, `134`, `232`, `335`, `339`, `341`, and `351`. Preserve, in those
     same specs, the generic 32 m/4 m index caps and queries; exact
     changed-mask attribution; authored-versus-derived ownership; key/token/
     revision stale-result rejection; pin/evict/reactivation lifecycle;
     atomic payload replacement; union invalidation after load; resource
     ledger accounting; expected-versus-ready barrier rejection; terminal
     reconciliation; identity/digest validation; and truthful evidence state.
     In particular, `issue-67`, `issue-85`, and `issue-351` continue to specify
     the existing generic `HorizonLifecycle` state transitions, but no longer
     require assigned-tree partitions, base-card membership, derived records,
     or tombstones.
   - Update generic specs `issue-49`, `52`, `69`, `81`, `341`, `343`, and
     `352` to remove forest vocabulary/coupling while retaining their
     substrate properties. A superseded spec remains a historical file; do
     not delete it or rewrite it into a replacement requirement.
   - In `docs/issues.json`, first repair the retained inventory nodes:
     `M-001` must list the three remaining workspace packages and must not
     produce or require `moria-curate`; `M-003` must describe the exact
     19-declaration runtime inventory/17 content entries instead of 30 and
     must not claim removed stable paths are immutable. Reword `M-006` from a
     curated-manifest product contract to the retained generic identity,
     feature/water/route/object/stamp metadata contracts. Remove the
     curated-manifest/curator/removed-asset bullets from mixed historical
     verification nodes `V-3`, `V-4`, `V-9`, and `V-10` while preserving
     their unrelated verified work.
   - Label and annotate obsolete forest/curator/global-manifest nodes,
     including manifest and vegetation/card scaffolds `M-017`–`M-027`,
     `M-042`, `M-043`, `M-044`, `M-081`, `M-099`, `M-100`, generated-manifest
     and species/understory/card acquisition `M-129`–`M-139`, and their
     wire-in nodes `M-161`–`M-171`, as superseded by #380/#381. Also
     supersede `V-26`, whose only subject is `M-081`. Clear
     active `depends_on` data from the superseded gate/verification/asset
     nodes; historical predecessors may be named only in their supersession
     prose. Mixed nodes such as `M-086`, `M-119`, and `M-180` must retain their
     generic clauses but remove/reword curated-manifest and forest clauses as
     superseded/needs-replan; do not invent replacement deliverables.
   - Repair every local incoming retired-surface edge, not only the direct gate
     list: remove `M-017`–`M-027` from retained loader/audit nodes `M-038` and
     `M-119`, remove `M-018`–`M-027` from mixed `V-3`, and remove `M-017` from
     mixed `V-4`; remove `M-042` from `M-071`, `M-086`, and `V-9`, and remove
     `M-043` from `V-10`;
     remove `M-044` from `M-071`, `M-077`, `M-081`, `M-180`, and `V-11`;
     remove `M-081` from `M-082`, `M-083`, `M-084`, `M-085`, `M-091`, `M-099`,
     `M-100`, `M-101`, `M-107`, and `M-180`; and remove superseded `V-26` from
     `M-082`, `M-084`, `M-085`, `M-091`, and `M-101`. `M-081`, `M-099`,
     `M-100`, and `V-26` remain superseded rather than becoming runnable.
     Remove `M-099`/`M-100` from mixed route/verification nodes
     `M-110`, `V-34`, and `V-35`; mark only the affected clauses
     needs-replan. Remove obsolete `M-129`–`M-139` ordering edges from retained
     license/budget nodes `M-148`–`M-151`, and remove `M-161`–`M-171` from
     mixed final audit `M-180`.
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
- Run the executable negative scan with
  `cargo test -p moria-world --test forest_surface_removed`. Its fixed path
  inventory and allowlists are part of the test and must not be replaced by
  an ad hoc `rg` over only the two proof flags. It proves the deleted
  file/package/workspace member and lockfile entry stay absent; all active
  `moria-curate` generate/check/proof references and curated-manifest runtime
  path/stable-ID references are absent; the narrow retained generic
  `CuratedManifest` symbol allowlist has no forest/tool coupling; executable
  config/report sources contain no retired fields; the runtime/license/budget
  inventories are exactly 19/17; every active documentation/spec/tracker
  occurrence is classified; and stale three-package/19-asset tracker
  inventory cannot regress. Separately run the `moria-bench` CLI unit test
  proving `feasibility-mutation` succeeds without a proof and any supplied
  `--forest-proof` is an unknown argument. Historical, supersession,
  disposition, plan, and negative-test references remain allowed only at the
  exact paths listed in the Red test; their tokens are not evidence that a
  command, asset, or gate is still active.
- Re-run the generic characterization tests above, then:

```sh
cargo test -p moria-world --test forest_surface_removed
cargo test -p moria-world --test telemetry_reports
cargo test -p moria-bench cli::tests
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
  its Cargo feature/lockfile/runtime asset identity, `prove-forest`, and
  `--forest-proof` are absent from active product paths. Every remaining
  repository occurrence of the package or curated-manifest vocabulary is
  either a narrow retained generic model reference, an explicit #380/#381
  supersession, auditable history/disposition, or a negative-test literal.
- Active config, validators, reports, tests, assets, and docs do not require
  forest area, species ratios, canopy bins/ranges, tree/bush/prop population
  density, understory, global tree spacing, or forest-route clearance.
- The three retired tree-Horizon rendering settings and three retired
  Horizon/base-card workload evidence fields are absent from executable/config
  schemas; no replacement aggregation was introduced.
- Optional tree-shaped analytic fixtures may remain, but no species/canopy
  value is a product pass condition.
- The existing 32 m/4 m object index, sorted/deduplicated bounded queries,
  exact affected-object cap/oracles, overlap safety, ownership, mutation
  reconciliation, resource accounting, digest/machine identity, truthful
  failure state, and atomic evidence-output tests still pass.
- `MutationFeasibilityReport` tests retain generic build/world/manifest/machine
  identity, exact workload/stage/count/barrier serialization, frame and
  barrier reconciliation rejection, finite-metric rejection, required stage
  and workload-role checks, and both truthful passing and truthful failing
  evidence states after forest fields are removed.
- `HorizonLifecycle` keeps its existing token/revision, pin/evict,
  previous-presentation, and reactivation tests green using the relocated
  `HorizonCellKey`; only tree membership in the object index is removed.
- #66/#111 and single-purpose verification #112 are superseded; no active
  local or live dependency metadata (including numeric footers) points to
  `M-044`/#66, `M-081`/#111, or `V-26`/#112, and no generic substrate issue or
  first human-review path remains transitively blocked by them.
- Open forest-only asset acquisition/wire-in issues and their curated-manifest
  counterparts are superseded; retained license/budget work and the mixed final
  audit no longer depend on those retired nodes. The active tracker describes
  exactly three workspace packages, 19 runtime declarations, and 17 registry
  content entries, with no stale produces/inputs/acceptance inventory for the
  removed curator or global manifest.
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
- False surface scan: checking only `prove-forest`/`--forest-proof` misses
  `generate`/`check`, Cargo.lock/package/feature entries, the curated-manifest
  asset path/stable ID, active specs, and stale tracker package/asset
  cardinalities.
- Accidental architecture: replacing the deleted manifest/proof in this issue
  violates the completion boundary.
- Recovery contamination: do not cherry-pick PR #363; its branch includes
  forest logic and unrelated changes. Port only later through a separately
  reviewed generic issue.
- PR #365 mutation: snapshot its remote state before any tracker work and
  compare afterward.
