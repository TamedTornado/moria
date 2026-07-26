# Moria — Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**: a Rust crate (or small family of tightly scoped crates) that owns the world layer for continuous three-dimensional play. Downstream games consume it through public interfaces; they do not live in this repository.

The substrate’s job is matter, physics-facing world behavior, mutation, observation, events, object behavior, fluids, integrity, granular behavior, meshing, surface dressing, generation, and activity-oriented sparse streaming—exposed so upper layers never touch voxels directly. Consumers issue commands into GPU-resident state and read a mirror plus events out.

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
| Generation: seed-and-coordinate geology pipeline, columns, lazy independent brick materialization, POI metadata, material palette | Game rules, progression, combat, stats, AI / agent labor |
| Matter: brick pool, homogeneous sentinels, CA-capable format and sim hooks, multi-tier fluids, integrity, granular settle, particle/rigid coupling, meshing, dressing, voxel objects | The System / LLM, spells, gas pricing policy, intent |
| Mutation, observation, and events: command path into GPU-resident state; mirror queries and events out; nothing above matter touches voxels directly | Building UI, blueprints, mechanisms, work orders, designation gameplay |
| Derived navigation data that stays mutation-safe; persistent world and object state (truth = generation + deltas + journals) | Multiplayer as a shipped feature; any concrete game mode, characters, content IP, or campaign |
| Activity-oriented sparse streaming (rings around anchors) | |

**Cargo workspace** (or equivalent) must separate reusable substrate crates from the harness executable. The precise crate graph is a design decision; the consumer boundary is not.

Compatibility seams may be designed where substrate requirements demand them (e.g. gas as an injectable policy plug-in, System as a same-API client). Those layers must not be implemented here.

### First delivery (Product One — substrate slice + adjacent harness)

Product One **narrows what is built first**; it does not redefine what the substrate owns.

**Substrate slice in Product One**
- Generation layer — full for one curated region (continent pass may be stubbed to region parameters; columns, strata, caves, ore, lazy materialization, POI metadata ship as designed).
- Matter layer — partial: brick pool, sentinels, lazy materialization, GPU meshing (dirty-brick incremental), grass/clutter dressing, voxel-object placement/registration/rendering, static tier-1 water. **Not** in Product One: CA, fire, fluid flow (tier 2/3), integrity, granular settle, rigid conversion of objects (tree felling is stretch only).
- Script/API — sliver: dig/place verbs and mirror queries, establishing the no-direct-voxel-touch boundary. The complete substrate still owns the broader mutation, observation, and event contract beyond this first sliver.

**Adjacent harness (not substrate-owned)**
- Curated seed parameters for one generated region (authored *seed*, not authored *world*).
- Third-person character, free-orbit camera, traversal route, debug presentation (dig/place keys, visualization toggles, time-of-day slider).
- Scripted benchmark workload and performance reporting through milestone 6.
- Tree felling remains **stretch**, not the completion bar.

---

## Required product-level outcomes

### Substrate outcomes (product altitude — authorized for Moria)

These are what the substrate must be *for*, even where Product One only partially exercises them:

1. **Normal look over voxel truth.** Terrain and structures extract as a smooth mesh view (surface nets / dual contouring class); physics, queries, and gameplay run against voxel truth. Mesh is never authoritative and never saved.
2. **Matter vs dressing.** Everything that can burn, break, or block is voxel-backed (terrain, structures, voxel objects). Non-voxel dressing (grass, flowers, clutter) is derived from and anchored to voxel state and must stay synchronized with it—not asserted as independent voxel identity for every visible pixel.
3. **Mutable everywhere, continuous deep Z.** Any voxel can be destroyed, moved, or placed; the underground is first-class content on the same representation, not a skybox floor.
4. **Deterministic, lazy generation.** Generation is a pure function of world seed and coordinates so any brick can materialize independently and lazily; untouched world stays cheap (column index + coarse maps + homogeneous sentinels).
5. **GPU-resident consumer contract.** Upper layers issue commands into GPU-resident state and consume mirror queries plus events; they do not touch voxel storage directly. Access need not be synchronous or raw.
6. **Interactive voxel objects and reactive/ambient matter.** Objects (trees, boulders, and kin) participate in the matter system; the substrate owns fluids, integrity, granular behavior, fire/CA-class ambient rules, and related physics-facing behavior as substrate capabilities (not as game modes).
7. **Mutation-safe derived navigation and persistent state.** Nav data derives from bricks and invalidates locally on dirties. World truth persists as generation + edit deltas; objects/entities journal as needed. Streaming is activity-oriented and sparse around anchors.
8. **Reusable without game lock-in.** Same crate stack supports multiple consumer genres; gas and System are policy/client concerns above the substrate, not substrate features.

### First-delivery outcomes (Product One — fused proof)

When Product One is done (through milestone 6; tree felling stretch), the **substrate slice and adjacent harness together** must make the following true. The vision does not restate feature or material inventories; it requires their **combined** proof:

1. **Fused walkable proof.** One curated generated region, traversed in third person, collectively validates: a dressed surface that reads as a normal world; voxel-object density in the field; carved static water bodies; contrasting natural terrain and stamped structure surfaces in one view; real geology underground; continuous deep traversal; sparsity/streaming under a region that cannot live as raw voxels; and dig/place mutation with incremental remesh and cut faces that read as cut earth.
2. **Collision against voxel truth.** Character collision uses occupancy/matter, not the render mesh, so the mesh remains a view.
3. **API boundary from the first commit.** Dig/place and mirror queries exist as the harness’s only path into matter, matching the external consumer contract.
4. **Exact restoration.** Save/load (seed + deltas; single slot is enough for Product One) restores the defaced world **exactly**, not merely a small delta on disk.
5. **Measurable performance.** Frame rate, dig-to-remesh latency, cold-start to walkable, GPU-resident memory under streaming, and delta-save size are product claims, with a scripted benchmark scene that reports numbers plus machine profile.
6. **Portable GPU stack.** Load-bearing work stays on portable GPU abstraction (wgpu/WGSL); design respects primary dev-platform constraints (e.g. no dependence on 64-bit buffer atomics).

---

## Non-goals

- Shipping a game, combat loop, economy, multiplayer, or LLM-driven content system in this repository.
- Implementing System/spell/gas layers, building UI, blueprints, mechanisms, work orders, agent labor, or other consumer-owned gameplay here—even where the substrate exposes seams those layers will need.
- Treating Product One’s curated region, character, camera, debug UX, or benchmark scene as substrate-owned product surface rather than harness validation.
- Treating the walkable executable as the long-term product identity rather than a consumer of the crate.
- Importing future-game content, characters, or campaign structure into current scope; those remain example consumers the substrate must not preclude.
- Relitigating open engineering choices (exact voxel size, LOD scheme, fluid pressure depth, object spatial scaling, milestone sequencing tactics) as vision questions—they belong in design after vision approval.

**Product One delivery exclusions** (formats/seams may anticipate; runtime not required for first ship): CA/fire, fluid flow beyond static tier-1 bodies, integrity, granular settle, rigid conversion / tree felling except as stretch, embedded scripting language, multi-save versioning, weather/seasons/growth beyond a fixed time-of-day control in the harness.

---

## Unresolved questions for humans

None. Product identity (substrate vs game), Product One completion bar (through benchmarks; tree felling stretch), and authored-seed vs authored-world for the harness region are settled by the supplied seeds.

---

## What each seed contributed

| Seed | Role in this vision |
|---|---|
| **README.md** | Names Moria as the reusable GPU-resident substrate; walkable executable is a separate consumer/validation harness for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer. |
| **project-boundary.md** | Binding product identity: substrate crate(s) only; game separate; harness on public APIs; game/System/LLM/spell/gas/combat/AI/building out of scope. Workspace boundary required; crate split is design. |
| **product-one-seed.md** | Binding *first delivery*: Product One statement and non-goals; layer cut (gen full, matter partial, API dig/place + mirror sliver); harness player/camera/debug as proof vehicle; fused seed-region proof points; performance targets and exact save/load restore; milestones through 6 with tree felling stretch; authored seed not authored world. Does **not** redefine the substrate’s full mandate. |
| **voxel-world-substrate.md** | Binding substrate mandate at product altitude: design goals; matter vs dressing; GPU command/mirror/event contract; generation determinism and lazy materialization; objects, fluids, integrity, granular, ambient matter; mutation-safe nav; persistence and sparse streaming; layering rules (nothing above matter touches voxels; gas policy and System are above). Product One selects a first slice; omitted runtime features remain substrate-owned outcomes, not optional game context. Multi-game examples and low-level engineering open questions stay as rationale and design handoff, not current inventory. |

### Intentionally omitted from current vision

- Concrete future games (System ARPG, DF-style fortress, Moria-style descent) as products of this repo—kept only as consumers the substrate must not preclude.
- Game-owned layers: System attachment authoring, spells, gas policy, combat, AI/agent labor, building UI, blueprints, mechanisms, work orders, rooms/economy hooks as gameplay.
- Low-level payload bit layouts, exact meshing algorithm choice, and open engineering questions (LOD strategy, fluid pressure solve, object spatial index, 25cm vs 12.5cm final call)—belong in technical design after vision approval.
- Feature/material inventories and debug-key lists from Product One—fixtures for the fused harness proof, not vision identity.

---

## Summary sentence

**Moria is the reusable GPU-resident voxel-world substrate—matter, generation, mutation, observation, and related world behavior behind a command/mirror boundary; Product One is the first proof, a substrate slice plus walkable harness that looks like a natural world and digs like matter, while games live elsewhere.**
