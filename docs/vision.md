# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a **Rust crate** (or a small family of tightly scoped Rust crates). It is an engine-layer world foundation for external games—not a game, not a gameplay stack, and not defined by any single demo scene.

## Purpose

Moria exists so multiple games can share one material world layer: landscapes that read as ordinary natural worlds while remaining fully diggable voxel truth, including deep underground as real content. The substrate owns matter, geology-first generation, matter-derived visualization, physics-relevant world behavior, observability, derived world services, and public mutation. It must stand alone with **no LLM or System dependency**. Game identity, rules, game-specific presentation, and policy live only in downstream consumers.

## Product boundary

**In product**

- The reusable substrate: authoritative voxel matter; geology-first generation; material-derived meshes and matter-coupled dressing as non-authoritative views; matter-coupled interactable objects and lifecycles; support, granular, fluid, and ambient world behavior; observable mirrors and events; mutation-safe navigation/traversal data; public mutation and query surfaces; streaming; voxel edit deltas and separate per-region object/entity journals; cross-run or cross-mode reuse of delta sets.
- Delivery as Rust crate(s) for external game consumers.
- Load-bearing layers stay on **wgpu/WGSL** (no native-Metal fork), keeping the crate portable to Vulkan/DX12.

**Adjacent, not product identity**

- A **walkable-world executable** may exist in this repository as a public-API consumer and validation harness. Product One is a product-shaped first slice for that harness (curated natural region, third-person traversal, public dig/place proof, streaming/persistence/collision validation, benchmarks). Whether the harness is a required current delivery is unresolved (**Q1**). Character, controller, camera, route, content, presentation, workloads, platforms, and performance gates stay harness-owned.

**Out of product / repository**

- The actual game and all game rules.
- System / LLM, spells, gas policy, combat, AI, and building *game* layers. Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.
- Game-specific presentation, content, controllers, characters, and policy—not the substrate’s own world visualization and generation.

## Required product outcomes

Downstream design must make these true for the substrate:

1. **Authoritative mutable matter** — A GPU-resident voxel world is the source of truth; any volume can be destroyed, moved, or placed. Matter-derived meshes and dressing are regenerated views and never the saved authority.
2. **Natural surface, material truth** — Continuous natural landscapes while geology generation, material-derived meshes, matter-coupled dressing, and voxel-backed interactables stay substrate-owned so appearance cannot desync from dig or other material change.
3. **Deep-Z geology at scale** — Underground is first-class content (strata, caves, ores, aquifers and similar), from geology-first generation with lazy materialization and sparse representation so large regions stay tractable until touched.
4. **Matter-coupled world behavior** — Interactable objects have matter-coupled lifecycles (including fall, break, and re-integration); granular materials and structural support fail honestly; fluids have active behavior and material interactions; ambient time, weather, and fire ecology run as thin but present world behavior.
5. **Public mutation and observability boundary** — Consumers change the world only through public verbs; they observe via queries, a coarse world mirror that may be stale, and events. Nothing above the matter core owns privileged voxel paths.
6. **Derived world services, streaming, and persistence** — Mutation-safe navigation/traversal data stay valid as the world changes; large worlds stream around activity; persistence stores generation plus voxel edit deltas and separate per-region object/entity journals, with cross-run or cross-mode reuse of delta sets. Locomotion and interaction collide against voxel occupancy, not the render mesh alone.
7. **Portable graphics stack** — Load-bearing substrate code stays on wgpu/WGSL so the crate can target Vulkan and DX12 without a native-Metal fork.

## Future products and enabling implications

Future **consumers** (not this product) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandboxes. They motivate the substrate outcomes above; their gameplay, content, controllers, characters, animation, UX, and policies remain with those products.

Enabling implications only: keep layering so gas/pricing policy, LLM authorship, and game semantics can plug in above the substrate without becoming substrate features.

## Non-goals

- Shipping an actual game or game-rule stack in this repository.
- Implementing System/LLM, spells, gas, combat, AI, or building-game layers here.
- Treating the harness’s demo fiction, UI, controls, content, acceptance scene, or performance gates as substrate identity.
- Making the substrate depend on an LLM.
- Forking load-bearing layers to native Metal.

## Confirmed vision constraints

- Product form is a **Rust** crate or small family of tightly scoped crates.
- Any in-repo harness and all external games share one **public** consumer boundary; no privileged substrate paths for the harness.
- Substrate operation has **zero LLM dependency**.
- Matter is **GPU-resident** as part of product identity.
- Consumer-facing observation may be a **stale mirror** plus events, not a live authoritative CPU copy of every voxel.
- Load-bearing layers use **wgpu/WGSL only**—no native-Metal fork—with **Vulkan/DX12 portability** as a crate outcome. Mid-GPU and M4 frame-rate or memory gates are harness-owned, not substrate promises.

## Deferred design decisions

- Precise crate split and workspace layout (beyond the consumer boundary itself).
- Mechanisms, encodings, and delivery sequence for generation, meshing, fluids, integrity, ambient simulation, object lifecycles, navigation, and persistence—not whether those outcome families belong to the product.
- Harness content, controls, demo world, platforms, and performance acceptance (if a harness is delivered).
- How exact restoration of a delta save after defacement is measured for Product One’s benchmark (harness acceptance, not a blanket substrate claim for object/entity journals).
- Measurement questions (for example voxel-size tradeoffs) answered by design and validation, not by vision.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world validation harness a **required current delivery** of this repository, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted only—the product promise is the substrate crates; a harness may exist and must use public APIs, but shipping it is not required for the product to be complete.
- **If answered “required”:** Product identity stays substrate-only, but the repository must also deliver the adjacent Product One–shaped harness: curated natural region, third-person traversal, public dig/place proof, streaming/persistence/collision validation, and benchmark evidence—without importing character, controls, content, platforms, or performance gates into substrate scope. (An in-repository harness remains allowed either way; prohibition is not on the table.)

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and frames the walkable-world executable as a separate validation consumer—not a game layer.
- **docs/seeds/project-boundary.md** — Settles product identity (substrate Rust crate(s)), excludes the actual game and listed game layers, permits a public-API-only in-repo harness, and forbids privileged harness paths.
- **docs/seeds/product-one-seed.md** — Defines the adjacent first validation slice, crate-owned wgpu/WGSL portability with Vulkan/DX12 as the crate point, harness-owned M4/3060 gates, and exact delta restore only for the Product One save/load benchmark after defacement.
- **docs/seeds/voxel-world-substrate.md** — Authorizes the substrate’s full outcome families: natural mutable worlds, deep-Z geology, generation and matter-derived visualization, object/fluid/support/ambient behavior, stale mirror and events, mutation-safe navigation, voxel edit deltas plus separate object/entity journals with cross-run reuse of delta sets, and multi-game layering without LLM dependency.
