# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). A walkable-world executable may ship with the repository, but only as a **validation harness and product-shaped demo** that consumes the substrate through the same public interfaces available to any external game.

The actual game (ARPG, fortress, descent roguelike, sandbox, or otherwise) is a **separate downstream consumer** and is not part of this repository.

---

## Purpose

Give future games—and the audience that must believe the tech—a **material world foundation** that:

1. **Reads as a normal natural world** (rolling terrain, forest, water, cliffs, underground) while remaining fully voxel-backed underneath.
2. **Is mutable everywhere**, all the way down: dig, place, and scar the world; the mesh is a view of voxel truth, never the authority.
3. **Treats deep Z as first-class content**, not a painted floor under a heightmap.
4. **Stays a substrate, not a game**: clean layering so the same crate stack can support multiple game modes without baking their rules into the world layer.

Product One’s job is to make one claim undeniable in a runnable demo: *this is not a heightmap with props—it is a fully material world, and it looks good.* Dig and place on a debug path are the proof, not gameplay.

---

## Boundary

| In | Out |
|----|-----|
| Reusable substrate crate(s) and their public API | The actual game product |
| Generation, matter representation, meshing/dressing, streaming, persistence of world truth | Combat, stats, AI, entities beyond a harness player, economy |
| Engine-internal dig/place verbs and mirror queries (nothing touches voxels ad hoc) | The System / LLM layer, spells, gas pricing, intent |
| Walkable-world executable as consumer + validation harness | Building UI, blueprints-as-gameplay, mechanisms-as-gameplay |
| Compatibility *seams* where substrate requirements demand them | Implementing game-rule, semantic, or script-language layers beyond the API sliver needed for proof |

Workspace separation between reusable substrate and harness is required so the harness never owns privileged or game-specific world paths.

---

## Required product-level outcomes

These are the outcomes the product must deliver; how they are partitioned into crates, kernels, or milestones is design work downstream.

1. **Voxel truth, smooth view** — World state is material voxels (bricks, palettes, sparsity). Rendering uses smooth isosurface extraction (and related dressing); collision and queries use voxel occupancy, not the render mesh.
2. **Mutability with responsive remesh** — Dig and place work anywhere in loaded matter; dirtied regions remesh quickly enough that mid-traversal carving feels continuous.
3. **Geology-first generation, lazy materialization** — Worlds are generated as geology (columns, strata, caves, ore, POI metadata), not as a heightmap with rock painted under it. Untouched volume stays cheap via homogeneous sentinels and on-demand materialization.
4. **Natural surface over deep volume** — A curated region can present meadow, forest, water bodies (static tier-1 surfaces), cliffs, and a walkable underground route in one continuous 3D space.
5. **Dressing and voxel-backed objects as substrate capabilities** — Grass/clutter as data-driven dressing; trees, boulders, and similar interactables as voxel objects placed and rendered through the substrate (full felling/rigid conversion is not required for the first product-shaped demo).
6. **Streaming and persistence model** — Truth is worldgen function plus edit deltas; active regions stream in rings; memory stays tractable for a region that must not fit as raw voxels.
7. **Public consumer boundary** — Games (and the harness) mutate and query only through verbs and mirror APIs. That boundary is the sandbox, multiplayer-readiness, and reuse seam.
8. **Credible performance story** — Sustained interactive frame rate on target mid-class and M4-class hardware; measurable dig-to-remesh latency, cold start, memory, and save/load; regression benchmarks with machine profile.
9. **Portable GPU stack** — Load-bearing compute and graphics stay on portable abstractions (wgpu/WGSL); design respects platforms without 64-bit buffer atomics and with bandwidth-bound GPUs.

High-level capabilities the substrate must *eventually* support for downstream games—without implementing those games here—include richer matter simulation (CA, multi-tier fluids, granular settle, structural integrity), building and mechanism verbs, nav derived from matter, and ambient weather/time. Product One ships format and seams where needed; it does not run full CA, flow, integrity, or game layers.

---

## Non-goals

- Shipping a playable game, campaign, combat, or progression systems in this repository.
- Implementing the System/LLM, spells, gas policy, or intent stacks.
- Fluids beyond static bodies in the first product-shaped demo; weather/seasons/growth sims; multi-slot save versioning.
- Building-game UX, work orders, agent labor, or room economy.
- Treating the walkable executable as privileged engine code rather than an external-style consumer.
- Locking aesthetic or content to a single future game’s characters, factions, or assets.

---

## Unresolved human questions

The seeds agree on **current product identity** (substrate + optional harness; game elsewhere). No ambiguity requires blocking this vision. Residual items for later human/design decisions (not product-identity blockers):

- Final voxel size (25 cm baseline vs finer options) and distant-terrain LOD strategy—Product One is the measurement bed.
- How far the first public demo must go on voxel-object physics (e.g. tree felling) versus placement/render only.
- When multiplayer-oriented scope language becomes an explicit commitment versus an architectural non-preclusion.

---

## Seed contributions

| Seed | What it contributed to this vision |
|------|-------------------------------------|
| **README.md** | Names the repository product (Moria), states crate-vs-harness split, and points at the seed set as preserved substrate inputs. |
| **project-boundary.md** | Locks product identity to the reusable substrate; forbids implementing the game and higher layers here; requires public-API consumption by any walkable executable and a workspace consumer boundary. |
| **product-one-seed.md** | Defines the first product-shaped proof: one generated natural region, third-person traversal of continuous 3D (including underground), dig/place as demo proof, explicit non-goals, performance credibility targets, and the slice of generation / matter / API layers that ship first. Concrete seed-world set dressing (species counts, ruin stamp, material list, milestone order) is **demo content and validation criteria**, not imported here as product scope. |
| **voxel-world-substrate.md** | Supplies the long-horizon substrate intent: normal-looking mutable world, deep Z, GPU-resident brick model, geology-first gen, matter/dressing/fluids/integrity/building layering, and reuse across multiple game modes. Future game modes, System attachment points, and full sim feature depth are **context for required capabilities and seams**, not current implementation or content scope. |

Where Product One narrows the full substrate specification (e.g. static water only, no CA runtime, no felling required), this vision follows Product One for **current** outcomes and keeps the broader substrate document as the capability envelope the crate family must remain compatible with.
