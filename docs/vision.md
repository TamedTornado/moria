# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a **Rust crate** (or a small family of tightly scoped Rust crates) for external games and tools to consume. It is an engine-layer product: world matter, generation, presentation of that matter as a normal-looking world, queries, and mutation—not a game.

## Purpose

Moria exists so multiple game styles can share one material world foundation: a natural-looking surface over fully mutable voxel truth, with deep underground as real space, without each game re-implementing geology, matter, meshing, and world I/O. The substrate must stand alone with **zero dependency on an LLM or “System” layer**. Game rules, economy, and content authorship live above it.

## Product boundary

**In product:** the reusable substrate and its public integration surface for consumers written against the Rust crate boundary.

**Adjacent, not identity:** a walkable-world executable may exist as a **validation harness** that exercises the substrate. It is not a game layer and must not define product identity. Whether that harness is a required delivery is unresolved (see Q1). If present, it must use the **same public interfaces** available to an external game—no privileged or game-specific substrate paths.

**Out of repository / downstream:** the actual game (or games). Game rules and the System, LLM, spell, gas, combat, AI, and building **game** layers are not implemented here. Compatibility seams may be designed where substrate requirements demand them; those layers stay outside this product.

**Consumer-owned (even when a harness demonstrates them):** character control, camera, authored demo routes and content, presentation polish beyond substrate look guarantees, game-specific policy, acceptance scenarios, and platform- or hardware-specific performance gates chosen by a consumer.

## Required product outcomes

Downstream design must make these true of the substrate product:

1. **Normal-looking material world.** Consumers can present rolling terrain, forests, rivers, cliffs, and meadows where the voxel grid is the authority and the visible surface reads as a natural world, not a cube aesthetic as the primary look.
2. **Mutable geology through deep Z.** Any volumetric cell in play can be destroyed, changed, or placed; underground is first-class content; worlds are generated as coherent geology that materializes on demand so dig and explore reveal true structure, not a heightmap skin over filler.
3. **Matter-consistent life and dynamics.** Interactable vegetation and clutter stay consistent with voxel truth; the substrate provides fluid bodies and flow, structural support and failure, granular settle, and light ambient matter-linked world behavior so the surface and underground behave honestly under dig, load, and environment.
4. **Public matter API.** Mutation, queries, navigation data derived from matter, and related events go through a consumer-facing surface so nothing above the matter layer touches voxels directly—the reuse, sandbox, and multiplayer-readiness boundary.
5. **Persistence and streaming.** Truth is regenerable world definition plus edit deltas; activity-centered residency keeps large regions tractable without treating the entire volume as eagerly resident.
6. **Multi-game reuse.** The same Rust crate stack supports adventure, fortress/colony, descent, and sandbox-style consumers by supplying matter, physics, queries, and mutation—not by embedding a single game’s rules.

## Future products and enabling implications

**Downstream consumers (not this product):** the actual game(s)—including a System/LLM-driven ARPG, a fortress/colony mode, a Moria-style descent fantasy, and pure sandbox use. A first walkable-world demo or harness may prove the substrate but does not become the product.

**Enabling implications (not a committed roadmap inventory):** games can later own combat, AI, spells, gas pricing, building UX and work policy, and System-authored content while relying on Moria for world truth, mutation, and material behavior—without importing game narrative or System adjudication into this product.

## Non-goals

- Shipping the actual game, its rules, or game-only layers (System/LLM, spells, gas policy, combat, AI, building-as-gameplay).
- Treating a walkable demo’s character, route, content set, or marketing milestones as the substrate’s identity or full scope.
- Privileged harness or in-repo paths that external games cannot share.
- Making the substrate depend on an LLM to generate or run the world.

## Confirmed vision constraints

- **Rust crate integration:** the product is consumed as a Rust crate or small family of tightly scoped Rust crates.
- **Strict consumer boundary:** adjacent executables and external games share the public interface; privileged substrate access is disallowed.
- **Repository boundary:** the actual game is a separate downstream consumer, not part of this product’s repository scope.
- **Standalone substrate:** zero LLM/System dependency in the load-bearing world layer.
- **GPU-resident world substrate:** the product promise includes GPU-resident world matter suitable for large, sparse regions (depth and sequence of residency mechanisms are design).

## Deferred design decisions

- Precise crate/package split and workspace layout (boundary intent is fixed; structure is not).
- Capability depth and delivery sequence (e.g. first vertical slice vs full fluid, integrity, vegetation, and ambient suites).
- Meshing strategy details, voxel size, LOD, storage layouts, and sim algorithms.
- How far multiplayer-ready command/authority patterns are designed in the first delivery.
- Harness existence, content, controllers, platforms, and numeric acceptance gates—if the harness is in delivery scope after Q1.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** of this effort, or only **permitted** as an adjacent artifact?

- **Proposed safe answer:** Permitted adjacent harness only—product identity and required outcomes stay on the substrate; if a harness is built, it must use public interfaces solely.
- **If answered “required”:** the brief gains a delivery obligation that a public-interface harness exist and exercise substrate capabilities, still without absorbing harness-owned control, content, presentation, or performance gates into product identity.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and positions a walkable-world executable as a separate validation consumer rather than a game layer.
- **`docs/seeds/project-boundary.md`:** Locks product identity to the substrate and Rust crate boundary, excludes the game and listed game layers from the repo, and sets the non-optional public-interface consumer boundary (harness delivery as “may”).
- **`docs/seeds/product-one-seed.md`:** Describes a first walkable proof slice and demo motivations; supplies evidence for why dig/place, continuous material world, and API discipline matter, without transferring demo content, controller, hardware gates, or milestone plan into substrate identity.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes the substrate outcome families—normal look from voxel truth, full mutability, deep Z, geology-first generation, matter-consistent dressing, fluids/integrity/granular behavior, public verbs/queries, persistence/streaming, and multi-game reuse without LLM dependency.
