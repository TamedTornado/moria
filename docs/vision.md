# Project vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer material world for external consumers—not a game, not game rules, and not defined by any single demo scene.

**Product One** is the binding first delivery slice of that substrate: generation and matter depth selected for a walkable proof, plus a **walkable-world validation executable** that consumes the substrate only through the same public interfaces available to an external game. That executable is a required adjacent delivery for the first slice; it is not product identity and must not own privileged or game-specific world paths.

The actual game remains a separate downstream consumer outside this repository.

## Purpose

Give multiple future games one shared foundation: a natural-looking surface world over continuous, fully mutable voxel matter that stays honest underground. The substrate owns matter, geology-first generation, observation and mutation, material and ambient world behavior, derived spatial services, streaming, and persistence. Game rules, policy, controllers, content authorship, and presentation live above it.

Product One’s job is to make one claim undeniable under real sparsity and interaction pressure: **this is not a heightmap with props—it is a fully material world, and it looks good.** Dig and place on a debug path are the mutability proof, not gameplay.

## Boundary

**In product (substrate mandate)**

- Reusable GPU-resident voxel world: authoritative matter, geology-first generation with lazy/sparse residency, smooth terrain presentation as a regenerated view (never saved truth), public mutation and query surfaces, streaming, and persistence of world truth.
- Matter-coupled dressing and voxel-backed interactable objects; substrate-owned material and ambient behavior (fluids, fire/CA, integrity, granular settle, day/night and seasons, weather, fire ecology, voxel-object growth/lifecycle) as outcome families the crate exists to provide.
- Derived mutation-aware spatial support over continuous 3D (collision and navigation from voxel truth, not the render mesh); generic placement/stamp and POI metadata seams where generation and mutability require them.
- Public consumer contract: commands in; queries against a coarse, potentially stale mirror; events out. Nothing above the matter layer touches voxels directly. Zero LLM/System dependency.

**In repository for Product One (first slice; not identity)**

- The portions of generation and matter selected by the Product One seed: full generation foundation for a curated region; sparse matter with incremental smooth meshing; grass/clutter dressing and registered voxel objects (placement/render; no felling/rigid conversion or growth runtime); static water only; dig/place and mirror queries; streaming; seed-plus-deltas exact restore for that matter scope.
- A walkable third-person validation harness that proves continuous 3D traversal (including surface-to-underground), public-path dig/place, streaming, persistence of that slice, and performance under measurement—using public APIs only. Controllers, character, route, seed dressing, presentation, platforms, and numeric gates are harness-owned proof detail, not substrate identity.

**Out of product / repository**

- The actual game and all game-owned layers: rules, UX, combat, AI/agent labor, economy, System/LLM, spells, gas policy, and building *gameplay* (UI, work orders, mechanisms-as-game entities, semantic room/structure services).
- Privileged harness or in-repo paths that bypass the public crate surface.
- Transferring Product One’s narrower first-slice depth into a permanent limit on substrate outcome families.

Compatibility seams may be designed where substrate requirements demand them; forbidden layers must not be implemented here. A Cargo workspace boundary between reusable substrate and harness is required so the consumer split stays enforceable.

## Required product-level outcomes

Downstream design must make these true. Product One may ship narrower *depth* first; it does not demote these families from the product mandate.

1. **Reusable standalone Rust substrate** — External games and the in-repo harness integrate only through public interfaces, with no privileged path and no LLM/System dependency for the world layer to function.
2. **Natural mutable volume, deep Z** — Continuous, natural-looking terrain and dressing over authoritative voxel matter; destroy, alter, or place matter anywhere; geology, caves, and underground depth are first-class continuous-3D content from geology-first generation with lazy, sparse residency. Meshes and dressing are regenerated, synchronized views—never saved authority.
3. **World matter, physics, ambient behavior, and object lifecycle** — World matter and voxel-backed interactables behave as material systems (move, burn/wet, flow, settle, lose support, couple to dynamic proxies); dressing responds to matter state. Ambient natural-world behavior (day/night, seasons, weather, fire ecology) and growth-capable voxel-object lifecycle are substrate responsibilities. Fidelity and release order are design decisions; Product One may prove a thinner slice first.
4. **Consumer interaction and observability** — Consumers mutate and inspect only through public commands and queries; the substrate exposes a coarse, stale asynchronous mirror plus events.
5. **Mutation-aware spatial, traversal, and placement seams** — Spatial queries, mutation-safe navigation data, continuous-3D traversal support, and generic placement/stamp and POI seams—distinct from building gameplay, blueprints-as-gameplay, mechanisms, consumer semantic room services, work orders, economy, AI, controllers, and presentation.
6. **Persistence, streaming, and adjacent walkable proof** — Reproducible world truth combines generation with voxel edit deltas; separate journals cover moved or stateful substrate-owned objects/entities; delta scars support cross-run or cross-mode reuse. Streaming around activity keeps large worlds workable without loading raw voxels wholesale. Product One requires exact restoration of same-seed-plus-deltas matter for its slice and a public-API walkable harness proving continuous 3D traversal, dig/place mutability, streaming, and benchmark validation—without making harness controls, content, platforms, or numeric gates substrate scope. Portable GPU work stays on portable abstractions (wgpu/WGSL); no load-bearing native Metal fork.

## Non-goals

- Shipping an actual game, campaign, combat loop, progression, or multiplayer *product* in this repository.
- Implementing System/LLM, spells, gas policy, combat, AI agents, fortress labor/economy, or building-gameplay layers.
- Treating harness scenery, character, camera, curated route, debug keys, or machine-specific performance gates as product identity or product IP.
- Making the substrate depend on an LLM.
- Treating mesh, dressing, or other views as saved authority over voxel matter.
- Permanently excluding deferred matter, ambient, object-lifecycle, or full object-state persistence outcomes merely because Product One ships a thinner first slice.
- Native Metal (or other API) forks in load-bearing layers.

## Unresolved human questions

None that change product identity, purpose, or boundary. The seeds agree:

- **Identity** is the reusable substrate crate(s).
- **Product One** is the binding first implementation and validation slice, including a public-API-only walkable harness.
- **Architecture reference** material beyond that slice remains the long-horizon mandate and seam envelope, not current feature inventory or game content.

Residual engineering and design choices (voxel size, LOD, object-registry scale, multiplayer as a future product, sequence of deferred matter behaviors) do not re-open product identity and are left to design and measurement.

## Seed contribution account

| Source | Contribution |
| --- | --- |
| **`README.md`** | Names Moria as the GPU-resident Rust substrate; separates the walkable-world executable as consumer/validation harness, not a game layer. |
| **`docs/seeds/project-boundary.md`** | Binds product identity to reusable Rust crate(s); places the actual game outside the repository; permits a harness only as a non-privileged public-API consumer; excludes game/System/LLM/spell/gas/combat/AI/building layers; allows compatibility seams without implementing those layers. |
| **`docs/seeds/product-one-seed.md`** | Binds the first delivery slice: curated generated region under real sparsity; dig/place as mutability proof; generation full / matter partial (static water, no CA/flow/integrity/felling); seed-plus-deltas exact restore; walkable third-person harness as proof vehicle; portable wgpu/WGSL pressure. Concrete set dressing, material lists, milestone order, and machine numbers are harness/validation detail—capability pressure only, not product identity. |
| **`docs/seeds/voxel-world-substrate.md`** | Authorizes enduring substrate outcome families at engine altitude: normal look over voxel truth, universal mutability, deep Z, geology-first generation, fused voxel objects vs derived dressing, matter/physics/ambient/growth behavior, command/mirror/events contract, spatial/nav and placement/POI seams, persistence with object journals and cross-run reuse of delta scars, multi-game reuse, zero LLM dependency. Future modes (ARPG, fortress, System hooks) justify seams; their gameplay, content, and full implementation are not current scope. |
| **`docs/seeds/README.md`** | Manifest authority: Product One binds the first milestone and its harness; the substrate doc is architecture reference (only Product One-selected portions are required for that milestone); operator clarification that Moria is substrate-only. |

**Deferred past Product One, still in product mandate:** flowing fluids, fire/CA, structural integrity, granular settle, dynamic object/rigid coupling, thin ambient weather/seasons, voxel-object growth runtime, full object-state journals and cross-run multi-mode reuse beyond seed-plus-deltas matter.

**Omitted from repository scope on purpose:** fortress/ARPG/descent game designs; System-authored content loops; multiplayer as a shipped product; game-authored weather/growth rules and content (as opposed to substrate ambient behavior); treating harness scenery as game IP.

**Conflicts:** None among the seeds regarding which product is current. Product One narrows *when* substrate capabilities ship and *how* they are first proven; it does not redefine the product as a walkable game demo.
