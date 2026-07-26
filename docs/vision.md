# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates).

An adjacent **walkable-world executable** is a validation harness only. It must consume the substrate through the same public interfaces an external game would use. It is not a game layer and must not own privileged or game-specific paths.

**Product One — “The Walkable World”** is the first delivery slice of that substrate plus the harness: one curated generated region, fused visible proof of a mutable material world, and measurable validation. It is not the whole of Moria; it narrows *what ships first*, not *what Moria is for*.

The actual game (or games) that will sit on this substrate are **downstream consumers outside this repository**.

---

## Purpose

Make one claim undeniable and reusable:

> This is not a heightmap with props — it is a fully material voxel world that reads as a normal natural world, and it is engineered as a clean substrate for other products.

Moria exists so future games can share one matter foundation: continuous 3D terrain and geology, smooth presentation over voxel truth, mutation and queries everywhere, deep underground as real content, material and environmental behavior that consumers can feel, sparse streaming, and durable world continuity across runs. Game rules stay above the substrate; the substrate provides the world they stand on.

Product One’s job is narrower and concrete: prove a first slice of that foundation as a product-shaped demo — walk it, cut it, reload it, and measure it — so later products start from evidence rather than a whiteboard. Capabilities deferred from Product One remain **Moria outcomes**, not optional future flavor.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| The reusable voxel-world substrate (generation, matter, presentation views over voxel truth, mutation/query surface, and the consumer-visible matter/physics outcomes the substrate owns) | Game rules, combat, stats, AI, economy |
| Walkable-world executable as **public-API validation harness** only | The System / LLM, spells, gas pricing, intent layers |
| Compatibility *seams* where substrate requirements demand them | Building *gameplay* (UI, blueprints, mechanisms, work orders) — not implemented here |
| Cargo workspace separation between reusable crates and the harness | Full ARPG, fortress/colony, or descent-roguelike products and their loops |
| Load-bearing substrate functionality that stays **portable** across the intended GPU backend classes (not a machine-tied or native-backend fork) | Game-mode loops, content, characters, and consumer-specific progression |

**Layering intent (product-level, not a crate map):** generation and matter are substrate; script/API verbs and mirror queries exist so nothing above the matter layer touches voxels directly; semantic and game layers belong to consumers. Gas, if ever present, is a policy injected by a consumer — not a Moria feature.

**Product One layering (binding first delivery):** the **generation layer ships full** for the curated region — continent parameters may be stubbed to that region’s curated seed, but columns, strata, caves, ore, lazy materialization, and POI metadata ship as designed. This generation layer is the reusable asset; it is not a partial slice. The **matter layer ships partial** (e.g. static water bodies without dynamic flow; placed voxel objects without felling). Partial matter is a first-delivery exclusion, not a demotion of deferred matter outcomes out of Moria’s mandate.

**Portability constraint (product-level):** load-bearing substrate functionality must remain portable across the intended GPU backend classes rather than becoming tied to the development machine or a native backend fork. Machine-specific performance numbers and platform ceilings are validation targets, not product identity.

---

## Required product-level outcomes

### Moria (enduring substrate mandate)

These are fused consumer-visible outcomes Moria must provide as substrate. Mechanisms are not enumerated here; implementation of some may arrive after Product One.

1. **Material world, not scenery.** The world reads as a normal natural place — terrain, forests, water, cliffs, meadows — while the voxel grid remains the truth underneath, not the look.
2. **Mutable everywhere, all the way down.** Any part of the material world can be destroyed, moved, or placed; nothing important is decorative geometry outside that truth.
3. **Deep Z is first-class.** Underground is content: continuous 3D play space, geology, voids, and depth as real volume — not a skybox floor.
4. **Presentation is a view.** Smooth, material-aware meshing makes the world read as terrain (and sharp cuts as cuts); collision, queries, and gameplay always run against voxel truth, never against the mesh as authority.
5. **Interactive voxel-backed objects.** Things that can burn, break, fall, or block participate as matter (trees, boulders, and similar) rather than as static props frozen out of the world system. Objects are **lifecycle-capable** (growth and stage change over time) as substrate behavior, not only as placed static stamps.
6. **Voxel-responsive dressing.** Surface clutter that is not individually simulated still tracks the matter world — it appears, changes, and vanishes with the voxels it depends on.
7. **Material and environmental behavior.** The living world can respond in consumer-visible ways: fire and wetness-class propagation, granular materials that settle, and related ambient honesty so underground and surface play feel consequential.
8. **Time- and environment-driven material change.** Thin but present ambient substrate behavior — day/night and seasonal drivers, weather-driven material effects (wetness, accumulation, ignition cues), and fire ecology — so the surface world *behaves* as a living place. Weather *gameplay*, seasonal *progression*, and authored ambient loops remain downstream consumer concerns; the substrate still owns the material and lifecycle responses those systems drive.
9. **Fluid response.** Water and similar bodies are part of the material world — still volumes at minimum, with dynamic flow and interaction as substrate capability so consumers can drain, flood, and breach without a separate water engine.
10. **Structural consequences.** Unsupported structure fails honestly; digs and builds have load-bearing consequences consumers can read and exploit.
11. **Sparse streaming.** Large regions stay tractable: only interesting volume costs resident work; cold world stays cheap under sparsity.
12. **Durable delta-based world continuity.** Truth is generation plus edit deltas (and related object/entity change journals as needed). Scars and construction persist across runs so one world’s edits can be reused later — including, when consumers choose, cross-mode reclaim. Fortress/ARPG *loops* are downstream; the substrate’s ability to keep a changed world durable is not.
13. **Portable GPU-resident substrate.** Core load-bearing functionality remains portable across the intended backend classes; development-machine ceilings inform validation, not a fork of product identity onto one platform.

**Queries and mutation as the only door.** Nothing above the matter layer touches voxels directly. Verbs, mirror queries, and events are the shared boundary for harness, games, and future agents.

### Product One (adjacent first-slice validation)

Product One succeeds when the **first slice itself** demonstrates a fused visible proof of the material-world claim — not a deferred checklist under the general Moria mandate — through a generated-region walkable demo and measurable validation.

**Fused first-slice outcome families** (content counts, control bindings, exact dimensions, and mechanisms belong to design, not here):

1. **Full geology-first generation for the curated region.** Columns, strata, caves, ore, lazy materialization, and POI metadata ship; the region is generated geology, not a heightmap with rock painted under props. Continent-scale parameters may be stubbed to this seed’s curated values; the reusable generation outcomes above must not be cheapened.
2. **Smooth natural surface over voxel truth.** The walkable surface reads as continuous terrain while remaining backed by the material grid.
3. **Sharp mutable cuts.** Dig/place as proof (not as a game system) produce cut faces that read as cut earth; mutation is visible and honest mid-traversal.
4. **Dressing that tracks matter.** Surface clutter and scatter respond to the voxels they depend on rather than floating as independent scenery.
5. **Placed voxel-backed objects.** Trees, boulders, and similar occupy the object layer as matter participants (placement, registration, rendering). Felling and rigid conversion remain stretch for this slice, not a Product One acceptance bar.
6. **Static water bodies.** Still lakes and river channels with water surface (tier-1 bodies); no dynamic flow sim in the first ship.
7. **Deep geological and cave traversal.** Continuous play from surface into walkable underground volume so deep Z is experienced, not asserted.
8. **Collision against voxel truth.** Traversal collides with occupancy truth, not with the render mesh as authority — proving the mesh is a view.
9. **Public-interface harness.** Third-person traversal and debug exercise the substrate only through the same public dig/place and query surface an external game would use; the harness does not bypass the crate boundary.
10. **Measurable validation.** The first slice is complete only when it also demonstrates, at vision altitude: **responsive traversal and mutation**, **bounded startup and resident memory under sparsity**, **exact restoration of edits** from a seed-plus-deltas save, and **comparable measurement** so later substrate changes can be judged against this baseline.

Product One’s partial **matter** slice may omit dynamic fluids, cellular automata–driven behavior, structural integrity, granular settling, weather/seasons/growth *as running systems*, and rigid-object conversion from the *first delivery*. Format and architecture must not foreclose them; they remain Moria outcomes above. Rigid-tree conversion is stretch for Product One, not a Product One acceptance bar.

Exact controller verb lists, debug-view inventories, seed-content checklists, storage/meshing implementation choices, and benchmark-scene composition are design and validation details — not product-identity outcomes.

---

## Non-goals

Explicitly **not** in this repository / not part of Moria’s product identity:

- Combat, RPG stats, entities as game characters beyond harness needs, or any AI
- The System, LLMs, spells, gas metering, or intent/pricing policy
- Building as a *product*: placement UI, blueprints-as-gameplay, mechanisms, rooms/work orders
- Weather-as-game-system, seasons-as-progression, or full ambient *gameplay* loops as shipped consumer features (ambient *material* drivers remain Moria outcomes; see above)
- Embedded scripting languages for content authors (as a Product One / near-term requirement)
- Implementing multiplayer *play* or network session products in this repo
- Implementing any full game mode (ARPG, fortress, descent roguelike) or those modes’ loops in this repo

**First-delivery exclusions (Product One only — not Moria non-goals):**

- Dynamic fluid simulation beyond static bodies
- Running fire/wetness CA, granular settle, or structural integrity in the first ship
- Weather sim, seasons, and growth as *active* Product One systems (a fixed time-of-day control is enough for the demo)
- Voxel-object felling and rigid-body conversion (stretch for Product One)
- Persistence beyond seed + deltas for the first save surface (no multi-save versioning, no cross-mode reclaim *product* — while the substrate still owns durable delta continuity as an outcome)

Reference material in the substrate seed describing future games and richer sim is context for **what Moria must eventually enable**. Product One does not implement those games or the full sim surface; it also does not redefine them out of Moria’s mandate.

---

## Unresolved questions for humans

1. **Multiplayer-readiness vs multiplayer implementation.** Seeds agree multiplayer is not built here. The substrate seed explicitly leaves open whether **server-authoritative readiness** (verb/command architecture kept multiplayer-compatible even without a network product) belongs in scope *statements*. Should Moria’s product identity claim multiplayer-readiness as a load-bearing constraint, treat it as an architectural preference only, or leave readiness out of identity until a consumer demands it?

Remaining choices that do **not** reopen product identity (and belong to design, measurement, or ops): voxel size, LOD strategy, object-registry scale, naming presentation, harness shell evolution, publication timing, and machine-specific performance gates used only as validation baselines.

---

## Seed contributions (traceability)

| Seed | Role in this vision |
|---|---|
| **`README.md`** | Names the product (Moria), states substrate-as-crate and harness-as-consumer, and points at the seed set. |
| **`docs/seeds/project-boundary.md`** | **Binding identity and boundary:** reusable Rust substrate; game is out of repo; harness must use public APIs; System/LLM/spell/gas/combat/AI/building layers out of scope; workspace split required. Excludes game/System/building *layers*, not substrate-owned matter and physics outcomes. |
| **`docs/seeds/product-one-seed.md`** | **Binding first-slice validation:** Product One statement; **generation layer full** (strata, caves, ore, lazy materialization, POI metadata) and **matter layer partial**; fused demo proof (smooth surface, mutable cuts, dressing, placed objects, static water, deep traversal, collision vs voxel truth); harness/demo role; vision-altitude performance outcomes; portability across intended GPU backends (no native Metal fork in load-bearing layers); first-slice non-goals including weather/seasons/growth for Product One only. Narrows delivery order; does not redefine Moria’s mandate. |
| **`docs/seeds/voxel-world-substrate.md`** | **Binding substrate mandate for Moria:** consumer-visible outcomes for matter, mutation, deep Z, lifecycle-capable voxel objects, responsive dressing, material/environmental behavior, thin time/weather/fire-ecology ambient drivers, fluids, structural consequences, sparse streaming, and durable delta-based continuity (§§1, 5–8, 11). Layering rules and future game examples supply capability horizon and seams; §14 leaves multiplayer-readiness-in-scope-statements open. Gameplay, content, characters, and full later-layer implementation are not imported into current *delivery* scope, but the fused outcomes remain Moria’s job. |

**No blocking ambiguity** about which product is current: all seeds agree Moria is the substrate, Product One is the first deliverable slice, and game/System material is context for capability seams only. Product One’s absences are first-slice exclusions, not demotions of substrate outcomes out of Moria. The one open identity-adjacent question is multiplayer-readiness as a stated constraint, not which product is being built.
