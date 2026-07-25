# Project vision

## What we are building now

Moria is a reusable voxel-world substrate: a Rust and Bevy library that downstream games and tools install and drive through a public consumer contract. It is engine-shaped world infrastructure, not a playable game product.

## Purpose

Voxel worlds only work when generation, sparse truth, queries, mutation, streaming, collision truth, persistence, and measurable presentation of that truth agree as explicit contracts. Moria exists so natural, fully material worlds can be consumed without each game rebuilding those contracts, and without hiding them behind a single demo.

## Product boundary

**This product owns** the reusable substrate: deterministic world generation, sparse material storage, bounded inspection, mutation admission, streaming lifecycle, collision truth against voxel matter, persistence of generated truth plus edits, and the public facade that makes those outcomes available to external consumers in Rust/Bevy.

**Adjacent, not the product** are curation, benchmark, and visual-validation executables. They may exist to exercise and evidence the substrate, but they must use the same public interfaces available to an external game. Their controllers, characters, camera, authored routes, presentation polish, and acceptance scenarios are consumer concerns, not substrate identity.

**Downstream, not this repository** are actual games and game layers: player control, characters, animation, game-specific presentation, combat, AI, economy, building policy, the System/LLM layer, spells, gas pricing, and other gameplay rules.

## Future products and enabling implications

Described future consumers include a System-driven ARPG, a fortress/colony game, a descent-style roguelike, and pure sandboxes. They motivate substrate outcomes—normal-looking natural worlds, mutability all the way down, first-class deep underground content, and a clean matter/query/mutation boundary—but their gameplay, content, controls, and presentation are not current Moria scope. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Non-goals

- Shipping a game, game mode, or game-rules stack in this product.
- Treating validation harness content, controllers, or demo routes as substrate features.
- Implementing excluded game systems (System/LLM, spells, gas, combat, AI, building layers) under substrate ownership.

## Confirmed vision constraints

- Adjacent consumers, including any validation harness, have no privileged access path into the substrate.
- The product is defined as a Rust/Bevy library ecosystem for crate consumers, not an ecosystem-neutral engine abstract.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world visual validation harness a **mandatory current delivery** of this repository, or only a **permitted adjacent artifact**?

- **Proposed answer:** Permitted adjacent artifact only. Product identity and “done” for Moria are the reusable substrate and its public contracts; a walkable harness may be built to validate them but is not required to define the product.
- **If answered differently:** Making the harness mandatory keeps it outside product identity but adds a required adjacent deliverable (still without importing its controls, content, or acceptance details into the substrate). Treating it as optional leaves harness work non-blocking for substrate completion.

## Seed synthesis

- **README.md** — States Moria as a Rust/Bevy voxel-world substrate consumed as a crate, with executables for curation, exercise, benchmark, and visual validation; game-layer ownership is excluded. Compatible status and evidence detail remain subordinate downstream input.
- **docs/seeds/project-boundary.md** — Binding correction: product is the reusable substrate (Rust crate family); games are separate consumers; any walkable executable is a public-interface validation harness only; game/System/building layers are out of scope. Compatible workspace-boundary reasoning remains subordinate design input.
- **docs/seeds/product-one-seed.md** — Supplies a first “walkable world” proof shape and demo non-goals that motivate substrate outcomes; its character, camera, content palette, performance gates, and milestones do not transfer into current product identity. Compatible detailed requirements remain subordinate design input.
- **docs/seeds/voxel-world-substrate.md** — Supplies long-horizon substrate design goals (material natural worlds, deep Z, reusability across game genres) and marks game examples and many extensions nonbinding unless selected. Compatible mechanism and inventory detail remain subordinate design input.
