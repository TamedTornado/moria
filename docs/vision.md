# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate or a small family of tightly scoped Rust crates. It owns world matter: natural-looking generated terrain and deep underground volumes that remain fully mutable, queryable voxel truth for any external game that consumes the public API. A walkable-world executable, if shipped, is only an adjacent validation harness—not a game and not a second product identity.

## Purpose

Give multiple future games one shared, honest material world so each consumer can own gameplay, content, and presentation while the substrate owns geology-capable generation, matter mutability, and world-truth queries. The substrate must stand alone without any LLM or game-rules dependency.

## Product boundary

**In product:** the reusable voxel-world substrate and its public consumer-facing surface (matter representation, generation foundations, mutation and query capabilities, streaming and persistence of world state at the substrate level).

**Adjacent, not identity:** a walkable-world validation harness that may exist in-repo solely to exercise the substrate. It must use the same public interfaces available to an external game and must not own privileged or game-specific implementation paths.

**Out of product / downstream:** the actual game; game rules; System/LLM layers; spells; gas/pricing policy; combat; AI; and building layers (including blueprints-as-gameplay, mechanisms, and designation workflows). Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here. Harness-owned character control, camera, demo route, authored showcase content, and consumer-chosen performance or hardware gates are not substrate scope.

## Required product outcomes

- External Rust consumers integrate a GPU-resident world substrate through public interfaces only, with no privileged in-tree access path.
- Generated worlds read as natural, continuous terrain while remaining fully mutable voxel truth; any visual mesh is a non-authoritative view of that truth.
- Matter is mutable everywhere consumers need it: destroy, alter, and place operations are first-class substrate capabilities, not decorative exceptions.
- Deep underground is first-class world space with geology-capable generation (strata, voids/caves, distinct materials)—not a skybox floor under a heightmap shell.
- Large regions stay tractable via sparse, lazy presence and streaming; consumer-visible edits persist relative to generation rather than requiring a fully authored static map.
- World-truth queries support consumer needs such as collision and inspection against matter occupancy, independent of the render mesh.

## Future products and enabling implications

Downstream consumers (not this repository’s product) include a pure sandbox, a Moria-style descent experience, a Dwarf Fortress–style fortress/colony game, and a System-driven ARPG. Long-horizon matter behaviors described for those games (richer fluid and structural simulation, fire ecology, agent labor, gas-priced verbs, System-authored content) are enabling implications for substrate extensibility, not a committed current roadmap and not a transfer of gameplay, controllers, characters, presentation, or content into Moria. Gas/pricing and similar policies remain plug-in concerns above the substrate.

## Non-goals

- Shipping a game, combat loop, AI, spells, gas economy, or System/LLM integration in this product
- Implementing building/gameplay layers (blueprint workflows, mechanisms, room/economy designation)
- Granting the validation harness privileged substrate paths or treating it as the product identity
- Making Minecraft-cube aesthetics the intended primary look of the world
- Importing harness demo content, routes, cameras, or machine-specific acceptance numbers into substrate scope

## Confirmed vision constraints

- Delivery form is a Rust crate or small family of tightly scoped Rust crates for external game consumers
- Any in-repo validation harness consumes only public interfaces shared with external games
- Substrate stands alone with zero LLM dependency
- World substrate is GPU-resident as part of product identity
- Game rules and System, spell, gas, combat, AI, and building layers are not implemented here

## Deferred design decisions

- Depth and sequencing of ambient matter simulation beyond the core mutable natural world (flowing fluids, structural integrity, granular settle, fire/wetness CA, vegetation lifecycle)
- Concrete look/meshing strategy, resolution choices, LOD, and object-layer scaling—to be settled in design with measurement, not vision
- Exact crate partitioning, persistence encoding, streaming ring policy, and numeric performance budgets
- How far multiplayer-oriented command boundaries are emphasized beyond the public mutation/query surface

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a *required* current deliverable of the Moria effort, or only *permitted* as an adjacent consumer?

*Proposed answer:* Required as a thin validation executable that exercises the substrate only through public interfaces—not a game, and not a second product identity. Its controller, content, presentation, route, and performance gates remain outside substrate scope.

*If answered differently:* If only permitted, the current commitment is the substrate library alone and demo/harness milestones are non-binding; if required with game-like scope, product identity expands beyond the reusable substrate boundary this brief preserves.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and frames the walkable executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Binding correction: product is the substrate crate(s); game and listed game layers are out of repo; harness must share public interfaces; consumer boundary is mandatory.
- **docs/seeds/product-one-seed.md** — Motivates first proof of a natural mutable world (generation, meshing-as-view, dig/place, deep-Z, streaming, persistence) via an adjacent walkable demo; demo controls, content, and platform gates do not redefine substrate identity.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate purpose and matter-world outcomes (natural look over voxel truth, full mutability, deep-Z, reusable engine layer without LLM) while leaving mechanisms and long-horizon sim depth to design.
