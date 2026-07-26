# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world product for downstream games—not a game, demo experience, or content pack. This repository also delivers a required adjacent walkable-world executable (Product One) that validates the substrate through public interfaces only.

## Purpose

Moria exists so multiple games can share one material world foundation: a continuous outdoor and underground volume that reads as a natural world, stays fully mutable, and exposes matter, living and reactive behavior, queries, mutation-safe navigation support, and mutation without embedding game rules. The substrate stands alone with no LLM or “System” dependency. Product One proves that foundation as a walkable validation experience and benchmark bed—not as a game layer.

## Product boundary

**Belongs to Moria (substrate)**

- The reusable voxel-world substrate and its public integration surface for external consumers.
- Generation, material truth, mutation, meshing-as-view, dynamic and living matter behavior, derived navigation support, streaming, and persistence for a diggable natural world with first-class deep Z.

**Required adjacent delivery (not substrate identity)**

- A walkable-world executable that consumes the substrate only through public interfaces available to an external game. It validates generation, streaming, meshing, editing, collision, persistence, and performance, and ships a character-driven walkable experience plus downloadable demo and benchmarks. Character, route, presentation, machine gates, and platform details are harness-owned, not substrate requirements.

**Does not belong to Moria**

- The actual game: gameplay, UX, controllers, characters, campaigns, presentation polish, and game-specific policy (including AI and movement policy).
- Implementation of game rules and of System, LLM, spell, gas, combat, AI, and building *layers* (compatibility seams may be designed where substrate requirements demand them).
- Harness-owned content and acceptance: seed-route scenery, player character, camera, debug presentation, scripted workloads, machine profiles, and numeric performance gates.

## Required product outcomes

Downstream design must make these product-level outcomes true:

1. **Reusable Rust integration, LLM-free, public command/observe contract.** Consumers obtain a GPU-resident voxel world through public crate interfaces only. Integration is command-in and observe-out: mutations via verbs; a stale/coarse mirror plus events out—not privileged voxel access. Layers above matter must not touch voxels directly. Zero LLM or System dependency.
2. **Natural material world, full geology, deep Z.** Surface and underground read as ordinary natural worlds while remaining fully material; diggable geology materializes lazily; underground is continuous content—not a false floor. Generation is a reusable substrate asset. Matter can be destroyed, moved, or placed throughout the volume.
3. **Dynamic living matter and ambient world response.** Burnable, breakable, or blocking instances (trees, bushes, boulders, and similar) are voxel-backed world objects with physical transitions (including movable/falling when support fails). Growth and ecological evolution respond to time and weather. Matter-responsive dressing; flowing and reactive fluids; day/night, seasons, weather, and fire; and structural and granular failure are substrate world services. Product One may omit felling, active simulation, and growth as a first-slice limit without shrinking these substrate families.
4. **Authoritative material truth for interaction.** Collision, physical interaction, and world queries operate against material/voxel truth, not the render mesh, and remain correct after mutation. Rendering is a derived, regenerated view.
5. **Mutation-safe navigation support.** Derived walkability/navigation data is invalidated after material mutation and supports continuous-3D traversal queries. Agent AI and game movement policy remain downstream.
6. **Persistence and streaming lifecycle.** Untouched generation plus voxel edit deltas restore exactly for material truth. Separate journals cover felled trees, moved objects, entities, and script state for cross-run reuse, without Product One’s exact voxel-delta restoration modifier. Residency follows consumer anchors so worlds can be entered, left, and reused without always-resident raw voxels. Mutation yields incremental interactive updates; residency stays sparse and scalable. Exact hardware thresholds, benchmark scenes, and machine profiles are not product identity.

## Future products and enabling implications

- **Downstream games** (separate products): System-driven ARPG, fortress/colony play, Moria-style descent, pure sandbox. They own rules, economy, agents, spells, gas policy, combat, AI, and building gameplay.
- **Walkable-world executable / Product One**: required adjacent consumer and validation harness—not a game layer and not substrate identity. Character, route, presentation, and numeric gates remain harness-owned.
- Enabling implication only: clean seams so future semantic or game layers can attach later without becoming current substrate work.
- Product One’s narrower first-slice matter depth does not shrink the substrate’s required outcome families.

## Non-goals

- Shipping a playable game, ARPG, fortress mode, or campaign in this product.
- Implementing System/LLM authorship, spells, gas policy, combat, AI, or building-layer features.
- Treating the harness demo route, controller, or benchmark scene as substrate requirements.
- Making the substrate depend on LLM generation for geology or core world behavior.
- Requiring every visual prop to be an independent voxel instance when matter-responsive dressing is authorized instead.

## Confirmed vision constraints

- Product identity is the reusable substrate as Rust crate(s); the walkable-world executable is a required adjacent delivery outside that identity.
- Consumer boundary is mandatory: public interfaces only; no privileged path; no direct voxel access from layers above matter.
- Integration observability: commands in; stale/coarse mirror plus events out.
- Collision, physical interaction, and world queries use material truth and stay correct after mutation; the mesh is never authoritative.
- Game rules and System, LLM, spell, gas, combat, AI, and building layers are out of product implementation scope.
- No LLM requirement for the substrate; GPU-resident world, interactive incremental updates, and sparse scalable residency are substrate quality outcomes; harness numeric gates and machine profiles are not.

## Deferred design decisions

- Exact crate split, internal layering, and API shape within the public-consumer boundary.
- Mechanisms, algorithms, data layouts, tuning, and delivery *sequence* for already-required outcome families.
- Harness content, controls, platforms, and numeric targets for Product One (harness-owned).
- Open technical trade-offs left by the seeds (fidelity vs cost, distant representation, object-scale limits).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Should the substrate’s public scope statement include **server-authoritative multiplayer readiness** (command/observe architecture kept multiplayer-ready) even though multiplayer itself is not built?

- **Proposed answer:** Yes—keep multiplayer readiness in the scope statement as an integration promise, without implementing multiplayer.
- **If different:** Omitting readiness leaves authority and integration promises single-player-only until a later product redefinition; including it binds the public contract to server-authoritative-ready command/observe semantics without requiring multiplayer now.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Settles product identity (substrate as Rust crate(s)), mandatory public-consumer boundary, game-out-of-repo, and exclusion of game/System/LLM/spell/gas/combat/AI/building layers from implementation.
- **docs/seeds/product-one-seed.md** — Establishes Product One as the required first-slice adjacent delivery (walkable experience, demo, benchmarks); exact voxel-delta restore and dig/place-plus-mirror API; collision against voxel occupancy; first-slice omission of felling, active sim, and growth does not redefine the substrate.
- **docs/seeds/voxel-world-substrate.md** — Authorizes natural-looking, fully mutable (destroy/move/place), deep-Z, geology-generated, GPU-resident outcomes including voxel-backed objects with physical transitions and growth, matter-responsive dressing, fluids, ambient weather/time/fire, structural/granular failure, voxel-truth queries and physics, command/mirror/event integration, mutation-safe navigation, multi-anchor streaming, edit-delta plus object journals, and the open multiplayer-readiness scope question—without mechanism inventory or future game layers in this brief.

