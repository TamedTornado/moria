# Moria — Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**: a Rust crate or small family of tightly scoped Rust crates that owns the world layer for continuous three-dimensional play. Downstream games consume it through public interfaces; they do not live in this repository.

The substrate’s job is the material world: generation, mutation, observation, events, voxel-backed objects, reactive matter behavior, meshing, surface dressing, persistence, and activity-oriented sparse streaming—exposed so upper layers never touch voxels directly. Consumers issue **commands in** and receive a **stale mirror plus events out**. Observation is eventual; the contract does not require a synchronous authoritative mirror.

This repository may also ship a **walkable-world executable**. That executable is a **separate consumer and validation harness**, not part of the substrate’s owned surface. It must use the same public APIs an external game would use—no privileged or game-specific paths into storage.

**Product One** (“The Walkable World”) is the **first delivery**, not the product identity. It is a substrate capability slice *plus* an adjacent harness that collectively prove the substrate works: one curated generated region, a third-person character who can traverse it, dig/place as mutability proof, and benchmarks. The harness and its demo content are validation fixtures; the long-term product remains the reusable crate stack.

---

## Purpose

1. **Own the world layer.** Provide a reusable substrate that any future game mode (sandbox, fortress/colony, descent, ARPG) can sit on without forking matter, generation, or mutation code.
2. **Prove it with Product One.** Deliver a public, playable, measurable first artifact—substrate slice plus harness—that makes one claim undeniable: *this is not a heightmap with props; it is a fully material world, and it looks good.*
3. **Keep the consumer boundary honest.** Enforce crate and API separation so the harness never becomes a privileged second product surface.
4. **Decide open engineering questions with numbers.** Use Product One’s region and benchmarks as the measurement bed (voxel size, LOD, object-layer scale), not as product-identity commitments.

---

## Boundary

### Substrate (current product — Moria)

| In the substrate | Out of this repository (consumer / game layers) |
|---|---|
| Seed-and-coordinate geology generation with lazy independent materialization, POI metadata, and material palettes | Game rules, progression, combat, stats, AI / agent labor |
| Matter truth and its views: meshing, surface dressing, voxel-backed objects with full lifecycle, reactive fluids/granular/integrity/ambient ecology | The System / LLM, spells, gas pricing policy, intent |
| Mutation, observation, and events: commands in; stale mirror and events out; nothing above the matter layer touches voxels directly | Building UI, blueprints, mechanisms, work orders, designation gameplay |
| Mutation-safe derived navigation; persistent world and object state (truth = generation + deltas + journals) | Multiplayer as a shipped feature; any concrete game mode, characters, content IP, or campaign |
| Activity-oriented sparse streaming around anchors | |

A **Cargo workspace** must separate reusable substrate crates from the harness executable. The precise crate split is a technical-design decision; the consumer boundary is not optional.

Compatibility seams may be designed where substrate requirements demand them (e.g. gas as an injectable policy plug-in, System as a same-API client). Those layers must not be implemented here.

**Portability** is an enduring substrate constraint: load-bearing paths stay on a portable GPU abstraction (wgpu/WGSL class) with cross-backend portability retained; no native Metal-only fork in load-bearing layers.

### First delivery (Product One — substrate slice + adjacent harness)

Product One **narrows what is built first**; it does not redefine what the substrate owns.

**Substrate slice in Product One**
- Generation for one curated region ships as designed (continent pass may be stubbed to region parameters).
- Matter partial: sparse GPU-resident storage, lazy materialization, incremental meshing, grass/clutter dressing, voxel-object placement/registration/rendering, static water bodies only.
- **Deferred to full substrate (not Product One runtime):** reactive fluid redistribution and material reactions; granular settle; structural failure and cascades; fire/CA-class ambient ecology; seasons/weather/growth beyond a harness time-of-day control; rigid conversion / object fall-and-reconcile (tree felling is stretch only for Product One).
- API sliver: dig/place verbs and mirror queries, establishing the no-direct-voxel-touch boundary. The complete substrate still owns the broader command / stale-mirror / event contract.

**Adjacent harness (not substrate-owned)**
- Curated seed parameters for one generated region (authored *seed*, not authored *world*).
- Third-person character, free-orbit camera, traversal route, debug presentation (dig/place keys, visualization toggles, time-of-day slider).
- Scripted benchmark workload and performance reporting through milestone 6.
- Tree felling remains **stretch**, not the completion bar.

---

## Required product-level outcomes

### Substrate outcomes (product altitude — authorized for Moria)

These are what the substrate must be *for*, even where Product One only partially exercises them:

1. **Normal look over voxel truth.** Terrain and structures present as a smooth, material-aware mesh *view*; physics, queries, and gameplay run against voxel truth. The mesh is never authoritative and never saved.

2. **Matter vs dressing.** Everything that can burn, break, or block is voxel-backed (terrain, structures, voxel objects). Non-voxel dressing (grass, flowers, clutter) is derived from and anchored to voxel state and stays synchronized with it—not independent voxel identity for every visible pixel.

3. **Mutable everywhere, continuous deep Z.** Any voxel can be destroyed, moved, or placed; the underground is first-class content on the same representation, not a skybox floor.

4. **Deterministic, lazy generation.** Generation is a pure function of world seed and coordinates so any region of matter can materialize independently and lazily; untouched world stays cheap.

5. **GPU-resident command / stale-mirror / event contract.** Upper layers issue commands into GPU-resident state. They observe through a **stale (eventual) mirror** and through **events**, not through a synchronous authoritative live mirror or by reading voxel storage directly. Staleness is part of the contract, not a temporary optimization.

6. **Voxel-object lifecycle.** Interactable objects (trees, boulders, and kin) are full participants in the matter system: they can convert into moving matter, fall or break under physics, and reconcile back with voxel truth where they land or shatter. Growth of living objects over time is also substrate behavior. Product One may ship placement and rendering only, with rigid conversion as stretch—but the substrate mandate includes the full lifecycle.

7. **Reactive world behavior.** Disturbed water can redistribute and react with materials (quench, wet, transform on contact with heat/magma-class matter, and similar consumer-visible interactions). Granular matter settles when unsupported. Unsupported solid matter fails according to material support and can cascade. A thin but required ambient layer—time of day, seasons, weather, wetness, and fire ecology—makes the natural world *behave* as well as look normal: rain wets and can extinguish; dryness and ignition enable wildfire that spreads and can be quenched. Product One excludes these runtimes; they remain full-product substrate outcomes, not optional game-layer context.

8. **Mutation-safe derived navigation and persistent state.** Nav data derives from matter and invalidates locally on dirties. World truth persists as generation + edit deltas; objects journal as needed. Streaming is activity-oriented and sparse around anchors.

9. **Reusable, portable crate stack.** Same crates support multiple consumer genres; gas and System are policy/client concerns above the substrate. Cross-backend GPU portability is retained in load-bearing layers.

### First-delivery outcomes (Product One — fused proof)

When Product One is done (through milestone 6; tree felling stretch), the **substrate slice and adjacent harness together** must make the following true. The vision does not restate feature or material inventories; it requires their **combined** proof:

1. **Fused walkable proof.** One curated generated region, traversed in third person, collectively validates: a dressed surface that reads as a normal world; voxel-object density in the field; carved static water bodies; contrasting natural terrain and stamped structure surfaces in one view; real geology underground; continuous deep traversal; sparsity/streaming under a region that cannot live as raw voxels; and dig/place mutation with incremental remesh and cut faces that read as cut earth.
2. **Collision against voxel truth.** Character collision uses occupancy/matter, not the render mesh, so the mesh remains a view.
3. **API boundary from the first commit.** Dig/place and mirror queries exist as the harness’s only path into matter, matching the external consumer contract.
4. **Exact restoration.** Save/load (seed + deltas; single slot is enough for Product One) restores the defaced world **exactly**, not merely a small delta on disk.
5. **Measurable performance.** Frame rate, dig-to-remesh latency, cold-start to walkable, GPU-resident memory under streaming, and delta-save size are product claims, with a scripted benchmark scene that reports numbers plus machine profile.

---

## Non-goals

- Shipping a game, combat loop, economy, multiplayer, or LLM-driven content system in this repository.
- Implementing System/spell/gas layers, building UI, blueprints, mechanisms, work orders, agent labor, or other consumer-owned gameplay here—even where the substrate exposes seams those layers will need.
- Treating Product One’s curated region, character, camera, debug UX, or benchmark scene as substrate-owned product surface rather than harness validation.
- Treating the walkable executable as the long-term product identity rather than a consumer of the crate.
- Importing future-game content, characters, or campaign structure into current scope; those remain example consumers the substrate must not preclude.
- Relitigating open engineering choices (exact voxel size, LOD scheme, fluid pressure depth, object spatial scaling, milestone sequencing tactics, platform-specific kernel bit-width rules) as vision questions—they belong in design after vision approval.
- Treating Product One delivery exclusions as optional for the full substrate: deferred runtimes (reactive fluids, granular settle, integrity/cascades, ambient ecology, full object fall-and-reconcile) remain authorized substrate outcomes.

**Product One delivery exclusions** (formats/seams may anticipate; runtime not required for first ship): CA/fire, fluid flow beyond static bodies, integrity, granular settle, rigid conversion / tree felling except as stretch, embedded scripting language, multi-save versioning, weather/seasons/growth beyond a fixed time-of-day control in the harness.

---

## Unresolved questions for humans

None. Product identity (substrate vs game), Product One completion bar (through benchmarks; tree felling stretch), and authored-seed vs authored-world for the harness region are settled by the supplied seeds.

---

## What each seed contributed

| Seed | Role in this vision |
|---|---|
| **README.md** | Names Moria as the reusable GPU-resident substrate; walkable executable is a separate consumer/validation harness for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer. |
| **project-boundary.md** | Binding product identity: substrate crate(s) only; game separate; harness on public APIs; game/System/LLM/spell/gas/combat/AI/building out of scope. **Cargo workspace** boundary required; only the precise crate split is design. |
| **product-one-seed.md** | Binding *first delivery*: Product One statement and non-goals; layer cut (gen full, matter partial, API dig/place + mirror sliver); harness player/camera/debug as proof vehicle; fused seed-region proof points; performance targets and exact save/load restore; milestones through 6 with tree felling stretch; authored seed not authored world. Enduring crate constraint: portable GPU stack, no Metal-only load-bearing path. Does **not** redefine the substrate’s full mandate. |
| **voxel-world-substrate.md** | Binding substrate mandate at product altitude: design goals; matter vs dressing; **commands in, stale mirror + events out**; generation determinism and lazy materialization; full voxel-object lifecycle (fall, break, re-voxelize, growth); reactive fluids, granular settle, structural failure/cascades; thin required ambient (time, seasons, weather, wetness, fire ecology); mutation-safe nav; persistence and sparse streaming; layering rules (nothing above matter touches voxels; gas policy and System are above). Product One selects a first slice; omitted runtimes remain substrate-owned outcomes, not optional game context. Multi-game examples and low-level engineering open questions stay as rationale and design handoff, not current inventory. |

### Intentionally omitted from current vision

- Concrete future games (System ARPG, DF-style fortress, Moria-style descent) as products of this repo—kept only as consumers the substrate must not preclude.
- Game-owned layers: System attachment authoring, spells, gas policy, combat, AI/agent labor, building UI, blueprints, mechanisms, work orders, rooms/economy hooks as gameplay.
- Storage layouts, algorithm catalogs, simulation pass inventories, platform-specific atomic-width rules, and open engineering questions (LOD strategy, fluid pressure solve, object spatial index, 25cm vs 12.5cm final call)—belong in technical design after vision approval.
- Feature/material inventories and debug-key lists from Product One—fixtures for the fused harness proof, not vision identity.

---

## Summary sentence

**Moria is the reusable GPU-resident voxel-world substrate—matter, generation, mutation, and a reactive natural world behind a command / stale-mirror / event boundary; Product One is the first proof, a substrate slice plus walkable harness that looks like a natural world and digs like matter, while games live elsewhere.**
