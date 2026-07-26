# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer world foundation for external games—not a game, demo fantasy, or content product.

## Purpose

Moria exists so multiple game styles can share one material world foundation: a natural-looking surface over fully mutable voxel truth, with first-class deep underground, without embedding game rules or any LLM dependency in the substrate. The substrate provides matter, physics, queries, and mutation; game policy lives above it.

## Product boundary

**This product owns** the reusable world substrate and the public consumer integration surface in Rust—world generation suited to dig-honest geology, material world state, mutation and query surfaces, and presentation of the world as a view of that matter.

**Adjacent or downstream (not this product’s identity):**

- The actual game is a separate consumer and is not part of this repository.
- Game rules and the System, LLM, spell, gas, combat, AI, and building *game* layers are out of scope here (compatibility seams may be designed where substrate needs demand them; those layers are not implemented in Moria).
- A walkable-world executable may exist as an adjacent validation harness. Whether it is a required repository delivery is unresolved (see Q1). While Q1 is open, this brief does not treat that executable as required or optional delivery—only as a permitted adjacent form.
- Harness- or game-owned work stays outside product identity: character controllers, cameras, authored demo routes and seed content, UI, gameplay policy, presentation choices, platform/hardware targets, and performance acceptance gates.

**Boundary rule:** Any in-repo consumer must use the same public interfaces available to an external game—no privileged or game-specific substrate paths.

## Required product outcomes

1. **Natural material world.** Consumers can present a natural-looking surface world (terrain, vegetation, water, rock) whose look is a view of continuous voxel matter—not decorative geometry sitting outside the material world.
2. **Mutable everywhere.** Any voxel matter can be destroyed, moved, or placed through substrate mutation surfaces, all the way down; mutability is substrate truth, not a cosmetic overlay.
3. **Deep Z is first-class.** Underground volume is real content (depth, strata, caves and voids), not a shallow floor under a heightmap skin.
4. **Reusable engine layer.** The same substrate supports external games (sandbox, adventure, fortress-style, and similar) by providing matter, physics, queries, and mutation while game rules remain consumer-owned.
5. **Stands alone.** The substrate functions with zero LLM dependency; higher intelligence or authoring clients are optional consumers, not load-bearing substrate features.
6. **Fair public integration.** Consumers—including any validation harness—integrate only through public interfaces an external game would use.

## Future products and enabling implications

**Future / external consumers** (not current Moria product scope): a System-driven ARPG, fortress/colony-style games, Moria-style descent experiences, pure sandboxes, and any later “product two” built on a walkable world. Their gameplay, content, controllers, characters, and presentation remain consumer-owned.

**Enabling implications** (motivated by substrate purpose; delivery depth and sequence are design, not a committed roadmap here): geology-first generation and lazy large-world residency; surface dressing and interactable voxel-backed objects; multi-tier fluids and ambient world behavior; structural integrity and granular matter; edit-delta persistence and streaming around active anchors; priced verb / mirror-style seams so higher layers can attach without owning voxels.

## Non-goals

- Shipping the actual game, its rules, or its content in this repository.
- Implementing System/LLM, spells, gas policy, combat, AI, or building game-layer systems here.
- Importing harness or demo specifics (controllers, routes, fixtures, hardware, benchmarks) into substrate identity or required outcomes.
- Making substrate correctness or operation depend on an LLM.
- Treating milestone depth of any first consumer slice as a narrowing of substrate identity.

## Confirmed vision constraints

- **Form factor:** Rust crate or small family of tightly scoped Rust crates intended for game consumption.
- **GPU-resident substrate posture:** world matter the engine owns is designed to live GPU-resident as part of product identity (not a consumer-local choice).
- **Mandatory consumer boundary:** in-repo validation consumers do not receive privileged implementation paths versus external games.
- **Excluded layers stay unimplemented** here even when seams are designed for them.
- **No LLM dependency** for the substrate to operate.

## Deferred design decisions

- Exact crate split, APIs, and enforcement layout for the public boundary.
- Capability depth and build order inside the substrate (storage, meshing, simulation tiers, resolution, streaming/persistence encodings, and similar).
- How far seams for multiplayer, scripting, or higher layers are realized in early deliveries.
- If a walkable-world harness is delivered (Q1): its coverage, content, platforms, and acceptance thresholds remain adjacent design—not product identity.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current repository delivery**, or only **permitted** as an adjacent consumer of the substrate?

- **Proposed safe answer:** Permitted only—the product under design is the substrate; a harness may exist but is not required for product completeness.
- **If answered differently:** Requiring the harness adds a repository delivery obligation and a duty that validation exercise public interfaces; it does not move controllers, demo content, presentation, or performance gates into substrate identity or outcomes.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, with the walkable-world executable described as a separate consumer/validation harness rather than a game layer.
- **`docs/seeds/project-boundary.md`:** Locks current product identity to the substrate and Rust crate boundary; keeps the real game and higher game layers out of repo; permits a harness only if it shares external public interfaces.
- **`docs/seeds/product-one-seed.md`:** Describes an adjacent walkable demo/harness slice (content, controller, performance, milestones) that motivates mutability proof and public-API discipline without transferring those demo specifics into substrate scope; raises the open delivery question in Q1.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies substrate purpose and outcome families—natural look over voxel truth, universal mutability, deep Z, reusable matter/physics/query/mutation layer, GPU residency, geology-first world, and zero LLM dependency—while leaving mechanisms and delivery sequencing to design.
