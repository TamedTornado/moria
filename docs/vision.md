# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a **Rust crate** (or a small family of tightly scoped Rust crates). Downstream games and tools consume it as library code. It is not a game, not a demo product, and not an LLM-dependent runtime.

## Purpose

Moria exists so multiple future games can share one material world foundation: a natural-looking surface over fully mutable voxel truth, deep underground as first-class space, and game rules above the substrate. The substrate stands alone—generation, matter, queries, and mutation—without depending on the System, LLM features, or any single game’s policies.

## Product boundary

**In product (Moria):** the reusable substrate and its public integration surface for world generation, matter, presentation of matter as a non-authoritative view, mutation, queries, streaming, and persistence of edits. Consumers integrate only through that public surface.

**Adjacent, not identity:** a walkable-world executable may exist in this repository as a validation harness—a separate consumer, not the product and not a game layer. Whether it is a required current delivery is unresolved (see Q1). If present, it must use the same public interfaces available to an external game—no privileged or game-specific paths into the substrate.

**Out of this product and repository:** the actual game and its rules; System/LLM layers; spells, gas policy, combat, AI, and game-facing building layers. Compatibility seams may be designed where the substrate’s outcomes require them; those layers must not be implemented here.

**Not transferred into product scope:** harness character control, camera, authored demo route, postcard seed content, consumer presentation polish, or consumer-chosen performance gates and machine profiles.

## Required product outcomes

1. **Natural look on mutable material truth, including deep Z** — Worlds read as ordinary natural terrain while remaining grounded in voxel matter; the render is a view, not decorative authority. Material can be destroyed, placed, and reshaped throughout the volume, and continuous 3D includes underground geology and voids as first-class content.
2. **Geology-first, sparse large regions** — Worlds are produced as geology and related generation, materializable on demand so large regions stay tractable when most volume is homogeneous or untouched.
3. **GPU-resident interactive matter** — Core world matter and the work that keeps views (and any active simulation) consistent with it are organized as a GPU-resident substrate fit for interactive mutation.
4. **Public-only verbs and queries** — Consumers mutate and inspect the world only through public interfaces; nothing above the matter core reaches voxels by privileged side channels. External games and any in-repo harness share that boundary.
5. **Matter-consistent surface life** — Interactive volume that should break, burn, or block stays voxel-backed; lightweight dressing is driven by matter so digs and state changes do not desync the living surface.
6. **Streamable, delta-persistent, multi-game foundation** — Touched world streams around activity and persists as generative truth plus edit deltas (and substrate-owned object/entity journals where applicable). The substrate supplies matter, generation, queries, and mutation for future sandbox, adventure/ARPG, fortress-style, and descent-style consumers without embedding any one game’s rules, pricing, or System/LLM stack.

## Future products and enabling implications

Future products are **downstream consumers**, not Moria: System-driven ARPG, fortress/colony play, descent experiences, pure sandbox, and similar titles. They own gameplay, UX, controllers, authored content, presentation policy, and game-specific rules.

**Enabling implications (not a roadmap):** mutability, geology, deep Z, controlled APIs, and persistence make those games possible on one foundation. Long-horizon substrate notes (richer fluids, structural integrity, weather ecology, nav derivation, placement/stamping, mechanism hooks) motivate capability families for design; they do not schedule releases or import game-layer work.

A first walkable consumer slice may validate core outcomes early; it does not narrow Moria’s identity to that slice or move character, camera, or demo content into the substrate.

## Non-goals

- Shipping a playable commercial game or game rules in this repository
- Implementing System/LLM, spells, gas economy, combat, AI, or game-facing building/UX layers
- Treating harness controller, demo route, seed postcard, or benchmark theater as product features
- Substrate dependence on an LLM or on one game’s policy objects
- “Voxel cubes as the look” as the primary surface goal (grid is truth; look is natural world)

## Confirmed vision constraints

- **Ecosystem:** Rust crate or small family of tightly scoped Rust crates for Rust consumers
- **Strict consumer boundary:** harnesses and external games share public interfaces; privileged access disallowed
- **Standalone substrate:** zero LLM/System dependency in the substrate
- **GPU-resident** world substrate as product identity
- **Repository scope:** the actual game lives elsewhere; listed game layers are not implemented here

## Deferred design decisions

- Crate split and internal packaging (consumer boundary fixed; shape is design)
- Meshing, storage, generation, simulation, streaming, and persistence mechanisms and parameters
- Depth and sequence across matter-simulation capability families
- Voxel resolution, region sizing, LOD, and object-layer scaling
- Runtimes, graphics backends, target hardware, and numeric performance SLAs
- Whether multiplayer-oriented command authority appears in APIs

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required current delivery** alongside the substrate, or only a **permitted** adjacent validation artifact?

- **Proposed safe answer:** Permitted only—Moria to design and ship is the substrate; a harness may exist and must obey the public-interface boundary, but it is not mandatory current delivery.
- **If answered “required”:** the brief gains a second current deliverable (harness on public APIs) without absorbing game/UX/content ownership; delivery planning must name both. If “permitted,” harness work stays optional adjacency and must not gate substrate identity.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident voxel-world substrate Rust crate and positions the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Binding identity: substrate as product; game out of repo; harness only as validation consumer on public APIs; game-rule and listed future layers excluded; consumer boundary mandatory.
- **docs/seeds/product-one-seed.md** — First consumer/validation slice and early-proof motivation; does not redefine product identity or import demo/controller/content into the substrate.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate purpose and outcome families at design-goal altitude without elevating mechanism inventory into vision.
