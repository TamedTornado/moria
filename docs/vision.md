# Project vision

## What we are building now

**Moria** is a reusable, **GPU-resident voxel-world substrate**, delivered as a **Rust crate** (or a small family of tightly scoped Rust crates). It is the material-world engine layer for natural-looking, fully mutable 3D volumes—not a game, not a playable demo product, and not a presentation stack for a particular title.

## Purpose

Downstream games need one trustworthy world foundation: surface and underground that read as continuous nature, remain diggable and placeable everywhere, and keep collision, queries, and mutation on material truth rather than decorative geometry. Moria exists so those games share geology-backed generation, sparse mutable matter, meshing-as-view, streaming, and edit-aware persistence through public interfaces, while game rules, policy, and presentation stay outside the crate. The substrate must **stand alone with zero LLM or System dependency**.

## Product boundary

**In product**

- The reusable substrate and its public integration surface: geological world generation, sparse GPU-resident matter, dig/place and related mutation and query verbs, smooth surface presentation as a **derived view** of voxel truth, matter-consistent surface dressing and voxel-backed world objects the substrate owns, static fluid bodies as matter, streaming-oriented residency, and persistence as generation identity plus edit deltas.
- External consumers integrate only through published crate interfaces.

**Adjacent (not product identity)**

- A **walkable-world executable** may exist as a **validation harness** for substrate capabilities (generation, streaming, meshing, editing, collision, persistence, performance). If present, it must use the **same public interfaces** available to an external game and must not own privileged or game-specific engine paths. Whether it is a current delivery obligation is **Q1**; until answered, it is not treated as required or as part of product identity.
- Controllers, cameras, characters, authored demo routes, curated seed-world content lists, presentation polish, harness workloads, and acceptance theater are **consumer-owned**.

**Out of this product and repository**

- The actual game; game rules; and the System, LLM, spell, gas, combat, AI, and **building** layers (gameplay, UX, blueprints-as-work-orders, mechanism entities, room/economy policy).
- Compatibility seams may be designed where substrate outcomes require them; those layers are not implemented here.

## Required product outcomes

1. **Natural world, voxel truth** — Consumers can present rolling terrain, water bodies, vegetation, and clutter that read as an ordinary landscape while authoritative occupancy and material remain voxel matter, not a heightmap with non-material props as the world model.
2. **Mutable everywhere; deep Z first-class** — Matter can be destroyed or placed throughout the volume; underground is continuous 3D content (strata, caves, ores, voids), not a skybox floor under a painted surface.
3. **Geology-capable generation on demand** — Worlds are produced as layered geology that supports honest digging, with large regions kept tractable by sparse residency and lazy materialization of untouched volume.
4. **Mesh is a view** — Rendering derives from matter and updates when matter changes; physics, collision, and queries use voxel truth—the mesh is never authoritative and is not the save format.
5. **Public mutation and query boundary** — Consumers dig, place, and inspect only through the published verb/query surface; nothing above the matter core touches voxels ad hoc, and any harness uses that same surface.
6. **Residency and scars** — Active neighborhoods stream; untouched volume stays cheap; edits persist as deltas over generation so reloads restore material change without treating the whole world as authored assets.

## Future products and enabling implications

Described **future consumers** (separate products, not this repository’s identity): a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent adventure, and pure sandboxes. They own gameplay, UX, content, controllers, characters, and presentation.

**Enabling implications** (not a committed feature roadmap): multi-game reuse of one matter stack; richer ambient and structural matter behavior (flowing fluids, fire, granular settle, structural integrity); vegetation objects that can later couple to rigid motion; and policy-pluggable verb pricing so different games share verbs without sharing rules. Long-horizon build, room, mechanism, and agent-labor fantasies remain consumer or later-layer work; excluded building and System layers stay out unless a future approved vision expands the boundary.

## Non-goals

- Shipping the actual game, combat, stats, AI, spells, gas economy, weather/seasons as product features, or System/LLM integration inside Moria.
- Owning consumer controllers, cameras, characters, demo narratives, seed-content inventories, or harness acceptance scenarios as product identity.
- Treating validation-harness content, presentation, workloads, platforms, or performance numbers as substrate requirements.
- Implementing building-game layers or making the substrate depend on an LLM.

## Confirmed vision constraints

- **Rust crate ecosystem:** delivery is a Rust library surface (crate or small crate family) for game integration.
- **GPU-resident** world/matter foundation as part of the product promise.
- **Consumer isolation:** adjacent harnesses and external games use only public substrate interfaces; no privileged in-repo game paths.
- **Standalone substrate:** zero LLM/System dependency; those attach only as external clients later.
- **Excluded layers** under Product boundary are not implemented here (seams only where required).

## Deferred design decisions

- Exact crate split, API shape, data layouts, algorithms, and how the public boundary is enforced in the workspace.
- Voxel resolution, meshing strategy details, LOD, object-layer capacity, and related measurement tradeoffs.
- How much richer matter simulation (multi-tier fluids, CA, integrity, granular settle, object felling) lands in which engineering slice.
- Persistence encoding, streaming policy, quantitative performance budgets, and the environments used to collect evidence.
- Depth of later-enabling seams once identity and boundary are fixed.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation harness a **required current delivery** of this repository, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** **Permitted, not mandatory.** Product identity and completeness remain the substrate crate(s). A harness may exist to exercise public APIs; its controller, content, presentation, route, platform, and acceptance gates stay outside substrate scope.
- **If different:** Making the harness **mandatory** keeps substrate identity but adds a required adjacent delivery (still without importing its character, world tour, or numbers into product scope). Treating the **walkable demo itself as the product** would replace crate-substrate identity with a playable vertical slice.

## Seed synthesis

- **`README.md`** — Names Moria as the GPU-resident voxel-world substrate consumed as a Rust crate and frames the walkable-world executable as a separate validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`** — Binding product and repository boundary: reusable Rust crate(s) in; actual game out; harness may exist only via public APIs; game/System/spell/gas/combat/AI/building layers out of scope (seams only).
- **`docs/seeds/product-one-seed.md`** — First-slice and demo motivation (material walkable world, dig proof, deep-Z traversal, gen/mesh/stream/persist pressure). Used as harness-shaped motivation; controllers, seed content, milestones, and machine gates are not imported as substrate identity or requirements (see Q1).
- **`docs/seeds/voxel-world-substrate.md`** — Substrate design goals and multi-game purpose (natural look, mutability, deep Z, matter/physics/query foundation, standalone of LLM). Mechanisms and inventories deferred; high-level outcomes and enabling implications retained.
