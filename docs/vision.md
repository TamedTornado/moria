# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). Downstream games consume it; they are not built in this repository.

A **walkable-world executable** may ship with the repo. It is a consumer and validation harness for terrain generation, streaming, meshing, editing, collision, persistence, and performance. It must use the same public interfaces an external game would use—no privileged or game-specific paths inside the substrate.

The first product-shaped deliverable is a **vertical slice**: one generated natural region plus a third-person character that can run through continuous, mutable voxel terrain. Dig and place exist as debug proof that the world is material truth, not heightmap scenery. That slice proves the substrate; it is not a game.

---

## Purpose

Make one claim undeniable and reusable:

> This is not a heightmap with props—it is a fully material world that looks like a normal outdoor/underground landscape, and it is mutable everywhere.

The substrate exists so multiple future game modes (adventure, fortress/colony, pure sandbox, and similar) can share the same matter, queries, and mutation surface without baking any of those games into Moria. Performance numbers and a clean consumer API are part of the product, not afterthoughts: the customers are future game work and anyone evaluating the crate.

---

## Boundary

Two different boundaries matter. Conflating them would either shrink Moria to only the first slice forever, or import whole games into this repository.

### Outside this repository (permanently external)

These are not Moria’s product and must not be implemented here:

| Outside this repository |
|---|
| Game rules, combat, stats, AI, entities beyond a harness player |
| The System / LLM layer and content-authoring products |
| Spells, gas/economy policy, intent systems |
| Building *gameplay* layers: blueprints-as-gameplay, work orders, mechanisms as game systems, room/economy semantics |
| Any complete game (ARPG, fortress/colony, descent roguelike, sandbox product) on top of the substrate |

Compatibility *seams* may be designed where substrate requirements demand them (e.g. verb/query boundaries that future gas policy or multiplayer can plug into). Those layers must not be implemented here.

### Inside this repository

| In scope for the repository | Role |
|---|---|
| Voxel matter representation (bricks, sparsity, materials) | Core substrate |
| Geology-first generation and lazy materialization | Core substrate |
| GPU meshing and surface dressing as a *view* of voxel truth | Core substrate |
| Mutation and query API (dig/place and mirror-style reads) | Core substrate |
| Collision against voxel occupancy | Core substrate |
| Streaming rings, edit-delta persistence, harness benchmarks | Core substrate + validation |
| Cargo workspace separation: substrate crate(s) vs harness | Structural boundary |
| Further matter/API capabilities deferred from the first slice | Deferred substrate direction (below) |

### Outside the current slice (deferred substrate direction)

The Product One seed deliberately ships only the bottom generation layer, a partial matter layer, and a sliver of verb/query API. The architecture seed still describes additional substrate capabilities as *engine* work—not as game products. Format and API may anticipate them now; full behavior is not required for the first vertical slice.

These remain **on the repository’s substrate trajectory** unless a later human decision removes them. They are **not** “outside this repository”:

- Cellular automata and ambient matter rules (fire, wetness propagation, related state consumption)
- Structural integrity / support graphs and cave-in behavior
- Granular settle (sand, gravel, snow as matter rules)
- Fluid simulation beyond static tier-1 bodies (coarse brick flow, fine splash/particles)
- Tree/object felling and rigid-body conversion (placement and rendering of voxel objects stay in the first slice; physics coupling is deferred)
- Weather, seasons, and growth simulation (a fixed time-of-day control is enough for the slice)
- Richer script/API surface (embedded scripting, fuller priced-verb and event model beyond dig/place + mirror queries)
- Navigation aggregates and movement-class path data derived from bricks
- Multi-anchor streaming policies and cross-run delta reuse patterns beyond the slice’s single-save “seed + deltas” model

The first slice’s job is to prove material world, look, mutation, deep Z, sparsity, and the public API—not to ship the full matter sim stack.

**Not settled as repository roadmap:** substrate-side building primitives beyond proof dig/place (grid/stamp placement APIs, blueprint *format* as data) and multiplayer-oriented hardening of the command/mirror boundary are **not** defaulted onto the trajectory above. Compatibility *seams* for public verbs/queries may still be designed where required; whether fuller building-data primitives or multiplayer hardening belong in Moria at all is an open human question (see below).

---

## Required product-level outcomes

When this product is “done” for its first slice, the following hold at the product level—not as a feature checklist, but as what success means:

1. **Material world, normal look** — Generated terrain reads as rolling natural landscape (hills, rock, water, vegetation dressing) while remaining continuous voxel truth; the mesh is regenerated view, never authoritative.
2. **Mutable everywhere** — Any reachable volume can be destroyed or filled through the public mutation API; remesh stays responsive enough that mid-run carving feels live.
3. **Deep Z is real** — Underground volume is first-class: walkable descent, strata/geology that dig-down can expose, not a painted floor under a heightmap.
4. **Collision equals truth** — Character motion collides with voxel occupancy, not with the render mesh alone, so the mesh cannot lie.
5. **Sparsity and scale under load** — A region large enough that raw voxels cannot all live in memory streams and stays within the harness performance envelope (frame rate, cold start, resident memory, save/load deltas). Homogeneous/sentinel bricks and lazy materialization are load-bearing, not optional polish.
6. **Consumer-ready API boundary** — Nothing above matter touches voxels directly; harness and future games share the same verbs and queries. The substrate is usable as a crate without the harness.
7. **Measured credibility** — Benchmarked flythrough/carve scenarios with machine profiles; open substrate questions (e.g. voxel size, LOD, object-layer scaling) can be answered from this slice’s measurements rather than speculation.
8. **Portable GPU stack** — Load-bearing compute stays on portable GPU abstractions (wgpu/WGSL), with design constraints that match real shipping targets (including unified-memory / no-64-bit-atomic class limits on the primary dev platform).

---

## Non-goals

### Non-goals of this repository

- Implementing any game layer: combat, AI, NPC entities, ARPG camera fantasy, fortress labor, spells, gas pricing, LLM “System” behavior.
- Shipping a complete game product (adventure, fortress, descent, or sandbox) from this codebase.
- Treating the walkable harness as the long-term product surface or giving it private substrate hooks.
- Native Metal (or other platform) forks in load-bearing layers for short-term gain.

### Non-goals of the current slice only

These are deferred substrate direction (see Boundary), not permanent exclusions from Moria:

- Running full multiphase fluid CA, fire ecology, structural integrity cave-ins, granular settle, or tree-felling rigid bodies in the first deliverable—formats/APIs may anticipate them.
- Weather/seasons/growth sims beyond a simple time-of-day control.
- Building/mechanism *gameplay*, room semantics, work orders, multiplayer servers.
- Authored “one true world” content as product identity; the seed region is a curated validation route, not a shipping campaign.
- Persistence beyond reload-the-same-seed-plus-deltas (single save slot, no versioning) for the first slice.

---

## Unresolved human questions

The seeds agree on what Moria *is* (substrate + harness), what the first slice must prove, and that broader game/System layers are external. Two items appear in the architecture seed as substrate-adjacent but are **not** settled onto the repository roadmap here—human intent is required before treating either as in-scope direction.

### Product boundary (need human resolution)

1. **Building primitives / blueprint format beyond proof dig/place** — The architecture seed describes grid/stamp placement APIs and blueprints as sparse voxel stamps + material manifests (data/API, not fortress gameplay). Project boundary puts building *layers* out of scope and allows only compatibility seams. **Question:** Beyond the dig/place proof verbs already in the first slice, should Moria own substrate-side building primitives and a blueprint *format* as data, or should those stay outside Moria entirely (with only thin compatibility seams if needed)? Do not assume either answer from seed plausibility alone.

2. **Multiplayer-oriented hardening of the command/mirror boundary** — The architecture seed notes that verb/command architecture is server-authoritative-ready by construction and may be “worth keeping in scope statements even if not built.” Servers are not the product. **Question:** Should multiplayer-oriented hardening of the command/mirror boundary be part of Moria’s substrate trajectory at all, or is “ready by construction” / seam design enough without roadmap commitment to hardening work?

### Design and measurement (not vision blockers)

- Final voxel size (25 cm vs 12.5 cm or hybrid) for the benchmark region.
- Distant terrain LOD strategy under the harness camera.
- How far object-layer scale (trees/clutter counts) needs acceleration in the first slice.
- Which performance targets remain provisional until discrete-GPU baselines exist.
- Ordering and prioritization among settled deferred substrate capabilities after the first slice (integrity vs fluids vs CA, etc.)—implementation sequence, not product identity.

If product intent ever shifts from “crate + validation harness” to “shipping walkable demo as the primary product,” or from “no game systems” to “include a thin fortress/adventure mode,” that would rewrite this vision and should be an explicit human decision. If any *settled* deferred substrate items in Boundary should instead be cut from the repository roadmap entirely, that also needs an explicit human decision.

---

## Seed contributions (traceability)

| Seed | What it contributed to this vision |
|---|---|
| **README.md** | Names the product (Moria), states substrate-as-crate and harness-as-consumer, points at seeds as preserved inputs. |
| **project-boundary.md** | Hard repository boundary: substrate is the product; game is external; harness must use public APIs; game/System/LLM/spell/gas/combat/AI/building *layers* out of scope; workspace split between crate and harness. |
| **product-one-seed.md** | First deliverable identity (walkable vertical slice), product-level claim, slice non-goals, dig/place as proof not gameplay, performance and platform constraints as product outcomes, milestones as proof order—not imported as a content GDD or game design. Explicitly defers CA, integrity, granular, fluid tiers 2–3, felling physics, weather, and rich scripting while keeping formats ready. Specific seed-world features (ruin, ore types, tree species, etc.) are validation-scenario context for capabilities the substrate and harness must exercise, not current product lore. |
| **voxel-world-substrate.md** | Capability envelope and layering for the substrate over time (smooth meshing over density, brick sparsity, geology-first gen, lazy materialization, column index, dressing vs voxel objects, tiered fluids, integrity, nav/Z, streaming/persistence). Future games (ARPG, fortress, Moria descent) and System attachment points are context for *why* seams exist; their gameplay, content, characters, and full implementation are **not** current scope and remain outside the repository. Establishes that deferred matter/API work is still substrate direction. Building-data primitives/blueprint format and multiplayer hardening from this seed are **not** imported as settled roadmap—held as unresolved human questions. |

### Intentionally omitted or reclassified from seeds

- **Deferred (still substrate / this repo):** full CA, integrity/cave-ins, granular settle, fluid tiers 2–3, tree felling physics, weather/seasons/growth, richer script/API, nav aggregates, multi-anchor/cross-run streaming patterns—kept as deferred direction, not first-slice identity and not “external forever.”
- **Unresolved (not defaulted onto roadmap):** substrate building primitives / blueprint format beyond proof dig/place; multiplayer-oriented command/mirror hardening—present in the architecture seed, held as open human questions rather than settled trajectory (see Unresolved human questions).
- **Outside this repository:** game rules, System/LLM products, spells, gas policy, combat/AI, building *gameplay* layers, complete games—from project-boundary and the upper layers of the architecture seed.
- Demo marketing plan (X thread cadence, viral clips, downloadable demo as *the* product)—useful communication intent; the vision keeps the product as the substrate with a demonstrable harness, not a content release pipeline.
- Precise crate graph, kernel designs, and open engineering options—belong in technical design after this vision is approved.

---

## Summary sentence

**Moria is building a portable, GPU-resident voxel-world substrate crate—validated by a walkable, diggable natural-region harness—so later games can stand on real mutable geology without this repository becoming those games.**
