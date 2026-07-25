# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate (or a small family of tightly scoped crates). It is the material world engine other products consume: natural-looking terrain that is fully mutable matter, continuous deep underground play, and public interfaces for generation, query, and mutation—not a game.

## Purpose

Games and demos need a world that *reads* as ordinary landscape while remaining editable, diggable, and honest in three dimensions. Moria exists so those consumers share one substrate for matter, world structure, and change, instead of each reinventing geology-shaped voxels and presentation-as-truth. The substrate must stand alone with **no LLM or game-rules dependency**.

## Product boundary

**In this product**
- The reusable world substrate and the public surfaces an external game would use for material truth, world generation hooks, queries, and mutation.
- Optional in-repo validation that exercises those same public interfaces.

**Not this product**
- The eventual game (ARPG, fortress, descent, sandbox) and its rules, content, UX, characters, cameras, and controllers.
- System / LLM, spells, gas policy, combat, AI, and building *game* layers. Compatibility seams may be anticipated; those layers are not implemented here.

A walkable-world executable, if present, is an **adjacent consumer**: proof that the substrate works, not a privileged game path and not ownership of demo narrative, controls, or presentation.

## Future products and enabling implications

Downstream consumers described in the seeds include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox play. They are **not** current scope.

High-level enabling implications the substrate is meant to support over time (without committing sequence or feature inventory): natural surface worlds over voxel truth; mutation everywhere including deep Z; geology-first generation so digging reveals real structure; and matter/physics/query affordances rich enough that building, fluids, integrity, and agent labor can sit *above* the same world later. Gameplay policy, authored content, and presentation remain consumer-owned.

## Non-goals

- Shipping a finished game, combat loop, economy, or LLM-authored runtime.
- Treating heightmap-plus-props scenery as the product claim; mesh and dressing are views over material truth, not the authority.
- Implementing future game layers (System, spells, gas, combat, AI, fortress building gameplay) inside this repository.

## Confirmed vision constraints

- Product identity and repository boundary: reusable substrate; actual game lives elsewhere.
- Exposure as Rust crate(s); workspace separation between substrate and any validation harness is required in principle (exact crate split is design).
- Any harness must use the same public interfaces available to an external game.
- Zero LLM dependency in the substrate.

## Assumptions proposed for approval

1. **Long-horizon matter depth is enabling, not a silent current roadmap.** Fluids beyond static bodies, fire ecology, structural integrity, granular settle, full building/mechanism stacks, and multiplayer-ready deployment are implied futures in the broad substrate seed; they do not redefine the current product as those systems shipping now.
2. **“Reads as a normal world” is a substrate outcome.** Generation and material-consistent surface presentation needed for that claim are substrate responsibilities; curated postcard routes, character traversal scripts, and milestone marketing clips belong to consumers or harnesses, not to product identity.

## Questions for human review

**Q1.** Is an in-repo walkable-world executable a **required current delivery**, or only a **permitted** adjacent validation consumer?

- **Proposed safe answer:** Permitted only. Current product success is the reusable substrate and its public interfaces; a harness may exist to validate them but is not mandatory scope.
- **If answered otherwise:** Required delivery expands repository scope to include shipping that executable as part of “done,” while still excluding game layers—without automatically importing its controller, content, or performance gates into substrate identity.

## Seed synthesis

- **`README.md`** — Named the product Moria; defined it as a reusable GPU-resident voxel-world substrate consumed as a Rust crate; classified the walkable-world executable as separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`** — Locked current identity to the substrate crate(s); excluded the actual game from the repo; required harness-through-public-API and workspace consumer boundary; put System/LLM, spell, gas, combat, AI, and building layers out of scope (seams only).
- **`docs/seeds/product-one-seed.md`** — Motivated first vertical proof (natural mutable world, dig/place as honesty proof) and harness-shaped demo concerns (character, route, seed region, targets, milestones). Contributed outcome pressure and non-goals for game systems; demo controls, content inventory, benchmarks, and slice mechanics were **not** lifted into current product scope.
- **`docs/seeds/voxel-world-substrate.md`** — Supplied the long-horizon substrate purpose: normal-looking world over full voxel mutability, deep Z, geology-first generation, layering so many games share matter/physics/query/mutation with no LLM dependency. Implementation inventories, algorithms, and open engineering questions deferred to design.
