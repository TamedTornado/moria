# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer world foundation for external games—not a game, demo product, or gameplay stack.

## Purpose

Moria exists so multiple future games can share one continuous, fully material voxel world: natural-looking surface terrain, deep underground play, and Minecraft-grade dig and place anywhere, without each title reimplementing geology, matter, mutation, and world residency. The substrate must stand alone with **zero LLM dependency**; higher systems (including any LLM “System”) are clients, not core features.

## Product boundary

**In product**

- Reusable world substrate: generation, matter truth, mesh-as-view, mutation and query surface, residency/streaming and persistence foundations needed to run a large mutable world.
- Public Rust integration surface that external games and any in-repo validation executable share without privileged paths.

**Adjacent, not product identity**

- A walkable-world executable may exist in this repository as a **validation harness** for the substrate. Whether that harness is a required repository delivery is unresolved (see Q1). While open, treat it only as a permitted adjacent artifact. Even if delivery is required later, harness controller, character, camera, authored route, presentation, workload, platform, and performance gates stay harness-owned—not substrate outcomes.
- Downstream games (ARPG with System, fortress/colony, descent, sandbox) own rules, UX, content, controllers, presentation, and policy.

**Out of this repository’s product work**

- The actual game(s).
- Game rules and the System, LLM, spell, gas, combat, AI, and building **game** layers. Compatibility seams may be designed where substrate needs demand them; those layers are not implemented here.

## Required product outcomes

1. **Material world, natural look** — Consumers get a continuous 3D voxel world that reads as ordinary terrain (hills, forests, water, cliffs, underground) while remaining fully mutable matter; the rendered surface is a regenerated view of voxel truth, not an authoritative heightmap-with-props world.
2. **Deep Z as first-class space** — Underground volume is real playable content (geology, voids, buried structure hooks), not a thin floor under a skybox.
3. **Geology-first generation at scale** — Worlds are produced as geology and related volume (columns/strata/caves/materials with hardness and similar properties), materializing interest on demand so large regions stay tractable without loading all raw voxels.
4. **Mediated mutation and query** — Dig, place, and related matter access go through a public verb/query surface. Nothing above the matter boundary writes voxels by private path; any harness or external game uses the same surface.
5. **GPU-resident interactive matter** — Live matter and remesh-after-mutation stay on a GPU-resident path so carving and rebuild remain interactive at world scale.
6. **Durable, streamable world state** — Truth is generation inputs plus edit deltas; active regions stream around anchors without requiring the full volume resident as raw voxels.

## Future products and enabling implications

Future **consumers** (not current product): System-driven ARPG, Dwarf Fortress–style fortress/colony play, Moria-style descent, pure sandbox. They motivate substrate responsibilities for matter physics and dressing—voxel-backed interactable objects (e.g. vegetation that is real matter), multi-tier fluid support beyond static bodies, structural support and collapse, ambient matter rules, and semantic hooks (nav, rooms, blueprints) as **enabling seams**—without importing those games’ gameplay, content, controllers, or policies into Moria now. Delivery depth and order of those matter capabilities are design concerns, not a separate product identity.

## Non-goals

- Shipping a commercial or narrative game in this repository.
- Implementing System/LLM, spells, gas pricing, combat, AI agents, or building-game layers here.
- Treating harness- or demo-specific content (seed postcard region, third-person hero, debug keybinds, trailer routes, benchmark theater) as substrate scope.
- Making decorative-only geometry, heightmap-only terrain, or mesh-authoritative physics the product model.

## Confirmed vision constraints

- **Ecosystem**: product is a Rust crate or small family of tightly scoped Rust crates for game integration.
- **Residency**: substrate is GPU-resident for the live world/matter path.
- **Independence**: substrate has zero LLM dependency and must function as a standalone engine layer.
- **Consumer parity**: any in-repo validation executable must use the same public interfaces as an external game; no privileged game-specific implementation paths inside the substrate boundary.
- **Repository exclusion**: game rules and System/LLM/spell/gas/combat/AI/building layers are not implemented in this product; seams only where substrate requirements demand them.

## Deferred design decisions

- Precise crate split and internal module layout (consumer boundary is fixed; packaging is not).
- Voxel resolution, brick/layout encodings, meshing strategy details, LOD, and object-layer capacity.
- How deep each matter capability (fluids beyond static bodies, CA/fire, integrity, granular settle, weather) lands in early deliveries versus later substrate growth.
- Graphics/API stack choices and performance envelopes for particular machines.
- Harness scenario design if a walkable-world executable is delivered.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required repository delivery**, or only **permitted** as an adjacent harness?

- **Proposed safe answer:** Permitted only—aligned with the explicit project boundary that Moria *may* include such an executable. Product identity remains the substrate; no harness is mandated for the vision to hold.
- **If answered differently:** “Required delivery” adds a repository obligation to ship a harness that exercises the public substrate boundary, still outside product identity and still without pulling controller, content, presentation, platform, or performance gates into substrate outcomes.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and frames the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Fixes current product and repository boundary: Rust substrate crates; game out of repo; optional harness via public APIs only; excludes System/LLM/spell/gas/combat/AI/building layers while allowing seams.
- **docs/seeds/product-one-seed.md** — Adjacent first-slice consumer/demo vision (walkable region, character, dig proof, targets, milestones); motivates that the substrate must support a material walkable world and dig/place proof, without transferring demo content, controls, or machine gates into substrate scope.
- **docs/seeds/voxel-world-substrate.md** — Long-horizon substrate purpose and outcome families (natural look vs voxel truth, deep Z, geology generation, matter/mutation, dressing and fluids/integrity as matter responsibilities, persistence/streaming, layering with games above); stands alone with no LLM core dependency.
