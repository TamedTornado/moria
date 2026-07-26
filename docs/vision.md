# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). Games consume it through public interfaces; they do not live in this repository.

A **walkable-world executable** may ship with the repo. It is a consumer and validation harness for generation, streaming, meshing, editing, collision, persistence, and performance — not a game layer. It must use the same public APIs an external game would use; it must not own privileged or game-specific paths.

The first product-shaped deliverable is a **generated natural region** a player can run through in third person, where what you see is voxel truth: continuous, smooth terrain that dig and place can mutate. Its job is one claim: this is not a heightmap with props — it is a fully material world, and it looks good.

---

## Purpose

Provide a standalone engine layer that:

1. **Reads as a normal world** — rolling terrain, forest, water, cliffs, caves — without a blocky “voxel game” aesthetic as the primary look.
2. **Treats matter as truth** — any volume can be destroyed, placed, or queried; the render mesh is a view regenerated from voxels, never authoritative.
3. **Makes deep Z first-class** — underground space is real content (strata, caves, ore, aquifers), not a flat floor under a skybox.
4. **Stays a substrate, not a game** — clean layering so the same crate stack can later support adventure, fortress/sandbox, or other modes without baking those rules into Moria.

The substrate must stand alone with no dependency on LLM/System, spells, combat, economy, or other game systems. Those are future consumers.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| Generation, matter storage, meshing/dressing, dig/place and related matter verbs/queries, streaming, persistence deltas, collision against voxel truth | The actual game, its rules, content pipeline as a product, multiplayer service, and live ops |
| Public crate APIs and a walkable validation harness that exercises them | System / LLM, spells, gas pricing, combat, AI, building UI/mechanisms, agent labor |
| Compatibility *seams* where substrate requirements demand them (e.g. command/mirror shape, material registry hooks) | Implementation of semantic game layers (rooms, work orders, economy, designation tools) |

Cargo workspace separation between reusable substrate and harness is an immediate structural consequence of this boundary; exact crate split is a later technical-design choice.

---

## Required product-level outcomes

Success for the current product means the following are true and demonstrable:

1. **Reusable crate boundary** — substrate crates exist; the harness (and any future game) only mutates and queries the world through public verbs and mirrors. Nothing above the matter layer touches voxels directly.
2. **Material world you can traverse** — a generated region with continuous 3D space: surface terrain, dressed nature (grass, voxel-backed vegetation/objects, static water bodies), and a walkable underground route. A third-person character collides with voxel occupancy, not the render mesh.
3. **Look without lying** — smooth isosurface meshing (surface nets / dual contouring class) over dirty bricks so hills, cut faces, and geology read as intended; mesh is always a disposable view of voxel data.
4. **Mutability as proof** — dig and place (debug-driven is enough) dirties bricks, remeshes promptly, and shows honest cut earth/stone — so the demo is not interchangeable with a static terrain scene.
5. **Geology-first generation** — columns, strata, caves, ore/aquifer bands, and lazy brick materialization from seed parameters; untouched world stays cheap via homogeneous sentinels and sparsity.
6. **Operational honesty at scale for one region** — streaming rings and delta persistence so a region large enough that raw voxels do not all fit in memory remains walkable, savable, and reloadable from seed + edits.
7. **Credible performance story** — frame-rate, dig-to-remesh, cold-start, memory, and save-size targets are measured on real hardware (including M4/wgpu constraints such as no 64-bit buffer atomics and bandwidth-first design), with regression-friendly benchmarks.

Milestone order and demo content (specific postcard route, materials set, ruin stamp, etc.) are validation design, not separate product identity. They exist to force the outcomes above under real load and camera.

---

## Non-goals

Explicitly out of current product scope:

- Combat, stats, NPCs/AI, multiplayer play, and full game loops
- System / LLM integration, spells, gas, pricing, and intent layers
- Building as gameplay (blueprints UI, mechanisms, work orders, room semantics) — stamp/prefab *paths* may be exercised only as harness proof if needed
- Fluids beyond static bodies (no flow sim, pressure, or particle splash as product requirements)
- Weather simulation, seasons, growth, fire CA, structural integrity, granular settle, tree felling / rigid conversion (format and seams may anticipate them; they do not run as product-one outcomes)
- Rich persistence (versioning, multi-slot, cross-mode fortress reclaim) beyond reload of the same seed plus edit deltas
- Native Metal (or other) forks of load-bearing layers — portability stays on wgpu/WGSL
- Owning or shipping the downstream game in this repository

---

## Unresolved questions for humans

These do not block stating product identity, but they will shape design and acceptance:

1. **Voxel size** — seeds assume ~25 cm with 16³ bricks; 12.5 cm remains an open fidelity/cost call, possibly per-region. Product One’s region is the intended benchmark bed.
2. **Distant representation** — chunked mesh LOD vs column-derived impostors for far terrain under the demo camera.
3. **Object-layer scale** — when voxel-object counts (trees, clutter) need their own spatial acceleration.
4. **Discrete-GPU baselines** — mid-GPU frame targets are provisional until verified on target discrete hardware; M4 numbers are the current hard dev floor.
5. **Stretch physics clip** — felled-tree rigid fall is explicitly stretch; include only if coupling stays cheap without pulling integrity/physics into the critical path.

No seed conflict requires human resolution of *what product is being built now*: all seeds agree the product is the substrate; the walkable world is harness/demo; full architecture beyond the Product One slice is reference.

---

## What each seed contributed

| Source | Role in this vision |
|---|---|
| **README.md** | Names the product (Moria), states substrate-as-crate and harness-as-consumer, points at `docs/seeds/`. |
| **project-boundary.md** | Binding product/consumer boundary: reusable crate(s); game out of repo; harness uses public APIs only; game rules and System/LLM/spell/gas/combat/AI/building layers out of scope. |
| **product-one-seed.md** | Binding first deliverable: walkable generated world as product-shaped proof; dig/place as mutability proof; non-goals; performance targets and M4/wgpu constraints; which substrate layers are in vs deferred. Specific seed-world content and milestone list treated as harness/validation design, not imported as permanent product content. |
| **voxel-world-substrate.md** | Architecture reference for capabilities the substrate must eventually support (geology, smooth meshing, sparsity, object vs dressing split, fluid tiers, integrity, building verbs, streaming/persistence, layering rules). Only the capabilities selected by Product One and required for the outcomes above are in current scope; game examples (ARPG, fortress, System-authored content) remain future-consumer context. |
| **docs/seeds/README.md** *(manifest helper)* | Confirms priority: Product One binds implementation slice; substrate doc is reference; boundary doc is operator clarification that Moria is only the voxel-world substrate. |

---

## Handoff note

Approve or amend this vision before technical design freezes crate APIs, region parameters, or acceptance benchmarks. Downstream design should turn outcomes and non-goals into a design doc and PR plan; it should not expand scope into game layers without an explicit product decision.
