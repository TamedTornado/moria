# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**: a Rust crate (or a small family of tightly scoped Rust crates) that owns mutable matter, world generation hooks, queries, mutation, streaming, and persistence for large natural-looking voxel worlds.

The **current binding deliverable** is **Product One — the walkable world**: that substrate slice plus a separate **walkable-world executable** that consumes the substrate only through its public interfaces. The executable is a product-shaped validation harness and demo — a generated natural region you can run through in third person, with dig/place as proof that the world is fully material matter underneath — not a game layer and not a privileged second implementation path.

Moria is **not** the eventual game. Games (System ARPG, fortress/colony, descent-style adventure, pure sandbox, and others) are separate downstream consumers.

## Purpose

Moria exists so multiple games can share one honest material world: continuous 3D terrain and deep underground that **reads as a normal world**, remains **mutable everywhere**, and treats the **voxel grid as truth** while presentation (smooth meshes, dressing) remains a non-authoritative view.

At vision altitude, the product proves and packages:

- A world that is geology and matter, not a heightmap with props.
- Clean layering so game rules, economy, combat, LLM/System authorship, and content live **above** the substrate.
- A public, measurable artifact (the walkable harness) that makes the substrate claim undeniable and regression-testable before any game is built on it.

## Product boundary

### In this product (Moria)

| Belongs here | Does not belong here |
|---|---|
| Reusable voxel-world substrate as Rust crate(s) | The actual game(s) and their rules |
| Cargo workspace separation of substrate vs harness | Privileged harness-only matter paths that games cannot use |
| Generation of natural worlds (columns, strata, caves, ore, lazy materialization, POI metadata) at the level required by Product One | Authored campaign content, quests, balance, progression |
| Matter representation: bricks, materials, density, meshing as view, static fluid bodies (tier 1), vegetation dressing and voxel objects as scoped by Product One | Combat, stats, AI, NPCs beyond a single player avatar in the harness |
| Dig/place and mirror queries as the public mutation/query surface | Building UI, blueprints-as-gameplay, mechanisms-as-gameplay |
| Streaming, edit-delta persistence, performance and platform constraints for the substrate | System / LLM runtime, spells, gas/pricing policy, intent layers |
| Walkable-world executable as **consumer and validation harness** only | Semantic game layers (rooms-as-gameplay, work orders, economy) |

The harness must use the **same public interfaces** an external game would use. A Cargo workspace boundary between reusable substrate and validation harness is an immediate product requirement; the precise crate split is left to technical design.

Compatibility **seams** may be designed where substrate requirements demand them (so future games can attach). Those seams must not become implementations of game layers inside Moria.

### Adjacent / downstream (not Moria)

- Full game products: System ARPG, DF-style fortress/colony, Moria-style descent, pure sandbox, and any other consumer.
- Game-owned layers: combat, AI, spells, gas policy, building gameplay, economy, agent labor, content authoring pipelines beyond substrate registries and metadata hooks.
- Presentation and control choices owned by each game (e.g. locked isometric camera vs free orbit) except insofar as the harness demonstrates third-person traversal for validation.

## Future products and enabling implications

Described consumers of Moria (not built here):

1. **System / ARPG** — game layer that may use LLM/System authorship; consumes matter, queries, events, and registries.
2. **Fortress / colony-style game** — deep dig/build, hydrology toybox, structural integrity, designations; consumer of substrate capabilities.
3. **Moria-style descent / adventure** — deep-Z exploration and danger gated by geology and tools; consumer of continuous 3D underground.
4. **Pure sandbox** — same verbs with different or zero pricing policy.

**High-level enabling implications** (substrate-side only; no consumer gameplay imported):

- **Matter truth vs view** — physics, collision, queries, and mutation run on voxels; meshes and dressing are regenerated views.
- **Deep Z and geology-first generation** — underground is first-class content space; strata, caves, aquifers, ore exist so descent and dig-down honesty are possible without game-specific hacks.
- **Public verb/query boundary** — nothing above the matter layer touches voxels directly; supports sandboxing, multiple games, and later multiplayer-ready command architectures without committing to multiplayer shipping.
- **Sparsity, streaming, and edit deltas** — large regions and scarred worlds stay tractable; saves are worldgen + deltas so worlds can be reused across modes/runs when a game chooses.
- **Column / aggregate mirrors** — cheap surface height, Z-slice convenience, and coarse sim hooks for future consumers without baking fortress or ARPG rules into the substrate.
- **Extensible materials and placement stamps** — data-driven materials and prefab voxel stamps so games or a System client can author content without forking the core.
- **Structural integrity, fluid tiers beyond static bodies, CA (fire/wetness), granular settle, rigid coupling for felled objects** — directional substrate capabilities called out in the architecture seed; **not** required for Product One except where that seed’s non-goals already include or exclude them.

Product One deliberately **excludes** most of those advanced matter systems while keeping formats and API seams ready where the seeds say so. Expanding substrate depth after Product One is roadmap, not current product identity.

## Non-goals

For **Moria overall** (product identity):

- Shipping a complete game, campaign, or multiplayer service in this repository.
- Implementing System/LLM, spells, gas/pricing, combat, AI, or building-as-gameplay layers here.
- Treating the walkable executable as a game product or as a path that bypasses the public substrate API.
- Making voxels a pure aesthetic (cube look as the product goal); the grid is truth, not the required surface look.
- Depending on an LLM for geology or core substrate behavior.

For **Product One** (current binding deliverable), additionally out of scope:

- Entities beyond the player, combat, stats, AI.
- Fluids beyond static bodies (no flow sim); weather/seasons/growth simulation (fixed time-of-day is enough).
- Building UI, blueprints, mechanisms as features.
- CA-driven fire/wetness rules; structural integrity; granular settle; tree felling / rigid conversion (stretch only).
- Rich persistence (versioning, multi-slot saves beyond seed + deltas as specified for the harness).
- Final answers to all long-horizon architecture open questions (LOD strategy, multiplayer shipping, full fluid pressure model, etc.) except as measured by Product One where the seeds say the demo is the decision bed.

## Confirmed vision constraints

Only constraints explicit in the seeds:

1. **Identity** — Moria is the reusable voxel-world substrate; the game is a separate consumer; the walkable executable is a validation harness only.
2. **API parity** — harness consumes substrate through the same public interfaces available to an external game.
3. **Workspace boundary** — Cargo workspace separates substrate from harness; consumer boundary is not optional.
4. **GPU-resident substrate direction** — brick-oriented, sparsity/homogeneous sentinels load-bearing; FleX-style command-in / mirror+events-out architecture kept even on unified memory.
5. **Voxel truth** — collision and gameplay-relevant queries against voxel occupancy/matter, not against render mesh as authority.
6. **Product One world shape** — one curated generated region on the order of **1 km × 1 km × 256 m**, **25 cm** voxels and **16³** bricks as the working Product One grid (benchmark bed for size tradeoffs).
7. **Product One matter slice** — full generation layer for that seed’s needs; partial matter layer (pool, lazy materialization, meshing, dressing, voxel objects without felling, static water); dig/place + mirror queries as API surface; no CA/fire/integrity/flow in the required slice.
8. **Product One proof** — dig and place remain in scope as proof of mutability (debug-driven), not as game systems.
9. **Dev platform** — M4 Mac Mini (32 GB unified) is a binding dev target; **no 64-bit buffer atomics**; bandwidth-conscious design; **wgpu/WGSL only** in load-bearing layers (no native Metal fork).
10. **Performance intent for Product One** — seeds state targets including ~60 fps class goals, low dig-to-remesh latency, cold start, memory under streaming, and compact delta saves; treat as product-facing performance intent for the first deliverable (exact engineering tables belong downstream).
11. **Out-of-scope game systems** — System, LLM, spells, gas, combat, AI, and building layers are not implemented in Moria; seams only where required.

## Assumptions proposed for approval

**A1.** The single current product commitment is **substrate + Product One harness** as described above; broader capabilities in the voxel-world substrate architecture seed are **directional substrate roadmap** for future consumers, not parallel “build everything now” scope.

**A2.** **25 cm** voxels are the **default Product One target**; changing voxel size is allowed only as a measured outcome of the Product One benchmark bed, not as an open product-identity choice.

**A3.** A **third-person free-orbit** character controller in the harness is validation presentation only; future games may use different cameras and controls without changing Moria’s identity.

**A4.** **Tree felling / rigid-body timber** remains optional stretch for Product One and is not required to call Product One done.

**A5.** **Multiplayer** is not a shipping goal for Product One; keeping a command/verb architecture that could later support server authority is desirable hygiene, not a multiplayer product promise.

**A6.** Naming: **“Moria”** names the substrate project; “Moria-style” games are future consumers and do not share this product’s scope or release identity.

**A7.** Milestone ordering and calendar estimates in the seeds (e.g. multi-week vertical slices, X-audience posting) are planning color, not binding vision constraints beyond the product claims those milestones illustrate.

## Questions for human review

**Q1.** Is **Product One (walkable harness + substrate slice)** correctly treated as the **sole current shipping commitment**, with the rest of the architecture seed deferred as substrate roadmap?

- **Proposed:** Yes — approve A1.
- **If no:** State what additional substrate systems (e.g. flow fluids, integrity, CA fire) must enter current identity before Product One is considered the right first product.

**Q2.** Should Product One’s listed **performance and platform targets** (M4/wgpu rules, 60 fps class goals, remesh latency, memory/streaming bounds) be treated as **binding acceptance intent** for the first deliverable, or as **aspirational engineering goals** that design may relax?

- **Proposed:** Binding acceptance intent for Product One (as the product-one seed frames them as the product spec), with discrete-GPU numbers provisional until re-baselined — matching the seed’s own provisional note.
- **If aspirational only:** Downstream plans should not block “done” on those numbers without a revised vision note.

**Q3.** After Product One, does vision prefer the next major step to be **(a)** deepen substrate systems for many games (fluids, integrity, CA, building APIs), or **(b)** stand up a **specific first game consumer** outside this repo using the Product One substrate as-is?

- **Proposed:** **(a) then (b)** is not mandated; Product One seed leaves “product two” open (“fortress toybox or ARPG”). Record **no chosen product-two** until a later vision revision — current vision ends at Product One readiness.
- **If (b) is already chosen:** Name the consumer so enabling priorities can be ordered without implementing that game inside Moria.

## Seed synthesis

| Seed | Role in this brief |
|---|---|
| **`README.md`** | Project name and one-line identity: reusable GPU-resident voxel substrate as a Rust crate; walkable executable as consumer/validation harness, not a game layer. Points at seeds as inputs. |
| **`docs/seeds/project-boundary.md`** | Binding boundary: substrate is the product; game is out of repo; harness must use public APIs; Cargo workspace separation required; game/System/LLM/spell/gas/combat/AI/building layers out of scope except compatibility seams. |
| **`docs/seeds/product-one-seed.md`** | Current deliverable shape: walkable natural region, third-person proof, dig/place as proof, region/grid scale, materials and feature composition for the seed world, matter/generation slice in vs out, player/debug surface, performance and M4/wgpu constraints, milestones as illustration of done-ness claims, and what Product One buys for future work. Primary source for “what we are building **now**.” |
| **`docs/seeds/voxel-world-substrate.md`** | Long-horizon substrate purpose and architecture intent: normal-looking mutable world, deep Z, substrate-not-game layering, matter/view split, geology-first generation, vegetation/fluids/integrity/building/nav/persistence as **future-capable substrate surface**, and multi-game reuse. Used for purpose, future consumers, and enabling implications — **not** for importing full design or expanding Product One scope. Open technical questions left to design unless Product One makes them decision-bed measurements. |
| **`docs/seeds/README.md`** (supporting index) | Authority among seeds: Product One binds the first implementation slice; substrate doc is architecture reference with only Product One-selected portions required now; project-boundary is operator clarification that Moria is substrate-only. |

**Contradiction handling:** The architecture seed describes a wide substrate; Product One and project-boundary narrow identity and first scope. This brief **fuses** them by making the substrate the product identity, Product One the current commitment, and the remainder directional enabling roadmap — not by treating all seed sentences as equal requirements.

**Deliberately not elevated to vision:** meshing algorithm choice details, full storage bit layouts, CA rule tables, milestone calendars, viral/X marketing tactics, and exhaustive feature inventories. Those belong in design and planning after this vision is approved.

---

*This document, once accepted, supersedes the raw seeds as the authoritative vision input to downstream design. Seeds remain historical source material.*
