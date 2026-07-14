## Auditor Turn — 2026-07-14T06:38:16Z

Mode: continue

Responding to: none

### Prior Findings Status

No prior ledger existed, so there are no prior findings to resolve.

### New Findings

1. **Asset-pair check failed — the manifest uses family bundles rather than one scaffold/acquire/wire triple per individual asset, omits triples for authored/config assets, and has no final production-asset audit.**

   Evidence:

   - `docs/tdd/assets.md:9-50` identifies individual runtime assets. The visual inventory alone contains four world textures, six tree LOD GLBs, three understory GLBs, one Horizon atlas, three prop GLBs, and one explorer GLB.
   - `M-007`, `M-008`, `M-009`, and `M-011` each scaffold several assets in one issue. `M-065`/`M-066`, `M-067`/`M-068`, `M-069`/`M-070`, and `M-073`/`M-074` repeat the same bundling in acquisition and wire-in. This directly violates “one issue per individual asset per phase.” For example, `M-008` produces all six birch/pine LOD files and `M-067` acquires those six files together.
   - The inventory's authored/config assets (`product_one_region.ron`, `curated_manifest.ron`, `presentation.ron`, `input.ron`, `materials.ron`, `ruin_p1.ron`, four WGSL files, and the two asset manifests) do not have scaffold/acquire/wire triples. The sparse ruin stamp is especially material: `docs/tdd/assets.md:83-107` calls it the sole hand-authored world shape and includes it in world identity, but `M-044` directly implements the final asset.
   - Wire issues `M-068`, `M-070`, `M-072`, and `M-074` do not depend on the completed placeholder-backed gameplay slice `M-060`; they depend only on a subsystem (`M-050`, `M-051`, or `M-034`) plus acquisition. Only `M-066` and `M-076` name `M-064`/`M-060` respectively.
   - Every acquisition issue cites “Pipeline initial wiring audit” as an input, but no manifest ID produces that audit and no `depends_on` edge can enforce it.
   - `M-064` runs capture/acceptance automation before all acquisitions. There is no issue after `M-066`, `M-068`, `M-070`, `M-072`, `M-074`, and `M-076` that reruns asset validation, benchmarks, and the visual checklist with production assets. This also conflicts with `implementation-plan.md`'s `B1` ordering, where full release evidence follows final `V1` assets.

   Required changes:

   - Replace each bundled asset phase with one issue per file. At minimum instantiate the following three-issue template once for each listed slug:

     ```yaml
     id: ASSET-SCAFFOLD-<slug>
     title: "[setup] Scaffold <runtime path> placeholder"
     labels: [setup, auto]
     depends_on: [M-001]
     body: "Create the loader entry, stable ID/path, format-faithful placeholder, budget/license manifest entry, and a test proving the app runs through the final runtime path. Acceptance: placeholder has the exact per-file format, dimensions/bounds/origin/LOD role required by assets.md; fallback is observable; stable ID is immutable."

     id: ASSET-ACQUIRE-<slug>
     title: "[asset] Acquire <runtime path>"
     labels: [asset, manual]
     depends_on: [ASSET-SCAFFOLD-<slug>, ASSET-PLACEHOLDER-WIRING-AUDIT]
     body: "Acquire exactly this one production asset without replacing the runtime placeholder. Acceptance: verify its individual format/dimensions or bounds/origin, portability/budget constraints, source/author/license/modifications, redistribution permission, and checksum."

     id: ASSET-WIRE-<slug>
     title: "[integration] Wire acquired <runtime path>"
     labels: [integration, auto]
     depends_on: [ASSET-ACQUIRE-<slug>, M-060]
     body: "Install exactly this one acquired file at its existing stable path/ID, update factual provenance and budget metadata, validate readability/format/runtime loading, and rerun the relevant placeholder-backed behavior without changing identity."
     ```

     Required visual slugs are: `terrain-albedo`, `terrain-normal`, `terrain-orm`, `water-normal`, `birch-near`, `birch-mid`, `birch-far`, `pine-near`, `pine-mid`, `pine-far`, `bush-near`, `bush-far`, `grass-cluster`, `tree-horizon-cards`, `boulder`, `stump`, `rock`, and `explorer`. Apply the same per-file treatment to the authored/config/template inventory or explicitly replace the current design with a reviewed asset classification; the current manifest cannot silently exempt it.

   - Add these concrete issues:

     ```yaml
     id: ASSET-PLACEHOLDER-WIRING-AUDIT
     title: "[integration] Audit placeholder asset wiring before acquisition"
     labels: [integration, auto]
     depends_on: [M-013, M-037, M-060, M-064]
     body: "Audit every stable asset ID from loader declaration through placeholder-backed runtime use. Acceptance: all individual scaffold issues are complete; no undeclared path or per-instance asset exists; automated placeholder gameplay and capture verification pass; emit a checked audit artifact that acquisition issues can depend on."

     id: M-077
     title: "[integration] Audit final production-asset integration"
     labels: [integration, assisted]
     depends_on: [ASSET-WIRE-terrain-albedo, ASSET-WIRE-terrain-normal, ASSET-WIRE-terrain-orm, ASSET-WIRE-water-normal, ASSET-WIRE-birch-near, ASSET-WIRE-birch-mid, ASSET-WIRE-birch-far, ASSET-WIRE-pine-near, ASSET-WIRE-pine-mid, ASSET-WIRE-pine-far, ASSET-WIRE-bush-near, ASSET-WIRE-bush-far, ASSET-WIRE-grass-cluster, ASSET-WIRE-tree-horizon-cards, ASSET-WIRE-boulder, ASSET-WIRE-stump, ASSET-WIRE-rock, ASSET-WIRE-explorer]
     body: "Run release asset validation, headed shader loading, flythrough, carve storm, milestone captures, and the rendering visual checklist with production assets and zero fallbacks. Acceptance: every production path is readable/licensed/in budget; stable IDs are unchanged; final reports and captures identify the production asset manifest digest; no pre-acquisition evidence is presented as final evidence."
     ```

2. **Dependency/wave check failed — manifest prerequisites disagree with issue Inputs, setup work is ordered after implementation, and unrelated asset scaffolds are serialized into six extra waves.**

   Evidence:

   - Graph validation itself passes: all 76 IDs exist, there are no cycles, and `M-001` is the only dependency-free issue.
   - `M-007` is category `setup` but depends on `M-003` (`config`) and `M-004` (`component`), contrary to the rule that setup issues have no dependencies or only setup dependencies. Manual acquisition issues `M-065`, `M-067`, `M-069`, `M-071`, `M-073`, and `M-075` are also category `setup` while depending on integration issue `M-064`; these are asset gates, not project setup.
   - The asset setup chain is artificial: `M-008 -> M-007`, `M-009 -> M-008`, `M-010 -> M-009`, `M-011 -> M-010`, `M-012 -> M-011`. Birch scaffolding is not a prerequisite for bush scaffolding, and bush scaffolding is not a prerequisite for the Horizon atlas. The computed graph consequently has 35 waves, with these six independent placeholder families spread across waves 4–9.
   - The body explicitly names issue inputs that are absent from `depends_on`: `M-008` needs `M-005`; `M-011` needs `M-005`; `M-012` needs `M-003`; `M-020` needs `M-005`; `M-025` needs `M-003`; `M-029` needs `M-019`; `M-042` needs `M-003`; and `M-055` needs `M-003`. Acquisition/wire issues similarly name their scaffold issues as inputs but omit the edge (`M-065`/`M-066`→`M-007`, `M-067`→`M-008`, `M-069`→`M-009`, `M-071`→`M-010`, `M-073`→`M-011`, and `M-075`→`M-012` and `M-060`). Text in `Inputs` is not a schedulable prerequisite.

   Required changes:

   - Put project setup (`M-001`, test harness, shared asset-manifest/ID infrastructure) in the first waves. Make individual asset scaffold issues siblings after that shared setup, not a serial family chain.
   - Relabel manual acquisitions from `setup` to an asset/manual category and depend on the explicit placeholder wiring audit.
   - Make every listed Inputs ID a direct `depends_on` edge or remove the claim and show why it is not required. Direct dependency correctness matters even when a transitive path happens to exist.
   - Recompute waves after the asset and high-risk splits; do not preserve the current 35-wave critical path.

3. **Integration-wiring check failed — the “complete generated world” integration omits systems it claims to compose, and two designed renderer boundaries have no explicit integration deliverable.**

   Evidence:

   - `M-046` says it composes generation, store, query, streaming, terrain, water, object, dressing, presentation, and telemetry behind the facade, but its direct dependencies omit public read/query issues `M-020`–`M-023`, object activation `M-033`, portable materials `M-037`, render installation/accounting `M-038`, and edit readiness `M-039`. Transitive availability does not meet the integration rule requiring an integration issue to depend on both sides it wires.
   - The TDD requires character visual interpolation and animation selection (`rendering.md`, “Player, camera, and cave lighting”). `M-055` mentions fixed-pose interpolation in its description but produces only camera/light files and none of its acceptance criteria tests player visual spawning, fixed-pose interpolation, or mapping `Grounded/Airborne/Paddling` to `Idle/Run/Sprint/Jump/Fall/Paddle`. `M-076` only validates this after final art arrives, leaving placeholder-backed gameplay without an implementable owner.
   - `assets.md` requires headed pipeline compilation on Metal and Vulkan. `M-037` only requires shader parse/asset tests; no integration issue owns headed pipeline smoke, fallback-fatal behavior on both backends, or the DirectX-class wgpu validation evidence.

   Required dependency correction for `M-046`: either add direct dependencies on every system named in its composition (`M-020`, `M-023`, `M-033`, `M-037`, `M-038`, and `M-039` at minimum), or split it into boundary-specific integrations such as generation↔store/read, streaming↔extraction/install, and mutation↔render-readiness, then make the facade issue depend on those integrations.

   Draft missing issues:

   ```yaml
   id: M-078
   title: "[ui] Render and animate the placeholder-backed player visual"
   labels: [ui, auto]
   depends_on: [M-012, M-013, M-053, M-054]
   body: "Create CharacterVisual under PlayerRoot, interpolate rendered transforms from fixed poses, and select/blend Idle/Run/Sprint/Jump/Fall/Paddle from locomotion and velocity while collision remains capsule-authoritative. Acceptance: headless presentation-state tests cover every mapping and zero/multiple fixed-tick frames; missing clips use only the declared observable development fallback; no mesh/collider state feeds world truth."

   id: M-079
   title: "[integration] Smoke-test portable headed shader pipelines"
   labels: [integration, assisted]
   depends_on: [M-013, M-037, M-038, M-046]
   body: "Compile and render the terrain, water, vegetation, and raw-voxel material pipelines in headed smoke scenes on Metal and Vulkan acceptance machines, with DirectX-class compatibility checked through wgpu validation/CI parsing. Acceptance: all bindings and 32-bit constraints validate; every required pipeline draws; shader fallback is fatal in release evidence; artifacts record adapter/backend/build/asset digest."
   ```

4. **High-risk size check failed — several issues combine multiple async/state-machine/concurrency/failure categories and would plausibly become large, multi-cycle PRs despite having only three domain criteria.**

   Rejected issues and why:

   - `M-039` combines barrier state, all terrain/water/object/Horizon/dressing keys, render extraction, GPU create/free, render queue acknowledgement, exactly-once semantics, a 13-stage trace ring, zero-fixed-tick/cutoff timing, stale results, queue saturation, and eviction pinning. This is the SafeShelf-style concentration of async state, concurrency, failure/terminal behavior, and output validation.
   - `M-040` combines four independently difficult deliverables: public query-cost harness, signature carve, maximum-candidate active-Horizon stress carve, stage/barrier report validation, plus an assisted hardware gate run. A failure in any one forces a broad revise cycle.
   - `M-047` combines exact codec/checksum semantics, revision snapshots, async compression/disk I/O, single-operation concurrency, flush/rename durability, terminal results, and prior-file preservation on failure.
   - `M-048` combines async decode, hostile-input validation, transaction FSM, preaccepted edit draining, rejection of concurrent edit/save/load, a fixed-tick atomic swap, coherent reads, and failure rollback.
   - `M-061` combines CLI parsing/exit codes, a ten-state benchmark FSM, watchdog failure behavior, frame/allocation/streaming capture, machine profiling, and atomic report writing.
   - `M-063` combines a 128-step edit state machine, request/readiness validation, traversal checks, workload construction, async save, and a second-app exact round trip.

   Required split chains (draft issue definitions; replace the rejected umbrella issues and rewire downstream IDs):

   ```yaml
   - id: M-039A
     title: "[component] Define edit readiness barriers and required keys"
     labels: [component, auto]
     depends_on: [M-029, M-030, M-032, M-034, M-035, M-036]
     body: "Define revisioned barrier items and pure exact-once accounting for terrain, seams, water, object roots, Horizon aggregates/derived/tombstones, and dressing. Test duplicate, missing, empty, and stale-revision keys without the render sub-app."
   - id: M-039B
     title: "[integration] Bridge render extraction, GPU prepare/free, and queue acknowledgements"
     labels: [integration, auto]
     depends_on: [M-038, M-039A]
     body: "Carry each barrier item through Bevy extraction, GPU create/write/free, and draw/removal queue acknowledgement. Test current-revision creates, replacements, empty removals, and stale acknowledgement rejection."
   - id: M-039C
     title: "[state] Enforce edit deadlines, pins, and terminal readiness"
     labels: [state, auto]
     depends_on: [M-026, M-027, M-039B]
     body: "Emit exactly one EditSurfaceReady only after barrier completion and preserve submit+2/commit+2 behavior for zero-tick and after-cutoff frames. Test admission rejection, queue saturation, superseded tasks, and pinned eviction separately."
   - id: M-039D
     title: "[system] Record and validate the fixed feasibility trace"
     labels: [system, auto]
     depends_on: [M-039A, M-039B]
     body: "Implement the fixed 256-entry, allocation-free trace and validate all 13 named stages including explicit zero-work records; do not own deadline state."

   - id: M-040A
     title: "[system] Implement the bounded public query-cost probe"
     labels: [system, auto]
     depends_on: [M-006, M-018, M-021, M-022, M-023]
     body: "Run the exact cold/max/normal query bundles, reject repeated cold coordinates, capture work counters/distributions, and validate each independent limit."
   - id: M-040B
     title: "[integration] Run the clean signature-carve feasibility scenario"
     labels: [integration, auto]
     depends_on: [M-018, M-024, M-028, M-032, M-035, M-039C, M-039D, M-040A]
     body: "Open a clean normal world and prove public submit, full barrier, two-frame/33.3 ms timing, current presentation, and capsule traversability for the signature target."
   - id: M-040C
     title: "[integration] Run the maximum-candidate active-Horizon stress scenario"
     labels: [integration, auto]
     depends_on: [M-034, M-040B]
     body: "Open a separate clean world at F1's stress target, force the affected cell into Horizon, and prove card exclusion, derived/tombstone ownership, dependency timing, full acknowledgements, and stale-result rejection."
   - id: M-040D
     title: "[integration] Produce and approve the immutable F2 artifact"
     labels: [integration, assisted]
     depends_on: [M-040C]
     body: "Run release M4/Metal 2560x1440 with the exact F1/build/world/manifest digest, validate both scenario artifacts plus query evidence, write the immutable report, and block downstream work on any failure."

   - id: M-047A
     title: "[system] Encode and decode the deterministic delta format"
     labels: [system, auto]
     depends_on: [M-019, M-029]
     body: "Implement sorted base-relative layout, checksum, zstd level 3, bounds/count checks, and deterministic codec tests with no filesystem or task-pool behavior."
   - id: M-047B
     title: "[system] Snapshot deltas at a fixed revision"
     labels: [system, auto]
     depends_on: [M-047A]
     body: "Capture a sorted immutable request-revision snapshot at a fixed boundary; prove later edits cannot alter it and base-equal/derived data is absent."
   - id: M-047C
     title: "[state] Write the single save slot asynchronously and atomically"
     labels: [state, auto]
     depends_on: [M-024, M-047B]
     body: "Own one-operation admission, off-thread encode/write, sibling temp flush/rename, exact byte reporting, and terminal failure behavior. Test busy, write/flush/rename failure, and preservation of the prior slot."
   - id: M-048A
     title: "[system] Decode and validate a staged load"
     labels: [system, auto]
     depends_on: [M-047A]
     body: "Off-thread decode into an isolated map and reject every documented magic/identity/bounds/order/material/checksum/compression fault without touching live truth."
   - id: M-048B
     title: "[state] Gate edits and disk requests during load"
     labels: [state, auto]
     depends_on: [M-027, M-047C, M-048A]
     body: "Implement Idle→Staging→SwapPending admission, drain preaccepted edits, reject later edits and concurrent save/load with typed results, and return pre-swap failures to Idle."
   - id: M-048C
     title: "[state] Swap staged deltas atomically at a fixed boundary"
     labels: [state, auto]
     depends_on: [M-019, M-029, M-048B]
     body: "Swap the complete validated map once, increment revision once, expose only coherent pre/post reads, and hand the new revision to rebuilding; post-swap failure is fatal only."

   - id: M-061A
     title: "[system] Implement strict benchmark CLI and atomic report output"
     labels: [system, auto]
     depends_on: [M-001, M-006]
     body: "Own argument validation, scenario selection, output paths, exit-code mapping, complete/null JSON serialization, flush/rename, and human summary without owning runtime states or metrics."
   - id: M-061B
     title: "[system] Capture benchmark timing, allocation, streaming, and machine metrics"
     labels: [system, auto]
     depends_on: [M-006, M-038]
     body: "Capture each contracted metric and identity with explicit unsupported metadata; validate distributions and profile IDs independently of scenario progression."
   - id: M-061C
     title: "[state] Implement benchmark lifecycle and watchdog"
     labels: [state, auto]
     depends_on: [M-049, M-061A, M-061B]
     body: "Implement legal benchmark-state transitions, exact 300-frame warmup, explicit event gates, watchdog failure context, and no fabricated values."
   - id: M-063A
     title: "[system] Drive and validate the 128 carve-storm edits"
     labels: [system, auto]
     depends_on: [M-060, M-061C, M-039C]
     body: "Own deterministic targets, one-open-request progression, commit/readiness/inside-outside checks, signature traversal, latency samples, and final workload minima; do not save or reload."
   - id: M-063B
     title: "[integration] Save the heavy carve-storm world"
     labels: [integration, auto]
     depends_on: [M-047C, M-063A]
     body: "Request the public save after workload validation and verify terminal bytes/counts and the 50 MB contract without opening a second app."
   - id: M-063C
     title: "[integration] Prove exact round trip in a second public consumer"
     labels: [integration, auto]
     depends_on: [M-049, M-048C, M-063B]
     body: "Open a presentation-disabled second app, load through public messages, compare every delta voxel plus deterministic unedited samples and identity, reject derived bytes, and append round-trip evidence."
   ```

5. **TDD/final-acceptance coverage failed — no issue can satisfy the resident graphics-memory acceptance dependency.**

   Evidence:

   - `docs/tdd/implementation-plan.md`, “Final acceptance dependency,” says `B1` cannot report overall acceptance without reviewed resident measurement below 2,097,152,000 bytes on every named acceptance configuration or an explicit Product-approved estimate-substitution ID.
   - `M-006` correctly prevents the ledger from claiming this target, and `M-064` correctly keeps acceptance false when evidence is absent. No later issue obtains the measurement or approval. Therefore the issue list can finish with Product One still necessarily failing its own final acceptance.

   Draft missing issue:

   ```yaml
   id: M-080
   title: "[integration] Resolve resident graphics-memory acceptance"
   labels: [integration, assisted]
   depends_on: [M-077, M-062, M-063C]
   body: "Run the final production-asset flythrough and carve-storm on every named M4 and Linux acceptance configuration under a reviewed process-resident graphics-memory provider, attach provider/scope/sampling/artifact hashes, and validate peak <2,097,152,000 bytes. If measurement is unavailable, obtain and record an explicit Product-approved estimate-substitution ID; do not set product_target_proven from the application ledger. Acceptance requires final reports to carry valid evidence or the approval ID and to preserve separate machine/profile/resolution identities."
   ```

### Questions For Coder

1. Is “Pipeline initial wiring audit” intended to be a mechanically injected issue? If yes, provide the exact injected ID/contract and make acquisition dependencies machine-readable; otherwise add `ASSET-PLACEHOLDER-WIRING-AUDIT`.
2. Will authored/config assets in `assets.md` be decomposed into per-file triples, or is there a reviewed rule that classifies some inventory entries as implementation outputs rather than assets? The current issue list contains no such exception.
3. Which resident-memory path will close final acceptance: measurement artifacts on every named configuration, or an explicit Product-approved estimate-substitution ID?

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

- The AGENTS.md scaffolding check passes: `M-001` is first-wave setup, produces repository-root `AGENTS.md`, and requires all commands/naming/public-boundary/schedule/feasibility rules from `overview.md`.
- Basic graph integrity passes: there are no cycles or dangling references, and every non-setup issue has at least one dependency. Those passes do not cure the missing direct prerequisites and incorrect setup categorization above.
- No reverse TDD filename reference was found; the coverage failures are missing deliverables/contracts, not references to nonexistent design files.

### Approval

Not approved.

## Coder Turn — 2026-07-14T06:52:14Z

Mode: continue

Responding to: 2026-07-14T06:38:16Z

### Responses To Findings

1. Addressed the asset-lifecycle finding. `M-008`–`M-037` now scaffold one exact runtime file each; `M-110`–`M-139` acquire one file each under `assets/incoming/<manifest-id>/`; and `M-140`–`M-169` wire one acquired file each while preserving its stable path/ID. This covers all 30 inventory entries, including the authored configuration, generated-manifest template/output, material registry, ruin stamp, four shaders, and two asset manifests. `M-109` is the explicit placeholder wiring audit, and `M-170` is the final production-asset audit. `M-170` reruns digest-matched F1/F2 plus the final flythrough, carve, shader, capture, and visual evidence so an authoritative config/stamp replacement cannot reuse stale pre-acquisition proof.

2. Addressed the dependency/wave finding. `M-001`–`M-003` establish workspace, tests, and shared asset-ID infrastructure; every individual scaffold is an independent sibling after `M-003`; setup issues depend only on setup issues; and every issue-ID named in an Inputs section exactly matches its direct `depends_on` array. Acquisitions and wire-ins are each parallel waves. The graph now has 45 dependency levels because the required edit/readiness, feasibility, persistence, benchmark, and final-acceptance risk chains were split; the former six-wave artificial asset-family serialization is gone.

3. Addressed the missing integration wiring. `M-077` wires complete generation to storage/public reads, `M-078` wires streaming to extraction/installation, `M-079` wires mutation through renderer readiness, and `M-080` composes the public plugin facade with direct dependencies on the relevant boundaries. `M-094` owns placeholder-backed CharacterVisual spawning, fixed-pose interpolation, and all six locomotion mappings. `M-100` owns headed Metal/Vulkan shader pipeline smoke plus DirectX-class wgpu validation evidence.

4. Addressed the high-risk size finding. The former edit acknowledgement/gate work is split across `M-064`–`M-071`; save/load codec, snapshot, disk failure, hostile decode, transaction admission, atomic swap, and rebuild are split across `M-081`–`M-087`; and benchmark CLI, metrics, lifecycle, flythrough, carve progression, heavy save, and second-app round trip are split across `M-101`–`M-107`. Each issue has three domain criteria plus the two standard test/regression criteria.

5. Addressed the resident-memory acceptance gap. `M-171` requires reviewed process-resident measurement artifacts for flythrough and carve storm on M4 Metal at 1080p and 1440p and the designated Linux 3060-class Vulkan machine at 1440p. `M-172` cannot synthesize a passing final result without those artifacts and matching final-production identities.

### Answers To Auditor

1. “Pipeline initial wiring audit” is not mechanically injected. It is now explicit manifest issue `M-109`, and every acquisition has a direct dependency on `M-109` plus its own scaffold.

2. I decomposed authored/config/runtime documents into the same per-file triples rather than relying on an unstated implementation-output exception. The 30 triples include all files listed under `assets/` in `assets.md`.

3. The selected final-acceptance path is reviewed resident-memory measurement on every named configuration. No estimate-substitution ID is assumed or requested by this manifest. If measurement later proves unavailable, Product would need to provide a separate explicit approval ID before the acceptance contract could be revised.

### TDD Changes Made

- Replaced `docs/issues.json` with 172 sequential, dependency-ordered issues.
- Added exact per-file placeholder specifications, acquisition inbox/provenance/checksum contracts, wire-in validation, explicit initial/final asset audits, and post-wire F1/F2 reruns.
- Added boundary-specific world integrations, player presentation/animation ownership, headed shader portability evidence, and final resident-memory/final-acceptance closure.
- Added failure-mode slices for renderer acknowledgement, deadline/pinning, feasibility scenarios, save/load transactions, and benchmark orchestration.
- Renumbered issues after the initial setup nodes so newly inserted prerequisites appear before their consumers; retaining the prior downstream numbers would have violated sequential IDs and dependency order.

### Questions For Auditor

1. The allowed manifest category list has no `asset` category, so acquisition issues use `component` + `manual`. Does that schema-compliant category satisfy your request to move them out of `setup`?

### Notes To Auditor

- Automated validation reports 172 sequential IDs, no missing or forward dependencies, no cycles, exactly five acceptance criteria per issue, and exact parity between issue-ID Inputs and `depends_on`.
- The asset audit reports 30 scaffolds, 30 manual acquisitions, and 30 automated wire-ins. All acquisitions depend on `M-109`; all wire-ins depend on their acquisition and completed placeholder gameplay `M-099`; `M-170` depends directly on all 30 wire-ins.
- `M-171` deliberately makes the measured path concrete: both scenarios on all three named machine/resolution configurations, each below 2,097,152,000 bytes with provider/scope/sampling/path/hash metadata.

### Explicit Divergences

- I did not introduce the auditor-drafted `asset` label/category because the governing manifest schema enumerates only setup, config, component, system, state, integration, ui, verification, and bug. Manual acquisitions are labeled `component`/`manual`; their titles and bodies still identify the exact acquired asset and lifecycle phase. No Product/TDD behavior was weakened.

## Auditor Turn — 2026-07-14T06:58:42Z

Mode: continue

Responding to: 2026-07-14T06:52:14Z

### Prior Findings Status

1. **Asset lifecycle triples — partially_resolved.** The manifest now has a numerically complete 30-by-3 lifecycle: `M-008`–`M-037` scaffold, `M-110`–`M-139` acquire, and `M-140`–`M-169` wire. `M-109` is the explicit placeholder wiring audit, every acquisition follows it, every wire-in follows `M-099`, and `M-170` is a distinct final production-asset audit. However, the two registry assets are acquired before the production files whose provenance/budgets they must enumerate, and the wire-in issues concurrently rewrite those registries. Finding 2 below keeps this prior finding open.

2. **Dependency and wave correctness — partially_resolved.** Mechanical graph checks now pass: 172 unique sequential IDs, no dangling references, no cycles, no forward dependencies, setup nodes depend only on setup nodes, every non-setup issue has a dependency, and issue-ID Inputs exactly match direct `depends_on`. The graph has 45 dependency depths. The claimed scaffold and wire-in parallel waves are not actually independent because sibling issues declare the same output files; see Finding 1.

3. **Missing integration wiring — resolved.** `M-077` depends on complete public reads and curation; `M-078` depends on materialization plus object/Horizon/install systems; `M-079` depends on public generation/read and streaming integrations plus deadline readiness; and `M-080` composes the facade from those boundaries. `M-094` explicitly owns placeholder player visual spawning/interpolation/all six locomotion mappings, and `M-100` owns headed Metal/Vulkan plus DirectX-class shader validation.

4. **Oversized high-risk async/state issues — resolved.** The prior renderer-readiness umbrella is split into `M-064`–`M-071`, persistence into `M-081`–`M-087`, and benchmark orchestration into `M-101`–`M-107`. The resulting issues separate data shape, happy-path processing, stale/concurrency protection, failure terminals, and integration evidence with concrete dependencies.

5. **Resident graphics-memory acceptance — resolved.** `M-171` requires qualifying resident measurements for both scenarios on M4/Metal at 1080p and 1440p and Linux 3060-class/Vulkan at 1440p, including provider/scope/sampling/path/hash metadata and the strict byte threshold. `M-172` depends on it and forbids an overall pass for missing evidence.

### New Findings

1. **Wave structure failed — the two large asset waves contain direct shared-file writers, so their issues are not independent.**

   Evidence:

   - All 30 scaffold issues `M-008`–`M-037` are at dependency depth 3. Every one declares `crates/moria-world/src/presentation/asset_ids.rs` in `Produces` and says it will “extend the stable loader declaration.” Parallel agents therefore edit the same Rust file in the same wave. This contradicts the coder note that they are independent siblings and creates order-dependent merge/overwrite behavior.
   - All 30 wire-in issues `M-140`–`M-169` are at dependency depth 41. Every one declares both `assets/manifests/asset_licenses.ron` and `assets/manifests/asset_budgets.ron` as outputs (including `M-168` and `M-169`, which each claim both registry paths). Parallel execution therefore has 30 writers for each registry. No dependency chooses the authoritative last writer, and a clean wave cannot be partitioned from the present graph.

   Required issue changes:

   - Change `M-003` so it predeclares the complete fixed set of 30 stable IDs/paths and creates either all per-ID module stubs or another isolated per-asset declaration mechanism. Change each `M-008`–`M-037` to produce only its unique placeholder and unique validation fixture/module; remove the shared `asset_ids.rs` output and shared-file mutation from every sibling. Each scaffold must still test its exact loader declaration and runtime placeholder path.
   - Change content wire-ins so each owns only its one stable runtime asset path. They may validate the installed license/budget entries, but they must not rewrite either shared registry.
   - Give `M-168` exclusive ownership of `assets/manifests/asset_licenses.ron` and `M-169` exclusive ownership of `assets/manifests/asset_budgets.ron`; neither may claim the other's path. Renumber/reorder as needed so the resulting issue IDs remain sequential and dependency-ordered.

2. **Asset-pair causal ordering failed — the production license and budget registries can be acquired without any production asset acquisition, so their promised factual contents are unknowable.**

   Evidence:

   - `M-138` depends only on its placeholder `M-036` and placeholder audit `M-109`, yet its acceptance requires a canonical registry containing each asset's source, author, license/text, modifications, and redistribution permission. Those facts are supplied by `M-110`–`M-137`, none of which is a prerequisite.
   - `M-139` likewise depends only on `M-037` and `M-109`, although its per-file texture/mesh/clip/triangle/portability entries must describe the acquired production bytes.
   - `M-168`/`M-169` then install these prematurely authored registry files in the same wave where `M-140`–`M-167` replace the corresponding production assets. Their criteria say metadata is current, but the dependency graph provides no artifact from which that can be established.

   Required dependency chain and issue changes (no additional lifecycle issue is required):

   ```text
   M-110..M-137 acquire the 28 non-registry production files in parallel
       -> revised license-registry acquisition (current M-138)
       -> revised budget-registry acquisition (current M-139; may be parallel with the license registry)
       -> two independent registry wire-ins, one path each
       -> 28 independent content wire-ins, each one path and read-only registry validation
       -> M-170 final asset-integration audit
   ```

   `M-138` and `M-139` must directly depend on all acquisitions whose bytes/provenance they enumerate and must validate that every acquired stable ID appears exactly once with matching SHA-256/factual metadata. The two registry wire-ins must depend on those completed acquisitions plus `M-099`. Every content wire-in must depend on its own acquisition, `M-099`, and both installed registries, while producing only its own runtime path. This preserves one asset per phase, keeps the human gate after placeholder gameplay, and yields collision-free parallel content wire-ins.

### Questions For Coder

1. Will you make `M-003` own the complete immutable declaration table, or use isolated per-asset declaration modules/fixtures? Either is acceptable if `M-008`–`M-037` no longer share a writable output.
2. Will you order the two registry acquisitions after the 28 content acquisitions and install the registries before the 28 content wire-ins, as the dependency chain above requires?

### Answers To Coder

1. Yes. `component` + `manual` is schema-compliant and satisfies the acquisition-phase classification requirement. The lifecycle rule requires a `manual` or `assisted` tag; it does not require a nonexistent `asset` category. The remaining objection is dependency/output ownership, not the category label.

### Notes To Coder

- The AGENTS.md check passes: first-wave `M-001` produces repository-root `AGENTS.md` and covers the exact command, naming, schedule, boundary, portability, and gate rules from `overview.md`.
- Apart from the asset-wave ownership defects above, the dependency graph is acyclic, complete, navigable, and naturally layered; all TDD filenames are referenced by at least one issue and no issue cites a nonexistent TDD file.
- The revised issue set now covers the previously missing public facade boundaries, player presentation, portable headed shaders, final asset audit, and resident-memory closure. No additional non-asset TDD coverage gap was found in this pass.

### Approval

Not approved.

## Coder Turn — 2026-07-14T07:03:37Z

Mode: continue

Responding to: 2026-07-14T06:58:42Z

### Responses To Findings

1. Addressed the shared-writer wave finding. `M-003` now owns the complete immutable 30-ID/path declaration table. Each of `M-008`–`M-037` produces only its unique runtime placeholder plus a uniquely named validation fixture and explicitly may not edit the shared table or either registry. `M-140` and `M-141` are the exclusive one-path registry installers, while `M-142`–`M-169` each produce only one content runtime path and treat both registries as read-only inputs.

2. Addressed the registry causal-ordering finding. The 28 content acquisitions `M-110`–`M-137` remain parallel after `M-109`. Both registry acquisitions `M-138` and `M-139` directly depend on all 28 and must match every acquired stable ID and SHA-256 exactly once. Registry wire-ins `M-140`/`M-141` follow those acquisitions and placeholder gameplay `M-099`; every content wire-in then directly depends on its own acquisition, `M-099`, and both installed registries. `M-170` remains the final all-asset audit after all 30 wire-ins.

### Answers To Auditor

1. I chose the complete immutable declaration-table option. `M-003` predeclares all 30 stable IDs/paths and fixture hooks; the parallel scaffold issues no longer share a writable output.

2. Yes. The manifest now implements the requested order: 28 content acquisitions, then two registry acquisitions in parallel, then two independent registry installs, then 28 independent content wire-ins with read-only registry validation.

3. For the harness-carried resident-memory question, the selected final-acceptance path remains reviewed measurement artifacts on every named configuration. `M-171` requires both scenarios on all named M4/Metal and Linux/Vulkan configurations; no estimate-substitution approval is assumed.

### TDD Changes Made

- Revised `M-003` and all 30 scaffold bodies to establish single-writer ownership and unique per-file fixtures.
- Revised `M-138`/`M-139` dependencies and criteria so factual production registries are derived from completed content acquisitions.
- Reordered and renumbered the wire-in block: registry installs are now `M-140`/`M-141`, and the 28 content installs are `M-142`–`M-169`.
- Added direct machine-readable registry dependencies and exclusive output contracts to every affected issue.

### Questions For Auditor

None. The latest dependency and output-ownership requirements were unambiguous.

### Notes To Auditor

- Validation reports 172 sequential IDs, no missing or forward dependencies, no cycles, five acceptance criteria per issue, and exact parity between direct dependencies, `Depends on`, and issue-ID Inputs (with the dependency-free setup issue handled as `none`).
- The graph now has 47 dependency depths: scaffold siblings share no outputs at depth 4; content acquisitions are parallel at depth 41; registry acquisitions are parallel at depth 42; registry installs are independent at depth 43; and content wire-ins share no outputs at depth 44.
- A same-depth output collision scan found zero collisions. Targeted asset checks found zero shared `asset_ids.rs` outputs, zero missing acquisition-to-registry edges, zero multi-output registry/content wire-ins, and zero content wire-ins missing `M-099`, `M-140`, or `M-141`.

## Auditor Turn — 2026-07-14T07:11:35Z

Mode: continue

Responding to: 2026-07-14T07:03:37Z

### Prior Findings Status

1. **Shared asset writers in parallel waves — resolved.** `M-003` now exclusively owns the immutable 30-ID/path declaration table. Each scaffold `M-008`–`M-037` produces one unique runtime placeholder and one unique fixture and expressly forbids edits to the table/registries. At the production end, `M-140` and `M-141` each own one registry path, and `M-142`–`M-169` each own one content path. A dependency-depth/output scan found zero same-depth path collisions.

2. **Registry acquisition causal ordering — resolved.** Both `M-138` and `M-139` directly depend on all 28 content acquisitions `M-110`–`M-137` and require exact stable-ID/SHA-256 coverage. The two registry installs follow those acquisitions and `M-099`; every content install follows its own acquisition, `M-099`, `M-140`, and `M-141` and treats the registries as read-only. `M-170` follows every wire-in.

### New Findings

1. **Integration wiring and dependency correctness failed — initial activation/readiness is assigned before the systems that can satisfy it, and no later issue owns that boundary.**

   Evidence:

   - `M-049` depends only on `M-038`, `M-044`, and `M-045`, but its first acceptance criterion requires the collision neighborhood, required initial presentation, fallback-ready object handles, and supported spawn before `WorldReady`; its third criterion also requires presentation-disabled registration behavior.
   - The necessary collaborators do not exist at that wave: materialization/eviction is `M-051`, object presentation is `M-058`/`M-059`, water presentation is `M-061`, render installation is `M-063`, and streaming-to-presentation composition is `M-078`.
   - `api.md` makes this a concrete cross-system barrier: enabled readiness waits for initial terrain/water meshes and object handles, while `PresentationConfig.enabled == false` must omit render systems and use a different readiness condition. `M-080` composes plugins but does not explicitly take ownership of this barrier, and the only manifest occurrence of `WorldReady` is `M-049`. Thus the `M-049` agent must either invent unavailable collaborators or leave an implicit stub for later agents.

   Required changes:

   - Reduce `M-049` to the legal `WorldLifecycle` types/transitions, pre-ready query/edit rejection, and typed failure behavior that can be implemented from its actual prerequisites.
   - Insert an explicit integration issue after render installation/object/water systems are available and make the F2 startup scenario and final facade depend on it. It must own initial activation coordination and the enabled/headless readiness barrier.

   Draft insertion (provisional ID; renumber later issues to retain sequential dependency order):

   ```yaml
   id: M-066A
   title: "[integration] Wire initial activation into WorldReady"
   labels: [integration, auto]
   depends_on: [M-049, M-051, M-058, M-061, M-063]
   body: "Coordinate spawn support/collision activation, camera-frustum terrain and water installation, required object fallback handles, and exactly-once WorldReady emission. In presentation-disabled mode, register no render/water/object/dressing systems and omit visual readiness items while retaining truth/query/edit/persistence readiness. Acceptance requires external-consumer tests for enabled and disabled startup, every typed fatal dependency failure, no early query/edit access, and exactly one readiness event. Make the signature-carve startup issue and the final world-facade issue depend directly on this integration."
   ```

2. **Integration wiring failed — the persistence subsystem is never registered into the public `MoriaWorldPlugin` facade.**

   Evidence:

   - `overview.md`, “Plugin composition,” requires `MoriaWorldPlugin` to install `PersistencePlugin` and `api.md` exposes save/load request/result messages plus `WorldTransactionState` through that public library boundary.
   - `M-080` produces `lib.rs`/`plugin.rs` and calls itself the complete facade, but it precedes and has no dependency on `M-081`–`M-087`.
   - `M-081`–`M-086` implement codec/save/load pieces. `M-087` integrates post-load rebuilding with render collaborators, but it produces only `persistence/rebuild.rs` and a rebuild test; it neither produces `persistence/mod.rs`/`plugin.rs` nor requires registration/export/schedule tests. Later demo/benchmark issues merely assume their public save/load messages are live.

   Required change: add a post-`M-087` integration issue that depends on both the facade and the completed save/load boundary, registers `PersistencePlugin`, exports only the documented public messages/state, and proves external consumers receive terminals through normal schedules. Demo load flow, heavy save, round trip, and benchmark lifecycle must depend on this issue rather than on implementation files alone.

   Draft insertion:

   ```yaml
   id: M-087A
   title: "[integration] Wire persistence into the public world plugin"
   labels: [integration, auto]
   depends_on: [M-080, M-083, M-087]
   body: "Create the persistence feature facade and register its save polling, load admission/staging/swap/rebuild, and transaction observability in MoriaWorldPlugin with the TDD schedule ordering. Export exactly SaveWorldRequest/results, LoadWorldRequest/results, and read-only WorldTransactionState—never codec/store internals. Acceptance requires an external-consumer headless app to save, load, observe Busy/Started/one terminal result, and complete enabled and presentation-disabled rebuilds. Update demo/benchmark persistence consumers to depend directly on this integration."
   ```

3. **Integration wiring failed — the benchmark CLI, lifecycle, scenario drivers, and metric capture are never composed into the required benchmark binary/plugins.**

   Evidence:

   - `overview.md` requires `moria-bench` to add `DefaultPlugins`, `MoriaWorldPlugin`, `BenchmarkScenarioPlugin`, and `MetricCapturePlugin`; `benchmarks.md` requires CLI selection to drive exactly one scenario through the documented state machine and atomically report it.
   - `M-101` implements CLI/output, `M-102` metrics, `M-103` lifecycle, `M-104` flythrough, `M-105` carve storm, and `M-106`/`M-107` persistence completion. None produces or updates `crates/moria-bench/src/main.rs`, a benchmark plugin composition module, or an integration test connecting CLI selection to those systems.
   - `M-001` only creates thin package entry points before these modules exist. `M-108` produces milestone/capture outputs and says automation reruns scenarios, but it neither declares the binary/plugin files nor acceptance for scenario/plugin registration. In contrast, demo composition is explicitly owned by `M-099`, which produces `moria-demo/src/main.rs`.

   Required change: insert a benchmark composition issue before milestone automation/final runs and make `M-108` depend on it.

   Draft insertion:

   ```yaml
   id: M-107A
   title: "[integration] Compose the public benchmark application"
   labels: [integration, auto]
   depends_on: [M-080, M-101, M-102, M-103, M-104, M-105, M-106, M-107]
   body: "Wire the thin moria-bench binary, BenchmarkScenarioPlugin, and MetricCapturePlugin so validated CLI selection launches exactly one flythrough or carve-storm path through the documented lifecycle, public world API, save/round-trip orchestration, metric capture, report validation, and exit code. Acceptance requires end-to-end integration tests for both scenarios and argument/runtime failure branches, proves no private moria-world import or privileged reset/store/mesh path, and produces crates/moria-bench/src/main.rs plus focused plugin wiring."
   ```

### Questions For Coder

None. The required boundaries and dependency corrections are explicit in `api.md`, `overview.md`, and `states.md`.

### Answers To Coder

No coder questions were pending. I confirm the chosen immutable declaration table and the registry-first production wire-in order satisfy the prior asset findings.

### Notes To Coder

- Mechanical graph checks otherwise pass: 172 sequential unique IDs, no missing/forward dependencies, no cycles, no non-setup roots, setup-only setup dependencies, and zero same-depth exact-output collisions.
- The 30 scaffold/acquire/wire triples, placeholder audit, final asset audit, and resident-memory closure remain valid after the latest revision.
- `M-001` still satisfies the first-wave `AGENTS.md` requirement, all ten TDD files are referenced, and no issue cites a nonexistent TDD file.
- The new findings are integration ownership gaps, not requests to merge behavior back into the already split high-risk edit/persistence implementations.

### Approval

Not approved.

## Coder Turn — 2026-07-14T07:16:14Z

Mode: continue

Responding to: 2026-07-14T07:11:35Z

### Responses To Findings

1. Addressed the initial-activation ownership finding. `M-049` now owns only legal `WorldLifecycle` transitions, typed failures, and pre-ready read/edit rejection. New `M-064` owns the enabled and presentation-disabled activation barriers, concrete collision/terrain/water/object readiness coordination, typed fatal startup failures, and exactly-once `WorldReady`. The clean signature-carve startup scenario `M-070` and public facade `M-081` both depend directly on `M-064`.

2. Addressed the persistence registration finding. New `M-089` creates the persistence facade, installs `PersistencePlugin` into `MoriaWorldPlugin` in the documented order, limits public exports to the specified request/result messages and read-only transaction state, and tests normal-schedule terminals/rebuilds in enabled and presentation-disabled consumers. Demo flow `M-100`, benchmark lifecycle `M-105`, heavy save `M-108`, and second-app round trip `M-109` now depend directly on `M-089` instead of persistence implementation files alone.

3. Addressed the benchmark composition finding. New `M-110` owns `moria-bench` main/plugin composition and external application-flow tests. It composes `DefaultPlugins`, `MoriaWorldPlugin`, `BenchmarkScenarioPlugin`, and `MetricCapturePlugin`; connects validated CLI selection through lifecycle/scenarios/persistence/metrics/report/exit behavior; and prohibits private world imports. Milestone automation `M-111` now depends directly on this composed binary.

### Answers To Auditor

1. Final resident-memory acceptance will close through reviewed measurement artifacts for both scenarios on every named configuration. The manifest assumes no estimate substitution and no Product-approved estimate-substitution ID; `M-174` requires the measurements and `M-175` cannot pass when any is absent.

### TDD Changes Made

- Revised `docs/issues.json` from 172 to 175 dependency-ordered issues by inserting the three explicit integration boundaries and renumbering all later IDs and asset inbox paths consistently.
- Reduced the lifecycle issue to implementable local state behavior and moved cross-system readiness into its own integration slice.
- Rewired every named persistence and benchmark consumer to the newly completed public boundaries.

### Questions For Auditor

None. The three requested integration boundaries and dependency corrections were explicit.

### Notes To Auditor

- Validation reports 175 unique sequential IDs, no missing or forward dependencies, no cycles, no non-setup roots, exactly five acceptance criteria per issue, and exact parity between direct dependencies, body dependency declarations, and issue-ID Inputs.
- The dependency graph now has 48 depths and still has zero same-depth exact-output collisions.
- The prior 30 asset lifecycle triples remain intact after renumbering: scaffolds `M-008`–`M-037`, content acquisitions beginning at `M-113`, registry-first installation, content wire-ins, final audit `M-173`, resident measurements `M-174`, and final acceptance `M-175`.

## Auditor Turn — 2026-07-14T07:22:35Z

Mode: continue

Responding to: 2026-07-14T07:16:14Z

### Prior Findings Status

1. **Initial activation/readiness ownership — resolved.** `M-049` now contains only lifecycle-local behavior. `M-064` directly depends on lifecycle, materialization, object, water, and render-installation work; it owns enabled/headless readiness and exactly-once `WorldReady`. `M-070` and `M-081` both directly depend on `M-064`.

2. **Persistence registration in the public facade — resolved.** `M-089` creates the persistence facade, updates `plugin.rs`, registers the documented schedule path, restricts exports, and tests enabled/headless external consumers. The named consumers `M-100`, `M-105`, `M-108`, and `M-109` directly depend on `M-089`.

3. **Benchmark application composition — partially_resolved.** `M-110` now owns `moria-bench` main/plugin composition and `M-111` depends on it, so the later production flythrough/carve-storm path has an explicit binary owner. It does not resolve the earlier feasibility-carve path: the manifest schedules and approves F2 at `M-070`–`M-072`, while the public world facade and benchmark executable that acceptance criterion says launch `feasibility-carve` do not exist until `M-081` and `M-110`, both downstream of F2.

### New Findings

1. **Integration wiring and wave correctness failed — Gate F2 cannot be built or run through the required public executable at the wave where the manifest approves it.**

   Evidence:

   - `implementation-plan.md` requires F2 to run exactly as `cargo run --release -p moria-bench -- --scenario feasibility-carve ...`; `overview.md` requires that binary to use `DefaultPlugins`, `MoriaWorldPlugin`, `BenchmarkScenarioPlugin`, and `MetricCapturePlugin` and prohibits private `moria-world` imports.
   - `M-070` and `M-071` produce scenario modules and claim clean public-world runs, and `M-072` claims the release F2 artifact. None depends on a public world-facade issue, benchmark CLI/output `M-103`, metric capture `M-104`, or a benchmark application-composition issue.
   - The first issue that produces/updates `crates/moria-world/src/lib.rs` and `plugin.rs` and exports the non-persistence `MoriaWorldPlugin` facade is `M-081`. It depends on broad-generation `M-077`, which itself depends on passing `M-072`; therefore it cannot be moved in front of F2 without splitting the minimal F2 boundary from broad G1 completion.
   - The first issue that produces `crates/moria-bench/src/main.rs` and composes the required benchmark plugins is `M-110`, at graph depth 38. `M-072` is at depth 22. `M-110` says it launches `feasibility-carve`, but only after F2 has already supposedly run and authorized all downstream work.
   - The generic boundary issues are also only transitively wired. `M-078` claims generation + private storage + public reads but directly depends only on `M-045` and `M-077`, omitting storage `M-044` and the generator systems it composes. `M-079` claims band planning/materialization + terrain/water/object/Horizon/dressing extraction/install/eviction but directly omits `M-050`, `M-056`, `M-057`, `M-060`, and `M-061`. `M-080` claims submission/staging/commit/dirtying/snapshot/install/readiness composition but directly depends only on three downstream umbrellas and none of `M-052`–`M-055` or `M-066`. This violates the rule that an integration issue directly depend on the systems it wires.

   Required change: split out a minimal public F2 integration/facade and a feasibility-only benchmark composition before `M-070`–`M-072`; retain later issues as broad extensions. The following dependency-ordered draft is ready to adapt and renumber:

   ```yaml
   - id: M-064A
     title: "[integration] Wire the feasibility generator, store, and public reads"
     labels: [integration, auto]
     depends_on: [M-043, M-039, M-041, M-042, M-044, M-045, M-046, M-047, M-048]
     body: "Compose the F1-validated minimal terrain/forest generator and object index into private storage and every public F2 read/query path. Acceptance: active/inactive reads match; identity is validated before access; an external consumer reaches all scalar, ray, capsule, focus, diagnostic, and telemetry contracts without private imports."

   - id: M-066A
     title: "[integration] Wire feasibility streaming through render installation"
     labels: [integration, auto]
     depends_on: [M-064A, M-050, M-051, M-056, M-057, M-058, M-059, M-060, M-061, M-062, M-063]
     body: "Compose F2 focus/band planning and materialization with terrain/seams, object/Horizon, dressing, water, shader, installation, and eviction paths. Acceptance: the signature and stress targets install the documented production representations; current token/revision/LOD ownership is unique; eviction preserves truth and decrements every counted resource."

   - id: M-068A
     title: "[integration] Wire feasibility mutation through renderer readiness"
     labels: [integration, auto]
     depends_on: [M-064A, M-066A, M-052, M-053, M-054, M-055, M-065, M-066, M-067, M-068]
     body: "Compose public submission, fixed staging/commit, dependency discovery, snapshots, barrier construction, extraction acknowledgements, deadlines, and trace output for the F2 slice. Acceptance: signature/object/Horizon/dressing/no-op/stale/saturation paths have exact public terminals and no private reset/store/mesh path."

   - id: M-069A
     title: "[integration] Expose the public feasibility world facade"
     labels: [integration, auto]
     depends_on: [M-038, M-049, M-064, M-064A, M-066A, M-068A, M-069]
     body: "Produce the pre-persistence `moria_world::{...}` facade and MoriaWorldPlugin needed by F2, installing the minimal generation/storage/query/mutation/streaming/terrain/object/presentation/telemetry/activation plugins with named schedule ordering. Acceptance: enabled and presentation-disabled external consumers compile and reach readiness; only the documented public types are exported; no broad G1 feature is pulled before F2."

   - id: M-071A
     title: "[integration] Compose the feasibility-carve benchmark application"
     labels: [integration, auto]
     depends_on: [M-069A, M-070, M-071, M-103, M-104]
     body: "Wire the thin moria-bench binary and feasibility-only BenchmarkScenarioPlugin/MetricCapturePlugin path so strict CLI selection runs the signature and stress worlds, query probe, production trace/barrier capture, report validation, atomic output, and exit mapping. Acceptance: the documented cargo command runs end to end through MoriaWorldPlugin; digest/argument/runtime/report failures are tested; no private world import or privileged reset exists."
   ```

   Then make `M-070` and `M-071` directly depend on `M-069A`, make the F2 artifact issue depend on `M-071A`, and make later `M-078`–`M-081` explicit broad extensions of `M-064A`/`M-066A`/`M-068A`/`M-069A`. `M-110` should extend the already working feasibility binary with persistence and the two production scenarios, not be the first issue that makes `feasibility-carve` launchable. While revising, add the direct system dependencies omitted by `M-078`–`M-080` or narrow their descriptions to boundaries already represented by an integration prerequisite.

2. **Wave structure failed — downstream demo/UI work is runnable before the blocking F2 artifact.**

   Evidence:

   - The computed graph has `M-092` at depth 4 and `M-098` at depth 17, while F2 approval `M-072` is at depth 22.
   - `M-092` implements the complete physical input/action layer for movement, camera, debug edits, HUD focus, and UI; `M-098` implements the demo's brick/raw-voxel/streaming rendering. Neither is needed to produce the scripted public F2 gate.
   - `implementation-plan.md` says passing F1/F2 authorizes outward implementation and explicitly blocks traversal polish before the paired gate artifacts pass. These two issues can currently begin during P0/F1/F2 and continue even after a red gate, contrary to that rule.

   Required issue changes:

   ```yaml
   id: M-092
   depends_on: [M-002, M-004, <passing-F2-artifact-issue>]
   change: "Retain the existing body, but make the passing F2 artifact a direct prerequisite."

   id: M-098
   depends_on: [M-048, M-062, M-092, <passing-F2-artifact-issue>]
   change: "Retain the existing body, but directly encode the feasibility gate because this is downstream headed demo presentation, not an F2 harness dependency."
   ```

   Recompute depths after inserting the public F2 composition chain. Shared CLI/report/metric infrastructure may remain pre-gate only because the repaired F2 executable directly consumes it; unrelated demo input and visualizers may not.

### Questions For Coder

None. The required command, public boundary, and gate ordering are explicit in `overview.md` and `implementation-plan.md`.

### Answers To Coder

No coder questions were pending. The three newly inserted issues do exist and the activation and persistence fixes are accepted. The benchmark composition fix is accepted for the later production scenarios but is incomplete for the earlier F2 executable as described above.

### Notes To Coder

- Mechanical validation still passes: 175 unique sequential IDs, one setup root, no missing/forward references, no cycles, exact dependency/body/Input parity, setup-only setup prerequisites, and zero same-depth exact-output collisions.
- The 30 asset scaffold/acquire/wire triples, registry causal order, placeholder audit, final production-asset audit, resident-memory measurement, and final acceptance issues remain structurally valid.
- `M-001` continues to satisfy the first-wave `AGENTS.md` requirement. No new asset or resident-memory finding is being raised.

### Approval

Not approved.

## Coder Turn — 2026-07-14T07:29:30Z

Mode: continue

Responding to: 2026-07-14T07:22:35Z

### Responses To Findings

1. Addressed the unrunnable F2 boundary. New `M-070` composes the F1-validated generator, private store, and all public F2 reads; `M-071` composes the F2 streaming/render path; `M-072` composes public mutation through renderer readiness; and `M-073` exposes the pre-persistence public `MoriaWorldPlugin` facade. Shared CLI/output and metric capture moved to `M-074`/`M-075`, the public signature and stress scenarios are now `M-076`/`M-077` and directly depend on the facade, and new `M-078` produces the actual feasibility-capable benchmark binary/plugin composition. The assisted F2 artifact `M-079` now depends on that runnable executable.

2. Addressed the broad-integration direct-dependency gap. `M-085`–`M-088` are explicitly narrow extensions of the completed F2 generation, streaming, mutation, and facade boundaries, with the complete-route generator issues wired directly where they are composed. `M-115` now extends the already runnable feasibility executable with persistence, flythrough, and carve-storm behavior instead of first making `feasibility-carve` launchable.

3. Addressed the pre-gate demo/UI work. Semantic demo input `M-099` and diagnostic rendering `M-105` both directly depend on the passing F2 artifact `M-079`; their computed depths are 26 and 27, after F2 at depth 25. CLI/report and metric capture remain pre-gate only because the F2 executable directly consumes them.

### Answers To Auditor

1. Final resident-memory acceptance will close through reviewed measurement artifacts for both named scenarios on every named configuration. No estimate-substitution path or Product-approved substitution ID is assumed; `M-179` requires the artifacts, and `M-180` remains false if any artifact is absent or invalid.

### TDD Changes Made

- Revised `docs/issues.json` from 175 to 180 dependency-ordered issues, inserting five focused F2 integration boundaries and renumbering later IDs and asset inbox paths consistently.
- Moved the existing benchmark CLI/output and metric-capture issues before F2 without duplicating their behavior.
- Recast the later public world and benchmark composition issues as broad extensions of the now-runnable feasibility boundaries.
- Added direct passing-gate prerequisites to the two identified demo-only slices.

### Questions For Auditor

None. The required executable, public-boundary, direct-dependency, and wave-order changes were explicit.

### Notes To Auditor

- Validation reports 180 sequential unique IDs, one setup root, no missing or forward dependencies, no cycles, exactly five acceptance criteria per issue, exact dependency/body/Input parity, and references to all ten TDD files.
- The graph has 50 dependency depths and zero same-depth exact-output collisions. The public F2 path is `M-070`–`M-073` → `M-076`/`M-077` → `M-078` → `M-079`.
- The asset lifecycle remains intact after renumbering: placeholder audit `M-117`, acquisitions `M-118`–`M-147`, registry installs `M-148`/`M-149`, content wire-ins `M-150`–`M-177`, final asset audit `M-178`, resident measurements `M-179`, and final acceptance `M-180`.

## Auditor Turn — 2026-07-14T07:36:35Z

Mode: continue

Responding to: 2026-07-14T07:29:30Z

### Prior Findings Status

1. **Runnable public F2 boundary — partially_resolved.** `M-070`–`M-073` now compose the minimal generator/read, streaming/render, mutation/readiness, and public facade boundaries with the direct system prerequisites requested. `M-076`/`M-077` consume that facade, and `M-078` now produces the feasibility-capable `moria-bench` binary before the assisted gate. However, `M-079` still produces `crates/moria-bench/src/scenarios/feasibility_report.rs`; therefore the prior `M-078` executable does not own all code needed to assemble the complete `CarveFeasibilityReport` that its acceptance criteria say it validates and writes.

2. **Pre-gate demo/UI work — resolved.** `M-099` and `M-105` directly depend on passing F2 artifact `M-079` and compute to depths 26 and 27, after `M-079` at depth 25. The shared CLI/output and metric-capture issues are legitimate pre-gate work because `M-078` consumes them.

### New Findings

1. **Wave/gate correctness failed — the manifest never produces the required same-commit F1/F2 approval pair before breadth begins.**

   Evidence:

   - `implementation-plan.md` states that F1 and F2 must come from the same gate-baseline commit, that their passing pair authorizes downstream work, and that stale build/manifest evidence fails validation.
   - `M-043` produces `target/feasibility/forest.json` at depth 10. The production store, queries, edit pipeline, extraction/install path, public facade, F2 scenarios, and executable are then implemented by `M-044`–`M-078`. `M-079` at depth 25 only says to run F2 "against the passing F1 digest" and produces only `carve.json`; it neither reruns F1 nor asserts equal F1/F2 `BuildProfile.git_commit` values.
   - Consequently, the immutable F1 artifact consumed by `M-079` necessarily predates the F2 implementation commit. Matching only the manifest digest is insufficient: the TDD requires the same git commit as well. `M-080`, `M-082`, `M-083`, `M-089`, and `M-099` become runnable immediately after this invalid pair.
   - The later `M-178` final-asset audit reruns F1/F2, but it occurs after all broad implementation, placeholder benchmarks, asset acquisition, and wire-in. It cannot retroactively make the pair at `M-079` authorize `G1`, `S1`, `T1`, and `B1`.

   Required change: retain `M-043` as the early F1 preflight that permits construction of the F2 half of the feasibility wave, but change the assisted issue after the completed F2 executable to rerun **both** commands from one clean source commit and approve them only as an identity-matched pair. Every broad downstream issue must depend on that paired artifact.

   Draft replacement:

   ```yaml
   id: M-079
   title: "[integration] Rerun and approve the paired F1/F2 feasibility artifacts"
   labels: [integration, assisted]
   depends_on: [M-043, M-078]
   body: "From one clean release gate-baseline commit on the named M4, rerun prove-forest and then feasibility-carve without changing source/config/assets between commands. Write new immutable forest and carve artifacts, validate that BuildProfile.git_commit, world/config/manifest/asset identities, machine profile, and the forest-proof SHA consumed by F2 agree exactly, and approve only when both complete reports pass. Preserve the earlier preflight and every failed artifact. Any mismatch or failure blocks all G1/V1/T1/S1/B1 issues. This issue produces evidence only; it changes no Rust source."
   ```

2. **Integration ownership failed — the assisted F2 execution issue still implements a required runtime module.**

   Evidence:

   - `M-078` claims the documented cargo command runs end to end, covers report validation and atomic output, and produces `main.rs`, plugin wiring, and application-flow tests.
   - `M-007` defines/validates evidence representations and `M-074` owns generic CLI/output mechanics, but the F2-specific report assembler `crates/moria-bench/src/scenarios/feasibility_report.rs` is not produced until `M-079`.
   - Thus `M-079` is both a code implementation issue and the assisted measurement/approval gate. Running it changes the source being measured unless that code is committed first, while committing it means `M-078` was not actually the complete runnable executable claimed. It also makes the same-commit defect harder to detect because the F2 run can reference a commit that omits its own report module.

   Required change: move F2 report construction into an automatic issue before application composition, make `M-078` depend on it, and leave the paired assisted gate evidence-only.

   Draft insertion before application composition:

   ```yaml
   id: M-077A
   title: "[system] Assemble and validate carve-feasibility reports"
   labels: [system, auto]
   depends_on: [M-007, M-068, M-069, M-074, M-075, M-076, M-077]
   body: "Produce crates/moria-bench/src/scenarios/feasibility_report.rs. Assemble both clean-world trial records, query-cost evidence, complete trace/barrier evidence, identities, machine/backend/resolution, and the consumed F1 digest into CarveFeasibilityReport; validate before the shared atomic writer and map all incomplete/invalid branches to the documented failure report and exit. Tests cover missing stages, identity/F1 mismatch, non-finite/timing/count failures, explicit optional zero-work branches, and a complete passing report. M-078 must depend on this issue and prove the final binary calls it."
   ```

### Questions For Coder

None. The same-commit requirement and the fact that the assisted gate may not be the first owner of code needed by its executable are explicit from the current issue outputs and `implementation-plan.md`.

### Answers To Coder

No coder questions were pending. I acknowledge that the public feasibility facade, benchmark application composition, direct system prerequisites, and post-F2 demo dependencies requested in the previous turn were added correctly.

### Notes To Coder

- Mechanical validation passes for the current manifest: 180 sequential unique IDs, one setup root, no missing or forward references, no cycles, setup-only setup prerequisites, and a maximum depth of 50.
- The 30 scaffold/acquire/wire triples, placeholder audit ordering, registry-first installation, final asset audit, resident-memory measurements, and final acceptance closure remain valid.
- `M-001` still satisfies the first-wave `AGENTS.md` requirement. No new asset-pair, public-facade, demo-wave, or resident-memory finding is raised.

### Approval

Not approved.

## Coder Turn — 2026-07-14T07:41:53Z

Mode: continue

Responding to: 2026-07-14T07:36:35Z

### Responses To Findings

1. Addressed the missing same-commit approval pair. `M-043` remains the early F1 preflight, while revised assisted `M-080` preserves that preflight and reruns the exact F1 and F2 commands from one unchanged clean gate-baseline commit. It approves only when both reports pass and their `BuildProfile.git_commit`, world/config/manifest/asset identities, machine profile, and consumed forest-proof SHA-256 match exactly. The issue is explicitly evidence-only and retains failed artifacts. Every one of the 101 later issues is now a direct or transitive descendant of this paired gate.

2. Addressed F2 report-assembly ownership. New automatic `M-078` produces and tests `feasibility_report.rs`, including both trials, query evidence, complete stage/barrier evidence, identity validation, truthful failure construction, and pre-write validation. Executable composition moved to `M-079`, directly depends on `M-078`, and proves the final binary invokes the completed assembler. Assisted `M-080` produces only preserved/rerun evidence and changes no Rust source.

### Answers To Auditor

1. Final resident-memory acceptance will use measurement artifacts on every named configuration, not estimate substitution. `M-180` requires reviewed provider evidence for flythrough and carve storm on M4 Metal at 1920x1080 and 2560x1440 and the designated Linux 3060-class Vulkan configuration at 2560x1440. No Product-approved estimate-substitution ID is assumed; `M-181` remains false if any required measurement is missing or invalid.

### TDD Changes Made

- Revised `docs/issues.json` from 180 to 181 dependency-ordered issues by inserting the automatic F2 report-assembly slice and renumbering every later issue and asset inbox reference consistently.
- Replaced the prior code-owning F2 assisted issue with the evidence-only paired F1/F2 rerun and approval issue.
- Extended the feasibility application composition dependency, inputs, description, and acceptance checks to consume the completed report assembler.

### Questions For Auditor

None. The report ownership and same-commit paired-gate requirements were explicit.

### Notes To Auditor

- Validation reports 181 sequential unique IDs, one setup root, no missing or forward dependencies, no cycles, exactly five acceptance criteria per issue, exact dependency/body/Input parity, and references to all ten TDD files.
- The graph has 51 depths, zero same-depth exact-output collisions, and all 101 issues after `M-080` descend from the paired passing gate. The executable gate path is `M-078` report assembly → `M-079` application composition → `M-080` clean same-commit F1/F2 rerun.
- The 30 asset lifecycle triples remain intact after renumbering: placeholder audit `M-118`, acquisitions `M-119`–`M-148`, registry installs `M-149`/`M-150`, content wire-ins `M-151`–`M-178`, final asset audit `M-179`, resident measurements `M-180`, and final acceptance `M-181`.

## Auditor Turn — 2026-07-14T07:46:59Z

Mode: continue

Responding to: 2026-07-14T07:41:53Z

### Prior Findings Status

1. **Same-commit F1/F2 approval pair — resolved.** `M-080` now depends on the code-complete executable `M-079`, preserves the earlier F1 preflight separately, reruns both exact release commands from one unchanged clean commit, compares `BuildProfile.git_commit` plus every world/config/manifest/asset/machine identity and the consumed F1 SHA-256, and is explicitly evidence-only. The computed graph confirms all 101 later issues descend from `M-080`.

2. **F2 report-assembly ownership — resolved.** Automatic `M-078` now produces and tests `feasibility_report.rs`; `M-079` directly consumes it and owns the runnable application composition; assisted `M-080` owns only immutable evidence. The gate therefore no longer introduces source code during measurement.

### New Findings

1. **Integration wiring and wave structure failed — complete-route streaming can run before complete-route generation/store/read wiring exists.**

   Evidence:

   - `M-086` is the issue that registers the broad strata, cave, water, ruin, route, and metadata generators behind the private store and exposes their current truth through the public reads.
   - `M-087` claims that every complete-route distance band installs the documented representation and that eviction/reactivation preserves complete-route current truth, but it depends only on the feasibility streaming boundary `M-071` and the manifest/curation issue `M-085`. It does not depend on `M-086`, the producer boundary whose data it must stream.
   - The computed graph places `M-086` and `M-087` at the same depth, 30. They are therefore eligible for the same wave even though `M-087` cannot exercise complete-route streaming until `M-086` has wired those generators into storage and reads. `M-088` depending on both afterward does not repair `M-087`'s own claimed integration tests.

   Required issue change:

   ```yaml
   id: M-087
   title: "[integration] Extend streaming across the complete route"
   labels: [integration, auto]
   depends_on: [M-071, M-085, M-086]
   body: "Retain the existing description and acceptance criteria, but consume the completed complete-route generation/store/read boundary directly. Inputs must include M-086. Tests must open the complete route through that boundary and prove every band, eviction/reactivation, and LOD transition observes the same current public truth."
   ```

2. **Asset/TDD correctness failed — the derived curated manifest is modeled as independently human-authored and can be installed before its authoritative production inputs.**

   Evidence:

   - `assets.md` says `curated_manifest.ron` is always generated by `moria-curate generate`, is never manually edited, and contains the seed/config digest. `data-model.md` further says the parameters digest includes canonical authoritative config plus ruin-stamp content.
   - Current manual acquisition `M-120` says "Acquire or author" the manifest, depends only on its scaffold and the placeholder audit, and has no dependency on complete curator `M-085`, acquired production region config `M-119`, or acquired production ruin stamp `M-128`. A human-supplied file can therefore look schema-valid while not being the deterministic output of the production generator for those acquired inputs.
   - Wire-in `M-152` can run in parallel with `M-151` (production region config) and `M-160` (production ruin stamp). Its "matching seed/config digest" check consequently has no direct prerequisite guaranteeing that the exact authoritative config and stamp it derives from are already installed. The final audit can detect a mismatch, but there is no issue that regenerates the manifest from the acquired pair, so detection alone does not yield a buildable final asset set.

   Required change: make the manifest acquisition an assisted deterministic derivation after both authoritative input acquisitions, and wire it only after those inputs are installed. Renumber within the acquisition/wire block if backward-only dependency ordering is retained.

   ```yaml
   id: M-120
   title: "[component] Derive the production assets/config/curated_manifest.ron"
   labels: [component, assisted]
   depends_on: [M-009, M-085, M-118, M-119, M-128]
   body: "Run the production moria-curate generator against the exact acquired product_one_region.ron and ruin_p1.ron bytes in an isolated staging root, then place exactly that canonical output and provenance in the acquisition inbox. The output must byte-match an independent rerun, carry the digest of those exact inputs, pass every complete-route/forest/index/overlap check, and must never be manually edited or accepted merely because it parses. Preserve the stable runtime ID/path until wire-in."

   id: M-152
   title: "[integration] Wire acquired assets/config/curated_manifest.ron"
   labels: [integration, auto]
   depends_on: [M-109, M-120, M-149, M-150, M-151, M-160]
   body: "Install the assisted generated bytes only after the production region config and ruin stamp are installed. Regenerate from those installed inputs, require a byte-for-byte match and matching parameters digest before readiness, then run the existing stable-ID, registry, loader, and no-fallback checks."
   ```

### Questions For Coder

None. Both corrections are determined by explicit producer/consumer and generated-asset contracts in the current TDD.

### Answers To Coder

No coder questions were pending. I acknowledge the stated resident-memory closure: `M-180` requires provider measurements for both scenarios on every named configuration, and `M-181` remains false if any required artifact is missing or invalid.

### Notes To Coder

- Mechanical validation passes for the current 181-issue manifest: sequential unique IDs, one setup root, no missing references or cycles, exact dependency/body/Input parity, five acceptance criteria per issue, and maximum depth 51.
- The paired-gate path, AGENTS.md scaffold, 30 nominal asset triples, placeholder audit, final production-asset audit, resident measurement, and final acceptance issues remain present. The second finding concerns the causal semantics of one generated asset triple, not a missing triple.
- No new high-risk-size finding is raised; the edit/readiness, persistence, and benchmark flows remain split into focused issues.

### Approval

Not approved.

## Coder Turn — 2026-07-14T07:50:35Z

Mode: continue

Responding to: 2026-07-14T07:46:59Z

### Responses To Findings

1. Addressed the complete-route streaming dependency gap. `M-087` now directly depends on `M-086`, names the completed generation/store/read boundary in its description and inputs, and requires its integration tests to open the complete route through that public boundary. Its band, eviction/reactivation, and LOD checks must observe the same current public truth. The computed depth moves from 30 to 31, after `M-086` at depth 30.

2. Addressed deterministic curated-manifest provenance and installation order. The ruin-stamp acquisition is now `M-120`; assisted `M-128` runs the production `moria-curate generate` path from the exact `M-119` region-config and `M-120` ruin-stamp bytes after the complete curator `M-085`, requires an independent byte-identical rerun, records exact input/output and generator provenance, and forbids manual editing. The ruin stamp is installed by `M-152`, and curated-manifest wire-in is now `M-160`; `M-160` directly depends on installed production config `M-151` and installed ruin stamp `M-152`, regenerates from those installed inputs, and blocks readiness unless bytes and parameters digest match.

### Answers To Auditor

1. Final resident-memory acceptance closes through reviewed measurement artifacts for both flythrough and carve storm on every named configuration: M4 Metal at 1920x1080 and 2560x1440, plus the designated Linux 3060-class Vulkan configuration at 2560x1440. No estimate substitution or Product-approved substitution ID is assumed; `M-181` remains false if any required `M-180` measurement is missing or invalid.

### TDD Changes Made

- Revised `M-087` dependency, inputs, description, and test contract to consume `M-086` directly.
- Reordered the ruin-stamp and curated-manifest acquisition identities and their wire-in identities so all dependencies remain backward-only while preserving all stable runtime paths.
- Recast curated-manifest acquisition as assisted deterministic derivation and made its wire-in regenerate from the installed authoritative production inputs.

### Questions For Auditor

None. Both producer/consumer corrections were explicit.

### Notes To Auditor

- Validation reports 181 sequential unique IDs, no missing or forward dependencies, no cycles, exactly five acceptance criteria per issue, and exact dependency/body/Input parity.
- The corrected causal paths are `M-086` → `M-087` and `M-119` + `M-120` + `M-085` → `M-128` → registries/input installs → `M-151` + `M-152` → `M-160`.
- The manifest remains at 181 issues; no new implementation behavior or asset stable ID/path was introduced.

### Explicit Divergences

- The auditor draft kept curated-manifest identities at `M-120`/`M-152`. To preserve sequential IDs and the manifest's no-forward-dependency invariant, I swapped the ruin-stamp and curated-manifest identities within both ordered blocks: ruin acquisition/wire-in are `M-120`/`M-152`, while curated-manifest derivation/wire-in are `M-128`/`M-160`. The requested semantic dependencies are preserved exactly.

## Auditor Turn — 2026-07-14T07:55:24Z

Mode: continue

Responding to: 2026-07-14T07:50:35Z

### Prior Findings Status

1. **Complete-route streaming dependency — resolved.** `M-087` now directly depends on `M-086`, names that completed generation/store/read boundary in its description and inputs, and requires complete-route streaming tests to open the world through it. The computed graph places `M-086` at depth 30 and `M-087` at depth 31.

2. **Production curated-manifest derivation and installation order — resolved.** `M-128` is now an assisted deterministic derivation from the exact acquired `M-119` region-config and `M-120` ruin-stamp bytes through the completed curator, with independent byte comparison and derivation provenance. `M-160` waits for installed production config `M-151` and stamp `M-152`, regenerates from those installed inputs, and blocks readiness on any byte or digest mismatch. The ID swaps preserve the requested causal semantics.

### New Findings

1. **Asset scaffolding, wave structure, and F1 input ownership failed — the placeholder/generated curated manifest still has no dependency on the authoritative config and ruin-stamp scaffolds from which its digest is derived.**

   Evidence:

   - `data-model.md` defines `parameters_digest` as SHA-256 over canonical authoritative config bytes **and ruin-stamp content**. `assets.md` says `curated_manifest.ron` is always generated by `moria-curate generate`, carries that digest, and is never manually edited.
   - `M-009` requires its placeholder manifest to carry a matching seed/config digest, but depends only on shared declarations `M-003`. It can run in the same wave as `M-008` (the exact placeholder region-config bytes) and `M-017` (the exact placeholder ruin-stamp bytes). An isolated agent therefore cannot calculate or validate the required digest from its declared inputs.
   - `M-042` is the first issue that runs canonical generation and overwrites `assets/config/curated_manifest.ron` with the checked-in full forest manifest used by F1, yet it depends on neither the path scaffold `M-009` nor the exact config/stamp assets `M-008` and `M-017`. Its declared inputs permit generation from implicit or invented bytes, and a topological scheduler can run it without any of those three asset issues. `M-043` waiting for `M-038` afterward can detect a mismatch, but no issue in that path is then responsible for regenerating the F1 manifest from the exact checked-in source bytes.
   - The corrected production acquisition path now has the required derivation chain, but F1/F2 approval occurs much earlier and must already consume a digest-correct checked-in manifest. Production wire-in cannot retroactively repair the feasibility artifact's source ownership.

   Required change: order the manifest scaffold after the config and stamp scaffolds, then make F1 manifest generation consume all three exact scaffold outputs. Renumber the early scaffold block if backward-only dependency ordering is retained.

   ```yaml
   id: M-009
   title: "[setup] Scaffold assets/config/curated_manifest.ron placeholder"
   labels: [setup, auto]
   depends_on: [M-003, M-008, M-017]
   body: "Create the format-faithful placeholder only after the exact placeholder product_one_region.ron and ruin_p1.ron bytes exist. Compute parameters_digest from those exact canonical inputs, preserve the predeclared stable ID/path, and test that changing either source invalidates the manifest. Retain the existing schema, isolated-loader, and no-shared-table-edit criteria."

   id: M-042
   title: "[system] Generate the checked-in forest manifest"
   labels: [system, auto]
   depends_on: [M-004, M-006, M-008, M-009, M-017, M-039, M-041]
   body: "Implement canonical generation/check of the full forest manifest by reading the exact checked-in placeholder region config and ruin stamp through the stable scaffolded paths. Replace only the scaffolded curated-manifest content, require its digest to match those exact bytes, and retain the existing forest, route-clearance, overlap, canopy, index, stress-target, determinism, and byte-comparison criteria used by F1."
   ```

### Questions For Coder

None. The digest inputs and `moria-curate generate` ownership are explicit in `data-model.md` and `assets.md`; the only implementation choice is whether to renumber the early setup block or permit a forward ID reference while retaining an acyclic graph.

### Answers To Coder

No coder questions were pending. I acknowledge the deliberate ID swap: ruin acquisition/wire-in are now `M-120`/`M-152`, and deterministic curated-manifest derivation/wire-in are `M-128`/`M-160`; those production paths are correct.

### Notes To Coder

- Mechanical validation passes for the current 181-issue manifest: sequential unique IDs, one setup root, no missing/forward references, no cycles, setup-only setup prerequisites, five acceptance criteria per issue, exact dependency/body/Input parity, and no same-depth exact-output collision. Maximum computed depth is 54, and all 101 issues after `M-080` descend from the paired gate.
- The 30 scaffold/acquire/wire triples, placeholder audit, production registry ordering, final asset audit, resident measurements, and final acceptance closure remain present. This finding is the missing causal edge inside the early derived-asset/F1 path, not another missing asset phase.
- No new high-risk-size, public-boundary, persistence, benchmark, or final resident-memory finding is raised.

### Approval

Not approved.

## Coder Turn — 2026-07-14T07:59:05Z

Mode: continue

Responding to: 2026-07-14T07:55:24Z

### Responses To Findings

1. Addressed the early generated-manifest/F1 input-ownership gap. To retain backward-only dependencies, the ruin-stamp scaffold is now `M-009` and the curated-manifest scaffold is `M-017`. `M-017` directly depends on exact region-config scaffold `M-008` and exact ruin-stamp scaffold `M-009`, computes `parameters_digest` from those bytes, and tests that changing either source invalidates the manifest. `M-042` now directly consumes `M-008`, `M-009`, and `M-017`, reads the exact checked-in source bytes through their stable paths, replaces only the scaffolded manifest content, and requires the generated digest and canonical byte comparison to match before F1.

### Answers To Auditor

1. Final resident-memory acceptance will close through reviewed measurement artifacts for both flythrough and carve storm on every named configuration: M4 Metal at 1920x1080 and 2560x1440, plus the designated Linux 3060-class Vulkan configuration at 2560x1440. No estimate substitution or Product-approved estimate-substitution ID is assumed; `M-181` remains false if any required `M-180` measurement is missing or invalid.

### TDD Changes Made

- Revised `docs/issues.json` without changing its 181-issue count.
- Swapped the early ruin-stamp and curated-manifest scaffold identities, then updated the corresponding production acquisition dependencies so each lifecycle still points to the correct stable asset.
- Added exact config/stamp/scaffold dependencies, inputs, digest invalidation tests, and replacement-only generation semantics to `M-017` and `M-042`.

### Questions For Auditor

None. The generated-asset provenance contract determines the dependency order.

### Notes To Auditor

- Validation reports 181 sequential unique IDs, one setup root, no missing or forward dependencies, no cycles, exactly five acceptance criteria per issue, exact dependency/body/Input parity, and no same-depth exact-output collision; maximum depth remains 54.
- The corrected early causal path is `M-008` + `M-009` → `M-017` → `M-042` → `M-043`, so F1 cannot run without the exact checked-in config, ruin-stamp, and digest-valid scaffold outputs.
- Production lifecycle ownership remains consistent: config scaffold/acquisition/wire-in are `M-008`/`M-119`/`M-151`, ruin stamp is `M-009`/`M-120`/`M-152`, and generated curated manifest is `M-017`/`M-128`/`M-160`.

### Explicit Divergences

- The auditor draft retained curated-manifest scaffold ID `M-009` with a forward dependency on `M-017`. I instead swapped the two early scaffold identities so the ruin stamp is `M-009` and the derived curated manifest is `M-017`; this preserves the requested causal semantics while retaining the manifest's backward-only dependency invariant.

## Auditor Turn — 2026-07-14T08:04:19Z

Mode: continue

Responding to: 2026-07-14T07:59:05Z

### Prior Findings Status

1. **Early generated-manifest/F1 input ownership — resolved.** The ruin-stamp scaffold is now `M-009`, and derived-manifest scaffold `M-017` directly depends on exact region-config scaffold `M-008` and exact stamp scaffold `M-009`. Its criteria compute `parameters_digest` from those bytes and invalidate on either source change. `M-042` now directly consumes all three scaffold outputs, reads the stable checked-in paths, replaces only the manifest content, and requires the generated digest and canonical byte comparison to match before F1. The computed path is `M-008` + `M-009` -> `M-017` -> `M-042` -> `M-043`.

### New Findings

1. **TDD API coverage, integration wiring, and gate ownership failed — the required feature-gated curation facade is absent from the issue contracts, while the assisted F1 gate still creates its command.**

   Evidence:

   - `api.md` defines an exact development-only public boundary: `derive_manifest(seed: u64, config: &RegionConfig, stamp: &SparseVoxelStamp) -> Result<CuratedManifest, CurationError>` and `validate_manifest(config: &RegionConfig, manifest: &CuratedManifest) -> Result<CurationReport, CurationError>`. It requires these functions to be exported only under the `curation` Cargo feature, forbids live `WorldStore`/delta/render access, and says the shipped demo and benchmark do not enable or call the feature.
   - `overview.md` independently says `moria-curate/search.rs` is CLI orchestration over `moria-world`'s curation API and that other packages import through public facades. This is a designed system boundary between the library and CLI, not an implementation detail.
   - No manifest issue names either exact function, their inputs/results, the `curation` feature, the negative feature-isolation contract, or an external compile test for this boundary. `M-042` produces internal `curation/generate.rs` and `moria-curate/search.rs` together and says only that `generate`/`check` work. `M-085` later produces internal `curation/check.rs`. Neither produces or tests the crate-root/feature facade. `M-073` explicitly limits its F2 facade criteria to configuration/lifecycle/read/edit/focus/telemetry exports, and `M-089` adds runtime Product One plugins without taking ownership of the development-only curation exports.
   - Current assisted `M-043` says "Implement the production `prove-forest` command and obtain" F1 evidence in the same issue and produces both `crates/moria-curate/src/main.rs` and the measured artifact. Thus the first explicit CLI composition remains a code-producing assisted measurement issue, with no preceding automatic integration deliverable proving that the CLI consumes only the designed public feature. The later F2 gate was correctly split into automatic composition/report ownership followed by evidence-only `M-080`; F1 does not yet have the equivalent buildable boundary.
   - References to `api.md`/`overview.md` elsewhere do not satisfy the TDD-coverage rule that an exact public signature and system-to-system boundary appear as testable issue criteria. An implementer could fulfill every current `M-042`/`M-043` criterion by importing internal modules or duplicating curation logic in the CLI, while omitting both documented public functions and feature isolation.

   Required change: insert an automatic integration issue before the assisted F1 run, make it own the feature-gated facade and complete CLI command composition, and make the F1 run evidence-only. Renumber current `M-043` and later issues consistently if the backward-ID invariant is retained.

   ```yaml
   id: M-043
   title: "[integration] Wire moria-curate through the public curation feature"
   labels: [integration, auto]
   depends_on: [M-001, M-006, M-041, M-042]
   body: "Define the moria-world `curation` Cargo feature and export the exact `derive_manifest` and `validate_manifest` signatures from the crate root only when that feature is enabled. Compose moria-curate generate/check/prove-forest through those public functions, never private source modules or duplicated generator/validator logic. External-package compile/runtime tests must prove deterministic values and typed errors, prove the API cannot access a live WorldStore/deltas/render state, and prove moria-demo and moria-bench build without enabling or calling the feature. Produce the completed CLI command and report validator code; do not create acceptance evidence in this issue. Reference: docs/tdd/api.md; docs/tdd/overview.md; docs/tdd/implementation-plan.md. Include the standard tests/no-regressions criteria."

   id: M-044
   title: "[integration] Run and approve the forest feasibility gate"
   labels: [integration, assisted]
   depends_on: [M-007, M-038, M-041, M-042, M-043]
   body: "From one unchanged clean gate-baseline commit, run the already completed production prove-forest command on the named M4 and retain immutable passing or failed F1 evidence. Validate every existing content/index/time/memory/allocation witness and exact build/world/config/stamp/manifest/machine identity. This issue is evidence-only: it changes no Cargo manifest, Rust source, checked-in curated manifest, command implementation, validator, threshold, or test. Preserve failed artifacts and block the later paired gate on failure. Reference: docs/tdd/implementation-plan.md. Include the standard tests/no-regressions criteria."
   ```

### Questions For Coder

None. The exact facade signatures, feature isolation, and library-to-CLI relationship are explicit in `api.md` and `overview.md`.

### Answers To Coder

No coder questions were pending. I acknowledge the deliberate early scaffold ID swap; it preserves both the requested digest provenance and the no-forward-dependency invariant.

### Notes To Coder

- Mechanical validation passes for the current 181-issue manifest: sequential unique IDs, one setup root, no dangling or forward references, no cycles, setup-only setup prerequisites, exact dependency/body/Input parity, five acceptance criteria per issue, no same-depth exact-output collision, and maximum depth 54.
- The 30 asset lifecycle triples, corrected production and early curated-manifest provenance, complete-route integration ordering, paired same-commit F1/F2 rerun, final production-asset audit, resident measurements, and final acceptance closure remain intact.
- No new asset-pair, async-size, persistence, benchmark, or resident-memory finding is raised. This finding is limited to the exact development-tool public API and keeping its assisted gate evidence-only.

### Approval

Not approved.

## Coder Turn — 2026-07-14T08:08:25Z

Mode: continue

Responding to: 2026-07-14T08:04:19Z

### Responses To Findings

1. Addressed the missing development-only public curation boundary and F1 ownership split. New automatic `M-043` defines the `curation` Cargo feature; exports the exact `derive_manifest(seed: u64, config: &RegionConfig, stamp: &SparseVoxelStamp) -> Result<CuratedManifest, CurationError>` and `validate_manifest(config: &RegionConfig, manifest: &CuratedManifest) -> Result<CurationReport, CurationError>` signatures only under that feature; composes `generate`, `check`, and `prove-forest` through those public functions; and tests external-package behavior, typed errors, forbidden live-world access, and demo/benchmark feature isolation. Assisted `M-044` now depends on that code-complete boundary, produces only `target/feasibility/forest.json`, and explicitly forbids source, manifest, command, validator, threshold, configuration, or asset changes while recording F1 evidence.

### Answers To Auditor

1. Final resident-memory acceptance will close through reviewed measurement artifacts for flythrough and carve storm on every named configuration: M4 Metal at 1920x1080 and 2560x1440, plus the designated Linux 3060-class Vulkan configuration at 2560x1440. No Product-approved estimate-substitution ID is assumed; final acceptance remains false if any required measurement artifact is missing or invalid.

### TDD Changes Made

- Inserted automatic `M-043` for the feature-gated moria-world facade, external-consumer compile/runtime coverage, and complete moria-curate CLI composition.
- Converted assisted `M-044` into an immutable evidence-only F1 run and added `M-043` as its direct prerequisite.
- Shifted former `M-043` and all later manifest IDs/references by one so dependencies remain backward-only; the paired same-commit gate is now `M-081`, resident measurement is `M-181`, and final acceptance is `M-182`.

### Questions For Auditor

None. The exact API signatures, negative feature-isolation contract, and evidence-only ownership boundary are now explicit.

### Notes To Auditor

- Validation reports 182 sequential unique IDs, one setup root, no missing or forward dependencies, no cycles, exactly five acceptance criteria per issue, exact dependency/body/Input parity, references to all ten TDD files, and maximum depth 54.
- All 101 issues after paired gate `M-081` descend from it. The graph retains 30 scaffold issues, 30 acquisition/derivation issues, and 30 wire-in issues; curated-manifest provenance and complete-route streaming dependencies remain intact after renumbering.
- The new causal path is `M-042` -> automatic public facade/CLI composition `M-043` -> evidence-only F1 `M-044` -> feasibility integration -> code-complete F2 executable `M-080` -> paired immutable F1/F2 rerun `M-081`.

## Auditor Turn — 2026-07-14T08:14:22Z

Mode: approved

Responding to: 2026-07-14T08:08:25Z

### Prior Findings Status

1. **Feature-gated curation facade — resolved.** Automatic `M-043` now owns the exact `derive_manifest(seed: u64, config: &RegionConfig, stamp: &SparseVoxelStamp) -> Result<CuratedManifest, CurationError>` and `validate_manifest(config: &RegionConfig, manifest: &CuratedManifest) -> Result<CurationReport, CurationError>` crate-root exports under the `curation` feature. It composes all three `moria-curate` commands through those functions and tests the external-package boundary, typed errors, forbidden live-world access, and demo/benchmark feature isolation. Its direct dependencies include the workspace, public curation model, bounded object index, and deterministic generator/validator work it connects.

2. **F1 code/evidence ownership — resolved.** Assisted `M-044` now depends directly on code-complete `M-043`, produces only `target/feasibility/forest.json`, and explicitly forbids source, Cargo manifest, command, validator, threshold, configuration, curated-manifest, or asset changes. Automatic implementation and immutable acceptance execution are therefore separate before downstream feasibility composition.

3. **All earlier findings — remain resolved after renumbering.** The AGENTS.md scaffold remains in `M-001`; generated-manifest digest ownership remains `M-008` + `M-009` -> `M-017` -> `M-042`; complete-route streaming remains `M-086` -> `M-087` -> `M-088`; the F2 report/executable/evidence chain remains code-complete before assisted `M-081`; persistence, renderer-readiness, public-consumer, benchmark, and resident-memory responsibilities remain split; and the production curated-manifest derivation/wire-in path still consumes installed authoritative config and ruin-stamp bytes.

### New Findings

None. The revised issue set covers the design and every TDD section with implementable contracts, explicit public/integration boundaries, bounded high-risk work, and final evidence closure.

### Questions For Coder

None.

### Answers To Coder

No coder questions were pending. The repeated resident-memory clarification is accepted: `M-181` requires reviewed measurements for flythrough and carve storm on M4 Metal at both 1920x1080 and 2560x1440 and on the designated Linux 3060-class Vulkan configuration at 2560x1440; `M-182` cannot pass if any required artifact is missing or invalid.

### Notes To Coder

- Independent mechanical validation passes for all 182 sequential unique issues: one setup root, no dangling or forward dependencies, no cycles, no duplicate dependency entries, setup-only setup prerequisites, exact JSON/body/Input dependency parity, and exactly five acceptance criteria per issue. Maximum dependency depth is 54, and every issue after paired gate `M-081` descends from that gate.
- The asset inventory has exact one-per-file lifecycle coverage: 30 automatic scaffolds, 30 manual/assisted acquisitions or deterministic derivations, and 30 automatic wire-ins, followed by the distinct final production-asset audit. No scaffold path lacks an acquisition or wire-in phase.
- The graph preserves the placeholder wiring audit before human acquisition, the final asset audit after all production wire-ins, exact production-route/public-consumer coverage, and resident-memory evidence before final Product One acceptance.

### Approval

Approved.

All issues resolved. Decomposition approved.
