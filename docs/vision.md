# Moria — Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). It is an engine-layer foundation: matter, generation, meshing, mutation, queries, streaming, and persistence for continuous 3D natural worlds.

It is **not** a game. Downstream games (ARPG, fortress/colony, descent, sandbox) are separate consumers. This repository may ship a **walkable-world executable**, but that binary is only a **validation harness**: it must exercise the substrate through the same public interfaces available to any external game, with no privileged or game-specific paths.

The first binding slice of that product is **Product One — “The Walkable World”**: one generated region plus a third-person character controller, used to prove that the substrate is a fully material world (not a heightmap with props) and that it performs.

---

## Purpose

Make one claim undeniable and reusable:

> This is not decorative terrain sitting on empty space — it is a fully material voxel world that reads as a normal landscape, supports continuous 3D traversal (surface to deep underground), and is mutable everywhere under a clean API.

Product One exists so that claim can be shown (demo route, dig-as-proof) and measured (frame rate, dig-to-remesh, cold start, memory, save/load). The substrate exists so later games can build on that truth without reimplementing world matter.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| Reusable voxel-world substrate crates | Any full game layer |
| Public matter / generation / mutation / query APIs | System, LLM, spells, gas pricing, combat, AI |
| Validation harness (walkable demo) that consumes those public APIs | Building/fortress gameplay, work orders, economy |
| Cargo workspace split between substrate and harness | Downstream character/content/rules ownership |

Compatibility seams may be *designed* where substrate requirements demand them (e.g. verb/query boundary, material registry shape). Game rules and higher layers must not be *implemented* here.

---

## Required product-level outcomes

These are outcomes the current product must deliver—not a feature checklist.

1. **Substrate as product.** A reusable crate boundary exists from day one; nothing above the matter layer touches voxels except through verbs and queries.
2. **Voxel truth, not voxel look.** Terrain reads as a continuous natural world (smooth extraction / dual-contouring class meshing, material dressing). The render mesh is a view; collision and mutation run against voxel occupancy.
3. **Mutable matter everywhere.** Dig and place are first-class substrate verbs. In Product One they ship as debug proof (carve a tunnel mid-sprint; cut faces look like cut earth)—not as building gameplay.
4. **Geology-first generation.** Worldgen produces strata, caves, channels, and sparse brick materialization so digging reveals true underground content, not painted rock under a heightmap.
5. **Deep Z is first-class.** Continuous 3D play from surface (and canopy-level vantage) into walkable underground is a product claim, not a later mode.
6. **Streaming and sparsity are load-bearing.** Homogeneous bricks, lazy materialization, and streaming rings are part of the product design—not deferred optimization—so a demo-scale region need not fit in memory as raw voxels.
7. **Persistence as seed + deltas.** Untouched world is regenerable; player scars and edits save as compressed deltas; reload restores the same world state for the harness scope.
8. **Measurable credibility.** Product One targets interactive performance (order of 60fps on mid/dev hardware), low dig-to-remesh latency, short cold-start to walkable, bounded GPU-resident memory with streaming, and small delta saves—backed by a scripted benchmark scene that reports numbers with machine profile.
9. **Portable GPU stack.** Load-bearing compute stays on portable GPU APIs (wgpu/WGSL); platform constraints (e.g. no 64-bit buffer atomics on Apple GPUs) shape kernel design rather than forking native backends.

---

## Non-goals

Explicitly out of current product scope:

- Combat, stats, AI, and entities beyond a single player avatar in the harness
- The System / LLM, spells, gas, intent, and pricing policies
- Building UI, blueprints-as-gameplay, mechanisms, rooms, and work orders
- Fluids beyond static bodies (lakes / river channel with a surface)—no flow simulation
- Weather simulation, seasons, growth, fire CA, structural integrity, granular settle (format may leave room; nothing runs them in Product One)
- Multiplayer, versioned saves, multi-slot campaign persistence
- Any concrete game content, characters, narrative, or balance from reference fantasies (Moria descent, DF fortress, System ARPG)

High-level substrate capabilities that those future products would need (e.g. clean verb API, material registry, object-layer shape, POI metadata hooks) remain design context for seams; their gameplay, content, and implementation do not enter current scope.

---

## Unresolved human questions

None that change product identity, purpose, or boundary: the seeds agree that **Moria is the substrate**, Product One is the **binding first slice plus harness**, and the full substrate architecture doc is **reference** beyond the Product One selection.

Open technical tradeoffs (voxel size 25cm vs 12.5cm, distant LOD strategy, object-layer scaling, later fluid tier fidelity) are design/measurement questions for Product One’s decision bed, not vision-level identity questions.

If humans later intend a different “current product” (e.g. shipping a game in this repo, or treating the full substrate architecture as the immediate build target), that would supersede this vision and should be stated explicitly.

---

## Seed contribution account

| Seed | Role in this vision |
|---|---|
| **README.md** | Names the product (Moria), crate-vs-harness split, and points at seeds as preserved inputs. |
| **docs/seeds/README.md** | Declares hierarchy: Product One is binding for the first milestone; substrate architecture is reference filtered by Product One; project-boundary is the operator clarification; broader game/System intent is out of scope. |
| **project-boundary.md** | Locks identity: reusable crate product; game is external; harness uses public APIs only; game layers (System, LLM, spells, gas, combat, AI, building) out of scope with optional seams only. |
| **product-one-seed.md** | Pins first deliverable: walkable generated region, dig/place as proof, controller/camera traversal of continuous Z, performance/credibility targets, and explicit non-goals. Supplies the product-shaped demo outcomes without becoming a second product. |
| **voxel-world-substrate.md** | Architecture reference for the substrate’s long-term capability surface (geology pipeline, brick sparsity, meshing, fluids tiers, integrity, building verbs, entity/nav, streaming rings, layering). Only the portions selected by Product One and required for substrate reusability inform current outcomes; future game modes and full sim stack remain context, not current scope. |

**Omitted or deferred from current vision (intentionally):** detailed material palettes, milestone scheduling, specific seed-region feature lists (tree species counts, ruin stamps), and full technical prescriptions from the substrate doc—those belong to design and implementation after vision approval. **Not in conflict:** Product One’s “product-shaped demo” language and the boundary’s “harness only” language both describe the same artifact: a public-API consumer that proves the substrate.
