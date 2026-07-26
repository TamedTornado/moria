# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation: material truth, generation, mutation, queries, and real-time-friendly residency for external games—not a game, demo, or content product.

## Purpose

Games that need a fully material, diggable, deep, natural-looking world should build on one shared substrate instead of reimplementing voxel matter, geology, and mutation for each title. Moria exists so downstream games (sandbox, fortress, descent, or ARPG) can consume the same public world capabilities while keeping rules, presentation, and content in the consumer.

## Product boundary

**In product (Moria):** the reusable substrate and its public Rust integration surface. The substrate owns high-level responsibility for voxel matter as world truth, geology-aware generation, mutation and query of that matter, view-derived presentation of terrain (mesh is never authoritative), deep continuous volume (surface through underground), and the residency patterns that make large worlds practical (streaming and edit-delta persistence at product altitude).

**Adjacent, not identity:** a walkable-world executable may exist only as a validation harness. It must use the same public interfaces an external game would use—no privileged or game-specific substrate paths. Its character, camera, controls, authored demo route, content checklist, presentation, platform choices, and performance gates are harness-owned, not product features.

**Out of repository / product:** the actual game and all game layers—rules, System/LLM, spells, gas, combat, AI, and building-as-gameplay. Compatibility seams may be planned where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

- **Reusable public integration:** consumers integrate Moria as Rust crate(s) through public APIs only; any adjacent validation uses that same surface, with zero LLM/System dependency inside the substrate.
- **Material world truth:** the world is fully material and mutable end-to-end—any region of volume can be destroyed, altered, or filled; nothing important is decorative geometry outside matter.
- **Natural read over voxel truth:** seedable, geology-aware generation yields continuous natural terrain (surface through caves) while remaining voxel-backed; meshed or dressed presentation is a regenerated view, never save authority.
- **Deep continuous volume:** underground depth is first-class playable volume—strata, caves, descent—not a flat floor under a heightmap skin.
- **Live mutation with coherent views:** dig/place (and equivalent matter mutation) update world truth and drive remeshed/dressed views so cuts and fills stay honest; collision and interaction can rely on matter, not the render mesh.
- **Scale-friendly residency:** large regions stay practical via sparse residency, streaming around activity, and persistence of generation-plus-edit-deltas rather than full raw volumes.

## Future products and enabling implications

Future **consumers** (not current Moria scope) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandboxes. They motivate—without transferring gameplay, UX, content, or policy into this product—that the substrate remain rule-agnostic, queryable, and mutable, and that later matter behaviors (richer fluids, fire/ecology, structural integrity, granular settle, interactable voxel objects, priced verb policies) can attach without rewriting world truth. Those behaviors are enabling implications or later substrate depth, not a committed current feature inventory.

## Non-goals

- Shipping a game, combat loop, stats, AI, or non-player entity ecosystems as Moria itself.
- System/LLM authorship, spells, gas/pricing policy, or intent layers inside the substrate.
- Building-as-gameplay (blueprints, work orders, mechanisms UI, fortress designation policy).
- Owning consumer presentation, controllers, cameras, characters, authored levels, or marketing demo content as product requirements.
- Treating validation-harness benchmarks, device models, or graphics-backend preferences as the product’s portable identity.

## Confirmed vision constraints

- **Rust crate ecosystem:** the integration form is a Rust crate or small family of crates—not an ecosystem-neutral binary-only engine with no library surface.
- **GPU-resident substrate:** world matter and the hot path that keeps it interactive are intended to live as a GPU-resident design, not a CPU-only toy voxel grid.
- **Public-API consumer boundary:** harness and games share the public surface; no privileged in-tree game path.
- **Zero LLM dependency:** the substrate stands alone; System/LLM is optional game-layer client later.
- **Explicit exclusions:** game rules and System, spell, gas, combat, AI, and building layers stay out of this product.

## Deferred design decisions

- Precise crate split, package layout, and workspace mechanics (boundary intent is fixed; structure is not).
- Voxel resolution, brick/storage layouts, meshing algorithms, and dressing pipelines.
- Which matter subsystems (fluids beyond static bodies, fire, integrity, granular, rigid object coupling) ship in the first design slice versus later.
- Streaming ring policy, persistence encoding, and multiplayer/server authority readiness.
- Harness-only choices: demo region content, controller feel, cameras, platforms, and numeric performance gates.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required current delivery** adjacent to the substrate, or only a **permitted** validation consumer that may be omitted from the first ship set?

- **Proposed safe answer:** Required as an adjacent validation delivery (it must exist and exercise the public substrate APIs) but remains outside product identity; its controller, character, demo route, content set, presentation, and performance gates stay harness-owned and do not expand Moria’s scope.
- **If different:** If only permitted, design need not schedule a walkable executable as a delivery; if required with harness details treated as product scope, the brief would wrongly absorb demo UX, content, and acceptance into Moria itself.

## Seed synthesis

- **`README.md`:** Names Moria as the GPU-resident voxel-world Rust substrate and states the walkable executable is a separate validation consumer, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binding product and repository boundary—crate substrate in-scope; game layers out; harness must use public APIs only.
- **`docs/seeds/product-one-seed.md`:** First-slice and harness motivation—natural generated region, dig/place proof, matter-backed traversal—without transferring demo controls, content, or device gates into product identity.
- **`docs/seeds/voxel-world-substrate.md`:** Long-horizon substrate intent—material mutable world, deep Z, natural read over voxel truth, standalone engine layer, future game consumers—used for outcomes and implications, not mechanism inventory.
