# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for downstream games, not a game and not a gameplay product.

## Purpose

Moria exists so multiple future games can share one material world: a natural-looking continuous landscape whose truth is fully mutable voxels, with deep underground as first-class content. The substrate stands alone—matter, physics, queries, and mutation with no LLM or game-rule dependency—so consumers can build ARPG, fortress, descent, or sandbox experiences on the same stack.

## Product boundary

**This product owns:** the reusable voxel-world substrate and its public consumer surface—geology-first generation; GPU-resident matter; non-authoritative presentation derived from matter; a command-and-observation contract (commands in; stale/coarse mirrors and events out); matter-coupled interactive objects, fluids, granular and ambient behavior, and structural integrity; mutation-safe derived navigation; and persistence/streaming including object continuity.

**Adjacent required current delivery (not product identity):** an in-repo walkable-world validation harness is part of current delivery. It is the first public proof—generated walkable world, dig/place through public verbs, streaming/persistence, and benchmarks—while remaining a separate consumer. It must use only public interfaces available to an external game and must not own privileged substrate paths. Controller, camera, authored route, content inventory, presentation polish, detailed workloads, machine targets, and acceptance theater are harness-owned, not substrate identity.

**Not this product:** the actual game; game rules; System/LLM features; spells; gas policy; combat; AI; and building layers as game systems. Seams may be designed where substrate responsibilities demand them; those layers are not implemented here. The real game is a separate downstream consumer outside this repository’s product scope.

## Required product outcomes

Downstream design must make these consumer-visible guarantees true for the full substrate. First-slice depth and sequence are design choices; they do not shrink the product mandate.

1. **Natural world, voxel truth.** A continuous natural landscape (terrain, cover, water, cliffs, caves) that reads as an ordinary world while every interactable solid remains mutable voxel matter—not a heightmap with props. Surface dressing stays synchronized with that matter.
2. **Mutable continuous volume.** Through public commands, consumers dig, place, and alter matter anywhere; carved faces remain honest material; the mesh is a regenerated non-authoritative view. Deep underground, strata, voids, and buried materials share the same continuous 3D volume. Worlds arise from seedable geology-first generation and materialize cost only where touched.
3. **Dynamic matter and object lifecycle.** The substrate owns voxel-backed interactive objects; matter-synchronized dressing; active fluids beyond static bodies; granular response; fire/wetness and related ambient simulation; and material-dependent structural failure. Interactive objects stay coupled to voxel matter through mutation, motion and re-voxelization, streaming, and saved object state—substrate continuity of world objects, not game entities or content.
4. **Mutation-safe derived navigation.** Navigation is derived from voxel truth, invalidated by mutation, and supports continuous 3D traversal. Consumer AI, labor policy, and movement presentation stay outside the substrate.
5. **Shared command-and-observation surface.** Matter, physics, queries, and mutation go only through public commands, mirrors, and events—no privileged back doors. Dig/place and mirror observation exist from the first proof. Every consumer, including a future System client, uses the same commands, mirrors, policy-neutral verbs, and public authoring registries as hand-authored clients; pricing and game rules inject above the substrate.
6. **Scar-cheap continuity.** World truth is generation plus edit deltas (and object/state journals as provided), with activity-centered residency so untouched bulk stays cheap and heavily edited regions reload as the same material world.

**First adjacent proof (harness delivery, not identity):** the walkable-world harness must show a generated walkable region, public dig/place validation, streaming and persistence, and benchmarks on the public surface. Narrower first-proof matter depth (for example static water only) does not remove the full dynamic-matter mandate from the product.

## Future products and enabling implications

Future **consumers** (not this product) include a System-driven ARPG, a DF-style fortress/colony game, a Moria-style descent experience, and pure sandbox modes. They own gameplay, UX, controllers, content, economy, and policy.

Enabling implications already mandated at substrate altitude: gas or labor pricing as consumer policy on shared verbs; an LLM/System client that only observes mirrors/events and authors through the same public registries; fortress- or adventure-scale reuse of the same deltas and material world. Their gameplay, content, presentation, and acceptance scenarios stay out of this product.

## Non-goals

- Shipping a commercial game, ARPG, fortress sim, or descent roguelike here
- Implementing System/LLM, spells, gas, combat, AI, or building layers here
- Treating harness character, camera, debug UX, seeded postcard content, or benchmark theater as substrate features
- Making the substrate depend on an LLM
- Replacing the public command/mirror/event boundary with privileged in-repo validation paths

## Confirmed vision constraints

- **Rust crate delivery:** a Rust crate or small family of tightly scoped crates; the boundary between substrate and validation executable is mandatory (exact crate graph is design).
- **Public-only harness consumption:** the walkable-world harness uses only interfaces available to an external game.
- **GPU-resident command/observation contract:** consumers interact via commands in and stale/coarse mirrors plus events out—not by direct voxel ownership.
- **No LLM in the substrate:** the world layer functions with zero LLM dependency.
- **Game systems stay out:** game rules and future System, spell, gas, combat, AI, and building layers are not implemented here (seams only where needed).
- **Policy-neutral shared surface:** verbs and registries stay consumer-neutral so multiple games and authoring paths share one substrate without privileged System or in-repo paths.

## Deferred design decisions

- Sequence and depth of full dynamic-matter families relative to the first harness proof
- Crate split, concrete APIs, algorithms, data layouts, voxel scale, LOD, and meshing
- Streaming rings, persistence encoding, physics coupling, and navigation representation
- Harness controller, route, content, presentation, workloads, and measurement machines
- Performance budgets, platforms, and backends (unless later design adopts an explicit product promise)

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and identifies the walkable-world executable as a separate validation consumer, not a game layer.
- **`docs/seeds/project-boundary.md`:** Fixes product identity (Rust substrate crates), forbids game/System/building-layer implementation here, classifies the harness as adjacent, and requires public-only consumption.
- **`docs/seeds/voxel-world-substrate.md`:** Supplies the full substrate mandate—natural look over voxel truth, universal mutation, deep Z, geology-first lazy worlds, dynamic matter families, object lifecycle, derived navigation, command/mirror/event layering, consumer-neutral registries, and persistence/streaming.
- **`docs/seeds/product-one-seed.md`:** Pins the required first adjacent delivery (walkable proof, public dig/place, streaming/persistence, benchmarks) and a narrower first matter slice; controller, region content, milestones, and hardware gates remain harness detail and do not redefine the product or shrink full substrate outcomes.
