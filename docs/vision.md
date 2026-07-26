# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate for games and tools, exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer matter world—not a game, not a character demo, and not an LLM system. A walkable validation executable is a required adjacent delivery that exercises the substrate through public interfaces; it is not part of product identity.

## Purpose

Moria exists so multiple downstream games can share one honest material world: natural-looking terrain that is fully mutable voxel truth, deep underground as first-class space, and generation, matter-derived presentation, mutation, physics-side behavior, collision, streaming, navigation support, and persistence that stay coherent without baking any game’s rules into the substrate. It must stand alone with zero dependency on game policy or an LLM.

## Product boundary

**This product owns** the reusable substrate: geology-oriented world generation as a coordinate-and-seed function; sparse GPU-resident matter; matter-derived surface meshing and dressing as views of voxel truth; public command, query, mirror, and event interfaces; matter destruction, movement, and placement; interactable voxel-backed objects and substrate physics-side behaviors (active fluid flow and material interactions, fire/wetness and ambient ecology, granular settling, structural failure, and dynamic object movement with re-voxelization); collision against voxel truth; mutation-coherent spatial/navigation data and continuous-3D movement support; streaming of active regions; and persistence of voxel mutations plus substrate-owned object lifecycle and state. Consumers share those public interfaces and registries.

**Adjacent, not this product.** A walkable-world validation executable is a required repository delivery. It must use the same public interfaces available to an external game. Its character controller, camera, authored demo content and route, and presentation polish remain harness-owned, not substrate identity.

**Downstream, not this product.** The actual game(s)—including any System/LLM layer, spells, gas or pricing policy, combat, AI and pathfinding behavior, agents, and building gameplay layers—are separate consumers. Compatibility seams may exist where substrate needs them; those layers are not implemented here. Game presentation beyond matter-derived world views remains consumer- or harness-owned.

## Required product outcomes

- **Integrable, standalone substrate.** Downstream games and tools consume Moria as Rust crate(s) through public command, query, mirror, and event interfaces only; no privileged path into matter. Simulation, queries, and collision use voxel matter (views are not truth). The substrate does not own game rules, economies, AI policy, or LLM authoring.
- **Natural look, full mutability, deep Z.** Worlds read as continuous natural terrain while remaining mutable voxel truth—not a heightmap with non-material props. Matter can be destroyed, moved, placed, and queried anywhere, including deep underground; strata, caves, and buried materials are real content space. Matter-derived meshes and clutter dressing are regenerated views of that truth.
- **Matter and physics-side behavior.** The substrate supplies interactable voxel-backed objects and world behavior covering dynamic movement and re-voxelization, active fluid flow and material interactions, fire/wetness and ambient ecology, granular settling, and structural failure—honest digs, floods, and collapses at the matter layer without game rules.
- **Generate, stream, navigate, persist.** Large regions generate as coherent geology from coordinates and a world seed (reproducible, independent materialization); materialize and stream under sparsity so idle volume stays cheap; expose derived, invalidation-safe navigation data and continuous-3D movement support; persist world generation plus voxel edit deltas so first-slice save/load restores voxel edits exactly; and journal substrate-owned object lifecycle and state for cross-run reuse.
- **Measurable interactive performance.** Interactive rendering, edit-to-remesh responsiveness, cold-start into a walkable world, residency under streaming, and save/load performance are binding quality outcomes, with repeatable, hardware-contextualized regression validation—without making any specific device line or numeric gate part of product identity.
- **First-slice adjacent proof.** The required walkable harness must, through public interfaces, prove terrain generation of a natural surface-to-underground world (full first-slice geology), streaming, meshing, editing (public dig/place), collision against voxel truth, persistence (seed plus deltas with exact voxel-edit restore), traversal of continuous 3D space, and measurable performance. That slice intentionally ships a partial matter and API surface: static water bodies and placed/rendered voxel objects are in; active fluid flow, fire/wetness CA, granular settling, structural failure, and object felling/rigid conversion are postponed. Harness controller, camera, content, and presentation stay adjacent.

## Future products and enabling implications

Future consumers include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox tools. They own gameplay, UX, content, controllers, game-specific pathfinding and AI, policy (including gas/labor pricing), and game presentation beyond matter-derived world views.

Moria enables them as a shared matter substrate with public commands, queries, mirror, and events; geology and deep Z; the matter behaviors above; streaming and scar/object persistence; and room for game-specific policy above. Those games’ systems, spells, agents, building UX, and acceptance scenarios are not current Moria scope. First-slice depth does not remove postponed matter outcome families from the substrate.

## Non-goals

- Shipping the actual game or any game-rules layer in this repository
- Implementing System/LLM, spells, gas metering, combat, AI, or building gameplay layers here
- Treating harness characters, cameras, demo content, or presentation polish as product identity
- Making the voxel grid the intended surface aesthetic (chunky cubes as the primary look)
- Requiring an LLM or game policy object for the substrate to function

## Confirmed vision constraints

- **Ecosystem:** delivered as a Rust crate or small family of tightly scoped Rust crates for Rust consumers.
- **Consumer isolation:** the validation harness and any external game use the same public interfaces; privileged substrate access is out of bounds.
- **Layering:** game rules and the future System, LLM, spell, gas, combat, AI, and building layers stay out of this product; seams only where substrate needs them.
- **Independence:** the substrate has zero LLM dependency and must work as a standalone engine layer.
- **Residency model:** the world substrate is GPU-resident in the product sense (hot matter on GPU-oriented residency), without prescribing backends or device lines here.
- **Required adjacent delivery:** a walkable validation executable that exercises public interfaces and proves the first-slice outcomes above is part of repository delivery, outside product identity.

## Deferred design decisions

- Exact crate split, workspace layout, and API surface shape (including command/mirror/event design)
- Voxel scale, brick layout, meshing strategy, LOD, and storage encodings
- Mechanisms and delivery sequence for matter behaviors (fluid tiers, integrity, granular, fire/ecology, object dynamics)—not whether those outcome families belong to the substrate
- Streaming ring policy, persistence encoding details, and navigation data structures
- Harness controller, camera, authored route/content, presentation polish, numeric performance thresholds, benchmark choreography, and hardware baselines
- Open substrate tuning (fidelity vs cost, distant representation, object-layer scale)

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and positions the walkable-world executable as a separate harness for generation, streaming, meshing, editing, collision, persistence, and performance.
- **docs/seeds/project-boundary.md** — Binding boundary: substrate as the product; game out of repo; harness not privileged; game/System/building layers out of scope.
- **docs/seeds/product-one-seed.md** — First-slice proof (natural surface-to-underground world, geology, dig/place, traversal, exact seed+delta restore) and measurable interactive performance with profiled regression; partial matter postponements; harness-owned controller/camera/content.
- **docs/seeds/voxel-world-substrate.md** — Substrate purpose and outcome space (natural look over voxel truth, full mutability, deep Z, matter/physics behaviors, seed-based generation, gen-plus-deltas and object journals, commands/mirror/events, navigation support, reusable layering).
