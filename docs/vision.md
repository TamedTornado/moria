# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate or small family of tightly scoped Rust crates. It is an engine-layer world foundation for downstream games—not a game, not an ARPG, and not a fortress/colony product.

## Purpose

Moria exists so future games can share one material world layer: natural-looking terrain whose truth is fully mutable voxels (including deep underground), exposed cleanly so game rules, content, and presentation stay above the substrate. The substrate must stand alone with no LLM or game-policy dependency.

## Product boundary

**This product owns** the reusable world substrate and the public consumption surface games use for that world—generation, streaming, query and mutation of matter, mesh/view derivation, collision against voxel truth, and persistence. High-level matter and geology responsibilities stay here even when future games motivate them.

**Adjacent, not product identity:** a walkable-world executable may exist only as a validation harness. If present, it must use the same public interfaces as an external game—no privileged or game-specific substrate paths. Its controller, character, camera, authored route and content, presentation, and performance gates are not substrate scope.

**Downstream / out of this product:** the actual game; game rules; System/LLM features; spells, gas policy, combat, AI, and building-as-gameplay layers. Compatibility seams may be designed where substrate needs demand them; those layers are not implemented here.

## Future products and enabling implications

Future consumers include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, and a Moria-style descent experience. They are not current scope. Enabling implication only: the substrate remains a reusable crate stack those games can sit on without forking privileged world implementation. Their gameplay, UX, controllers, characters, authored content, and presentation are not imported here. Broad long-horizon substrate capabilities described in design seeds are enabling design input for later planning, not a committed current roadmap.

## Non-goals

- Shipping a playable game or game-mode ruleset in this repository
- Implementing System/LLM, spell, gas, combat, AI, or building-game layers here
- Folding harness demo content, controls, routes, or performance theater into product identity

## Confirmed vision constraints

- The validation harness and any external game share one public interface surface; adjacent consumers get no privileged substrate access.
- The substrate is independent of LLM/System features and must remain usable without them.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a *required* current repository delivery, or only *permitted* as an adjacent consumer of the substrate crates?

- **Proposed answer:** Required as a thin adjacent validation executable that exercises the substrate only through public interfaces; its character, camera, authored world content, presentation, and performance gates stay outside product identity and scope.
- **If answered differently:** If only permitted, repository success is substrate crates alone. If required together with a rich demo acceptance story, delivery still must not redefine the product as that demo—but planning must schedule the adjacent artifact as mandatory.

## Seed synthesis

- **README.md** fixed the product name and identity as a GPU-resident voxel-world substrate Rust crate with a walkable executable framed as harness, not game; listed exercise areas remain subordinate design input.
- **docs/seeds/project-boundary.md** supplied the binding repository boundary: substrate crates in, real game and listed game layers out, harness-only executable with equal public interfaces; workspace/crate-split mechanics stay downstream technical design.
- **docs/seeds/product-one-seed.md** motivated early walkable validation pressure and described a demo consumer’s world, controls, and gates; those consumer specifics stay subordinate and do not redefine product identity—hence Q1.
- **docs/seeds/voxel-world-substrate.md** contributed the long-horizon substrate purpose (normal-looking, fully mutable, deep-Z, game-agnostic matter world) and multi-game enabling intent; its mechanisms, inventories, and milestones remain subordinate design input for later planning.
