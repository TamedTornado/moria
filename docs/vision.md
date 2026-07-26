# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for matter, world generation, queries, mutation, and world persistence—not a game, character demo, or LLM-backed system.

## Purpose

Moria exists so multiple games can share one material world layer: a natural-looking continuous surface and underground volume whose **authoritative truth is fully mutable voxels**, with simulation, meshing, streaming, and persistence below game rules. Games own gameplay, content, policy, and presentation; Moria owns the reusable substrate, with **no LLM or System dependency**.

## Product boundary

**In product (Moria):**

- Substrate crates and the public integration surface: consumers issue **commands** (priced verbs) and **queries**, receive a **potentially stale mirror** plus **events**, and never touch voxels by private path.
- Matter responsibilities: material world truth, geologically coherent generation, deep vertical extent, dynamic matter simulation (fluids, integrity, ambient ecology, object lifecycles), and persistence of changed worlds for reuse.
- **Authoring and extension seams** (metadata, material/structure/script registries) and other compatibility seams the substrate requires—without implementing authors, content, or game layers.

**Adjacent / not this product:**

- The **actual game** is a separate downstream consumer and is **not** part of this repository.
- A **walkable-world executable** is a **required adjacent first-delivery slice**: a separate consumer and validation harness whose purpose is walkable validation of **generation, streaming, meshing, editing, voxel-truth collision, persistence, and performance**. It is **not** product identity and is **not** a game layer. It must use the **same public interfaces** as an external game—no privileged paths. Controller, curated seed content, presentation, machine-specific targets, and delivery mechanisms stay outside substrate scope.
- Game rules; System / LLM; spells; gas / pricing policy; combat; AI; building UX / controllers; and **authored content** produced through the registries remain consumer-owned and out of Moria scope.

## Required product outcomes

1. **Reusable Rust integration surface.** Consumers use crate public APIs only. Higher layers operate via **verbs, queries, and events** on a **command-in / (stale) mirror-plus-events-out** boundary—not direct voxel access or private paths.
2. **Voxel matter is world truth.** Any occupied volume can be destroyed, moved, or placed; surface and deep underground share one continuous mutable world—not a heightmap shell with non-material decoration as authority.
3. **Reads as a normal world, looks are not truth.** Consumers may present continuous natural terrain while physics, queries, and gameplay authority stay on voxel data; rendered geometry is a regenerated view, not the saved world.
4. **Deep Z is first-class.** Underground volumes (caves, strata, deep descent) are real content and play space, not a skybox floor.
5. **Generation, streaming, and full world persistence.** Worlds materialize lazily from generation plus edits; large sparse regions stream. Persistence is **generation plus edit deltas**, including **moved/felled objects and entity/script state**, so untouched volume stays cheap and **changed worlds reload and reuse across runs and modes**.
6. **Dynamic matter services.** At engine altitude: material identity and hardness-class behavior; dig/place-class mutation; occupancy and related queries; **active fluid flow and breaches** (not only static bodies) with **material interaction rules** (e.g. quench, wetness, magma contact); **structural integrity with failure and cascading cave-ins**, **consumer-tunable** support/span policy; **granular settle**; and **thin-but-present time, seasons, weather, growth ticks, and fire ecology**—as reusable world capabilities, not a particular game’s rules.
7. **Matter-backed object lifecycles.** Things that burn, break, fall, or block (e.g. trees, rocks) share the matter model, including **growth and physical lifecycles** (growth stages, felling/conversion, re-voxelization). Pure visual dressing is a pure function of that matter and cannot desync from truth.
8. **Extensibility without owning authors.** The substrate exposes **readable/annotatable world metadata** and **shared registries** so consumers (including a future System client) can extend materials, structures, scripts/rules, and related packages through the same public surface; the System itself and content packages are not product deliverables.
9. **Standalone substrate.** Moria runs without an LLM/System; future LLM-facing consumers attach as clients of the same public surface, not as substrate features.

## Future products and enabling implications

Downstream consumers include a System-driven ARPG, a DF-style fortress/colony game, a Moria-style descent experience, sandboxes, and the walkable validation harness as first adjacent consumer. They motivate—but do not redefine—Moria’s matter, generation, mutation, streaming, persistence, and extension-seam outcomes. Their gameplay, controllers, combat, spells, gas policy, building UX, and authored content are **not** current-product scope. Enabling implication only: keep the public command/query/event surface and registries so those games can price, present, and author without forking world truth.

## Non-goals

- Shipping a full game or game layers (System/LLM, spells, gas, combat, AI, building gameplay).
- Making the walkable-world executable, its controller, camera, demo route, seed content, or machine-specific performance gates part of **product identity** (required adjacent delivery, not the substrate).
- Privileged harness-only substrate paths.
- Treating first-slice demo limits (static water only, no felling, no weather/growth in product one) as permanent substrate exclusions; delivery depth and sequence are design choices.
- Multiplayer product commitment, harness-derived platform promises, or mechanism inventories as vision mandates.

## Confirmed vision constraints

- Product form: **Rust crate or small family of tightly scoped Rust crates**; GPU-resident world substrate.
- **Consumer boundary is mandatory:** external games and the harness share one public command/query/event surface; no privileged in-repo paths; higher layers do not touch voxels directly.
- **Actual game is outside this repository.**
- **Walkable-world harness is a required adjacent first-delivery slice** outside substrate identity, public-API-only, for the validation purposes named under Product boundary.
- Game rules and future System, LLM, spell, gas, combat, AI, and building **layers are not implemented here**; seams and registries only where the substrate requires them.
- Substrate must **stand alone with zero LLM dependency**.

## Deferred design decisions

- Precise crate split, API shape, storage layout, voxel scale, meshing and sim algorithms, streaming rings, and registry/package encoding.
- Delivery depth and order of matter services—not whether those outcome families belong to the substrate.
- Harness-owned controller, curated seed content, presentation, platform, performance gates, and milestone mechanisms.
- Compatibility details beyond mandated cross-run/mode reuse of generation-plus-delta worlds; multiplayer-readiness depth beyond the public command/query/event boundary.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident voxel-world Rust substrate and the walkable-world executable as a separate validation consumer for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md** — Binds product identity to the reusable Rust substrate, excludes the actual game from the repo, places the harness adjacent and public-API-only, and hard-excludes game/System/building layers while allowing compatibility seams.
- **docs/seeds/product-one-seed.md** — Pins first delivery as substrate plus generated region and runnable character, then walkable demo and benchmark milestones; harness validates mutation, generation, meshing, streaming, collision, and persistence without transferring controller, seed content, presentation, or machine targets into substrate identity; first-slice depth limits do not shrink substrate outcome families.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (voxel truth, deep Z, dynamic matter including fluids/integrity/ambient ecology/object lifecycles, generation, command/query/event integration, registries/metadata seams, generation-plus-delta persistence with cross-run reuse, multi-game reuse, no LLM in-substrate) without importing game layers or mechanism design into this brief.
