# Moria — Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**: a Rust crate (or small family of tightly scoped crates) that owns matter, generation, mutation, queries, meshing, streaming, and persistence for a continuous three-dimensional world.

It is **not a game**. Downstream games consume Moria through public interfaces. This repository may ship a **walkable-world executable**, but only as a **validation harness** that uses the same public APIs an external game would use—not privileged or game-specific paths.

**Product One** is the first shippable slice of that substrate: one curated generated region plus a third-person character who can traverse it, with dig/place as proof that the world is fully material voxel truth. The harness makes one claim undeniable: *this is not a heightmap with props—it is a fully material world, and it looks good.*

---

## Purpose

1. **Prove the substrate.** Deliver a public, playable, measurable artifact that validates smooth meshing, geology-first generation, deep continuous Z, incremental mutation, streaming sparsity, and a clean consumer boundary.
2. **Be reusable.** Establish crate and API boundaries so future games (ARPG, fortress/colony, descent, pure sandbox) sit above the same matter stack without forking world code.
3. **Decide with numbers.** Use Product One as the benchmark bed for open substrate questions (voxel size, LOD, object-layer scale) rather than settling them by speculation.

---

## Boundary

| In Moria | Out of Moria |
|---|---|
| Generation layer (geology pipeline, columns, lazy materialization, POI metadata, material palette) | Game rules, progression, combat, stats, AI |
| Matter layer (brick pool, sentinels, meshing, surface dressing, voxel objects, static tier-1 water) | The System / LLM, spells, gas pricing, intent |
| Script/API sliver: dig/place verbs and mirror queries (nothing touches voxels directly) | Building UI, blueprints, mechanisms, work orders |
| Walkable-world harness consuming public substrate APIs | Fluids flow sim (tier 2+), fire/CA, integrity, granular settle as product features |
| Persistence as seed + edit deltas; streaming rings | Multiplayer, weather/seasons/growth sims, multi-save versioning |
| Performance targets and regression benchmarks for the harness | Any concrete game mode, characters, content IP, or campaign |

**Cargo workspace** (or equivalent) must separate reusable substrate crates from the harness executable. The precise crate graph is a design decision; the consumer boundary is not.

Future product layers described in the full substrate reference (structural integrity, multi-tier fluids, building verbs, nav/agents, ambient sim, semantic rooms) are **capability context** for what the substrate may eventually provide. Product One only requires the bottom generation and matter layers plus a thin verb/query surface—**not** those higher layers as current deliverables.

---

## Required product-level outcomes

When Product One is done, the following must be true:

1. **Material world, not scenery.** Terrain reads as a natural surface world (smooth isosurface / dual-contour style extraction, material blending), yet every visible volume is backed by mutable voxels. Mesh is a view; collision and mutation run against voxel truth.
2. **Mutable everywhere in the harness.** Dig and place (debug keys are enough) dirty bricks, remesh incrementally, and show cut faces that read as cut earth—proof the world is substrate, not a painted mesh.
3. **Continuous deep Z.** A player can move continuously from surface/canopy-scale terrain into underground space on the same world representation (no skybox floor).
4. **Geology-first generation.** Worldgen produces real structure underground (strata, voids/caves, at least one subsurface resource band)—digging down hits *truth*, not filler rock under a heightmap.
5. **Sparsity and streaming that matter.** A region large enough that the full volume cannot live as raw voxels in memory; homogeneous bricks, lazy materialization, and streaming rings keep idle wilderness cheap and cold-start walkable quickly.
6. **Public API boundary from day one.** Harness and any external consumer mutate and query only through substrate verbs/mirrors; no game-layer shortcut into storage.
7. **Measurable performance.** Targets (frame rate, dig-to-remesh latency, cold-start, GPU-resident memory budget, delta save size) are part of the product claim, with a scripted benchmark scene that reports numbers plus machine profile.
8. **Portable GPU stack.** Load-bearing work stays on portable GPU abstraction (wgpu/WGSL); design respects constraints of the primary dev platform (e.g. no dependence on 64-bit buffer atomics).

---

## Non-goals

- Shipping a game, combat loop, economy, multiplayer, or LLM-driven content system in this repository.
- Implementing System/spell/gas layers, CA (fire, wetness rules), fluid *flow*, structural integrity, granular settle, or building/blueprint/mechanism gameplay in Product One—even where formats or seams anticipate them.
- Authoring production game content (characters, factions, quests, campaign maps). Curated generation parameters for the harness seed world are validation fixtures, not product identity.
- Treating the walkable executable as the long-term product surface rather than a consumer of the crate.
- Relitigating the full multi-game roadmap inside Moria; games are separate consumers.

---

## Unresolved questions for humans

These would materially change identity, purpose, or boundary if answered differently:

1. **Product name vs. fantasy.** Seeds use "Moria" for the substrate and also as a label for a future descent-style *game*. Confirm that **this repository's product is only the substrate**, and that "Moria" here does not imply shipping descent gameplay.
2. **How far is "done" for Product One?** Binding seed milestones run through a numbers/benchmarks milestone, with tree-felling as stretch. Confirm whether the approval bar is the full ordered milestone set through benchmarks, or a shorter public demo (e.g. continuous run only).
3. **Harness fidelity of the seed region.** Product One specifies a detailed curated region (feature list, material palette, size). Confirm that this content is **validation fixture**, not a commitment that Moria "is" that postcard world—only that the substrate must be able to host such a world.

No other seed disagreements block a coherent product identity: boundary, Product One, and substrate reference nest cleanly (see below).

---

## What each seed contributed

| Seed | Role in this vision |
|---|---|
| **README.md** | Names Moria as reusable GPU-resident voxel substrate; harness is consumer/validation, not game layer. |
| **project-boundary.md** | Binding scope: crate(s) only; game separate; harness on public APIs; game/System/LLM/spell/gas/combat/AI/building out of scope. Workspace boundary required. |
| **product-one-seed.md** | Binding *first slice*: walkable demo claims, non-goals, substrate layer cut (gen full, matter partial, API sliver), player traversal as proof of continuous Z, dig/place as proof of mutability, performance targets, milestone order. Specific terrain features, materials, and debug keys treated as fixture/spec for the harness, not as game content to import into product identity. |
| **voxel-world-substrate.md** | Architecture *reference*: design goals (normal look, mutable everywhere, deep Z, substrate-not-game, GPU-resident), look strategy, storage/gen/vegetation/fluids layering model, future-game support rationale. Only portions selected by Product One are current scope; higher layers and multi-game examples remain context for required seams and capabilities, not current implementation inventory. |
| **docs/seeds/README.md** (manifest note) | States the binding hierarchy above: Product One selects required substrate portions; boundary clarification is operator-binding. |

### Intentionally omitted from current vision (present in seeds as future or example)

- Full CA, fire ecology, multi-tier fluids, integrity/cave-ins, building/blueprints/mechanisms, nav classes, agent labor, rooms/economy hooks, weather/seasons, System attachment authoring, rigid-body tree felling (stretch in Product One only).
- Concrete future games (System ARPG, DF-style fortress, Moria-style descent) as products of this repo—kept only as consumers the substrate must not preclude.
- Low-level payload bit layouts, exact meshing algorithm choice, and open engineering questions (LOD strategy, fluid pressure solve, object spatial index)—belong in technical design after vision approval.

---

## Summary sentence

**Moria is the voxel-world engine layer—GPU-resident, mutable, continuous in depth, and cleanly consumable—proven first by a walkable harness that looks like a natural world and digs like matter; games live elsewhere.**
