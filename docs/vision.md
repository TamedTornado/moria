# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate (or a small family of tightly scoped crates).

Its first product-shaped delivery is **Product One — “The Walkable World”**: one generated natural region plus a third-person character who can traverse it, with dig/place available as debug proof that everything visible is mutable voxel matter. That walkable executable is a **consumer and validation harness**, not a game layer: it must use the same public substrate interfaces any external game would use.

This repository does **not** ship the eventual game(s). Games are downstream consumers of the substrate.

---

## Purpose

Make one claim undeniable and reusable:

> This is not a heightmap with props. It is a fully material world that looks like a normal outdoor place, supports continuous 3D traversal (surface to underground), and remains honest under mutation — and it exists as an engine layer other products can build on.

Product One exists to prove that claim in a walkable, demoable form; to force the public API boundary early; and to leave a measured baseline so later substrate work can regress against real numbers rather than whiteboard intent.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| Voxel matter: storage, generation, meshing/view, mutation verbs, collision against voxel truth | Game rules, progression, combat, stats, economy |
| Streaming, sparse residency, edit-delta persistence | The System / LLM content pipeline as a product feature |
| Walkable-world harness (controller, camera, debug tools) consuming public APIs | Spells, gas/pricing policy, intent systems |
| Seams and formats that *future* layers may need (materials, POI metadata, object registration, verb/query shape) | AI agents, fortress labor, multiplayer services |
| | Building UI, blueprints-as-gameplay, mechanisms, work orders |

**Layering rule (product-level):** nothing above the substrate touches voxels directly. Games and harnesses go through verbs, queries, and events. Gas, policy, and “System” authorship are injectors or clients of that boundary — not substrate features to implement here.

**Workspace rule:** keep a Cargo (or equivalent) boundary between reusable substrate crates and the validation harness so the harness cannot grow privileged, game-specific paths.

---

## Required product-level outcomes

These are outcomes the *current product* must make true. They are not a backlog of systems.

1. **Reads as a normal world.** Rolling terrain, forest, water, cliffs, and surface dressing that sell “outdoor place,” not “cube sandbox.” Voxel grid is truth; the render mesh is a regenerated view.
2. **Mutable everywhere it claims.** Dig and place (at least as debug tools) mutate real matter; remeshing and collision stay consistent with that truth. Without this, the demo is interchangeable with any static terrain scene.
3. **Deep Z is first-class.** Underground volume is playable content (walkable cave route, geology you can hit when you dig), not a painted floor under a skybox.
4. **Substrate, not game.** The same crate surface can later support sandbox, fortress-style building, descent adventure, or ARPG-style play *without* those games living in this repo. Matter, queries, mutation, and persistence are the product; rules are not.
5. **GPU-resident sparsity is load-bearing.** Large regions must not require raw full-volume residency. Homogeneous empty/solid regions, lazy materialization, and streaming rings are part of the product claim, not deferred polish.
6. **Harness proves the consumer boundary.** A third-person run through a curated seed region exercises generation, meshing, collision-vs-voxels, Z traversal, static water bodies, surface dressing, voxel objects (trees/rocks as registered matter, not baked deco), and seed+delta reload — all through the public interface.
7. **Credibility is measurable.** Walkable performance, dig-to-remesh latency, cold start, memory under streaming, and save size after defacement are part of what “done” means for Product One, including a repeatable benchmark path.

**High-level capabilities the substrate must remain able to support later** (design seams only; not current implementation scope): active matter rules (fire, wetness, granular settle), richer fluids, structural integrity, falling/rigid voxel objects, weather/ambient sim, building placement and mechanisms, 3D nav classes, multi-anchor streaming, and multiplayer-ready command/mirror shape. Product One may ship formats and API shapes that leave room for these; it must not implement the game systems that consume them.

---

## Non-goals (current scope)

- Any playable *game*: combat, entities beyond the player avatar, AI, quests, economy.
- System / LLM integration as a built feature.
- Gas, pricing, spells, intent.
- Building UI, blueprints-as-gameplay, doors/pumps/levers, fortress designations.
- Fluid *simulation* beyond static bodies (lakes / river channel with a surface).
- Weather, seasons, growth, fire ecology, structural cave-ins, granular settle (format may reserve fields; nothing runs them in Product One).
- Voxel-object felling / rigid-body conversion (stretch only if cheap; not required for “done”).
- Full multiplayer, content pipelines for authored campaigns, or shipping a commercial title from this repo.

---

## Unresolved questions for humans

Seeds agree on product identity (substrate + walkable harness; games out). The following still affect how design should weight the first slice; they do **not** block stating what the product is.

1. **Voxel resolution baseline.** Seeds assume **25 cm** voxels (4/m) with **16³** bricks, and treat Product One’s region as the decision bed for 25 cm vs 12.5 cm. Confirm 25 cm as the working product assumption until measured otherwise.
2. **Product Two direction.** Seeds leave fortress-style toybox vs ARPG/System as a later choice. Confirm that Product One deliberately does not pick a game genre — only that the substrate must not foreclose either.
3. **Timber stretch.** Is one felled tree with rigid fall part of the approval bar for Product One, or strictly optional if physics coupling stays expensive?

No seed conflict requires resolving “what is the product?” before design proceeds.

---

## Seed contributions (provenance)

| Seed | What it contributed to this vision |
|---|---|
| **README.md** | Product name and one-line identity: GPU-resident voxel substrate as a Rust crate; walkable executable is harness, not game. |
| **project-boundary.md** | Hard repo boundary: substrate is the product; games and System/spell/gas/combat/AI/building layers are out; harness must share public APIs; workspace split is mandatory. |
| **product-one-seed.md** | First delivery shape (“Walkable World”), proof claim, curated natural region as *validation content* (not shipped game world), dig/place as proof, player traversal as harness behavior, performance/credibility outcomes, explicit non-goals and milestone intent. Seed-world checklist and material lists are treated as harness content examples, not product identity. |
| **voxel-world-substrate.md** | Long-horizon substrate capabilities and layering (matter vs generation vs API vs game). Used for purpose and “must remain able to support” outcomes. Look strategies, CA, integrity, building, nav, and full fluid tiers are **reference design context** — not imported as current gameplay or implementation scope. Open technical questions stay for design/tech planning. |

**Intentionally omitted from current vision as product content:** ARPG/System fantasy, DF fortress labor and machinery, Moria-style descent progression, specific worldgen art direction beyond “natural material world,” viral milestone marketing, and step-by-step build order. Those inform *why* the substrate exists and *what seams to preserve*; they are not this product’s identity.
