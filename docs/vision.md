# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer library for natural, fully material voxel worlds—not a game and not a gameplay shell.

## Purpose

Games need a shared foundation where the surface looks like ordinary terrain, the underground is real content, and every visible solid is mutable matter—not decorative geometry. Moria exists so multiple games (sandbox, fortress, descent, ARPG) can share one matter-and-world stack: generation, material truth, mutation, queries, and related physics foundations, with game rules living entirely above the substrate.

## Product boundary

**This product owns:** the reusable substrate crates and the public interfaces external games use for world generation, matter storage and residency, meshing-as-view, mutation and query verbs, material/object foundations that keep the world consistent, and persistence/streaming of world truth.

**Adjacent, not identity:** a walkable-world executable *may* exist as a validation harness. It is a separate consumer. Whether shipping it is part of current delivery is open (see Q1). If present, it must use the same public interfaces as an external game and must not own privileged or game-specific substrate paths.

**Downstream, not this repository:** the actual game and any game rules; System/LLM features; spell, gas, combat, and AI layers; building-game policy and UX (blueprints as gameplay, work orders, mechanisms as game entities). Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

**Harness-owned when a harness exists:** character control, camera, authored demo route, presentation polish, debug tools, scripted benchmark scenes, and machine-specific performance gates.

## Required product outcomes

- **Reusable integration surface.** Downstream games consume Moria through public Rust-crate interfaces; nothing above the matter boundary touches voxels directly—mutation and observation go through verbs, queries, and events.
- **Natural-looking material world.** Generated terrain reads as continuous natural ground (hills, strata, water, living cover) while remaining fully material: dig, place, and related matter operations work anywhere the world has content, including deep underground.
- **Continuous 3D and deep Z.** The underground is first-class content space (geology, caves, voids, subsurface materials), not a flat floor under a heightmap shell.
- **Voxel truth, mesh view.** Collision, queries, and mutation run against material occupancy; extracted meshes are regenerated views, never the authority of the world.
- **Tractable large worlds.** Regions stay practical through sparse residency, lazy materialization from generation, and streaming-oriented active sets so idle volume does not force full voxel residency.
- **Consistent matter presentation.** Interactable solid features and surface dressing stay driven by, and consistent with, the material world so what players see remains what the substrate can mutate.
- **Seed-plus-delta persistence.** Unedited world can be recovered from generation parameters; player and sim scars persist as deltas and reload correctly.
- **Standalone substrate.** The stack has zero LLM dependency; higher systems attach only as external consumers of the same interfaces.

## Future products and enabling implications

Future **consumers** (not current product) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony mode, and a Moria-style descent experience. They motivate a substrate that stays game-policy-free while exposing matter, queries, mutation, and enough physics/world foundations that those modes need not reimplement the world. Gameplay, content, controllers, presentation, economy, and AI remain consumer-owned. Long-horizon matter capabilities (richer fluids, structural failure, ambient ecology, fuller object dynamics) are enabling implications of that reusable foundation; delivery depth and sequence are design concerns, not a committed multi-product roadmap in this brief.

## Non-goals

- Implementing a shippable game, ARPG, fortress sim, or descent roguelike in this repository
- System/LLM runtime, spells, gas policy, combat, agent AI, or building-game layers
- Treating the walkable-world harness’s character, route, UI, content palette, or acceptance numbers as substrate scope
- Making decorative-only terrain or a pure heightmap world the product truth model

## Confirmed vision constraints

- Delivered as a **Rust crate** or small family of tightly scoped Rust crates for Rust-game consumers
- **GPU-resident** world substrate
- Any in-repo harness or external game uses **only public interfaces**—no privileged substrate paths
- **Zero LLM dependency** inside the substrate
- **Out of product scope:** game rules; System/LLM; spell, gas, combat, AI; building layers (seams only where required)

## Deferred design decisions

- Precise crate split, APIs, data layouts, algorithms, and meshing approach
- Voxel scale, LOD strategy, object-layer capacity, and fluid/physics subsystem depth per release
- Streaming ring policy, persistence encoding, and synchronization patterns
- Whether and how a validation harness is structured, and all harness content, controls, platforms, and performance gates
- Delivery sequence and milestone depth for substrate capabilities

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world validation harness **required current delivery**, or only **permitted** as an adjacent artifact?

- **Proposed safe answer:** Permitted only. Current product identity and required delivery are the substrate crates and public interfaces; a harness may exist later or alongside but is not assumed mandatory until design commits to it.
- **If answered “required”:** Delivery must include a harness executable that validates the substrate through public APIs, still outside product identity—without importing its controller, content, presentation, or performance gates into substrate scope.

## Seed synthesis

- **`README.md`** — Names Moria as the GPU-resident Rust-crate substrate and separates the walkable-world executable as consumer/harness, not game layer.
- **`docs/seeds/project-boundary.md`** — Fixes product identity (reusable substrate crates), consumer vs harness boundary, public-interface-only access, and exclusion of game/System/building layers.
- **`docs/seeds/product-one-seed.md`** — Motivates a first product-shaped proof of a walkable material world and which substrate outcome families that proof depends on; demo player, content route, and platform gates stay harness/consumer-side.
- **`docs/seeds/voxel-world-substrate.md`** — Authorizes substrate outcome families (natural material world, deep Z, mutability, generation, matter foundations, persistence/streaming, layering, standalone engine role) without importing mechanism inventory or consumer games into current identity.
