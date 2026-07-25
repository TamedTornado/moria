# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or small family of tightly scoped Rust crates. It provides natural-looking, fully material worlds—mutable everywhere, continuous in 3D, with deep underground as first-class space—so downstream games and tools can consume matter, queries, and mutation through public interfaces rather than owning privileged world paths.

## Purpose

The product exists so multiple games can share one honest world foundation: terrain that reads as a normal surface world while remaining voxel truth underneath; dig, place, and related mutation that stay consistent with collision and simulation; and geology-depth that supports mines, caves, and strata without decorative fake floors. Game rules, economies, AI, and presentation stay above the substrate so the same stack can serve sandbox, adventure, fortress, or other modes without forking the world engine.

## Product boundary

- **In product:** the reusable substrate and its public interfaces (generation of material worlds, matter representation and mutation, queries, and the seams needed for external consumers). A walkable-world executable, if present, is an adjacent validation harness that must use those same public interfaces—not a game layer and not a privileged path.
- **Out of product:** the actual game(s), game rules, UX, controllers, cameras, authored demo content and routes, combat, AI, building/ fortess policy, System/LLM, spells, gas/pricing, and other game-layer systems. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.
- **Repository boundary:** Cargo workspace separation between substrate crates and any validation harness is required; exact crate splits are downstream design.

## Future products and enabling implications

Downstream consumers described in the seeds include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox modes. They motivate substrate capabilities (deep-Z geology, mutability, natural surface look, reusable matter/query APIs) but do not import their gameplay, content, controllers, characters, or presentation into current scope. Long-horizon substrate depth that would enable those games (richer ambient simulation, full fluid/building/integrity toyboxes, semantic/nav layers, and similar) is an enabling implication, not a committed current roadmap.

## Non-goals

- Shipping a game, combat loop, progression systems, or LLM/System runtime in this repository.
- Treating the validation harness’s specific character, controls, camera, seed postcard, demo route, or performance gates as product identity.
- Implementing game-owned layers (spells, gas policy, AI, building UI/blueprints as gameplay, economy) inside the substrate.

## Confirmed vision constraints

- Substrate is GPU-resident and stands alone with zero LLM dependency.
- Nothing above the matter/API boundary should need direct voxel ownership; consumers mutate and query through public verbs/interfaces.
- Validation harness, if built, consumes the same public substrate interfaces an external game would use.

## Assumptions proposed for approval

1. **Current identity is the substrate, not “the walkable demo.”** Product-one language about a character running a curated region describes how the substrate is proven, not a second product that replaces the crate boundary.
2. **Capability depth and first-slice mechanisms are deferred.** Seeds list storage, meshing, fluids tiers, milestones, and benchmarks; those shape later design and do not expand current vision beyond the outcome-level substrate identity above.

## Questions for human review

**Q1.** Is a walkable-world executable a **mandatory current deliverable**, or only a **permitted** validation harness adjacent to the substrate?

- **Proposed safe answer:** Permitted and expected as the primary way to prove the substrate, but not the product itself; its controller, content, presentation, route, and acceptance numbers stay harness-owned.
- **If answered differently:** Making the harness mandatory as “product one” still should not import game UX into the crate boundary; declaring it optional out of scope would allow substrate-only delivery without a playable world consumer in-repo.

## Seed synthesis

- **`README.md`:** Names Moria as the GPU-resident voxel-world substrate consumed as a Rust crate; states the walkable-world executable is a separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binds current product to reusable substrate crates; requires public-API harness consumption and Cargo workspace separation; places game rules and System/LLM/spell/gas/combat/AI/building layers out of scope (seams allowed, implementation not).
- **`docs/seeds/product-one-seed.md`:** Motivates early proof that the world is fully material and natural-looking (including mutability as proof); supplies demo/harness-shaped acceptance and first-slice inventory that was treated as consumer/validation detail, not product identity—except where it reinforces substrate outcomes (mutable natural world, public dig/place boundary).
- **`docs/seeds/voxel-world-substrate.md`:** Contributes design goals at vision altitude (normal look vs voxel truth, mutability, deep-Z, substrate-not-game, GPU-resident) and future-consumer motivations; detailed mechanisms, open tech questions, and build-order catalogs remain downstream design.
