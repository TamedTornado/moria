# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is the material world engine layer for games—not a game.

## Purpose

Provide a standalone world foundation so downstream games share the same mutable voxel truth: natural-looking surface terrain over fully diggable and placeable matter, with deep underground as first-class space. Game rules, content policy, and how play is presented live above the substrate. The substrate supplies matter, physics, queries, and mutation, and does not depend on an LLM or on any game system.

## Product boundary

**In product:** the reusable substrate and its public consumer interfaces for world generation, material storage and mutation, derived presentation of voxel truth, streaming, collision and queries against voxels, and persistence of edits.

**Adjacent, not this product:** the actual game is a separate downstream consumer and is not part of this repository. A walkable-world executable may exist in-repo only as an adjacent validation harness that consumes the same public interfaces available to an external game (see Q1). That harness’s character, camera, controls, demo route, content inventory, platform choices, and performance gates are harness-owned and are not substrate scope.

**Excluded from implementation here:** game rules and the System, LLM, spell, gas, combat, AI, and building gameplay layers. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented in this product.

## Required product outcomes

- Downstream consumers integrate a reusable Rust voxel-world substrate through public APIs only—no privileged in-repo access paths.
- The world is fully material voxel truth: volumes can be destroyed, moved, or placed; presentation is a non-authoritative view of that truth.
- Generated regions read as continuous natural worlds, with deep underground as real content space rather than a false floor.
- Consumers can mutate matter, query the world, and collide against voxel occupancy rather than against a separate decorative mesh.
- Large sparse worlds stream in and out of residency; edits persist as changes over generative truth so unloaded space stays consistent with play.
- The substrate stands alone with zero LLM dependency and remains usable by multiple future game styles without embedding their rules.

## Future products and enabling implications

Future consumers include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style deep descent experience, and pure sandbox modes. Those products own gameplay, UX, controllers, authored content, economy and pricing policy (including gas), and presentation of play. The substrate’s enabling role is shared matter, physics, query, and mutation foundations—and optional seams those games can attach to—without implementing those game layers here.

## Non-goals

- Shipping the actual game in this repository
- Implementing System/LLM features, spells, gas or pricing policy, combat, AI, or building/fortress gameplay systems
- Importing the walkable demo’s character, route, clip goals, content list, or machine-specific targets as substrate requirements
- Requiring LLM adjudication for the world substrate to function

## Confirmed vision constraints

- Product form is a Rust crate or a small family of tightly scoped Rust crates.
- Any in-repo validation executable, if present, must use only the public substrate interfaces available to an external game.
- World substrate operation is GPU-resident as part of product identity.
- The substrate must not require System or LLM features to function.

## Deferred design decisions

- Exact crate split, internal layering, and how the public consumer boundary is enforced in the repo layout.
- Delivery depth and sequence for matter and physics capabilities (generation fidelity, meshing approach, fluids, integrity, vegetation objects, ambient simulation, and similar). First-slice limits described for a walkable demo do not redefine product identity.
- Voxel resolution, streaming policy, persistence encoding, and performance budgets.
- Supported runtimes, hardware targets, and graphics backends beyond the Rust-crate integration surface.
- Whether and how multiplayer-oriented command and mirror patterns are realized.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world executable a required repository delivery for this phase, or only a permitted adjacent validation artifact?

*Proposed answer:* Permitted only. It is not part of product identity. If later required as a repository delivery, it remains an adjacent harness that must use public APIs; its controller, content, presentation, and acceptance details stay outside substrate scope.

*If different:* Making it mandatory adds a repository deliverable without expanding substrate identity. Forbidding it removes the in-repo validation path and leaves validation entirely to external consumers.

## Seed synthesis

- **README.md:** Names Moria as a reusable GPU-resident voxel-world substrate (Rust crate) and frames a walkable-world executable as a separate consumer and validation harness, not a game layer.
- **docs/seeds/project-boundary.md:** Binds the product to the substrate crate(s), places the real game outside the repo, requires any harness to use public interfaces, and excludes game, System, LLM, spell, gas, combat, AI, and building layers from implementation here.
- **docs/seeds/product-one-seed.md:** Describes an adjacent first walkable-world demo and harness slice (region, character, proof mutation, benchmarks). It motivates validation themes without transferring demo controls, content, or platform gates into current product scope.
- **docs/seeds/voxel-world-substrate.md:** Supplies the substrate’s purpose-level mandate—natural mutable worlds, deep Z, matter, physics, queries, mutation, multi-game reuse, and zero LLM dependency—while leaving mechanisms and build order to design.
