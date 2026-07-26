# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world **substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is the material-world engine layer that downstream games consume; it is not a game.

## Purpose

Moria exists so multiple games can share one trustworthy world of matter: continuous 3D space whose voxel truth supports natural-looking terrain, deep underground play, and dig/build mutability, without embedding any particular game’s rules, presentation, or content pipeline. Games inject policy above the substrate; the substrate provides matter, generation, simulation hooks, queries, and mutation.

## Product boundary

**In product**

- The reusable substrate crates and their public interfaces for world generation, matter storage and mutation, presentation of voxel truth, streaming, persistence of edits, collision/query against voxel truth, and related matter-level simulation.
- Compatibility seams where substrate requirements need them so future game layers can attach without living in this product.

**Adjacent / not identity**

- A walkable-world executable may exist as a separate validation consumer of those public interfaces. It is not the product identity and must not own privileged or game-specific paths. Whether shipping that executable is a required current delivery remains open (see Q1).
- The actual game (or games) that use Moria are separate downstream products and are not part of this repository.

**Out of product**

- Game rules and future game layers: System/LLM, spells, gas/pricing policy, combat, AI, and building-as-gameplay systems (UI, work orders, designations, game economy).

## Required product outcomes

Downstream design must make these true for the substrate:

1. **Voxel matter is authoritative truth.** The world is a fully material voxel volume; presentation (smooth terrain, dressing) is a view regenerated from that truth, never the source of physics or mutation.
2. **Reads as a natural world.** Generated surface and underground can present as ordinary terrain (hills, forest, water, rock, caves) rather than a cube aesthetic as the default look, while remaining editable matter underneath.
3. **Mutable everywhere.** Consumers can destroy, place, and reshape voxels throughout the volume (surface through deep Z); mutability is a product capability, not decorative geometry outside the material model.
4. **Deep Z is first-class.** Continuous 3D play space supports meaningful underground content and traversal; depth is not a fake floor under a heightmap surface.
5. **Geology-oriented generation.** Worlds can be produced so digging and exploration encounter real strata, voids, and materials, with lazy materialization and sparse representation so large regions remain practical.
6. **Consumer-facing world services.** The substrate exposes mutation and query paths so external games can run, collide, edit, stream, and persist against the same public surface; nothing above the matter layer needs private voxel access. Streaming and edit-delta persistence keep large, scarred worlds viable. Matter-level behaviors that multiple game styles depend on (e.g. fluids, structural support, interactable surface objects vs pure dressing) are substrate responsibilities at capability altitude; depth and sequence are design choices.

## Future products and enabling implications

Future consumers (not this product) include an ARPG that may use a System/LLM layer, a fortress/colony-style game, a Moria-style descent experience, and pure sandbox modes. The substrate’s layering and public API are meant to support those styles without implementing their rules, content, controllers, combat, or presentation.

Enabling implications already motivated by the seeds: reusable dig/build matter, deep continuous Z, natural presentation over voxel truth, and generation/persistence/streaming that outlive any single game mode. Gameplay systems for those titles remain consumer-owned.

## Non-goals

- Implementing a shippable game, campaign, or authored adventure inside this repository.
- System/LLM orchestration, spells, gas/intent pricing, combat, stats, or agent AI as product features.
- Game-owned building UX, blueprints-as-gameplay, work orders, or economy.
- Treating the validation harness’s character, camera, demo route, content set, platform gates, or performance numbers as the product contract.
- Making the substrate depend on an LLM to function.

## Confirmed vision constraints

- **Identity:** reusable GPU-resident voxel-world substrate, not a game layer.
- **Integration:** Rust crate (or small crate family) for consumption by external games.
- **Consumer boundary:** any validation executable uses the same public interfaces as an external game; privileged harness-only world paths are excluded.
- **Standalone engine:** substrate must operate with zero LLM dependency; System-style clients are optional game-layer consumers if added later elsewhere.
- **Repository scope:** the actual game is out of this repo; game-rule layers listed under Non-goals are not implemented here (seams only where required).

## Deferred design decisions

- Crate split, API shape, storage layout, meshing approach, and simulation algorithms.
- First delivery depth and milestone order for substrate capabilities (generation completeness, which matter sims ship when, object rigid-body coupling, etc.).
- Voxel resolution, LOD strategy, object-layer scaling, fluid fidelity, and multiplayer timing—open technical choices, not product identity.
- Whether, when, and how a walkable-world harness is built, and what demo content or benchmarks it uses (subject to Q1 for delivery obligation only).
- Platform backends and performance targets for specific machines or demo scenes.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** of this repository, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted only—the product promise is the substrate crates and public interface; a harness may be added to exercise them but is not itself a committed deliverable until design plans it.
- **If answered differently:** Requiring the harness as current delivery keeps product identity on the substrate but adds an adjacent delivery obligation (still without importing its controller, content, or performance gates into the product boundary). Treating it as merely permitted leaves harness work optional relative to substrate outcomes.

## Seed synthesis

- **README.md** — Names Moria as a GPU-resident voxel-world substrate consumed as a Rust crate, and separates a walkable-world executable as consumer/validation rather than game.
- **docs/seeds/project-boundary.md** — Binds product identity to the reusable Rust substrate, keeps the real game out of repo, permits a public-API-only validation harness, and excludes game-rule layers.
- **docs/seeds/product-one-seed.md** — Motivates first-slice validation of a walkable material world and dig/place proof; its controller, region, content, platform, and performance detail stay harness/design depth, not product identity.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate outcome families (natural look over voxel truth, full mutability, deep Z, geology generation, matter services, multi-game reuse, no LLM dependency) without transferring game systems into this product.
