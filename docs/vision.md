# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** for game and sandbox consumers. It is delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer product: a material world foundation other software builds on—not a game, not a content pack, and not a character-driven demo product.

## Purpose

Games and tools need a shared foundation where a natural-looking outdoor world is also fully material voxel truth: mutable anywhere, continuous in three dimensions including deep underground, and free of decorative geometry that lies about the world. Moria exists so multiple downstream products—adventure, fortress, descent, or pure sandbox—can share that foundation through one public integration surface, with game rules, economy, and AI living above the substrate rather than inside it.

## Product boundary

**Belongs to Moria (current product)**

- The reusable voxel-world substrate: generation of geological and natural terrain, matter representation, mutation and query surfaces, non-authoritative visual meshing driven by voxels, streaming of large worlds, collision against voxel truth, and persistence of world state as generation plus edits.
- The public consumer boundary of that substrate. Any in-repo walkable-world executable, if present, is an adjacent validation harness and must use the same public interfaces an external game would use—no privileged or game-specific paths through the substrate.

**Does not belong to Moria**

- The actual game (or games). They are separate downstream consumers and are not part of this product’s identity or repository purpose.
- Game rules and the System / LLM, spell, gas, combat, AI, and building *game* layers. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.
- Harness- or game-owned work: controllers, characters, cameras, authored demo routes and content, presentation and UX, gameplay policy, and acceptance scenarios for a particular walkable demo.

A walkable-world executable may exist in the repository as an adjacent validation consumer of the substrate (see Q1). Its presence does not expand product identity to include demo content, controls, or platform-specific acceptance gates.

## Required product outcomes

Downstream design must make these product-level outcomes true:

1. **Reusable Rust substrate.** Consumers integrate Moria as a Rust crate (or small family of crates) and treat it as a library boundary, not as a game shell they fork open.
2. **Natural look over material truth.** Surface worlds read as ordinary natural terrain (hills, forests, water, cliffs, and similar), while the voxel field remains the authoritative material world; render meshes are views regenerated from matter, never the source of truth.
3. **Mutable everywhere, including deep Z.** Any matter can be destroyed, moved, or placed; underground volume is first-class playable space (caves, strata, buried structure hooks), not a painted floor under a heightmap.
4. **Geology-first generation with lazy materialization.** Worlds are generated as geology and natural structure so digs and descents remain honest; large regions stay tractable by materializing and retaining cost only where the world is touched or active.
5. **Shared public world services.** Through one public interface style, consumers can drive terrain generation use, streaming, meshing, editing, collision against voxels, and persistence—enough for external games and any validation harness to exercise the substrate without private APIs.
6. **Standalone engine layer.** The substrate has zero dependency on an LLM or “System” client. Matter access goes through substrate verbs and queries so game policy (pricing, rules, agents) can sit above without reaching into raw storage.

## Future products and enabling implications

Described future consumers include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style deep-descent experience, and pure sandbox tools. They are not current deliverables.

High-level enabling implications (not a roadmap inventory): the substrate should remain a clean matter, generation, mutation, and query foundation those genres can share; gas/pricing, labor, combat, and similar policies stay injectable above the substrate rather than hard-wired into it. Vegetation-as-matter, multi-tier fluids, structural integrity, ambient weather, and building-oriented placement are long-horizon substrate *capabilities* motivated by those consumers; delivery depth and sequence are design decisions, not a commitment that every subsystem ships as current scope merely because a broad seed describes it.

## Non-goals

- Shipping a playable game, ARPG, fortress mode, or descent roguelike in this product.
- Implementing System/LLM authorship, spells, gas economies, combat, AI agents, or building-game UX and work-order loops inside the substrate.
- Treating the walkable-world harness’s character, route, seed-world postcard content, or benchmark theater as the product itself.
- Making decorative non-material geometry authoritative for physics, queries, or gameplay-facing world truth.

## Confirmed vision constraints

- **Ecosystem:** Moria is a Rust crate (or small family of crates) for Rust consumers.
- **Residency model:** The world substrate is GPU-resident in the product sense established by the seeds (matter lives where the engine can simulate and mesh it at scale).
- **Consumer equality:** Validation and external games share the public substrate boundary; the harness must not own privileged implementation paths.
- **Layering:** Game rules and listed future game layers stay out of this product; seams may be designed, implementations of those layers must not land here.
- **LLM independence:** The substrate must stand alone with no LLM dependency.

## Deferred design decisions

- Capability depth and build order for matter systems beyond the identity outcomes (e.g. which fluid, integrity, vegetation-object, or ambient behaviors ship when).
- Voxel scale, LOD strategy, object-layer scaling, and related fidelity tradeoffs.
- Exact crate split within the small family, internal storage and meshing schemes, persistence encoding, and streaming ring policy.
- Performance budgets, target machines, and benchmark workloads (including any harness-specific gates).
- Whether multiplayer-oriented command authority is in scope statements for later work.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world executable a **required repository delivery** for the current effort, or only a **permitted** adjacent validation artifact?

- **Proposed safe answer:** Permitted adjacent validation harness only—not part of product identity; if present, it must consume public substrate interfaces only.
- **If answered differently:** Making it a required delivery keeps product identity as the substrate but adds a repository obligation to ship a harness that exercises the public boundary; it still must not import harness controls, content, presentation, or performance gates into substrate outcomes.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and frames the walkable-world executable as a separate validation consumer of core world services—not a game layer.
- **`docs/seeds/project-boundary.md`:** Fixes product identity on the substrate crate family, excludes the actual game and listed game layers from this repository, and binds any harness to public interfaces with a non-optional consumer boundary.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies the substrate design goals and outcome altitude (natural look, full mutability, deep Z, reusable engine layer, GPU-resident matter) and situates future game genres as consumers above the substrate.
- **`docs/seeds/product-one-seed.md`:** Describes a first walkable demo/harness slice and proof points that motivate substrate capabilities; its controllers, seed content, milestones, and machine-specific targets remain adjacent-consumer detail, not expanded product scope.
