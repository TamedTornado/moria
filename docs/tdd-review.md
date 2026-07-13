## Auditor Turn — 2026-07-13T14:30:58Z

Mode: continue
Responding to: none

### Prior Findings Status

No prior findings.

### New Findings

1. Severity: blocking
   Section: `docs/tdd/data-model.md` — Pure sample pipeline / Voxel objects and ruin; `docs/tdd/rendering.md` — Material-aware terrain extraction / Scene structure
   Issue: Render-artifact ownership for voxel objects is undefined and the stated contracts currently imply duplicate geometry. Objects and the ruin are applied into the composite voxel sample (`data-model.md:51-61`), near terrain extraction meshes solid density from that composite query (`rendering.md:39-50`), and separate object/ruin meshes are also rendered under `ObjectRoot` (`rendering.md:15-27`, `rendering.md:62`, `rendering.md:72-76`). Moreover, an edit delta stores only `VoxelValue`, not an artifact/object owner (`data-model.md:156-175`), while the plan says only the edited object instance gets a unique mesh (`data-model.md:130`). The TDD therefore does not say which extractor owns a surface once a sphere crosses terrain and an object, or how it avoids double-rendering unedited trees and the ruin.
   Evidence: The design requires trees, bushes, boulders, stumps, rocks, and the ruin to remain registered material objects while still participating in one authoritative material world (`design-document.md:38-42`, `design-document.md:123-146`). The TDD's current composite-meshing plus separate-instancing paths do not establish a one-surface/one-artifact invariant.
   Required change: Clarify the render partition and edit-attribution contract. Define the exact filtered sample/ownership rule used by terrain versus object extraction, how dig/place deltas crossing object/terrain boundaries are attributed and dirtied, how seams are owned, and when an edited instance returns to its shared base mesh. Add testable invariants that every visible solid boundary is emitted exactly once and cross-boundary edit/revert cases produce neither gaps nor duplicates.

2. Severity: major
   Section: `docs/tdd/api.md` — Queries
   Issue: The public bulk queries are unbounded eager allocations. `overlap_aabb` and `sphere_contacts` return `Vec` and accept arbitrary in-bounds extents/radii (`api.md:27-46`), while an inactive-space miss procedurally samples truth rather than rejecting an unloaded region. A consumer can therefore request the entire 16,384,000,000-voxel region and force an infeasible traversal/allocation. The only documented errors are `OutOfBounds`, `WorldNotReady`, and malformed-input `InvalidInput`; there is no volume/result budget or streaming/paging contract.
   Evidence: `config.md:19` defines 4000 x 1024 x 4000 voxel bounds. The design makes these queries the supported reusable consumer boundary and requires sparse idle wilderness and memory traffic control (`design-document.md:42`, `design-document.md:63`, `design-document.md:245-246`). Internal collision happens to issue local swept-AABB queries, but the public contract does not enforce that locality.
   Required change: Clarify contract by adding exact maximum scanned-volume/result limits with a typed `QueryTooLarge`/capacity error, or replace eager vectors with a bounded visitor/iterator/paged API. Specify cache-allocation behavior and add whole-region/adversarial-query tests proving bounded time and memory. Apply the same rule to both AABB and sphere contact enumeration.

3. Severity: major
   Section: `docs/tdd/config.md` — MutationConfig; `docs/tdd/api.md` — Mutation commands
   Issue: Dig semantics contradict the TDD's immutable-water boundary. The density formula subtracts from `old_density` without checking material phase (`config.md:70`), so a dig sphere overlapping full-density water creates fluid deltas or canonical air. The next paragraph says generated water is immutable (`config.md:72`), the API says there is no fluid edit command (`api.md:114`), and rendering relies on water itself being non-editable (`rendering.md:68`).
   Evidence: The product includes static water and explicitly excludes draining/fluid simulation (`design-document.md:250`). An operation that deletes water would create precisely an unsupported static hole/drain behavior and make persistence semantics dependent on an unstated exception.
   Required change: Align behavior by defining phase-specific dig inputs and outputs (most consistently: only `Solid` samples lose density and `Fluid`/`Empty` remain byte-identical), including changed-voxel/event counting and mixed solid/water spheres. Add property and persistence tests proving no water delta can be created by Product One commands. If editable water is intended instead, surface it as an explicit Design Divergence and specify its static-hole behavior.

4. Severity: major
   Section: `docs/tdd/benchmarks.md` — Scripted flythrough / Carve-storm scenario
   Issue: The benchmark plan does not satisfy the explicit per-scenario metric-completeness requirement. The flythrough accepts no mutation command and creates no canonical delta save (`benchmarks.md:45-58`), so it cannot produce a measured mutation-to-surface latency or meaningful save-size sample. Only carve storm defines those measurements (`benchmarks.md:60-76`). A report schema containing empty arrays/nulls is not a run outputting every listed metric.
   Evidence: The design states both that each run reports frame rate, mutation latency, cold start, graphics memory, and save size (`design-document.md:69-71`) and that both flythrough and carve-storm scenarios output every listed metric plus a machine profile (`design-document.md:236`). The TDD declares no divergence from this behavior (`overview.md:11`).
   Required change: Align behavior by specifying a real, reproducible source for every required metric in each scenario report (for example, a bounded representative mutation and canonical save phase in flythrough as well), with non-null schema and pass/fail rules; or explicitly surface the suite-level-only interpretation as a Design Divergence requiring product approval.

5. Severity: major
   Section: `docs/tdd/systems.md` — System sets / Mutation pipeline; `docs/tdd/config.md` — StreamingConfig
   Issue: Job priority alone cannot establish the hard two-presented-frame mutation deadline. The plan gives mutation jobs queue priority (`systems.md:15`) but does not cap already-running normal jobs, bound their worker duration, reserve compute capacity, or provide cancellation/preemption. A carve arriving while the Bevy compute pool is saturated can wait behind non-preemptible streaming work. On the apply side, mutation commits bypass the normal upload cap (`config.md:53`) while still claiming a 4 ms main-thread ceiling, with no maximum affected-artifact count or two-frame admission schedule.
   Evidence: The design makes two-frame completion and no hitch acceptance requirements (`design-document.md:61`, `design-document.md:176`, `design-document.md:217`, `design-document.md:232-233`). `systems.md:58-70` restates the outcome but supplies no resource-capacity invariant that makes it achievable under concurrent streaming.
   Required change: Clarify the scheduling contract with bounded normal work in flight and per-job cost, reserved/cancellable worker capacity or another concrete anti-starvation mechanism, plus an exact worst-case affected-artifact/upload bound and frame-by-frame apply admission that respects both two frames and 4 ms. Add a release acceptance case that issues the boundary-aligned 3 m carve while normal near/mid/far queues are saturated.

6. Severity: major
   Section: `docs/tdd/config.md` — GenerationConfig / Camera and lighting; `docs/tdd/api.md` — Plugin and configuration
   Issue: The configuration specification claims all product parameters are typed and defaulted but leaves multiple behavior-bearing parameters opaque or unresolved. `GenerationConfig` exposes nested parameter types without their fields/defaults and defers the seed/noise/strata/hydrology/species values to later tuning (`config.md:21-39`); the player-light intensity is literally `acceptance-tuned portable point-light value` rather than a default (`config.md:117-129`); and public `PersistenceConfig`/`WorldRenderConfig` have no complete field/type/default schemas (`api.md:7-18`). Other contracts reference a configured dressing upward-normal threshold (`systems.md:108`) and checked-in terrain crease threshold (`rendering.md:46`) that are absent from the config specification.
   Evidence: The design makes the stable seed/curated generation parameters part of the World Seed entity (`design-document.md:75-78`) and the lighting, eligible upward surfaces, visual material treatment, save, and benchmark fingerprint observable acceptance behavior. These placeholders cannot be instantiated or tested from the TDD alone.
   Required change: Clarify contract by enumerating the fields, types, units, validity ranges, and Product One defaults for every public/nested config and checked-in behavior threshold, including the fixed seed and codec/render/dressing/light values. Values may remain benchmark-tunable, but the initial acceptance preset must be concrete and fingerprinted rather than described as future tuning.

### Questions For Coder

1. Is the intended object partition to exclude samples carrying a base `ObjectId` from terrain extraction and assign deltas within object bounds to that object, or is a different ownership rule intended? Please state the rule explicitly because place edits in adjacent empty cells and sphere edits crossing the object/terrain boundary make either simple rule incomplete.
2. Do you intend the product requirement that both benchmark scenarios output every metric literally, or do you intend a suite-level interpretation? The latter needs to be recorded as a Design Divergence rather than left implicit.

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

The crate/demo separation is justified by the two explicit deliverables and is consistent with the Bevy scaffold guidance. FixedUpdate/input separation, feature-plugin organization, root assets, Cargo profiles, headless/fixed-time testing, persistence canonicalization, and the concrete AGENTS.md command/naming/module rules are all adequately covered. No dependency cycle was found in the stated plugin/data-flow graph; the unresolved object extraction issue is an ownership ambiguity, not a compile-time dependency cycle.

### Approval

Not approved.

## Coder Turn — 2026-07-13T14:43:41Z

Mode: continue
Responding to: 2026-07-13T14:30:58Z

### Responses To Findings

1. Addressed. `data-model.md` now defines immutable base-provenance ownership per lattice sample, one composite topology extraction, deterministic primitive partitioning, cross-owner dirty detection, shared-object return conditions, and explicit dig/place/revert/seam invariants. `rendering.md` now consumes that single partitioned extraction, so terrain and object paths cannot independently emit the same boundary.
2. Addressed. `api.md` and `config.md` now impose a 262,144-cell scan cap and 65,536-result cap on both eager bulk methods, add typed `QueryTooLarge` and `ResultCapacityExceeded` errors, forbid activation/dense/persistent-cache allocation, and specify whole-region/adversarial tests.
3. Addressed. Dig is now solid-only; fluid and empty samples remain byte-identical. Place also preserves fluid samples. Changed-voxel counts, no-op events, dirty sets, delta creation, properties, and persistence tests now use those phase rules.
4. Addressed. Both successful scenario reports now have a non-optional complete metrics schema. Flythrough performs one named representative public carve and saves its canonical one-carve slot; carve storm retains 512 latency samples and its canonical save. Each scenario has independent metric sources and pass/fail rules.
5. Addressed. Normal work is bounded by worker/job/sample caps and cooperative cancellation. Mutation uses an exclusive two-thread pool, bounded 27-brick/125-dependency-brick work, two preallocated 48 MiB commit packets, a 224-range table, a 12 ms preparation deadline, and an exact F/F+1/F+2 atomic admission schedule. The release gate now saturates normal queues before a boundary/corner-aligned object-intersecting 3 m carve.
6. Addressed. `config.md` now enumerates concrete fixed seed, nested terrain/geology/hydrology/biome/object/constraint values, query/streaming/mutation/player/camera-light/render/persistence fields, types, units, ranges, and defaults. It fixes the light at 2200 lm, dressing normal Y at 0.82, terrain crease angle at 38 degrees, save codec options, and the BLAKE3 fingerprint encoding/domain tags. `api.md` points plugin construction to these exhaustive schemas.

### Answers To Auditor

1. The rule is different from assigning all coordinates in an object AABB to that object. `SurfaceOwner(coord)` is determined by immutable base provenance: a coordinate whose winning `sample_base` solid has an `ObjectId` is object-owned; every other coordinate is terrain-owned. A delta never changes that attribution. Thus refill of a dug object coordinate stays object-owned, while placement into adjacent base air is terrain-owned. Extraction first creates exterior crossings from the full composite world and only then partitions each primitive by the solid-side owner; solid/solid owner interfaces emit no exterior cap. A cross-boundary sphere dirties the union of before/after crossing owners.
2. I intend the requirement literally. Both flythrough and carve-storm now output their own non-null frame rate, mutation latency, cold start, graphics memory, canonical save size, and machine profile, and each passes or fails independently.

### TDD Changes Made

- Revised `api.md`, `config.md`, `data-model.md`, `rendering.md`, `systems.md`, `benchmarks.md`, `persistence.md`, and `overview.md`.
- Added testable ownership, query-capacity, water-immutability, saturated-scheduler, and per-scenario report contracts.
- Replaced deferred/opaque acceptance settings with a concrete fingerprinted Product One preset.

### Questions For Auditor

None.

### Notes To Auditor

The object render partition is history-independent and does not add owner bytes to persistence: the same seed/config/coordinate reproduces owner attribution. Object uniqueness is based on a changed extraction stencil, which also covers a neighboring terrain edit that changes an object boundary; exact stencil reversion restores the shared base handle.

### Explicit Divergences

None. The benchmark requirement is implemented literally, and water remains immutable as implied by the Product One static-fluid boundary.

## Auditor Turn — 2026-07-13T14:48:52Z

Mode: continue
Responding to: 2026-07-13T14:43:41Z

### Prior Findings Status

1. Status: resolved
   Prior finding: Render-artifact ownership for voxel objects was undefined and implied duplicate terrain/object geometry.
   Evidence: `docs/tdd/data-model.md` — “Surface ownership and object render lifecycle” now derives one composite crossing set, assigns each primitive through immutable base provenance, unions before/after owners for dirtying, and defines exact-stencil return to a shared handle. `docs/tdd/rendering.md` — “Material-aware terrain extraction” consumes that one topology pass and adds one-for-one partition, cross-owner edit/revert, and seam properties.
   Notes: The revised rule answers both prior questions: adjacent base-air placement is terrain-owned, refill of a base object coordinate remains object-owned, and terrain/object partitions cannot independently emit the same exterior primitive.

2. Status: resolved
   Prior finding: Public eager bulk queries allowed unbounded whole-region traversal and allocation.
   Evidence: `docs/tdd/api.md` — “Queries” preflights both AABB and sphere enclosing volumes against 262,144 cells, caps results at 65,536, returns typed capacity errors with no partial vector, and forbids activation/dense/persistent-cache allocation. `docs/tdd/config.md` — `QueryConfig` supplies exact defaults and ranges, and the API lists adversarial whole-region tests.
   Notes: Time, result memory, scratch memory, and inactive-space cache behavior are now bounded and testable for both affected methods.

3. Status: resolved
   Prior finding: Dig semantics could mutate immutable generated water.
   Evidence: `docs/tdd/config.md` — `MutationConfig` makes dig solid-only and preserves fluid/empty bytes; place also preserves fluid. `docs/tdd/api.md` excludes unchanged fluid/empty samples from counts, deltas, and invalidation. `docs/tdd/persistence.md` adds mixed-phase no-water-delta round trips.
   Notes: The phase rule is consistent through command results, dirtying, properties, and persistence, so Product One cannot create an unsupported drained/static water hole.

4. Status: resolved
   Prior finding: The flythrough did not produce its own mutation-latency and canonical-save metrics.
   Evidence: `docs/tdd/benchmarks.md` — “Scripted flythrough” now performs the named public 3 m carve and a completed public one-carve save; “Per-scenario completeness and pass rules” identifies independent non-null sources and pass rules for every required metric in both scenarios.
   Notes: This implements the design requirement literally rather than relying on a suite-level interpretation.

5. Status: resolved
   Prior finding: Priority alone did not make the two-presented-frame mutation deadline achievable under saturated normal work.
   Evidence: `docs/tdd/config.md` — `StreamingConfig` now bounds normal workers/jobs/samples/cancellation, reserves a two-thread mutation pool, and fixes packet/range/byte ceilings. `docs/tdd/systems.md` — “Mutation pipeline” provides the F/F+1/F+2 atomic admission schedule and compiled extraction bound. Its release scheduling test saturates normal near/mid/far work and enables raw view before a boundary/corner/object/dressing carve.
   Notes: The anti-starvation, worst-case affected-artifact, admission, failure, and acceptance-test contracts requested by the prior finding are now explicit.

6. Status: partially_resolved
   Prior finding: Behavior-bearing configuration fields/defaults and their acceptance fingerprint were incomplete.
   Evidence: `docs/tdd/config.md` now gives concrete field/type/default/range tables for generation, query, streaming, mutation, player, camera/light, render, and persistence. However its fingerprint section defines only `GENERATION`, `MATERIALS`, `RENDER`, and `PERSISTENCE` domain tags and never defines the “configuration fingerprint” that `docs/tdd/config.md` (opening paragraph), `docs/tdd/benchmarks.md` — “Machine profile”/“Baselines and comparison,” and `docs/tdd/overview.md` — downloadable bundle require. Query, streaming, mutation, player, and camera/light settings therefore have no stated fingerprint membership. In addition, `docs/tdd/data-model.md` — “Pure sample pipeline” makes procedural voxel templates part of base truth, and `docs/tdd/assets.md` stores them in `vegetation_templates.ron`, but `GenerationConfig` has no vegetation-template reference/content digest (only the ruin stamp does), while `docs/tdd/persistence.md` validates only generation/material fingerprints before applying deltas.
   Notes: The concrete schemas resolve the placeholder/default portion, but reproducibility and save compatibility remain underspecified. Define an exact aggregate acceptance-config fingerprint (domain tag, field composition/order, and inclusion of all report-affecting configs), and include every base-truth asset—especially `vegetation_templates.ron`—in the generation/save-compatibility fingerprint. Then align the benchmark metadata, bundle metadata, boot checks, and save validation names with those exact digests.

### New Findings

No new findings.

### Questions For Coder

None.

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

The ownership, bounded-query, water, benchmark-completeness, and saturated-scheduler changes are acknowledged as resolved. Approval is held only on the remaining fingerprint membership/compatibility portion of prior finding 6; no redesign is requested.

### Approval

Not approved.

## Auditor Turn — 2026-07-13T14:51:58Z

Mode: continue
Responding to: 2026-07-13T14:48:52Z

### Prior Findings Status

1. Status: resolved
   Prior finding: Render-artifact ownership for voxel objects was undefined and implied duplicate terrain/object geometry.
   Evidence: `docs/tdd/data-model.md` — “Surface ownership and object render lifecycle” defines one composite crossing set, immutable base-provenance ownership, deterministic primitive partitioning, cross-owner dirtying, and exact-stencil return to shared handles; `docs/tdd/rendering.md` — “Material-aware terrain extraction” consumes that partition and states the one-for-one primitive property.
   Notes: No regression was found in the ownership, seam, edit, or revert contracts.

2. Status: resolved
   Prior finding: Public eager bulk queries allowed unbounded whole-region traversal and allocation.
   Evidence: `docs/tdd/api.md` — “Queries” preflights both bulk methods against `QueryConfig::max_bulk_scan_cells`, caps result allocation, returns typed capacity errors, and specifies adversarial whole-region tests; `docs/tdd/config.md` — `QueryConfig` fixes the Product One limits.
   Notes: Both scanned work and eager result memory remain bounded without activating persistent caches.

3. Status: resolved
   Prior finding: Dig semantics could mutate immutable generated water.
   Evidence: `docs/tdd/config.md` — `MutationConfig` preserves fluid and empty dig samples byte-for-byte; `docs/tdd/api.md` — “Mutation commands” excludes unchanged fluid samples from deltas, counts, and invalidation; `docs/tdd/persistence.md` includes mixed-phase no-water-delta tests.
   Notes: Mutation and persistence continue to enforce the static-water scope boundary consistently.

4. Status: resolved
   Prior finding: The flythrough did not produce its own mutation-latency and canonical-save metrics.
   Evidence: `docs/tdd/benchmarks.md` — “Scripted flythrough” performs a named public carve and canonical one-carve save, while “Per-scenario completeness and pass rules” requires an independent non-null source for every metric in each scenario.
   Notes: The literal per-scenario evidence requirement remains implementable and testable.

5. Status: resolved
   Prior finding: Priority alone did not make the two-presented-frame mutation deadline achievable under saturated normal work.
   Evidence: `docs/tdd/config.md` — `StreamingConfig` bounds normal work and reserves mutation workers/packets; `docs/tdd/systems.md` — “Mutation pipeline” fixes extraction bounds, the F/F+1/F+2 admission schedule, and a saturated-queue release test.
   Notes: No scheduling regression was found in the revised cross-file contract.

6. Status: unresolved
   Prior finding: Behavior-bearing configuration defaults were made concrete, but fingerprint membership and base-asset save compatibility remained incomplete.
   Evidence: `docs/tdd/config.md:95` still defines only generation, materials, render, and persistence fingerprint domains and does not define the aggregate “configuration fingerprint” required by `docs/tdd/config.md:3`, `docs/tdd/benchmarks.md:47,104`, and `docs/tdd/overview.md:41`. `docs/tdd/assets.md:32` still makes `vegetation_templates.ron` determine voxel-template parameters used by the base sampler, but `GenerationConfig` at `docs/tdd/config.md:23-34` references only the ruin asset digest and `docs/tdd/persistence.md:16-18,28,43` validates only generation/material fingerprints before applying deltas.
   Notes: No TDD file changed after the preceding auditor turn, so the identified omission remains. A changed vegetation template can change `sample_base` while preserving the save envelope fingerprints, allowing absolute deltas to be applied to a different procedural base. Separately, benchmark comparability cannot implement the named aggregate config comparison without exact membership and encoding.

### New Findings

No new findings.

### Questions For Coder

1. Will `vegetation_templates.ron` be incorporated into the generation fingerprint, or will the save envelope gain a separate base-truth asset fingerprint? Either is viable, but the selected digest must be defined and validated before deltas are applied.

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

The review remains converged on one narrow contract fix. Define the aggregate acceptance-config fingerprint’s domain tag and ordered membership across report-affecting configs, include every asset that can change `sample_base` in save compatibility (at minimum the vegetation templates and ruin stamp), and align the same digest names across boot, bundle metadata, benchmark reports/comparison, and persistence validation.

### Approval

Not approved.

## Coder Turn — 2026-07-13T14:57:33Z

Mode: continue
Responding to: 2026-07-13T14:51:58Z

### Responses To Findings

6. Addressed. `config.md` now gives `GenerationConfig` an exact-path/exact-byte `VegetationTemplateRef`, includes it and the ruin stamp in the normatively ordered `generation_fingerprint`, defines every component digest and the exact `MORIA-P1-ACCEPTANCE-CONFIG\0` aggregate order, and specifies the canonical scalar/sequence encoding. `persistence.md` now stores and pre-validates the exact generation, material, and persistence digests before base sampling or delta application. The five already-resolved findings remain unchanged.

### Answers To Auditor

1. `vegetation_templates.ron` is incorporated directly into `generation_fingerprint`. The canonical generation stream contains its normalized logical path and the BLAKE3-256 digest of its exact loaded bytes, as it already does for the ruin stamp. Boot verifies both asset hashes before manifest/base construction, and load compares the resulting generation digest before sampling or applying any saved delta.

### TDD Changes Made

- Added `GenerationConfig.vegetation_templates` with concrete path/hash fields and made both vegetation and ruin exact-byte hashes base-truth inputs.
- Added exact domain tags, field membership/order, canonical encoding, golden/boot checks, and the ordered `acceptance_config_fingerprint` composition across generation, materials, query, streaming, mutation, player, camera/light, render, persistence, and acceptance limits.
- Added `persistence_fingerprint` to the save envelope and made generation/material/persistence validation precede record decoding, base sampling, and delta application; added byte-change rejection fixtures for both base-truth assets.
- Aligned digest names and uses across asset boot, `BaseRegion`, worker results, bundle verification metadata, benchmark reports/comparison, and persistence.

### Questions For Auditor

None.

### Notes To Auditor

The aggregate intentionally governs benchmark comparability, while saves validate only truth/codec compatibility. Thus query/render/player tuning changes `acceptance_config_fingerprint` without unnecessarily changing base truth; any generator, vegetation-template, ruin-stamp, material-truth, or persistence-codec change rejects an incompatible save.

## Auditor Turn — 2026-07-13T15:01:09Z

Mode: continue
Responding to: 2026-07-13T14:57:33Z

### Prior Findings Status

1. Status: resolved
   Prior finding: Render-artifact ownership for voxel objects was undefined and implied duplicate terrain/object geometry.
   Evidence: `docs/tdd/data-model.md` — “Surface ownership and object render lifecycle” still defines one composite topology extraction, immutable base-provenance ownership, deterministic primitive partitioning, cross-owner dirtying, and exact-stencil return to shared handles; `docs/tdd/rendering.md` — “Material-aware terrain extraction” retains the one-for-one partition property.
   Notes: The latest fingerprint changes do not regress the ownership, seam, edit, or revert contracts.

2. Status: resolved
   Prior finding: Public eager bulk queries allowed unbounded whole-region traversal and allocation.
   Evidence: `docs/tdd/api.md` — “Queries” still preflights both eager methods against `QueryConfig::max_bulk_scan_cells`, caps result allocation, returns typed capacity errors, and prohibits persistent cache activation; `docs/tdd/config.md` keeps the exact query limits.
   Notes: Both scanned work and eager result memory remain bounded.

3. Status: resolved
   Prior finding: Dig semantics could mutate immutable generated water.
   Evidence: `docs/tdd/config.md` — `MutationConfig` preserves fluid/empty samples byte-for-byte; `docs/tdd/api.md` excludes unchanged fluid samples from deltas, counts, and invalidation; `docs/tdd/persistence.md` retains mixed-phase no-water-delta tests.
   Notes: The static-water scope boundary remains consistent through mutation and persistence.

4. Status: resolved
   Prior finding: The flythrough did not produce its own mutation-latency and canonical-save metrics.
   Evidence: `docs/tdd/benchmarks.md` — “Scripted flythrough” still performs the named public carve and canonical one-carve save, and “Per-scenario completeness and pass rules” requires an independent non-null source for every metric.
   Notes: Both benchmark scenarios continue to satisfy the literal per-scenario requirement.

5. Status: resolved
   Prior finding: Priority alone did not make the two-presented-frame mutation deadline achievable under saturated normal work.
   Evidence: `docs/tdd/config.md` — `StreamingConfig` retains bounded normal work and exclusive mutation capacity; `docs/tdd/systems.md` — “Mutation pipeline” retains the extraction bounds, F/F+1/F+2 admission schedule, and saturated-queue release test.
   Notes: No scheduling regression was introduced.

6. Status: resolved
   Prior finding: Behavior-bearing configuration defaults were concrete, but aggregate fingerprint membership and base-asset save compatibility were incomplete.
   Evidence: `docs/tdd/config.md:91-130` adds `VegetationTemplateRef`, includes the exact vegetation-template and ruin-stamp path/byte digests in normatively ordered `generation_fingerprint`, defines all component domain tags and canonical encoding, and defines `acceptance_config_fingerprint` with exact ordered membership. `docs/tdd/persistence.md:16-29,43-44,63` stores and validates generation/material/persistence digests before base sampling or delta application and tests one-byte changes to both base-truth assets. `docs/tdd/assets.md:32,40`, `docs/tdd/overview.md:41`, `docs/tdd/benchmarks.md:47,104`, and `docs/tdd/systems.md:23` use the same digest names and boot/comparison rules.
   Notes: The requested aggregate, both base-truth asset digests, save envelope, boot ordering, bundle metadata, benchmark metadata, and compatibility validation are now aligned. The separate generator-algorithm revision issue below became visible only once this exact registry and its compatibility claims were defined.

### New Findings

1. Severity: major
   Origin: emergent
   Section: `docs/tdd/config.md` — “Fingerprint registry and compatibility”; `docs/tdd/persistence.md` — “Fixed Product One envelope”
   Issue: Save compatibility still cannot detect a change to the procedural base-sampler algorithm when its serialized config and asset bytes remain unchanged. The revised TDD explicitly says a “generator” change rejects a save, and persistence says compatibility requires the exact matching Product One build/preset, but `generation_fingerprint` contains only coordinate constants, generation field values, and the two asset path/digests. It contains no stable generator/base-sampler schema or algorithm revision, and the save envelope contains no build revision. A code change to terrain noise evaluation, precedence, object-template interpretation, or another `sample_base` rule can therefore produce a different base with the same three stored compatibility fingerprints, causing absolute deltas to be accepted against the wrong truth.
   Evidence: `docs/tdd/config.md:104` exhaustively defines `generation_fingerprint` membership without a generator revision, while `docs/tdd/config.md:130` claims a generator change rejects the save. `docs/tdd/data-model.md:51-61` makes procedural code and precedence part of `sample_base`, and `docs/tdd/persistence.md:29,44` relies only on generation/material/persistence fingerprints to reject a different base before applying deltas.
   Required change: Clarify contract by adding a stable `base_sampler_revision`/generator-schema identifier to the normative `generation_fingerprint` membership (or an equivalent exact generator-compatibility digest stored and prevalidated by the save envelope), requiring it to change whenever any code/schema semantics can change `sample_base`. Align boot, bundle, cache/worker, benchmark, and persistence names as applicable, and add a fixture proving that changing only this revision rejects a save before base sampling or delta application.

### Questions For Coder

1. Will the compatibility key be an explicit stable `base_sampler_revision` inside `generation_fingerprint`, or a separate generator-compatibility digest? Either is implementable; it must cover semantic code/schema changes that do not alter config or asset bytes.

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

The prior fingerprint finding is acknowledged as resolved exactly as requested. Approval is now held only on the newly exposed generator-algorithm compatibility gap; the narrowest fix is a fingerprinted base-sampler revision constant plus a pre-base-sampling rejection test.

### Approval

Not approved.

## Coder Turn — 2026-07-13T15:03:19Z

Mode: continue
Responding to: 2026-07-13T15:01:09Z

### Responses To Findings

1. Addressed. `config.md` now defines compiled `BASE_SAMPLER_REVISION = 1` as the first canonical member of `generation_fingerprint`, specifies exactly which sampler code/schema semantics require a bump, and states that all existing digest consumers inherit it. `data-model.md` binds the pure sampler value and provenance semantics to that revision. `persistence.md` adds the required revision-only incompatibility fixture and zero-sampling/zero-application assertions.

### Answers To Auditor

1. The compatibility key is an explicit stable `BASE_SAMPLER_REVISION` inside `generation_fingerprint`. It is not separately serialized because the save envelope already stores and pre-validates that digest. The compiled revision covers all code and schema behavior that can change `sample_base` value or provenance with otherwise identical inputs.

### TDD Changes Made

- Defined `BASE_SAMPLER_REVISION: u32 = 1`, its semantic coverage, mandatory bump rule, and normative first position in the generation digest.
- Added a revision-headed base-sampler conformance corpus contract so an unrevisioned value/provenance change fails CI.
- Clarified that boot, bundle metadata, cache keys, worker results, benchmark component/aggregate digests, and save validation all receive the revision through `generation_fingerprint`.
- Added a revision-only fingerprint fixture and a load fixture proving rejection occurs before any base-sampler call or delta-store application.

### Questions For Auditor

None.

### Notes To Auditor

This keeps one compatibility authority: generator semantics, generator inputs, and base-truth assets all feed `generation_fingerprint`; codec compatibility remains independently governed by `persistence_fingerprint`. A byte/provenance-preserving refactor need not invalidate saves, but any observable sampler semantic change must increment the revision.

## Auditor Turn — 2026-07-13T15:06:05Z

Mode: approved
Responding to: 2026-07-13T15:03:19Z

### Prior Findings Status

1. Status: resolved
   Prior finding: Render-artifact ownership for voxel objects was undefined and implied duplicate terrain/object geometry.
   Evidence: `docs/tdd/data-model.md:134-140` retains the single composite topology pass, deterministic owner partition, cross-owner dirtying, and shared-handle revert contract; `docs/tdd/rendering.md:39-54` consumes the same ownership rule and requires the owner partitions to equal the unpartitioned output one-for-one.
   Notes: No ownership, seam, mutation, or revert regression was introduced by the sampler-revision change.

2. Status: resolved
   Prior finding: Public eager bulk queries allowed unbounded whole-region traversal and allocation.
   Evidence: `docs/tdd/api.md:48-50` still preflights eager scans, caps returned results and scratch allocation, rejects whole-region requests without sampling, and prohibits activation or persistent-cache population; `docs/tdd/config.md:138-146` retains the concrete limits.
   Notes: Query work and result memory remain bounded.

3. Status: resolved
   Prior finding: Dig semantics could mutate immutable generated water.
   Evidence: `docs/tdd/api.md:103-118` keeps fluid and empty dig samples byte-identical and excludes them from deltas, counts, and invalidation; `docs/tdd/systems.md:74-82` retains the fluid-immutability property; `docs/tdd/persistence.md:57-63` retains the mixed-phase round-trip fixture.
   Notes: Static generated water remains immutable through mutation and persistence.

4. Status: resolved
   Prior finding: The flythrough did not produce its own mutation-latency and canonical-save metrics.
   Evidence: `docs/tdd/benchmarks.md:63-75` still requires the flythrough's public radius-3 m carve and completed public one-carve save, and `docs/tdd/benchmarks.md:93-100` requires independent non-null metric sources and pass rules for both scenarios.
   Notes: Each benchmark scenario remains independently complete.

5. Status: resolved
   Prior finding: Priority alone did not make the two-presented-frame mutation deadline achievable under saturated normal work.
   Evidence: `docs/tdd/systems.md:56-82` retains the exclusive two-worker pool, bounded extraction/upload artifacts, atomic F/F+1/F+2 schedule, and hard release invariants; `docs/tdd/systems.md:146` retains the saturated normal-queue acceptance test.
   Notes: The latest compatibility edits do not alter or weaken the anti-starvation contract.

6. Status: resolved
   Prior finding: Behavior-bearing configuration defaults were concrete, but aggregate fingerprint membership and base-asset save compatibility were incomplete.
   Evidence: `docs/tdd/config.md:98-134` retains canonical encoding, exact component memberships, the ordered aggregate, both base-truth asset digests, boot verification, and aligned digest consumers; `docs/tdd/persistence.md:13-29,41-47` stores and prevalidates the truth/codec digests before base sampling or delta application.
   Notes: Aggregate benchmark comparability and asset-sensitive save compatibility remain fully specified.

7. Status: resolved
   Prior finding: Save compatibility could not detect procedural base-sampler code/schema changes that preserved serialized config and asset bytes.
   Evidence: `docs/tdd/config.md:102-106` defines compiled `BASE_SAMPLER_REVISION = 1`, its semantic coverage and mandatory bump rule, and makes it the first normative member of `generation_fingerprint`; `docs/tdd/config.md:130-134` propagates that digest to every consumer and specifies conformance plus revision-only compatibility tests. `docs/tdd/data-model.md:47-63` binds base values and provenance/metadata semantics to the revision. `docs/tdd/persistence.md:29,44,63` prevalidates the resulting generation digest and requires a revision-only fixture to reject with zero sampler calls and zero delta applications.
   Notes: The selected explicit revision inside the existing generation digest closes the algorithm-compatibility gap without creating a competing compatibility authority.

### New Findings

No new findings.

### Questions For Coder

None.

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

The fingerprint revision fix is acknowledged. The revised TDD is implementable, internally consistent, testable, and faithful to the design document; no blocking or major gaps remain.

### Approval

Approved.

All issues resolved. TDD approved.
