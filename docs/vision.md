# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). Its first product-shaped proof is **Product One — “The Walkable World”**: one curated generated natural region and a third-person character who can traverse it, with dig/place as debug proof that the world is fully material matter—not a heightmap with props.

The walkable-world executable is a **consumer and validation harness**, not a game layer. It must use the same public substrate interfaces an external game would use.

Two altitudes apply throughout this vision:

| Altitude | What it is |
|---|---|
| **Substrate mandate** | Enduring outcomes the crate stack must own so later consumers can stand on it—including matter, physics, persistence, and observability families that Product One does not fully exercise. |
| **Product One slice** | The first delivery that proves the claim: generation, meshing, static matter presentation, dig/place, traversal, single-save scars, and measurable performance—without running the full simulation suite. |

---

## Purpose

Make one claim undeniable and reusable: **this is not decorative terrain—it is a fully mutable material world that still looks like a normal place**, and the same engine layer can underpin later games (sandbox, fortress, descent, ARPG) without those games living in this repository.

Product One exists to:

- Prove the substrate as a shippable, demoable artifact (audience-facing clips, a playable run, and a public crate boundary).
- Enforce the consumer API boundary from the first implementation.
- Ground open substrate decisions (scale, streaming, meshing latency, memory) in measured, comparable results rather than speculation.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| Voxel matter substrate (generation, storage, meshing, mutation, matter/physics families, streaming, persistence of world truth) | The actual game(s) that will consume Moria |
| Public crate API and the validation harness that only talks through it | Game rules, progression, combat, stats, AI |
| Compatibility *seams* where substrate requirements demand them | System / LLM, spells, gas/pricing, intent |
| | Building as gameplay (UI, blueprints-as-economy, work orders, mechanisms-as-game-systems) |
| | Consumer content, characters, lore, and art pipelines for downstream titles |

Workspace separation between reusable substrate and harness is required; the exact crate split is a later technical-design choice. Game-specific implementation paths must not live inside the substrate.

**Product One does not shrink the substrate mandate.** Capabilities deferred from the first slice remain required substrate outcomes; they are not non-goals of the product, only of this delivery.

---

## Required product-level outcomes

### Substrate mandate (enduring)

Outcomes the reusable substrate must provide so later products can stand on it—stated at consumer-visible altitude, without importing game content or rules:

1. **Material world truth** — The authoritative world is a voxel substrate. Rendered geometry is a derived view; collision and mutation operate on voxel truth, not the mesh.
2. **Mutable matter lifecycle** — Matter can be destroyed, moved, or placed. Interactive surface objects are voxel-backed; noninteractive dressing is derived from and stays synchronized with voxel truth (no free-floating props that desync from the world).
3. **Reads as a normal world** — Continuous, smooth terrain (not cube aesthetic as the default look) over mutable matter: hills, forest, water, cliffs, caves.
4. **Deep Z is first-class** — Underground volume is real content space (walkable caves, strata, sparse solid rock), not a skybox floor.
5. **Generated geology, not painted heightmaps** — Seed-driven, lazy generation that produces surface terrain, layered strata, caves, aquifers/resources, vegetation placement, and POI metadata so digging reveals honest underground structure and stamps can land where the world says they should.
6. **Matter and physics families** — The substrate owns running (or ready-to-run) behavior for GPU cellular/material response, multi-tier fluids, fire and ambient material response, granular settle, structural support/failure, and voxel-object dynamics (objects that can convert, fall, impact, and re-enter the world as matter). Consumer games supply rules and content on top; the substrate supplies the matter physics.
7. **World lifecycle: persistence and streaming** — Truth is generation plus deltas of touched matter, moved objects, and entity state; the world streams around consumer-defined active anchors; deltas support cross-run reuse so later modes can load prior scars as content.
8. **Consumer-safe interaction and observability** — Nothing above the matter layer touches voxels directly. Mutation is command-based; consumers observe through a **coarse, stale mirror** and events—not synchronous authoritative CPU voxel access.
9. **Portable GPU load path** — Load-bearing compute and graphics stay on **wgpu/WGSL**. Native backend forks in those layers are forbidden; portability across GPU APIs is part of the product identity.

### Product One first delivery slice

What the first ship must prove, without claiming the full simulation suite is running:

1. **Fused proof region** — One curated generated seed produces a natural surface world that **must** contain the proof points: continuous terrain that reads as normal, true geology (strata/caves), aquifer or resource honesty underground, vegetation and surface dressing, static water bodies, and a stamped ruin/POI exercise. The reusable generation layer—including POI metadata—ships in full for this region class; continent-scale pass may be stubbed to curated parameters.
2. **Mutable everywhere (proof path)** — Dig and place work as first-class substrate verbs so any visible solid can be cut or filled; remeshing stays interactive. This is proof of substrate, not building gameplay.
3. **Static water and placed voxel objects** — Tier-1 still water surfaces; trees, boulders, and similar objects placed, registered, and rendered as voxel-backed matter. **No** running CA, fire ecology, integrity/cave-in, granular settle, multi-tier flow, or object felling/rigid dynamics in this slice (tree felling remains a stretch if cheap; it is still a substrate outcome above).
4. **Playable validation** — Third-person traversal of a continuous surface-to-underground route; collision against voxel truth; harness-only debug tools for dig/place and observability.
5. **First-slice persistence** — Single seed, single save slot: generation + terrain/edit deltas reload the same defaced world. No multi-save versioning, no cross-run multi-mode reuse in this slice—those remain substrate lifecycle outcomes.
6. **Credible, comparable performance** — Frame rate, dig-to-remesh latency, cold-start, memory, and save/load targets are part of done. A **benchmark artifact** (scripted scene plus **machine profile**) is mandatory so results support regression testing and stay comparable across hardware. Discrete-GPU targets may be provisional until re-baselined; the M4 class machine is a development and measurement environment, not a universal product gate.

---

## Non-goals (current)

- Implementing any full game (ARPG, fortress, Moria-descent fantasy, or sandbox product) in this repository.
- Combat, RPG stats, AI, or entities beyond the player avatar in the harness.
- System / LLM features, spells, gas, pricing, or intent layers.
- Building as a game mode (UI, work orders, blueprint economy, mechanisms as gameplay systems). Stamp/prefab appears as the required generation/POI proof path, not as player construction gameplay.
- Weather/seasons/growth simulation as delivered Product One behavior.
- Multiplayer *delivery* (sessions, netcode, matchmaking). Whether multiplayer *readiness* remains a substrate design constraint is an open question below.
- Platform-native GPU forks in load-bearing layers.
- Authoring final game content, characters, lore, or art pipelines for downstream titles.

**Not non-goals of the substrate** (deferred only from Product One’s running behavior): multi-tier fluid simulation, fire/CA and ambient material response, granular settle, structural support/failure, and full voxel-object dynamics. Format, seams, and ownership stay with the substrate.

---

## Unresolved questions for humans

No seed conflict blocks product identity: **product = Moria substrate; first ship slice = Product One walkable harness**. One boundary question would materially change the compatibility contract if answered differently:

1. **Server-authoritative multiplayer readiness as a current substrate constraint** — The substrate seed notes that the verb/command architecture is server-authoritative-ready by construction and asks whether that readiness should remain in scope statements even though multiplayer is not built. Must design treat multiplayer-ready command/mirror isolation as a **required current boundary**, or may it optimize for single-player harness until a later product reopens the question?

Other open items in the substrate seed (voxel size, distant LOD strategy, object-layer scaling, fluid pressure fidelity) are design or depth choices for later technical work; they do not change product identity or repository boundary here.

---

## Seed contributions (traceability)

| Seed | What it contributed to this vision | What was treated as context / not imported as current scope |
|---|---|---|
| **README.md** | Product name and one-line identity: GPU-resident voxel substrate as Rust crate; harness is consumer/validation, not game. | — |
| **project-boundary.md** | Hard repo boundary: substrate in; games out; harness must use public APIs; game/System/LLM/spell/gas/combat/AI/building layers out of scope; workspace split required. | Exact crate topology (left to technical design). |
| **product-one-seed.md** | First delivery slice: walkable natural region, required fused proof points, dig/place as demonstration, generation-full / matter-partial / script-sliver split, single-save limitation, performance + benchmark/machine-profile as product outcomes, wgpu/WGSL lock, dual demo-and-crate success, tree dynamics as stretch. | Concrete material palette inventory, key bindings, week estimates, milestone checklists, per-metric numeric tables (kept as outcome classes), M4 as universal gate. |
| **voxel-world-substrate.md** | Enduring substrate mandate: mutable/movable matter, dressing coherence, deep-Z geology, generation/POI layer, matter/physics families (CA, fluids, fire, granular, integrity, voxel-object dynamics), persistence of objects/entities + streaming anchors + cross-run deltas, command + coarse/stale mirror contract, layering rules. Surfaces multiplayer-readiness as the genuine open boundary question. | Full build order, consumer gameplay (blueprints, work orders, mechanisms, rooms/nav as game systems), System attachment model, aesthetic option debates, game-mode examples (DF/ARPG/descent content and rules). |

---

## Summary sentence

**Build Moria as a portable, GPU-resident voxel-world substrate that owns a full material-world and physics mandate, prove it first with a walkable diggable natural region that only exercises a thin slice of that mandate—and keep every real game out of this repository.**
