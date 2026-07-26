# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is the material world engine layer for games—not a game.

## Purpose

Provide a standalone world foundation so downstream games share the same mutable voxel truth: geology-first generated matter that can be dug, placed, and transformed throughout continuous surface and deep underground space. Game rules, content policy, pricing of verbs, and how play is presented live above the substrate. The substrate supplies matter, environmental simulation, derived presentation, navigation data, consumer-facing mutation and observation contracts, and persistence—and does not depend on an LLM or on any game system.

## Product boundary

**In product:** the reusable substrate and its public consumer interfaces for geology-first world generation, material storage and mutation, voxel-backed interactive objects and matter-synchronized dressing, granular and fluid behavior, material interactions, ambient environmental simulation, structural support and failure, smooth derived presentation that updates after mutation, mutation-safe navigation derived from voxel truth, command/verb mutation with mirror-and-event observation, consumer-injected pricing policy, shared registries for consumer-authored materials and world content, streaming residency, and exact persistence of substrate-owned mutable and object lifecycle state.

**Adjacent, not this product:** the actual game is a separate downstream consumer and is not part of this repository. A walkable-world executable may exist in-repo only as an adjacent validation harness that consumes the same public interfaces available to an external game (see Q1). That harness’s character, camera, controls, demo route, content inventory, first-slice capability cuts, machine-specific targets, and performance gates are harness-owned and are not substrate scope.

**Excluded from implementation here:** game rules and the System, LLM, spell, gas, combat, AI, and building gameplay layers. Compatibility seams and injection points may be designed where substrate requirements demand them; those game layers are not implemented in this product.

## Required product outcomes

- Downstream consumers integrate a reusable Rust voxel-world substrate through public APIs only—no privileged in-repo access paths.
- Generation is real geology, not a heightmap: strata, caves, ores, aquifers, carved waterways, deep volumes, lazy coordinate-and-seed realization of bricks, and consumer-facing generation metadata ship as designed and must not be cheapened.
- Matter is fully interactive voxel truth: volumes can be destroyed, moved, or placed; voxel-backed interactive objects and dressing stay synchronized with that truth; granular settle, fluid bodies and flows, material interactions, ambient weather/time/fire behavior, and structural support/failure are substrate-owned outcome families.
- Derived presentation is smooth and non-blocky as a non-authoritative view of voxel truth, and updates incrementally after mutation so dig/place and similar changes produce responsive remeshing feedback.
- Consumers mutate through command/verb interfaces, observe through mirrors and events, inject pricing policy without embedding game rules, register consumer-authored materials and world content through shared registries, and obtain mutation-safe navigation derived from changing voxel truth; collision and queries run against voxel occupancy, not a separate decorative mesh.
- Large sparse worlds stream in and out of residency; truth reconstructs from world generation plus deltas and journals for moved or changed objects and entity/script state, with exact restoration of substrate-owned mutable and object lifecycle state and support for cross-run reuse of those deltas.

## Future products and enabling implications

Future consumers include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress or colony game, a Moria-style deep descent experience, and pure sandbox modes. Those products own gameplay, UX, controllers, authored content, concrete pricing tables and economy policy (including gas), and presentation of play. The substrate’s enabling role is shared geology, matter, environmental simulation, presentation, navigation, mutation/observation contracts, and persistence—plus seams those games can attach to—without implementing those game layers here.

## Non-goals

- Shipping the actual game in this repository
- Implementing System/LLM features, spells, gas or pricing policy tables, combat, AI, or building/fortress gameplay systems
- Importing the walkable demo’s character, route, clip goals, content list, first-slice exclusions, or machine-specific targets as substrate requirements
- Requiring LLM adjudication for the world substrate to function

## Confirmed vision constraints

- Product form is a Rust crate or a small family of tightly scoped Rust crates.
- Any in-repo validation executable, if present, must use only the public substrate interfaces available to an external game.
- World substrate operation is GPU-resident as part of product identity.
- Load-bearing graphics layers stay on wgpu/WGSL only; a native Metal fork is rejected so Vulkan/DX12 portability remains a crate goal. Harness-specific hardware and frame-rate gates are not this constraint.
- The substrate must not require System or LLM features to function.

## Deferred design decisions

- Exact crate split, internal layering, and how the public consumer boundary is enforced in the repo layout.
- Delivery depth, sequence, and mechanisms that realize the required generation, matter, presentation, navigation, observation, and persistence outcomes (algorithms, encodings, size budgets, and similar). First-slice limits described for a walkable demo do not redefine product identity or make those outcome families optional.
- Voxel resolution, streaming policy details, and performance budgets beyond the vision-level requirement for responsive mutation feedback.
- Whether multiplayer is built, and concrete command/mirror implementation patterns beyond any multiplayer-readiness property settled by Q2.

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

- **README.md:** Names Moria as a reusable GPU-resident voxel-world substrate (Rust crate) and frames a walkable-world executable as a separate consumer and validation harness, not a game layer.
- **docs/seeds/project-boundary.md:** Binds the product to the substrate crate(s), places the real game outside the repo, requires any harness to use public interfaces, and excludes game, System, LLM, spell, gas, combat, AI, and building layers from implementation here.
- **docs/seeds/product-one-seed.md:** Describes an adjacent first walkable-world demo and harness slice; requires full geology generation and responsive remeshing as substrate proof; pins wgpu/WGSL-only portability as a crate goal; its first-slice matter exclusions do not narrow substrate outcome families.
- **docs/seeds/voxel-world-substrate.md:** Supplies the substrate’s full outcome mandate—geology-first generation, matter and environmental families, smooth incremental presentation, navigation and consumer contracts, lifecycle persistence, multi-game reuse, zero LLM dependency—and leaves multiplayer readiness as an open scope question.
