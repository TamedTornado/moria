# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for downstream games, not a game and not a gameplay product.

## Purpose

Moria exists so multiple future games can share one material world: a natural-looking continuous landscape whose truth is fully mutable voxels, with deep underground as first-class content. The substrate stands alone—matter, physics, queries, and mutation with no LLM or game-rule dependency.

## Product boundary

**This product owns:** the reusable voxel-world substrate and its public consumer surface—geology-first generation; GPU-resident matter; non-authoritative presentation derived from matter; a command-and-observation contract (commands in; stale/coarse mirrors and events out); matter-coupled interactive objects, fluids, granular and ambient environmental behavior, structural integrity, and object growth; mutation-safe derived navigation; shared authoring registries for placement, materials/palettes, structures/stamps, and rules; and persistence/streaming including object continuity.

**Adjacent required current delivery (not product identity):** an in-repo walkable-world validation harness is part of current delivery. It is the first public proof through public interfaces only—not a game layer and not substrate identity. Controller, camera, route, content, presentation polish, workloads, machine targets, and acceptance theater remain harness-owned.

**Not this product:** the actual game; game rules; System/LLM features; spells; gas policy; combat; AI; and building layers as game systems. Seams may be designed where substrate responsibilities demand them; those layers are not implemented here.

## Required product outcomes

Downstream design must make these consumer-visible guarantees true for the full substrate. First-slice depth and sequence are design choices; they do not shrink the product mandate.

1. **Natural world, voxel truth.** A continuous natural landscape (terrain, cover, water, cliffs, caves) that reads as an ordinary world while every interactable solid remains mutable voxel matter—not a heightmap with props. Surface dressing stays synchronized with that matter. Presentation is a regenerated non-authoritative view of voxel truth.
2. **Mutable continuous volume.** Through public commands, consumers dig, place, and alter matter anywhere; carved faces remain honest material. Deep underground, strata, voids, and buried materials share the same continuous 3D volume. Worlds arise from seedable geology-first generation and materialize cost only where touched.
3. **Dynamic matter, growth, and environment.** The substrate owns voxel-backed interactive objects; matter-synchronized dressing; active fluids beyond static bodies; granular response; material-dependent structural failure; and ambient environmental behavior. Voxel objects grow over game time. Day/night, seasons, and weather affect growth, water, wetness, snow, and fire. Objects stay coupled to voxel matter through mutation, motion and re-voxelization, streaming, and saved object state—not game entities or content.
4. **Mutation-safe derived navigation.** Navigation is derived from voxel truth, invalidated by mutation, and supports continuous 3D traversal. Consumer AI, labor policy, and movement presentation stay outside the substrate.
5. **Shared command, observation, and authoring surface.** Matter, physics, queries, and mutation go only through public commands, mirrors, and events—no privileged back doors. Dig/place and mirror observation exist from the first proof. Every consumer—including an optional future System client—authors placement, palettes/materials, structures/stamps, and rules through the same public registries and policy-neutral verbs as hand authors; pricing and game rules inject above the substrate.
6. **Scar-cheap continuity.** World truth is generation plus edit deltas (and object/state journals as provided), with activity-centered residency so untouched bulk stays cheap and heavily edited regions reload as the same material world.

**First adjacent proof (harness delivery, not identity):** the walkable-world harness must demonstrate a traversable, continuous surface-to-underground natural world whose smooth presentation is demonstrably derived from voxel truth and whose collision follows voxel truth; public dig/place validation; streaming and persistence; and benchmarks on the public surface. Narrower first-proof matter depth (static water only, fixed time-of-day, no growth yet) does not remove full dynamic-matter, growth, or environmental mandates from the product.

## Future products and enabling implications

Future **consumers** (not this product) include a System-driven ARPG, a DF-style fortress/colony game, a Moria-style descent experience, and pure sandbox modes. They own gameplay, UX, controllers, content, economy, and policy.

Enabling implications already mandated at substrate altitude: gas or labor pricing as consumer policy on shared verbs; an LLM/System client that only observes mirrors/events and authors through the same public registries; fortress- or adventure-scale reuse of the same deltas and material world.

## Non-goals

- Shipping a commercial game, ARPG, fortress sim, or descent roguelike here
- Implementing System/LLM, spells, gas, combat, AI, or building layers here
- Treating harness character, camera, debug UX, seeded content, or benchmark theater as substrate features
- Making the substrate depend on an LLM
- Replacing the public command/mirror/event boundary with privileged in-repo validation paths
- Shipping multiplayer networking or multiplayer gameplay as part of this product

## Confirmed vision constraints

- **Rust crate delivery:** a Rust crate or small family of tightly scoped crates; the boundary between substrate and validation executable is mandatory (exact crate graph is design).
- **Public-only harness consumption:** the walkable-world harness uses only interfaces available to an external game.
- **GPU-resident command/observation contract:** consumers interact via commands in and stale/coarse mirrors plus events out—not by direct voxel ownership.
- **No LLM in the substrate:** the world layer functions with zero LLM dependency.
- **Game systems stay out:** game rules and future System, spell, gas, combat, AI, and building layers are not implemented here (seams only where needed).
- **Policy-neutral shared surface:** verbs and registries stay consumer-neutral so multiple games and authoring paths share one substrate without privileged System or in-repo paths.

## Deferred design decisions

- Sequence and depth of full dynamic-matter, growth, and ambient families relative to the first harness proof
- Crate split, concrete APIs, algorithms, data layouts, voxel scale, LOD, and meshing
- Streaming, persistence encoding, physics coupling, and navigation representation
- Harness controller, route, content, presentation, workloads, and measurement machines
- Performance budgets, platforms, and backends (unless later design adopts an explicit product promise)
- Whether and how multiplayer readiness is expressed beyond the command architecture (see Q1)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1. Multiplayer-readiness as current product scope.** The seeds establish that the public command architecture is server-authoritative-ready by construction, and leave open whether that readiness is a current scope promise. Is promising multiplayer-readiness of the command/observation contract (without implementing multiplayer) part of current product scope?

- **Proposed safe answer:** Yes—the shared command/observation surface must remain server-authoritative-ready; multiplayer networking and gameplay stay out of scope.
- **If answered no:** multiplayer-readiness is only incidental to the command design, not a current-scope quality outcome.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and the walkable-world executable as a separate validation consumer, not a game layer.
- **`docs/seeds/project-boundary.md`:** Fixes product identity (Rust substrate crates), forbids game/System/building-layer implementation here, classifies the harness as adjacent, and requires public-only consumption.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies the full substrate mandate—natural look over voxel truth, universal mutation, deep Z, geology-first lazy worlds, dynamic matter and object growth, ambient day/night/season/weather effects, shared authoring registries, derived navigation, command/mirror/event layering, and persistence/streaming; leaves multiplayer-readiness open.
- **`docs/seeds/product-one-seed.md`:** Pins the required first adjacent proof (continuous surface-to-underground walkable world, voxel-truth presentation and collision, public dig/place, streaming/persistence, benchmarks) and a narrower first matter slice that does not shrink full substrate outcomes.
