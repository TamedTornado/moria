# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** for external games, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer product: a fully material, mutable natural world that consumers drive through public interfaces—not a playable game and not a game rules stack.

## Purpose

Games that need honest dig-anywhere terrain, continuous underground depth, and a surface that reads as a normal outdoor world should not each reimplement world matter, mutation, generation, and presentation of voxel truth. Moria exists so those capabilities live once in a reusable substrate with **zero dependency on LLM or “System” features**, and so multiple game types can sit above the same matter foundation.

## Product boundary

**Belongs to Moria**

- The reusable voxel-world substrate and its public consumer API (queries, mutation verbs, events/mirrors, material and world registries as the integration surface).
- Geology-first world generation, sparse GPU-resident matter, smooth meshing of voxel truth, edit-aware remeshing, collision against voxel occupancy (not the render mesh), streaming, and seed-plus-delta persistence—as substrate responsibilities, not as a shipped game experience.
- Compatibility seams where substrate outcomes require them (so future game layers can attach without forking the world).

**Adjacent, not the product**

- A walkable-world **validation harness** may live in-repo as a separate consumer. It must use the same public interfaces as an external game and must not own privileged or game-specific substrate paths. Its controller, character, camera, authored demo route, presentation polish, and benchmark pageantry are harness concerns—not substrate identity (see Q1).

**Downstream / out of this product**

- The actual game repository and all game rules.
- System/LLM features, spells, gas/pricing policy, combat, AI, agent labor, and game-facing building/UX layers (blueprints-as-gameplay, mechanisms-as-entities, room designation, fortress or ARPG policy).

## Required product outcomes

A downstream design must make these product-level guarantees true:

1. **Material world, not decorated heightmap.** Visible terrain is backed by mutable voxel matter end-to-end; the mesh is a regenerated view, never authoritative truth.
2. **Normal-looking natural surface over voxel truth.** Rolling terrain, forests, water bodies, cliffs, and caves can read as an ordinary outdoor world while remaining diggable, placeable matter—not a cube aesthetic as the primary look.
3. **Mutation everywhere, deep Z first-class.** Any voxel can be destroyed or placed; underground space, strata, and descent are real content dimensions, not a thin floor under a skybox.
4. **Consumer-safe reuse.** External games and any in-repo harness share one public integration surface; nothing above the matter core reaches voxels through privileged shortcuts.
5. **Honest geology generation.** Worlds are generated as geology (columns, strata, caves, materials, lazy materialization) so digging reveals true underground structure, not painted rock under a height surface.
6. **Operable at region scale.** Sparse residency, streaming, and seed-plus-edit-delta persistence keep large regions workable so mutation and exploration remain credible without treating the whole volume as eagerly resident raw voxels.

## Future products and enabling implications

Described **future consumers** (not current Moria scope): a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent roguelike, and pure sandboxes. Those products own gameplay, UX, controllers, content, presentation, and policy.

**Enabling implications** the broader substrate seeds motivate (not a committed current roadmap): richer matter simulation (fire, multi-tier fluids, granular settle, structural integrity), voxel-object vegetation that can couple to rigid-body fall, priced verb/gas policy plugs, and semantic/nav seams for rooms, work orders, and multi-agent play. Preserve high-level substrate responsibility for matter, physics, queries, and mutation; do not import consumer gameplay or treat full simulation breadth as settled first delivery.

## Non-goals

- Shipping a game, combat loop, RPG stats, AI creatures, or multiplayer product.
- Implementing System/LLM authorship, spells, gas economies, or intent pipelines inside the substrate.
- Game-layer building (player blueprints, mechanisms, work orders, room economy) as Moria deliverables.
- Fluids beyond static bodies, weather/season growth sims, or full CA as required current outcomes.
- Treating the validation harness’s character, demo seed composition, or trailer milestones as the product definition.

## Confirmed vision constraints

- **Ecosystem:** Rust crate(s), GPU-resident world, **wgpu/WGSL** load-bearing path (no native-Metal fork in core); portability across graphics backends is intentional.
- **Consumer isolation:** Adjacent consumers have no privileged voxel access; harness and external games share the public surface.
- **LLM independence:** Substrate stands alone with zero LLM dependency; the System is a future game-layer client only.
- **Dev-environment reality:** Apple Silicon (e.g. M4-class) is an intended development target; load-bearing GPU work must not require 64-bit buffer atomics unavailable there.
- **Layering discipline:** Game rules and excluded layers stay out of this product; seams may exist, implementations of those layers do not.

## Deferred design decisions

- First delivery depth within the matter/simulation family (what ships before richer CA, integrity, object felling, multi-tier fluids).
- Voxel resolution, LOD/streaming ring policy, and object-layer scaling—chosen with measurement, not vision fiat.
- Public API shape, crate partitioning, persistence encoding, and meshing strategy details.
- Harness content, controls, platforms, and performance gates (after Q1 settles whether a harness is required).
- How far multiplayer-ready command/mirror discipline is enforced in v1 versus preserved as architecture intent.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1. Is an in-repo walkable-world validation harness a required current delivery, or only a permitted adjacent consumer?**  
**Proposed answer:** Permitted and expected for proving the substrate, but not mandatory for product completeness; when present it remains outside product identity and must use public APIs only.  
**If different:** Making the harness mandatory keeps substrate identity but adds a required adjacent deliverable; forbidding it leaves only crates/tests as proof surfaces and drops any implied walkable demo from current delivery planning.

## Seed synthesis

- **`README.md`:** States product identity as a reusable GPU-resident voxel-world Rust substrate and frames the walkable executable as harness, not game.
- **`docs/seeds/project-boundary.md`:** Binding boundary—substrate is the product; game and excluded layers out; harness may exist only via public interfaces.
- **`docs/seeds/product-one-seed.md`:** First-slice demo intent and material-world proof claims; informs outcomes and non-goals without transferring character, route, or acceptance pageantry into substrate identity.
- **`docs/seeds/voxel-world-substrate.md`:** Long-horizon substrate design goals and future-consumer motivation; fused into purpose, outcomes, constraints, and enabling implications without adopting mechanism inventory as current scope.
