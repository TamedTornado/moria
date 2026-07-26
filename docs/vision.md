# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer world foundation for external games—not a game, demo fantasy, or content product.

## Purpose

Moria exists so multiple game styles can share one material world foundation: a natural-looking surface over fully mutable voxel truth, with first-class deep underground, without embedding game rules or any LLM dependency. The substrate provides matter, physics, queries, and mutation; game policy lives above it.

## Product boundary

**This product owns** the reusable world substrate and its public consumer integration surface in Rust: geology-first generation, material world state, unified material behavior, construction-enabling semantics, mutation-safe navigation and continuous-3D traversal support, durable world/object residency and persistence, and world presentation as a view of that matter (smooth voxel-derived terrain and structures; separate voxel-backed interactive objects; voxel-anchored non-voxel dressing).

**Adjacent or downstream (not this product’s identity):**

- The actual game is a separate consumer and is not part of this repository.
- Game rules and the System, LLM, spell, gas, combat, AI, and building *game* layers are out of scope here (compatibility seams may be designed where substrate needs demand them; those layers are not implemented in Moria).
- A walkable-world executable may exist as an adjacent validation harness. Whether the already-described adjacent slice is a required repository delivery is unresolved (see Q1). While Q1 is open, this brief treats it only as a permitted adjacent form—not as required or optional delivery.
- Harness- or game-owned work stays outside product identity: character controllers, cameras, authored demo routes and seed content, UI, gameplay and labor policy, game presentation choices, platforms, and performance acceptance gates.

**Boundary rule:** In-repo consumers use the same public interfaces available to an external game—no privileged or game-specific substrate paths. Separately, higher layers integrate only through the substrate’s command/query/event boundary with consumer-supplied pricing policy.

## Required product outcomes

1. **Natural material world with correct representations.** Surface look is a view of material truth: terrain and structures use smooth voxel-derived presentation; interactive vegetation and objects are voxel-backed with their own representations; non-voxel clutter is voxel-anchored dressing responsive to the underlying world—not decorative geometry outside material truth, and not one uniform voxel look for all elements.
2. **Mutable everywhere with deep Z.** Any voxel matter can be destroyed, moved, or placed through substrate mutation surfaces, all the way down; underground volume (depth, strata, caves and voids) is first-class content, not a shallow floor under a heightmap skin.
3. **Unified material behavior.** World-matter behavior includes voxel-backed interactive objects, voxel-anchored responsive dressing, disturbed fluids, thin-but-present ambient fire/weather/time behavior, granular movement, and structural failure. First-harness-slice exclusions do not narrow this substrate responsibility.
4. **Construction-enabling world semantics.** Placement and stamps, reusable structure descriptions, mechanism participation in the material world, and queryable spatial structure so build-style consumers can attach. Consumer gameplay, labor policy, UI, and authored content remain outside this product.
5. **Traversable, seed-reproducible, durable worlds.** Mutation-safe navigation and continuous-3D traversal support; seed-reproducible geology with scalable lazy residency; durable world and object changes across runs and around active anchors. Mechanisms and machine-specific thresholds are design, not identity.
6. **Reusable engine with mandatory higher-layer boundary.** Matter, physics, queries, and mutation for external games while game rules remain consumer-owned; higher layers use a command/query/event boundary with consumer-supplied pricing; zero LLM dependency; consumers—including any validation harness—use only public interfaces an external game would use.

## Future products and enabling implications

**Future / external consumers** (not current Moria scope): a System-driven ARPG, fortress/colony-style games, Moria-style descent experiences, pure sandboxes, and later products on a walkable world. Their gameplay, content, controllers, characters, animation, and presentation remain consumer-owned. Product One’s first harness slice is an adjacent validation consumer, not a narrowing of substrate identity.

**Enabling implications** (depth and sequence are design, not a roadmap): optional higher intelligence or authoring clients on the same public surfaces; seams for multiplayer- or script-ready attachment without those systems being substrate features.

## Non-goals

- Shipping the actual game, its rules, or its content in this repository.
- Implementing System/LLM, spells, gas policy, combat, AI, or building game-layer systems here.
- Importing harness or demo specifics (controllers, routes, fixtures, hardware, benchmarks, machine thresholds) into substrate identity.
- Making substrate correctness or operation depend on an LLM.
- Treating milestone depth or first-slice exclusions of an adjacent consumer as a narrowing of substrate identity.

## Confirmed vision constraints

- **Form factor:** Rust crate or small family of tightly scoped Rust crates intended for game consumption.
- **GPU-resident substrate posture:** world matter the engine owns is designed to live GPU-resident as part of product identity.
- **Mandatory public consumer boundary:** in-repo validation consumers do not receive privileged implementation paths versus external games.
- **Mandatory higher-layer integration boundary:** higher layers mutate or read world truth only through command/query/event surfaces; pricing policy is consumer-supplied.
- **Excluded game layers stay unimplemented** here even when seams are designed for them.
- **No LLM dependency** for the substrate to operate.

## Deferred design decisions

- Exact crate split, APIs, encodings, and enforcement layout for public and internal boundaries.
- Capability depth and build order inside the substrate (storage, meshing, simulation tiers, resolution, streaming/persistence detail).
- How far multiplayer, scripting, or higher-layer seams are realized early.
- If a harness is delivered (Q1): acceptance thresholds, platforms, and fixture protocol remain adjacent design. Validation domains and proof role are already seed-described; only delivery obligation is open.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the already-described walkable-world validation slice a **required current repository delivery**, or only **permitted** as an adjacent consumer of the substrate?

- **What is already settled:** README identifies the harness’s validation domains (terrain generation, streaming, meshing, editing, collision, persistence, performance). `product-one-seed.md` defines its walkable generated-region proof, dig/place proof, substrate slice, and benchmark role. Those describe the adjacent artifact; they do not transfer controllers, demo content, presentation, or machine-specific gates into substrate identity.
- **Proposed safe answer:** Permitted only—the product under design is the substrate; that adjacent slice may exist but is not required for product completeness.
- **If answered differently:** Requiring the harness adds a repository delivery obligation and a duty that validation exercise public interfaces; it does not move controllers, demo content, presentation, or performance gates into substrate identity.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and identifies the walkable-world executable as a separate consumer/validation harness with stated validation domains rather than a game layer.
- **`docs/seeds/project-boundary.md`:** Locks current product identity to the substrate and Rust crate boundary; keeps the real game and higher game layers out of repo; permits a harness only if it shares external public interfaces.
- **`docs/seeds/product-one-seed.md`:** Describes an adjacent walkable demo/harness slice—walkable generated-region and dig/place proofs, substrate slice, and benchmark role—without transferring demo controls, content, or machine thresholds into substrate scope; first-slice exclusions apply only to that slice and raise Q1.
- **`docs/seeds/voxel-world-substrate.md`:** Authorizes substrate purpose and outcomes at product force—correct representations, universal mutability, deep Z, unified material behavior, construction-enabling semantics, mutation-safe continuous-3D traversal support, seed-reproducible geology with lazy residency and durable change, and the mandatory command/query/event boundary with consumer pricing—leaving mechanisms and sequencing to design.
