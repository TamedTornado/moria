# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer product: the material world foundation for future games—not a game, not a vertical-slice ARPG, and not a fortress or descent title.

## Purpose

Moria exists so multiple future games can share one stand-alone world foundation: a natural-looking continuous landscape whose ground truth is fully mutable voxels, including deep underground play, without baking in any one game’s rules, content, controllers, or presentation. The substrate must stand alone with zero dependency on an LLM or “System” layer.

## Product boundary

**In product:** the reusable substrate and its public integration surface for external Rust game consumers—generation of material worlds, GPU-resident matter, meshing as a non-authoritative view, mutation and query through that public surface, streaming of active regions, persistence of edits over generable truth, and occupancy truth suitable for consumer collision and traversal.

**Adjacent, not identity:** a walkable-world executable may live in-repo only as a **validation harness**. If present, it must use the same public interfaces an external game would; it must not own privileged or game-specific substrate paths. Its character controls, camera, authored demo route and scenery, debug UX, presentation polish, scripted workloads, machine-specific benchmarks, and performance gates are harness-owned, not substrate scope.

**Out of product / out of this repository’s product work:** the actual game; game rules; and the System, LLM, spell, gas, combat, AI, and building **layers**. Compatibility seams may be designed where substrate requirements demand them; those layers must not be implemented here.

## Required product outcomes

1. **Reusable Rust substrate.** External games (and any in-repo harness) depend on Moria only through public crate interfaces; adjacent consumers have no privileged access to the world implementation.
2. **Material world that reads as a normal world.** Consumers get continuous, natural-looking terrain and subsurface space whose look is derived from voxel truth—not a heightmap with disconnected props—so mutation remains honest everywhere reachable.
3. **Mutable everywhere, deep Z first-class.** Any material cell can be destroyed, altered, or placed; underground structure (caves, strata, buried matter) is real content in continuous 3D, not a decorative floor.
4. **Geology-capable generation with sparse scale.** Worlds are produced so large regions remain practical: generation and materialization support a walkable surface and diggable subsurface without treating the entire volume as eagerly resident raw voxels.
5. **Authoritative voxels, view meshes, public verbs.** Physics-relevant and gameplay-relevant truth stays on the material world; rendered geometry is regenerated and never the save or collision authority. Consumers change and inspect the world only through the substrate’s public mutation and query surface.
6. **Streaming and edit persistence.** Active regions can stream; untouched generable world stays cheap; player or tool scars persist as edit deltas over generation truth and reload faithfully.

## Future products and enabling implications

Described **future consumers** (not this product): a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox modes. Gameplay, UX, authored content, presentation, and game-specific policy remain theirs.

**Enabling implications** (not a committed delivery roadmap): keep the substrate free of game policy so different titles can attach their own pricing, rules, and content; preserve room for later matter-side simulations and consumer features (richer fluids, fire, integrity, interactive vegetation objects, ambient weather) without implementing game layers here. A polished “postcard” walkable demo and its milestone theater are consumer/harness concerns that motivate proving the outcomes above—they do not redefine Moria as that demo.

## Non-goals

- Shipping a game, combat loop, economy, or AI agents in this product
- Implementing System/LLM, spell, gas, or building layers in-repo
- Importing harness demo content, controllers, cameras, or acceptance scenarios into substrate scope
- Making substrate correctness or operation depend on an LLM

## Confirmed vision constraints

- **Rust crate (or small crate family)** as the integration ecosystem for intended consumers
- **GPU-resident** voxel-world substrate as part of product identity
- **Substrate, not game:** clean separation so game rules live above; zero LLM dependency for the world layer to function
- **Harness privilege ban:** any validation executable uses only public interfaces available to an external game
- **Excluded layers** (System, LLM, spell, gas, combat, AI, building) are not implemented here; seams only where substrate needs demand them

## Deferred design decisions

- Exact crate graph, API shape, and enforcement layout for the consumer boundary
- Voxel resolution, meshing approach, LOD, sparsity mechanisms, and sim feature depth per delivery slice
- Persistence encoding, streaming policy, and multiplayer readiness beyond the public-verb posture
- Whether, when, and how a walkable harness is built—and all harness-local content, controls, platforms, and numeric gates

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required current delivery** for this repository, or only **permitted**?

- **Proposed answer:** Required as an **adjacent** validation artifact that exercises the substrate through public interfaces, but remains outside product identity (no transfer of its controller, content, presentation, route, or performance gates into Moria’s scope).
- **If different:** If only permitted, the binding handoff is substrate crates and public outcomes alone; demo/harness milestones are optional and must not be treated as product delivery obligations.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident voxel-world Rust substrate and positions the walkable-world executable as a separate validation consumer, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binds product identity to the substrate crate(s), keeps the real game out of repo, forbids privileged harness paths, and excludes game/System/LLM/spell/gas/combat/AI/building layers.
- **`docs/seeds/product-one-seed.md`:** Motivates first-slice proof of a natural, fully material walkable region (mutation, continuous Z, generation, streaming, persistence, performance pressure) without transferring demo controls, content, or machine gates into substrate identity.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies the substrate’s design goals and reusable-engine posture (normal look, full mutability, deep Z, GPU-resident, game-free layering) and names future game consumers; detailed mechanisms stay for design.
