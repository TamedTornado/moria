# Implementation Sequence and Feasibility Gates

## Sequencing rule

Implementation begins with one feasibility wave that retains production code and proves the two highest-risk assumptions before feature breadth is allowed. The wave is not a throwaway prototype: it uses the public `moria-world` boundary, the checked-in Product One configuration/manifest, the production edit kernel, snapshot format, extractor, seam ownership, object dependency logic, dressing derivation, Bevy mesh installation, and render-extraction acknowledgement. Placeholder visual assets are allowed only when they have the final bounds, LOD count, handle-sharing behavior, vertex formats, and shader/material paths declared in [assets.md](assets.md).

Both gates below are mandatory. A green unit test, theoretical complexity bound, or partial pipeline timing cannot substitute for either headed/manifest proof. Gate artifacts record the git commit, world/config/manifest digests, release profile, command line, and complete machine profile. F1 and F2 must be produced from the same gate-baseline commit. Their passing pair authorizes downstream work; any later change to object generation/indexing, edit dirtying/dependency evaluation, mesh/seam extraction, dressing refresh, installation, barrier acknowledgement, or feasibility thresholds invalidates the pair and must rerun both gates before that change merges.

## Gate F1 — checked-in forest and object-index feasibility

Run on the 32 GB M4 Mac Mini in a release build:

```sh
cargo run --release -p moria-curate -- prove-forest \
  --output target/feasibility/forest.json
```

`prove-forest` regenerates the canonical manifest, byte-compares it with `assets/config/curated_manifest.ron`, opens that checked-in manifest through the production runtime validator, and writes `moria-product-one-forest-feasibility` JSON. It must prove all of these in the same manifest and run:

- forest eligible area is at least 120,000 m2; tree anchors have at least 5 m pairwise spacing; tree and per-kind object counts meet the density formulas in [config.md](config.md); both species meet their required shares;
- every tree canopy radius is in 2–4 m, each species contains lower-range and upper-range examples, and no accepted non-ruin solid voxel shape overlaps another or an authored ruin-stamp coordinate;
- the configured 3 m route-clearance volume around every forest traversal segment is free of registered-object solid cells while the route remains inside the qualifying forest corridor;
- the production two-level immutable object index satisfies every per-object, per-cell, edit-candidate, dependency-footprint, and 64 m Horizon-tree membership cap; complete retained bytes are at most 16 MiB and dependency-coordinate allocation is zero;
- runtime object-manifest validation plus index construction is at most 1,000 ms, of which index construction is at most 250 ms; and
- deterministic enumeration of legal radius-3 m forest/hillside surface-cell centers with at least one non-ruin surface-dependency hit and one eligible dressing anchor finds and records the stress target used by Gate F2. Selection maximizes `(broad_dependency_candidates, exact_dependency_ids, dependency_bricks, changed_bricks)` lexicographically, then chooses the lexicographically smallest voxel center; the recorded target must remain inside all configured edit candidate/affected-object bounds.

The report includes eligible areas, expected and actual count by kind/species, minimum observed tree spacing, canopy min/max and range-bin counts, route-clearance minimum, exact overlap conflict count, placement count, both grid entry/cell maxima, edit-candidate maximum and target, dependency-brick maximum, object validation/index times, retained-byte category totals, and dependency-coordinate allocation bytes. Any missing field fails report validation; aggregate values do not replace the stable first-conflict/first-failing-ID detail.

The command performs deterministic correctness checks on every machine. Its time thresholds are acceptance evidence only on the named M4; an unlabelled or different machine report cannot open the gate.

## Gate F2 — production mutation vertical slice

F2 depends on F1's exact manifest digest and worst-case target. Run on the same 32 GB M4 Mac Mini, Metal backend, 2560 x 1440 window, shipping presentation settings, final vertex formats/shaders, and release profile:

```sh
cargo run --release -p moria-bench -- --scenario feasibility-mutation \
  --resolution 2560x1440 \
  --forest-proof target/feasibility/forest.json \
  --output target/feasibility/mutation.json
```

The scenario warms the production renderer for 300 frames, then exercises three public workloads. First, the manifest-tagged 3 m hillside carve proves the Product One debug interaction and capsule traversal. Second, a colony workload models eight independent workers by submitting bounded box/sphere edits across a 32 m x 32 m x 16 m designation volume; designation ownership and worker AI remain outside the substrate. Third, a progressive 16 m-radius catastrophic dig proves that one logical operation can change landscape scale without materializing, pinning, or reconciling the entire volume in one frame. Each workload starts from the same seed and configured baseline with no private voxel write/reset path.

The colony workload keeps up to eight accepted worker requests active, uses deterministic disjoint and intersecting targets, and requires every request to make progress under contention. The catastrophic workload intersects F1's deterministic maximum-candidate forest/hillside target and installs a secondary presentation focus that simultaneously exercises an active Horizon aggregate. It must therefore prove base-card exclusion, revision-matched derived payload/tombstone work, and eventual full reconciliation while primary-focus presentation is allowed to finish earlier.

For each accepted command, tracing begins in `WorldEditWrite::submit` and ends at `EditReconciliationComplete`. The trace must contain nonempty or explicit no-work records for admission, scheduling, batch staging, atomic batch mutation/delta update, dirty-brick and object discovery, dependency eligibility, snapshot creation, terrain/object mesh extraction, same-LOD and LOD seam work, old dressing removal and revised dressing installation, Bevy `Assets<Mesh>`/entity installation or removal, primary-focus readiness, render extraction, GPU buffer create/write/free, render queue acknowledgement, and terminal reconciliation. An omitted stage is a failed gate, not zero time.

Pass conditions are:

- every `submit` call completes within 2.0 ms and accepted work emits `EditAccepted` exactly once;
- the interactive carve and catastrophic operation produce their first nonempty `EditBatchCommitted` within 100 ms of acceptance, while every colony request produces its first nonempty commit within 250 ms;
- a committed primary-focus batch reaches `EditPrimaryPresentationReady` within 250 ms at p95 and 500 ms maximum;
- while runnable work exists, aggregate mutation throughput is at least 32 changed bricks per second and no accepted request waits more than 500 ms between nonterminal commit batches;
- the interactive carve reaches full reconciliation within 1 second; the colony and catastrophic workloads each reach `EditReconciliationComplete` within 30 seconds;
- the normal full-manifest headed startup reaches `WorldReady`/scenario-view readiness in less than 5,000 ms from process entry;
- no rendered-frame interval from acceptance through reconciliation exceeds 33.3 ms;
- the signature opening is clear under the public capsule sweep and its revised terrain/object/dressing state is visible at its final revision;
- the catastrophic reconciliation contains at least one non-ruin object rebuild/swap, one Horizon aggregate membership rebuild with a current-truth per-ID payload or empty tombstone, and at least one old dressing removal plus revised dressing batch result, so zero-work records cannot satisfy the proof;
- the catastrophic workload records `horizon_partition_checked:true`, at least one excluded base card, and at least one derived record/tombstone;
- dirty discovery plus dependency intersection/eligibility has combined measured wall time at most 1.0 ms for the stress target, with separate values and candidate/affected-ID/delta-brick/predicate-test counts reported;
- before mutation, the query-cost subrun exercises exact-max and cold inactive-forest inputs plus a normal frame bundle (player substep sweeps, one 9 m camera probe, one 64 m debug ray, water/contact samples, and active-band reads): each frame-critical call has p99 at most 1.0 ms, the bundle p99 is at most 2.0 ms, and no call exceeds 4.0 ms; maximum column/metadata page and two-brick cell-page costs meet the 1.0/4.0 ms p99 and 8.0 ms cell-page maximum in [api.md](api.md);
- every committed batch's affected terrain tiles, seams, non-ruin object roots/Horizon aggregate members, ruin chunks when applicable, water patches when applicable, and dressing batches are accounted for; every installed/removed item has a matching extraction plus GPU-prepare/free plus render-queue acknowledgement before terminal reconciliation;
- terrain mesh, seam, Bevy install/removal, render extraction, GPU upload/free, and render-queue acknowledgement counts are each nonzero across the three workloads; optional ruin/water branches may report zero only when no workload intersects them; and
- there is no asset fallback, stale-result installation, queue overflow, synchronous rejection, missing trace stage, or measurement invalidation.

The 1.0 ms discovery/eligibility share and 4 ms per-frame mutation CPU budget reserve the rest of the frame for simulation, extraction, installation, rendering, and backend work. The 100/250/500 ms progress limits distinguish responsive feedback from complete reconciliation; the 32 changed-bricks/second floor proves useful sustained work without demanding an atomic landscape rewrite. F2 records stage timings and counters rather than treating analytic bounds as performance evidence. The full mutation-workload benchmark later repeats the contract; F2 is the earlier permission to build outward.

## Failure and change control

Failure of F1 or F2 closes the feasibility wave, blocks every downstream issue listed below, and immediately returns this TDD for revision. The revision must attach the failed immutable artifact, identify the measured failing constraint/stage, and specify the technical redesign and replacement proof before implementation resumes. Even an optimization that preserves public behavior must first appear in that reviewed TDD revision; the team does not build outward or perform an unreviewed sequence of target-driven tweaks after a red gate.

The revision may optimize or replace internals behind the existing public/deterministic contracts, then rerun both gates from one new baseline commit. It may not lower density/counts, shrink canopy ranges, increase route tolerance, skip shape-disjointness, relax startup/index budgets, shrink the interactive/colony/catastrophic workloads, omit a lifecycle or reconciliation stage, raise the 33.3 ms/1.0 ms/progress/throughput thresholds, or relabel partial progress as terminal completion. If a contract change is proposed, the revision must mark an explicit `Design Divergence` with rationale, user-visible effect, and Product approval requirement. Until the revision is approved and its replacement proofs pass, downstream issues remain blocked. A report retains `passed:false` and stable failure reasons; reruns never overwrite a failed artifact.

## Issue and dependency plan

| ID | Deliverable | Depends on | Exit condition |
|---|---|---|---|
| `P0` | Workspace/public boundary, deterministic scalar fixtures, feasibility tracing/report validators, query boundary tests | — | Headless contracts pass; no broad world content |
| `F1` | Minimal production generator/curator, full checked-in forest manifest, compact two-level index | `P0` | Gate F1 artifact passes on M4 |
| `F2` | Progressive mutation vertical slice: interactive carve, colony streams, catastrophic volume, seams, object dependency/derived root, dressing, staged readiness and GPU reconciliation | `P0`, passing `F1` digest | Gate F2 artifact passes on M4 |
| `G1` | Broad region generation: complete geology, cave, ore, aquifer, river/lake, ruin and distance bands | passing `F1` and `F2` | Generator/system/property tests and milestone geology evidence |
| `V1` | Final vegetation/prop/player assets and full forest/dressing presentation | `G1`, passing `F1` and `F2` | Asset validation and dressed-world checklist |
| `T1` | Traversal/camera/light polish and continuous playable route | `G1`, `V1` | Public-query route integration and manual run |
| `S1` | Delta save/load and exact restoration | passing `F1` and `F2`; may proceed with `G1` after gates | Persistence properties and heavy-save round trip |
| `B1` | Full flythrough/mutation-workloads, Linux baseline, release evidence | `G1`, `V1`, `T1`, `S1` | All benchmark/visual contracts, subject to resident-memory decision |

`G1`, `V1`, `T1`, `S1`, and `B1` may not be marked in progress before both gate artifacts pass on one baseline commit. In particular, broad generation, final assets, generalized dressing, traversal polish, persistence, and full benchmarking are downstream of the feasibility wave even if staff are otherwise available. Changes to a gated subsystem trigger the rerun rule above; unrelated downstream commits do not erase the historical proof.

## Final acceptance dependency

Passing F1/F2 authorizes implementation; it does not resolve the graphics-memory Design Divergence. `B1` cannot report overall Product One acceptance while resident graphics-memory evidence is absent. Final acceptance requires either a reviewed resident measurement below 2,097,152,000 bytes on every named acceptance configuration or an explicit Product-approved estimate-substitution ID recorded in each report. The application ledger alone remains insufficient.
