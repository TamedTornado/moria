# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). Its near-term, product-shaped proof is **Product One — “The Walkable World”**: one generated natural region and a third-person character who can traverse it, with dig/place as debug proof that the world is fully material matter—not a heightmap with props.

The walkable-world executable is a **consumer and validation harness**, not a game layer. It must use the same public substrate interfaces an external game would use.

---

## Purpose

Make one claim undeniable and reusable: **this is not decorative terrain—it is a fully mutable material world that still looks like a normal place**, and the same engine layer can underpin later games (sandbox, fortress, descent, ARPG) without those games living in this repository.

Product One exists to:

- Prove the substrate as a shippable, demoable artifact (audience-facing clips and a playable run).
- Enforce the consumer API boundary from the first implementation.
- Ground open substrate decisions (scale, streaming, meshing latency, memory) in measured results rather than speculation.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| Voxel matter substrate (storage, generation, meshing, mutation, streaming, persistence of world truth) | The actual game(s) that will consume Moria |
| Public crate API and the validation harness that only talks through it | Game rules, progression, combat, stats, AI |
| Compatibility *seams* where substrate requirements demand them | System / LLM, spells, gas/pricing, intent |
| | Full building UI, blueprints-as-gameplay, mechanisms, agent labor |
| | Weather/growth ecology, full fluid simulation, structural integrity sim as shipped gameplay |

Workspace separation between reusable substrate and harness is required; the exact crate split is a later technical-design choice. Game-specific implementation paths must not live inside the substrate.

---

## Required product-level outcomes

Outcomes the current product must achieve so later products can stand on it—without importing those products’ content or rules:

1. **Material world truth** — The authoritative world is a voxel substrate (bricks, materials, density). Rendered geometry is a derived view; collision and mutation operate on voxel truth.
2. **Reads as a normal world** — Continuous, smooth terrain (not cube aesthetic as the default look) over mutable matter: hills, forest, water bodies, cliffs, caves.
3. **Mutable everywhere (proof path)** — Dig and place work as first-class substrate verbs so any visible solid can be cut or filled; remeshing stays interactive. This is proof of substrate, not building gameplay.
4. **Deep Z is first-class** — Underground volume is real content space (walkable caves, strata, sparse solid rock), not a skybox floor.
5. **Generated geology, not painted heightmaps** — Lazy, seed-driven generation that can produce surface terrain, layered strata, caves, and material variety so digging reveals honest underground structure.
6. **Streaming and sparsity at region scale** — A region large enough that raw full-voxel residency is not the design; homogeneous/empty bulk stays cheap; cold-start and memory stay within product targets.
7. **Persistence of scars** — World state as generation + edit deltas; reload restores the same defaced world.
8. **Consumer-safe API** — Nothing above the matter layer touches voxels directly; harness and future games share dig/place, queries, and events through the same boundary.
9. **Playable validation** — Third-person traversal of a curated seed route (surface to underground) that exercises continuous 3D, collision against voxels, and the demo claim end-to-end.
10. **Credible performance** — Targets and benchmarks (frame rate, dig-to-remesh latency, cold-start, memory, save size) are part of what “done” means for Product One, including portability constraints that keep the load-bearing path on portable GPU APIs (e.g. wgpu).

Capabilities the full substrate design anticipates for *future* consumers—fluids beyond static bodies, fire/CA, structural integrity, voxel-object dynamics (e.g. falling trees), mechanisms, rooms/nav as gameplay systems, multiplayer readiness—are **direction for seams and data shapes**, not current ship scope, except where Product One already requires a thin slice (static water bodies, voxel-backed trees/boulders as placed matter without full dynamics, format room for later rules).

---

## Non-goals (current)

- Implementing any full game (ARPG, fortress, Moria-descent, or sandbox product).
- Combat, RPG stats, AI, or entities beyond the player avatar in the harness.
- System / LLM features, spells, gas, pricing, or intent layers.
- Building as a game mode (UI, work orders, blueprint economy); stamp/prefab may appear only as a one-shot generation/validation path if needed for the seed.
- Flowing fluids, weather/seasons/growth simulation, fire CA, granular settle, or structural cave-in simulation as delivered Product One behavior.
- Multiplayer, multi-save versioning, or platform-native GPU forks in load-bearing layers.
- Authoring final game content, characters, lore, or art pipelines for downstream titles.

---

## Unresolved questions for humans

No seed conflict blocks identity: **product = substrate; current ship slice = Product One walkable harness**. The following would still change emphasis or acceptance criteria if answered differently:

1. **Primary “done” audience** — Is Product One success defined first as *public demo / downloadable walkable world*, or first as *stable crate API + benchmarks with the harness secondary*? (Seeds treat both as goals; priority affects packaging and polish bar.)
2. **Stretch physics in scope or explicitly deferred** — Product One lists felled-tree rigid-body as stretch. Should vision treat any dynamic voxel-object coupling as out until a later product, or allow it if cheap?
3. **Name collision** — Repository/product name “Moria” also appears in seeds as a *future game fantasy* (descent). Confirm the shipped product name remains the substrate (Moria), with games unnamed or separately branded downstream.

If these are left open, default stance for design handoff: crate + harness co-deliver; stretch dynamics deferred; product name = substrate.

---

## Seed contributions (traceability)

| Seed | What it contributed to this vision | What was treated as context / not imported as current scope |
|---|---|---|
| **README.md** | Product name and one-line identity: GPU-resident voxel substrate as Rust crate; harness is consumer/validation, not game. | — |
| **project-boundary.md** | Hard repo boundary: substrate in; games out; harness must use public APIs; game/System/LLM/spell/gas/combat/AI/building layers out of scope; workspace split required. | Exact crate topology (left to technical design). |
| **product-one-seed.md** | Current ship slice: walkable natural region, third-person proof, dig/place as demonstration, non-goals, region intent, generation/matter slice, performance as product outcomes, milestone spirit. | Concrete feature lists, material palette inventory, key bindings, week estimates, milestone checklists, machine-specific numbers as *spec detail* (kept as outcome class, not a full target table here). |
| **voxel-world-substrate.md** | Capability horizon the substrate must remain able to support: smooth meshing over voxel truth, deep-Z geology, sparsity/streaming, vegetation-as-matter-or-dressing, layered reuse for many game types, API/matter separation. | Full build order, CA/fluids/integrity/building/entity systems, System attachment model, aesthetic option debates, and game-mode examples (DF/ARPG/Moria descent content and rules). |

---

## Summary sentence

**Build Moria as a portable, GPU-resident voxel-world substrate whose first undeniable proof is a walkable, diggable natural region—and keep every real game out of this repository.**
