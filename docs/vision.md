# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate for games, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation: matter, world generation, presentation of voxel truth, mutation, queries, streaming, and persistence. It is not a game.

## Purpose

Downstream games need a shared, standalone world foundation that reads as a natural surface world, is fully material and mutable all the way down, treats deep underground as real content space, and exposes clean public interfaces so genre rules, economy, AI, and presentation policy live above the substrate. Moria exists so those games share one voxel-truth engine layer with zero LLM dependency inside the substrate.

## Product boundary

**This product owns** the reusable substrate and its public consumer-facing interfaces: geology-oriented world generation; GPU-resident matter as authority; meshing and dressing that present matter without becoming authority; mutation and query verbs; collision and navigation data derived from matter; streaming of active regions; and persistence of generated worlds plus edits.

**Adjacent, not product identity.** A walkable-world executable may exist in this repository as a validation harness. If present, it must consume the substrate through the same public interfaces available to an external game and must not own privileged or game-specific implementation paths. Whether that harness is a required current delivery remains open (see Q1). Harness controllers, characters, cameras, authored regions, demo routes, debug UX, benchmark scenes, and performance gates are not product scope.

**Not this product.** The actual game is a separate downstream consumer and is not part of this repository. Game rules and System, LLM, spell, gas, combat, AI, and building layers are out of scope here. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented in this product.

**Consumer-owned.** Gameplay, game UX, controllers, characters, animation, authored content and presentation stacks, genre policy (including gas/labor pricing), and any harness- or game-specific acceptance scenario remain with adjacent or future products unless a later approved vision change says otherwise.

## Required product outcomes

1. **Natural look from voxel truth.** The world must read as ordinary natural terrain (terrain, water bodies, vegetation presence, rock and cut faces) while voxels remain the material authority. The mesh or other render is a regenerated view, never the source of truth for physics, queries, or saves.
2. **Mutable continuous 3D matter.** Any part of the material world can be destroyed, altered, or placed through substrate mutation. Deep Z is first-class: underground volumes, strata, and voids are real content space, not a decorative floor under a skybox.
3. **Geology-first generation and lazy residency.** Generation produces coherent diggable geology (terrain, strata, caves, materials, placement metadata) as a reusable pipeline. Untouched world does not require eager full-voxel residency; matter materializes on need so large regions stay workable.
4. **Public mutation and query boundary.** Consumers change and inspect the world only through public verbs and queries. Nothing above the matter layer touches voxels directly. That boundary is the integration surface for external games and sandboxes.
5. **Streamable, persistent, GPU-resident world.** The substrate keeps active world state GPU-resident, streams around activity, and persists truth as generation plus edit deltas so large mutable regions remain practical across sessions.
6. **Standalone multi-genre foundation.** The same crate stack must support surface adventure, deep descent, fortress-style building games, and pure sandbox consumers without embedding those games’ rules or any LLM runtime in the substrate. Matter-oriented world behavior (including interactable matter-backed objects versus pure surface dressing) is substrate responsibility at this altitude; delivery depth of individual sim families is design.

## Future products and enabling implications

Future consumers include a System-driven ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style descent experience, and pure sandbox modes. They motivate why the substrate must stay rule-agnostic and interface-clean. Enabling implications only: continuous diggable geology, deep Z, public command/query boundaries for policy and agents above the substrate, and persistence that can carry world scars between modes. Their combat, spells, gas, AI, building UX, content, and presentation are not Moria scope.

## Non-goals

- Shipping the actual game, game rules, or genre systems in this repository.
- Implementing System/LLM, spell, gas, combat, AI, or building layers here.
- Defining product success as a particular demo character, route, art pass, or social post.
- Absorbing harness or game performance targets, machine profiles, or backend choices into substrate identity.
- Treating decorative non-material geometry as an acceptable substitute for mutable voxel truth.

## Confirmed vision constraints

- Integration form is a Rust crate or small family of tightly scoped Rust crates.
- Substrate must stand alone with zero LLM dependency.
- Any validation harness in-repo must use only public interfaces; privileged harness paths are forbidden.
- Consumer boundary between substrate and games or harness is required, not optional.
- Game rules and System, LLM, spell, gas, combat, AI, and building layers are not implemented here; seams only where substrate needs demand them.

## Deferred design decisions

- Exact crate split and workspace layout (boundary is required; packaging shape is design).
- Delivery sequence and depth of matter subsystems (for example fluid tiers, structural integrity, fire or granular sim, object felling).
- Voxel resolution, LOD, meshing strategy details, storage encodings, and streaming ring policy.
- Whether and how a harness is contented, controlled, benchmarked, or platform-targeted if delivery of a harness is confirmed.
- Open technical tradeoffs left by the substrate seed (object-layer scale, distant terrain presentation, fluid pressure fidelity).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation harness a **required current delivery**, or only a **permitted adjacent artifact** that this repository may include?

- **Proposed answer:** Permitted only. Product identity stays the substrate; a harness may exist later or alongside without being promised as current delivery. While this is open, the vision neither requires nor schedules the harness.
- **If answered differently:** Requiring the harness adds an adjacent delivery obligation (still outside product identity, still public-API-only) and lets design plan harness milestones without moving controllers, content, or performance gates into the substrate product.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as consumer and validation surface for generation, streaming, meshing, editing, collision, persistence, and performance.
- **docs/seeds/project-boundary.md** — Binding identity and repository boundary: substrate crates only; game downstream; harness public-API-only if present; game/System/LLM/spell/gas/combat/AI/building layers out of scope.
- **docs/seeds/product-one-seed.md** — Adjacent first-slice demo consumer (region, character, debug dig/place proof, milestones, machine targets). Motivates mutability and substrate validation; does not redefine product identity or import demo ownership into Moria.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look from voxel truth, full mutability, deep Z, geology generation, matter/API layering, streaming and persistence, multi-genre reuse without LLM) without making its mechanism inventory or future game layers part of this brief.
