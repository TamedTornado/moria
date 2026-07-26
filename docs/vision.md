# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world substrate for games—not a game, not a demo product, and not an LLM-dependent system.

## Purpose

Moria exists so multiple games can share one material world foundation: a continuous 3D voxel volume that looks like a natural world on the surface, remains fully mutable all the way down, treats deep underground as first-class content, and exposes matter, physics, queries, and mutation without embedding game rules. The same substrate is meant to underpin future adventure, fortress/colony, descent, and sandbox games while standing alone with zero LLM dependency.

## Product boundary

**In product:** the reusable substrate and its public integration surface for external Rust consumers.

**Out of product identity:** the actual game; game rules; System/LLM, spell, gas, combat, AI, and building *layers*; game-specific controllers, characters, presentation, UX, authored content, and mode policy.

**Adjacent, not identity:** a walkable-world executable may exist in this repository as a validation harness only. If present, it must consume the substrate through the same public interfaces available to an external game—no privileged or game-specific substrate paths. Whether shipping that harness is a current delivery obligation is unresolved (see Q1). The precise Cargo layout is design; the consumer boundary is not optional.

Compatibility seams may be designed where substrate requirements demand them; excluded layers must not be implemented here. The game is a separate downstream consumer and is not part of this repository.

## Required product outcomes

- External games integrate only through public Rust crate interfaces; the substrate provides matter, physics, queries, and mutation, not game rules or LLM/System behavior.
- A continuous 3D material world that reads as a natural surface environment (terrain, forests, rivers, cliffs, meadows and similar) while the voxel grid remains the truth, not the aesthetic.
- Full-volume mutability: matter can be destroyed, moved, or placed throughout; dig and place are first-class substrate verbs. Simulation and queries run against voxel truth; any rendered surface is a regenerated, non-authoritative view that still reads as cut earth or organic terrain where those cuts occur.
- Deep Z is first-class: underground geology (strata, caves, ores, voids, and related subsurface structure) is content, not a skybox floor under a heightmap skin.
- World volumes arise from a geology-first generation model with lazy materialization so large, sparse regions stay tractable without eager full-volume residency.
- Higher layers never touch voxels directly: mutation verbs and mirror-style queries are the consumer path to world state; persistence is generation plus edit deltas, with streaming for active regions.

## Future products and enabling implications

Future *consumers* (not this product) include a System/LLM ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox modes. Product One’s walkable demo is an adjacent consumer slice that motivates proving the substrate, not a redefinition of Moria’s identity.

**Enabling implications only:** a shared material world with honest dig/place, deep geology, natural surface readability, and a verb/query boundary so gas pricing, labor, combat, AI, and building *game* systems can sit above without forking the world model. Gameplay, content, controllers, characters, animation, and presentation remain consumer-owned.

## Non-goals

- Implementing the actual game or any game-rule layer in this product.
- System/LLM features, spells, gas policy, combat, AI, or building-layer systems (UI, blueprints-as-gameplay, mechanisms-as-gameplay, room economies).
- Treating the validation harness’s character, camera, seed route, debug palette, content inventory, benchmarks, or platform gates as substrate scope.
- Making decorative non-matter geometry the authoritative world for play or simulation.

## Confirmed vision constraints

- Delivery form is the Rust crate ecosystem (one crate or a small tightly scoped family).
- Substrate must stand alone with zero LLM dependency.
- Adjacent validation, if present, uses only public interfaces; no privileged harness paths.
- Consumer boundary between substrate and any harness or game is mandatory; excluded layers stay out even when future games need them.

## Deferred design decisions

- Crate split and workspace packaging details within the Rust family.
- Capability depth and delivery sequence for generation, meshing, fluids, structural integrity, ambient sim, object/vegetation behavior, and related matter features.
- Voxel scale, LOD, storage layout, algorithms, performance budgets, and platform-specific engineering limits.
- Harness content, controls, acceptance scenarios, and whether/when a public demo ships (subject to Q1).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a *required current delivery* of this repository, or only a *permitted* adjacent validation artifact?

- **Proposed safe answer:** Permitted adjacent validation artifact only—not part of product identity; if present, it must use public substrate interfaces.
- **If answered “required delivery”:** Product identity remains the substrate, but the repository also commits to shipping the harness as an adjacent deliverable; harness-owned content, controls, presentation, and acceptance details still do not enter substrate scope.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world substrate (Rust crate consumer model) and positions the walkable-world executable as a separate harness, not a game layer.
- **docs/seeds/project-boundary.md** — Fixes current product identity, Rust crate boundary, game-out-of-repo rule, harness-as-validation-only rule, public-interface obligation, and exclusion of game/System/building layers.
- **docs/seeds/product-one-seed.md** — Describes a first walkable consumer/demo slice that motivates substrate proof points; does not redefine product identity or import demo content, controllers, or platform gates into substrate scope.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look vs voxel truth, full mutability, deep Z, geology generation, matter/physics/queries/mutation, persistence/streaming, reusable layering) without transferring game-layer ownership into this product.
