# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine layer for natural-looking, fully material worlds—not a game, demo experience, or content product.

## Purpose

Moria exists so downstream games can share one standing world foundation: geology-backed terrain that reads as a normal outdoor world, remains mutable matter all the way down, treats deep underground as first-class space, and exposes generation, matter, queries, and mutation through public interfaces. Game rules, presentation, and policies live above the substrate; the substrate must stand alone with zero dependency on LLM or game-system layers.

## Product boundary

**In product:** the reusable world substrate—world generation, material voxel truth, view meshing/dressing driven by that truth, streaming and persistence of generated world plus edit deltas, and the public mutation/query surface consumers use to dig, place, inspect, and collide against voxel occupancy.

**Adjacent, not the product identity:** a walkable-world executable may exist in-repo only as a **validation harness**. It must use the same public interfaces available to an external game and must not own privileged or game-specific implementation paths inside the substrate. Harness character, camera, controls, authored demo route, presentation polish, and acceptance workloads are consumer concerns, not product identity (see Q1).

**Out of repository / downstream:** the actual game(s). Game rules and future System, LLM, spell, gas, combat, AI, and building layers are not implemented here. Compatibility seams may be designed where substrate requirements demand them; those layers themselves are not Moria.

## Required product outcomes

- **Natural-looking material world.** Surface worlds read as ordinary terrain (hills, forest, water, cliffs, caves)—the voxel grid is truth, not the primary aesthetic; extracted mesh is a non-authoritative view regenerated from matter.
- **Mutable everywhere, deep Z included.** Any material can be destroyed, moved, or placed; underground strata, caves, and dig-down honesty are first-class, not a painted floor under a heightmap.
- **Geology-first generation.** Worlds are produced as layered geology and biome structure with lazy materialization so large regions idle cheaply until touched.
- **Consumer-facing matter API.** Dig, place, and mirror-style queries are the only way adjacent code touches voxels; nothing above matter reaches storage directly. This is the reuse, sandbox, and multiplayer-readiness boundary.
- **Operational world runtime.** Streaming around active interest and persistence as generation function plus compact edit deltas so scars and progress reload correctly without saving untouched volume.
- **Enforced consumer boundary.** Substrate and any in-repo harness are separated so harnesses and future games share the public surface; no privileged internal path for “our” demo.

## Future products and enabling implications

Future **consumers** (not current Moria scope) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent adventure, and pure sandboxes. They own gameplay, UX, controllers, authored content, presentation, economy, and policy.

High-level enabling implications only: continuous 3D mutable geology supports descent and fortification fantasies; verb/query symmetry lets later games inject pricing and agents without forking matter; sparsity, streaming, and delta persistence make large, scarred worlds practical. Full fluid simulation, structural integrity, fire ecology, vegetation felling, mechanisms, blueprints, rooms, nav policy, and multiplayer are long-horizon substrate directions or consumer layers—motivators, not a committed current roadmap.

## Non-goals

- Shipping a game, ARPG loop, fortress mode, or downloadable “product one” experience as Moria itself
- System/LLM integration, spells, gas metering, combat, stats, AI agents, or building/game-rule layers
- Treating the validation harness’s character, camera, demo route, content set, or platform performance gates as substrate requirements (unless later human decision elevates harness delivery—see Q1—without importing those details into product identity)
- Decorative worlds that cannot dig, or voxel-cube aesthetics as the intended surface look

## Confirmed vision constraints

- **Rust/Cargo ecosystem:** substrate is one or more Rust crates; workspace separation from any harness is the consumer-boundary enforcement surface (crate split detail is design).
- **GPU-resident world substrate** with portable GPU shading/compute via **wgpu/WGSL** (no native Metal-only fork in load-bearing paths).
- **Dev environment reality:** primary development includes Apple Silicon (M4-class unified memory); designs must respect missing 64-bit buffer atomics and bandwidth-bound behavior.
- **Zero LLM dependency** in the substrate; the System is a future game-layer client only.
- **Equal public interfaces** for harness and external games—no privileged in-tree access to voxel truth.

## Deferred design decisions

- Voxel size, brick/layout parameters, meshing algorithm choice, LOD, and object-layer capacity
- Depth of the first vertical slice (which matter sims, fluid tiers, integrity, vegetation rigidity ship when)
- Exact harness content, controls, demo route, and quantitative performance/acceptance gates
- Crate package graph within the substrate family; streaming ring policy details; multiplayer timeline

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world validation harness a **required current delivery**, or only **permitted** beside the substrate crates?

- **Proposed safe answer:** Required as an adjacent deliverable that exercises public APIs for generation, streaming, meshing, editing, collision, persistence, and performance—but remains outside product identity; its specific controller, character, content, presentation, and numeric gates stay out of substrate scope until design.
- **If different:** “Permitted only” means current committed scope is crates alone and harness work is optional; “Required with demo-owned content/targets as product gates” would expand identity toward a playable product and must restate which harness outcomes bind the substrate.

## Seed synthesis

- **`README.md`:** Names Moria as the GPU-resident voxel-world Rust substrate and positions the walkable executable as consumer/harness, not game layer.
- **`docs/seeds/project-boundary.md`:** Binding repository and layer boundary—substrate crates in, game and listed future layers out; harness only via public interfaces.
- **`docs/seeds/product-one-seed.md`:** Motivates a first walkable proof and partial substrate slice; supplies demo content, controls, and gates treated here as harness/design altitude, not fused product identity.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies design-goal altitude for natural look, full mutability, deep Z, geology generation, matter/API layering, and future multi-game reuse without making mechanism inventory or full roadmap current scope.
