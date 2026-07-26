# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). Its walkable-world executable is a **validation harness**, not a game: it consumes the substrate only through the same public interfaces an external game would use.

The first product-shaped milestone is **Product One — “The Walkable World”**: one curated generated region, a third-person character that can traverse it, and debug dig/place as proof that the world is fully material voxel truth—not a heightmap with props.

---

## Purpose

Make one claim undeniable and reusable:

> This is not decorative terrain. It is a fully mutable material world that looks like a normal natural landscape, and the same crate stack can be consumed by games without privileged implementation paths.

Downstream games (sandbox, fortress, descent, ARPG, and others) are separate consumers. Moria exists so those products start from a walkable, editable world with an enforced API boundary—not from a whiteboard or a game-specific engine fork.

---

## Boundary

| In product | Out of product |
|---|---|
| Voxel matter storage, generation, meshing, mutation, collision against voxel truth | Game rules, combat, stats, AI, entities beyond a player harness |
| Public crate API: queries, dig/place (and related matter verbs), events/mirror seams as required | The System / LLM layer, spells, gas, intent pricing |
| One seed-curated region + walkable validation harness | Building UI, blueprints-as-gameplay, mechanisms, work orders |
| Persistence as seed + edit deltas; streaming rings for a region that must not fit as raw voxels | Weather/growth sim, seasons, full fluid flow (beyond static bodies) |
| Substrate-level seams that keep future consumers clean | Any game-specific content, characters, or progression |

**Hard rule:** nothing above the matter layer touches voxels directly. The harness and all future games go through verbs and queries. Compatibility seams may be *designed* where substrate requirements demand them; game layers must not be *implemented* here.

---

## Required product-level outcomes

1. **Material world, normal look.** Surface terrain reads as hills, forest, river, cliffs, and caves—smooth extracted mesh as a *view*; voxel grid as *truth*. Grass and clutter dressing derive from voxel data, not independent props.
2. **Mutable everywhere.** Dig and place work on the matter model (debug-facing in Product One). Cuts look like cuts; remesh is incremental and fast enough to feel live.
3. **Deep Z is real.** Continuous 3D from surface into walkable underground (caves, strata, aquifer/ore as geology honesty)—not a flat floor under a skybox.
4. **Generated as geology, not a painted heightmap.** Lazy materialization from a generation pipeline (columns, strata, caves, ore); homogeneous sparsity so an idle region is cheap and a full region is not “all voxels in memory.”
5. **Consumer-grade crate boundary.** Substrate is usable by an external game through public interfaces. The walkable executable proves that boundary by living outside privileged paths.
6. **Harness proves traversal and truth.** Third-person run/sprint/jump (and surface swim as needed); collision against voxel occupancy, not the render mesh; a continuous demo route (e.g. cliff-top to deep cave) that exercises Z.
7. **Credibility under load.** Streaming, delta persistence, and benchmarked performance (frame rate, dig-to-remesh latency, cold start, memory, save size) are part of “done,” not polish after the demo.

Product One intentionally includes a **partial** matter layer (brick pool, meshing, dressing, voxel-object placement/render, static water) and a **sliver** of API (dig/place + mirror queries as engine API). Full CA, fire, flowing fluids, structural integrity, granular settle, and tree felling/rigid conversion are substrate-direction capabilities, not Product One commitments—except where format/API seams must not foreclose them.

---

## Non-goals

- Shipping a game, campaign, or mode (fortress, ARPG, roguelike, multiplayer).
- Combat, AI, NPCs, economy, gas/mana, spells, or LLM-driven content.
- Building-as-gameplay (UI, blueprints, mechanisms, designations) beyond exercising stamp/POI paths if needed for substrate proof.
- Fluids beyond static bodies; weather, seasons, growth simulation.
- Rich persistence (versioning, multi-slot, cross-run fortress reclaim) beyond seed + deltas for the harness.
- Privileged harness paths that bypass the public substrate API.
- Native Metal (or other platform) forks in load-bearing layers; portability via wgpu/WGSL is part of the product stance.

---

## Seed contributions (source accounting)

| Seed | Contribution to this vision |
|---|---|
| **README.md** | Names the product (Moria), states crate + harness split, points at seeds as preserved inputs. |
| **docs/seeds/project-boundary.md** | Binding boundary: substrate is the product; game is external; harness uses public APIs; game/System/LLM/spell/gas/combat/AI/building layers out of scope. |
| **docs/seeds/product-one-seed.md** | Binding first milestone: walkable world scope, seed-region intent, dig/place as proof, player/harness shape, performance credibility, explicit non-goals and milestones. |
| **docs/seeds/voxel-world-substrate.md** | Architecture *reference* for the substrate’s long-horizon capabilities (geology, meshing, fluids, integrity, building, entities, layering). Only portions selected by Product One are required for the current product. Future games and System hooks are **context** for seams and outcomes—not imported content or implementation scope. |
| **docs/seeds/README.md** | Operator index confirming Product One as binding implementation slice, substrate doc as selective reference, and project-boundary as binding clarification. |

**Omitted from current scope (intentionally):** ARPG/System/LLM attachment model; fortress labor and designation UX; full fluid tiers and weather ecology; structural integrity and granular CA; mechanism entities and room semantics; multiplayer; viral stretch goals (e.g. tree felling) as commitments; detailed build-order and open technical questions (voxel size fine-tuning, LOD, object-layer scaling)—those belong to design and engineering, not product identity.

**No conflict requiring a human product-identity call:** all seeds agree that Moria is the substrate, Product One is the first walkable slice, and games are downstream. Technical open questions (e.g. 25 cm vs 12.5 cm voxels, distant LOD strategy) affect implementation tradeoffs, not whether this is a game or a crate.

---

## Unresolved questions for humans

None that change product identity, purpose, or boundary. Downstream design may still need:

1. Confirmation that Product One remains the sole near-term delivery slice (vs. expanding matter-layer CA/integrity earlier).
2. Audience/delivery intent for the harness (internal validation only vs. public downloadable demo)—affects packaging polish, not the product’s nature.
3. Crate packaging preference (monolith vs. small family)—technical design, not vision.

If any of the above is wrong as human intent, correct this document before design proceeds.
