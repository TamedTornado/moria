# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world product: generated natural surface and deep underground volumes whose authoritative truth is mutable voxel matter, consumable by external games through public interfaces. It is not a game.

The complete product mandate is the full substrate (generation, matter behavior families, and consumer contract below). The **first delivery slice** is narrower but settled: a full reusable generation layer; a partial matter layer (operable sparse world, non-authoritative view, surface dressing, voxel-object placement and presentation, static water bodies); and an initial dig/place/query surface. Cellular automata, flowing fluids, structural integrity, granular settling, and richer object lifecycle are postponed past that first slice—not removed from the product.

A **required first delivery adjacent to Moria** (not part of product identity) is a walkable generated-world validation harness: a playable/downloadable proof and benchmark consumer that exercises generation, streaming, meshing, editing, voxel-truth collision, persistence, and performance through public interfaces only.

## Purpose

Moria exists so multiple games can share one material world foundation instead of each rebuilding terrain, dig/build mutability, deep-Z space, and matter-facing observation and mutation. The substrate must stand alone with zero LLM or “System” dependency: game rules live above it; the substrate supplies world matter, physics-relevant behavior, queries, events, and mutation. Downstream consumers should be able to prove that what players see is continuous and natural-looking while remaining fully editable material underneath—not a heightmap with decorative props.

## Product boundary

**In product (Moria):**
- The reusable voxel-world substrate and its public consumer surface as Rust crate(s).
- World generation, material voxel truth, presentation of that truth as a non-authoritative view, mutability, deep-Z occupancy, streaming and persistence (terrain edit truth and durable moved/object state), physics/collision authority against voxel truth, and command-mediated queries, mutations, and events games need.
- Matter-behavior outcome families of the complete substrate: voxel-backed interactive objects and matter-responsive dressing; fluid behavior and material interactions; ambient fire, wetness, weather, and time behavior; granular behavior; structural support and collapse; dynamic matter and object lifecycle—capability depth deferred, ownership not reassigned to games.
- Compatibility seams the substrate itself requires so higher layers can attach later—without implementing those layers here.

**Adjacent, required first delivery (not product identity):**
- A walkable-world executable that validates the fused first-slice and performance claims above through the same public interfaces available to an external game. Character control, camera policy, authored routes, content packs, presentation chrome, workloads, platforms, and machine-specific gates belong to that harness—not to Moria’s identity.

**Out of repository / downstream:**
- The actual game (or games) that will consume Moria.
- Game rules and future System, LLM, spell, gas, combat, AI, and building *gameplay* layers.

## Required product outcomes

- **Reusable Rust substrate:** External consumers integrate Moria as crate(s) with no privileged in-repo path; the adjacent validation harness is bound by the same public surface.
- **Material world truth in continuous space:** Fully mutable voxel volume (destroy, move, place anywhere, including deep underground); surface and underground read as a normal world; mesh and dressing are regenerated views. Authoritative simulation, queries, and physics/collision resolve against voxel truth—not the render mesh.
- **Consumer contract:** Mutation is command-mediated; consumers observe via stale/coarse mirror state and events. Dig/place-class operations and matter queries ship through that public surface so nothing above the matter layer needs direct privileged voxel access.
- **Complete matter-behavior families:** Beyond the first slice, the substrate—not future games—owns interactive voxel objects and matter-responsive dressing, fluid behavior and material interactions, ambient fire/wetness/weather/time behavior, granular behavior, structural support and collapse, and dynamic matter/object lifecycle (depth and mechanisms remain design decisions).
- **GPU-resident operable worlds:** Large regions stay tractable via GPU-resident representation, sparse/lazy materialization, streaming of active neighborhoods, and persistence of generation-plus-edit truth including durable moved and object state.
- **Standalone engine layer:** No LLM/System dependency; no game policy (pricing, combat, AI, spells, fortress labor, etc.).

## Future products and enabling implications

Future *consumers* (not current product scope) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandboxes. Those games own rules, UX, controllers, authored content, presentation, and policy.

High-level enabling implications (not a substitute for the required matter outcomes above):
- Games can price and script the same dig/build/query verbs differently without forking world truth.
- Cross-mode reuse of scarred worlds (e.g. abandoned fort as later dungeon) is enabled by treating edits and object journals as durable world truth.

## Non-goals

- Implementing the playable game(s), game rules, combat, stats, AI agents, or entity ecosystems beyond substrate matter/object infrastructure.
- System/LLM features, spells, gas/pricing policy, intent stacks, or any substrate dependency on those.
- Building *gameplay* layers (work orders, designation UX, mechanism game logic, economy) even where seams may be planned.
- Absorbing harness-owned controllers, cameras, routes, content sets, platforms, or performance gates into Moria’s product identity.
- Multiplayer shipping or a frozen inventory of every matter subsystem’s depth in this brief.

## Confirmed vision constraints

- Product form: Rust crate or small family of tightly scoped Rust crates.
- Consumer boundary: no privileged game- or harness-only path into world truth; public interfaces only.
- Runtime character: GPU-resident voxel-world substrate.
- Independence: zero LLM/System dependency; substrate stands alone as an engine layer.
- First-slice boundary: full generation layer; partial matter (static water, dressing, object placement/presentation; no CA, flowing fluids, integrity, granular settle, or richer object lifecycle yet); initial dig/place/query surface.
- Adjacent delivery: walkable validation harness is required as first delivery, remains outside product identity, and stays unprivileged.

## Deferred design decisions

- Capability depth and mechanisms within each complete matter-behavior family (how far fluids, integrity, ambient sim, objects, and lifecycle go after Product One).
- Representation, meshing, streaming, and persistence mechanisms; voxel scale and LOD strategy; exact public API shape and crate split.
- Whether and how far multiplayer-ready command authority is carried in early design.
- Harness-owned content, controls, platforms, workloads, and acceptance measures (not product identity).

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md:** Names Moria as the reusable GPU-resident voxel-world Rust substrate and identifies the walkable executable as a separate validation consumer of generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md:** Binding boundary—product is the substrate crate(s); game is out of repo; harness is unprivileged when present; game/System/LLM/spell/gas/combat/AI/building layers out of scope (does not cancel required adjacent delivery).
- **docs/seeds/product-one-seed.md:** Settles first-slice substance and required adjacent walkable proof/demo/benchmarks; full generation, partial matter, dig/place/query; harness-owned controls, content, and gates stay adjacent.
- **docs/seeds/voxel-world-substrate.md:** Supplies complete substrate purpose and outcomes—natural look over voxel truth, full mutability, deep Z, matter-behavior families, command/mirror/events contract, voxel-truth physics, persistence including object state, GPU-resident reusable engine without LLM dependency—without transferring game layers into this product.
