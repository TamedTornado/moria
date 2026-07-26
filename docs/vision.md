# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a Rust crate or a small family of tightly scoped Rust crates. Downstream games and tools consume it as an engine layer for matter, generation, mutation, queries, navigation support, and world physics—not as a finished game.

## Purpose

Moria exists so adventure, fortress, sandbox, and related games can share one material world foundation: a landscape that reads as a normal natural surface world while remaining fully mutable voxel truth all the way down, including deep underground play. Game rules, economy, combat, and AI live above it. The substrate has no LLM or “System” dependency.

## Product boundary

**This product owns** the reusable substrate and its public integration surface: geology-backed generation; GPU-resident matter and non-authoritative views; command-mediated mutation with stale-mirror observation and events; the voxel-backed interactable versus matter-anchored dressing invariant; streaming and persistence of world plus substrate-owned object/entity state; mutation-safe navigation from voxel truth; and world-level matter behavior (fluids, granular settle, structural integrity, ambient weather/fire ecology).

**Adjacent, not identity:** a walkable-world executable may exist as a validation harness (see Q1). If present, it must use the same public interfaces available to an external game and must not own privileged paths. Character control, camera, authored route, presentation, debug chrome, workloads, and machine-specific acceptance numbers are harness concerns—not product scope.

**If that harness is delivered**, its settled first substrate slice is fixed: full generation for a curated proof region; partial matter (sparse occupancy, GPU incremental meshing, surface dressing, voxel-object placement/presentation without felling or rigid conversion, static water bodies only); dig/place verbs and mirror queries from day one; streaming; persistence with exact restore from seed plus deltas; and benchmark evidence. That cut omits active CA/fire, flowing fluids, structural integrity, granular settle, growth, and weather/seasons. Those omissions bound the first slice only—not the full substrate mandate.

**Not this product:** the actual game (downstream, not this repository); game rules; System/LLM; spells; gas/pricing; combat; AI/agent path policy; building gameplay layers (UI, blueprints, mechanisms, room/economy). Seams may be designed where substrate needs them; those layers are not implemented here.

## Required product outcomes

- **Reusable Rust substrate with command/mirror access.** External games integrate through public crate APIs. Consumers submit commands and observe a potentially stale query mirror plus events; nothing above the matter layer touches voxels directly. Any validation harness shares that contract with no privileged path.
- **Natural look, material truth, categorical objects.** Continuous natural terrain with voxels authoritative. Mesh and dressing are regenerated views—never saved truth. Everything that can burn, break, or block is voxel-backed; non-voxel dressing stays matter-anchored so it cannot desync. Physics and world-facing queries run against voxel matter, not the mesh.
- **Mutable everywhere, deep Z, full generation.** Any voxel can be destroyed, moved, or placed, with consistent remesh and queries. Underground is content: geology-first generation (strata, caves, ores, aquifers, biomes, lazy materialization, POI metadata)—not a painted heightmap underside.
- **Matter behavior and object lifecycle (full substrate).** The complete product includes multi-tier fluids (static bodies, coarser active flow, fine splash coupling), structural support and failure, granular settle, thin ambient time/weather/fire ecology, and physical lifecycles for voxel-backed objects (including growth and falling/rigid conversion). Design, tuning, and delivery sequence are open; exclusion from the first slice does not remove them. Pricing and win conditions stay out of the crate.
- **Mutation-safe navigation.** Navigation data derived from voxel truth stays valid under mutation and supports multiple continuous-3D movement classes (walk, climb, fly, burrow, swim). Agent AI and path policy remain consumer-owned.
- **Scale, streaming, durable persistence.** Sparse residency, lazy materialization, and streaming keep large regions practical. Persistence is generation plus voxel edit deltas plus object/entity journals so scars, placements, and durable object/entity state restore across runs, including cross-run world reuse. If a walkable validation slice is delivered, that cut restores exactly from seed plus deltas.

## Future products and enabling implications

Future **consumers** (not current product) include the Moria/System ARPG, a DF-style fortress or colony game, a descent roguelike, and pure sandbox tools. A walkable “product one” demo is a validation and communication artifact, not a second product identity.

**Enabling implications only:** the same public matter, generation, mutation, navigation, streaming, and persistence surface should let those games author content, attach agents, and apply rules without privileged world paths. Multiplayer-ready command/mirror separation is a design posture, not a committed shipping milestone. Gameplay, UX, controllers, characters, content, and presentation remain consumer-owned.

## Non-goals

- Shipping the actual game or game-layer systems in this repository
- Implementing System/LLM, spells, gas/pricing, combat, AI, or agent labor policy
- Building UI, blueprints, work orders, mechanisms, or room/economy layers as product features
- Treating harness demo content or machine-specific performance gates as substrate requirements
- Making the mesh or dressing authoritative for simulation or saves

## Confirmed vision constraints

- Integration form is a **Rust crate or small family of tightly scoped Rust crates**
- World representation is **GPU-resident** (matter sim, command/mirror coupling, and related GPU work—not graphics alone); every load-bearing substrate GPU layer stays on **wgpu/WGSL** with no native Metal fork ever, for crate-level Vulkan/DX12 portability
- **No privileged paths** for any consumer, including a validation harness
- **Commands in; stale mirror plus events out**; no direct voxel access above the matter layer
- **Zero LLM/System dependency**; named game layers stay out of implementation scope here
- **Voxel matter is truth**; burnable/breakable/blocking interactables are voxel-backed

## Deferred design decisions

- Precise crate split and workspace layout (boundary intent is fixed; packaging is design)
- Voxel resolution, LOD strategy, object-layer capacity, and fidelity/cost tradeoffs
- Design, tuning, and delivery sequence of full-substrate matter families (not whether they are product outcomes)
- Mechanisms for storage, meshing, streaming, delta encoding, sim scheduling
- Machine-specific performance thresholds, benchmark workloads, and harness acceptance gates
- Whether a walkable validation executable is part of current delivery (Q1)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** alongside the substrate crates, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted—not required. Moria’s identity and success criteria are the reusable substrate crates; a harness may exist and must use public APIs if built, but shipping it is not mandatory for product completeness. If built, the settled first-slice contour and proof outcomes above apply.
- **If answered differently:** Making the harness mandatory adds a required adjacent deliverable (a walkable public-API executable under that first-slice contour) without moving harness-owned controls, content, presentation, or machine gates into substrate identity.

## Seed synthesis

- **`README.md`** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and separates the walkable-world executable as consumer/harness, not a game layer.
- **`docs/seeds/project-boundary.md`** — Binds identity to substrate crate(s), excludes the game and named game layers from this repository, and fixes the public-API consumer boundary for any validation harness.
- **`docs/seeds/product-one-seed.md`** — Settles the conditional first-slice proof contour and crate-level wgpu/WGSL portability; motivates dig/place, geology, meshing-as-view, streaming, and exact seed-plus-delta restore; keeps harness controls, content, and machine gates out of product identity.
- **`docs/seeds/voxel-world-substrate.md`** — Authorizes full substrate outcome families (natural look over voxel truth, object/dressing invariant, deep-Z geology, matter sim, command/mirror access, mutation-safe multi-class navigation, generation, streaming, delta plus object/entity persistence, layered reuse without LLM) without importing game-layer features.
