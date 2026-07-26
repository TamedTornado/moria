# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is the material world engine that downstream games consume: generated geology, mutable voxel matter, smooth visual presentation of that matter, queries, and mutation through public interfaces. It is not a game, not a demo, and not an LLM-backed system.

## Purpose

Moria exists so multiple games can share one honest material world: a natural-looking surface and deep underground whose appearance is a view of voxel truth, not decorative geometry. The substrate stands alone with no LLM dependency. Game rules, economy, spells, combat, agents, and authored gameplay live in consumers above it. The same foundation should support adventure, fortress/colony, descent, and sandbox-style games without rewriting the world layer.

## Product boundary

**In product:** the reusable substrate and its public consumer-facing interfaces (generation of dig-honest geology, GPU-resident matter representation, meshing/presentation of voxel truth, material mutation and query verbs, and the world/persistence/streaming foundations those require).

**Adjacent, not identity:** a walkable-world executable may exist in this repository as a validation harness. If present, it is a separate consumer of the substrate and must use the same public interfaces available to an external game—no privileged or game-specific substrate paths. Whether shipping that harness is part of current delivery is open (see Q1).

**Out of repository / out of product:** the actual game; game rules; System/LLM features; spells; gas policy; combat; AI; and building-game layers (blueprints, work orders, mechanism gameplay, room economies). Compatibility seams may be designed only where substrate requirements demand them; those layers are not implemented here.

**Consumer-owned (any harness or game):** character controllers, cameras, HUD, authored demo routes and seed-world content inventories, trailer presentation, and consumer-chosen performance gates or target machines.

## Required product outcomes

1. **Reusable world substrate.** Consumers integrate Moria as Rust crates and drive the world only through public verbs and queries; nothing above the matter layer owns direct voxel storage or privileged mutation paths.
2. **Voxel truth, normal look.** Terrain and structures read as a continuous natural world (smooth isosurface presentation with sharp cut/built features where the matter demands), while physics, collision, and gameplay-relevant queries run against voxel occupancy and materials—not the render mesh. The mesh is a regenerated view, not authoritative truth.
3. **Mutable material everywhere.** Any voxel can be destroyed, eroded, or placed; dig and place are first-class substrate capabilities so cuts and fills remain material world edits with honest remeshed faces.
4. **Deep Z as content.** Underground volume is first-class: continuous 3D play space supporting caves, strata, ore, and dig-down discovery—not a painted floor under a heightmap.
5. **Geology-first generation.** Worlds are produced as layered geology (columns, strata, caves, resources, lazy materialization from seed and parameters) so digging reveals true material structure rather than a heightmap shell.
6. **GPU-resident matter foundation.** The live world representation is GPU-resident and sparsity-aware so large regions can idle cheaply and activate under consumer-driven use, with edit-delta-style persistence of touched world state as the substrate-level save model.

Supporting matter capabilities that the substrate is responsible for at product altitude—surface dressing driven by voxel data, voxel-backed interactable objects (e.g. vegetation and rocks), static fluid bodies, and the public mutation/query boundary—belong to Moria even when a first consumer exercises only a slice. Depth and sequence of simulation richness (flows, fire, integrity, granular settle, weather) are design delivery choices, not a narrowing of product identity.

## Future products and enabling implications

Downstream consumers (not this product) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style descent experience, pure sandbox tools, and any walkable validation harness. Enabling implications only: honest mutability, deep-Z geology, matter queries/events, and a clean verb boundary so those games can attach rules, agents, pricing, and content without forking the world engine. Their gameplay, controllers, characters, UI, and authored content are not Moria scope.

## Non-goals

- Shipping a playable game or game systems in this product
- Implementing System/LLM, spells, gas, combat, AI, or building-game layers here
- Treating harness controls, demo seed content, cameras, or consumer benchmarks as substrate identity
- LLM dependency inside the substrate
- Heightmap-with-props worlds that cannot be dug as true matter

## Confirmed vision constraints

- **Ecosystem:** Rust crate (or small crate family) consumption boundary.
- **Consumer isolation:** adjacent consumers, including any validation harness, use only public interfaces; no privileged game paths in the substrate.
- **Standalone substrate:** zero LLM dependency; game policy injects above, not inside, the world layer.
- **GPU-resident world:** product promise is a GPU-resident voxel-world substrate, not a CPU-only prototype architecture.
- **Explicit exclusions:** game rules and future System, spell, gas, combat, AI, and building layers stay out of this product (seams only where substrate needs demand).

## Deferred design decisions

- Exact crate graph, APIs, algorithms, brick/payload layouts, and meshing method choices
- Voxel scale, LOD, streaming ring policy, and simulation-tier depth/order
- How much matter simulation ships in which release versus later substrate increments
- Harness/demo content, controls, platforms, and numeric acceptance thresholds (consumer/design concerns)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable **mandatory current delivery** alongside the substrate crates, or only a **permitted adjacent artifact**?

- **Proposed safe answer:** Permitted adjacent artifact only—the current product commitment is the reusable substrate and its public interfaces; a harness may live in-repo for validation but is not required to define or complete the product.
- **If answered differently:** Making the harness mandatory keeps substrate identity unchanged but expands current delivery to include a separate walkable consumer; treating harness-owned content, controls, or machine targets as substrate requirements would incorrectly widen product identity.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident voxel-world substrate consumed as a Rust crate, with the walkable-world executable called out as a separate consumer/harness.
- **`docs/seeds/project-boundary.md`:** Fixes product identity on the substrate crates, places the real game outside the repo, permits a public-API-only validation harness, and excludes game/System/building layers while allowing necessary compatibility seams.
- **`docs/seeds/product-one-seed.md`:** Motivates first-consumer proof of a diggable, walkable material world and a partial substrate exercise; its controller, seed content, milestones, machines, and numeric gates stay consumer/harness concerns, not product redefinition.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies the substrate’s outcome families—normal look over voxel truth, full mutability, deep Z, geology-first generation, GPU-resident matter, and reusable layering without LLM dependency—while leaving mechanisms and build order to design.
