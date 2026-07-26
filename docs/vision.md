# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate for games, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for material worlds—generation, matter authority, presentation of that truth, mutation, observation, derived world data, material-world behavior, streaming, persistence, and consumer extension seams—not a game.

## Purpose

Downstream games need a shared world foundation that reads as a natural surface world, is fully material and mutable all the way down, treats deep underground as real content space, and keeps genre rules, economy, AI, and presentation policy above the substrate. Moria exists so those games share one voxel-truth engine layer with zero LLM dependency inside the substrate.

## Product boundary

**This product owns** the reusable substrate and its public interfaces: geology-oriented generation; GPU-resident matter as authority; meshing and dressing as non-authoritative views; command-in / mirror-and-event-out observation; collision and navigation derived from matter and kept coherent under mutation; material-world behavior; streaming; persistence of generation, matter edits, and object/entity lifecycle state; and registries plus policy seams for materials, placement, objects, and consumer-authored behavior without privileged voxel access.

**Adjacent, not product identity.** A walkable-world executable may exist here as a validation harness (see Q1). If present, it must use the same public interfaces as an external game, with no privileged paths. Fused validation purpose: a walkable generated natural region whose third-person traversal and debug dig/place prove continuous voxel truth while exercising generation, streaming, meshing, collision, persistence, and performance. Controllers, characters, cameras, authored content, demo routes, debug UX, benchmark scenes, machine profiles, and performance gates are not product scope.

**Not this product.** The actual game is a separate downstream consumer and is not in this repository. Game rules and System, LLM, spell, gas, combat, AI, and building layers are out of scope; seams only where the substrate needs them.

**Consumer-owned.** Gameplay, UX, controllers, characters, animation, authored content and presentation, genre policy (including gas/labor pricing), System logic, economy, building gameplay, and harness- or game-specific acceptance scenarios remain with adjacent or future products.

## Required product outcomes

1. **Natural look from voxel truth.** The world reads as ordinary natural terrain while voxels remain material authority. Mesh and render are regenerated views, never truth for physics, queries, or saves.
2. **Mutable continuous 3D matter.** Any material part of the world can be destroyed, altered, or placed. Deep Z is first-class content space, not a decorative floor under a skybox.
3. **Geology-first generation and lazy residency.** Generation yields coherent diggable geology (terrain, strata, caves, materials, placement metadata). Untouched world materializes on need so large regions stay workable.
4. **GPU-authority observation contract.** Consumers issue commands into GPU-resident authority and observe via mirror queries plus events. The mirror may be stale relative to authority. Nothing above the matter layer touches voxels directly.
5. **Material-world behavior.** Required outcomes: interactable voxel-backed objects (including falling and growth), granular settling, active water beyond static bodies, ambient weather/time/fire ecology, and material-dependent structural failure. Tiering and mechanisms are design.
6. **Mutation-safe derived world.** Collision and navigation stay coherent as matter changes; continuous-3D movement classes are supported. Exact graphs and class implementations are design.
7. **Streamable, multi-facet persistence.** Active state stays GPU-resident and streams around activity. Truth = reproducible generation + voxel edit deltas + journals for moved/felled objects and entity state, with cross-run reuse. Exact reconstruction where consumers or validation depend on it; harness-specific save limits stay with that slice.
8. **Consumer extensibility without consumer content.** Registries and policy seams let consumers extend materials, placement, objects, and authored behavior. System logic, economy, and building gameplay stay out.
9. **Standalone multi-genre foundation.** The same crate stack supports surface adventure, deep descent, fortress-style building, and pure sandbox without embedding those rules or any LLM runtime.

## Future products and enabling implications

Future consumers include a System-driven ARPG, a DF-style fortress/colony game, a Moria-style descent experience, and pure sandbox modes. They motivate a rule-agnostic, interface-clean substrate. Enabling implications only: diggable geology, deep Z, command/mirror/event boundaries, material-world behavior those games exploit, and persistence that carries world scars between modes. Their combat, spells, gas, AI, building UX, content, and presentation are not Moria scope.

## Non-goals

- Shipping the actual game, game rules, or genre systems in this repository.
- Implementing System/LLM, spell, gas, combat, AI, or building layers here.
- Defining success as a particular demo character, route, art pass, or post.
- Absorbing harness or game performance targets, machine profiles, or backend choices into substrate identity.
- Treating decorative non-material geometry as a substitute for mutable voxel truth.
- Building multiplayer sessions or netcode here (server-authoritative readiness is Q2).

## Confirmed vision constraints

- Integration form: Rust crate or small family of tightly scoped Rust crates.
- Substrate stands alone with zero LLM dependency.
- GPU-resident matter is authority; integrate via commands in and mirror queries plus events out; mirror may be stale.
- Any in-repo validation harness uses only public interfaces; privileged harness paths are forbidden.
- Consumer boundary between substrate and games or harness is required.
- Game rules and System, LLM, spell, gas, combat, AI, and building layers are not implemented here; seams only where substrate needs demand them.

## Deferred design decisions

- Exact crate split and workspace layout (boundary required; packaging shape is design).
- Delivery sequence and tier depth of material-world behavior families without dropping those outcomes from product responsibility.
- Voxel resolution, LOD, meshing details, storage encodings, streaming policy, nav structures, and movement-class implementations.
- Harness content, controls, benchmarks, and platform targeting if harness delivery is confirmed.
- Open substrate-seed tradeoffs (object-layer scale, distant terrain presentation, fluid pressure fidelity).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation harness a **required current delivery**, or only a **permitted adjacent artifact** this repository may include?

- **Proposed answer:** Permitted only. Product identity stays the substrate; the vision neither requires nor schedules the harness while this is open, but retains its fused validation purpose so either answer is actionable.
- **If answered differently:** Requiring the harness adds an adjacent delivery obligation (outside product identity, public-API-only) without moving controllers, content, or performance gates into the substrate.

**Q2.** Should **server-authoritative multiplayer readiness** (command architecture kept suitable for later authoritative servers) be a **required substrate scope constraint** even though multiplayer itself is not built?

- **Proposed answer:** Yes at interface-constraint level only: keep the public command/mirror/event boundary structured for a later authoritative server. Do not require multiplayer implementation, sessions, or netcode now.
- **If answered differently:** Omitting readiness may force a later boundary break; requiring full multiplayer work would expand product identity beyond the substrate.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as consumer/validation surface.
- **docs/seeds/project-boundary.md** — Binding identity and repository boundary: substrate crates only; game downstream; harness public-API-only if present; game/System/LLM/spell/gas/combat/AI/building layers out of scope.
- **docs/seeds/product-one-seed.md** — Adjacent first-slice validation consumer (walkable generated region, third-person traversal, debug dig/place proof). Narrower sim exclusions and machine targets constrain that harness slice only.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural voxel truth, mutability, deep Z, geology, GPU command/mirror/event contract, material-world behavior, mutation-safe derived world, multi-facet persistence, registries/policy seams, multi-genre reuse without LLM); leaves multiplayer-readiness scope open.
