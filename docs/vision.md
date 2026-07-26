# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world product for downstream games—not a game, demo experience, or content pack.

## Purpose

Moria exists so multiple games can share one material world foundation: a continuous outdoor and underground volume that reads as a natural world, stays fully mutable matter, and exposes matter, physics-relevant world behavior, queries, and mutation without embedding game rules. The substrate must stand alone with no LLM or “System” dependency.

## Product boundary

**Belongs to Moria**

- The reusable voxel-world substrate and its public integration surface for external consumers.
- Generation, storage, material truth, mutation, meshing-as-view, streaming, and persistence responsibilities needed for a diggable natural world with first-class deep Z.
- Optional presence of an adjacent walkable-world executable only as a validation harness (delivery obligation unresolved—see Q1). If present, it must use the same public interfaces an external game would use; it must not own privileged or game-specific paths into the substrate.

**Does not belong to Moria**

- The actual game and its repository concerns: gameplay, UX, controllers, characters, authored campaigns, presentation polish, and game-specific policy.
- Implementation of game rules and of System, LLM, spell, gas, combat, AI, and building *layers* (compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here).
- Validation-harness-owned content and acceptance: seed-route scenery, player character, camera, debug presentation, scripted workloads, machine profiles, and performance gates.

## Required product outcomes

Downstream design must make these product-level outcomes true:

1. **Reusable Rust integration.** Consumers obtain a GPU-resident voxel world through public crate interfaces, with no privileged access path reserved for in-repo tools or games.
2. **Natural look, material truth.** Surface and underground environments read as ordinary natural worlds (terrain, vegetation, water, rock, caves) while remaining fully material—not heightmaps with non-material decoration.
3. **Mutable everywhere.** Matter can be destroyed, placed, and otherwise changed throughout the volume; dig and place are substrate responsibilities that keep mutation honest.
4. **Deep Z is first-class.** The underground is continuous content of the same world as the surface—strata, voids, and dig-down discovery—not a false floor under a skybox.
5. **Geology-first generation.** Worlds are produced as diggable geology (columns, strata, caves, materials, placement metadata) with lazy materialization so large regions stay tractable without pre-expanding all voxels.
6. **Matter services for games.** The substrate supplies matter representation, physics-relevant world behavior, mirror-style queries/events, and mutation verbs so ARPG, fortress, descent, and sandbox games can sit above it without reimplementing the world.
7. **View is non-authoritative.** Rendering is derived from voxel truth and regenerated after change; physics, queries, and gameplay-facing truth run against the material world, not the mesh.
8. **Persistence and streaming of material truth.** Untouched generation plus edit deltas, and active-region streaming behavior, are substrate outcomes so worlds can be entered, left, and resumed without treating the whole volume as always-resident raw voxels.
9. **LLM-free substrate.** The world engine functions with zero dependency on an LLM or System client.

## Future products and enabling implications

- **Downstream games** (separate products): System-driven ARPG, fortress/colony play, Moria-style descent, pure sandbox. They own rules, economy, agents, spells, gas policy, combat, AI, and building gameplay.
- **Walkable-world executable**: adjacent consumer/harness for exercising generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer. Whether shipping it is a current repository delivery is Q1; its character, route, and gates are never Moria product scope.
- Enabling implication only: clean seams so future semantic or game layers (rooms, work orders, priced policies, mechanisms-as-entities) can attach later without becoming current substrate work.

## Non-goals

- Shipping a playable game, ARPG, fortress mode, or campaign in this product.
- Implementing System/LLM authorship, spells, gas policy, combat, AI, or building-layer features (UI, blueprints-as-gameplay, mechanisms-as-game systems).
- Treating the validation harness’s demo route, controller, or benchmark scene as substrate requirements.
- Making the substrate depend on LLM generation for geology or core world behavior.

## Confirmed vision constraints

- Product identity is the reusable substrate, exposed as Rust crate(s), intended for consumption by external games.
- The consumer boundary is mandatory: harness and games use public interfaces only; no privileged in-repo world path.
- Game rules and System, LLM, spell, gas, combat, AI, and building layers are out of product implementation scope.
- Substrate design must not require an LLM; the System is a future game-layer client, not a substrate feature.
- GPU-resident material world is part of the product promise (not a mere implementation preference).

## Deferred design decisions

- Exact crate split, internal layering, and API shape within the public-consumer boundary.
- Voxel resolution, meshing approach, storage layout, streaming rings, and related mechanisms.
- Delivery depth and sequence among generation, matter simulation, integrity, fluids, vegetation objects, and related capabilities.
- Whether and how a walkable harness is structured if delivery is affirmed (content, controls, platforms, targets remain design/harness concerns).
- Open technical trade-offs left by the seeds (for example fidelity vs cost, distant representation, object-scale limits).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation harness a **required current delivery** of this repository, or only a **permitted** adjacent artifact?

- **Proposed answer:** Permitted adjacent artifact—not required for the substrate product to be complete. The repository may include such a harness, and if it does, it must consume public interfaces only.
- **If different:** Treating the harness as mandatory current delivery adds a repository deliverable outside the substrate identity, but still must not import harness-owned character, content, presentation, or acceptance details into the substrate’s required outcomes.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and positions the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Settles product identity (substrate as Rust crate(s)), mandatory public-consumer boundary, game-out-of-repo, and exclusion of game/System/LLM/spell/gas/combat/AI/building layers from implementation.
- **docs/seeds/product-one-seed.md** — Motivates first-slice validation pressure and demo-shaped proof of a material world; supplies harness and milestone detail that must not redefine product identity or transfer consumer-owned work into current substrate scope.
- **docs/seeds/voxel-world-substrate.md** — Authorizes outcome families for a natural-looking, fully mutable, deep-Z, geology-generated, GPU-resident world substrate reusable under multiple games, without making mechanism inventory or future game layers part of this brief.
