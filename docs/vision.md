# Project vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer material world for external consumers—not a game, not game rules, and not defined by any single demo scene.

**Product One** is the binding first delivery slice of that substrate: generation and matter depth selected for a walkable proof, plus a **walkable-world validation executable** that consumes the substrate only through the same public interfaces available to an external game. That executable is a required adjacent delivery for the first slice; it is not product identity and must not own privileged or game-specific world paths.

The actual game remains a separate downstream consumer outside this repository.

## Purpose

Give multiple future games one shared foundation: a natural-looking surface world over continuous, fully mutable voxel matter that stays honest underground. The substrate owns matter, geology-first generation, observation and mutation, streaming, and persistence for the world layer. Game rules, policy, controllers, content authorship, and presentation live above it.

Product One’s job is to make one claim undeniable under real sparsity and interaction pressure: **this is not a heightmap with props—it is a fully material world, and it looks good.** Dig and place on a debug path are the mutability proof, not gameplay.

Broader matter, ambient, and object-lifecycle behaviors described in the architecture reference (flowing fluids, fire/CA, integrity, granular settle, weather/seasons, growth, dynamic object coupling, and related seams) are **candidate / deferred directions** until human intent settles whether they are binding long-term product commitments or illustrative future architecture. They must not be treated as required product-level outcomes for this vision’s approval.

## Boundary

**In product (substrate mandate — binding now)**

- Reusable GPU-resident voxel world: authoritative matter, geology-first generation with lazy/sparse residency, smooth terrain presentation as a regenerated view (never saved truth), public mutation and query surfaces, streaming, and persistence of world truth for the substrate’s owned matter scope.
- Matter-coupled dressing and voxel-backed interactable objects at the depth Product One selects (placement, registration, rendering; not deferred lifecycle/physics coupling as requirements).
- Derived mutation-aware spatial support over continuous 3D for harness proof needs (collision against voxel truth, not the render mesh); generic placement/stamp and POI metadata seams where generation and mutability require them.
- Public consumer contract: commands in; queries against a coarse, potentially stale mirror; events out. Nothing above the matter layer touches voxels directly. Zero LLM/System dependency.

**In repository for Product One (first slice; not identity)**

- The portions of generation and matter selected by the Product One seed: full generation foundation for a curated region; sparse matter with incremental smooth meshing; grass/clutter dressing and registered voxel objects (placement/render; no felling/rigid conversion or growth runtime); static water only; dig/place and mirror queries; streaming; seed-plus-deltas exact restore for that matter scope.
- A walkable third-person validation harness that proves continuous 3D traversal (including surface-to-underground), public-path dig/place, streaming, persistence of that slice, and performance under measurement—using public APIs only. Controllers, character, route, seed dressing, presentation, platforms, and numeric gates are harness-owned proof detail, not substrate identity.

**Candidate / deferred (architecture reference; not binding outcomes until the open question is answered)**

- Extended matter and ambient families from the architecture reference: flowing fluids beyond static bodies, fire/CA, structural integrity, granular settle, day/night and seasons, weather, fire ecology, voxel-object growth/lifecycle and felling/rigid conversion, full object-state journals, and cross-run multi-mode reuse of delta scars beyond Product One’s seed-plus-deltas matter restore.
- Additional spatial, placement, and semantic seams sketched for future games (full mutation-safe nav graphs, mechanism entities, room/structure services, etc.) when not required by Product One’s public proof surface.

These remain visible as design context and format/API headroom where Product One already establishes seams (e.g. state nibble present but unused). They are not required product-level outcomes of this vision.

**Out of product / repository**

- The actual game and all game-owned layers: rules, UX, combat, AI/agent labor, economy, System/LLM, spells, gas policy, and building *gameplay* (UI, work orders, mechanisms-as-game entities, semantic room/structure services as game features).
- Privileged harness or in-repo paths that bypass the public crate surface.
- Importing future products’ gameplay, content, characters, assets, or full implementation from architecture examples into current scope.
- Treating Product One’s thinner first-slice depth as a permanent ban on later matter families *if* those families are later affirmed as product commitments—or, conversely, treating architecture-reference sketches as already-approved commitments.

Compatibility seams may be designed where substrate requirements demand them; forbidden layers must not be implemented here. A Cargo workspace boundary between reusable substrate and harness is required so the consumer split stays enforceable.

## Required product-level outcomes

Downstream design must make these true for the **binding** product and Product One slice. Architecture-reference capabilities past that slice are **not** listed here until human intent affirms them as commitments.

1. **Reusable standalone Rust substrate** — External games and the in-repo harness integrate only through public interfaces, with no privileged path and no LLM/System dependency for the world layer to function.
2. **Natural mutable volume, deep Z** — Continuous, natural-looking terrain and dressing over authoritative voxel matter; destroy, alter, or place matter anywhere within the owned interaction surface; geology, caves, and underground depth are first-class continuous-3D content from geology-first generation with lazy, sparse residency. Meshes and dressing are regenerated, synchronized views—never saved authority.
3. **Product One matter and dressing depth** — Sparse brick residency with incremental smooth meshing; grass/clutter dressing driven from matter; voxel objects placed, registered, and rendered; static water bodies; dig/place as the mutability proof. Deferred matter sims and object lifecycle are not required outcomes of this vision.
4. **Consumer interaction and observability** — Consumers mutate and inspect only through public commands and queries; the substrate exposes a coarse, stale asynchronous mirror plus events (at least to the degree dig/place and harness proof need).
5. **Traversal and generation seams for the walkable proof** — Collision and continuous-3D traversal against voxel truth; generation/POI and stamp seams exercised where Product One’s curated region requires them—distinct from building gameplay, consumer semantic room services, work orders, economy, AI, controllers, and presentation.
6. **Persistence, streaming, and adjacent walkable proof** — Reproducible world truth for Product One combines generation with voxel edit deltas and exact same-seed-plus-deltas restore for that matter scope. Streaming around activity keeps a large sparse region workable without loading raw voxels wholesale. A public-API walkable harness proves continuous 3D traversal, dig/place mutability, streaming, and benchmark validation—without making harness controls, content, platforms, or numeric gates substrate scope. Portable GPU work stays on portable abstractions (wgpu/WGSL); no load-bearing native Metal fork.

## Non-goals

- Shipping an actual game, campaign, combat loop, progression, or multiplayer *product* in this repository.
- Implementing System/LLM, spells, gas policy, combat, AI agents, fortress labor/economy, or building-gameplay layers.
- Treating harness scenery, character, camera, curated route, debug keys, or machine-specific performance gates as product identity or product IP.
- Making the substrate depend on an LLM.
- Treating mesh, dressing, or other views as saved authority over voxel matter.
- Treating architecture-reference deferred capabilities (flow, fire/CA, integrity, granular, ambient weather/seasons, growth, rigid felling, full object journals, multi-mode reclaim, etc.) as **required** product-level outcomes of this vision until the open human question is answered.
- Native Metal (or other API) forks in load-bearing layers.

## Unresolved human questions

1. **Binding vs illustrative deferred architecture** — Are Product One–deferred capabilities in `voxel-world-substrate.md` (and related architecture-reference material)—including flowing fluids, fire/CA, structural integrity, granular settle, ambient day/night/seasons/weather/fire ecology, voxel-object growth and felling/rigid conversion, full object-state journals, cross-run multi-mode reuse of deltas, and fuller nav/placement/semantic seams—**binding long-term product commitments** for Moria, or **illustrative future architecture** that may inform seams without obligating outcomes?

Until answered, those capabilities are **candidate / deferred directions** only. They must not be planned, staffed, or accepted as required product-level outcomes of this vision. Product One’s boundary, the substrate-vs-game identity, the public-API-only harness rule, and all other explicit human handoffs in the seeds remain as stated above.

Residual engineering choices inside the binding slice (voxel size, LOD, object-registry scale, sequence of optional stretch demos such as timber) do not re-open product identity and are left to design and measurement.

## Seed contribution account

| Source | Contribution |
| --- | --- |
| **`README.md`** | Names Moria as the GPU-resident Rust substrate; separates the walkable-world executable as consumer/validation harness, not a game layer. |
| **`docs/seeds/project-boundary.md`** | Binds product identity to reusable Rust crate(s); places the actual game outside the repository; permits a harness only as a non-privileged public-API consumer; excludes game/System/LLM/spell/gas/combat/AI/building layers; allows compatibility seams without implementing those layers. |
| **`docs/seeds/product-one-seed.md`** | Binds the first delivery slice: curated generated region under real sparsity; dig/place as mutability proof; generation full / matter partial (static water, no CA/flow/integrity/felling); seed-plus-deltas exact restore; walkable third-person harness as proof vehicle; portable wgpu/WGSL pressure. Concrete set dressing, material lists, milestone order, and machine numbers are harness/validation detail—capability pressure only, not product identity. |
| **`docs/seeds/voxel-world-substrate.md`** | Architecture reference for engine-layer outcome families: normal look over voxel truth, universal mutability, deep Z, geology-first generation, fused voxel objects vs derived dressing, matter/physics/ambient/growth sketches, command/mirror/events contract, spatial/nav and placement/POI seams, persistence with object journals and cross-run reuse, multi-game reuse, zero LLM dependency. **Only Product One–selected portions are required for the first milestone** (per seeds README). Material past that slice is candidate/deferred until the open human question is resolved—not imported as required outcomes or game content. |
| **`docs/seeds/README.md`** | Manifest authority: Product One binds the first milestone and its harness; the substrate doc is architecture reference (only Product One-selected portions are required for that milestone); operator clarification that Moria is substrate-only. |

**Candidate / deferred past Product One (not required outcomes until Q1 is answered):** flowing fluids, fire/CA, structural integrity, granular settle, dynamic object/rigid coupling, thin ambient weather/seasons, voxel-object growth runtime, full object-state journals and cross-run multi-mode reuse beyond seed-plus-deltas matter.

**Omitted from repository scope on purpose:** fortress/ARPG/descent game designs; System-authored content loops; multiplayer as a shipped product; game-authored weather/growth rules and content; treating harness scenery as game IP.

**Conflicts / intent gap:** The seeds agree on current product identity (reusable substrate), Product One as the binding first slice with a public-API harness, and that the architecture doc is reference for the milestone. They do **not** explicitly state whether deferred architecture-reference capabilities are long-term product commitments or illustrative envelope; that gap is the open human question above. No silent elevation of those capabilities into required outcomes.
