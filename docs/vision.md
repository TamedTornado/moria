# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates). That substrate is the product of this repository.

Its first product-shaped delivery is **Product One — “The Walkable World”**: one generated natural region plus a third-person character who can traverse it, with dig/place available as debug proof of material mutability. That walkable executable is a **consumer and validation harness**, not a game layer: it must use the same public substrate interfaces any external game would use.

This repository does **not** ship the eventual game(s). Games are downstream consumers of the substrate. Product One is the first slice that proves and benchmarks the substrate; it does not exhaust the substrate’s required outcomes.

---

## Purpose

Make one claim undeniable and reusable:

> This is not a heightmap with props. It is a fully material world that looks like a normal outdoor place, supports continuous 3D traversal (surface to underground), and remains honest under mutation — and it exists as a portable engine layer other products can build on.

Product One exists to prove that claim in a walkable, demoable form; to force the public API boundary early; and to leave a measured baseline so later substrate work can regress against real numbers rather than whiteboard intent.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| Voxel matter: storage, generation, meshing/view, mutation, collision against voxel truth | Game rules, progression, combat, stats, economy |
| Matter physics and material behavior the substrate owns (see substrate outcomes below) | Game-layer interpretation of that behavior (damage tables, progression gates, quests) |
| Streaming, sparse residency, edit-delta persistence, durable world/object state | The System / LLM content pipeline as a product feature |
| Walkable-world harness (controller, camera, debug tools) consuming public APIs | Spells, gas/pricing policy, intent systems |
| Formats and seams future layers need (materials, POI metadata, object registration, verb/query shape) | AI agents, fortress labor, multiplayer services |
| | Building UI, blueprints-as-gameplay, mechanisms, work orders |

**Layering rule (product-level):** nothing above the substrate touches voxels directly. Games and harnesses go through verbs, queries, and events. Gas, policy, and “System” authorship are injectors or clients of that boundary — not substrate features to implement here.

**Rules distinction:** *game* rules (combat, economy, AI, labor, spells) live outside this repository. *Material* rules the world must obey (mutability, reactions, granular settle, fluids, ambient material change, structural support/failure) are substrate obligations, even when Product One does not yet run them.

**Workspace rule:** keep a Cargo (or equivalent) boundary between reusable substrate crates and the validation harness so the harness cannot grow privileged, game-specific paths.

**Portability:** the substrate is a portable product. Load-bearing layers must not become a machine-specific or single-backend fork; cross-backend portability is part of the crate’s point. Hardware-specific gates and provisional targets belong to measurement and design, not to product identity.

---

## Required product-level outcomes

Outcomes are split so Product One’s first slice stays sharp while the substrate’s fuller mandate stays obligatory rather than optional “seams.”

### Substrate product (Moria) — eventual obligations

These are fused, consumer-visible outcome families the substrate product must deliver. They are not a mechanism inventory and need not all ship in Product One, but they are not mere compatibility possibilities.

1. **Reads as a normal world.** Rolling terrain, forest, water, cliffs, and surface dressing that sell “outdoor place,” not “cube sandbox.” The voxel grid is truth; the render mesh is a regenerated view.
2. **Mutable material world.** Destroy, move, and place matter across the full world volume. Visible substance that can burn, break, or block is voxel-backed matter; non-voxel dressing (grass, ground clutter) is anchored to and derived from that truth, not granted individual voxel identity.
3. **Deep Z is first-class.** Underground volume is content (caves, strata, buried matter), not a painted floor under a skybox.
4. **Matter layer behavior.** Dynamic voxel objects; material reactions; granular behavior; richer fluids beyond static bodies; ambient material change; structural support and failure. These are substrate-owned material outcomes, not game features to invent later outside the matter model.
5. **Mutation-safe spatial world.** Spatial and navigation data stay valid under mutation; multi-anchor streaming; durable world and object state so scars and objects survive across sessions.
6. **Substrate, not game.** The same crate surface can support sandbox, fortress-style building, descent adventure, or ARPG-style play *without* those games living in this repo. Matter, physics, queries, mutation, and persistence are the product; game rules are not.
7. **GPU-resident sparsity is load-bearing.** Large regions must not require raw full-volume residency. Homogeneous empty/solid regions, lazy materialization, and streaming are part of the product claim, not deferred polish.
8. **Portable crate surface.** Consumable as a reusable library across intended graphics backends without specializing the product into a single-machine fork.

### Product One — first slice that must be true now

These outcomes define “done” for the walkable-world delivery. They prove and measure the substrate; they do not replace the substrate outcomes above.

1. **Walkable natural region.** A curated generated region with continuous third-person traversal from surface (including height extremes) into walkable underground.
2. **Mutation proof.** Dig and place (at least as debug tools) mutate real matter; remeshing and collision stay consistent with voxel truth. Without this, the demo is interchangeable with any static terrain scene.
3. **Generation slice with authored placement.** Procedural geology ships with POI metadata and can incorporate metadata-directed authored voxel placement (a stamped structure in the harness), so stamp/prefab placement is exercised once alongside organic terrain.
4. **Dressing vs matter distinction in the demo.** Surface grass/clutter dressing is present and derived from voxel truth; trees, boulders, and similar interactable objects register as voxel-backed matter (placement/render; felling not required).
5. **Static water bodies.** Lakes and a river channel with a water surface (no flow simulation in this slice).
6. **Exact persistence proof.** Delta save/load restores the mutated voxel truth **exactly** (seed + deltas are the mechanism; exact restoration is the product claim).
7. **Harness proves the consumer boundary.** Controller, camera, debug tools, and the run all consume public substrate interfaces only.
8. **Credibility is measurable.** Walkable performance, dig-to-remesh latency, cold start, memory under streaming, and save size after defacement are part of what “done” means, including a repeatable benchmark path.

---

## Non-goals (current scope)

**Out of this repository entirely (game / adjacent products):**

- Any playable *game*: combat, entities beyond the player avatar, AI, quests, economy.
- System / LLM integration as a built feature.
- Gas, pricing, spells, intent.
- Building UI, blueprints-as-gameplay, doors/pumps/levers, fortress designations and labor.
- Full multiplayer services, content pipelines for authored campaigns, or shipping a commercial title from this repo.

**Out of Product One’s first slice** (substrate may still owe these later; formats may reserve fields):

- Fluid *simulation* beyond static bodies.
- Weather, seasons, growth, fire ecology, structural cave-ins, granular settle as running systems.
- Voxel-object felling / rigid-body conversion (optional stretch only; not required for Product One “done”).
- Full multi-anchor fortress-style pinning and rich object journals beyond what exact voxel delta restoration needs.

---

## Unresolved questions for humans

None. Seeds agree on product identity (substrate + walkable harness; games out), on what Product One must prove, and on which fuller matter and world outcomes belong to the substrate rather than to downstream games. Open technical choices (e.g. voxel size final call, LOD strategy) are measurement- and design-owned, not product-identity ambiguities.

---

## Seed contributions (provenance)

| Seed | What it contributed to this vision |
|---|---|
| **README.md** | Product name and one-line identity: GPU-resident voxel substrate as a Rust crate; walkable executable is harness, not game. |
| **project-boundary.md** | Hard repo boundary: substrate is the product; games and System/spell/gas/combat/AI/building layers are out; harness must share public APIs; workspace split is mandatory. |
| **product-one-seed.md** | First delivery shape (“Walkable World”), proof claim, curated natural region as *validation content*, dig/place as proof, POI/stamped placement in the generation slice, dressing vs voxel objects, exact restore on load, portability of the crate, performance/credibility outcomes, explicit Product One non-goals. Seed-world checklist and material lists are harness content examples, not product identity. |
| **voxel-world-substrate.md** | Substrate design goals and matter/world outcome families (full mutability, matter physics, dynamic objects, reactions, granular/fluids/ambient/integrity, mutation-safe nav, multi-anchor streaming, durable state). Layering (matter vs generation vs API vs game) and the dressing-vs-voxel-backed distinction. Mechanism inventories, look-strategy options, and open technical questions stay reference for design — not imported as current gameplay scope. |

**Intentionally omitted from current vision as product content:** ARPG/System fantasy, DF fortress labor and machinery detail, Moria-style descent progression as game content, specific worldgen art direction beyond “natural material world,” viral milestone marketing, M4/machine-specific gates, and step-by-step build order. Those inform *why* the substrate exists and *what outcomes it must eventually own*; they are not Product One’s identity or a feature inventory.
