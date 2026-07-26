# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation—not a game and not a gameplay vertical slice.

## Purpose

Moria exists so multiple games can share one honest material world: a natural-looking surface over fully mutable voxel truth, continuous in three dimensions including deep underground, with generation, mutation, queries, matter behavior, persistence, and streaming living below game rules. It must stand alone as a substrate with no dependency on an LLM or any particular game mode.

## Product boundary

**In product**
- The reusable substrate and its public interfaces for world generation, matter representation, meshing as a non-authoritative view, mutation and query access, matter-level simulation capabilities, persistence, and streaming.
- Compatibility seams where substrate requirements demand them, without implementing the game layers that would sit above those seams.

**Adjacent / not this product**
- The actual game (and future games that consume Moria) live outside this product and repository.
- A walkable-world executable may exist only as an adjacent validation harness that consumes the same public interfaces available to an external game. Whether that harness is a required deliverable is unresolved (see Q1); until resolved, it is not treated as product identity or as settled current delivery.
- Controllers, characters, cameras, authored demo content and routes, presentation polish, game UI, and acceptance scenarios belong to consumers or harnesses, not to the substrate product.

**Excluded from this product**
- Game rules and the System, LLM, spell, gas, combat, AI, and building layers. Those must not be implemented here.

## Required product outcomes

1. **Reusable Rust substrate.** Downstream games integrate Moria as crate(s) without forking privileged world paths; the substrate remains a standalone, GPU-resident engine layer with zero LLM dependency.
2. **Natural look, voxel truth.** The world can read as ordinary natural terrain while voxel matter—not decorative geometry or a heightmap with props—is authoritative for physics, queries, and mutation. Rendered surfaces are regenerated views of that matter.
3. **Mutable deep world, geology-first generation.** Any matter can be destroyed, moved, or placed; underground content is first-class in continuous 3D; worlds generate as geology and related natural structure and materialize on demand so large regions stay tractable.
4. **Matter-consistent world behavior.** Interactable features that can burn, break, or block stay backed by the matter model; lighter dressing stays derived from matter; the substrate supplies mutation verbs and mirror-style queries plus world-side fluid, integrity, granular, fire, and related ambient matter capabilities—without embedding game policy for pricing or use.
5. **Persistence, streaming, and sealed access.** Truth is generation plus edit deltas; active regions stream while untouched volume stays cheap; nothing above the matter layer touches voxels except through the public access model.
6. **Multi-consumer readiness.** The same substrate can underpin adventure, fortress-style, sandbox, and related modes; combat, AI, System behavior, and gas/pricing policy remain concerns above Moria.

## Future products and enabling implications

Future products are **downstream game consumers**, not expansions of Moria’s identity: a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox modes. Moria enables those by offering shared mutable matter, deep-Z geology, generation, streaming/persistence, and a non-privileged public API. Gameplay systems, authored content, presentation, characters, and mode-specific policy stay with those consumers. A walkable-world harness may exist as an adjacent consumer used to exercise substrate claims (see Q1)—not as a game layer and not as a substitute for those future products.

## Non-goals

- Shipping the actual game, its rules, combat, stats, AI, economy, or player-facing building gameplay.
- Implementing System/LLM features, spells, gas metering policy, or other game-layer services inside the substrate.
- Treating a validation harness’s character, camera, debug presentation, seed-route content, device model, or numeric performance gates as the product’s identity or promised environment.
- A Minecraft-cube aesthetic as the primary surface look, or non-mutable “fake” terrain that only looks interactive.
- Expanding this repository into game layers an explicit boundary already excludes, even when future consumers need them.

## Confirmed vision constraints

- Delivery form is the Rust crate ecosystem (one crate or a small family of tightly scoped crates).
- External games and any validation harness must use the same public interfaces; privileged or game-specific substrate paths are not allowed.
- Game rules and System, LLM, spell, gas, combat, AI, and building layers are out of scope for implementation here; compatibility seams may be designed only where the substrate itself requires them.
- The substrate must not depend on an LLM to function as a world engine layer.
- Authoritative world state is voxel matter; meshes and dressing are views or derived presentation, not sources of truth.

## Deferred design decisions

- How far each matter-capability family goes in any given delivery slice, and in what sequence capabilities are proven.
- Internal crate split, algorithms, data layouts, voxel size, LOD strategy, and similar mechanism choices.
- Harness-specific content, controls, presentation, workloads, platforms, and numeric performance gates (if a harness is in scope per Q1).
- Exact persistence encodings, streaming policy, and platform/backend implementation choices not fixed by product identity.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation executable a **required deliverable** of the current Moria effort, or only **permitted** as an adjacent artifact that may exist beside the substrate crates?

*Proposed safe answer:* Permitted only—current product delivery is the reusable substrate; a harness may exist beside the crates but is not required for product identity.

*If answered differently:* Requiring the harness expands delivery scope to include an adjacent executable that validates generation, streaming, meshing, editing, collision, persistence, and performance strictly through public APIs, without moving controllers, content, or game systems into the substrate product.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world substrate consumed as a Rust crate, and distinguishes the walkable-world executable as a separate consumer/validation harness rather than a game layer.
- **docs/seeds/project-boundary.md** — Fixes product identity (Rust substrate crates), repository boundary (game out of repo), harness-as-validation-only with public-interface consumption, and explicit exclusion of game/System/LLM/spell/gas/combat/AI/building layers.
- **docs/seeds/product-one-seed.md** — Describes a first walkable-world consumer slice that motivates dig/place proof, natural generated terrain, and public API discipline; its controller, seed content, milestones, device targets, and numeric gates remain consumer/harness concerns and do not redefine substrate identity.
- **docs/seeds/voxel-world-substrate.md** — Authorizes the substrate’s outcome families (natural look over voxel truth, full mutability, deep-Z, geology-first generation, matter behaviors, persistence/streaming, multi-game layering with zero LLM dependency) at product altitude without making mechanism inventory or future games part of this product.
