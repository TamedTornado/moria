# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer library for games and tools to consume. It is not a game, and it does not own game rules or presentation of a finished playable title.

## Purpose

Moria exists so multiple future games can share one stand-alone world foundation: continuous, fully mutable voxel matter that can read as a natural surface world, treats deep underground as first-class space, and provides the matter, physical matter behaviors, queries, and mutation surfaces those games need—without embedding any one game’s rules, economy, combat, AI, or LLM-driven System, and without requiring an LLM to operate.

## Product boundary

**In product**

- The reusable world substrate: material voxel world, geology-oriented generation, non-authoritative visual presentation over voxel truth, mutation and query surfaces, substrate-owned physical matter behaviors, and world-layer streaming and edit persistence for external consumers.
- Integration as a public Rust library boundary that any consumer—including an in-repo validation harness—must use without privileged paths.

**Adjacent, not product identity**

- A walkable-world executable *may* live in this repository as a validation harness that consumes those public interfaces. While [Q1](#questions-for-human-review) is open, it is neither classified as required nor optional delivery; only that such an artifact may exist and, if it does, must not own game-specific or privileged substrate paths. Its character, controls, camera, authored route, seed content, fixtures, workloads, machine targets, and performance gates remain harness-owned.

**Out of product**

- The actual game and repository-external game products.
- Game rules and the System/LLM, spell, gas/pricing-policy, combat, AI, and game building layers. Compatibility seams may be designed where substrate needs demand them; those layers are not implemented here.

## Required product outcomes

1. **Reusable Rust substrate.** External products integrate Moria as a library and receive a world engine, not a shipped game.
2. **Fully material, continuous world.** Occupied space is mutable matter end to end—surface through deep vertical extent—so dig, destroy, move, and place are meaningful anywhere the world exists; deep Z is content, not a decorative floor.
3. **Natural reading over voxel truth.** Generated worlds can present continuous, natural surface and underground spaces while interaction and simulation remain on voxel truth; presentation is a regenerated view and is never the authority or the save.
4. **Shared world services.** The substrate provides generation, matter mutation and spatial/matter queries, substrate-owned physical matter behaviors that make dig, flood, collapse, and similar world consequences real for consumers, and streaming and persistence of world edits so genre-specific games can sit above one world stack.
5. **Policy-free engine core.** Verb pricing, labor, mana, and other game policies are not hard-wired; the substrate runs with zero LLM dependency. Higher layers may attach later; they are not required for Moria to stand alone.
6. **Equal public access.** No consumer path—including any in-repo harness—bypasses the public interfaces available to an external game.

## Future products and enabling implications

Described future products are **consumers**, not Moria itself: a System/LLM-driven ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style descent experience, and pure sandbox modes. Enabling implications only: one crate stack must remain usable across dig/build-heavy play, deep vertical adventure, structural and fluid-heavy construction fantasies, and interchangeable higher-layer policy. Their gameplay, content, controllers, characters, animation, UX, and acceptance scenarios are not Moria scope.

## Non-goals

- Shipping a playable commercial or genre game in this product
- Implementing System/LLM authorship, spells, gas economies, combat, or gameplay AI
- Implementing game-layer building UX, blueprints-as-gameplay, work orders, rooms-as-game systems, or economy
- Absorbing harness demo content, third-person avatar design, camera policy, scripted routes, or harness benchmark gates into substrate identity
- Treating first-slice harness limits as a permanent ceiling on substrate responsibility

## Confirmed vision constraints

- Product form is a Rust crate or small family of tightly scoped Rust crates for GPU-resident voxel world use.
- The consumer boundary is mandatory: validation and games share public interfaces; layout that enforces that split is design, the boundary itself is not optional.
- The substrate must stand alone with no LLM dependency.
- Game rules and the future System, LLM, spell, gas, combat, AI, and building layers stay out of implementation here; seams only where substrate requirements demand them.

## Deferred design decisions

- Capability depth and delivery sequence within the substrate responsibilities above
- Crate family split, internal module boundaries, and repository layout details
- Representation, simulation, meshing, generation, streaming, and persistence mechanisms
- Performance budgets, supported machines, graphics backends, and validation workloads
- How far multiplayer-oriented command boundaries are carried beyond clean public mutation/query surfaces

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** For the current product cycle, is an in-repo walkable-world validation harness a **required repository delivery**, or only **permitted** as an adjacent artifact?

**Proposed safe answer:** Permitted only—not a required delivery. If present, it must use the same public substrate interfaces as an external game and remains outside product identity.

**If different:** Requiring the harness does not change Moria’s identity as a reusable substrate, but it adds a repository delivery that design must plan; it still must not import harness-owned character, content, presentation, route, or performance gates into product outcomes.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and places the walkable-world executable as a separate validation consumer rather than a game layer.
- **docs/seeds/project-boundary.md** — Locks current product identity to the crate-level substrate, forbids privileged harness paths, excludes game and listed future game layers, and permits (does not itself mandate) an in-repo validation executable.
- **docs/seeds/product-one-seed.md** — Supplies an adjacent first-slice walkable demo/harness vision that motivates public proof of material mutability and natural reading; does not redefine product identity or import demo content, controls, or platform gates into the substrate.
- **docs/seeds/voxel-world-substrate.md** — Supplies the long-horizon substrate purpose: natural-looking fully mutable worlds, first-class deep Z, and reusable matter/physics/query/mutation foundations without LLM dependency, for multiple future game consumers.
