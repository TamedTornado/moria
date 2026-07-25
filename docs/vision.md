# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate (or a small family of tightly scoped Rust crates) for external games and tools to consume.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface over fully mutable voxel truth, continuous deep underground play, and public matter queries and mutations—without each consumer re-implementing the world engine or embedding game rules in the substrate.

## Product boundary

**In product:** the substrate outcomes games need from a world engine—generation of geology-first regions, GPU-resident mutable matter, non-authoritative visual presentation of that matter, and a public interface for queries and mutation—such that adjacent consumers have no privileged access paths.

**Out of product (adjacent or downstream):** the actual game; game rules; System/LLM, spell, gas, combat, AI, and building layers; and any walkable-world executable’s controller, character, camera, authored demo content, presentation, routes, debug UX, workload, or performance gates. That executable, if present, is a validation harness and external-style consumer only—not a game layer and not the product identity.

Compatible detailed requirements in the seeds (storage layout, meshing choices, APIs, milestones, numbers) remain subordinate input to later design and do not expand this boundary.

## Future products and enabling implications

Future consumers include a System-driven ARPG, a fortress/colony-style game, a descent-style adventure, and pure sandbox modes. They own gameplay, policy, content, and presentation.

Enabling implications supported by the seeds (not a committed multi-product roadmap): the substrate should remain reusable across those modes; matter, mutation, and queries stay game-rule free; long-horizon capabilities such as richer fluids, structural integrity, ambient simulation, and building-oriented verbs may later sit in the substrate stack where a seed already assigns them there—but they are not declared current delivery by this brief.

## Non-goals

- Implementing the actual game or its rules, UI, combat, AI, economy, or System/LLM features in this product.
- Treating the validation harness’s demo route, character fantasy, or acceptance scenario as substrate scope.
- Shipping game-layer building, spell, gas, or agent-labor systems as part of Moria’s current identity.

## Confirmed vision constraints

- The product is consumed as Rust crate(s); the intended integration ecosystem is Rust/Cargo library use by external games and tools.
- Any validation harness in-repo must use the same public interfaces available to an external game—no privileged or game-specific substrate paths.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world validation harness **mandatory current delivery**, or only **permitted** beside the substrate crates?

- **Proposed safe answer:** Permitted, not mandatory—the product ships when the reusable substrate is consumable; a harness may exist to exercise public interfaces but is not required for product completeness.
- **If different:** Mandatory delivery keeps product identity on the substrate but adds “ship a harness that consumes public APIs” as a settled adjacent deliverable (still without importing controller, content, or performance gates into substrate scope). “Permitted only” means crate completeness alone can satisfy current delivery.

## Seed synthesis

- **README.md** — Named the product Moria, fixed identity as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and cast the walkable-world executable as a separate validation consumer, not a game layer; no further detailed requirements beyond that framing.
- **docs/seeds/project-boundary.md** — Locked repository and consumer boundary (substrate in, actual game out), required public-interface-only harness consumption, and excluded game-rule layers while allowing compatibility seams; crate-split and other enforcement mechanics stay downstream.
- **docs/seeds/product-one-seed.md** — Motivated early vertical proof of a material walkable world and listed demo-oriented non-goals; its character, seed content, milestones, and performance gates are consumer/harness material and remain subordinate design input, not current-product identity.
- **docs/seeds/voxel-world-substrate.md** — Supplied the long-horizon substrate purpose (normal-looking mutable world, deep Z, substrate-not-game, multi-game reuse) and high-level enabling capabilities; algorithms, formats, feature tiers, and build-order detail remain subordinate design input and do not expand current committed scope.
