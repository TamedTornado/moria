# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world product: generated natural surface and deep underground volumes whose authoritative truth is mutable voxel matter, consumable by external games through public interfaces. It is not a game.

## Purpose

Moria exists so multiple games can share one material world foundation instead of each rebuilding terrain, dig/build mutability, deep-Z space, and matter-facing queries. The substrate must stand alone with zero LLM or “System” dependency: game rules live above it; the substrate supplies world matter, physics-relevant behavior, queries, and mutation. Downstream consumers should be able to prove that what players see is continuous and natural-looking while remaining fully editable material underneath—not a heightmap with decorative props.

## Product boundary

**In product (Moria):**
- The reusable voxel-world substrate and its public consumer surface as Rust crate(s).
- World generation, material voxel truth, presentation of that truth as a non-authoritative view, mutability, deep-Z occupancy, streaming/persistence of world state, and matter-facing query and mutation capabilities that games need.
- Compatibility seams the substrate itself requires so higher layers can attach later—without implementing those layers here.

**Adjacent, not product identity:**
- A walkable-world executable may exist as an adjacent validation harness. If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific implementation paths. Whether that harness is a required current delivery is unresolved (see Q1).
- Character control, camera policy, authored demo routes, demo content packs, presentation chrome, and harness-specific workloads or gates belong to that adjacent artifact—not to Moria’s identity.

**Out of repository / downstream:**
- The actual game (or games) that will consume Moria.
- Game rules and future System, LLM, spell, gas, combat, AI, and building *gameplay* layers.

## Required product outcomes

- **Reusable Rust substrate:** External consumers integrate Moria as crate(s) with no privileged in-repo path; adjacent validation code, if any, is bound by the same public surface.
- **Material world truth:** The world is a fully mutable voxel volume (destroy, move, place anywhere, including deep underground). Authoritative simulation and queries run on that matter, not on render meshes.
- **Natural-looking, continuous space:** Surface and underground read as a normal world (terrain, strata, caves, surface dressing) rather than a cube aesthetic as the primary look; the mesh or dressing is a regenerated view of voxel truth.
- **Deep Z as content space:** Underground is first-class playable volume—geology, voids, and descent—not a thin floor under a heightmap.
- **Consumer-facing matter API:** Games obtain queries and mutations (including dig/place-class operations) through the substrate’s public surface so nothing above the matter layer needs direct privileged voxel access.
- **GPU-resident operable worlds:** Large regions remain tractable via GPU-resident representation, sparse/lazy materialization, streaming of active neighborhoods, and persistence of generation-plus-edit truth so player scars and long sessions are real.
- **Standalone engine layer:** The substrate does not depend on an LLM/System and does not implement game policy (pricing, combat, AI, spells, fortress labor, etc.).

## Future products and enabling implications

Future *consumers* (not current product scope) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandboxes. Those games own rules, UX, controllers, authored content, presentation, and policy.

High-level enabling implications already motivated by the substrate’s role (not a committed delivery catalog here):
- Games can price and script the same dig/build/query verbs differently without forking world truth.
- Fortress-, hydrology-, and collapse-oriented play can sit on substrate matter behavior rather than fake geometry.
- Cross-mode reuse of scarred worlds (e.g. abandoned fort as later dungeon) is enabled by treating edits as durable world deltas.

## Non-goals

- Implementing the playable game(s), game rules, combat, stats, AI agents, or entity ecosystems beyond what the substrate needs as matter/objects infrastructure.
- System/LLM features, spells, gas/pricing policy, intent stacks, or any substrate dependency on those.
- Building *gameplay* layers (work orders, designation UX, mechanism game logic, economy) even where seams may be planned.
- Importing harness- or demo-owned controllers, cameras, routes, content sets, or performance gates into product identity.
- Multiplayer shipping, platform-specific forks, or a frozen full feature inventory of every matter subsystem’s depth in this brief.

## Confirmed vision constraints

- Product form: Rust crate or small family of tightly scoped Rust crates.
- Consumer boundary: no privileged game- or harness-only implementation path into world truth; public interfaces only.
- Runtime character: GPU-resident voxel-world substrate.
- Independence: zero LLM/System dependency; substrate stands alone as an engine layer.
- Scope exclusion: game rules and System, LLM, spell, gas, combat, AI, and building gameplay layers are not implemented in this product.

## Deferred design decisions

- Capability depth and delivery sequence for individual matter behaviors (fluids tiers, structural integrity, fire/wetness CA, vegetation object lifecycle, particle coupling, ambient weather).
- Representation, meshing, streaming, and persistence mechanisms; voxel scale and LOD strategy; exact public API shape and crate split.
- Whether and how far multiplayer-ready command authority is carried in early design.
- Any walkable harness’s content, controls, platforms, and acceptance measures—if that artifact is built (see Q1).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required current delivery** of this effort, or only a **permitted adjacent artifact**?

- **Proposed safe answer:** Permitted adjacent artifact only. Product identity and success criteria remain the reusable substrate; a harness may be added to exercise public interfaces but is not itself the product promise.
- **If answered differently:** Making the harness mandatory adds a shippable walkable executable to current delivery commitments (still consuming only public APIs) without moving game rules or demo-owned content/controls into substrate identity—but the program then owns producing that adjacent binary, not substrate crates alone.

## Seed synthesis

- **README.md:** Names Moria as the reusable GPU-resident voxel-world Rust substrate and positions the walkable executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md:** Binding boundary—product is the substrate crate(s); game is out of repo; harness if present is unprivileged; game/System/LLM/spell/gas/combat/AI/building layers out of scope.
- **docs/seeds/product-one-seed.md:** Motivates a first walkable proof of material world + dig/place and describes harness-shaped demo content, controls, and targets; those details stay adjacent and do not redefine product identity (see Q1).
- **docs/seeds/voxel-world-substrate.md:** Supplies substrate purpose and outcomes—natural look over voxel truth, full mutability, deep Z, matter/physics/queries/mutation, GPU-resident reusable engine without LLM dependency—without transferring game layers into this product.
