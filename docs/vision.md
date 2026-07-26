# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a **Rust crate** or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for external games—not a game, not a content pack, and not a particular demo experience.

## Purpose

Moria exists so multiple downstream games can share one material-world foundation: a natural-looking surface grounded in mutable voxel truth; deep underground as real content; and substrate-level matter, queries, mutation, and material-world behavior—including interactable objects, matter-responsive dressing, granular and fluid dynamics, thin ambient ecology, structural failure, and mutation-safe navigation over continuous 3D volume. Game rules, presentation policy, and authored gameplay stay above it. The substrate stands alone with no LLM or “System” dependency.

## Product boundary

**This product owns** the reusable world substrate: voxel-backed matter and objects, geology-oriented generation, consumer-facing mutation/query/event surfaces, material-world behaviors (integrity, fluids, granular settle, ambient ecology), derived navigation over continuous mutable volume, streaming of active regions, and persistence of edits and object change. Integration is through public Rust crate interfaces.

**Adjacent required delivery (not product identity):** a Product One walkable-world validation slice—substrate crates, a generated region, and a character proving dig/place mutability in a natural voxel-truth world—is part of what this repository builds first and what “done” means for that proof. Its executable is a separate validation consumer that exercises generation, streaming, view derivation, editing, collision against voxel truth, persistence, and performance, and must use the same public interfaces an external game would use—no privileged or game-specific substrate paths. Harness-owned controller, character, camera, demo route, authored region content, presentation, platforms, machine targets, milestones, and performance gates are not Moria product scope.

**Outside this repository / not this product:** the actual game and game layers—game rules, System/LLM features, spells, gas/pricing policy, combat, AI, and building layers. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

1. **Reusable Rust substrate with observable interaction.** Downstream games and the validation harness integrate only through public crate interfaces. Consumers mutate and observe world truth through public command, query, and event surfaces (commands in; mirror and change observation out); higher layers do not touch voxels directly. Concrete mirror realization is design, not identity. The consumer boundary is mandatory and non-privileged.
2. **Natural world on authoritative voxel truth.** Consumers can obtain a continuous, natural-looking world (surface terrain and cover, water bodies, underground volumes) whose presentation and collision are derived, non-authoritative views of underlying voxel matter. Physics and queries operate on voxel truth—not a heightmap with disconnected decoration as world authority.
3. **Mutable material everywhere, with deep-Z first-class.** Matter can be destroyed, moved, or placed throughout the volume; dig and place are substrate capabilities so scars remain coherent material. Underground structure (strata, caves, subsurface features) is real content so dig-down and descent-style play rest on geology and volume, not a shallow floor.
4. **Material-world behavior as substrate responsibility.** The substrate supplies interactable voxel-backed objects with object lifecycle and persistent object changes; matter-responsive surface dressing; granular behavior; dynamic fluids and material interactions; thin-but-present time, weather, and fire ecology; and structural failure—so honest material play is available to consumers. Pricing, combat, agent goals, and other game policy remain consumer-owned.
5. **Mutation-safe navigation over continuous 3D volume.** Consumers can obtain pathing/navigation derived from voxel truth across the continuous volume, with invalidation after world mutation and support for materially distinct movement classes—without the substrate implementing AI, agents, or fortress presentation.
6. **Generate, stream, persist; multi-game portable delivery.** Worlds generate as coherent geology, materialize on demand, stream around activity, and persist edits and object changes relative to generation. The same substrate can underpin ARPG, fortress/colony, descent, or sandbox consumers without embedding those games or any LLM/System stack, and remains implementable across common GPU backends without a load-bearing native-Metal fork.

## Future products and enabling implications

**Future / separate consumers** (not current product scope): the actual game outside this repository; System-driven ARPG play; fortress/colony modes; Moria-style descent; pure sandbox.

**Enabling implications** (substrate seams only): public verbs/queries/events games can price and script differently; extension points for materials and placement metadata so hand or System authorship can specialize regions without owning geology; seams for structure semantics where substrate requirements demand them—without implementing game building, combat, AI, or System layers here. Product One’s first-slice depth limits (partial matter behaviors, demo content) constrain that adjacent proof, not substrate identity.

## Non-goals

- Shipping the actual game or game-rule layers in this repository
- System/LLM features, spells, gas/pricing policy, combat, AI
- Building layers (gameplay construction systems, work orders, fortress machinery as game policy)
- Absorbing harness-owned controls, characters, demo content, routes, platforms, machine targets, or performance gates into the product promise
- A load-bearing native-Metal implementation path for the substrate crates

## Confirmed vision constraints

- Identity: reusable **GPU-resident voxel-world substrate**
- Delivery form: **Rust crate** or small family of tightly scoped Rust crates
- **Consumer boundary is not optional:** external games and the validation harness share public interfaces only; no privileged harness/game paths into substrate internals
- **Game / System / LLM / spell / gas / combat / AI / building layers** are out of scope here; seams only where the substrate itself requires them
- Substrate **stands alone with zero LLM dependency**
- **Cross-backend GPU portability** (including Metal, Vulkan, and DX12 paths) is part of the crate promise; no load-bearing native-Metal fork

## Deferred design decisions

- Exact crate split and packaging layout inside the Rust boundary
- Delivery depth and sequence for material-world outcome families (objects, dressing, granular, fluids, ambient ecology, integrity)—not whether those families exist in the substrate
- Algorithms, resolutions, data layouts, LOD, streaming policy, persistence encodings, and navigation graph realization
- Concrete command/mirror/event implementation; multiplayer readiness beyond the public interaction contract
- All harness-specific UX, content, platform, machine targets, milestones, and performance gates

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Establishes Moria as a reusable, GPU-resident voxel-world substrate consumed as a Rust crate, and separates the walkable-world executable as a consumer/validation harness for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md** — Binding identity and boundary: Rust substrate crate(s); game out of repo; harness is public-API validation only; game/System/LLM/spell/gas/combat/AI/building layers excluded.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural voxel-truth world with non-authoritative derived views, universal mutability, deep-Z geology, interactable objects and lifecycle, matter-responsive dressing, granular/fluid/ambient/integrity behavior, mutation-safe navigation, command/query/event interaction, generation/streaming/persistence, multi-game reusability without LLM) without redefining identity as a game.
- **docs/seeds/product-one-seed.md** — Defines the required adjacent Product One validation slice (substrate, generated region, character; dig/place proof; benchmarked playable demo) as what is built first and what “done” means for that proof; motivates crate-level cross-backend GPU portability; partial matter-slice limits and harness-specific controls, content, platforms, and gates stay with that adjacent consumer.
