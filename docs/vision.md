# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. Downstream games and tools consume it as a library: the substrate owns world matter, physics-relevant matter behavior, ambient natural-world behavior, generation, mutation, derived spatial and traversal services, consumer observability, and persistence; it does not own a game.

## Purpose

Moria exists so multiple games—and adjacent validation tools—can share one material world foundation: a natural-looking continuous volume that is fully voxel-backed, mutable all the way down, and playable underground as first-class content. Game rules, presentation policy, controllers, and content authorship stay above the substrate so the same crate stack can underpin adventure, fortress-style, sandbox, or other products without embedding any one of them.

## Product boundary

**In product:** the reusable substrate and its public consumer-facing surface—commands in; queries, a coarse stale mirror, and events out; generation, matter, mutation, ambient behavior, derived spatial/nav/traversal and placement/POI seams, and persistence those responsibilities require. Adjacent consumers use the same public interfaces available to an external game; nothing game-specific or privileged may live inside the substrate path. Layers above matter do not touch voxels directly.

**Adjacent required delivery (not identity):** a walkable-world executable is a required current delivery in this repository. It is a validation-harness consumer, not a game layer and not product identity. It must use the same public interfaces available to an external game. Its fused proof slice (“Product One”): a curated generated natural region with third-person traversal, proving continuous 3D traversal, public-path dig/place mutability, seed-plus-deltas exact save/load for that slice, streaming, and benchmark validation—a natural-looking, continuously traversable voxel world whose mutability is directly demonstrated. That slice’s controllers, character, route, content, presentation, platforms, and numeric gates remain adjacent-consumer ownership.

**Out of repository / downstream:** the actual game and every game-owned layer—rules, UX, controllers, characters, authored routes and content, combat, AI, work orders, economy, building gameplay (including blueprints, mechanisms, and semantic room/structure services), and presentation—are separate products. They may motivate substrate outcomes; they do not transfer into current scope. Compatibility seams may be designed where substrate requirements demand them; those layers must not be implemented here.

## Required product outcomes

- **Reusable standalone Rust substrate.** Ship a GPU-resident voxel-world library (crate or small crate family) that external games and tools integrate only through public interfaces—no privileged in-repo path—and that functions with zero LLM/System dependency.
- **Natural mutable volume, deep Z.** Continuous, natural-looking terrain and dressing over authoritative voxel matter (meshes and dressing are regenerated, synchronized views, never saved truth); destroy, alter, or place matter anywhere; geology, caves, and underground depth are first-class content from geology-first generation with lazy, sparse residency.
- **World matter, physics, ambient behavior, and object lifecycle.** World matter and voxel-backed interactive objects visibly behave as material systems—they move, burn/wet, flow, settle, lose support, and interact; dressing responds to matter state. Ambient natural-world behavior (day/night, seasons, weather, fire ecology) and growth-capable voxel-object lifecycle are substrate responsibilities. Fidelity and release order are design decisions; an adjacent first proof may ship narrower depth without demoting these families.
- **Consumer interaction and observability.** Consumers mutate and inspect only through public commands and queries; the substrate exposes a coarse, stale asynchronous mirror plus events. Nothing above the matter layer touches voxels directly.
- **Mutation-aware spatial, traversal, and placement seams.** Spatial queries, mutation-safe navigation data, continuous-3D traversal support, and generic placement/stamp and POI seams where generation and mutability require them—distinct from building gameplay, blueprints, mechanisms, consumer semantic room/structure services, work orders, economy, AI, controllers, and presentation.
- **Persistence, streaming, and adjacent walkable proof.** Reproducible world truth combines generation with voxel edit deltas; separate journals cover moved or stateful substrate-owned objects/entities; delta scars support cross-run or cross-mode reuse. Streaming around activity keeps large worlds workable without loading raw voxels wholesale. A required adjacent walkable-world harness proves continuous 3D traversal, dig/place mutability, exact restoration of that slice’s same-seed-plus-deltas save, streaming, and benchmark validation—through public interfaces only, without making its controls, content, platforms, or numeric gates substrate scope.

## Future products and enabling implications

Future consumers include a System-driven ARPG; fortress/colony-style play; descent/adventure modes; and pure sandbox tools. The walkable-world validation executable is an adjacent required delivery (above), not a future product. Those products own gameplay, UX, controllers, content, and policy.

Enabling implications (not a delivery roadmap): the same public matter, mutation, observability, derived-service, and persistence seams let those games inject pricing policy, author materials and placements, and reuse world delta scars across modes without forking the world engine.

## Non-goals

- Implementing the actual game, game rules, or game-layer systems (System/LLM, spells, gas policy, combat, AI, building gameplay, blueprints, mechanisms, semantic room/structure services, work orders, economy).
- Owning player controllers, cameras, characters, demo routes, authored set-pieces, or marketing-facing presentation as product identity.
- Privileged harness or in-repo paths that bypass the public crate surface.
- Treating mesh, dressing, or other views as saved authority over voxel matter.
- Transferring an adjacent first slice’s narrower behavioral depth into a permanent limit on substrate outcome families.
- Mandating exact restoration for every journal, or cross-mode reuse of object/entity state, beyond seed authority.

## Confirmed vision constraints

- Product identity is the reusable GPU-resident voxel-world substrate as Rust crate(s), not a shipped game.
- The walkable-world executable is a required current adjacent delivery and remains a public-interface validation harness, not product identity.
- The consumer boundary is mandatory: validation and games consume the same public interfaces; layers above matter do not touch voxels directly.
- Compatibility seams may be designed where substrate requirements demand them; forbidden layers must not be implemented here.
- The substrate must not depend on an LLM or System to function.
- Views (mesh, dressing) remain derived and synchronized with matter; they are not authoritative saved truth.

## Deferred design decisions

- Crate split, workspace layout, and API shape beyond the public-consumer and observability contract.
- Voxel resolution, LOD, meshing strategy, storage encodings, streaming-ring policy, and simulation fidelity/tiers per release.
- Delivery depth and order within authorized matter/physics, ambient, object-lifecycle, and derived-service outcome families.
- Whether multiplayer is pursued later; not a current product commitment.
- Validation harness content, controls, platforms, workloads, and numeric acceptance gates (harness is required; those details stay adjacent).

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident Rust substrate and separates the walkable-world executable as consumer/harness, not game.
- **docs/seeds/project-boundary.md** — Binds current identity to reusable Rust crate(s), mandates the public-interface consumer boundary, permits an in-repo harness only as a non-privileged consumer, and excludes game and forbidden layers (including future building layers except required compatibility seams).
- **docs/seeds/product-one-seed.md** — Pins the required first adjacent walkable-world delivery (curated region, third-person traversal, dig/place proof, seed-plus-deltas exact save/load, streaming, benchmarks) and narrows first-slice depth without demoting broader substrate outcome families.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families at engine altitude: natural voxel-truth world, mutability, deep Z, matter/physics behavior, ambient day/night–seasons–weather–fire ecology, voxel-object growth lifecycle, commands/mirror/events contract, spatial/nav and placement/POI seams (not consumer building/semantic-structure layers), persistence as generation plus edit deltas with object/entity journals and cross-run reuse of delta scars, multi-game reuse, zero LLM dependency.
