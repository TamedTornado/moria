# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world product for downstream games—not a game, demo experience, or content pack.

## Purpose

Moria exists so multiple games can share one material world foundation: a continuous outdoor and underground volume that reads as a natural world, stays fully mutable, and exposes matter, reactive world behavior, queries, mutation-safe navigation support, and mutation without embedding game rules. The substrate must stand alone with no LLM or “System” dependency.

## Product boundary

**Belongs to Moria**

- The reusable voxel-world substrate and its public integration surface for external consumers.
- Generation, material truth, mutation, meshing-as-view, reactive matter behavior, derived navigation support, streaming, and persistence for a diggable natural world with first-class deep Z.
- An adjacent walkable-world executable only as a validation harness if present (delivery unresolved—see Q1). If present, it must use the same public interfaces an external game would use; no privileged or game-specific paths.

**Does not belong to Moria**

- The actual game: gameplay, UX, controllers, characters, authored campaigns, presentation polish, and game-specific policy (including AI agents and movement policy).
- Implementation of game rules and of System, LLM, spell, gas, combat, AI, and building *layers* (compatibility seams may be designed where substrate requirements demand them).
- Validation-harness-owned content and acceptance: seed-route scenery, player character, camera, debug presentation, scripted workloads, machine profiles, and numeric performance gates.

## Required product outcomes

Downstream design must make these product-level outcomes true:

1. **Reusable Rust integration, LLM-free.** Consumers obtain a GPU-resident voxel world through public crate interfaces, with no privileged access path and zero dependency on an LLM or System client.
2. **Natural material world, full geology, deep Z.** Surface and underground read as ordinary natural worlds while remaining fully material; diggable geology (columns, strata, caves, materials, placement metadata) materializes lazily; underground is continuous content of the same world—not a false floor. Generation is a reusable substrate asset, not a demo stub. Matter can be destroyed, placed, and otherwise changed throughout the volume.
3. **Interactive objects and reactive matter.** Burnable, breakable, or blocking instances (trees, bushes, boulders, and similar) are voxel-backed world objects. Noninteractive dressing may be derived from and respond to voxel state without every visual instance itself being a voxel. Matter-responsive dressing; flowing and reactive fluids; ambient weather, time, and fire behavior; and structural and granular failure are substrate world services—not optional game features.
4. **Mutation-safe navigation support.** Derived walkability/navigation data is invalidated after material mutation and supports continuous-3D traversal queries. Agent AI and game movement policy remain downstream.
5. **Non-authoritative view; persistence and streaming lifecycle.** Rendering is derived from voxel truth and regenerated after change. Voxel edits plus moved, broken, or extended object state persist and restore faithfully across runs; residency and activity follow consumer anchors (not only one rendered vicinity) so worlds can be entered, left, and reused without always-resident raw voxels.
6. **Interactive, scalable responsiveness.** Material mutation yields incremental interactive updates; residency stays sparse and scalable under large worlds. Exact hardware thresholds, benchmark scenes, and machine profiles are not product identity.

## Future products and enabling implications

- **Downstream games** (separate products): System-driven ARPG, fortress/colony play, Moria-style descent, pure sandbox. They own rules, economy, agents, spells, gas policy, combat, AI, and building gameplay.
- **Walkable-world executable**: adjacent consumer/harness for exercising generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer. Whether shipping it is a current repository delivery is Q1; its character, route, and gates are never Moria product scope.
- Enabling implication only: clean seams so future semantic or game layers (rooms, work orders, priced policies, mechanisms-as-entities) can attach later without becoming current substrate work.
- Product One’s narrower first-slice matter depth does not shrink the reusable substrate’s required outcome families; first-slice absence is consumer demo sequencing, not optionality.

## Non-goals

- Shipping a playable game, ARPG, fortress mode, or campaign in this product.
- Implementing System/LLM authorship, spells, gas policy, combat, AI, or building-layer features.
- Treating the validation harness’s demo route, controller, or benchmark scene as substrate requirements.
- Making the substrate depend on LLM generation for geology or core world behavior.
- Requiring every visual prop to be an independent voxel instance when matter-responsive dressing is authorized instead.

## Confirmed vision constraints

- Product identity is the reusable substrate, exposed as Rust crate(s), intended for external games.
- Consumer boundary is mandatory: harness and games use public interfaces only; no privileged in-repo world path.
- Game rules and System, LLM, spell, gas, combat, AI, and building layers are out of product implementation scope.
- Substrate design must not require an LLM; the System is a future game-layer client, not a substrate feature.
- GPU-resident material world is part of the product promise.
- Interactive incremental updates and sparse scalable residency are substrate quality outcomes; harness-specific numeric gates and machine profiles are not.

## Deferred design decisions

- Exact crate split, internal layering, and API shape within the public-consumer boundary.
- Mechanisms, algorithms, data layouts, tuning, and delivery *sequence* for already-required outcome families (including full geology and reactive matter services).
- Whether and how a walkable harness is structured if delivery is affirmed (content, controls, platforms, targets remain design/harness concerns).
- Open technical trade-offs left by the seeds (fidelity vs cost, distant representation, object-scale limits).
- Whether a given outcome family is in product scope is **not** deferred when seeds already assign it to the substrate.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation harness a **required current delivery** of this repository, or only a **permitted** adjacent artifact?

- **Proposed answer:** Permitted adjacent artifact—not required for the substrate product to be complete. The repository may include such a harness, and if it does, it must consume public interfaces only.
- **If different:** Treating the harness as mandatory current delivery adds a repository deliverable outside the substrate identity, but still must not import harness-owned character, content, presentation, or acceptance details into the substrate’s required outcomes.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and positions the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Settles product identity (substrate as Rust crate(s)), mandatory public-consumer boundary, game-out-of-repo, and exclusion of game/System/LLM/spell/gas/combat/AI/building layers from implementation.
- **docs/seeds/product-one-seed.md** — Motivates first-slice validation of a material world; full generation and faithful restore/responsiveness apply to the substrate while harness routes, controllers, and numeric gates stay consumer-owned; partial matter slice does not cancel broader substrate outcomes.
- **docs/seeds/voxel-world-substrate.md** — Authorizes natural-looking, fully mutable, deep-Z, geology-generated, GPU-resident outcomes including interactive objects, matter-responsive dressing, fluids, ambient weather/time/fire, structural/granular failure, mutation-safe navigation, multi-anchor streaming, and cross-run object-aware persistence—without mechanism inventory or future game layers as part of this brief.
