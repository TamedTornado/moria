# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for external games—not a game, demo content pack, or gameplay stack.

## Purpose

Moria exists so multiple games can share one material world: a natural-looking surface over fully mutable voxels, with deep underground play as first-class content, and with game rules living above the substrate. The substrate must stand alone with no LLM or System dependency. Downstream titles (ARPG, fortress/colony, descent, sandbox) consume the same matter, queries, and mutation surface rather than each rebuilding a world engine.

## Product boundary

**This product owns:** the reusable voxel-world substrate—geology-oriented generation, sparse GPU-resident matter, presentation of that matter as a non-authoritative view, matter-coherent surface dressing and voxel-backed interactable objects, mutation and query surfaces, collision against voxel truth, streaming and persistence of world state, and compatibility seams where substrate requirements need them.

**Adjacent, not this product:** a walkable-world executable may exist as a validation harness that exercises the substrate through the same public interfaces available to an external game. Whether that harness is a required repository delivery is unresolved (see Q1). Harness-owned items—controllers, characters, cameras, authored demo routes, seed-world set dressing, debug presentation, scripted workloads, and performance gates—do not become product scope by implication.

**Downstream / future consumers own:** the actual game(s), game rules, System/LLM behavior, spells, gas/pricing policy, combat, AI, building/gameplay layers (blueprints, mechanisms, rooms, work orders, designation UX), and all game-specific content and presentation.

**Repository boundary:** the actual game is not part of this repository. Adjacent consumers have no privileged or game-specific implementation path into the substrate.

## Required product outcomes

- **Reusable substrate, not a game.** External games integrate through public Rust crate interfaces. Nothing above the matter surface reaches voxels except via substrate verbs and queries; that boundary is the sandbox and multiplayer-readiness seam as well as reuse.
- **Voxel truth that reads as a normal world.** Rolling terrain, forests, water, cliffs, and meadows present as a continuous natural surface; the mesh or other view is regenerated from matter and is never authoritative world state.
- **Mutable everywhere, deep Z first-class.** Any voxel can be destroyed, moved, or placed; underground geology (strata, caves, ore, aquifers, voids) is real content, not a painted floor under a heightmap.
- **Geology-first generation with sparse residency.** Worlds are produced as layered geology and related metadata so bricks can materialize lazily; homogeneous or untouched volume stays cheap so large regions remain tractable.
- **Matter-coherent living surface.** Interactable vegetation and clutter are voxel-backed objects in the world model; grass and similar dressing is derived from matter so dig, burn, and trample stay consistent with what the player sees.
- **Editable, streamable, persistable world.** Consumers can edit matter; the world streams around activity; truth is worldgen plus edit deltas (and related object/entity change journals where the substrate owns them), not a full raw voxel dump of untouched space.

Capability depth, algorithm choice, and delivery sequence for fluids, structural integrity, ambient weather/time, and related matter behaviors are design concerns; the product responsibility is a substrate that can host those matter-world behaviors without becoming a game layer.

## Future products and enabling implications

Described future consumers—not current product—include a System/LLM ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox modes. Product One’s walkable natural region is a consumer/validation scenario that motivates proving mutability, streaming, meshing, and geology—not a second product identity.

Enabling implications for those consumers (without importing their gameplay): continuous 3D matter they can dig and build on; deep-Z and column-friendly structure for level-style views; a verb/query surface games can price differently; and save/reload of scars so worlds and fortresses can be reused across modes. System/LLM hooks, if any, attach as ordinary clients and content authors above the substrate—never as substrate features.

## Non-goals

- Implementing the actual game, game rules, combat, AI, or entity gameplay beyond what the substrate must expose as world/matter services
- System/LLM runtime, spells, gas metering, or pricing policy
- Building/gameplay layers: blueprint economies, mechanisms-as-gameplay, room designation, agent labor, fortress UX
- Treating harness demo content, third-person fantasy presentation, or social-media milestone packaging as product requirements
- Making the substrate depend on an LLM

## Confirmed vision constraints

- Delivery form is a Rust crate or small family of tightly scoped Rust crates.
- The world substrate is GPU-resident in the sense required for a brick-pool / compute-backed matter world consumed by games.
- The substrate has zero LLM dependency and must function as a standalone engine layer.
- Any validation harness in this repository must use only the public interfaces an external game would use—no privileged substrate paths.
- Game rules and the System, LLM, spell, gas, combat, AI, and building layers are out of scope for implementation here; seams may be designed, those layers must not be built in this product.

## Deferred design decisions

- Voxel size, brick layout, meshing strategy, LOD, and object-layer capacity
- How far fluids, integrity, granular settle, fire, and ambient sim go in each delivery slice
- Crate split within the workspace, persistence encoding, and streaming-ring policy
- Whether and how multiplayer authority is realized on the verb/command surface
- Harness scenario content, controls, platforms, and benchmarks if a harness is delivered

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required current delivery** of this repository, or only a **permitted adjacent validation artifact**?

- **Proposed answer:** Permitted only—the product is the substrate; a harness may exist to validate public APIs but is not required for product completeness.
- **If different:** Requiring the harness keeps product identity on the substrate but adds a mandatory adjacent deliverable (still without pulling controller, character, demo route, or performance gates into substrate scope).

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, with a walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Binding boundary: substrate as Rust crate(s); game out of repo; harness only via public APIs; game/System/LLM/spell/gas/combat/AI/building layers out of scope.
- **docs/seeds/product-one-seed.md** — First walkable-region consumer/validation scenario (demo route, controller, seed content, perf gates); motivates substrate proof points without transferring harness ownership into the product.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look vs voxel truth, full mutability, deep Z, geology generation, matter-coherent dressing/objects, streaming/persistence, reusable layering) at design-detail altitude later planning will refine.
