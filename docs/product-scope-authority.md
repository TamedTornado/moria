# Moria product-scope authority

Status: **approved for execution**

Approved by the operator on 2026-07-23. This document is the active authority
for correcting Moria's scope under
[issue #380](https://github.com/TamedTornado/moria/issues/380). It supersedes
conflicting product requirements in active design, TDD, planning, benchmark,
test, asset, issue, and contributor-guidance artifacts. Historical artifacts
must be retained as project history but marked as superseded where they
conflict with this decision.

## Product boundary

Moria is a reusable voxel-world substrate. It provides:

- deterministic procedural three-dimensional material truth;
- sparse brick storage, activation, streaming, and eviction;
- terrain and registered-object meshing;
- bounded spatial queries and collision primitives;
- mutation with exact affected-region and affected-object rebuilding;
- delta persistence and deterministic save/reload;
- telemetry, resource accounting, benchmarks, and evidence integrity; and
- a thin diagnostic viewer for headed human inspection.

Moria is not a game or ecology simulator. The product does not require:

- canonical forest area, species diversity, species ratios, density,
  canopy-cover distributions, global tree spacing, or dense understory;
- a globally enumerated curated forest population;
- a third-person controller, player locomotion, or game-camera behavior;
- a human explorer mesh or a character-asset pipeline;
- skeletons, skinning, animation clips, or an animation state machine;
- a curated traversal experience; or
- one named machine's timing as a platform-independent correctness gate.

A free diagnostic camera is permitted. Tree-shaped or forest-looking fixtures
are permitted as optional bounded test or stress data, but they do not create
ecology requirements.

## Capabilities to preserve and generalize

Existing work must be judged by capability, not by the contaminated scenery
that exercised it. Preserve and generalize:

- stable registered-object identities;
- deterministic generation by bounded region or cell;
- compact spatial indexing and bounded broad-phase queries;
- an independent exact oracle for broad-phase agreement;
- exact mutation dependency attribution;
- object/object and object/stamp overlap safety where presentation needs it;
- invalidation and reconciliation after mutation;
- authored-versus-derived presentation ownership;
- bounded memory accounting;
- immutable, truthful pass/failure evidence; and
- machine-identified timing and resource measurements.

Remove species, canopy, ecological-density, player, character, and animation
assumptions from these generic capabilities.

## Data and fixture correction

Do not merge or recreate the globally enumerated forest population from
recovery PR #363. Product data must instead use:

- a world seed;
- compact procedural-generation parameters;
- generator and schema versions plus digests;
- explicitly authored anchors only for bounded fixtures; and
- small, reviewable checked-in fixtures.

Large populations belong in reproducible generated stress output, not Git.
Runtime generation must be deterministic and bounded by region or cell, with
stable identities, without first enumerating the whole world.

## Acceptance model

Replace the forest-oriented proof with a substrate-oriented proof, currently
named `prove-substrate`. It must demonstrate:

- identical inputs produce identical generation and query results;
- sparse activation does not expand the complete world;
- stable object identities and bounded queries;
- broad-phase output agrees exactly with an independent oracle;
- mutation attribution is exact;
- unaffected objects remain unchanged;
- mesh/render invalidation reconciles completely;
- save/reload reproduces material truth;
- retained memory is measured honestly; and
- timings include complete machine identity.

Large-population scalability must be tested through parameterized generated
stress fixtures. Population counts are workload parameters, not shipped-world
requirements.

Acceptance stages are:

- **F1:** cross-platform headless substrate correctness, resource accounting,
  and evidence integrity. Machine-specific performance is reported evidence,
  not universal functional correctness.
- **F2:** headed human inspection of three-dimensional terrain, underground
  geometry, streaming, edits, diagnostic voxel views, and object
  invalidation/rebuilding.

## Recovery disposition

Recovery PR #363 must not be merged or rebased wholesale. Inventory every
change as substrate-generic and salvageable, forest/ecology-specific and
rejected, or generated artifact and discarded. Port approved generic work onto
a clean branch from current `master`.

Recovery PR #365 must be audited. Preserve generic report identity, immutable
evidence, truthful pass/failure states, workload reconciliation, stage
accounting, and digest binding. Remove or generalize forest-F1, third-person-F2,
and universal named-machine assumptions.

Recovery epic #325 closes only after every recovery item is recorded as merged,
adapted, or explicitly superseded.

## Ordered execution

The correction must proceed in this order:

1. Treat this document as the approved scope and authority decision.
2. Inventory active documents, code, tests, assets, issues, and recovery PRs as
   keep, remove, or generalize.
3. Design and review the compact generation and bounded-fixture architecture.
4. Correct active design and TDD contracts before implementation agents follow
   them.
5. Decompose the reviewed correction into explicit, dependency-ordered issues
   with test specifications.
6. Implement the compact data path and generic substrate proof.
7. Port approved generic recovery changes from clean current `master`.
8. Remove or supersede contaminated code, assets, tests, and tracker items.
9. Run focused tests, parameterized stress fixtures, and a complete Linux Rust
   gate through cargo-reapi.
10. Run the headed diagnostic build on macOS for F2 human review.
11. Reconcile the recovery epic and all affected issues against the merged
    result.

Implementation may not run ahead of reviewed scope analysis, design, TDD, issue
decomposition, and test specifications.

## Completion conditions

- No ecology, player, skeletal-animation, or curated-game requirement remains
  authoritative.
- No full enumerated forest manifest is a checked-in product artifact.
- Substrate-generic protections salvaged from recovery pass.
- Remote `master` builds and tests from a clean checkout.
- F1 proves the substrate without product-specific scenery assumptions.
- F2 visibly demonstrates continuous, mutable three-dimensional voxel truth.
- Every affected recovery change and tracker issue has a recorded disposition.
- Active repository documents and contributor guidance describe the corrected
  substrate.
