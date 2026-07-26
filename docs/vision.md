# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world substrate for games—not a game, not a demo product, and not an LLM-dependent system.

## Purpose

Moria exists so multiple games can share one material world foundation: a continuous 3D voxel volume that looks like a natural world on the surface, remains fully mutable all the way down, treats deep underground as first-class content, and exposes matter, physics, queries, and mutation without embedding game rules. The same substrate is meant to underpin future adventure, fortress/colony, descent, and sandbox games while standing alone with zero LLM dependency.

## Product boundary

**In product:** the reusable substrate and its public integration surface for external Rust consumers—including the world-matter outcome families listed under Required product outcomes.

**Out of product identity:** the actual game; game rules; System/LLM, spell, gas, combat, AI, and building *layers*; game-specific controllers, characters, presentation, UX, authored content, and mode policy.

**Adjacent, not identity:** a walkable-world executable may exist in this repository as a validation harness only. If present, it must consume the substrate through the same public interfaces available to an external game—no privileged or game-specific substrate paths. Whether shipping that harness is a current delivery obligation is unresolved (see Q1). The precise Cargo layout is design; the consumer boundary is not optional.

**Product One (adjacent consumer slice, not product identity):** a walkable generated natural region that proves smooth, continuous voxel truth through public-interface traversal and dig/place, and that exercises generation, streaming, persistence, and performance validation. Its first-slice exclusions (e.g. limited fluids, no weather/growth sim, no integrity/granular settle) do not shrink the broader substrate mandate. Character, camera, curated content, benchmark scenarios, and machine/platform gates stay consumer-owned.

Compatibility seams may be designed where substrate requirements demand them; excluded layers must not be implemented here. The game is a separate downstream consumer and is not part of this repository.

## Required product outcomes

- External games integrate only through public Rust crate interfaces. The substrate provides matter, physics, queries, and mutation—not game rules or LLM/System behavior. Consumers never touch voxels directly: mutation commands enter the GPU-resident world; a stale coarse mirror plus events provide observable state.
- A continuous 3D material world that reads as a natural surface environment (terrain, forests, rivers, cliffs, meadows and similar) while the voxel grid remains the truth, not the aesthetic. Matter can be destroyed, moved, or placed throughout; dig and place are first-class substrate verbs. Simulation and queries run against voxel truth; any rendered surface is a regenerated, non-authoritative view that still reads as cut earth or organic terrain where those cuts occur.
- Deep Z is first-class: underground geology (strata, caves, ores, voids, and related subsurface structure) is content, not a skybox floor. World volumes arise from a geology-first generation model with lazy materialization so large, sparse regions stay tractable without eager full-volume residency.
- Substrate-owned world-matter behavior: responsive voxel-backed objects and surface dressing; granular-material behavior; tiered water and fluid behavior; ambient time, weather, and fire ecology; structural integrity and cave-ins. These are reusable substrate outcome families; gameplay policy, presentation, and consumer systems that use them remain above the substrate.
- Mutation-safe navigation and spatial query outcomes derived from the material world so consumers can path and query without owning voxel storage. Controllers, agents, and AI remain consumer-owned.
- Persistence separates concerns: generated brick truth recovers from worldgen plus voxel edit deltas; moved substrate objects and entity state use journals. Streaming supports active regions without requiring full-volume residency.

## Future products and enabling implications

Future *consumers* (not this product) include a System/LLM ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox modes. Product One’s walkable region is an adjacent consumer/validation slice that motivates proving the substrate through public interfaces; it does not redefine Moria’s identity or transfer harness-owned content into substrate scope.

**Enabling implications only:** a shared material world with honest dig/place, deep geology, natural surface readability, object/dressing and matter-sim outcomes, integrity and fluid behavior, navigation queries, and a command/mirror/event boundary so gas pricing, labor, combat, AI, and building *game* systems can sit above without forking the world model. Gameplay, content, controllers, characters, animation, and presentation remain consumer-owned.

## Non-goals

- Implementing the actual game or any game-rule layer in this product.
- System/LLM features, spells, gas policy, combat, AI, or building-layer systems (UI, blueprints-as-gameplay, mechanisms-as-gameplay, room economies).
- Treating the validation harness’s character, camera, seed route, debug palette, content inventory, benchmarks, or platform gates as substrate scope.
- Making decorative non-matter geometry the authoritative world for play or simulation.
- Shrinking the substrate’s world-matter mandate to Product One’s first-slice exclusions.

## Confirmed vision constraints

- Delivery form is the Rust crate ecosystem (one crate or a small tightly scoped family).
- Substrate must stand alone with zero LLM dependency.
- Adjacent validation, if present, uses only public interfaces; no privileged harness paths.
- Consumer boundary between substrate and any harness or game is mandatory; excluded layers stay out even when future games need them.
- GPU-resident world interaction: commands in; stale coarse mirror plus events out for observable state.

## Deferred design decisions

- Crate split and workspace packaging details within the Rust family.
- Mechanism choice and delivery depth/sequence for substrate-owned outcomes (objects/dressing, granular materials, fluid tiers, ambient/fire ecology, integrity, nav derivation, meshing, generation stages)—not whether those outcome families belong to the substrate.
- Voxel scale, LOD, storage layout, algorithms, performance budgets, and platform-specific engineering limits.
- Harness content, controls, acceptance scenarios, and whether/when a public demo ships (subject to Q1).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a *required current delivery* of this repository, or only a *permitted* adjacent validation artifact?

- **Proposed safe answer:** Permitted adjacent validation artifact only—not part of product identity; if present, it must use public substrate interfaces.
- **Material adjacent-delivery consequences if “required”:** Product identity remains the substrate, but the repository also commits to shipping Product One’s walkable generated natural region as an adjacent deliverable that proves continuous voxel truth via public-interface traversal and dig/place and validates generation, streaming, persistence, and performance. Harness-owned character, camera, curated content, benchmarks, and machine/platform gates still do not enter substrate scope. Product One’s first-slice exclusions still do not reduce the broader substrate mandate.
- **If answered “permitted only”:** No repository obligation to ship the harness; any harness that does exist remains public-interface-only validation.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world substrate (Rust crate consumer model) and positions the walkable-world executable as a separate harness, not a game layer.
- **docs/seeds/project-boundary.md** — Fixes current product identity, Rust crate boundary, game-out-of-repo rule, harness-as-validation-only (“may”) rule, public-interface obligation, and exclusion of game/System/building layers.
- **docs/seeds/product-one-seed.md** — Establishes Product One as an adjacent first walkable delivery slice that proves continuous voxel truth and validates generation, streaming, persistence, and performance; its exclusions and consumer-owned details do not redefine substrate identity.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look vs voxel truth, full mutability, deep Z, geology generation, objects/dressing, granular/fluids/ambient/integrity, mutation-safe nav, GPU command/mirror/event contract, persistence split, reusable layering) without transferring game-layer ownership into this product.
