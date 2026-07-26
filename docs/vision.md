# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation: matter, physics, queries, and mutation for natural-looking, fully mutable volumes with first-class depth—not a game, and not an LLM- or System-dependent runtime.

## Purpose

Moria exists so multiple games and tools can share one trustworthy material world: continuous terrain and underground space whose visual surface is a view over voxel truth, generated as geology, streamable and persistable, and open to dig, place, collapse, flood, and other matter operations through public interfaces. Game rules, presentation policy, and player fantasy live above the crate boundary; the substrate must stand alone without any System or LLM dependency.

## Product boundary

**In product**

- The reusable substrate crates and their public consumer-facing interfaces for world generation, matter representation, mutation, queries, view construction, streaming, and persistence.
- Compatibility seams the substrate itself requires so higher layers can attach later without privileged paths into the core.

**Out of product / adjacent**

- The actual game (ARPG, fortress/colony, descent roguelike, sandbox, or other) is a separate downstream consumer and is not part of this repository.
- A walkable-world executable may exist as an adjacent validation harness. It is not the product identity, not a game layer, and must not own privileged or game-specific implementation paths; if present, it consumes the substrate only through the same public interfaces available to an external game. Whether shipping that harness is a required current delivery is open (see Q1).
- Game rules and future System, LLM, spell, gas, combat, AI, and building *gameplay* layers are not implemented here. Controllers, cameras, authored demo routes, presentation polish, characters, and harness-specific acceptance scenes belong to consumers or the harness, not to substrate identity.

## Required product outcomes

Downstream design must make these true of the substrate:

1. **Reusable integration surface** — Consumers integrate Moria as Rust crate(s) with a non-optional public boundary: no privileged access for in-repo tools versus external games; matter changes and reads go through the product’s public verbs and queries.
2. **Voxel truth under a natural view** — The world is a fully material voxel volume. Surface presentation can read as ordinary terrain (hills, forest, water, rock), while the mesh or dressing remains a regenerated view, never the authority for physics, queries, or saves.
3. **Mutable everywhere, deep Z included** — Any voxel in the playable volume can be destroyed, moved, or placed; underground space is first-class content (caves, strata, depth), not a painted floor under a heightmap.
4. **Geology-first, lazy world substance** — Worlds are produced from seed-driven generation that yields coherent surface and subsurface materials (terrain, strata, voids, resources) and materializes cost only where touched, so large regions remain tractable.
5. **Matter-capable foundation** — Beyond static occupancy, the substrate owns world-layer matter behavior: interactable vegetation and clutter where they participate as matter; fluids and ambient matter effects; structural support and failure; granular settle where materials require it; and coupling so objects can leave static form as dynamic debris or rigid proxies. Delivery depth and sequence are design choices; the responsibility is product-level.
6. **Stream, remember, re-enter** — Active regions can stream in and out of residency; persistence is generation plus recorded change so edited worlds reload as the same material state consumers left.

## Future products and enabling implications

Described games—System/LLM-driven ARPG, Dwarf Fortress–style fortress/colony play, Moria-style deep descent, pure sandbox—are **future or external consumers**, not current Moria scope. They motivate why the substrate keeps gas/pricing, System authorship, combat, and building *policy* above the crate line.

Enabling implications only (not a committed roadmap or consumer feature list): public matter and query APIs that priced verbs and agents can share; metadata and placement hooks so higher layers can author materials, structures, and events without rewriting geology; continuous 3D navigation data derived from the volume so games can present levels or labor without redefining storage; seams for multiplayer-style command authority when a consumer needs them.

A first walkable demo or “product one” slice that limits fluids, weather, CA, felling, or UI does not shrink the substrate’s identity above; it only constrains an adjacent consumer’s first exercise of the crates.

## Non-goals

- Implementing the game, System/LLM runtime, spells, gas economy, combat, AI agents, or building gameplay systems in this product.
- Treating the validation harness’s character, camera, demo route, content set, or benchmark numbers as substrate scope.
- Making the substrate depend on an LLM or System to generate or run the world.
- Shipping privileged in-repo engine paths that external game consumers cannot use.

## Confirmed vision constraints

- **Identity** — Reusable voxel-world substrate; GPU-resident; Rust crate or small crate family.
- **Consumer boundary** — Non-optional public-interface separation between substrate and any harness or game; no game-specific private paths in the core.
- **Standalone** — Zero LLM/System dependency in the substrate.
- **Repository** — The actual game is not in this repository.
- **Seams vs implementation** — Compatibility seams where substrate requirements demand them; excluded game layers are not implemented here.

## Deferred design decisions

- Crate split, storage layouts, meshing and simulation algorithms, streaming ring policy, and persistence encodings.
- How far each matter capability (fluids, fire, integrity, objects, nav, stamps) goes in any given delivery slice, and in what order.
- Validation harness content, controls, platforms, and performance gates (if the harness is delivered).
- Exact generation parameters, material catalogs, and LOD or view strategies.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation executable a **required current delivery** of this effort, or only a **permitted adjacent artifact** that may be built to exercise the substrate?

- **Proposed answer:** Permitted adjacent artifact only—valuable for proof and regression, outside product identity, and if built must use solely public substrate interfaces.
- **If different:** Making it a required current delivery adds a mandatory adjacent deliverable (still not game scope) and binds the program to ship a harness; it does not expand substrate identity into controllers, content, or game systems.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident Rust voxel substrate and positions the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Binds product identity to the substrate crates, keeps the real game out of repo, permits a public-API-only harness, and excludes game/System/spell/gas/combat/AI/building layers while allowing required seams.
- **docs/seeds/voxel-world-substrate.md** — Supplies the substrate’s world-layer outcome families (natural view over voxel truth, full mutability, deep Z, geology-first generation, matter/physics foundation, streaming and persistence, reusable layering) and situates future games as consumers above a System-free engine layer.
- **docs/seeds/product-one-seed.md** — Describes a first walkable validation slice and demo non-goals; motivates early dig/place proof and public API discipline without transferring harness controls, content, platforms, or performance gates into substrate identity or narrowing long-horizon substrate responsibility to that slice alone.
