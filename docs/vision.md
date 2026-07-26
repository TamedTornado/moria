# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer world foundation: material truth, generation, presentation of that truth, mutation, queries, and related world physics—not a game.

## Purpose

Moria exists so multiple games can share one credible material world instead of each reimplementing geology, mutability, deep underground space, and matter-backed simulation. Downstream titles (adventure/ARPG, fortress/colony, descent, pure sandbox) consume the same public substrate; game rules, content policy, and presentation live above it. The substrate must stand alone with no LLM or “System” dependency.

## Product boundary

**In product**
- The reusable voxel-world substrate and its public consumer interfaces.
- The outcome families in Required product outcomes: natural material worlds, full mutability, deep Z, geology-first generation, public mutation/query surface, and reusable matter/physics for external games.

**Adjacent, not identity**
- A walkable-world executable may exist as a validation harness that exercises the substrate through the same public interfaces available to any external game. It is not a game layer and does not own privileged substrate paths. Whether that harness is a required current delivery is unresolved (see Q1).

**Out of product / other products**
- The actual game(s), including rules, UX, controllers, authored campaigns, characters, combat, AI, spells, gas/pricing policy, building *game* layers (building UI, blueprint gameplay, mechanisms as game systems), and the future System/LLM layer.
- Compatibility seams may be designed where substrate requirements demand them; those game layers are not implemented in this product.

## Required product outcomes

1. **Material world truth** — The world is voxel matter end-to-end: any voxel can be destroyed, moved, or placed. Interactive matter is voxel-backed; surface dressing may exist only as a pure function of that matter, not as a second authoritative world.
2. **Looks like a normal world, behaves like voxels** — Rolling terrain, forests, rivers, cliffs, meadows, and underground geology read as continuous natural form; the grid is authoritative truth, not the aesthetic. Presentation is a regenerated view of matter, not a parallel world.
3. **Deep Z is first-class** — Underground space (caves, strata, ore, aquifers, deep volumes) is real content volume, not a false floor under a heightmap skin.
4. **Geology-first generation** — Worlds are produced as geology that digs and explores honestly, with lazy materialization so large regions remain practical—not heightmaps with rock painted underneath.
5. **Consumer-facing world operations** — External games mutate and query the world only through public verbs and queries (no direct privileged voxel access). Dig/place-class mutation is substrate responsibility; mesh is never the source of truth for physics or queries.
6. **Reusable matter engine** — Matter, queries, mutation, and substrate-owned world physics (including fluids, structural integrity, and interactable voxel objects versus derived dressing) support multiple game styles above a clean layer boundary. Gas/pricing and higher game policy are consumer-injected. Persistence and streaming preserve generated truth plus edit deltas so large worlds and scarred regions remain workable.

## Future products and enabling implications

Future consumers—not this product—include a System-driven ARPG, a fortress/colony game, a Moria-style descent experience, and pure sandboxes. Product-shaped demos and later game slices sit on the substrate; they do not redefine it.

Enabling implications at vision altitude only: the substrate’s mutability, deep Z, generation, streaming, and public operation surface are what let those games share one world foundation. Their gameplay, content, controllers, characters, presentation, and acceptance scenarios remain consumer-owned.

## Non-goals

- Shipping a game, campaign, or playable genre fantasy in this repository’s product identity.
- Implementing System/LLM features, spells, gas economies, combat, AI, or building *game* layers here.
- Treating the validation harness’s character, camera, route, content set, controller scheme, or platform benchmarks as product scope (those belong to the adjacent artifact if present).
- Making the substrate depend on an LLM or any single game’s rules.

## Confirmed vision constraints

- Product form: Rust crate or small family of tightly scoped Rust crates.
- GPU-resident world substrate (as product identity, not a harness-only note).
- Consumers—including any in-repo harness—use the same public interfaces; no privileged or game-specific implementation paths into the substrate.
- Zero LLM dependency in the substrate; the System is a future game-layer client, not a substrate feature.
- Game rules and the listed out-of-scope layers are not implemented here; seams only where the substrate itself requires them.

## Deferred design decisions

- Crate split, internal layering, storage/meshing/sim algorithms, and API shape beyond the public-consumer boundary.
- Voxel scale, LOD, object-layer capacity, fluid/integrity/sim depth and sequencing, and which substrate capabilities land in which delivery slice.
- Whether and how a walkable-world harness is structured, what it demos first, and any performance or platform gates for that artifact.
- Multiplayer, higher semantic layers (rooms, work orders, designation views), and consumer-specific policy tables.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required current delivery** of this effort, or only a **permitted adjacent artifact** that may be added to validate the crate?

*Proposed safe answer:* Permitted adjacent artifact only—product identity stays the reusable substrate; harness delivery is not mandatory for the product to be “Moria.”

*If answered differently:* Requiring the harness keeps product identity as the substrate but adds a current non-game delivery obligation (still via public APIs only). That changes planning scope and “done” for the current effort without turning the harness into the product or importing its controller, content, or acceptance details into substrate identity.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident Rust substrate and positions the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Binding identity and boundary: reusable Rust substrate; game out of repo; harness (if present) via public APIs only; game/System/building layers excluded.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look vs. voxel truth, full mutability, deep Z, geology-first gen, matter/physics/queries/mutation, GPU-resident, multi-game reuse, no LLM dependency) without making design inventory the vision.
- **docs/seeds/product-one-seed.md** — Describes a first walkable-world demo/harness slice that motivates and validates substrate capabilities; its controllers, content, milestones, and platform gates remain consumer-owned and do not redefine current product identity.
