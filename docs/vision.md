# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate for games and tools, exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer matter world—not a game, not a character demo, and not an LLM system.

## Purpose

Moria exists so multiple downstream games can share one honest material world: natural-looking terrain that is fully mutable voxel truth, deep underground as first-class space, and generation, presentation, mutation, collision, streaming, and persistence that stay coherent without baking any one game’s rules into the substrate. It must stand alone with zero dependency on game policy or an LLM.

## Product boundary

**This product owns** the reusable substrate: geology-oriented world generation, sparse GPU-resident matter, derived surface presentation (mesh and dressing as views of matter), public matter queries and mutations, collision against voxel truth, streaming of active regions, and persistence of a generated world plus edit scars. Consumers integrate only through the same public interfaces.

**Adjacent, not this product.** A walkable-world executable may exist as a validation harness that exercises those interfaces. It is not the product identity. Its character, camera, controls, authored demo route, presentation polish, scripted workloads, and acceptance numbers are harness concerns, not substrate identity. Whether that harness is a required repository delivery remains open (see Q1).

**Downstream, not this product.** The actual game(s)—including any System/LLM layer, spells, gas or pricing policy, combat, AI, agents, and building gameplay layers—are separate consumers. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

- **Integrable substrate.** Downstream games and tools consume Moria as Rust crate(s) through public interfaces only; no privileged or game-specific path into matter.
- **Natural look, material truth.** Worlds read as continuous natural terrain (hills, forests, water, cliffs, caves) while everything visible remains backed by mutable voxels—not a heightmap with non-material props as the world.
- **Mutability and deep Z.** Matter can be destroyed, placed, and queried anywhere, including deep underground; strata, caves, and buried materials are real content space, not a decorative floor.
- **Generation, stream, persist.** Large regions generate as coherent geology, materialize and stream under sparsity so idle volume stays cheap, and persist as generation plus edit deltas rather than a full raw dump of every voxel.
- **View vs truth.** Surface meshes and clutter dressing are regenerated views; simulation, queries, and collision use voxel matter so digs, cuts, and traversal match what the world is.
- **Standalone matter foundation.** The substrate provides the material world and the matter-side behaviors that keep dig, fluid bodies, structural honesty, and similar interactions consistent, without owning game rules, economies, or LLM authoring.

## Future products and enabling implications

Future consumers include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox tools. They own gameplay, UX, content, controllers, policy (including gas/labor pricing), and presentation.

Moria enables them by remaining a shared matter substrate: public mutate/query verbs, geology and deep Z, streaming and scar persistence, and room for game-specific policy above the substrate. Those games’ systems, spells, agents, building UX, and acceptance scenarios are not current Moria scope.

## Non-goals

- Shipping the actual game or any game-rules layer in this repository
- Implementing System/LLM, spells, gas metering, combat, AI, or building gameplay layers here
- Treating harness characters, cameras, demo content, or benchmark gates as product identity
- Making the voxel grid the intended surface aesthetic (chunky cubes as the primary look)
- Requiring an LLM or game policy object for the substrate to function

## Confirmed vision constraints

- **Ecosystem:** delivered as a Rust crate or small family of tightly scoped Rust crates for Rust consumers.
- **Consumer isolation:** any validation harness and any external game use the same public interfaces; privileged substrate access is out of bounds.
- **Layering:** game rules and the future System, LLM, spell, gas, combat, AI, and building layers stay out of this product; seams only where substrate needs them.
- **Independence:** the substrate has zero LLM dependency and must work as a standalone engine layer.
- **Residency model:** the world substrate is GPU-resident in the product sense stated by the seeds (hot matter lives on GPU-oriented residency), without prescribing backends or device lines here.

## Deferred design decisions

- Exact crate split, workspace layout, and API surface shape
- Voxel scale, brick layout, meshing strategy, LOD, and storage encodings
- Depth and order of matter behaviors (cellular rules, fluid tiers, integrity, granular settle, object felling)
- Streaming ring policy, persistence encoding, and multiplayer-readiness detail
- Whether and how a walkable harness is structured if delivered (see Q1); its content and performance gates
- Open substrate tuning questions (fidelity vs cost, distant representation, object-layer scale)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** of this repository, or only a **permitted** adjacent artifact?

- **Proposed answer:** Permitted only—the product promise is the substrate; a harness may exist to validate public interfaces but is not required for the product identity to hold.
- **If different:** Making it mandatory adds a repository delivery obligation for a walkable validation executable (still outside product identity; still without importing its controller, content, or performance gates into the substrate).

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and positions the walkable-world executable as a separate consumer/validation harness for core world capabilities.
- **docs/seeds/project-boundary.md** — Binding boundary: substrate as the product; game out of repo; harness not privileged; game/System/building layers out of scope.
- **docs/seeds/product-one-seed.md** — First walkable demo slice and proof points (natural world, dig proof, traversal) that motivate substrate outcomes; harness-owned player, content, and gates do not expand product identity.
- **docs/seeds/voxel-world-substrate.md** — Substrate purpose and outcome space (natural look over voxel truth, full mutability, deep Z, generation, matter behaviors, streaming/persistence, reusable layering) at vision altitude; mechanisms left to design.
