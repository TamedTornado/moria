# Project vision

## What we are building now

**Moria** is a reusable, **GPU-resident voxel-world substrate** delivered as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer product: continuous material world, generation, mutation, query, presentation-of-matter, and persistence seams for downstream games—not a game, demo title, or content pack.

## Purpose

Give future games a shared foundation on which a natural-looking outdoor and underground world is fully material and mutable—so digs, builds, geology, and deep vertical play rest on the same truth as rendering and collision. The substrate must stand alone with zero dependency on LLM/System features, spell economies, or any single game’s rules.

## Product boundary

**This product owns** the reusable world/matter substrate and its public consumer surface: generation of material worlds, sparse residency and streaming readiness, smooth presentation derived from voxel truth, matter mutation and query APIs, collision-relevant occupancy against that truth, and edit-aware persistence. Integration is library-shaped for Rust consumers.

**Adjacent, not identity:** a walkable-world executable may live in-repo only as a **validation harness**. It must exercise the substrate through the **same public interfaces** an external game would use—no privileged or game-specific implementation paths. Harness controllers, characters, cameras, authored demo routes, presentation chrome, scripted workloads, and performance gates are **not** the product; they validate it.

**Downstream / other products own:** the actual game(s); game rules; System/LLM behavior; spells and gas policy; combat; AI/agents; building gameplay (blueprints, work orders, mechanisms-as-gameplay, room economy); UX and content authorship beyond substrate registries and seams.

Compatibility seams may be designed where substrate outcomes require them; those upper layers are not implemented here.

## Required product outcomes

1. **Material world, normal look** — Consumers can present rolling terrain, water bodies, cliffs, caves, and dressed surface cover such that the world reads as a natural place while remaining fully backed by mutable voxel matter (mesh and dressing are views of truth, not separate decorative geometry).
2. **Mutable everywhere** — Through public verbs/queries, matter can be destroyed, placed, and inspected anywhere in the volume; presentation and collision stay consistent with edits without consumers touching storage directly.
3. **Deep Z is first-class** — Underground space (strata, caves, buried material bands) is real playable volume and content substrate, not a shallow floor under a heightmap skin.
4. **Geology-first generation** — Worlds are produced so digging reveals coherent material structure (layers, voids, deposits), supporting honesty of descent and large-scale sparsity rather than paint-under-heightfield fakery.
5. **Scale-ready residency** — Large regions remain tractable via sparse representation, lazy materialization, streaming-oriented residency, and persistence as generation identity plus edit deltas—not full eager voxel dumps.
6. **Reusable library boundary** — Rust consumers (external games and any in-repo harness) share one public capability surface; gas/pricing, game modes, and policy plug in above the substrate rather than forking it.

## Future products and enabling implications

Described future **consumers** (not this product): a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox modes. They motivate substrate generality; they do not import their gameplay, content, UI, characters, or policies into current scope.

**Enabling implications** (high-level, not a committed roadmap): matter and query surfaces rich enough for later fluid, fire, integrity, granular, vegetation-object, and placement/stamp use; metadata/POI and registry hooks so an external System or tools can author materials and placements without generating geology; navigability and structural honesty as properties of matter usable by future agents and building games. Building *layers* and game systems stay consumer-owned even when the substrate enables them.

## Non-goals

- Shipping a playable game, campaign, or fortress/ARPG mode in this repository
- Implementing System/LLM, spells, gas economies, combat, or AI
- Implementing building-game layers (blueprints-as-gameplay, labor, mechanisms policy, room assignment)
- Treating the validation harness’s character, controls, demo route, art direction, or benchmark scene as product requirements
- Full ambient weather/season simulation, multiplayer service, or content pipeline as product identity

## Confirmed vision constraints

- **Form factor:** Rust library crate(s) for game and tool consumers; not a standalone game product identity
- **Consumer isolation:** no privileged in-repo path around the public substrate surface
- **GPU-resident world/matter** as the intended runtime posture for the substrate
- **Portable GPU stack intent:** load-bearing GPU work stays on a portable path (wgpu/WGSL-class), not a native Metal-only fork, so discrete and Apple Silicon-class targets remain in scope
- **Zero LLM/System dependency** inside the substrate
- **Upper game layers out of repo scope** (rules, System, spells, gas, combat, AI, building layers), with seams only where outcomes require them

## Deferred design decisions

- Vertical-slice depth and sequencing of matter behaviors (e.g. advanced fluids, fire, integrity, granular settle, vegetation felling) versus a minimal mutability-and-presentation core
- Voxel resolution, LOD/impostor strategy, and object-layer scaling tradeoffs
- Exact public API shape, crate family split, and persistence/streaming ring policy
- How thoroughly an in-repo harness must demonstrate outcomes (and on which platforms) once harness delivery is settled
- Multiplayer readiness beyond keeping command/query boundaries unhostile to later authority models

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world **validation harness executable** a **required deliverable** of this repository’s current product effort, or only **permitted** as an optional adjacent consumer?

- **Proposed safe answer:** **Permitted and encouraged**, not mandatory for substrate identity—substrate crates and public interfaces are the product; a harness is the preferred way to prove them when built.
- **If different:** Making the harness **mandatory** adds a required adjacent delivery (still not product identity) so planning must schedule a walkable consumer; keeping it **optional** allows substrate-only milestones without a playable executable.

## Seed synthesis

- **README.md** — Names Moria as a GPU-resident voxel-world Rust substrate and frames the walkable executable as harness, not game layer.
- **docs/seeds/project-boundary.md** — Binding boundary: substrate-only product, public-interface harness rule, and exclusion of game/System/spell/gas/combat/AI/building layers.
- **docs/seeds/product-one-seed.md** — Motivates first proof points (natural mutable world, dig honesty, geology, sparsity/persistence) and a walkable demo shape; consumer-owned demo detail is not transferred into product identity.
- **docs/seeds/voxel-world-substrate.md** — Long-horizon substrate responsibilities (material truth, deep Z, geology-first gen, matter/query layering, future-game enablement) fused into outcomes and implications without mechanism inventory or game-layer scope creep.
