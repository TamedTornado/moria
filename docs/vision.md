# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a **Rust crate** or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for external games—not a game, not a content pack, and not a particular demo experience.

## Purpose

Moria exists so multiple downstream games can share one material-world foundation: a natural-looking surface whose appearance is grounded in mutable voxel truth; deep underground space as real content rather than a painted floor; and substrate-level matter, physics-relevant world behavior, queries, and mutation. Game rules, presentation policy, and authored gameplay stay above it. The substrate must stand alone with no LLM or “System” dependency.

## Product boundary

**This product owns** the reusable world substrate: voxel-backed matter, geology-oriented world generation, consumer-facing mutation and query surfaces, and the world behaviors the substrate is responsible for (including structural-integrity-relevant and fluid-relevant matter behavior, streaming of active regions, and persistence of edits). Integration is through public Rust crate interfaces.

**Adjacent, not product identity:** a walkable-world executable **may** exist as a validation harness. It is not the game layer. Whether shipping that harness is a mandatory current delivery is unresolved (**Q1**). While open, this brief does not treat the harness as required or as part of product identity. If present, it must use the same public interfaces an external game would use—no privileged or game-specific substrate paths. Harness-owned controller, character, camera, demo route, seed content, presentation, workloads, platforms, and performance gates are not Moria product scope.

**Outside this repository / not this product:** the actual game and game layers—game rules, System/LLM features, spells, gas/pricing policy, combat, AI, and building layers. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

1. **Reusable Rust substrate.** Downstream games (and any harness) integrate only through public crate interfaces; the consumer boundary is mandatory and non-privileged.
2. **Natural world on voxel truth.** Consumers can obtain a continuous, natural-looking world (surface terrain and cover, water bodies, underground volumes) whose visible surface is a view of underlying voxel matter—not a heightmap with disconnected decoration as world authority.
3. **Mutable material everywhere.** Matter can be destroyed, moved, or placed throughout the volume; dig and place are substrate capabilities so scars and constructions remain coherent material, not cosmetic overrides.
4. **Deep-Z as first-class.** Underground structure (strata, caves, subsurface features) is real content so dig-down and descent-style play rest on geology and volume, not a shallow floor.
5. **Matter, queries, mutation, physics-relevant world behavior.** The substrate supplies material state, spatial/matter queries, mutation, and world behaviors needed for honest material play (including integrity-relevant failure and fluid-related behavior at substrate altitude). Pricing, combat, agent goals, and other game policy remain consumer-owned.
6. **Generate, stream, persist.** Worlds generate as coherent geology, materialize on demand, stream around activity, and persist edits relative to generation so long-lived and reusable worlds are possible without treating the whole volume as eager raw data.
7. **Multi-game, zero-LLM engine layer.** The same substrate can underpin ARPG, fortress/colony, descent, or sandbox consumers without embedding those games or any LLM/System stack.

## Future products and enabling implications

**Future / separate consumers** (not current product scope): the actual game outside this repository; System-driven ARPG play; fortress/colony modes; Moria-style descent; pure sandbox. Product-one-style walkable demos are validation or marketing consumers of the substrate, not a second product identity.

**Enabling implications** (substrate seams only): public verbs/queries games can price and script differently; extension points for materials and placement metadata so hand or System authorship can specialize regions without owning geology; optional seams for navigation or structure semantics where substrate requirements demand them—without implementing game building, combat, AI, or System layers here.

## Non-goals

- Shipping the actual game or game-rule layers in this repository
- System/LLM features, spells, gas/pricing policy, combat, AI
- Building layers (gameplay construction systems, work orders, fortress machinery as game policy)
- Absorbing harness-owned controls, characters, demo content, routes, or device-specific acceptance contracts into the product promise

## Confirmed vision constraints

- Identity: reusable **GPU-resident voxel-world substrate**
- Delivery form: **Rust crate** or small family of tightly scoped Rust crates
- **Consumer boundary is not optional:** external games and any harness share public interfaces only; no privileged harness/game paths into substrate internals
- **Game / System / LLM / spell / gas / combat / AI / building layers** are out of scope here; seams only where the substrate itself requires them
- Substrate **stands alone with zero LLM dependency**

## Deferred design decisions

- Exact crate split and packaging layout inside the Rust boundary
- Delivery depth and sequence for generation, view derivation, matter behaviors (fluids, integrity, objects, ambient rules), and related capabilities
- Algorithms, resolutions, data layouts, LOD, streaming policy, and persistence encodings
- Harness existence beyond the open delivery question, and all harness-specific UX, content, platform, and performance choices
- Multiplayer or command/mirror realization details

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **mandatory current delivery** alongside the substrate crates, or **only a permitted adjacent artifact**?

- **Proposed safe answer:** Permitted only—the product is complete as the substrate crates; a harness may exist later and must use public APIs, but is not required for current product completeness.
- **If answered differently:** Making the harness mandatory adds a sibling deliverable (still not product identity) that design must plan and accept; it does not move controller, content, or performance gates into the substrate, but it does change what “done” means for this repository’s current delivery.

## Seed synthesis

- **README.md** — Establishes Moria as a reusable, GPU-resident voxel-world substrate consumed as a Rust crate, and separates the walkable-world executable as consumer/validation rather than game layer.
- **docs/seeds/project-boundary.md** — Binding identity and boundary: Rust substrate crate(s); game out of repo; harness may exist only as public-API validation; game/System/LLM/spell/gas/combat/AI/building layers excluded.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural voxel-truth world, universal mutability, deep-Z geology, matter/queries/mutation and related world behavior, generation/streaming/persistence, multi-game reusability without LLM) without redefining identity as a game.
- **docs/seeds/product-one-seed.md** — First-slice validation consumer motivating early proof of a walkable natural world and dig/place mutability; harness-specific controls, content, platforms, and gates stay adjacent and do not narrow substrate identity.
