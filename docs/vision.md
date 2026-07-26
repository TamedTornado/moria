# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer foundation for natural, fully material 3D worlds—not a game, not a content pack, and not a privileged demo shell.

## Purpose

Games and tools need a shared world layer where geology, surface scenery, underground depth, and player-visible form all rest on the same mutable matter—not decorative terrain with props on top. Moria exists so multiple downstream products (sandbox, descent, fortress-style, and System-driven ARPG among them) can consume one substrate for matter, generation, queries, mutation, and related world physics without embedding game rules or depending on an LLM.

## Product boundary

**In product**
- The reusable substrate and its public interfaces for world generation, matter storage and presentation, mutation, query, streaming, and persistence.
- Compatibility seams the substrate itself requires so later game layers can attach without forking core paths.

**Adjacent, not product identity**
- A walkable-world executable may exist as a validation harness. It is a separate consumer of the substrate, not a game layer. Whether shipping that harness is part of current delivery is open (see Q1). While open, treat it only as a permitted adjacent artifact—not as required, optional, planned, or in-scope delivery content.
- If present, the harness must use the same public interfaces an external game would use; it must not own privileged or game-specific implementation paths inside the substrate.

**Out of repository / downstream**
- The actual game(s), game rules, and future System, LLM, spell, gas, combat, AI, and building layers. Those layers are not implemented here.
- Harness- or demo-owned presentation, character control, authored demo routes, debug UX, scripted benchmark scenes, and acceptance numbers—unless a later approved boundary moves a specific item in.

## Required product outcomes

Downstream design must make these true for the substrate:

1. **Material world, not heightmap theater.** Consumers get a natural-looking surface world (terrain, water bodies, vegetation, clutter) whose visible form is grounded in voxel matter; the world must not read as a heightmap with disconnected props.
2. **Mutable everywhere, deep Z first-class.** Any region of matter can be destroyed, altered, or placed; underground depth (caves, strata, buried content hooks) is real playable volume, not a false floor.
3. **Geology-first generation with cheap idle cost.** Worlds are produced as coherent geology (columns, strata, caves, ores, biome-relevant surface) such that untouched volume stays cheap (lazy materialization / sparse residency) until touched by render, sim, query, or mutation.
4. **Voxel truth, mesh as view.** Simulation, collision-relevant occupancy, and queries run against matter; extracted surfaces are regenerated views, not the authoritative save or physics source.
5. **Public mutation and query only.** Dig, place, and related world operations and mirror-style queries go through substrate interfaces so nothing above the matter layer touches voxels directly—the reuse, sandbox, and multiplayer-readiness boundary.
6. **Streamable, scar-cheap persistence.** Truth is regenerable world function plus edit deltas (and object/entity journals where the substrate owns those objects), with streaming around active anchors so large regions remain tractable.

## Future products and enabling implications

**Future consumers (not this product):** System-driven ARPG, Dwarf Fortress–style fortress/colony play, Moria-style descent, pure sandbox, and any other game that sits above the substrate. Product-one-style walkable demos are consumer/harness shapes, not the substrate’s identity.

**Enabling implications (substrate may need to support over time; depth and sequence are design):**
- Richer matter simulation (granular settle, fire/wetness-style cellular rules, multi-tier fluids, structural integrity) so caves, floods, and collapses stay honest for fortress and adventure games.
- Voxel-backed objects (trees, boulders, micro props) and dressing derived from matter so interaction, felling, and burn remain substrate-true when consumers turn those verbs on.
- Placement/stamp and mechanism footprints as substrate verbs and object participation—without implementing game building UX, labor, or economy here.
- Column/nav-style derived views and priced-verb architecture so Z-slice fortress UX and server-authoritative multiplayer remain possible without redesigning matter.

Do not import consumer gameplay, controllers, characters, animation, authored content, gas policy, or System behavior into Moria scope.

## Non-goals

- Implementing a shippable game, combat, stats, AI agents, spells, gas metering policy, or LLM/System runtime inside this repository.
- Implementing building-game layers (player building UX, work orders, fortress designations, economy) here—even when the substrate exposes low-level placement or object hooks.
- Treating decorative-only geometry, heightmap-only terrain, or mesh-authoritative physics as the product model.
- Making the substrate depend on an LLM to generate or run the world.
- Absorbing harness-specific demo content, routes, characters, or performance scoreboards into the substrate’s definition of done.

## Confirmed vision constraints

- **Ecosystem:** The product is consumed as a Rust crate (or small family of tightly scoped Rust crates) by external games and by any in-repo harness.
- **GPU-resident world substrate:** Core world residency and work are GPU-resident engine concerns, not a CPU-only voxel toy.
- **Strict consumer boundary:** Adjacent consumers have no privileged access; validation must exercise public interfaces.
- **Layering:** Game rules and System/LLM live above the substrate; the substrate stands alone with zero LLM dependency.
- **Out-of-scope layers stay out:** System, LLM, spell, gas, combat, AI, and building layers are not implemented in this product even when seams are reserved.

## Deferred design decisions

- Precise crate split and internal module boundaries (workspace enforcement shape is design; the consumer boundary is not).
- Voxel scale, LOD strategy, object-layer capacity limits, and fluid-model fidelity tradeoffs.
- How much matter simulation (CA, integrity, fluid tiers, felling/rigid coupling) ships in which delivery slice.
- Persistence encoding, streaming ring policy, and synchronization patterns.
- Whether and how multiplayer authority is exercised beyond keeping the verb/command boundary clean.
- Harness content, controls, platforms, and numeric acceptance gates—if a harness is delivered (Q1).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1. Is a walkable-world validation harness part of current delivery, or only permitted?**
Project boundary language allows a harness; the Product One seed treats a walkable demo as the first built artifact with its own player, route, and numbers.  
**Proposed safe answer:** Permit the harness as an adjacent consumer that must use public APIs, but do not make it mandatory for substrate identity—design may still schedule one without expanding product scope to include its controls, content, or performance gates.  
**If answered differently:** Making the harness mandatory adds a required adjacent deliverable (still not game content inside the crate); forbidding it entirely removes in-repo validation-by-walking and pushes all proof to external consumers or other tests.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world substrate (Rust crate) and separates the walkable-world executable as consumer/harness, not game layer.
- **docs/seeds/project-boundary.md** — Locks current product identity to the substrate crate(s), requires public-API-only harness access, excludes the actual game and listed future layers from this repository.
- **docs/seeds/product-one-seed.md** — Motivates first consumer-visible proof (generated walkable material world, dig/place honesty) and a harness-shaped demo; supplies first-slice exclusions and metrics that inform validation adjacency, not substrate identity expansion.
- **docs/seeds/voxel-world-substrate.md** — Defines substrate purpose and outcome families (natural material worlds, full mutability, deep Z, geology generation, matter/physics/query/mutation, layering rules) and long-horizon enabling capabilities for future games without making those games in-repo.
