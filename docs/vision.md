# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for matter, world generation, queries, and mutation—not a game, not a character demo, and not an LLM-backed system.

## Purpose

Moria exists so multiple games can share one material world layer: a natural-looking continuous surface and underground volume whose **authoritative truth is fully mutable voxels**, with simulation, meshing, streaming, and persistence provided below game rules. Games own gameplay, content, policy, and presentation; Moria owns the reusable world substrate they stand on, with **no LLM or System dependency**.

## Product boundary

**In product (Moria):**

- The substrate crates and the public interfaces through which any consumer generates, streams, queries, mutates, meshes views of, and persists voxel worlds.
- Substrate-level matter responsibilities: material world truth, generation of geologically coherent regions, deep vertical extent, and enabling physics-facing behavior of that matter for consumers.
- Compatibility seams only where substrate requirements need them—without implementing game layers above.

**Adjacent / not this product:**

- The **actual game** is a separate downstream consumer and is **not** part of this repository.
- A **walkable-world executable** may exist as a validation harness. It is **not** the product identity. Whether shipping it is a required adjacent delivery of the current effort is unresolved (**Q1**); until answered, treat it only as a permitted adjacent artifact. If present, it must use the **same public interfaces** as an external game—no privileged or game-specific substrate paths.
- Game rules; System / LLM; spells; gas / pricing policy; combat; AI; and game building / UX / controllers / authored content and presentation remain consumer-owned and out of Moria scope.

## Required product outcomes

1. **Reusable Rust integration surface.** Downstream games (and any harness) consume Moria as crate(s) through public APIs only; nothing above the substrate may depend on private world implementation paths.
2. **Voxel matter is world truth.** Any occupied volume is material that can be destroyed, moved, or placed; surface and deep underground share one continuous mutable world—not a heightmap shell with non-material decoration as authority.
3. **Reads as a normal world, looks are not truth.** Consumers can present continuous, natural-looking terrain and structures while physics, queries, and gameplay authority remain on voxel data; rendered geometry is a regenerated view, not the saved world.
4. **Deep Z is first-class.** Underground volumes (caves, strata, deep descent) are real content and play space, not a skybox floor under a surface shell.
5. **Generation, streaming, and persistence at scale.** Worlds materialize lazily from generation plus edits; large sparse regions stream in and out; persistence is generation-plus-deltas so untouched volume stays cheap and player scars survive.
6. **Matter services for games.** The substrate provides the material operations games need at engine altitude: material identity and hardness-class behavior, dig/place-class mutation, occupancy and related queries suitable for traversal and interaction, and substrate-owned support for fluid volumes, structural support of solid matter, granular settle, and fire/ambient matter effects—as reusable world capabilities, not as a particular game’s rules.
7. **Interactable surface life is matter-backed where it must be.** Things that burn, break, fall, or block (e.g. trees, rocks) participate in the same matter model; pure visual dressing stays a pure function of that matter so it cannot desync from truth.
8. **Standalone substrate.** Moria runs without an LLM/System; future LLM-facing consumers attach as clients of the same public surface, not as substrate features.

## Future products and enabling implications

Downstream consumers described in the seeds include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, pure sandboxes, and any walkable validation harness. They motivate—but do not redefine—Moria’s matter, generation, query, mutation, streaming, and persistence outcomes. Their gameplay, controllers, characters, combat, spells, gas policy, building UX, rooms/economy policy, and authored content are **not** current-product scope. Enabling implication only: keep a clean public verb/query surface so those games can price and present matter operations differently without forking world truth.

## Non-goals

- Shipping a full game, game rules, or game layers (System/LLM, spells, gas, combat, AI, building gameplay).
- Making the walkable-world executable, its controller, camera, demo route, seed postcard content, or benchmark scene part of product identity.
- Implementing privileged harness-only substrate paths.
- Treating first-slice demo limits (e.g. static water only, no felling) as permanent substrate exclusions; depth and sequence of delivery are design choices, not identity changes.
- Multiplayer product commitment, platform/hardware product promises derived only from a harness machine, or mechanism inventories (brick sizes, algorithms, crate graphs) as vision mandates.

## Confirmed vision constraints

- Product form: **Rust crate or small family of tightly scoped Rust crates**; GPU-resident world substrate.
- **Consumer boundary is mandatory:** external games and any validation harness share one public interface surface; no privileged in-repo consumer paths.
- **Actual game is outside this repository.**
- Game rules and future System, LLM, spell, gas, combat, AI, and building **layers are not implemented here**; seams only where the substrate itself requires them.
- Substrate must **stand alone with zero LLM dependency**.

## Deferred design decisions

- Precise crate split, API shape, storage layout, voxel scale, meshing and sim algorithms, and streaming ring design.
- Delivery depth and order of matter services (which fluid, integrity, fire, object, and persistence capabilities ship in which slice).
- Harness existence as a committed delivery (**Q1**), and all harness-owned controller, content, presentation, platform, and performance gates.
- How far multiplayer-readiness or cross-game save reuse is pursued beyond the public command/query boundary already implied by reuse.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required adjacent deliverable** of the current effort, or only a **permitted** validation artifact that may exist beside the substrate?

- **Proposed answer:** Permitted only. Current identity and required outcomes attach to the substrate crates and public interfaces; a harness may be built to exercise those interfaces but is not a committed delivery until design says so.
- **If different:** Requiring the harness adds a committed adjacent deliverable (ship a public-API-only walkable validation app) without changing substrate identity; harness controls, content, and performance gates still stay outside product scope.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident voxel-world Rust substrate and positions the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Binds product identity to the reusable Rust substrate, excludes the actual game from the repo, permits a public-API-only harness, and hard-excludes game/System/building layers while allowing compatibility seams.
- **docs/seeds/product-one-seed.md** — Describes a first walkable demo slice that motivates mutation proof, generation, meshing, streaming, and API boundary enforcement; its controller, seed content, and performance environment are harness/first-slice detail, not product redefinition.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families (natural look over voxel truth, full mutability, deep Z, generation, matter services, streaming/persistence, multi-game reuse, no LLM in-substrate) without importing game layers or mechanism design into this brief.
