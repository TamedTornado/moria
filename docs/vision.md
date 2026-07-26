# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is the material world engine layer for games—not a game.

## Purpose

Provide a standalone world foundation so downstream games share the same mutable voxel truth: a natural-looking surface world over geology-first generated matter that can be dug, placed, and transformed throughout continuous surface and deep underground space. Game rules, content policy, verb pricing, and presentation of play live above. The substrate supplies matter, environmental simulation, derived presentation, navigation, mutation and observation contracts, and persistence—without LLM or game-system dependency.

## Product boundary

**In product:** the reusable substrate and its public consumer interfaces for geology-first generation, material storage and mutation, voxel-backed interactive objects and matter-synchronized dressing, granular and fluid behavior, material interactions, ambient environmental simulation, structural support and failure, derived presentation of a natural-looking surface over first-class deep underground volume, mutation-safe navigation from voxel truth, command/verb mutation with mirror-and-event observation, consumer-injected pricing policy, shared registries for consumer-authored materials and world content, streaming residency, and persistence of substrate-owned mutable state via edit deltas, object and entity journals, and cross-run reuse.

**Adjacent, not this product:** the actual game is a separate downstream consumer outside this repository. A walkable-world executable may exist in-repo only as an adjacent validation harness using the same public interfaces available to an external game (see Q1). Its purpose is to validate generation, streaming, meshing, editing, collision, persistence, and performance via a traversable generated natural region that demonstrates a continuous, deeply volumetric, mutable voxel world—with full generation and a deliberately narrower matter and API slice for the proof. Character, camera, controls, demo route, content inventory, first-slice cuts, machine-specific targets, and performance gates are harness-owned, not substrate scope.

**Excluded from implementation here:** game rules and the System, LLM, spell, gas, combat, AI, and building gameplay layers. Compatibility seams may be designed where substrate requirements demand them; those game layers are not implemented here.

## Required product outcomes

- Downstream consumers integrate a reusable Rust voxel-world substrate through public APIs only—no privileged in-repo access paths.
- Generation is real geology, not a heightmap: strata, caves, ores, aquifers, carved waterways, deep volumes, lazy coordinate-and-seed realization of bricks, and consumer-facing generation metadata ship as designed and must not be cheapened.
- The surface world reads as a normal natural landscape—rolling terrain, forests, rivers, cliffs, meadows, water—while remaining fully interactive voxel truth all the way down; deep-Z underground (caves, strata, ore, buried volume) is first-class content, not a floor under decorative surface geometry.
- Matter is fully interactive voxel truth: volumes can be destroyed, moved, or placed; voxel-backed interactive objects and dressing stay synchronized with that truth; granular settle, fluid bodies and flows, material interactions, ambient weather/time/fire behavior, and structural support/failure are substrate-owned outcome families.
- Derived presentation is a non-authoritative view of voxel truth (not a Minecraft block look), and updates incrementally after mutation so dig/place produce responsive remeshing; collision and queries run against voxel occupancy, not a separate decorative mesh.
- Consumers mutate through command/verb interfaces, observe through mirrors and events, inject pricing policy without embedding game rules, register consumer-authored materials and world content through shared registries, and obtain mutation-safe navigation from changing voxel truth; large sparse worlds stream; truth reconstructs from world generation plus edit deltas and journals for moved or changed objects and entity/script state, with cross-run reuse.

## Future products and enabling implications

Future consumers include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style deep descent experience, and pure sandbox modes. Those products own gameplay, UX, controllers, authored content, pricing and economy policy (including gas), and presentation of play. The substrate enables shared geology, natural surface and deep-Z matter, environmental simulation, presentation, navigation, mutation/observation contracts, and persistence—plus attach seams—without implementing game layers here.

## Non-goals

- Shipping the actual game in this repository
- Implementing System/LLM features, spells, gas or pricing policy tables, combat, AI, or building/fortress gameplay systems
- Importing the walkable demo’s character, route, clip goals, content list, first-slice exclusions, machine-specific targets, or exact save/load acceptance as substrate requirements
- Requiring LLM adjudication for the world substrate to function

## Confirmed vision constraints

- Product form is a Rust crate or a small family of tightly scoped Rust crates.
- Any in-repo validation executable, if present, must use only the public substrate interfaces available to an external game.
- World substrate operation is GPU-resident as part of product identity.
- Load-bearing GPU substrate layers stay on wgpu/WGSL only; a native Metal fork is rejected so Vulkan/DX12 portability remains a crate goal. Harness-specific hardware and frame-rate gates are not this constraint.
- The substrate must not require System or LLM features to function.

## Deferred design decisions

- Exact crate split, internal layering, and how the public consumer boundary is enforced in the repo layout.
- Delivery depth, sequence, and mechanisms for generation, matter, natural-world presentation, navigation, observation, and persistence. First-slice demo limits do not redefine product identity or make those outcome families optional.
- Voxel resolution, streaming policy details, and performance budgets beyond vision-level responsive mutation feedback.
- Whether multiplayer is built, and concrete command/mirror patterns beyond any multiplayer-readiness property settled by Q2.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world executable a required repository delivery for this phase, or only a permitted adjacent validation artifact?

*Proposed answer:* Permitted only. It is not part of product identity. If later required as a repository delivery, it remains an adjacent harness that must use public APIs; its controller, content, presentation, and acceptance details stay outside substrate scope.

*If different:* Making it mandatory adds a repository deliverable without expanding substrate identity. Forbidding it removes the in-repo validation path and leaves validation entirely to external consumers.

**Q2.** Must the current product preserve server-authoritative multiplayer readiness as a scope property (command/verb and observation contracts compatible with that model) even if multiplayer itself is not built?

*Proposed answer:* Yes. Keep multiplayer readiness as a product compatibility property of the public mutation and observation contracts; do not require implementing multiplayer.

*If different:* Treating readiness as out of scope allows designs that are single-player-only at the contract boundary and may force later redesign if multiplayer consumers appear.

## Seed synthesis

- **README.md:** Names Moria as a reusable GPU-resident voxel-world substrate (Rust crate) and frames a walkable-world executable as a separate consumer validating generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md:** Binds the product to the substrate crate(s), places the real game outside the repo, requires any harness to use public interfaces, and excludes game, System, LLM, spell, gas, combat, AI, and building layers from implementation here.
- **docs/seeds/product-one-seed.md:** Describes an adjacent first walkable-world proof—a traversable generated natural region with full generation and a narrower matter/API slice; pins wgpu/WGSL-only portability across load-bearing layers; its exact restore target and first-slice exclusions stay harness-owned.
- **docs/seeds/voxel-world-substrate.md:** Supplies the substrate’s full outcome mandate—natural-looking surface over voxel truth, first-class deep-Z, geology-first generation, matter and environmental families, presentation, navigation and consumer contracts, delta/journal persistence with cross-run reuse, multi-game reuse, zero LLM dependency—and leaves multiplayer readiness open.
