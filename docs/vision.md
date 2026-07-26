# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. Downstream games and tools consume it as a library: the substrate owns world matter, generation, mutation, queries, and related engine outcomes; it does not own a game.

## Purpose

Moria exists so multiple games—and adjacent validation tools—can share one material world foundation: a natural-looking continuous volume that is fully voxel-backed, mutable all the way down, and playable underground as first-class content. Game rules, presentation policy, and content authorship stay above the substrate so the same crate stack can underpin adventure, fortress-style, sandbox, or other products without embedding any one of them.

## Product boundary

**In product:** the reusable substrate and its public consumer-facing surface (crate APIs for generation, matter, mutation, queries, and whatever seams those responsibilities require). Adjacent consumers must use the same public interfaces available to an external game; nothing game-specific or privileged may live inside the substrate path.

**Adjacent, not identity:** a walkable-world executable may exist in this repository as a validation harness for generation, streaming, meshing, editing, collision, persistence, and performance. It is not a game layer and does not redefine the product. Whether that harness is a required current delivery is open (see Q1).

**Out of repository / downstream:** the actual game and every game-owned layer—rules, UX, controllers, characters, authored routes and content, combat, AI, economy, and presentation—are separate products. They may motivate substrate outcomes; they do not transfer into current scope.

## Required product outcomes

- **Reusable Rust substrate.** Ship a GPU-resident voxel-world library (crate or small crate family) that external games and tools integrate only through public interfaces—no privileged in-repo path.
- **Natural material world.** Continuous, natural-looking terrain and dressing over authoritative voxel matter; meshes and surface dressing are regenerated views, never saved truth.
- **Mutable volume, deep Z.** Destroy, alter, or place matter anywhere; geology, caves, and underground depth are first-class content, produced by geology-first generation with lazy, sparse residency for large regions.
- **Public verbs, queries, and matter behavior.** Consumers dig, place, and inspect only through public APIs; the substrate owns engine-level matter behavior (interactive voxel-backed objects where interaction matters, matter-driven dressing, and physical foundations such as fluids, support, and granular materials) without game pricing or policy. Depth and sequence of those behaviors are design decisions.
- **Persistence and streaming.** World truth is generation plus edit deltas, with streaming around activity so large worlds stay workable without loading raw voxels wholesale.
- **Standalone engine layer.** Zero LLM/System dependency; later systems may attach but must not be required for the substrate to function.

## Future products and enabling implications

Future or adjacent consumers include a walkable-world validation/demo executable; a System-driven ARPG; fortress/colony-style play; descent/adventure modes; and pure sandbox tools. Those products own gameplay, UX, controllers, content, and policy.

Enabling implications (not a delivery roadmap): the same public matter, mutation, and query seams let those games inject pricing policy, author materials and placements, and reuse edit deltas across modes without forking the world engine.

## Non-goals

- Implementing the actual game, game rules, or game-layer systems (System/LLM, spells, gas policy, combat, AI, building gameplay layers).
- Owning player controllers, cameras, characters, demo routes, authored set-pieces, or marketing-facing presentation as product identity.
- Privileged harness or in-repo paths that bypass the public crate surface.
- Treating mesh, dressing, or other views as saved authority over voxel matter.

## Confirmed vision constraints

- Product identity is the reusable GPU-resident voxel-world substrate as Rust crate(s), not a shipped game.
- The consumer boundary is mandatory: validation and games consume the same public interfaces.
- Compatibility seams may be designed where substrate requirements demand them; forbidden layers must not be implemented here.
- The substrate must not depend on an LLM or System to function.

## Deferred design decisions

- Crate split, workspace layout, and API shape beyond the public-consumer rule.
- Voxel resolution, LOD, meshing strategy choices, storage encodings, and streaming-ring policy details.
- How deep matter simulation (fluids, structural integrity, granular settle, fire/CA, object felling) is delivered in each release slice—and in what order.
- Whether multiplayer is pursued later; not a current product commitment.
- Validation harness content, controls, platforms, workloads, and numeric acceptance gates (if the harness is delivered).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation executable a **required current delivery** alongside the substrate, or only a **permitted adjacent artifact** that may live in-repo when useful?

*Proposed safe answer:* Permitted adjacent artifact only—not part of product identity; if later required, it still must consume only public substrate interfaces and must not become a game layer.

*If answered differently:* Making it mandatory current delivery expands repository commitments to ship and maintain a harness product without changing substrate identity; treating it as forbidden would remove the in-repo validation path the seeds describe as allowed.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident Rust substrate and separates the walkable-world executable as consumer/harness, not game.
- **docs/seeds/project-boundary.md** — Binds current identity to reusable Rust crate(s), mandates the public-interface consumer boundary, and excludes game and forbidden layers from this repository.
- **docs/seeds/product-one-seed.md** — Describes an adjacent walkable-world demo slice (region, controller, proof of mutability, harness-style validation) that motivates substrate outcomes without redefining product identity.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate design goals and outcome families (natural voxel-truth world, mutability, deep Z, generation, matter/view split, verbs/queries, persistence/streaming, multi-game reuse, zero LLM dependency) at engine altitude.
