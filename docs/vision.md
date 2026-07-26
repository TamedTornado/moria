# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate for external games, exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world and matter foundation—not a game, not a demo product identity, and not an LLM-dependent stack.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface over continuous, fully mutable voxel truth with deep underground play, without each game reimplementing world generation, matter, mutation, queries, or related world physics. The substrate must stand alone; game rules and any System/LLM client sit above it and are not required for the substrate to be complete as a product.

## Product boundary

**This product owns**

- The reusable voxel-world substrate (generation, matter representation and mutation, visual meshing as a non-authoritative view of material truth, streaming, and world persistence).
- A public consumer-facing surface through which external games obtain world truth, queries, and edits—without privileged or game-specific implementation paths.
- Substrate-level world capabilities that games reuse: material mutability, continuous deep-Z space, and matter-oriented physics/query support motivated by downstream games but not owned as gameplay.

**Adjacent, not product identity**

- A walkable-world executable may exist in this repository as a validation harness. It is a separate consumer of the substrate, not a game layer and not the product itself. Whether it is a required repository delivery is open (see Q1). While open, this brief does not treat that executable as required, optional, or planned delivery—only as a permitted adjacent artifact shape.
- If present, the harness must exercise the substrate only through the same public interfaces available to an external game.

**Not this product**

- The actual game (separate downstream consumer, outside this repository’s product).
- Game rules and the System, LLM, spell, gas, combat, AI, and building *layers*—compatibility seams may be designed where substrate needs demand them; those layers are not implemented here.
- Harness- or demo-owned presentation, character control, cameras, authored demo routes, fixtures, and acceptance workloads.

## Required product outcomes

- **Material world, not decorative terrain.** Consumers get a generated natural world whose look is backed by voxel material truth; the render mesh is a regenerated view, not the authority for physics, queries, or mutation.
- **Mutable everywhere.** Any material volume can be destroyed, changed, or placed; edits update matter truth and the dependent view so dig/place is real substrate capability, not a cosmetic effect.
- **Deep Z is first-class.** Underground space is continuous playable volume—geology, voids, and descent—not a thin floor under a heightmap.
- **Reusable consumer boundary.** External games integrate through public substrate interfaces for matter, queries, and mutation; nothing above the matter core reaches voxels by private paths. Adjacent validation uses that same boundary.
- **Large sparse worlds stay tractable.** The substrate supports GPU-resident worlds at region scale via sparsity and streaming so untouched volume does not force full raw residency.
- **World persistence for reuse.** Truth is recoverable as generation plus recorded edits so scarred or explored worlds can be saved and restored without treating the entire grid as authored content.

## Future products and enabling implications

Future *consumers* (not current scope) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandboxes. They motivate substrate generality: the same world stack should admit different pricing policies, content authors, and game rules above a shared matter foundation. Enabling implication only: keep the substrate free of hard-wired game policy and of LLM dependency so those clients can attach later. Their gameplay, UX, controllers, characters, content, and presentation remain consumer-owned.

## Non-goals

- Shipping the actual game, combat, stats, AI, or entities beyond what a consumer adds.
- Implementing System/LLM features, spells, gas economies, or building/game-designation layers in this product.
- Treating the walkable-world harness’s demo content, controls, or benchmarks as the substrate’s product definition.
- A Minecraft-cube aesthetic as the primary surface look, or a heightmap-with-props world that is not fully material underneath.

## Confirmed vision constraints

- **Rust crate delivery.** The product is consumed as a Rust crate or tightly scoped Rust crate family.
- **GPU-resident substrate.** World matter residency and related heavy work are GPU-oriented by product intent.
- **Standalone substrate.** Zero LLM dependency; the System is a possible future client, not a substrate feature.
- **Strict consumer isolation.** Validation and games share the public interface only; privileged harness paths are disallowed.
- **Repository product focus.** The reusable substrate is in-scope product work; the game is not.

## Deferred design decisions

- Exact crate split and internal module boundaries (beyond the required consumer/substrate separation).
- Voxel resolution, meshing approach, storage layout, streaming rings, and related algorithms.
- Delivery depth and sequence for matter simulations (fluids, fire, integrity, granular settle, weather, vegetation object behavior) while preserving substrate-level responsibility for matter, physics, queries, and mutation.
- Performance budgets, target machines, graphics backends, and acceptance thresholds.
- Concrete harness content and protocols if a walkable-world executable is delivered (Q1).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world executable a **required repository delivery** alongside the substrate crates, or only a **permitted** adjacent validation artifact?

- **Proposed answer:** Permitted only—product identity and success are defined by the reusable substrate crates and their public boundary; a harness may be added to exercise that boundary but is not required for the product to be complete.
- **If different:** Making the executable mandatory adds a repository delivery commitment for an adjacent consumer without changing substrate identity; it still must not import demo controls, content, or performance gates into product scope.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust crate and situates the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Locks product identity to the substrate crate(s), excludes the game and named game/System layers, and requires any harness to use public interfaces only.
- **docs/seeds/product-one-seed.md** — Supplies a first walkable validation/demo consumer slice (controller, seed region, dig proof, numbers); used only to confirm substrate must enable material mutability and public dig/place-style use, not to import demo scope into the product.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate purpose and outcome families (natural material world, full mutability, deep Z, reusable matter/physics/query/mutation foundation, generation and persistence intent) and future game consumers, without pulling mechanisms or game layers into current delivery design.
