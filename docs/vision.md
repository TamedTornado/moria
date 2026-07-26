# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation for external games—not a game or gameplay stack.

## Purpose

Moria exists so multiple games can share one material world: a natural-looking surface over fully mutable voxels, deep underground as first-class content, and game rules above the substrate. The substrate stands alone with no LLM or System dependency. Downstream titles consume the same matter, simulation, queries, and mutation surface rather than each rebuilding a world engine.

## Product boundary

**This product owns:** the reusable voxel-world substrate—geology-oriented generation; sparse GPU-resident matter and active matter simulation; presentation as a non-authoritative view faithful under edit; matter-coherent dressing and dynamic voxel-backed objects; mutation and query surfaces with GPU/consumer coupling (commands in; potentially stale mirror and events out); collision and mutation-safe navigation from voxel truth; streaming and persistence; a game-neutral materials/behavior extension surface; and compatibility seams where substrate requirements demand them.

**Adjacent, not this product:** a walkable-world executable may exist as a validation harness that uses only the public interfaces available to an external game. Whether it is a required repository delivery is unresolved (see Q1). Harness-owned controls, characters, cameras, demo content, machine profiles, benchmark gates, and milestone packaging do not become substrate scope. Substrate wgpu/WGSL portability across Metal, Vulkan, and DX12 is a crate constraint, not a harness platform choice.

**Downstream / future consumers own:** the actual game(s), game rules, System/LLM execution, spells, gas/pricing policy, combat, AI and agent labor, building/gameplay layers, and game-specific content and presentation.

**Repository boundary:** the actual game is not part of this repository. Adjacent consumers have no privileged path into the substrate.

## Required product outcomes

- **Reusable substrate with a game-neutral surface.** External games integrate through public Rust crate interfaces. Nothing above the matter surface reaches voxels except via substrate verbs and queries. Materials, behavior rules, and related registries serve hand-authored and automated clients alike; System/LLM execution stays outside the product.
- **Voxel truth that reads as a normal world, including after edit.** Rolling terrain, forests, water, cliffs, and meadows present as a continuous, smooth, non-voxel-looking surface; cuts and sharp features stay legible; the view is regenerated from matter, never authoritative, and stays faithful under dig and place.
- **Mutable everywhere; deep Z; geology-first sparse worlds.** Any voxel can be destroyed, moved, or placed. Underground geology is real content. Worlds generate as layered geology with lazy materialization so homogeneous volume stays cheap; the world streams around activity; truth is worldgen plus edit deltas (and substrate-owned object/entity journals).
- **Active matter that behaves and interacts coherently.** Fluids, granular materials, fire and wetness, structural support and failure, and ambient weather/time simulation are substrate-owned so matter—including fluids, fire/weather effects, granular collapse, and cave-ins—interacts coherently.
- **Dynamic voxel objects and matter-coherent dressing.** Interactable vegetation and objects break, move or fall, and grow where applicable while coherent with voxel matter; tree falling is included. Grass and similar dressing derives from matter so dig, burn, and trample stay visually consistent.
- **Consumer coupling, collision, and mutation-safe traversal.** Consumers issue commands into GPU-resident matter and receive a potentially stale mirror plus events out. Collision runs against voxel truth. The substrate provides derived walkability and navigation across continuous 3D movement classes, with dirty matter invalidating affected navigation—without owning AI or agent labor.

If Q1 makes the harness a required delivery, its adjacent outcome is a third-person walkable natural region that proves smooth mutable voxel truth and validates generation, meshing, streaming, collision, persistence, and performance via public interfaces—without importing harness controls, content, platforms, or performance numbers into substrate identity. While Q1 is open, treat that artifact only as unresolved (see Q1).

## Future products and enabling implications

Future consumers—not current product—include a System/LLM ARPG, a fortress/colony game, a Moria-style descent, and pure sandbox modes. Product One’s walkable natural region motivates proof points; it does not narrow substrate matter responsibilities or transfer first-slice omissions into product identity. Enabling implications (no gameplay import): continuous 3D dig/build matter; deep-Z for level-style views; priced verbs and observable state; coherent matter simulation; scar save/reload across modes. System/LLM hooks attach only as ordinary clients and content authors above the substrate.

## Non-goals

- Implementing the actual game, game rules, combat, AI, agent labor, or entity gameplay beyond world/matter services the substrate must expose
- System/LLM runtime, spells, gas metering, or pricing policy
- Building/gameplay layers: blueprint economies, mechanisms-as-gameplay, room designation, fortress UX
- Treating harness demo content, third-person presentation, harness machine profiles, harness benchmark gates, or milestone packaging as substrate requirements
- LLM dependency in the substrate

## Confirmed vision constraints

- Delivery form is a Rust crate or small family of tightly scoped Rust crates.
- The world substrate is GPU-resident: compute-backed matter with commands in and a potentially stale mirror plus events out to consumers.
- Load-bearing GPU layers stay on wgpu/WGSL; no native Metal-only fork; crate intended to stay portable across Metal, Vulkan, and DX12 (crate constraint, not harness machine selection).
- The substrate has zero LLM dependency and must function as a standalone engine layer.
- Any validation harness in this repository must use only the public interfaces an external game would use—no privileged substrate paths.
- Game rules and the System, LLM, spell, gas, combat, AI, and building layers are out of scope for implementation here; seams may be designed, those layers must not be built in this product.

## Deferred design decisions

- Voxel size, brick layout, meshing strategy, LOD, and object-layer capacity
- Capability depth and delivery sequence within required matter-simulation, object-lifecycle, and navigation families
- Crate split within the workspace, persistence encoding, and streaming-ring policy
- Whether and how multiplayer authority is realized on the verb/command surface
- Harness scenario content, controls, selected run platforms, and benchmark gates if a harness is delivered

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required current delivery** of this repository, or only a **permitted adjacent validation artifact**?

- **Proposed answer:** Permitted only—the product is the substrate; a harness may exist to validate public APIs but is not required for product completeness.
- **If different:** Requiring the harness keeps identity on the substrate but adds a mandatory adjacent deliverable: a third-person walkable natural region that proves smooth mutable voxel truth and validates generation, meshing, streaming, collision, persistence, and performance via public interfaces (still without controller, character, demo content, machine gates, or milestone plan in substrate scope).

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world substrate (Rust crate); walkable-world executable is a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Binding boundary: substrate as Rust crate(s); game out of repo; harness only via public APIs; game/System/LLM/spell/gas/combat/AI/building layers out of scope.
- **docs/seeds/product-one-seed.md** — Walkable-region consumer scenario and first-slice limits; motivates proof points and harness natural-region proof if Q1 requires it; establishes substrate wgpu/WGSL portability (no Metal-only fork; Metal/Vulkan/DX12) without harness ownership, machine profiles, atomics gates, or performance baselines.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcomes: edit-faithful natural presentation, mutability, deep Z, geology generation, active matter, dynamic voxel objects, GPU coupling with events and stale mirror, navigation, streaming/persistence, and game-neutral extension registries.
