# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** for external games and tools. It is delivered as a **Rust crate** (or a small family of tightly scoped Rust crates). The product is the world engine layer—matter, generation, mutation, queries, and derived presentation of material truth—not a game.

An adjacent **walkable-world executable** may ship in-repo solely as a **validation harness** that exercises the substrate through the same public interfaces an external game would use. Whether that harness is a required current delivery is open (see Q1); its character, controls, route, content, and presentation are not part of the product identity either way.

## Purpose

Games and sandboxes need a shared foundation where the surface world reads as ordinary nature, every visible volume is material that can be changed, and the underground is real continuous space—not a heightmap with props. Moria exists so multiple downstream products (adventure, fortress, sandbox, or other) can stand on one substrate without reimplementing world truth, and without embedding game rules, economy, or AI into that foundation.

## Product boundary

**Belongs to Moria**

- Reusable substrate APIs and runtime for a GPU-resident voxel world: geological generation, material storage and sparsity, mutation, derived visual mesh and surface dressing driven by matter, streaming and persistence of world truth, and query/mutation surfaces for consumers.
- In-repo validation that consumes only public substrate interfaces (if the harness is in scope).
- Compatibility seams where substrate requirements demand them—without implementing game layers.

**Does not belong to Moria**

- The actual game, game rules, combat, stats, AI, economy, gas/pricing policy, spells, or the System / LLM layer.
- Building gameplay, blueprints-as-product, mechanisms-as-gameplay, work orders, rooms/designation UX, or other semantic game layers—though the substrate may later expose matter and verb primitives those layers need.
- Authored campaign content, characters, cameras, HUD, and controllers owned by a game or by the validation harness.
- Privileged in-repo paths that bypass public interfaces for “the official” consumer.

## Required product outcomes

A competent design must make these consumer-visible guarantees true of the substrate:

1. **Material world, not decorative terrain.** What the player can stand on, dig through, and place is voxel matter; the rendered surface is a non-authoritative view regenerated from that truth so carved and natural forms both read as real cuts and geology, not props on a heightfield.
2. **Mutable everywhere, including deep Z.** Any volume in the active world can be destroyed, altered, or filled; caves, strata, and underground structure are first-class continuous 3D content, not a thin floor under the sky.
3. **Generated as geology, loaded sparsely.** Regions come from seed-driven geological generation with lazy materialization so large volumes stay tractable; homogeneous empty or solid space does not pay full dense cost.
4. **Consumer isolation.** Nothing outside the substrate’s allowed surface touches voxels directly; games and harnesses use the same public mutation and query boundary (sandbox, multiplayer-readiness, and reuse hinge on this).
5. **Durable scars, streamable presence.** Truth is regeneration from seed plus edit deltas; active neighborhoods stream in for render and simulation relevance without requiring the whole region as dense resident data.
6. **Stand-alone engine layer.** The substrate has zero dependency on LLM/System features and does not implement game policy; gas, labor, or other pricing—if any—are consumer-injected policy, not baked identity.

## Future products and enabling implications

Downstream **games** (System ARPG, fortress/colony, descent/adventure, pure sandbox) are **future or external consumers**, not this repository’s product. They motivate seams and matter capabilities; they do not pull their gameplay, content, presentation, or acceptance scenarios into Moria.

Enabling implications the seeds support at substrate altitude (depth and order are design concerns, not a committed roadmap here): richer matter dynamics (flowing fluids beyond still bodies, fire/wetness-style aggregate behavior, granular settle, structural integrity), richer surface life (interactable vegetation and micro-objects as matter-backed objects), and building-oriented primitives (placement, stamps, queryable support) so fortress- or sandbox-style games can sit above the same stack. Ambient weather and seasonal behavior may exist thinly so a natural world continues to read as alive; they remain substrate-side only where they serve world legibility, not game systems.

## Non-goals

- Shipping a game, combat loop, progression, AI creatures, or multiplayer product in this repo.
- Implementing the System, LLM authorship runtime, spell/gas combat fantasy, or economy.
- Treating the validation harness’s demo route, character fantasy, UI, or marketing milestones as substrate scope.
- Full per-voxel fluid simulation at planetary scale as a product promise.
- Native-only graphics forks that abandon the portable GPU integration path.

## Confirmed vision constraints

- **Integration shape:** Rust crate(s) for engine consumers; not a closed standalone game app as the product.
- **Graphics portability:** Load-bearing GPU work stays on a portable path (wgpu/WGSL); no native Metal-only fork of those layers.
- **Consumer parity:** Any in-repo harness or demo uses the same public interfaces available to an external game—no privileged substrate access.
- **Supported intent:** GPU-resident world designed to develop and run on Apple Silicon-class unified memory as well as discrete GPUs; design must not assume discrete-only atomics or bandwidth headroom.
- **Independence:** Substrate stands alone with no LLM requirement.

## Deferred design decisions

- First delivery slice depth: which matter behaviors, dressing, and generation passes ship before others (including voxel scale tradeoffs and LOD strategy).
- Exact crate split inside the workspace boundary; public API surface shape and verb set.
- Whether and how far multiplayer-authoritative deployment is designed for vs only left architecturally open.
- Harness-only choices if a walkable world ships: controller, camera, seed composition, debug presentation, benchmarks, and performance gates.
- Algorithms, data layouts, brick/payload packing, meshing technique selection, and acceptance thresholds.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repo **walkable-world validation harness** a **required current delivery**, or only **permitted** beside the substrate crates?

- **Proposed safe answer:** **Required** as an adjacent validation executable that proves generation, streaming, meshing, editing, collision against matter truth, persistence, and basic performance through **public APIs only**—without treating its controller, character, seed route, or presentation as product scope.
- **If different:** If only **permitted**, current identity is library-first and a walkable demo is optional; design need not plan a shippable harness. If **required**, delivery and CI must include a harness consumer, but still must not absorb game- or demo-specific content into the substrate crates.

## Seed synthesis

- **README.md** — Names Moria as a reusable GPU-resident voxel-world Rust substrate and frames the walkable-world executable as consumer/validation, not game layer.
- **docs/seeds/project-boundary.md** — Binding identity and boundary: substrate crate(s) only; game and System/LLM/spell/gas/combat/AI/building layers out of scope; harness must use public interfaces.
- **docs/seeds/product-one-seed.md** — First vertical proof of a material walkable world and dig/place honesty; supplies non-goals and motivation for a harness-shaped demo without transferring its content or controls into substrate identity.
- **docs/seeds/voxel-world-substrate.md** — Long-horizon substrate responsibilities and design goals (normal-looking material world, deep Z, geology generation, sparsity, layering rules); mechanisms and milestone catalogs stay out of this brief.
