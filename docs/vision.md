# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a Rust crate (or a small family of tightly scoped Rust crates). Downstream games and tools consume it; it is not itself a game.

## Purpose

Moria exists so multiple games—and validation artifacts—can share one material world foundation: a natural-looking surface world over fully mutable voxel truth, with deep underground play, generation, matter behavior, queries, and mutation, and with **zero dependency on an LLM or “System” layer**. Game rules live above the substrate; the substrate provides the world they run on.

## Product boundary

**In product**

- The reusable voxel-world substrate and its public consumer-facing surface (crate API and related public interfaces).
- Substrate responsibilities for world generation, sparse GPU-resident matter, smooth visual reconstruction of that matter, mutation and query verbs, persistence and streaming of world truth, and matter-level physics and simulation hooks required for reuse across games.

**Adjacent, not identity**

- A walkable-world executable **may** exist as an adjacent validation consumer of the substrate. Whether delivering it is part of current work is unresolved (see Q1). It is not the product identity. If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific substrate paths.
- The actual game (or games) are separate downstream consumers and are **not** part of this repository.

**Out of product**

- Game rules; System/LLM features; spells; gas/pricing policy; combat; AI; and building/gameplay layers (including building UX, fortress work orders, and game-specific presentation or control schemes).
- Authored demo content, characters, cameras, controllers, routes, and acceptance scenarios belong to adjacent consumers or harnesses, not to the substrate’s product identity.

## Required product outcomes

A downstream design must make these product-level outcomes true:

1. **Material world as truth.** Any cell of the world can be destroyed, moved, or placed; decorative-only geometry outside the material model is not the source of play or collision truth. The rendered surface is a regenerated view of voxel matter, not the authority.
2. **Reads as a continuous natural world.** Rolling terrain, vegetation, water bodies, cliffs, and similar surface reading are supported without forcing a blocky cube aesthetic as the primary look; player-made cuts and sharp features remain legible as cuts and structures.
3. **Deep Z is first-class.** The underground is real continuous volume—strata, caves, ores, aquifers, and deep space—not a painted floor under a heightmap.
4. **Geology-first generation with cheap idle cost.** Worlds are produced as geology and related volume (not heightmap-plus-paint), materializable on demand so large regions need not reside fully as dense voxels until touched.
5. **Public mutation and query boundary.** Consumers change and inspect the world only through the substrate’s public verbs and queries; nothing above the matter core reaches voxels by privileged back doors. Gas/cost policy is injected by the consumer, not hard-wired as one game’s economy.
6. **Shared matter behavior, persistence, and streaming.** Games share one matter stack: interactable voxel-backed surface objects (distinct from dressing that still tracks underlying matter), fluids, structural honesty, granular materials, and ambient matter responses—expressed as substrate behavior, not combat or quest logic. Authoritative truth is regenerable world definition plus edit deltas; active regions stream so large editable worlds stay workable.

## Future products and enabling implications

Future **consumers** (not this product) include a System/LLM-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandboxes. Public mutation, query, and observation boundaries exist so those games can attach without forking world truth.

Enabling implications (not a committed multi-game roadmap or feature schedule): seams and registries so a later game or System can author materials, placements, and structures without the substrate embedding that game; column/Z conveniences that make fortress-style views comfortable; and verb/query architecture that stays multiplayer-ready in shape. Gameplay, content, UX, and policy for those titles remain entirely consumer-owned.

A first walkable-region demo may prove the substrate, but its character, camera, curated route, debug presentation, and performance gates do not define Moria’s product identity (see Q1).

## Non-goals

- Implementing the System, LLM hooks as product features, spells, gas economies, combat, or agent AI.
- Implementing building/gameplay layers (blueprints-as-gameplay, work orders, mechanisms-as-game systems, room economy).
- Shipping the eventual commercial game inside this repository.
- Treating the validation harness’s specific content, controls, or benchmark theater as substrate scope.

## Confirmed vision constraints

- **Ecosystem:** delivered as Rust crate(s) for Rust consumers.
- **GPU-resident** voxel-world substrate (product promise of residency model, not a particular GPU vendor or API).
- **Standalone:** no LLM/System dependency in the substrate.
- **Consumer isolation:** adjacent executables and external games share one public interface class; no privileged harness path into matter.
- **Repository boundary:** the actual game is out of this repo; Moria is the substrate (and only possibly an adjacent harness—Q1).

## Deferred design decisions

- Exact crate family split and internal module boundaries (beyond the non-optional consumer boundary).
- Voxel resolution, LOD strategy, object-registry scaling, and fluid-solver fidelity.
- Sequencing and depth of the first substrate slice versus later matter capabilities.
- Harness/demo content, controls, platforms, and numeric performance gates (consumer/design concerns once Q1 is settled).
- Whether and when multiplayer is built (architecture may remain ready without committing delivery).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** alongside the substrate crates, or only a **permitted adjacent artifact** that may be added without being part of the current delivery commitment?

- **Proposed safe answer:** Permitted adjacent artifact only—current committed product delivery is the substrate crates and public interfaces; a harness may exist later or in parallel but is not mandatory for product completeness.
- **If answered differently:** Requiring the harness expands current delivery to include an adjacent executable that proves walkability through the public API, without moving game content, controllers, or acceptance theater into substrate identity; treating it as out of scope entirely would drop even the permission stated in the project boundary.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and frames the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Binds product identity to the substrate crate(s), excludes the game and listed game layers from the repo, and requires any harness to use public interfaces only.
- **docs/seeds/product-one-seed.md** — Describes a first walkable-region consumer/demo that motivates proof of mutability and natural reading; its content, controller, milestones, platforms, and numeric gates stay consumer-side and do not redefine the substrate.
- **docs/seeds/voxel-world-substrate.md** — Supplies the substrate’s outcome-level design goals (natural look, full mutability, deep Z, reusable layering, generation, matter, persistence/streaming) and future game consumers, without making those games or mechanism catalogs part of this brief’s delivery plan.
