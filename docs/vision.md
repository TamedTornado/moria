# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. Downstream games and tools consume it; they are not the product. This repository also delivers a **walkable-world executable** as a required first adjacent slice: a validation harness using the same public interfaces an external game would use, proving generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer or product identity.

## Purpose

Moria exists so adventure, fortress, sandbox, and related games can share one foundation: a natural-looking, fully material 3D world whose voxel truth supports dig, place, deep underground play, and matter-faithful simulation—without embedding any particular game’s rules, economy, or presentation. The substrate has **no LLM or System dependency**.

## Product boundary

**In product:** geology-first generation; sparse GPU-resident matter; smooth mesh views of voxel truth; a GPU-resident consumer contract (commands in with a potentially stale mirror, events and queries out, mutation only through that contract); matter-consistent vegetation and dressing; interactable voxel objects with a full physical lifecycle; fluids, integrity, and related shared world systems; living-world temporal/seasonal/weather/fire-ecology behavior tied to matter; mutation-coherent navigation; persistence and streaming of substrate-owned mutable truth; integration seams without game policy.

**Adjacent, not identity:** a walkable-world executable is a **required first adjacent delivery**. It must use the same public substrate interfaces as an external game and must not own privileged or game-specific paths. Character, camera, route, content, debug controls, platform, machine targets, and performance gates are harness-owned. First-slice scope is narrower than the full substrate.

**Out of repository product:** the actual game(s); System/LLM; spells; gas pricing; combat; AI; and game building layers (building UI, blueprints-as-gameplay, mechanisms-as-game systems, work orders, room economy). Compatibility seams may be designed where needed; those layers are not implemented here. Excluding AI does not move navigation to consumers.

## Required product outcomes

1. **Reusable engine layer with a GPU-resident consumer contract.** Games and the walkable harness receive matter, queries, mutation, and related world physics through a public crate API: commands in against a mirror that may be stale, events out, plus queries and voxel mutation only through that contract. Nothing above the matter boundary touches voxels directly. Consumers observe world state under that staleness model without privileged back doors. Gas or labor costs are consumer policy.

2. **Required adjacent proof (not product identity).** A walkable-world validation executable is required adjacent delivery. Through the public contract it must exercise: generation, streaming, meshing, editing (dig/place), collision against voxel truth (not the render mesh), continuous traversal including deep Z, persistence of edit state, and reproducible performance evidence. Harness content, presentation, controller, route, workloads, machines, and gates stay harness-owned.

3. **Natural world, voxel truth, mutable deep Z.** Surface worlds read as ordinary terrain—not a cube aesthetic—while the voxel grid remains the authority for occupancy, materials, and edits. Rendered meshes are regenerated views, never saved as truth. Any volume can be destroyed, eroded, or filled. Continuous 3D underground (caves, strata, ore, depth) is real content, not a painted floor.

4. **Geology-first, deterministic, independently materializable generation.** Worlds are generated as geology so dig-down stays honest. Generation stages are functions of coordinates and world seed so bricks materialize independently and lazily; the same seed plus edit deltas can recreate a world. Empty or uniform regions stay cheap so large regions are feasible. Reusable generation includes columns, strata, caves, ore, lazy materialization, and POI metadata; a first adjacent slice may stub the continent pass to one curated region without shrinking that outcome.

5. **Matter systems, voxel-object lifecycle, and living world.** Interactable world objects can burn, break, block, move or fall, grow, and remain part of voxel truth; tree falling is a full-substrate outcome even when a first adjacent slice defers felling and rigid conversion. Surface dressing stays coherent with matter under edit and reaction. The substrate owns fluids, support and collapse, granular behavior, and material reaction. The natural world behaves coherently over time and weather—day/night, seasons, weather, and fire ecology—with vegetation and ecology responding through matter. Living-world and advanced matter systems are full-substrate outcomes outside the first adjacent slice (excludes weather/seasons/growth, CA/fire, dynamic fluids, integrity, granular settle, and felling).

6. **Mutation-coherent navigation; persistence and streaming.** Navigation is derived from voxel truth, invalidated by mutation, and supports continuous 3D traversal—a substrate spatial facility, not an AI or game-rule layer. Substrate-owned mutable truth survives across runs: world edits persist as generation seed plus edit deltas and, for the walkable first-slice save path, restore exactly on load; felled, moved, and entity object state is journaled and cross-run persistent without that exact-restoration force. Regions stream around active anchors so large worlds stay workable without keeping the whole volume always resident.

## Future products and enabling implications

Future **consumers** (not this product): a System-driven ARPG, a DF-style fortress/colony game, a Moria-style descent, pure sandbox modes. They motivate reusable geology, mutability, deep Z, integrity, fluids, navigation, living-world matter behavior, and verb/query symmetry. Their gameplay, content, presentation, controllers, characters, and acceptance scenarios stay consumer-owned. The walkable first slice is narrower than the full substrate (one-region continent stub allowed; static water only; no felling/rigid conversion, CA/fire, dynamic fluids, integrity, granular settle, or living-world weather/seasons/growth)—omissions that bound the adjacent proof, not product identity. Multiplayer readiness of a command-style API is enabling only.

## Non-goals

- Shipping a game, combat loop, stats, AI, or System/LLM features here
- Implementing spell, gas, building-as-gameplay, or economy layers here
- Treating demo content, third-person control, or trailer routes as substrate scope
- Decorative terrain not backed by mutable matter
- LLM dependency inside the substrate
- Treating first-slice exclusions as product non-goals

## Confirmed vision constraints

- Product form: **Rust crate(s)** for consumption by games and tools
- World model: **GPU-resident** voxel matter with a **commands-in / potentially stale mirror + events-out** consumer contract
- Generation: stages are **deterministic functions of coordinates and seed**, independently and lazily materializable
- Layering: substrate has **zero LLM dependency**; game rules live above
- Validation harness is **not** a privileged second implementation path
- Walkable-world executable is a **required** adjacent delivery; it is not product identity

## Deferred design decisions

- Voxel size, LOD, meshing algorithm, object-layer capacity limits
- Depth and sequence of world systems beyond settled first-slice proof coverage
- Persistence encodings, versioning, streaming policy, save formats
- Exact crate split and workspace layout for the public boundary
- Whether multiplayer authority is pursued later
- Harness-only choices: seed contents, controller, camera, benchmarks, machines

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident Rust substrate and the walkable harness as validation for generation, streaming, meshing, editing, collision, persistence, performance.
- **docs/seeds/project-boundary.md** — Binds identity to reusable Rust crate(s), excludes game layers, requires the harness under equal public interfaces, keeps navigation as substrate when excluding AI.
- **docs/seeds/voxel-world-substrate.md** — Full substrate outcomes: natural look over voxel truth, mutability, deep Z, deterministic lazy generation, command/mirror/event contract, voxel-object lifecycle (fall, growth), matter systems, living-world ecology, mutation-coherent navigation, edit deltas and object/entity journals.
- **docs/seeds/product-one-seed.md** — Required first adjacent walkable proof (continuous traversal, dig/place via API, voxel-authoritative collision, streaming, exact edit-delta restore, reproducible benchmarks), one-region continent-stub allowance, first-slice exclusions that leave broader outcomes intact.
