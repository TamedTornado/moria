# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world product: matter, queries, mutation, and related world physics that games consume. It is not a game, not a game ruleset, and not a character-driven experience product.

## Purpose

Moria exists so multiple downstream games can share one material world foundation: a natural-looking surface over fully mutable voxel truth, with deep underground as first-class space, without each game reimplementing world matter. The substrate stands alone with no LLM or “System” dependency. Game identity, policy, and presentation live above it.

## Product boundary

**Belongs to Moria**

- The reusable voxel-world substrate and its public consumer-facing interfaces (Rust crate boundary).
- Outcome-level world capabilities: authored/generated material worlds, mutability of matter, derived presentation of that matter, matter-backed interaction affordances, and enabling world services (queries, mutation verbs, persistence of edits, streaming of large regions) at the engine layer.
- Compatibility seams only where substrate requirements demand them—not implementations of excluded game layers.

**Does not belong to Moria**

- The actual game (any ARPG, fortress, descent, or sandbox title) and its rules, content, UX, controllers, characters, combat, AI, economy, gas/pricing policy, spells, or LLM/System layer.
- Game-facing building, designation, work-order, and similar policy layers (placement of matter as engine capability remains substrate; game building systems do not).
- Privileged or game-specific implementation paths that bypass the public substrate interfaces.

**Adjacent, not product identity**

- A walkable-world executable may exist as an adjacent validation harness that exercises the substrate only through the same public interfaces an external game would use. Whether that harness is a required repository delivery is unresolved (see Q1). Its controller, character, camera, demo route, seed content, presentation, workloads, platforms, and performance gates are not Moria’s product identity.

## Required product outcomes

Downstream design must make these product-level outcomes true:

1. **Material world truth.** Consumers operate on a fully mutable voxel material world: any matter can be destroyed, moved, or placed; dig and place are first-class substrate capabilities, not decorative geometry outside the simulation.
2. **Looks natural, stays voxel-true.** The world can read as ordinary continuous terrain and structure while remaining voxel-authoritative underneath; rendered surface is a non-authoritative view regenerated from matter, never the save or physics truth.
3. **Deep Z is first-class.** Underground volume (caves, strata, depth) is real playable/sim space, not a painted floor under a heightmap.
4. **Geology-backed generation.** Worlds generate as layered geology and related structure so digging and exploration encounter honest material depth (strata, voids, resources), materializable lazily so large regions remain tractable.
5. **Matter-scale interaction surface.** Interactable surface features that should burn, break, or block are matter-backed (or derived strictly from matter) so mutation, traversal, and sim stay consistent with what the player sees.
6. **Reusable engine boundary.** The same substrate supports multiple game genres above it by exposing matter, physics-relevant behavior, queries, and mutation without embedding game rules, gas policy, or LLM dependency; adjacent consumers have no privileged access beyond public interfaces.

Enabling world services implied by those outcomes—structural integrity, multi-tier fluids, ambient weather/time effects, path-relevant derived data, edit-delta persistence, and active-region streaming—are substrate responsibilities at outcome altitude. Delivery depth and sequence are design concerns, not a narrowing of product identity.

## Future products and enabling implications

Future or separate products that consume Moria (not built here):

- System-driven ARPG and related spell/gas/combat experiences.
- Dwarf Fortress–style fortress/colony play.
- Moria-style descent / adventure and pure sandbox titles.

High-level enabling implications only: consumers will need public mutation and query interfaces, material and placement registries, and seams for game-injected policy (e.g. pricing of verbs) without the substrate owning those policies. Gameplay, content, presentation, controllers, characters, and game-specific systems remain consumer-owned.

## Non-goals

- Implementing the game, System/LLM, spells, gas/intent pricing, combat, AI, or game building layers.
- Treating the validation harness’s demo fantasy (specific character, route, postcard seed, or clip goals) as Moria’s product definition.
- Making the substrate depend on an LLM or in-process game ruleset.
- Multiplayer product delivery as a current commitment (command-style boundaries may remain design-friendly; online service is not promised here).

## Confirmed vision constraints

- **Ecosystem:** Moria is a Rust crate (or small family of tightly scoped Rust crates) for integration by Rust consumers.
- **Residency model:** The substrate is GPU-resident in the sense established for this product (world matter and related sim live as a GPU-centered engine substrate).
- **Consumer isolation:** Any in-repo validation harness must use only public substrate interfaces available to an external game—no privileged harness-only world paths.
- **Standalone engine:** Zero LLM/System dependency inside the substrate; those remain optional game-layer clients.
- **Scope exclusion:** Game rules and the listed future game layers stay out of this repository’s product.

## Deferred design decisions

- Crate split, API surface shape, and internal layering within the substrate family.
- Representation, meshing, storage, generation pipeline, and sim scheme choices that realize the outcomes above.
- Capability depth and delivery sequence (what ships in the first vertical proof versus later substrate growth).
- Whether and how far multiplayer-authoritative deployment is pursued later.
- Validation harness design (if delivered): controls, content, platforms, benchmarks, and acceptance detail.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required current delivery** of this repository, or only **permitted** as an adjacent artifact?

- **Proposed safe answer:** Permitted only—Moria’s committed product is the substrate crates; a walkable-world harness may be added to validate public interfaces but is not required for product completeness.
- **If different:** Requiring it keeps substrate identity but adds a mandatory adjacent executable delivery (still not game scope); it does not import the harness’s controller, content, route, or performance gates into Moria’s identity unless separately decided.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident voxel-world Rust substrate and positions the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binds product identity to the substrate crate(s), excludes the actual game and listed game layers, and requires any harness to consume only public interfaces.
- **`docs/seeds/product-one-seed.md`:** Motivates a first walkable proof of material, mutable, natural-looking world outcomes; harness-specific controls, content, platforms, and metrics stay adjacent and do not redefine the product.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies the substrate’s purpose and outcome family (mutable material world, natural look, deep Z, geology-first generation, matter-consistent interaction, multi-game reuse without LLM dependency) without transferring mechanism inventory or consumer gameplay into this brief.
