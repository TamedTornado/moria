# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate for natural-looking, fully mutable 3D worlds. It is delivered as a Rust crate (or a small family of tightly scoped crates) for external games to consume—not as a game, demo title, or content product.

## Purpose

Games that need Minecraft-grade dig and build, deep underground play, and a surface that still reads as ordinary terrain should share one engine-layer foundation for matter, world generation, mutation, and queries. Moria exists so that foundation is real, reusable, and free of game rules or LLM dependency, rather than rebuilt per title or locked inside a single demo.

## Product boundary

**In product:** the substrate itself—the reusable world/matter layer and the public interfaces through which consumers generate, stream, inspect, and mutate voxel truth and obtain views suitable for rendering and simulation coupling.

**Adjacent, not the product:** a walkable-world executable, if present, is only a validation harness. It must use the same public substrate interfaces an external game would use; it does not own privileged paths. Its character, camera, controls, authored seed content, demo route, presentation, and performance gates are harness concerns, not substrate scope.

**Out of this repository:** the actual game and all game-owned layers (rules, System/LLM, spells, gas policy, combat, AI, building gameplay, UX, and authored campaign content). Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Future products and enabling implications

Downstream consumers described in the seeds include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, pure sandbox modes, and later “product two” titles. They are future or external products, not current Moria scope.

High-level enabling implications the substrate is meant to support (without importing consumer gameplay or content): continuous deep-Z geology as first-class space; mutable matter everywhere with dig/place as substrate verbs; natural surface worlds whose look is a view over voxel truth; and clean layering so gas/pricing policy, AI, and game rules stay above the crate boundary.

## Non-goals

- Shipping a game, ARPG, fortress mode, or spell/System stack in this repository
- Treating the validation harness’s controller, character, seed tour, or clip goals as product identity
- Embedding LLM/System generation or game policy inside the substrate

## Confirmed vision constraints

- Repository product is the reusable substrate; the game lives elsewhere
- Cargo workspace (or equivalent) keeps substrate and any harness on opposite sides of a public API boundary
- Substrate stands alone with zero LLM dependency
- GPU-resident world/matter direction is part of the product identity

## Assumptions proposed for approval

1. Public mutation and query surfaces that let a consumer dig, place, and collide against voxel truth are substrate obligations; how a harness proves them (keys, character, camera, tour) is not.
2. Long-horizon matter behaviors (richer fluids, integrity, weather, building semantics, and similar) remain enabling substrate implications, not a committed current delivery catalog or milestone list.

## Questions for human review

**Q1.** Is the walkable-world validation executable a **required current deliverable** of this repository, or only a **permitted** adjacent consumer?

- **Proposed safe answer:** Permitted and encouraged for validation, but not mandatory for the substrate product to be considered itself; if built, it remains a harness on public APIs.
- **If different:** Making it mandatory expands repository “done” to include an executable consumer product (without transferring its controls, content, or acceptance details into substrate identity); forbidding it narrows validation options to external or later consumers only.

## Seed synthesis

- **`README.md`:** Named the product Moria; stated substrate-as-crate identity and that the walkable-world executable is a separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Locked current product to the reusable substrate and public-API harness rule; excluded game rules and System/LLM/spell/gas/combat/AI/building layers; made the consumer boundary non-optional while leaving crate split to design.
- **`docs/seeds/product-one-seed.md`:** Motivated a first proof-shaped walkable scenario and dig/place as undeniable mutability proof; supplied harness-oriented non-goals and future “product two” pointers. Specific region content, controller/camera, materials lists, performance gates, and milestones were treated as adjacent validation detail, not current product scope.
- **`docs/seeds/voxel-world-substrate.md`:** Defined substrate purpose (natural look over voxel truth, mutability, deep Z, reusable engine layer, GPU-resident stance) and future multi-game consumption; detailed mechanisms, inventories, and build order deferred to design. Reinforced that game rules and the System live above the substrate.
