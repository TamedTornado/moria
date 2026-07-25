# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate or a small family of tightly scoped crates. It is the world-matter foundation other products consume: natural-looking, continuous 3D material worlds that stay fully mutable from surface through deep underground, exposed only through public generation, query, and mutation interfaces.

## Purpose

Game and sandbox products need a shared world layer that reads as ordinary terrain yet treats every visible volume as real, diggable, placeable matter—not a heightmap with props. Moria exists so that foundation is built once, stays free of game rules and LLM dependency, and can be validated and consumed without privileged in-repo paths.

## Product boundary

**This product owns** the substrate library surface: reusable world substrate behavior and the public interfaces external consumers use to generate, stream, inspect, present, and mutate material voxel worlds.

**This product does not own** the actual game, game rules, or consumer presentation. An optional in-repo walkable-world executable is only a **validation harness**—an adjacent consumer that must use the same public interfaces available to an external game. Harness-specific controllers, characters, cameras, demo routes, authored seed content, marketing clips, and numeric performance gates are not product scope.

**Repository vs product:** the repo may hold substrate crates and a harness behind a Cargo workspace split; the precise crate graph is design, but the consumer boundary is not optional.

## Future products and enabling implications

Described future consumers—not current Moria—include a System/LLM ARPG, a fortress/colony builder, a deep-descent adventure mode, and pure sandboxes. They remain separate products.

Enabling implications for the substrate (compatibility posture, not a committed roadmap of modules): keep deep continuous-Z material worlds first-class; keep mutation and query as the only path to voxels; leave seams for later consumer-owned physics, fluids, integrity, building, and agent policies without implementing those game layers here.

## Non-goals

- Game rules; System/LLM, spells, gas policy, combat, AI, economy, or building gameplay layers
- Harness- or demo-owned content, controls, characters, routes, or acceptance workloads as if they were substrate features
- Privileged implementation paths the external game cannot use

## Confirmed vision constraints

- Substrate stands alone with **zero LLM dependency**
- Validation harness, if present, consumes **only public interfaces**; reusable substrate and harness stay separated at the workspace/consumer boundary
- Product identity is the **substrate**, not a game or game-shaped demo slice

## Assumptions proposed for approval

**A1.** “Reads as a normal world” (smooth natural terrain presentation derived from voxel truth, not a cube aesthetic as the primary look) is a substrate responsibility at vision altitude, so consumers are not forced to re-solve that tension for ordinary terrain.

**A2.** Matter mutation and inspection (including dig/place-class verbs and queries) belong on the substrate public surface as world-foundation capabilities; game-facing building UI, blueprints, mechanisms, and work orders remain out of product and with future consumers.

## Questions for human review

_(None. Explicit project-boundary seeds fix current product identity as the reusable substrate; the walkable world is harness/consumer, not a second competing product. Capability depth, slice order, platforms, and acceptance numbers are downstream design.)_

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world substrate (Rust crate) for the actual game; frames the walkable-world executable as separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binding boundary—product is the substrate crate(s); game is out of repo; harness only via public APIs; workspace split required; System/LLM/spell/gas/combat/AI/building layers out of scope (seams only).
- **`docs/seeds/product-one-seed.md`:** Future/harness-shaped proof narrative (walkable region, character run, dig as mutability proof, demo milestones). Contributed the “material world, not heightmap with props” claim and that dig/place is proof of substrate honesty; controller, seed inventory, metrics, and milestone plan treated as harness/design detail, not current product scope.
- **`docs/seeds/voxel-world-substrate.md`:** Long-horizon substrate intent—natural look over full mutability, deep Z, substrate-not-game layering, multi-game reuse, GPU-resident posture, standalone without LLM. Design mechanisms, tiers, and build-order catalogs deferred; multi-consumer enabling implications retained at high level only.
