# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate (or a small family of tightly scoped Rust crates). It is engine infrastructure for external games—not a game, not a demo identity, and not a gameplay stack.

## Purpose

Moria exists so multiple game styles can share one natural-looking, fully material world layer instead of each reimplementing generation, mutable matter, deep underground space, and world integration. The substrate must stand alone: game rules, economy, and any LLM or “System” client live above it and are not required for the world layer to function.

## Product boundary

**In product**

- The voxel-world substrate and its public consumer-facing interfaces (mutation, queries, and world services games need to inhabit and edit a material world).
- Optional repository co-location of a walkable-world **validation harness** that consumes only those same public interfaces—never privileged or game-specific substrate paths.

**Adjacent / not product identity**

- The walkable-world executable, including any character, camera, debug presentation, authored demo route, scripted benchmark scene, or acceptance numbers used to prove the substrate.
- Downstream games (ARPG, fortress/colony, descent/roguelike, pure sandbox) and their UX, controllers, content, presentation, and policy.

**Out of this repository’s product**

- The actual game as a deliverable.
- Game rules and the System/LLM, spell, gas, combat, AI, and building **layers** (compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here).

## Required product outcomes

1. **Natural world, voxel truth** — Consumers can present a continuous, natural-looking surface world (terrain, vegetation-scale dressing, static water bodies as material volumes) while all authority for space, collision, and mutation remains voxel matter; extracted mesh is a regenerated view, not saved truth.
2. **Mutable everywhere, deep Z first-class** — Any region of the material field can be destroyed, placed into, or reshaped through public verbs; underground space (caves, strata, buried material) is real playable volume, not a painted floor under a heightmap.
3. **Geology-backed generation** — Worlds are produced as seed-driven geology (surface forms, strata, voids, and related material honesty) with lazy materialization so large regions stay tractable and dig-down reveals true structure.
4. **Live world services** — The substrate supports streaming of active space, persistence as generation-plus-edit-deltas, and queries/collision against voxel occupancy so an external consumer can walk, observe, and permanently scar a world.
5. **Reusable integration boundary** — Nothing above the matter core touches voxels directly; priced or unpriced policies, scripts, and games call the same public verb/query surface. The substrate has zero LLM dependency.
6. **Proof without becoming the game** — Dig/place (and kindred matter edits) remain substrate-owned capabilities so a consumer can prove material mutability; that is not a building-game, combat, or tools-UX product.

## Future products and enabling implications

Described future **consumers** (not current Moria scope) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style descent experience, and sandbox modes. They motivate a substrate that already treats matter, deep Z, geology, and public mutation/query as shared foundations.

Enabling implications only (no consumer gameplay, content, or presentation imported): long-horizon matter and world behaviors sketched for those games—richer fluid behavior, structural integrity, granular settle, fire/ambient ecology, object-scale vegetation with physical fall, stamp/blueprint-friendly placement, and nav/derived views—may extend the substrate later. They are not current committed roadmap merely because they appear in long-form seeds. Explicit current exclusions (game rules, System/LLM, spell, gas, combat, AI, building layers) are not revived as substrate scope by consumer desire.

## Non-goals

- Shipping an actual game, game loop, or game-rules stack in this product.
- Implementing System/LLM authorship, spells, gas policy, combat, AI agents, or building-game layers here.
- Treating the validation harness’s character controls, demo postcard content, trailer route, or platform-specific performance gates as product identity.
- Making the primary surface aesthetic a raw cube-voxel look; voxel resolution is truth, not the intended everyday appearance.
- Requiring LLM or game-layer services for the world substrate to operate.

## Confirmed vision constraints

- Delivery form is a **Rust** crate or small family of tightly scoped Rust crates for integration by Rust game consumers.
- The world substrate is **GPU-resident** as part of product identity.
- The substrate must stand alone with **no LLM dependency**.
- Any in-repo harness or future external game uses the **same public interfaces**; no privileged substrate path for first-party consumers.
- Game rules and System/LLM, spell, gas, combat, AI, and building layers are **not implemented** in this product (seams only where substrate needs demand them).

## Deferred design decisions

- Exact crate/package split and internal module boundaries (consumer boundary is fixed; packaging is not).
- Voxel scale, meshing strategy, LOD, sparsity representation, and related engine choices.
- How much of the long-horizon matter simulation (beyond the outcomes above) ships in the first substrate delivery versus later evolution.
- Streaming, persistence encoding, and synchronization mechanisms.
- Whether and how a validation harness is structured, what scene it uses, and what performance or platform gates it applies—once delivery obligation for that harness is settled.
- Open engine tradeoffs left unresolved in the seeds (e.g. fidelity vs cost, distant representation, multiplayer timing).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required current delivery** of this repository, or only a **permitted** adjacent validation artifact?

- **Proposed safe answer:** Required as an adjacent validation harness that exercises public substrate APIs and proves generation, streaming, meshing, editing, collision, persistence, and performance readiness—while remaining outside product identity (no game-layer ownership, content, or controls as substrate scope).
- **If answered differently:** If only permitted, the repository may ship substrate crates alone without committing to a playable harness as a current deliverable; harness-specific milestones and demo packaging drop out of release expectation without changing substrate identity.

## Seed synthesis

- **`README.md`** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and positions the walkable-world executable as a separate validation consumer, not a game layer.
- **`docs/seeds/project-boundary.md`** — Binds current product identity to the substrate crate(s), keeps the real game out of repo, constrains any harness to public interfaces, and excludes game-rules/System/LLM/spell/gas/combat/AI/building layers.
- **`docs/seeds/product-one-seed.md`** — Frames a first walkable proof experience and a partial substrate capability slice; used to motivate material-world outcomes and dig/place proof, not to import demo content, controllers, or hardware gates into product identity.
- **`docs/seeds/voxel-world-substrate.md`** — Supplies substrate-level goals (natural look, full mutability, deep Z, reusable non-game layer, GPU-resident standalone world services) and future-consumer context without transferring full long-horizon mechanism inventory into current commitments.
