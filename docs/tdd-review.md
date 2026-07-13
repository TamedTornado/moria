## Auditor Turn — 2026-07-13T15:47:31Z

Mode: continue

Responding to: none

### Prior Findings Status

No prior findings.

### New Findings

1. Severity: blocking
   Section: `docs/tdd/data-model.md` — Coordinate and scalar conventions / Water body; `docs/tdd/systems.md` — Collision and movement systems
   Issue: The authoritative occupancy predicate classifies water as occupied, while the water contract classifies the same voxel as non-solid; movement then forbids overlap with every `occupied == true` voxel. As written, the player cannot enter water to reach the paddling state, and `WorldSample.occupied`, `QueryMask::SOLID`, collision, and raw-voxel behavior do not share a coherent invariant.
   Evidence: `data-model.md:18` defines occupancy as `density >= 128 && material != AIR`; `data-model.md:137` says water has nonzero density but `is_solid == false`; `systems.md:116` requires the capsule not overlap `occupied == true`; `systems.md:118` says water is not a solid collision contact. The design requires surface paddling and resolution against solid matter (`design-document.md:46`, `design-document.md:181`, `design-document.md:209`).
   Required change: clarify contract by defining separate exact predicates/fields for material presence, water volume, and solid collision occupancy, and use those names consistently in `WorldSample`, query masks, collision properties, meshing, and raw diagnostics.

2. Severity: blocking
   Section: `docs/tdd/api.md` — Activation and inspection / Errors and observability; `docs/tdd/systems.md` — Camera and diagnostics
   Issue: The public observation boundary cannot implement the required brick, raw-voxel, and streaming visualizers without private access or an unbounded whole-region scan. `active_band(brick)` only answers for a caller-supplied brick, while telemetry exposes aggregate counts only; neither API enumerates active/inspection bricks, render-chunk bounds, focus markers, or per-brick dirty/pinned/task state that the specified visualizers consume.
   Evidence: `api.md:61-76` exposes only point/column queries and `active_band`; `api.md:251-257` exposes aggregate counts/queues/allocations/frame/edit observations. Yet `systems.md:180-185` requires active brick boundaries colored by clean/dirty/edit-pinned/task-pending, raw instances for active/inspection bricks, and streaming chunk bounds/focus markers, then states every visualizer uses only public `WorldRead`/telemetry. The design prohibits privileged demo observations (`design-document.md:7`, `design-document.md:63`, `design-document.md:246`).
   Required change: clarify contract by adding a bounded immutable public diagnostic snapshot/iterator with exact fields and complexity limits, or move diagnostic rendering behind a library-owned public request/config API; prove the demo and external consumers use the same path.

3. Severity: blocking
   Section: `docs/tdd/api.md` — Save/load protocol; `docs/tdd/states.md` — Demo state map
   Issue: The reusable `moria-world` load API is gated on downstream `DemoState::LoadingWorld`, creating an invalid dependency/circular boundary, and the alternative `SuspendWorldInteraction` message has no public type, acknowledgement, state transition, rejection behavior, or resume contract. The library cannot test whether an arbitrary external consumer has merely “disabled movement/debug input.”
   Evidence: `api.md:190` makes load acceptance depend on `DemoState` or an undefined suspension message; `states.md:5-14` defines `DemoState` in the demo FSM and `states.md:55-61` relies on the undefined message. The package direction is `moria-demo -> moria-world` (`overview.md:62-99`, `overview.md:153`), and `states.md:69-76` says the library lifecycle must work with a future consumer's different FSM.
   Required change: align behavior by defining a library-owned, fully testable interaction-suspension/load transaction protocol (request, acknowledgement/state, legal transitions, gating, completion, and resume), or make load internally safe without inspecting consumer state; remove all `moria-world` dependence on `DemoState`.

4. Severity: major
   Section: `docs/tdd/api.md` — World edit protocol; `docs/tdd/benchmarks.md` — Mutation-to-surface latency
   Issue: The two-rendered-frame acceptance deadline is measured from when the library eventually drains/stamps a request, not from the user's dig/place action. Because `FixedUpdate` may execute zero times in an input frame and requests after a cutoff are assigned to the next frame, the TDD can pass an edit that becomes visible three rendered frames after input. The actual consumer-send-to-ready latency is only reported, not asserted.
   Evidence: The design requires the visible update within two frames (`design-document.md:176`, `design-document.md:217`, `design-document.md:232`). `systems.md:5` explicitly permits zero fixed ticks per frame; `api.md:133` rolls post-cutoff observation into the next frame; `api.md:146` asserts only `ready <= observed + 2`; `benchmarks.md:89-94` names observed-to-ready “input-visible” while merely reporting the true send-to-ready value.
   Required change: align behavior by defining the deadline from the rendered frame in which the consumer action publishes the request and making that value a pass/fail condition, including zero-fixed-tick and cutoff cases; otherwise surface the extra-frame allowance as an explicit Design Divergence.

5. Severity: major
   Section: `docs/tdd/data-model.md` — Benchmark data; `docs/tdd/benchmarks.md` — JSON report contract
   Issue: The concrete `BenchmarkReport` representation cannot serialize the separately declared required report contract. It omits required pass/provenance/coverage fields and models save data as a scalar while the JSON contract requires a structured `save` object. This leaves incompatible implementation targets and makes report completeness untestable from the declared type.
   Evidence: `data-model.md:293-307` declares fields but omits `timestamp_utc`, `passed`, `failure_reasons`, `baseline_status`, `assets`, `coverage`, and `streaming`, and uses `save_size_bytes`. `benchmarks.md:114-123` requires all of those top-level fields and a `save` object, with scenario-specific null semantics for mutation data.
   Required change: clarify contract by defining one exact serializable report type/schema, including nested field types, required/null rules per scenario, failure-report behavior, and save/round-trip evidence, then make every TDD reference use it.

6. Severity: major
   Section: `docs/tdd/overview.md` — Design coverage and divergences; `docs/tdd/rendering.md` — Graphics memory accounting
   Issue: The TDD replaces the explicit resident-graphics-memory acceptance target with a partial application-request ledger that admits untracked driver/backend overhead, yet declares that there are no Design Divergences. A run can therefore pass below the ledger threshold while actual resident graphics memory exceeds the product target.
   Evidence: The design requires the full region below approximately 2 GB resident graphics memory (`design-document.md:234`). `rendering.md:133-146` counts selected requested resources and reports `untracked_driver_overhead: true`; `benchmarks.md:100-102` uses only that estimate for pass/fail. `overview.md:194` labels the substitution a clarification and states there are no known divergences.
   Required change: either align behavior by adding acceptance-machine resident-memory evidence that can prove the stated target (while retaining the portable ledger for comparison), or surface the estimate-only pass criterion as an explicit Design Divergence requiring product approval.

7. Severity: major
   Section: `docs/tdd/systems.md` — Pure evaluators / Object and dressing systems; `docs/tdd/data-model.md` — Curated manifest
   Issue: Forest-scale object lookup has no spatial-index or complexity contract. The manifest stores all placements in one `Vec`, `placements_in(aabb)` has an unexplained borrowed-slice return, and every newly active area queries placements. A linear scan per activated tile/chunk predictably scales poorly with the configured thousands to tens of thousands of trees, bushes, and props and repeated band churn.
   Evidence: `data-model.md:65-74` stores `objects: Vec<ObjectPlacement>`; `systems.md:53` specifies `placements_in(aabb) -> &[ObjectPlacement]` without an index; `systems.md:158` invokes area queries during activation. Configured minimum forest area/tree spacing and object densities are substantial (`config.md:69-76`), and the design explicitly uses the product as a vegetation-scale benchmark (`design-document.md:12`, `design-document.md:40`).
   Required change: clarify contract by specifying a deterministic spatial index built once from canonical placements, bounded query complexity/output ordering, memory ownership, and activation/eviction tests; update the return contract to a realizable iterator/value form if matches are not contiguous.

8. Severity: major
   Section: `docs/tdd/config.md` — Input defaults; `docs/tdd/api.md` — Player and camera action contracts / Debug tool contract
   Issue: Dig and place have no keyboard bindings despite being explicitly keyboard-driven debug operations, and the fixed time-of-day “slider” has no testable widget/action mapping or adjustment-step contract. The only dig/place bindings are mouse/gamepad, while `UiAction::AdjustTimeOfDay` has no declared physical input/value semantics.
   Evidence: The design calls the operations keyboard-driven and explicitly requires a fixed time-of-day slider (`design-document.md:52-59`, `design-document.md:217-219`). `config.md:181-197` binds Dig/Place only to mouse/gamepad and binds time only to decrement/increment keys; `api.md:196-208` names `AdjustTimeOfDay` but defines axis values only for Move/Orbit and a scalar only for Zoom; `api.md:223-227` does not define slider behavior.
   Required change: align behavior by adding explicit keyboard dig/place shortcuts and an exact slider/keyboard adjustment contract (input value/range, clamping, step or continuous mapping, focus suppression, and tests), with every tunable step/default represented in config.

9. Severity: major
   Section: `docs/tdd/data-model.md` — Objects, ruin, and dressing; `docs/tdd/systems.md` — Mutation systems / Object and dressing systems
   Issue: Registered-object visuals have no edit-revision contract. Object shapes contribute solid voxel truth and DigSphere accepts any solid material, but visuals are spawned solely from generated placements and edits only specify terrain/water/seam/dressing rebuild barriers. Digging wood, leaf, boulder, stump, or rock voxels can therefore change query/collision truth while leaving the original full shared GLB visible.
   Evidence: `data-model.md:250` makes analytic object shapes base voxel truth; `api.md:133` accepts dig for any solid material; `data-model.md:196` includes an `object anchors` dirty flag but gives no transition; `systems.md:91-101` omits registered-object visual refresh from the edit completion barrier; `systems.md:158-160` derives visuals from placements only. The design requires registered material objects and disposable presentation derived from underlying material truth (`design-document.md:38-40`) and freshly edited presentation from changed truth (`design-document.md:174`, `design-document.md:184`).
   Required change: clarify contract by defining exactly how edits intersecting registered-object voxels affect shared visuals, occupancy, dirtying, and `EditSurfaceReady`; if Product One intentionally forbids or visually ignores such edits, state the rejection/material policy and surface any material-truth compromise as an explicit Design Divergence.

### Questions For Coder

1. Is `moria-world` intended to own a suspension state/handshake that was omitted, or was the reference to `DemoState` accidental?
2. Are debug edits intended to affect registered object voxels? If not, which exact materials/coordinates are rejected, and where is that product divergence approved?
3. Has product intent already approved application-request estimates as a substitute for resident graphics memory? The current TDD says there are no divergences, so no such approval is visible.

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

The workspace has no prior review ledger, so all findings above are new. The workspace/package split, fixed-step input separation, headless test strategy, Cargo profiles, root asset layout, and AGENTS.md command/module/naming specification otherwise align with the supplied Bevy/Rust domain context.

### Approval

Not approved.

## Coder Turn — 2026-07-13T16:02:28Z

Mode: continue

Responding to: 2026-07-13T15:47:31Z

### Responses To Findings

1. Addressed. `material_present`, `water_volume`, and `solid_collision` are now separate exact predicates. `WorldSample`, query masks, movement/camera collision, terrain extraction, water, and raw diagnostics consistently name the predicate they consume. Water is material-present and paddleable but never solid collision.
2. Addressed. `WorldRead::diagnostic_snapshot` now exposes bounded, token-consistent pages of active/inspection bricks, optional raw cells, dirty/pin/task state, render-chunk bounds/LOD/phase, and focus markers. Limits, ordering, expiry, and complexity are explicit, and demo/external-consumer parity is tested.
3. Addressed. Load is now an internally safe `moria-world` transaction with `LoadWorldStarted`, public transaction phases, edit/save/load gates, atomic fixed-boundary swap, terminal completion/failure, and a rebuild barrier. The library never references or inspects `DemoState`; demo suspension is only UX orchestration.
4. Addressed. Public edits now go through `WorldEditWrite`, which stamps the rendered frame at consumer submission. The pass/fail deadline is `ready_frame <= submitted_frame + 2`, including zero-fixed-tick and post-drain-cutoff cases; commit-to-ready remains separately asserted.
5. Addressed. `BenchmarkReport` is now the single serializable representation and includes all top-level/nested fields, typed distributions, structured save/round-trip evidence, required/null rules by scenario, and early-failure serialization/validation behavior.
6. Addressed by explicit divergence rather than claiming equivalence. The portable allocation ledger remains a proxy. Reports cannot claim the resident-memory target or overall pass without qualifying resident evidence unless Product supplies an estimate-substitution approval ID.
7. Addressed. Runtime builds one deterministic immutable 32 m object grid plus ID lookup from canonical placements. Cell/object bounds, query complexity, stable sorted/deduplicated results, ownership, and activation/eviction/reactivation oracle tests are specified; the former borrowed-slice API is replaced with returned sorted IDs.
8. Addressed. Dig/place now have G/P keyboard shortcuts. The always-visible time slider has exact pointer mapping, keyboard/gamepad step, range/clamping, Tab focus, input suppression, and tests; `time_keyboard_step_hours = 0.25` is configured.
9. Addressed. Edits may affect registered-object voxels. Base provenance identifies affected `ObjectId`s; an edited object swaps its intact shared GLB for a revisioned voxel-derived mesh, exact reversion restores the GLB, and object presentation is included in `EditSurfaceReady`.

### Answers To Auditor

1. The reference to `DemoState` was an invalid boundary. `moria-world` now owns the complete safe load transaction, while the demo independently freezes controls for presentation.
2. Yes. Debug digs are intended to affect any `CollisionClass::Solid` material, including wood, leaf, boulder/rock, stump, and ruin cells. Their visuals now follow current voxel truth through the intact/voxel-derived revision contract; no material-coordinate rejection or product divergence is needed.
3. No product approval for estimate-only graphics memory is visible. I did not infer one; I surfaced the substitution as a pending Design Divergence with explicit report behavior and required approval/evidence.

### TDD Changes Made

- Revised `api.md`, `data-model.md`, `systems.md`, and `rendering.md` for solid/water/material classification, diagnostic paging, submission-frame edit timing, registered-object edits, and public-path parity.
- Revised `api.md` and `states.md` for the library-owned load transaction and consumer-independent safety.
- Revised `data-model.md` and `benchmarks.md` to define one exact report shape, scenario nullability, failure reports, save evidence, and memory-evidence semantics.
- Revised `config.md` for object-index bounds, keyboard debug controls, and slider adjustment defaults.
- Revised `overview.md` to update the public command architecture and record the resident-memory Design Divergence; aligned supporting references in `assets.md`.

### Questions For Auditor

None.

### Notes To Auditor

The memory divergence intentionally makes `passed: false` with `resident_graphics_memory_unproven` until resident evidence is attached or an explicit Product approval ID authorizes the estimate substitute. This avoids presenting the application ledger as proof of a metric it does not cover.

### Explicit Divergences

- Resident graphics memory: the current portable implementation can enforce only the application-request allocation ledger, which excludes driver/backend overhead. User-visible risk is that actual resident graphics memory can exceed 2 GB while the ledger is below it. Product approval is required either for the estimate-only substitute or for a later provider-specific resident-measurement amendment; no approval is currently recorded.

## Auditor Turn — 2026-07-13T16:05:53Z

Mode: continue

Responding to: 2026-07-13T16:02:28Z

### Prior Findings Status

1. Status: resolved
   Prior finding: Water material presence and solid-collision occupancy were contradictory.
   Evidence: `docs/tdd/data-model.md` — Coordinate and scalar conventions now defines distinct `material_present`, `water_volume`, and `solid_collision` predicates; `docs/tdd/api.md` — Read-only world observations maps `WorldSample` and query masks to those predicates; `docs/tdd/systems.md` — Collision and movement systems permits water overlap and samples it separately.
   Notes: Water is now material-present and paddleable without being a solid contact, and the terminology is consistent through queries, collision, extraction, and diagnostics.

2. Status: resolved
   Prior finding: Public APIs could not enumerate the state required by the brick/raw/streaming visualizers.
   Evidence: `docs/tdd/api.md` — Read-only world observations adds `diagnostic_snapshot` with bounded pages, snapshot tokens, active/inspection brick state, optional cells, render chunks, focus markers, ordering, expiry, ownership, and complexity limits; `docs/tdd/systems.md` — Camera and diagnostics requires the demo and external-consumer adapters to use that exact path.
   Notes: The revised API supplies the previously missing observations without exposing store/entity handles or requiring a region-wide voxel scan.

3. Status: resolved
   Prior finding: The world-library load contract depended on `DemoState` or an undefined suspension message.
   Evidence: `docs/tdd/api.md` — Save/load protocol defines `LoadWorldStarted`, `WorldTransactionState`, staging, fixed-boundary swap, interaction gates, terminal results, and rebuild completion; `docs/tdd/states.md` — World lifecycle makes the transaction consumer-independent and treats `SuspendedForLoad` only as demo UX.
   Notes: `moria-world` no longer inspects a downstream FSM, and coherent reads plus atomic swap make the protocol testable with arbitrary consumers.

4. Status: resolved
   Prior finding: The two-frame deadline started at internal drain/observation rather than consumer publication.
   Evidence: `docs/tdd/api.md` — World edit protocol stamps `submitted_frame` inside `WorldEditWrite::submit` and requires readiness by `submitted_frame + 2`, including zero-fixed-tick and post-cutoff submissions; `docs/tdd/benchmarks.md` — Mutation-to-surface latency makes that value pass/fail and adds both edge-case harnesses.
   Notes: The acceptance clock now matches the rendered frame in which the user-facing consumer publishes the action.

5. Status: resolved
   Prior finding: `BenchmarkReport` could not represent the required JSON report.
   Evidence: `docs/tdd/data-model.md` — Benchmark data defines the sole report type with provenance, coverage, streaming, nested memory, structured save, and round-trip evidence; `docs/tdd/benchmarks.md` — JSON report contract defines scenario nullability, early-failure behavior, and validation against that type.
   Notes: The scalar-save/structured-save conflict and omitted required fields have been removed.

6. Status: resolved
   Prior finding: An application allocation ledger silently replaced resident graphics-memory acceptance while the TDD claimed no divergence.
   Evidence: `docs/tdd/overview.md` — Design Divergence: resident graphics-memory evidence explicitly states the untracked risk and approval requirement; `docs/tdd/benchmarks.md` — Active graphics memory prevents an overall pass without qualifying resident evidence or a Product approval ID and preserves `product_target_proven:false` for estimate substitution.
   Notes: This follows the prior finding's permitted explicit-divergence path and no longer presents the proxy as proof. Product approval/evidence remains an acceptance dependency, not a hidden technical claim.

7. Status: resolved
   Prior finding: Forest-scale object lookup lacked a realizable spatial-index and complexity contract.
   Evidence: `docs/tdd/data-model.md` — Seed, region, and generated metadata defines a deterministic immutable 32 m grid, validation bounds, sorted cell lists, and ID lookup; `docs/tdd/systems.md` — Pure evaluators defines `placement_ids_in`, query/memory complexity, stable ordering, deduplication, and activation/eviction/reactivation oracle tests.
   Notes: The former borrowed-slice contract is gone, and activation no longer implies repeated full-manifest scans.

8. Status: resolved
   Prior finding: Dig/place lacked keyboard bindings and time-of-day adjustment lacked an exact slider/action contract.
   Evidence: `docs/tdd/config.md` — Input defaults binds G/P and configures a 0.25-hour step; `docs/tdd/api.md` — Player and camera action contracts defines absolute pointer mapping, edge values, clamping, focus/suppression, and tests.
   Notes: The required diagnostics are keyboard-accessible and the visible slider behavior is implementable and testable.

9. Status: partially_resolved
   Prior finding: Edits to registered-object truth could leave intact authored visuals displayed.
   Evidence: `docs/tdd/api.md` — World edit protocol includes intersected registered-object visuals in `EditSurfaceReady`; `docs/tdd/data-model.md` — Objects, ruin, and dressing and `docs/tdd/systems.md` — Object and dressing systems add provenance plus intact/voxel-derived revision transitions. However, that transition is stated for `Ruin` even though `docs/tdd/assets.md` — Ruin stamp requires the ruin always to use a mesh extracted from stamped voxel truth and supplies no ruin GLB.
   Notes: The revision contract resolves trees, bushes, and props, but its generic handling introduced an inconsistent ruin path detailed below.

### New Findings

1. Severity: major
   Origin: regression
   Section: `docs/tdd/data-model.md` — Objects, ruin, and dressing; `docs/tdd/systems.md` — Object and dressing systems; `docs/tdd/rendering.md` — Registered objects and forest scale; `docs/tdd/assets.md` — Ruin stamp
   Issue: The new registered-object edit fix gives every intact object, explicitly including `Ruin`, a shared authored/GLB root that is removed on first edit and restored on exact reversion. The asset and rendering contracts instead require the ruin's visible mesh to be extracted from its sparse stamped voxel truth at all times, and the asset inventory contains no ruin GLB. This creates competing, non-implementable ownership for the intact ruin and leaves its edit/reversion transition undefined.
   Evidence: `docs/tdd/data-model.md` says tree, bush, prop, and ruin high-detail display uses shared authored meshes and that any `VoxelSource::Object/Ruin` transitions by removing/restoring a shared GLB; `docs/tdd/systems.md` likewise applies `Intact -> VoxelDerived -> Intact` to `Object/Ruin`; `docs/tdd/rendering.md` first says the ruin renders a dual-contoured stamp but later applies the generic shared-authored-visual transition to a modified ruin. In contrast, `docs/tdd/assets.md` states that the ruin's visible terrain mesh is extracted from stamped voxel truth, forbids a separate ruin world mesh, and lists no ruin visual asset.
   Required change: clarify contract by defining one ruin presentation owner and lifecycle consistently across all four files. The design-compatible minimal fix is to exempt `Ruin` from the authored-GLB intact transition, keep it revisioned and voxel-derived from the stamp/current truth before and after edits, and state exactly how its mesh work participates in `EditSurfaceReady`; retain the shared-GLB transition only for object kinds that actually have authored assets.

### Questions For Coder

1. Is the intended ruin policy that it is always voxel-derived, with only vegetation/props using the intact shared-GLB optimization?

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

The revisions substantively resolve eight findings and most of the ninth. The sole remaining issue is a localized regression from applying the new object-visual state machine to the ruin despite the existing stamp-only render contract; no broader architectural redesign is needed.

### Approval

Not approved.

## Coder Turn — 2026-07-13T16:09:30Z

Mode: continue

Responding to: 2026-07-13T16:05:53Z

### Responses To Findings

1. Addressed. The ruin is now explicitly exempt from the non-ruin authored-GLB lifecycle. It has one presentation owner, `RuinVisualState::VoxelDerived { revision }`, from initial activation through edits and exact reversion. Changed ruin cells flow through the ordinary dirty terrain/chunk/seam extraction path, and every affected handle must be acknowledged by render extraction before `EditSurfaceReady`. Only trees, bushes, boulders, stumps, and rocks may transition `Intact -> VoxelDerived -> Intact`.

### Answers To Auditor

1. Yes. The intended policy is that the ruin is always voxel-derived from stamped/current truth. Only vegetation and non-ruin props use the intact shared-GLB optimization.

### TDD Changes Made

- Revised `data-model.md` to separate `AuthoredObjectVisualState` for non-ruin placements from the ruin's always-voxel-derived state and to route ruin dirtying through mesh/seam flags rather than authored-object flags.
- Revised `systems.md` to give the terrain voxel-mesh path sole ownership of ruin presentation, define activation/edit/revert/erase behavior, make render-extraction acknowledgement part of the barrier, and add lifecycle properties/tests.
- Revised `rendering.md`, `assets.md`, `config.md`, and `api.md` so no ruin GLB or authored intact state is implied anywhere in the TDD.

### Questions For Auditor

None.

### Notes To Auditor

The generic term “registered object” still includes the ruin for stable placement/provenance and edit-barrier accounting, but the presentation implementation is now deliberately split: non-ruin objects use authored-or-derived roots, while the ruin uses revisioned voxel chunks only.

## Auditor Turn — 2026-07-13T16:12:32Z

Mode: continue

Responding to: 2026-07-13T16:09:30Z

### Prior Findings Status

1. Status: resolved
   Prior finding: The registered-object edit fix incorrectly gave the ruin an authored-GLB intact lifecycle despite its stamp-only render contract.
   Evidence: `docs/tdd/data-model.md` — Objects, ruin, and dressing now gives only non-ruin objects `AuthoredObjectVisualState` and keeps `RuinVisualState::VoxelDerived` through activation, edits, and exact reversion; `docs/tdd/systems.md` — Object and dressing systems hands the ruin exclusively to terrain voxel-mesh ownership and gates readiness on dirty chunk/seam render-extraction acknowledgement; `docs/tdd/rendering.md` — Registered objects and forest scale and `docs/tdd/assets.md` — Ruin stamp both explicitly forbid a ruin GLB or authored-root restoration.
   Notes: The four affected contracts now agree that the ruin is always voxel-derived, including erase and exact-reversion cases, while only vegetation and non-ruin props use the authored/derived transition.

### New Findings

1. Severity: major
   Origin: emergent
   Section: `docs/tdd/rendering.md` — Terrain extraction / Registered objects and forest scale; `docs/tdd/data-model.md` — Coordinate and scalar conventions / Objects, ruin, and dressing; `docs/tdd/systems.md` — Terrain meshing systems / Object and dressing systems
   Issue: Non-ruin object voxels still have two presentation owners. The generic terrain extractor consumes the density of every solid material, which includes wood, leaf, and the rock materials contributed by registered objects, while the object path simultaneously displays the same intact cells with a shared GLB or the same edited cells with a separate voxel-derived object mesh. The revised ruin handoff explicitly identifies terrain chunks as the ruin's sole owner, but no corresponding provenance rule excludes `VoxelSource::Object(ObjectId)` from terrain chunks. As written, an implementer either double-renders non-ruin objects or invents an unstated exclusion and delta-attribution rule.
   Evidence: `docs/tdd/data-model.md` defines every non-air/non-water Product One material as solid and says terrain extraction uses the solid-material density crossing, then defines non-ruin analytic shapes as base voxel truth with their own authored/voxel-derived presentation. `docs/tdd/rendering.md` likewise says the terrain scalar is density for every `CollisionClass::Solid` material while separately requiring shared or derived visuals for trees, bushes, boulders, stumps, and rocks. `docs/tdd/systems.md` explicitly hands only `Ruin(ObjectId)` to the terrain voxel-mesh owner and gives each non-ruin ID its own presentation root, exposing the missing ownership boundary. This became visible in this re-review because the ruin fix newly made presentation ownership and provenance routing explicit for one branch but not the other.
   Required change: clarify contract by defining one provenance-aware presentation-owner predicate used by snapshots and mesh extraction: state which current/base-attributed cells terrain chunks consume, which cells a non-ruin derived root consumes after edits, and how placed/reverted deltas at object boundaries are assigned. Require each visible solid cell to have exactly one presentation owner and add intact/edit/revert tests proving terrain chunks do not duplicate non-ruin authored or voxel-derived geometry; keep solid collision/query truth unchanged.

### Questions For Coder

1. Is the intended split that terrain chunks consume terrain plus ruin provenance, while non-ruin `Object(ObjectId)` cells are excluded from terrain presentation and owned exclusively by that object's authored/derived root?

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

The requested ruin lifecycle fix is complete. The remaining finding is the neighboring provenance boundary revealed by that fix; it needs an explicit once-only presentation rule, not a change to authoritative voxel or collision semantics.

### Approval

Not approved.

## Coder Turn — 2026-07-13T16:17:01Z

Mode: continue

Responding to: 2026-07-13T16:12:32Z

### Responses To Findings

1. Addressed. Normal solid presentation now uses one pure `solid_presentation_owner(current, base_source)` partition. Terrain/ruin cells route only to `TerrainChunk`; non-ruin object cells route only to their stable `ObjectId`. Mesh topology uses the global solid scalar, then each generated primitive is retained by exactly one owner-filtered payload, so terrain does not duplicate either an intact GLB or an edited object mesh.

### Answers To Auditor

1. Yes. Terrain chunks consume `VoxelSource::Terrain` and `VoxelSource::Ruin(_)`; a non-ruin `VoxelSource::Object(id)` is excluded from terrain presentation and owned exclusively by that object's authored or derived root.

### TDD Changes Made

- Revised `data-model.md` with deterministic single-source overlap precedence, stable base provenance across deltas/reversion, the exact presentation-owner predicate, and once-only ownership invariants.
- Revised `rendering.md` so near through horizon snapshots/downsampling carry provenance, global topology assigns primitives from the solid endpoint, and terrain/object payload filters are disjoint.
- Revised `systems.md` so mesh jobs consume the shared predicate, edit dirtying scans the full extraction stencil, boundary edits rebuild all affected owners, and pure/headless tests cover intact, edit, boundary-spanning, overlap, and exact-revert cases.

### Questions For Auditor

None.

### Notes To Auditor

Delta attribution is intentionally coordinate-stable: a delta never inherits a neighboring object's ID. The regenerated base source owns that coordinate even when the delta changes its material. Collision and public query truth remain the unpartitioned current voxel field; only disposable normal-world presentation is routed. Raw-voxel diagnostics remain an explicitly enabled overlay/replacement and are not a second normal presentation owner.

## Auditor Turn — 2026-07-13T16:19:54Z

Mode: continue

Responding to: 2026-07-13T16:17:01Z

### Prior Findings Status

1. Status: resolved
   Prior finding: Non-ruin object voxels and terrain chunks lacked an exclusive provenance-based presentation owner.
   Evidence: `docs/tdd/data-model.md` — Objects, ruin, and dressing defines the pure `solid_presentation_owner(current, base_source)` partition and stable base provenance; `docs/tdd/rendering.md` — Terrain extraction assigns each global-solid primitive to the solid endpoint's owner and applies disjoint payload filters at every LOD; `docs/tdd/systems.md` — Terrain meshing systems and Object and dressing systems require the filtered payload union to equal global extraction exactly once and cover intact/edit/boundary/revert barriers.
   Notes: Terrain/ruin cells now route only to terrain chunks, non-ruin object cells route only to their stable object root, and collision/query truth remains unpartitioned. The previous missing ownership and delta-attribution contract is resolved.

### New Findings

1. Severity: major
   Origin: emergent
   Section: `docs/tdd/data-model.md` — Geological feature / Objects, ruin, and dressing; `docs/tdd/systems.md` — Object and dressing systems
   Issue: The new once-only ownership invariant does not hold for intact overlapping non-ruin objects. Base evaluation explicitly permits two analytic object shapes to overlap and assigns each shared coordinate only to the lowest `ObjectId`, but activation still gives every unedited placement its complete shared authored GLB. An authored GLB is not owner-filtered per voxel, so the higher-ID intact root can display the overlapped portion that voxel truth and `solid_presentation_owner` assigned to the lower ID. The owner-filtered extraction tests cover derived primitives, not this intact-GLB case.
   Evidence: `docs/tdd/data-model.md` states that the lowest `ObjectId` wins overlapping non-ruin shapes and later says an intact authored root represents all cells routed to its ID. `docs/tdd/systems.md` spawns one authored root for each active intact non-ruin placement, while its overlap property seeds two overlapping sources but asserts only disjoint owner-filtered primitive sets from extraction. No manifest validation rejects shape overlap and no rule forces an overlap-clipped object into voxel-derived presentation. This became visible only after the prior fix made lowest-ID overlap precedence and exclusive presentation ownership explicit.
   Required change: clarify contract by making overlap handling compatible with intact presentation. Either reject voxel-shape overlap between non-ruin placements during manifest validation/curation and test that invariant, or require every overlap-affected placement to use an owner-filtered voxel-derived root from initial activation (with an exact lifecycle rule). Add an intact-overlap test proving that no authored root displays geometry attributed to another object.

### Questions For Coder

1. Is non-ruin object overlap intended to be invalid curated metadata, or should overlap-affected objects start in owner-filtered voxel-derived presentation?

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

The provenance partition itself and its edit/reversion routing are now implementable. This finding is limited to the newly explicit overlap-precedence branch, where a full intact GLB cannot honor a partial per-coordinate ownership mask; either permitted-data validation or the initial visual state can close it without changing voxel truth.

### Approval

Not approved.

## Coder Turn — 2026-07-13T16:24:03Z

Mode: continue

Responding to: 2026-07-13T16:19:54Z

### Responses To Findings

1. Addressed by making raw voxel-shape overlap invalid curated metadata. Manifest validation now rejects two non-ruin shapes that share a solid voxel and rejects a non-ruin shape intersecting any authored ruin-stamp coordinate. Validation completes before readiness or visual spawning, so an intact authored root can never display cells assigned to another placement.

### Answers To Auditor

1. Non-ruin object overlap is intended to be invalid curated metadata. The deterministic curator retries placement candidates until both the configured density/count contracts and exact disjointness pass; a supplied conflicting manifest fails world opening rather than switching either object into an implicit initial state.

### TDD Changes Made

- Revised `data-model.md` to define exact pair/ruin overlap predicates, stable coordinate-bearing errors, validation-time index construction, and the accepted-manifest invariant that makes an unmasked intact root safe.
- Revised `systems.md` with bounded grid-candidate/exact-voxel validation, failure-before-spawn behavior, generation/validation properties, and a headless intact-overlap test that proves a rejected world spawns no authored root, derived root, or terrain chunk.
- Revised `rendering.md`, `config.md`, and `assets.md` so deterministic candidate retry preserves dense-forest requirements and the import/curation pipeline independently verifies disjointness.

### Questions For Auditor

None.

### Notes To Auditor

The evaluator retains lowest-ID/ruin precedence only to make curator candidate evaluation and malformed-input diagnostics deterministic. That branch is unreachable in an opened world because overlap validation runs before `BaseRegion` exposure or `WorldReady`. Terrain/object collision and query semantics are otherwise unchanged.

## Auditor Turn — 2026-07-13T16:27:57Z

Mode: continue

Responding to: 2026-07-13T16:24:03Z

### Prior Findings Status

1. Status: resolved
   Prior finding: Intact overlapping non-ruin objects could display authored geometry for cells attributed to another placement.
   Evidence: `docs/tdd/data-model.md` — Seed, region, and generated metadata rejects exact non-ruin solid-shape overlap and non-ruin intersection with any authored ruin-stamp coordinate before retaining the object index; `docs/tdd/config.md` — Biome, object, and route constraints makes the deterministic curator reject conflicting candidates while still requiring configured counts/density; `docs/tdd/systems.md` — Generation systems validates the same invariant before `WorldReady` or any visual spawn and adds coordinate-bearing rejection/headless no-spawn tests; `docs/tdd/rendering.md` — Registered objects and forest scale limits intact authored roots to accepted disjoint manifests.
   Notes: The deterministic precedence branch is now explicitly diagnostic/curation-only and unreachable in an opened world. Accepted intact shapes are not partially masked by another registered placement, resolving the prior authored-root ownership conflict.

### New Findings

1. Severity: major
   Origin: previously_missed
   Section: `docs/tdd/data-model.md` — Objects, ruin, and dressing; `docs/tdd/systems.md` — Object and dressing systems; `docs/tdd/rendering.md` — Terrain extraction / Registered objects and forest scale
   Issue: The intact-object state predicate still ignores terrain-owned deltas that change an object's exposed global boundary. A non-ruin root remains `Intact` whenever no delta changes a base cell attributed to that object, even if an adjacent terrain/ruin-attributed place or dig changes the global solid topology and should occlude, expose, or reshape owner-filtered object primitives. Because the intact GLB is unfiltered, it cannot follow that boundary. The same predicate can restore the full GLB after object cells are reverted while a boundary-affecting terrain delta remains. The TDD therefore promises stencil dirtying and once-only current-truth presentation but defines no transition for this reachable case.
   Evidence: `docs/tdd/data-model.md` defines `Intact` solely as having no current delta on an object-attributed base cell, while its dirtying contract explicitly discovers `Object(ObjectId)` owners in the extraction stencil of a terrain-attributed changed cell. `docs/tdd/systems.md` likewise scans the complete stencil but performs `Intact -> VoxelDerived` only after finding an object-attributed delta; it says only an already-derived root is rebuilt for adjacent terrain-owned edits. `docs/tdd/rendering.md` generates topology from the global solid scalar, so an adjacent placed/removed terrain cell can change primitives assigned to the object owner, yet its lifecycle again removes the authored root only when an edit changes an object-attributed voxel. The design requires visible surfaces and edits to derive from current material truth (`docs/design-document.md` — Present smooth material truth / Dig or place). This existed beside the prior partition fix but became concrete in this re-review while checking whether the new intact-GLB validation made every reachable intact state safe.
   Required change: clarify contract by basing intact/restore eligibility on whether the current owner-filtered object surface still matches its base/authored presentation, including boundary-affecting terrain/ruin deltas in the extraction stencil, rather than only on object-attributed delta presence. Transition an intact root to the revisioned owner-filtered mesh whenever such a neighboring delta changes its emitted/occluded boundary, and restore the GLB only after all relevant object and neighboring deltas cease to affect that surface. Add headless/extraction tests for an adjacent terrain-only place/dig and for object edit -> adjacent terrain edit -> exact object-cell revert, asserting no stale/full GLB and correct `EditSurfaceReady` acknowledgement.

### Questions For Coder

1. Is the intended intact-state condition surface equivalence to the base authored object, so a terrain-owned edit that changes the object's exposed boundary also forces `VoxelDerived` until that boundary returns to base?

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

The new manifest-overlap validation is deterministic, bounded by the object index, runs before readiness/spawning, and has the required failure tests. The remaining issue does not require changing truth attribution or overlap policy; it only broadens the authored/derived visual-state predicate to cover the adjacent-delta branch that the existing stencil scan already identifies.

### Approval

Not approved.

## Coder Turn — 2026-07-13T16:32:55Z

Mode: continue

Responding to: 2026-07-13T16:27:57Z

### Responses To Findings

1. Addressed. Authored eligibility is now based on the complete `ObjectSurfaceDependency(id)`: the extractor-declared union of object and adjacent terrain/ruin inputs that can affect that object's owner-filtered surface at any supported LOD. Any delta in that set forces `VoxelDerived`, regardless of provenance, and the GLB can return only after every dependency delta exactly reverts. This covers adjacent terrain-owned place/dig and prevents object-cell reversion from restoring a stale full GLB while a boundary delta remains.

### Answers To Auditor

1. Yes. Surface equivalence to the base authored presentation is the intended condition. The implementation uses the conservative, testable sufficient predicate that every coordinate in `ObjectSurfaceDependency(id)` equals base; it does not attempt a fragile runtime triangle comparison.

### TDD Changes Made

- Revised `data-model.md` to define `ObjectSurfaceDependency`, make it the sole intact/restore predicate, include adjacent terrain/ruin inputs, and route edit dirtying through the same extractor footprint.
- Revised `systems.md` so activation, save/load reactivation, edit refresh, and exact reversion all use that predicate; extended the object index for dependency-coordinate lookup and required render-extraction acknowledgement for every transition.
- Revised `systems.md` tests for adjacent terrain-only place and dig, object edit -> adjacent terrain edit -> object-cell revert, full dependency reversion, and footprint completeness/outside-independence.
- Revised `rendering.md` and `config.md` to align presentation lifecycle and bounded index contracts with the dependency-aware rule.

### Questions For Auditor

None.

### Notes To Auditor

The predicate is intentionally conservative: a delta in the declared extraction dependency may select the voxel-derived root even when the resulting triangles happen to be unchanged. This preserves correctness and gives dirty discovery, activation, restoration, and tests one deterministic rule while leaving authoritative provenance, collision, and query truth unchanged.

## Auditor Turn — 2026-07-13T16:36:13Z

Mode: continue

Responding to: 2026-07-13T16:32:55Z

### Prior Findings Status

1. Status: resolved
   Prior finding: Intact non-ruin object presentation ignored adjacent terrain/ruin deltas that changed the object's exposed global boundary.
   Evidence: `docs/tdd/data-model.md` — Objects, ruin, and dressing defines `ObjectSurfaceDependency(id)` as every object, adjacent terrain/ruin, material, provenance, gradient, and downsampling input capable of changing the owner-filtered payload, and permits `Intact` only when none of those coordinates has a delta; `docs/tdd/systems.md` — Object and dressing systems uses that same predicate on activation, reactivation, refresh, and restoration and adds adjacent place/dig plus mixed-reversion lifecycle tests; `docs/tdd/rendering.md` — Registered objects and forest scale requires any dependency delta to replace the GLB with the current owner-filtered derived mesh and restores it only after the entire dependency returns to base.
   Notes: The revised predicate covers terrain- and ruin-attributed boundary inputs as well as object-attributed cells. The stated tests verify both the missing adjacent-edit transition and the case where object cells revert while a neighboring delta remains, so the prior correctness/readiness gap is resolved.

### New Findings

1. Severity: major
   Origin: regression
   Section: `docs/tdd/data-model.md` — Seed, region, and generated metadata / Objects, ruin, and dressing; `docs/tdd/systems.md` — Generation systems
   Issue: The new dependency contract is represented as an eagerly retained, explicit sorted `VoxelCoord` set for every placement in the region, which defeats the sparse activation model at forest scale. `ObjectSurfaceDependency` must include every raw object cell plus all extractor halos/downsampling inputs because an edit to any such cell can change the derived payload. Materializing those coordinates for every tree, bush, and prop during manifest validation makes startup work and memory proportional to all 25 cm object voxels, even though most objects are inactive; the claimed object-index memory bound counts only 16 grid entries per placement and omits these retained sets. An implementation following the TDD can therefore pay hundreds of megabytes of coordinate storage and tens of millions of coordinate derivations before `WorldReady`, conflicting with the design's memory-traffic/sparse-wilderness constraints and the five-second cold-start target.
   Evidence: `docs/tdd/data-model.md` says runtime builds and retains the index during validation, calls each dependency a finite sorted coordinate set, and says the dependency sets are derived memory; it also requires the set to contain object base cells and every supported-LOD extraction input. `docs/tdd/systems.md` inserts the union AABB of each raw shape and derived dependency during validation, exact-filters point lookups against the set, yet states index memory is bounded only by 16 entries per placement plus cell headers. `docs/tdd/config.md` requires at least 120,000 m2 of forest with 5 m tree spacing and analytic tree canopies up to 4 m radii at 25 cm voxel resolution, so Product One contains thousands of placements whose individual raw/dependency domains can contain thousands to tens of thousands of voxel coordinates. `docs/design-document.md` — Present smooth material truth / Platform and portability requires only needed areas to become fully active, sparse untouched matter from the first milestone, and memory traffic as the primary development-machine limit. This regression appears now because the latest fix introduced the region-wide per-object dependency-set representation.
   Required change: clarify contract by retaining a compact dependency footprint per placement (for example bounded AABBs plus a pure analytic `dependency_contains(id, coord)` predicate) rather than materializing every coordinate for every inactive object. Define a bounded delta-intersection query for authored eligibility using the brick-delta/index structures, preserve the same shared extractor-footprint semantics for dirty discovery and snapshots, and state an explicit startup/index memory bound that includes all retained dependency metadata. Add an equivalence property against the explicit-set oracle on small shapes and a generated full-manifest test/benchmark proving dependency-index construction remains within the cold-start and memory budgets.

### Questions For Coder

1. Is `ObjectSurfaceDependency` intended as a mathematical set queried through a compact/lazy membership predicate, rather than a materialized coordinate vector retained for every placement? If so, please make that representation and its delta-intersection complexity explicit.

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

The dependency-aware lifecycle itself is correct and should remain the single eligibility/dirtying rule. This finding is about making that rule viable for the full curated object population; it does not require weakening the adjacent-boundary coverage or changing presentation ownership.

### Approval

Not approved.

## Coder Turn — 2026-07-13T16:42:25Z

Mode: continue

Responding to: 2026-07-13T16:36:13Z

### Responses To Findings

1. Addressed. `ObjectSurfaceDependency` remains the single lifecycle/dirtying rule but is now a mathematical set evaluated by an exact lazy analytic predicate. Runtime retains only fixed-size raw/dependency AABBs per placement and a shared bounded extractor stencil; it never expands or retains per-object dependency coordinates. Authored eligibility probes only sparse delta bricks overlapped by an active object's compact footprint.

### Answers To Auditor

1. Yes. `ObjectSurfaceDependency` is intended as a mathematical set, not a materialized coordinate vector. Exact membership is `exists stencil offset whose inverse coordinate is inside the placement's analytic raw shape`; a conservative AABB provides the spatial broad phase. `delta_intersects_dependency` probes at most 128 brick keys in the sparse delta map and tests only present deltas, with `O(b log D + m*s)` worst-case work and `O(1)` allocation.

### TDD Changes Made

- Revised `data-model.md` with the compact index record, exact lazy membership predicate, test-only explicit-set oracle, bounded sparse-delta intersection, and a complete retained-byte accounting formula.
- Revised `systems.md` so index construction is constant-space per placement, changed-coordinate discovery is a bounded batched query, only active objects pay eligibility probes, and properties compare lazy membership/queries against explicit brute-force oracles.
- Revised `config.md` with 128 dependency-brick and 16 MiB complete-index caps; revised `rendering.md` to forbid retained dependency coordinates.
- Revised `benchmarks.md` and benchmark data contracts so the full-manifest startup reports index build time, retained bytes, counts, maximum footprint, and zero dependency-coordinate allocation inside the existing five-second cold-start window.
- Revised `overview.md` to make compact forest dependency metadata part of the sparse-wilderness runtime contract.

### Questions For Auditor

None.

### Notes To Auditor

The explicit sorted coordinate set now exists only behind `cfg(test)` for small-shape equivalence properties. Normal manifest validation computes analytic bounds and grid entries only. The surface-equivalence lifecycle, adjacent terrain/ruin coverage, presentation ownership, and exact-reversion behavior from the prior turn are unchanged.

## Auditor Turn — 2026-07-13T16:45:36Z

Mode: approved

Responding to: 2026-07-13T16:42:25Z

### Prior Findings Status

1. Status: resolved
   Prior finding: Region-wide materialized `ObjectSurfaceDependency` coordinate sets defeated sparse activation and omitted their startup/memory cost from the object-index bound.
   Evidence: `docs/tdd/data-model.md` — Seed, region, and generated metadata retains only fixed-size raw/dependency bounds and explicitly accounts record, grid, member capacity, keys, and allocator padding under a 16 MiB cap; Objects, ruin, and dressing defines the dependency as a lazy mathematical set, bounds the shared stencil at 512 offsets, probes at most 128 sparse delta bricks with `O(1)` allocation, and confines explicit coordinate enumeration to a `cfg(test)` oracle. `docs/tdd/systems.md` — Generation systems constructs constant-space records without enumerating dependency coordinates, bounds changed-ID discovery, and requires lazy/oracle equivalence plus a full-manifest zero-coordinate-allocation test. `docs/tdd/benchmarks.md` — Run lifecycle makes build time, retained bytes, counts, maximum footprint, and dependency-coordinate allocation mandatory startup telemetry inside the existing five-second readiness window. `docs/tdd/config.md` — Biome, object, and route constraints makes the 128-brick and complete 16 MiB retained-index limits validated configuration contracts.
   Notes: The revised representation preserves the complete adjacent-input dependency semantics while making inactive-object metadata constant-size per placement. Eligibility work is restricted to active objects and sparse delta bricks, retained memory has an explicit inclusive accounting rule, and both equivalence properties and full checked-in-manifest acceptance telemetry cover the correctness and viability risks raised previously.

### New Findings

No new findings.

### Questions For Coder

None.

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

The compact dependency footprint, exact lazy membership, sparse-delta eligibility query, dirty-discovery contract, runtime lifecycle, tests, configuration caps, and startup evidence now agree across the TDD. The prior ownership, overlap-validation, adjacent-boundary, reversion, and readiness contracts remain intact.

### Approval

Approved.

## Auditor Turn — 2026-07-13T18:10:31Z

Mode: continue

Responding to: 2026-07-13T16:45:36Z

### Prior Findings Status

1. Status: resolved
   Prior finding: Region-wide materialized `ObjectSurfaceDependency` coordinate sets defeated sparse activation and omitted their startup/memory cost from the object-index bound.
   Evidence: `docs/tdd/data-model.md` — Seed, region, and generated metadata retains fixed-size raw/dependency bounds and two bounded indices under the inclusive 16 MiB cap; Objects, ruin, and dressing defines lazy `dependency_contains` and an at-most-128-brick sparse-delta probe without retained dependency coordinates. `docs/tdd/systems.md` — Generation systems constructs these records without enumerating coordinates and requires explicit-oracle equivalence plus full-manifest timing/allocation evidence. `docs/tdd/config.md` and `docs/tdd/benchmarks.md` retain the 128-brick, zero-coordinate-allocation, startup, and complete-index acceptance limits.
   Notes: The current files still consistently implement the compact mathematical-set interpretation approved in the prior turn. No regression was found in its retained-memory accounting, eligibility query, or verification contracts.

### New Findings

1. Severity: major
   Origin: emergent
   Section: `docs/tdd/rendering.md` — Terrain extraction / Registered objects and forest scale; `docs/tdd/systems.md` — Object and dressing systems; `docs/tdd/config.md` — Streaming and task configuration
   Issue: The edited-object current-truth lifecycle is defined only for per-placement authored or voxel-derived roots, while the Horizon band makes cell-level tree cards/clusters the exclusive non-ruin presentation. The TDD does not define whether a horizon aggregate excludes, rebuilds, or substitutes a placement whose `ObjectSurfaceDependency` has a persisted delta. Consequently an edited or fully removed tree can legally reappear as its base silhouette after the player moves away, after eviction/reactivation, or after save/load into a view where that placement is represented by a cluster. The aggregate also has no revision/dirty/readiness ownership rule comparable to the per-object root, so the stated exactly-once presentation partition cannot be proved across the Far-to-Horizon transition.
   Evidence: `docs/tdd/rendering.md` — Terrain extraction says Horizon object cards/clusters are the exclusive non-ruin presentation in that band, and Registered objects and forest scale allows generated cluster/impostor cards per spatial cell; the same section defines dependency-delta transitions only as authored per-object root to owner-filtered per-object mesh. `docs/tdd/config.md` sets individual object visibility to 320 m and Horizon tree clusters to 720 m, making the aggregate path part of the required four-band presentation rather than unreachable optimization. `docs/tdd/systems.md` — `spawn_registered_object_visuals`, `refresh_modified_object_visuals`, and their tests guarantee exactly one authored/derived root per active placement but never specify cluster membership, invalidation, revision matching, exact reversion, or save/load reactivation for edited placements. `docs/design-document.md` — Present smooth material truth and Save and load require visible surfaces to derive from current voxel truth and restored edits exactly. This became visible in this re-review because the prior fixes made the per-object dependency lifecycle precise enough to trace its consumers through every distance band; the Horizon consumer remains outside that lifecycle.
   Required change: clarify contract by defining a revision-aware Horizon policy for placements with dependency deltas. For example, rebuild the affected cell aggregate while excluding ineligible base cards and supply a current-truth derived horizon representation for each excluded placement, or retain a bounded per-placement derived LOD instead of aggregating it. Specify cluster dirty discovery, activation/eviction/reactivation and exact-reversion behavior, spatial/provenance ownership, and when an active aggregate participates in `EditSurfaceReady`. Add transition tests covering intact -> edited -> Horizon, Horizon eviction/reactivation, save/load directly into Horizon, full object removal, and exact dependency reversion, asserting that base cards do not resurrect edited truth and that aggregate plus derived payloads contain each visible non-ruin placement exactly once.

### Questions For Coder

1. When a tree with a persistent dependency delta enters the Horizon band, should its base card be removed from the cell cluster and replaced by an owner-filtered derived horizon payload, or should edited placements remain individual derived roots at a bounded coarse LOD?

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

The compact dependency/index work remains approved. This finding does not require materializing dependencies or changing voxel provenance; it requires carrying the already-defined eligibility result into the cell-level Horizon representation so distance-band transitions cannot resurrect base object visuals.

### Approval

Not approved.

## Coder Turn — 2026-07-13T18:19:30Z

Mode: continue

Responding to: 2026-07-13T18:10:31Z

### Responses To Findings

1. Addressed. Horizon is now governed by the same `ObjectSurfaceDependency` eligibility as per-placement LODs. Each revisioned 64 m cell partitions its sorted visible tree IDs into intact base cards and excluded per-ID owner-filtered 4 m derived payloads; a fully removed tree receives an empty ownership tombstone and cannot regain a card. Cell tokens/source revisions, atomic group installation, dirty discovery, edit pinning/readiness, Far/Horizon transition, eviction/reactivation, load invalidation, and exact reversion are explicit.

### Answers To Auditor

1. The base card is removed from the cell cluster and replaced by an owner-filtered derived Horizon payload. This keeps intact trees aggregated while making edited trees current-truth and independently revisioned; a fully removed tree has an acknowledged empty tombstone. Edited placements do not remain at their Near/Far root LOD in Horizon.

### TDD Changes Made

- Revised `data-model.md` with anchor-owned `HorizonCellKey`, exact base/derived partition and tombstone invariants, revision/token/cache rules, dirty flags, report evidence, and a 1,024-tree validation cap.
- Revised `systems.md` and `states.md` with bounded cell enumeration, current-delta partition construction, atomic install, relevant-edit/load invalidation, pin/evict/reactivate behavior, stale-result rejection, and the requested transition/removal/reload/reversion tests.
- Revised `rendering.md`, `assets.md`, and `config.md` so Horizon cards are base-only shared assets, edited trees use dynamic owner-filtered 4 m payloads, non-tree props cull at 320 m, and no filtered aggregate can be reused by cell key alone.
- Revised `api.md`, `overview.md`, `implementation-plan.md`, and `benchmarks.md` so active Horizon batches participate in `EditSurfaceReady`, Gate F2 forces this path, and feasibility evidence records the partition and cell-cap metrics.

### Questions For Auditor

None.

### Notes To Auditor

The compact dependency/index design is unchanged: Horizon membership enumeration reuses the existing 32 m dependency grid, stores no third runtime index or dependency coordinate sets, and validates the per-cell count in one sorted manifest pass. The logical exactly-once invariant covers every Horizon-visible tree; bushes and props are intentionally outside that set because their configured visibility ends at 320 m.

## Auditor Turn — 2026-07-13T18:22:51Z

Mode: approved

Responding to: 2026-07-13T18:19:30Z

### Prior Findings Status

1. Status: resolved
   Prior finding: Horizon tree aggregates could resurrect base silhouettes for edited or removed trees because they were outside the per-placement dependency/revision lifecycle.
   Evidence: `docs/tdd/data-model.md` — Objects, ruin, and dressing defines an anchor-owned `HorizonObjectCell` whose sorted `base_card_ids` and revisioned `derived` records form an exact disjoint partition, with ineligible or fully removed trees represented only by owner-filtered payloads or empty tombstones; `docs/tdd/systems.md` — Object and dressing systems rebuilds affected desired/resident cells from the current delta snapshot, rejects stale token/revision results, pins active edit work, invalidates affected cells on load, and specifies transition/removal/eviction/reactivation/load/reversion tests; `docs/tdd/rendering.md` — Registered objects and forest scale makes the filtered cell payload the exclusive Horizon tree presentation and permits only immutable base descriptors to survive cell eviction; `docs/tdd/api.md` — World edit protocol includes aggregate membership changes, derived replacements, and tombstones in `EditSurfaceReady` through GPU prepare/free and render-queue acknowledgement.
   Notes: The revised contract carries the existing `ObjectSurfaceDependency` eligibility result into Horizon rather than inventing a second truth rule. It covers activation, Far-to-Horizon handoff, edits while resident, full removal, eviction/reactivation, save/load, exact dependency reversion, stale work, and exactly-once membership. The bounded cell cap and Gate F2 active-Horizon stress path make the aggregate lifecycle implementable and acceptance-testable without adding a region-wide index or retained dependency coordinates.

### New Findings

No new findings.

### Questions For Coder

None.

### Answers To Coder

No coder questions were pending for this turn.

### Notes To Coder

The requested revision-aware Horizon policy is now consistent across the data model, runtime scheduling, rendering ownership, state transitions, public edit/load barriers, assets, configuration, feasibility gates, and benchmark evidence. The previously approved compact dependency representation and adjacent-boundary correctness remain intact.

### Approval

Approved.

All issues resolved. TDD approved.
