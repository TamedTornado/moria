# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). That substrate is the product of this repository.

Its first product-shaped delivery is **Product One — “The Walkable World”**: one generated natural region and a third-person character who can traverse it, with dig/place available as debug proof of material mutability. The walkable executable is a **consumer and validation harness**, not a game layer: it must use the same public substrate interfaces any external game would use.

This repository does **not** ship eventual games. Games are downstream consumers. Product One proves and benchmarks a selected substrate slice; it does not exhaust the substrate’s required outcomes.

---

## Purpose

Make one claim undeniable and reusable:

> This is not a heightmap with props. It is a fully material world that looks like a normal outdoor place, supports continuous 3D traversal (surface through underground), stays honest under mutation, and exists as a portable engine layer other products can build on.

Product One exists to prove that claim in a walkable, demoable form; to force the public API boundary from the first commits; and to leave a measured baseline so later substrate work regresses against numbers rather than whiteboard intent.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| Voxel matter: storage, generation, meshing/view, mutation, collision against voxel truth | Game rules, progression, combat, stats, economy |
| Matter behavior the substrate owns (see substrate outcomes below) | Game-layer interpretation of that behavior |
| Streaming, sparse residency, edit-delta persistence, durable world/object state | System / LLM content pipeline as a product feature |
| Walkable-world harness (controller, camera, debug tools) via public APIs | Spells, gas/pricing policy, intent systems |
| Formats and seams future layers need (materials, POI metadata, object registration, verb/query shape) | AI agents, fortress labor, multiplayer services |
| | Building UI, blueprints-as-gameplay, mechanisms, work orders |

**Layering rule:** nothing above the substrate touches voxels directly. Games and harnesses go through verbs, queries, and events. Gas, policy, and System authorship are injectors or clients of that boundary — not substrate features implemented here.

**Rules distinction:** *game* rules (combat, economy, AI, labor, spells) live outside this repository. *Material* rules the world must obey (mutability, reactions, granular settle, fluids, ambient material change, structural support/failure) are substrate obligations, even when Product One does not yet run them.

**Workspace rule:** keep a Cargo (or equivalent) boundary between reusable substrate crates and the validation harness so the harness cannot grow privileged, game-specific paths.

**Portability:** load-bearing layers must not become a machine-specific or single-backend fork. Cross-backend portability is part of the crate’s point; hardware-specific gates belong to measurement and design, not product identity.

---

## Required product-level outcomes

### Substrate product (Moria) — full obligations

These are consumer-visible outcome families the substrate must deliver. They need not all ship in Product One, but they are not optional “maybe later seams.”

1. **Reads as a normal world.** Rolling terrain, forest, water, cliffs, and surface dressing that sell “outdoor place,” not “cube sandbox.” The voxel grid is truth; the render mesh is a regenerated view.
2. **Mutable material world.** Destroy, move, and place matter across the full volume. Substance that can burn, break, or block is voxel-backed; non-voxel dressing is anchored to and derived from that truth.
3. **Deep Z is first-class.** Underground volume is content (caves, strata, buried matter), not a painted floor under a skybox.
4. **Matter-layer behavior.** Dynamic voxel objects; material reactions; granular behavior; richer fluids beyond static bodies; ambient material change; structural support and failure — substrate-owned material outcomes, not game features invented outside the matter model.
5. **Mutation-safe spatial world.** Spatial and navigation data stay valid under mutation; multi-anchor streaming; durable world and object state so scars and objects survive sessions.
6. **Substrate, not game.** The same crate surface can support sandbox, fortress-style building, descent adventure, or ARPG-style play *without* those games living in this repo.
7. **GPU-resident sparsity is load-bearing.** Large regions must not require raw full-volume residency. Homogeneous empty/solid regions, lazy materialization, and streaming are part of the product claim.
8. **Portable crate surface.** Consumable as a reusable library across intended graphics backends without specializing into a single-machine fork.

### Product One — first slice that must be true now

These define “done” for the walkable-world delivery. They prove and measure the substrate; they do not replace the obligations above.

1. **Walkable natural region.** A curated generated region with continuous third-person traversal from surface extremes into walkable underground.
2. **Mutation proof.** Dig and place (at least as debug tools) mutate real matter; remeshing and collision stay consistent with voxel truth. Without this, the demo is interchangeable with any static terrain scene.
3. **Generation slice with authored placement.** Procedural geology ships with POI metadata and can place a stamped structure once so stamp/prefab placement sits beside organic terrain.
4. **Dressing vs matter in the demo.** Grass/clutter dressing is present and derived from voxel truth; trees, boulders, and similar register as voxel-backed matter (placement/render; felling not required).
5. **Static water bodies.** Lakes and a river channel with a water surface (no flow simulation in this slice).
6. **Exact persistence proof.** Delta save/load restores mutated voxel truth exactly (seed + deltas are the mechanism; exact restoration is the claim).
7. **Harness proves the consumer boundary.** Controller, camera, debug tools, and the run consume public substrate interfaces only.
8. **Credibility is measurable.** Walkable performance, dig-to-remesh latency, cold start, memory under streaming, and save size after defacement are part of “done,” including a repeatable benchmark path.

---

## Non-goals

**Out of this repository entirely (game / adjacent products):**

- Any playable *game*: combat, entities beyond the player avatar, AI, quests, economy
- System / LLM integration as a built feature
- Gas, pricing, spells, intent
- Building UI, blueprints-as-gameplay, doors/pumps/levers, fortress designations and labor
- Full multiplayer services, authored campaign pipelines, or a commercial title from this repo

**Out of Product One’s first slice** (substrate may still owe these later; formats may reserve fields):

- Fluid *simulation* beyond static bodies
- Weather, seasons, growth, fire ecology, structural cave-ins, granular settle as running systems
- Voxel-object felling / rigid-body conversion (optional stretch only; not required for Product One “done”)
- Full multi-anchor fortress-style pinning and rich object journals beyond what exact voxel delta restoration needs

---

## Unresolved questions for humans

None. Seeds agree on product identity (substrate + walkable harness; games out), on what Product One must prove, and on which fuller matter and world outcomes belong to the substrate rather than to downstream games. Open technical choices (voxel size, LOD strategy, solver fidelity, crate split) are measurement- and design-owned, not product-identity ambiguities.

---

## Seed contributions (provenance)

| Seed | What it contributed |
|---|---|
| **README.md** | Product name and one-line identity: GPU-resident voxel substrate as a Rust crate; walkable executable is harness, not game. |
| **project-boundary.md** | Hard repo boundary: substrate is the product; games and System/spell/gas/combat/AI/building layers are out; harness must share public APIs; workspace split is mandatory. |
| **product-one-seed.md** | First delivery shape (“Walkable World”), proof claim, curated natural region as *validation content*, dig/place as proof, POI/stamped placement, dressing vs voxel objects, exact restore on load, portability, performance/credibility outcomes, explicit Product One non-goals. Seed-world checklists and material lists are harness content examples, not product identity. |
| **voxel-world-substrate.md** | Full substrate outcome families (natural look over voxel truth, universal mutability, deep Z, matter physics, ambient material response, mutation-safe nav, multi-anchor streaming, durable state, multi-game reuse). Layering (generation / matter / API / game) and dressing-vs-voxel-backed distinction. Mechanism inventories and open technical questions stay design reference — not current gameplay scope. |

**Intentionally omitted as product content:** ARPG/System fantasy, DF fortress labor and machinery detail, Moria-style descent as game content, worldgen art direction beyond “natural material world,” viral milestone marketing, machine-specific performance gates, and step-by-step build order. Those explain *why* the substrate exists and *what outcomes it must eventually own*; they are not Product One’s identity or a feature inventory.
