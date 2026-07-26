# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a **Rust crate** or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for external games—not a game, not a content pack, and not a particular demo experience.

## Purpose

Moria exists so multiple downstream games can share one material-world foundation: a natural-looking surface grounded in mutable voxel truth; deep underground as real content; and substrate-level matter, queries, mutation, and material-world behavior—including interactable objects, matter-responsive dressing, granular and fluid dynamics, thin ambient ecology, and structural failure. Game rules, presentation policy, and authored gameplay stay above it. The substrate stands alone with no LLM or “System” dependency.

## Product boundary

**This product owns** the reusable world substrate: voxel-backed matter and objects, geology-oriented generation, consumer-facing mutation and query surfaces, material-world behaviors (integrity, fluids, granular settle, ambient ecology), streaming of active regions, and persistence of edits and object change. Integration is through public Rust crate interfaces.

**Adjacent, not product identity:** a walkable-world executable **may** exist as a validation harness. It is not the game layer. When present, its purpose is to exercise and prove terrain generation, streaming, view derivation, editing, collision against voxel truth, persistence, and performance, and to serve as a first product-shaped proof of a generated walkable voxel-truth world with dig/place. Whether shipping that harness is a mandatory current delivery is unresolved (**Q1**). While open, this brief does not assign delivery status to the harness and does not treat it as product identity. If present, it must use the same public interfaces an external game would use—no privileged or game-specific substrate paths. Harness-owned controller, character, camera, demo route, authored region, presentation, platforms, machine targets, milestones, and performance gates are not Moria product scope.

**Outside this repository / not this product:** the actual game and game layers—game rules, System/LLM features, spells, gas/pricing policy, combat, AI, and building layers. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

1. **Reusable Rust substrate.** Downstream games (and any harness) integrate only through public crate interfaces; the consumer boundary is mandatory and non-privileged.
2. **Natural world on authoritative voxel truth.** Consumers can obtain a continuous, natural-looking world (surface terrain and cover, water bodies, underground volumes) whose presentation and collision are derived, non-authoritative views of underlying voxel matter. Physics and queries operate on voxel truth—not a heightmap with disconnected decoration as world authority.
3. **Mutable material everywhere, with deep-Z first-class.** Matter can be destroyed, moved, or placed throughout the volume; dig and place are substrate capabilities so scars remain coherent material. Underground structure (strata, caves, subsurface features) is real content so dig-down and descent-style play rest on geology and volume, not a shallow floor.
4. **Material-world behavior as substrate responsibility.** The substrate supplies interactable voxel-backed objects with object lifecycle and persistent object changes; matter-responsive surface dressing; granular behavior; dynamic fluids and material interactions; thin-but-present time, weather, and fire ecology; and structural failure—so honest material play is available to consumers. Pricing, combat, agent goals, and other game policy remain consumer-owned.
5. **Generate, stream, persist.** Worlds generate as coherent geology, materialize on demand, stream around activity, and persist edits and object changes relative to generation so long-lived worlds are possible without treating the whole volume as eager raw data.
6. **Multi-game, zero-LLM engine layer with portable GPU delivery.** The same substrate can underpin ARPG, fortress/colony, descent, or sandbox consumers without embedding those games or any LLM/System stack, and remains implementable across common GPU backends without a load-bearing native-Metal fork.

## Future products and enabling implications

**Future / separate consumers** (not current product scope): the actual game outside this repository; System-driven ARPG play; fortress/colony modes; Moria-style descent; pure sandbox. A product-one-style walkable proof is a validation or marketing consumer of the substrate, not a second product identity.

**Enabling implications** (substrate seams only): public verbs/queries games can price and script differently; extension points for materials and placement metadata so hand or System authorship can specialize regions without owning geology; optional seams for navigation or structure semantics where substrate requirements demand them—without implementing game building, combat, AI, or System layers here.

## Non-goals

- Shipping the actual game or game-rule layers in this repository
- System/LLM features, spells, gas/pricing policy, combat, AI
- Building layers (gameplay construction systems, work orders, fortress machinery as game policy)
- Absorbing harness-owned controls, characters, demo content, routes, platforms, machine targets, or performance gates into the product promise
- Device-specific atomics, single-machine acceptance contracts, or a load-bearing native-Metal implementation path

## Confirmed vision constraints

- Identity: reusable **GPU-resident voxel-world substrate**
- Delivery form: **Rust crate** or small family of tightly scoped Rust crates
- **Consumer boundary is not optional:** external games and any harness share public interfaces only; no privileged harness/game paths into substrate internals
- **Game / System / LLM / spell / gas / combat / AI / building layers** are out of scope here; seams only where the substrate itself requires them
- Substrate **stands alone with zero LLM dependency**
- **Cross-backend GPU portability** (including Metal, Vulkan, and DX12 paths) is part of the crate promise; no load-bearing native-Metal fork

## Deferred design decisions

- Exact crate split and packaging layout inside the Rust boundary
- Delivery depth and sequence for material-world outcome families (objects, dressing, granular, fluids, ambient ecology, integrity)—not whether those families exist in the substrate
- Algorithms, resolutions, data layouts, LOD, streaming policy, and persistence encodings
- Harness delivery status beyond **Q1**, and all harness-specific UX, content, platform, and performance choices
- Multiplayer or command/mirror realization details

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **mandatory current delivery** alongside the substrate crates, or **only a permitted adjacent artifact**?

- **Proposed safe answer:** Permitted only—the product is complete as the substrate crates; a harness may exist later and must use public APIs, but is not required for current product completeness.
- **If answered differently:** Making the harness mandatory adds a sibling deliverable (still not product identity) that design must plan and accept; it does not move controller, content, platforms, or performance gates into the substrate, but it does change what “done” means for this repository’s current delivery.

## Seed synthesis

- **README.md** — Establishes Moria as a reusable, GPU-resident voxel-world substrate consumed as a Rust crate, and separates the walkable-world executable as a consumer/validation harness for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md** — Binding identity and boundary: Rust substrate crate(s); game out of repo; harness may exist only as public-API validation; game/System/LLM/spell/gas/combat/AI/building layers excluded.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural voxel-truth world with non-authoritative derived views, universal mutability, deep-Z geology, interactable objects and lifecycle, matter-responsive dressing, granular/fluid/ambient/integrity behavior, generation/streaming/persistence, multi-game reusability without LLM) without redefining identity as a game.
- **docs/seeds/product-one-seed.md** — First-slice validation consumer motivating early proof of a walkable natural world and dig/place mutability, collision on voxel truth, and crate-level cross-backend GPU portability; partial matter-slice limits apply to that consumer’s first delivery, not to substrate identity; harness-specific controls, content, platforms, and gates stay adjacent.
