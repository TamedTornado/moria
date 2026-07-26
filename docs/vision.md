# Project vision

## What we are building now

**Moria** is a reusable, **GPU-resident voxel-world substrate**, delivered as a **Rust crate** (or a small family of tightly scoped Rust crates). It is the material-world engine layer for natural-looking, fully mutable 3D volumes—not a game, not a playable demo product, and not a presentation stack for a particular title.

## Purpose

Downstream games need one trustworthy world foundation: surface and underground that read as continuous nature, remain diggable and placeable everywhere, and keep collision, queries, mutation, and dynamic matter behavior on material truth rather than decorative geometry. Moria exists so those games share geology-backed generation, sparse mutable matter, meshing-as-view, streaming, edit-aware persistence, and a matter/physics/query foundation through public interfaces, while game rules, policy, and presentation stay outside the crate. The substrate must **stand alone with zero LLM or System dependency**.

## Product boundary

**In product**

- The reusable substrate and its public integration surface: geological world generation; sparse GPU-resident matter; dig/place and related mutation and query verbs; smooth surface presentation as a **derived view** of voxel truth; matter-consistent surface dressing and voxel-backed world objects the substrate owns; static and dynamic fluid, fire, granular, and structural-matter behavior the substrate owns; streaming-oriented residency; and persistence as generation identity plus edit deltas.
- External consumers integrate only through published crate interfaces.

**Adjacent (not product identity)**

- A **walkable-world executable** may exist as a **validation harness** for substrate capabilities (generation, streaming, meshing, editing, collision, persistence, performance). If present, it must use the **same public interfaces** available to an external game and must not own privileged or game-specific engine paths. Whether it is a current delivery obligation is **Q1**; until answered, it is not treated as required or as part of product identity.
- Controllers, cameras, characters, authored demo routes, curated seed-world content lists, presentation polish, harness workloads, and acceptance theater are **consumer-owned**.

**Out of this product and repository**

- The actual game; game rules; and the System, LLM, spell, gas, combat, AI, and **building** layers (gameplay, UX, blueprints-as-work-orders, mechanism entities, room/economy policy).
- Compatibility seams may be designed where substrate outcomes require them; those layers are not implemented here.

## Required product outcomes

1. **Natural world, voxel truth; mesh is a view** — Rolling terrain, water, vegetation, and clutter read as ordinary landscape while occupancy and material remain voxel matter, not a heightmap with non-material props. Rendering derives from matter and updates when it changes; physics, collision, and queries use voxel truth—the mesh is never authoritative and is not the save format.
2. **Mutable everywhere; deep Z first-class** — Matter can be destroyed or placed throughout the volume; underground is continuous 3D content (strata, caves, ores, voids), not a skybox floor under a painted surface.
3. **Geology-capable generation on demand** — Worlds are produced as layered geology that supports honest digging, with large regions kept tractable by sparse residency and lazy materialization of untouched volume.
4. **Public mutation and query boundary** — Consumers dig, place, and inspect only through the published verb/query surface; nothing above the matter core touches voxels ad hoc, and any harness uses that same surface.
5. **Dynamic matter, physics, and ambient behavior** — The substrate provides the shared matter/physics foundation: fluids (still bodies through active flow and fine splash), fire and material-state propagation, granular settle, structural integrity and collapse, and voxel-backed objects that couple to rigid motion when detached. Thin ambient drive (time of day, seasonal and weather influence on light, wetness, growth ticks, and fire ecology) keeps the surface world behaving as living landscape at the matter/aggregate layer—without each game reimplementing a private world sim.
6. **Residency and scars** — Active neighborhoods stream; untouched volume stays cheap; edits persist as deltas over generation so reloads restore material change without treating the whole world as authored assets.

## Future products and enabling implications

Described **future consumers** (separate products, not this repository’s identity): a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent adventure, and pure sandboxes. They own gameplay, UX, content, controllers, characters, and presentation.

**Enabling implications** (not a committed roadmap): multi-game reuse of one matter stack; policy-pluggable verb pricing so games share verbs without sharing rules; and semantic/game layers (rooms, work orders, agent labor, System-directed placement and palettes) as external clients. Multiplayer readiness of the command boundary is readiness only. Excluded building and System layers stay out unless a future approved vision expands the boundary.

## Non-goals

- Shipping the actual game, combat, stats, AI, spells, gas economy, or System/LLM integration inside Moria.
- Owning consumer controllers, cameras, characters, demo narratives, seed-content inventories, or harness acceptance scenarios as product identity.
- Treating validation-harness content, presentation, workloads, platforms, or performance numbers as substrate requirements.
- Implementing building-game layers (blueprints-as-work-orders, mechanism-entity gameplay, room/economy policy) or making the substrate depend on an LLM.
- Treating a consumer’s first vertical slice as the ceiling of substrate identity or outcome families.

## Confirmed vision constraints

- **Rust crate ecosystem:** delivery is a Rust library surface (crate or small crate family) for game integration.
- **GPU-resident** world/matter foundation as part of the product promise.
- **Consumer isolation:** adjacent harnesses and external games use only public substrate interfaces; no privileged in-repo game paths.
- **Standalone substrate:** zero LLM/System dependency; those attach only as external clients later.
- **Excluded layers** under Product boundary are not implemented here (seams only where required).

## Deferred design decisions

- Exact crate split, API shape, data layouts, algorithms, and workspace enforcement of the public boundary.
- Voxel resolution, meshing strategy details, LOD, object-layer capacity, and measurement tradeoffs.
- Delivery depth and engineering sequence for matter/physics outcomes (fluid tiers, CA rules, integrity, granular settle, object–rigid coupling)—not whether those outcome families belong to the product.
- Persistence encoding, streaming policy, quantitative performance budgets, and evidence environments.
- Depth of later-enabling seams (pricing plugs, multiplayer) once identity and boundary are fixed.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation harness a **required current delivery** of this repository, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** **Permitted, not mandatory.** Product identity and completeness remain the substrate crate(s). A harness may exist to exercise public APIs; its controller, content, presentation, route, platform, and acceptance gates stay outside substrate scope.
- **If different:** Making the harness **mandatory** keeps substrate identity but adds a required adjacent delivery (still without importing its character, world tour, or numbers into product scope). Treating the **walkable demo itself as the product** would replace crate-substrate identity with a playable vertical slice.

## Seed synthesis

- **`README.md`** — Names Moria as the GPU-resident voxel-world substrate consumed as a Rust crate and frames the walkable-world executable as a separate validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`** — Binding product and repository boundary: reusable Rust crate(s) in; actual game out; harness may exist only via public APIs; game/System/spell/gas/combat/AI/building layers out of scope (seams only).
- **`docs/seeds/product-one-seed.md`** — First-slice and demo motivation (material walkable world, dig proof, deep-Z traversal, gen/mesh/stream/persist pressure). First-slice depth and exclusions do not narrow enduring substrate outcome families; controllers, seed content, milestones, and machine gates are not substrate identity (see Q1).
- **`docs/seeds/voxel-world-substrate.md`** — Substrate design goals and multi-game purpose (natural look, mutability, deep Z, matter/physics/query foundation including dynamic matter and ambient behavior, standalone of LLM). Mechanisms deferred; binding high-level outcomes retained.
