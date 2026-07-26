# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. Downstream games and validation tools consume it; they are not the product.

## Purpose

Moria exists so adventure, fortress, sandbox, and related games can share one foundation: a natural-looking, fully material 3D world whose voxel truth supports dig, place, deep underground play, and matter-faithful simulation—without embedding any particular game’s rules, economy, or presentation. The substrate must stand alone with **no LLM or System dependency**.

## Product boundary

**In product:** the world substrate—generation of natural geology and surface, sparse GPU-resident matter, smooth mesh views of voxel truth, mutation and query through a public API, matter-consistent vegetation and dressing, fluid and integrity simulation as world systems, ambient world behavior, persistence and streaming of large regions, and integration seams games need without owning game policy.

**Adjacent, not identity:** a walkable-world executable may exist in this repository as a **validation harness**. If present, it must use the same public substrate interfaces as an external game and must not own privileged or game-specific paths. Whether that harness is a required repository delivery is open (see Q1). Its character, camera, route, content set, debug controls, platform, and performance gates are harness-owned, not product identity.

**Out of repository product:** the actual game(s); System/LLM; spells; gas pricing; combat; AI; and game building layers (building UI, blueprints-as-gameplay, mechanisms-as-game systems, work orders, room economy). Compatibility seams for those consumers may be designed where substrate outcomes require them; those layers are not implemented here.

## Required product outcomes

1. **Reusable engine layer.** Games receive matter, world queries, mutation, and related world physics through a public crate API. Nothing above the matter boundary touches voxels directly. Gas, labor, or other costs are consumer policy, not hard-wired into the substrate.

2. **Natural world, voxel truth.** Surface worlds read as ordinary terrain (hills, forest, water, cliffs, meadows)—not a cube aesthetic—while the voxel grid remains the authority for occupancy, materials, and edits. Rendered meshes are regenerated views, never saved as truth.

3. **Mutable everywhere, deep Z first-class.** Any volume can be destroyed, eroded, or filled. Continuous 3D underground (caves, strata, ore, depth) is real content, not a painted floor under a heightmap. Consumers can prove mutability by dig/place against that same truth (including collision against occupancy, not against a decorative mesh).

4. **Geology-first generation at sparse cost.** Worlds are generated as geology so dig-down stays honest; materialization is lazy and empty or uniform regions stay cheap so large regions are feasible.

5. **Matter-consistent surface and shared world systems.** Interactable vegetation and clutter stay coherent with matter under edit and reaction. The substrate owns fluids, support and collapse, granular behavior, and material reaction as world systems games share and tune—not reimplement as private engines—plus light ambient behavior that keeps a natural surface legible.

6. **Persistence and streaming.** Truth is worldgen function plus edit deltas; regions stream around active anchors so large worlds stay workable without keeping the whole volume always resident as raw voxels.

## Future products and enabling implications

Future **consumers** (not this product): a System-driven ARPG, a DF-style fortress/colony game, a Moria-style descent, pure sandbox modes, and any walkable-world demo or harness. They motivate reusable geology, mutability, deep Z, integrity, fluids, and verb/query symmetry. Their gameplay, content, presentation, controllers, characters, and acceptance scenarios stay consumer-owned. Long-horizon multiplayer readiness of a command-style API is an enabling implication only, not a committed delivery horizon here.

## Non-goals

- Shipping a game, combat loop, stats, AI, or System/LLM features in this product
- Implementing spell, gas, building-as-gameplay, or economy layers here
- Treating demo content, third-person control, or trailer routes as substrate scope
- Decorative terrain that is not backed by mutable matter
- LLM dependency inside the substrate

## Confirmed vision constraints

- Product form: **Rust crate(s)** for consumption by games and tools
- World model: **GPU-resident** voxel matter substrate
- Layering: substrate has **zero LLM dependency**; game rules live above
- If a validation harness ships in-repo, it is **not** a privileged second implementation path
- Consumer boundary between reusable substrate and any harness is mandatory; exact package layout is design, not vision

## Deferred design decisions

- Voxel size, LOD, meshing algorithm choice, and object-layer capacity limits
- Depth and sequence of world systems (which fluid/integrity/CA behaviors ship when)
- Persistence encodings, streaming ring policy, and save formats
- Exact crate split and workspace layout enforcing the public boundary
- Whether and how multiplayer authority is pursued later
- Harness-only choices: seed world contents, controller, camera, benchmarks, and target machines

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world **validation harness** a required delivery of this repository, or only permitted as an adjacent artifact?

- **Proposed safe answer:** Permitted and recommended for proving public APIs, but not part of product identity; substrate crates can ship without mandating a specific demo executable in the vision.
- **If different:** Making the harness mandatory adds a repository delivery obligation (still outside product identity) without importing its controls, content, or performance gates into the substrate promise.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident Rust voxel substrate and separates the walkable-world executable as consumer/harness, not game layer.
- **docs/seeds/project-boundary.md** — Binds current product identity to reusable Rust crate(s), excludes the game and listed game layers, and requires any harness to use public interfaces only.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate outcome families (natural look over voxel truth, mutability, deep Z, geology-first gen, matter systems, API layering, persistence/streaming) without making mechanism inventory the vision.
- **docs/seeds/product-one-seed.md** — Describes a first walkable-world validation slice that motivates dig/place proof and public API use; its content, controller, milestones, and machine targets remain harness-owned and do not narrow substrate identity.
