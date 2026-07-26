# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for games, not a game and not a game repository.

## Purpose

Games need a natural-looking, fully material world they can dig, reshape, explore underground, stream, and persist without baking game rules into the world engine. Moria exists so multiple downstream games—and any in-repo validation consumer—can share one voxel matter foundation with a clean public boundary.

## Product boundary

**This product owns**
- The reusable substrate: voxel matter, geology-oriented generation, meshing as a non-authoritative view, mutation and query interfaces, collision against voxel truth, streaming, durable restoration and cross-run reuse of material-world changes (including generated-world edits and moved world objects), mutation-aware navigation support, and the matter-behavior families listed under required outcomes.
- The public integration surface those consumers use. Adjacent consumers, including any walkable-world validation executable, get the same interfaces as an external game—no privileged or game-specific paths inside the substrate.

**Adjacent / not this product**
- The actual game lives outside this repository as a separate consumer.
- A walkable-world executable may exist here as an adjacent validation artifact only (see Q1). If delivered, its fused purpose is a walkable generated-world proof that exercises generation, streaming, meshing, editing, voxel-truth collision, persistence, and performance through the public substrate boundary. Its character, camera, controls, authored demo route, content inventory, presentation, machine targets, and numeric gates are not product scope.
- Game rules and the System, LLM, spell, gas, combat, AI, and building *layers* are out of scope. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here. Substrate-owned navigation is not the AI layer.

## Required product outcomes

- **Rust-crate substrate.** Ship a reusable GPU-resident voxel-world foundation consumable by external games through public crate interfaces.
- **Natural look over voxel truth.** Surface worlds read as ordinary terrain (hills, forests, rivers, cliffs, meadows); the voxel grid is the material truth, not the intended surface aesthetic. The mesh is a regenerated view—never authoritative for physics, queries, or saves.
- **Mutable everywhere.** Any voxel can be destroyed, moved, or placed; nothing important is decorative geometry outside the material world.
- **Deep Z is first-class.** Underground space—strata, caves, voids, buried materials—is real world content generated as geology, not a thin floor under a heightmap.
- **Consumer-facing world services.** Through the public boundary, consumers can generate, stream, mesh, edit, collide with (against voxels, not the render mesh), navigate on mutation-aware world-derived traversal support across continuous 3D and multiple movement classes, and durably restore material-world changes—including generated-world edits and moved world objects—for cross-run reuse, without embedding game policy in the substrate.
- **Matter, physics, queries, mutation.** The substrate provides matter and physics services plus public mutation and query interfaces so games share material truth through the API boundary—with zero LLM dependency and no game rules inside the crate. Full-product matter outcomes include interactive voxel-backed objects and voxel-coupled dressing; fluid behavior; ambient weather, time, and fire behavior; and material-dependent integrity and collapse. Depth, mechanisms, and delivery order remain design choices; first-slice deferrals do not drop these families from the product.

## Future products and enabling implications

Future consumers include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, pure sandboxes, and later product slices that start from a walkable world. Enabling implications only: the same substrate should support dig/build-anywhere play, deep underground adventure, and fortress-scale material honesty. Gameplay, UX, controllers, authored content, presentation, economy, and game policy remain consumer-owned. A walkable-world validation or demo consumer may exercise the public boundary; it does not redefine the product.

## Non-goals

- Shipping the actual game (or game rules) in this repository.
- Implementing System/LLM features, spells, gas policy, combat, AI, or building layers here.
- Treating the validation executable’s character, camera, demo world content, hardware-specific acceptance targets, or numeric performance gates as substrate requirements.
- Making cube-voxel aesthetics the primary intended look of the surface world.

## Confirmed vision constraints

- **Ecosystem:** product form is a Rust crate or tightly scoped family of Rust crates.
- **Consumer boundary:** non-optional; validation and games share the public surface only.
- **Standalone substrate:** must work with zero LLM/System dependency.
- **GPU-resident world foundation** as a product-defining placement of matter and related work.
- **Portable graphics stack:** load-bearing layers stay on wgpu/WGSL so the substrate remains portable across backends including Vulkan and DX12; no native Metal fork in those layers.
- **Excluded layers:** game rules and System, LLM, spell, gas, combat, AI, and building layers are not implemented in this product (seams only where substrate needs demand them).

## Deferred design decisions

- How deep to implement each matter-behavior family in any given delivery, and in what order.
- Internal crate split, storage/meshing/streaming/persistence mechanisms, and algorithm choices.
- Numeric performance budgets, benchmark protocol, device-specific atomics limits, and provisional hardware baselines.
- Whether and how a validation executable is structured if Q1 affirms delivery.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world executable a *required repository delivery* beside the substrate crates, or only a *permitted* adjacent validation artifact?

- **Proposed answer:** Permitted only. If present, it must consume the substrate through the same public interfaces as an external game; its controls, content, and acceptance details stay adjacent.
- **If different:** Making it required adds a second delivery commitment—a walkable generated-world proof that exercises generation, streaming, meshing, editing, voxel-truth collision, persistence, and performance through the public boundary—without moving character, content, presentation, machine targets, or numeric gates into product identity. Treating it as absent would remove even that optional adjacent artifact from repository expectations.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident Rust-crate substrate and positions the walkable-world executable as a separate validation consumer of generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **`docs/seeds/project-boundary.md`:** Binding repository and product boundary—substrate only; public-interface consumer rule; game and listed policy layers out of scope; harness permitted but non-privileged.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes substrate purpose and full-product matter outcomes (natural look over voxel truth, universal mutability, deep Z, geology-first world, object/dressing/fluid/ambient/integrity families, mutation-aware navigation, durable world restoration, multi-game reuse, no LLM dependency) without transferring game layers or mechanism inventories into this brief.
- **`docs/seeds/product-one-seed.md`:** First-slice demo/validation consumer motivation (walkable proof of material world) and substrate-level wgpu/WGSL portability; does not expand current product identity with its character, content, milestones, platforms, or numeric gates, and does not cap full-product matter or persistence outcomes by first-slice deferrals.
