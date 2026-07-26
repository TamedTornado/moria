# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. Downstream games and tools consume it; they are not the product. This repository also delivers a **walkable-world executable** as a required first adjacent slice: a validation harness and public-API consumer that proves the natural, continuous, mutable world and provides reproducible performance validation—not a game layer and not product identity.

## Purpose

Moria exists so adventure, fortress, sandbox, and related games can share one foundation: a natural-looking, fully material 3D world whose voxel truth supports dig, place, deep underground play, and matter-faithful simulation—without embedding any particular game’s rules, economy, or presentation. The substrate must stand alone with **no LLM or System dependency**. The walkable harness proves that claim through public interfaces, not by owning game policy.

## Product boundary

**In product:** the world substrate—geology-first generation of natural surface and underground; sparse GPU-resident matter; smooth mesh views of voxel truth; mutation and query through a public API; matter-consistent vegetation and dressing; interactable voxel objects with a full physical lifecycle; fluid and integrity simulation as shared world systems; ambient world behavior; mutation-coherent navigation derived from voxel truth; persistence and streaming of all substrate-owned mutable truth; and integration seams games need without owning game policy.

**Adjacent, not identity:** a walkable-world executable is a **required first adjacent delivery** of this repository. It must use the same public substrate interfaces as an external game and must not own privileged or game-specific paths. Its character, camera, route, content set, debug controls, platform, machine targets, and performance gates are harness-owned, not product identity. Its first-slice scope is narrower than the full substrate mandate.

**Out of repository product:** the actual game(s); System/LLM; spells; gas pricing; combat; AI; and game building layers (building UI, blueprints-as-gameplay, mechanisms-as-game systems, work orders, room economy). Compatibility seams may be designed where substrate outcomes require them; those layers are not implemented here. Project Boundary’s exclusion of AI does not transfer substrate navigation to consumers.

## Required product outcomes

1. **Reusable engine layer and first adjacent proof.** Games and the walkable harness receive matter, world queries, mutation, and related world physics through a public crate API. Nothing above the matter boundary touches voxels directly. Gas, labor, or other costs are consumer policy. A walkable-world validation executable is required adjacent delivery: it proves the natural, continuous, mutable world through those public interfaces and provides reproducible performance validation—without importing harness content, presentation, machine gates, or milestone plans into product identity.

2. **Natural world, voxel truth, mutable deep Z.** Surface worlds read as ordinary terrain (hills, forest, water, cliffs, meadows)—not a cube aesthetic—while the voxel grid remains the authority for occupancy, materials, and edits. Rendered meshes are regenerated views, never saved as truth. Any volume can be destroyed, eroded, or filled. Continuous 3D underground (caves, strata, ore, depth) is real content, not a painted floor under a heightmap. Consumers prove mutability by dig/place against occupancy, not a decorative mesh.

3. **Geology-first generation at sparse cost.** Worlds are generated as geology so dig-down stays honest; materialization is lazy and empty or uniform regions stay cheap so large regions are feasible.

4. **Matter-consistent surface, voxel-object lifecycle, and shared world systems.** Interactable world objects can burn, break, block, move or fall, and remain part of voxel truth; tree falling is a full-substrate outcome, even when a first adjacent slice defers felling and rigid conversion. Surface dressing stays coherent with matter under edit and reaction. The substrate owns fluids, support and collapse, granular behavior, and material reaction as shared world systems—plus light ambient behavior that keeps a natural surface legible.

5. **Mutation-coherent navigation.** Navigation is derived from voxel truth, invalidated by world mutation, and supports traversal through continuous 3D space. This is a substrate spatial facility, not an AI or game-rule layer.

6. **Persistence and streaming.** All substrate-owned mutable truth—world edits plus moved and felled object state—survives across runs with faithful restoration. Generation plus edit deltas alone is not sufficient if object lifecycle state is lost. Regions stream around active anchors so large worlds stay workable without keeping the whole volume always resident as raw voxels.

## Future products and enabling implications

Future **consumers** (not this product): a System-driven ARPG, a DF-style fortress/colony game, a Moria-style descent, pure sandbox modes. They motivate reusable geology, mutability, deep Z, integrity, fluids, navigation, and verb/query symmetry. Their gameplay, content, presentation, controllers, characters, and acceptance scenarios stay consumer-owned.

The walkable-world first slice is narrower than the full substrate: generation is complete; dig/place and query proof exist; static water bodies ship while flow and higher fluid tiers do not; voxel objects place and render while felling/rigid conversion wait; ambient CA, integrity, and granular settle wait. Those omissions bound the adjacent proof, not product identity. Long-horizon multiplayer readiness of a command-style API is an enabling implication only.

## Non-goals

- Shipping a game, combat loop, stats, AI, or System/LLM features in this product
- Implementing spell, gas, building-as-gameplay, or economy layers here
- Treating demo content, third-person control, or trailer routes as substrate scope
- Decorative terrain that is not backed by mutable matter
- LLM dependency inside the substrate
- Treating first-slice exclusions (felling, CA/fire, dynamic fluids, integrity, granular settle) as full-product non-goals

## Confirmed vision constraints

- Product form: **Rust crate(s)** for consumption by games and tools
- World model: **GPU-resident** voxel matter substrate
- Layering: substrate has **zero LLM dependency**; game rules live above
- Validation harness is **not** a privileged second implementation path
- Consumer boundary between substrate and harness is mandatory; package layout is design
- Walkable-world executable is a **required** adjacent repository delivery; it is not product identity

## Deferred design decisions

- Voxel size, LOD, meshing algorithm choice, and object-layer capacity limits
- Depth and sequence of world systems beyond the settled first-slice proof
- Persistence encodings, save-slot policy, versioning, streaming policy, and save formats
- Exact crate split and workspace layout enforcing the public boundary
- Whether and how multiplayer authority is pursued later
- Harness-only choices: seed world contents, controller, camera, benchmarks, and target machines

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident Rust voxel substrate and identifies the walkable-world executable as the validation consumer—not a game layer.
- **docs/seeds/project-boundary.md** — Binds product identity to reusable Rust crate(s), excludes the game and listed game layers, permits a walkable harness under equal public interfaces, and does not transfer substrate navigation to consumers when excluding AI.
- **docs/seeds/voxel-world-substrate.md** — Supplies full substrate outcomes: natural look over voxel truth, mutability, deep Z, geology-first gen, voxel-object lifecycle including fall, matter systems, mutation-coherent 3D navigation, and faithful persistence of edits plus moved/felled object state.
- **docs/seeds/product-one-seed.md** — Defines the required first adjacent walkable proof and benchmark delivery, its dig/place public-API use, and first-slice exclusions (including deferred felling) that do not erase broader substrate outcomes; harness content and gates remain harness-owned.
