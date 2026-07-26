# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. Downstream games and tools consume it as a library: the substrate owns world matter, physics-relevant matter behavior, generation, mutation, derived world services, consumer observability, and persistence; it does not own a game.

## Purpose

Moria exists so multiple games—and adjacent validation tools—can share one material world foundation: a natural-looking continuous volume that is fully voxel-backed, mutable all the way down, and playable underground as first-class content. Game rules, presentation policy, controllers, and content authorship stay above the substrate so the same crate stack can underpin adventure, fortress-style, sandbox, or other products without embedding any one of them.

## Product boundary

**In product:** the reusable substrate and its public consumer-facing surface—commands in; queries, a coarse stale mirror, and events out; generation, matter, mutation, derived structure/spatial/nav services, and persistence seams those responsibilities require. Adjacent consumers use the same public interfaces available to an external game; nothing game-specific or privileged may live inside the substrate path. Layers above matter do not touch voxels directly.

**Adjacent, not identity:** a walkable-world executable may exist in this repository as a validation harness. It is not a game layer and does not redefine the product. Whether that harness is a required current delivery is open (see Q1). Independently of Q1, the seeds specify a **first adjacent delivery slice** (“Product One” / walkable world): a curated generated natural region traversed by a third-person character, proving continuous 3D traversal, public-path dig/place mutability, persistence, streaming, and benchmark validation—a natural-looking, continuously traversable voxel world whose mutability is directly demonstrated. That slice’s controllers, character, route, content, presentation, platform, and numeric gates remain adjacent-consumer ownership.

**Out of repository / downstream:** the actual game and every game-owned layer—rules, UX, controllers, characters, authored routes and content, combat, AI, work orders, economy, building gameplay, and presentation—are separate products. They may motivate substrate outcomes; they do not transfer into current scope.

## Required product outcomes

- **Reusable standalone Rust substrate.** Ship a GPU-resident voxel-world library (crate or small crate family) that external games and tools integrate only through public interfaces—no privileged in-repo path—and that functions with zero LLM/System dependency.
- **Natural mutable volume, deep Z.** Continuous, natural-looking terrain and dressing over authoritative voxel matter (meshes and dressing are regenerated, synchronized views, never saved truth); destroy, alter, or place matter anywhere; geology, caves, and underground depth are first-class content from geology-first generation with lazy, sparse residency.
- **World matter and physics behavior.** World matter and voxel-backed interactive objects visibly behave: they move, burn/wet, flow, settle, lose support, and interact as material systems; dressing responds to matter state. Fidelity, algorithms, tiers, and release order are design decisions; the outcome family is not optional merely because an adjacent first demo may ship a narrower proof.
- **Consumer interaction and observability.** Consumers mutate and inspect only through public commands and queries; the substrate exposes a coarse, stale asynchronous mirror plus events. Nothing above the matter layer touches voxels directly.
- **Mutation-aware derived world services.** Reusable, game-neutral structure and stamp semantics, spatial queries, mutation-safe navigation data, and continuous-3D traversal support—distinct from building gameplay, work orders, economy, AI, controllers, and presentation.
- **Persistence and streaming.** Reproducible truth combines generation, voxel edit deltas, and journals for moved or stateful substrate-owned objects/entities, supporting exact restoration and cross-run or cross-mode reuse of scars and object state. Streaming around activity keeps large worlds workable without loading raw voxels wholesale.

## Future products and enabling implications

Future or adjacent consumers include the walkable-world validation/demo executable (first specified adjacent slice above); a System-driven ARPG; fortress/colony-style play; descent/adventure modes; and pure sandbox tools. Those products own gameplay, UX, controllers, content, and policy.

Enabling implications (not a delivery roadmap): the same public matter, mutation, observability, derived-service, and persistence seams let those games inject pricing policy, author materials and placements, and reuse world scars and object state across modes without forking the world engine.

## Non-goals

- Implementing the actual game, game rules, or game-layer systems (System/LLM, spells, gas policy, combat, AI, building gameplay, work orders, economy).
- Owning player controllers, cameras, characters, demo routes, authored set-pieces, or marketing-facing presentation as product identity.
- Privileged harness or in-repo paths that bypass the public crate surface.
- Treating mesh, dressing, or other views as saved authority over voxel matter.
- Transferring an adjacent first slice’s narrower behavioral depth into a permanent limit on substrate outcome families.

## Confirmed vision constraints

- Product identity is the reusable GPU-resident voxel-world substrate as Rust crate(s), not a shipped game.
- The consumer boundary is mandatory: validation and games consume the same public interfaces; layers above matter do not touch voxels directly.
- Compatibility seams may be designed where substrate requirements demand them; forbidden layers must not be implemented here.
- The substrate must not depend on an LLM or System to function.
- Views (mesh, dressing) remain derived and synchronized with matter; they are not authoritative saved truth.

## Deferred design decisions

- Crate split, workspace layout, and API shape beyond the public-consumer and observability contract.
- Voxel resolution, LOD, meshing strategy, storage encodings, streaming-ring policy, and simulation fidelity/tiers per release.
- Delivery depth and order within authorized matter/physics and derived-service outcome families.
- Whether multiplayer is pursued later; not a current product commitment.
- Validation harness content, controls, platforms, workloads, and numeric acceptance gates (if the harness is delivered).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation executable a **required current delivery** alongside the substrate, or only a **permitted adjacent artifact** that may live in-repo when useful?

*Proposed safe answer:* Permitted adjacent artifact only—not part of product identity; if later required, it still must consume only public substrate interfaces and must not become a game layer. The first adjacent slice specification remains the intended shape of that harness when it is delivered.

*If answered differently:* Making it mandatory current delivery expands repository commitments to ship and maintain a harness product without changing substrate identity; treating it as forbidden would remove the in-repo validation path the seeds describe as allowed.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident Rust substrate and separates the walkable-world executable as consumer/harness, not game.
- **docs/seeds/project-boundary.md** — Binds current identity to reusable Rust crate(s), mandates the public-interface consumer boundary, and excludes game and forbidden layers from this repository.
- **docs/seeds/product-one-seed.md** — Specifies the first adjacent walkable-world demo slice (curated region, third-person traversal, dig/place proof, persistence, streaming, benchmarks) and narrows first-slice depth without demoting broader substrate outcome families.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families at engine altitude: natural voxel-truth world, mutability, deep Z, matter/physics behavior, commands/mirror/events contract, derived structure/nav services, persistence including object journals and cross-run reuse, multi-game reuse, zero LLM dependency.
