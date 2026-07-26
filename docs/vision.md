# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate for games and tools, exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer matter world—not a game, not a character demo, and not an LLM system. A walkable validation executable is a required adjacent delivery that exercises the substrate through public interfaces; it is not part of product identity.

## Purpose

Moria exists so multiple downstream games can share one honest material world: natural-looking terrain that is fully mutable voxel truth, deep underground as first-class space, and generation, matter-derived presentation, mutation, physics-side behavior, collision, streaming, navigation support, and persistence that stay coherent without baking any game’s rules into the substrate. It must stand alone with zero dependency on game policy or an LLM.

## Product boundary

**This product owns** the reusable substrate: geology-oriented world generation; sparse GPU-resident matter; matter-derived surface meshing and dressing as views of voxel truth; public command, query, mirror, and event interfaces; matter destruction, movement, and placement; interactable voxel-backed objects and substrate physics-side behaviors (active fluid flow and material interactions, fire/wetness and ambient ecology, granular settling, structural failure, and dynamic object movement with re-voxelization); collision against voxel truth; mutation-coherent spatial/navigation data and continuous-3D movement support; streaming of active regions; and persistence of voxel mutations plus substrate-owned object lifecycle and state for reuse across runs. Consumers use the same public interfaces and may extend the world through the same registries.

**Adjacent, not this product.** A walkable-world validation executable is a required repository delivery and validation harness. It must use the same public interfaces available to an external game. Its character controller, camera, authored demo content and route, presentation polish, scripted workloads, hardware targets, and performance gates are harness concerns, not substrate identity.

**Downstream, not this product.** The actual game(s)—including any System/LLM layer, spells, gas or pricing policy, combat, AI and pathfinding behavior, agents, and building gameplay layers—are separate consumers. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here. Game presentation (UI, camera policy, demo polish, game-specific visuals beyond matter-derived world presentation) remains consumer- or harness-owned.

## Required product outcomes

- **Integrable, observable substrate.** Downstream games and tools consume Moria as Rust crate(s) through public command, query, mirror, and event interfaces only; no privileged path into matter. Consumers, including an optional System if present above, extend the world through the same registries and verb/query surface.
- **Natural look, material truth.** Worlds read as continuous natural terrain (hills, forests, water, cliffs, caves) while everything that matters remains backed by mutable voxels—not a heightmap with non-material props as the world. Matter-derived meshes and clutter dressing are regenerated views of that truth.
- **Full mutability and deep Z.** Matter can be destroyed, moved, placed, and queried anywhere, including deep underground; strata, caves, and buried materials are real content space, not a decorative floor.
- **Matter and physics-side behavior.** The substrate supplies interactable voxel-backed objects and world behavior covering dynamic movement and re-voxelization, active fluid flow and material interactions, fire/wetness and ambient ecology, granular settling, and structural failure—so digs, floods, and collapses stay honest at the matter layer without owning game rules.
- **Generation, stream, navigate, persist.** Large regions generate as coherent geology; materialize and stream under sparsity so idle volume stays cheap; expose derived, invalidation-safe navigation data and continuous-3D movement support for consumers; and persist voxel deltas plus substrate-owned object lifecycle/state so changed worlds can be reused across runs.
- **Standalone matter foundation.** Simulation, queries, and collision use voxel matter (views are not truth). The substrate does not own game rules, economies, AI policy, or LLM authoring.

## Future products and enabling implications

Future consumers include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox tools. They own gameplay, UX, content, controllers, game-specific pathfinding and AI, policy (including gas/labor pricing), and game presentation beyond matter-derived world views.

Moria enables them as a shared matter substrate with public commands, queries, mirror, and events; geology and deep Z; the matter behaviors above; streaming and cross-run scar/object persistence; and room for game-specific policy above. Those games’ systems, spells, agents, building UX, and acceptance scenarios are not current Moria scope. A first walkable demo slice may omit many matter behaviors without removing those outcome families from the substrate.

## Non-goals

- Shipping the actual game or any game-rules layer in this repository
- Implementing System/LLM, spells, gas metering, combat, AI, or building gameplay layers here
- Treating harness characters, cameras, demo content, or benchmark gates as product identity
- Making the voxel grid the intended surface aesthetic (chunky cubes as the primary look)
- Requiring an LLM or game policy object for the substrate to function

## Confirmed vision constraints

- **Ecosystem:** delivered as a Rust crate or small family of tightly scoped Rust crates for Rust consumers.
- **Consumer isolation:** the validation harness and any external game use the same public interfaces; privileged substrate access is out of bounds.
- **Layering:** game rules and the future System, LLM, spell, gas, combat, AI, and building layers stay out of this product; seams only where substrate needs them.
- **Independence:** the substrate has zero LLM dependency and must work as a standalone engine layer.
- **Residency model:** the world substrate is GPU-resident in the product sense (hot matter on GPU-oriented residency), without prescribing backends or device lines here.
- **Required adjacent delivery:** a walkable validation executable that exercises public interfaces is part of repository delivery, outside product identity.

## Deferred design decisions

- Exact crate split, workspace layout, and API surface shape (including command/mirror/event design)
- Voxel scale, brick layout, meshing strategy, LOD, and storage encodings
- Mechanisms and delivery sequence for matter behaviors (fluid tiers, integrity, granular, fire/ecology, object dynamics)—not whether those outcome families belong to the substrate
- Streaming ring policy, persistence encoding details, and navigation data structures
- Harness content, controller, presentation polish, and performance gates
- Open substrate tuning questions (fidelity vs cost, distant representation, object-layer scale)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Should **server-authoritative multiplayer readiness** remain a product-scope compatibility constraint for the substrate’s public command/verb architecture even though multiplayer itself is not built in this product?

- **Proposed answer:** Yes—keep the public integration surface compatible with a future server-authoritative multiplayer topology as a vision constraint, without implementing multiplayer, networking, or session systems now.
- **If different:** Dropping that constraint allows designs that optimize only for single-process local consumers and may force a later break of the public command/event contract if multiplayer is ever added.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and positions the walkable-world executable as a separate validation harness.
- **docs/seeds/project-boundary.md** — Binding boundary: substrate as the product; game out of repo; harness not privileged; game/System/building layers out of scope.
- **docs/seeds/product-one-seed.md** — Defines first-slice delivery and “done”: required walkable demo and benchmark as adjacent harness validation; first-slice omissions do not erase broader substrate outcomes.
- **docs/seeds/voxel-world-substrate.md** — Substrate purpose and outcome space (natural look over voxel truth, full mutability, deep Z, matter/physics behaviors, commands/mirror/events, navigation support, object and voxel persistence, reusable layering); mechanisms and multiplayer-readiness left open.
