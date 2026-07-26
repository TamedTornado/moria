# Moria — Vision

*Proposal for human approval and handoff to downstream design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates).

A walkable-world executable may ship with the repository, but only as a **validation harness**: it consumes the substrate through the same public interfaces an external game would use. It is not a game layer and must not own privileged or game-specific paths.

The first shippable slice of this product is **Product One — “The Walkable World”**: one curated generated region, smooth voxel-truth terrain you can traverse in third person, and dig/place as proof that the world is fully material matter—not a heightmap with props.

---

## Purpose

Make one claim undeniable and reusable:

> This is not a heightmap with decoration. It is a fully material voxel world that looks like a normal natural landscape, and the same substrate can later underpin games without rewriting the world layer.

Product One proves that claim with a demoable, benchmarked artifact. Downstream games (ARPG, fortress/colony, descent/sandbox, or others) are separate consumers; they are not built here.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| Voxel matter representation, storage, sparsity, streaming | Any full game (rules, progression, economy, modes) |
| Geology-first generation (as needed for Product One) | System / LLM content authorship and adjudication |
| Smooth isosurface meshing and surface dressing | Combat, stats, AI, entities beyond a harness player |
| Dig/place and mirror queries as the public mutation/query surface | Spells, gas pricing, intent systems |
| Persistence as seed + edit deltas; streaming rings | Building UI, blueprints-as-gameplay, mechanisms, labor |
| Walkable-world harness (controller, camera, debug tools) | Weather/seasons growth sim, fluid *flow*, CA fire/integrity |

**Consumer boundary is mandatory.** Prefer a Cargo workspace that separates substrate crates from the harness. The precise crate split is a technical-design choice; treating the harness as the product is not.

**Compatibility seams** may be designed where substrate outcomes require them (e.g. verb/query API, material format fields that later CA could use). Those higher layers must not be implemented here.

---

## Required product-level outcomes

What Product One must make true—not a feature list, but outcomes design and implementation must satisfy:

1. **Voxel truth, normal look.** Terrain reads as continuous natural landscape (hills, forest, river, cliffs, caves) while remaining fully material underneath. The render mesh is a view of voxel data, never the authority for collision or mutation.

2. **Mutable everywhere (proven).** Dig and place work against the real substrate; a mid-run carve remeshes cleanly and cut faces read as cut matter. Without this, the demo is indistinguishable from ordinary terrain engines.

3. **Deep Z is first-class.** Continuous vertical play from surface (and canopy/cliff) into walkable underground—not a skybox floor. Geology (strata, caves, at least one underground material truth) is real enough that digging down is honest.

4. **Generated, not hand-authored world.** One curated *seed* / parameter set produces a reliable demo region. Lazy materialization and sparsity are load-bearing: the region must not need to fit in memory as raw voxels.

5. **Public API boundary from day one.** Nothing outside the substrate touches voxels directly. Harness debug tools call the same dig/place and query surface external games will use.

6. **Credible performance and persistence.** Targets are part of the product identity: interactive frame rate on the stated dev class, fast dig-to-remesh, cold start to walkable, bounded GPU memory with streaming, and seed + delta save/load that restores edits. Benchmarks (scripted scene + machine profile) are deliverables, not afterthoughts.

7. **Reusable foundation.** Generation and matter capabilities shipped in this slice are assets for later consumers—not throwaway demo code. Product One answers open substrate questions (e.g. voxel size, LOD/object scaling) with measurements where possible.

---

## Non-goals

Explicitly out of current product scope:

- Game rules, combat, stats, AI, and entities beyond the harness player
- The System, LLMs, spells, gas, pricing, and intent
- Building-as-gameplay (UI, work orders, mechanisms, room semantics)—even though the substrate may later support them
- Full matter simulation: fluid *flow*, fire/CA ecology, structural integrity, granular settle, tree felling / rigid conversion (format may reserve fields; nothing runs these as product)
- Weather, seasons, and growth simulation (a fixed time-of-day control is enough for the demo)
- Rich multi-slot / versioned persistence; multiplayer
- Implementing future games named only as motivation (ARPG, DF-style fortress, Moria-style descent)

Future products and the full substrate architecture document are **context**: they justify why mutability, deep Z, sparsity, meshing quality, and a clean API matter. Their gameplay, content, characters, assets, and implementation details are **not** imported into current scope.

---

## Unresolved questions for humans

Seeds largely agree on product identity. These remain open at vision level and would materially affect boundary or success criteria if answered differently:

1. **Product naming vs. fantasy.** The repository and fantasy shorthand share the name “Moria.” Confirm that the *current* product name remains the substrate (not a descent game), and that no game-mode content is expected in-repo under that name.

2. **Public artifact intent.** Product One describes milestone posts and a downloadable demo for an external audience. Is a shippable public demo a required outcome of this repository’s first milestone, or is an internal harness + benchmarks enough?

3. **Success bar for “looks good.”** The headline claim is aesthetic as well as technical. Who accepts “reads as a normal world / cut faces look like cut earth”—and is there a reference bar beyond the seed’s qualitative language?

4. **Voxel size commitment.** Seeds assume 25 cm with Product One as the benchmark bed for 25 cm vs 12.5 cm. Is locking 25 cm for Product One acceptable, or must that decision wait on measured results before any milestone is “done”?

5. **Dev platform as gate.** Performance targets cite M4 Mac Mini (wgpu/Metal) and provisional discrete-GPU numbers. Is M4 the binding acceptance platform for Product One, with discrete targets explicitly provisional?

No seed conflict requires blocking the vision: all four sources name the **substrate** as the product and the walkable world as **harness / first proof**, not a game.

---

## What each seed contributed

| Source | Role in this vision |
|---|---|
| **README.md** | Names the product (Moria), positions it as a GPU-resident voxel substrate crate, and states that the walkable executable is harness, not game. |
| **docs/seeds/README.md** | Establishes seed authority: Product One is binding for implementation; substrate doc is architecture reference (only selected portions required); project-boundary is operator clarification. |
| **project-boundary.md** | Locks consumer boundary (crate vs game), workspace separation, harness-through-public-API rule, and explicit out-of-scope for game/System/LLM/spell/gas/combat/AI/building layers. |
| **product-one-seed.md** | Defines the first product-shaped slice: outcomes (material world + dig proof + traversal), non-goals, demo region intent, player/harness role, performance/persistence bar, and milestone spirit. Concrete seed content (exact biome list, material palette, ruin, etc.) is deferred to design—not re-imported here as vision scope. |
| **voxel-world-substrate.md** | Supplies the long-horizon capability context: smooth meshing over voxel truth, brick sparsity, geology-first gen, deep Z, streaming/deltas, layered crate stack, verb/query discipline. Full layers (fluids tier 2+, integrity, building, entities/nav, weather, System hooks) remain reference for *why* seams exist—not current deliverables. |

---

## Summary for approval

**Build:** a reusable Rust voxel-world substrate (Product One slice) plus a public-API walkable validation harness.

**Prove:** a generated natural region that is continuous, deep, mutable, good-looking, and performant enough to trust as the foundation for later games.

**Do not build:** those games, or the simulation/content layers that only they require.

Approve, amend the unresolved questions, or reject with a revised product identity before design proceeds.
