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

| In this repository | Outside this repository |
|---|---|
| Voxel matter representation (bricks, sparsity, materials) | Game rules, combat, stats, AI, entities beyond a harness player |
| Geology-first generation and lazy materialization | The System / LLM layer and content authoring products |
| GPU meshing and surface dressing as a *view* of voxel truth | Spells, gas/economy policy, intent systems |
| Mutation and query API (dig/place and mirror-style reads) | Building UI, blueprints-as-gameplay, mechanisms as game systems |
| Collision against voxel occupancy | Full fluid simulation beyond static bodies (tier-1 lakes/river surface) |
| Streaming rings, edit-delta persistence, harness benchmarks | Weather/seasons/growth sims; multi-save versioning |
| Cargo workspace separation: substrate crate(s) vs harness | Any complete game on top of the substrate |

Compatibility *seams* may be designed where substrate requirements demand them (e.g. verb/query boundaries that future gas policy or multiplayer can plug into). Those layers must not be implemented here.

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

- Implementing any game layer: combat, AI, NPC entities, ARPG lock-camera fantasy, fortress labor, spells, gas pricing, LLM “System” behavior.
- Full multiphase fluid CA, fire ecology, structural integrity cave-ins, granular settle, tree felling as rigid bodies—except where the *format* or API already anticipates them for later products.
- Building/mechanism gameplay, room semantics, work orders, multiplayer servers.
- Authored “one true world” content as product identity; the seed region is a curated validation route, not a shipping campaign.
- Native Metal (or other platform) forks in load-bearing layers for short-term gain.
- Treating the walkable harness as the long-term product surface or giving it private substrate hooks.

---

## Unresolved human questions

None that change product identity, purpose, or boundary. The seeds agree on what Moria *is* (substrate + harness) and what it is *not* (the game).

Questions left to design and measurement (not vision blockers):

- Final voxel size (25 cm vs 12.5 cm or hybrid) for the benchmark region.
- Distant terrain LOD strategy under the harness camera.
- How far object-layer scale (trees/clutter counts) needs acceleration in the first slice.
- Which performance targets remain provisional until discrete-GPU baselines exist.

If product intent ever shifts from “crate + validation harness” to “shipping walkable demo as the primary product,” or from “no game systems” to “include a thin fortress/adventure mode,” that would rewrite this vision and should be an explicit human decision.

---

## Seed contributions (traceability)

| Seed | What it contributed to this vision |
|---|---|
| **README.md** | Names the product (Moria), states substrate-as-crate and harness-as-consumer, points at seeds as preserved inputs. |
| **project-boundary.md** | Hard boundary: substrate is the product; game is external; harness must use public APIs; game/System/LLM/spell/gas/combat/AI/building layers out of scope; workspace split between crate and harness. |
| **product-one-seed.md** | First deliverable identity (walkable vertical slice), product-level claim, non-goals for slice one, dig/place as proof not gameplay, performance and platform constraints as product outcomes, milestones as proof order—not imported as a content GDD or game design. Specific seed-world features (ruin, ore types, tree species, etc.) are treated as validation-scenario context for capabilities the substrate and harness must exercise, not as current product lore. |
| **voxel-world-substrate.md** | Capability envelope the substrate must eventually support for reuse (smooth meshing over density, brick sparsity, geology-first gen, lazy materialization, column index, dressing vs voxel objects, tiered fluids, integrity, building verbs, nav/Z, streaming/persistence, layering rules). Future games (ARPG, fortress, Moria descent) and System attachment points are context for *why* seams exist; their gameplay, content, characters, and full implementation are **not** current scope. Only the bottom layers (generation + matter + a sliver of verb/query API) are implied for the first product slice. |

### Intentionally omitted or deferred from seeds

- Full CA (fire, wetness propagation), integrity/cave-ins, granular settle, fluid tiers 2–3, tree felling physics, weather/seasons/growth, blueprints/mechanisms/rooms, entity pathfinding classes, gas policy, multiplayer—preserved as substrate-direction context, not first-slice product identity.
- Demo marketing plan (X thread cadence, viral clips, downloadable demo as *the* product) — useful communication intent; the vision keeps the product as the substrate with a demonstrable harness, not a content release pipeline.
- Precise crate graph, kernel designs, and open engineering options—belong in technical design after this vision is approved.

---

## Summary sentence

**Moria is building a portable, GPU-resident voxel-world substrate crate—validated by a walkable, diggable natural-region harness—so later games can stand on real mutable geology without this repository becoming those games.**
