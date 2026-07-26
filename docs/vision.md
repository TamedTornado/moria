# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** consumed as a Rust crate (or a small family of tightly scoped Rust crates). It is the world layer—not a game—that future games integrate through public interfaces.

## Purpose

Moria exists so adventure, fortress, sandbox, and related games can share one foundation: a natural-looking surface world over fully mutable voxel matter, with deep underground as first-class space. Game rules, economy, combat, AI, and authoring policy stay above the substrate. The substrate supplies matter, generation, presentation of voxel truth, thin ambient ecological behavior, mutation-coherent traversal data, queries, and mutation so each game need not re-implement the world.

## Product boundary

**Belongs to Moria**
- The reusable world substrate and its public consumer-facing surface (Rust crate(s)).
- World responsibilities at product altitude: geology-first generation; material voxel truth; smooth non-authoritative presentation of that truth; mutation and query surface; matter-backed interactable objects and matter-driven surface dressing; vegetation growth and voxel-coupled surface response; fluid and solid-matter behavior the world layer owns; thin ambient simulation (time, weather, seasons, fire ecology); world-derived navigation/traversability coherent under voxel mutation; persistence of world generation, edit deltas, and substrate-owned world-object lifecycle state; streaming of large regions.
- Compatibility seams only where substrate requirements demand them—not implementations of game layers.

**Does not belong to Moria**
- The actual game product(s); they are separate downstream consumers, not this repository’s product.
- Game rules and the System / LLM, spell, gas, combat, AI, and building **gameplay** layers. Agents and AI stay consumer-owned even when they consume substrate traversal data.
- Harness- or demo-owned character control, camera, authored routes, curated content, UI/debug presentation, fixture scenarios, workloads, and machine-specific performance or acceptance gates.

A walkable-world executable **may** exist in-repo as an adjacent validation harness that consumes the substrate only through the same public interfaces available to an external game. Whether that harness is a required repository delivery is open (see Q1). It is not part of product identity and does not import its controls, content, or gates into substrate scope.

## Required product outcomes

- **Material world truth.** Any voxel can be destroyed, moved, or placed; dig and place are first-class; the world is not decorative geometry outside matter.
- **Natural look over voxel truth.** Surface worlds read as ordinary terrain, not a cube aesthetic. Presentation is a regenerated view; interaction and queries use voxel truth.
- **Deep, geology-first world.** Underground is playable space—caves, strata, ore, aquifers, depth. Worlds generate as layered geology so digging reveals true structure; large regions can materialize lazily.
- **Surface, matter, and ambient ecology.** Interactable vegetation and micro-objects are matter-backed; lighter clutter is voxel-coupled under dig and change; vegetation growth belongs here. Static water and further substrate-owned fluid/solid behavior remain world capabilities. Time, seasons, weather, and fire ecology are thin but present. Simulation depth and sequencing are design-time.
- **Mutation-coherent traversal.** Moria supplies world-derived navigation/traversability that stays coherent after voxel edits, with continuous-Z data and support for distinct movement classes. Agents, AI, and path presentation stay with games.
- **Public surface, persistence, streaming.** Consumers use only public verbs, queries, and events. Persistence restores world generation, edit deltas, and substrate-owned world-object lifecycle state (including felled or moved objects)—not consumer game state. Large worlds stream by activity.

## Future products and enabling implications

Future consumers (not current product): a System/LLM-driven ARPG, a fortress or colony game, a Moria-style descent experience, and pure sandbox modes. They own gameplay, content, controllers, presentation policy, and economy.

Supported enabling implications (not a committed roadmap): mutability and structural honesty at surface and depth; fluid and integrity behavior for later hydrological and engineering play; object/matter coupling for felling and collapse; ambient ecology games can exploit without owning weather or fire policy; traversal data agents can use without the substrate owning AI; metadata and placement hooks so higher layers author content without owning geology. Multiplayer patterns and room/economy features stay consumer-side or later design unless a thin non-policy seam is required.

Product One is an adjacent demo/harness concept, not Moria’s identity. Its first-slice exclusions (for example weather, seasons, growth, or save-slot limits) do not remove the corresponding substrate responsibilities above.

## Non-goals

- Implementing any full game, combat, stats, AI, or game entities.
- System / LLM integration, spells, gas metering, or intent/pricing policy as product features.
- Building-game UX, player blueprints, mechanisms-as-gameplay, or fortress labor/economy.
- Treating a walkable-world harness’s character, route, art direction, curated content, workload, or machine gates as substrate requirements.

## Confirmed vision constraints

- **Ecosystem:** a Rust crate or small family of tightly scoped Rust crates for Rust consumers.
- **GPU-resident** world substrate as the intended execution model for the matter world.
- **Strict consumer boundary:** any in-repo harness and any external game use the same public interfaces; adjacent consumers have no privileged internal access.
- **Zero LLM dependency:** the substrate stands alone; the System is a future game-layer client, not a substrate feature.
- **Layering force:** game policy is not implemented in Moria; only substrate-demanded compatibility seams may be designed.

## Deferred design decisions

- Crate split, APIs, algorithms, storage, meshing, voxel size, LOD, streaming policy, and persistence encoding (including object lifecycle journals).
- How much generation detail, matter simulation, ambient/ecological depth, object-physics coupling, and traversal fidelity ship in which delivery slice.
- Performance budgets, target hardware, graphics backends, and validation workloads.
- Exact seams for future fortress/ARPG needs without pulling game layers in-repo.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo walkable-world validation harness a **required repository delivery** for the current effort, or only **permitted** as an adjacent artifact?

- **Proposed safe answer:** Permitted only—identity stays the substrate; design may still add a harness that uses public APIs only.
- **If answered “required”:** the repository must ship a walkable-world harness that exercises the public product boundary and validates generation, streaming, meshing, editing, collision, persistence, and performance. Its first-slice delivery mandate is a generated natural region traversable from surface into deep space, collision against voxel truth, dig/place proof, and benchmarked validation. Harness-specific controller, camera, curated content, presentation, workloads, and machine gates remain adjacent—not Moria product outcomes.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and positions the walkable-world executable as a separate validation harness for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md** — Binds product identity to the substrate crate(s), excludes the actual game and named game layers (including AI), and requires any harness to use the same public interfaces as external games.
- **docs/seeds/product-one-seed.md** — Describes an adjacent first demo/harness slice; its narrow exclusions do not redefine substrate identity, while its high-level validation mandate is recoverable if the harness is required.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate responsibilities (material truth, natural look, deep Z, generation, surface/matter/ambient ecology, mutation-coherent traversal, persistence including world-object state, streaming, reusable layering) that ground required outcomes without importing game features.
