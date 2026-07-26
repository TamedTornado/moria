# Moria — Product Vision

*Proposal for human approval and handoff to downstream design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped crates.

It is consumed by games and tools as a library. This repository also includes a **walkable-world executable** that is only a validation harness: it must exercise the substrate through the same public interfaces an external game would use. It is not a game layer and must not own privileged or game-specific paths.

**Product One** (“the walkable world”) pins what is built *first*: one curated generated region a player can run through in third person, with dig/place and mirror queries as the initial proof that the world is fully material matter—not a heightmap with props. Product One’s Matter layer is partial and its API layer a sliver; those exclusions **defer** substrate-owned outcome families—they do not remove them from the product mandate.

---

## Purpose

Moria exists so that downstream games can stand on a world that is:

1. **Visually continuous** — terrain reads as a natural surface world, with the voxel grid as truth rather than as the aesthetic.
2. **Mutable everywhere** — any voxel can be destroyed, moved, or placed; interactive voxel objects participate in the same material world; the render mesh is a regenerated view, never authoritative.
3. **Fully three-dimensional** — deep underground is first-class content, not a painted floor under a heightmap.
4. **Reusable** — consumers drive the world through commands in and a stale mirror plus events out, with an object model and exclusive verb/query access to voxel truth, so the same substrate can later support adventure, fortress/colony, sandbox, or other modes without forking world truth.
5. **Credible under load** — sparsity, streaming, interactive regeneration after mutation, and seed-plus-delta persistence make large regions practical; measured performance is part of the product claim.

The substrate provides **matter, generation, meshing/dressing, mutation (including movement and object lifecycle), collision against voxel truth, cellular/ambient material behavior, temporal and weather-driven ambient world state, streaming, and persistence of substrate-owned state**. Game rules live above it.

---

## Boundary

### Product mandate (this repository)

The product is the full reusable substrate. Its owned outcome families include everything Product One proves first *and* the deferred substrate capabilities below. Implementation order is staged; ownership is not.

| Concern | Role |
|---|---|
| **Generation** | Geology-first world that materializes lazily so large sparse volumes stay practical; deep-Z content (strata, caves, aquifers, ore) is real matter. |
| **Matter** | GPU-resident voxel truth; smooth presentation of that truth with interactive regeneration after edits; surface dressing derived from matter; voxel objects (trees, micro objects) as first-class interactable matter, not baked decoration. |
| **Mutation & behavior (substrate-owned)** | Destroy, move, and place voxels; object lifecycle (e.g. felling / rigid conversion of substrate objects); material and ambient cellular behavior; dynamic fluids beyond static bodies; granular settle; structural integrity and cave-ins; fire and ecological behavior. |
| **Ambient world (substrate-owned)** | Thin but present day/night and seasons, weather-driven world state (wetness, water tables, ignition events), and growth / ecological progression driven by that temporal and weather cycle—so the surface world behaves as a living ambient system, not only as static terrain plus CA on demand. |
| **Consumer interface** | Commands in; mirror plus events out; object model; exclusive verb/query access to voxel truth—no privileged back doors for games or harness. |
| **Persistence & streaming** | Seed + deltas; restore substrate-owned state (terrain edits and moved/felled substrate objects); streaming so large worlds stay practical. |
| **Validation harness** | Public-interface-only walkable proof of the substrate (character traversal, collision vs voxel truth, mutation proof, benchmarks with machine profile). |
| **Workspace split** | Cargo boundary between reusable substrate crates and the harness executable. |

### First delivery — Product One (binding proof)

Product One is the first vertical proof, not the ceiling of product scope:

| Layer | First delivery |
|---|---|
| **Generation** | Full for one curated region (columns, strata, caves, ore/aquifer bands, lazy materialization, POI/stamp metadata). Continent-scale pass may be stubbed to that region’s parameters. |
| **Matter** | Partial: brick residency and sparsity, density-aware occupancy, GPU dirty-brick meshing, grass/clutter dressing, voxel-object *placement and rendering*, static water bodies (tier-1 surfaces). CA, fire, fluids beyond static bodies, integrity, granular settle, object felling/rigid conversion, and full ambient day/night–seasons–weather–growth progression do **not** run in the first proof (format may already support them; a fixed time-of-day control is enough for the demo). |
| **Script/API** | Sliver only: dig/place verbs and mirror queries as the enforced path to voxels. Broader command set, events, full object-lifecycle verbs, and embedded scripting remain substrate mandate for later delivery—not first-proof scope. |
| **Harness** | Third-person traversal of the seed region; collision vs voxel occupancy (not vs mesh); debug mutation and inspection sufficient to prove material truth; scripted benchmarks with machine profile. |
| **Persistence (first proof)** | Single-slot seed + edit deltas; **load restores exactly**. |

### Out of scope (non-goals)

These remain outside the product entirely (not deferred substrate work):

- The actual game(s), and any game-rule layer (combat, stats, economy, intent).
- The System / LLM, spells, gas metering or pricing policy.
- AI, NPCs, and other **game** entities (voxel objects and their substrate lifecycle are *in* product mandate; game agents are not).
- Building UI, player blueprints, mechanisms (doors, pumps, floodgates), rooms, and work orders.
- Multiplayer implementation and versioned multi-slot / cross-mode gameplay save loops.
- Native platform forks in load-bearing layers; substrate stays on a portable GPU abstraction (wgpu/WGSL), not a vendor-native fork.

Compatibility *seams* may be designed where substrate requirements demand them; those upper layers must not be implemented here.

**Deferred within product (not non-goals):** movable material and object lifecycle beyond placement/render; material/ambient cellular behavior; day/night, seasons, weather-driven world state, and growth/ecological progression; dynamic fluids; granular behavior; structural integrity/cave-ins; fire/ecological behavior; full consumer API (commands, events, object model beyond the dig/place + mirror sliver); persistence of substrate-owned object changes beyond first-proof terrain deltas. Product One explicitly leaves these out of the first proof; the substrate design assigns them to Moria.

---

## Required product-level outcomes

### Substrate mandate (current product)

These outcome families define what Moria must provide as a reusable substrate. First delivery proves a subset; the rest remain required product outcomes for later stages.

1. **Normal-looking material world** — Generated natural terrain that looks continuous and good, not like a cube world; voxel grid is truth, mesh is view.
2. **Authorized mutability** — Any voxel can be destroyed, moved, or placed; cuts and scars are real matter changes with interactive visual regeneration.
3. **Interactive voxel objects** — Substrate-owned objects (e.g. trees, boulders) participate in matter: placement, registration, rendering, and lifecycle behaviors such as felling / rigid conversion when delivered.
4. **Deep-Z honesty** — Continuous three-dimensional world; underground geology is content players and tools can reach and alter.
5. **Material and ambient cellular behavior** — Cellular and ambient rules over matter (wetness, fire/ecology, granular settle, structural integrity/cave-ins) as substrate capabilities for consumers to exploit—not game systems.
6. **Temporal and weather-driven ambient world** — Day/night and seasons drive light, growth ticks, and climate-sensitive surface state; weather fronts write wetness, affect water tables, and can ignite or extinguish ecological processes. Thin but present so the world *behaves* as a normal ambient surface, not only as static geology plus on-demand CA.
7. **Dynamic fluids** — Beyond static bodies: flow and interaction tiers that keep hydrology a substrate toybox without requiring a game layer.
8. **Reusable consumer boundary** — Commands in, stale mirror plus events out, object model, and exclusive verb/query access to voxel truth for all consumers (games and harness alike).
9. **Sparse large-world practicality** — GPU-resident design with lazy materialization and streaming so large regions are practical without full raw-volume residency.
10. **Persistence of substrate-owned state** — Seed + deltas restore edits and moved/felled substrate objects; scars stay compact. (Game-entity and cross-mode gameplay loops are not product scope.)
11. **Portable GPU path** — Load-bearing layers stay on a portable abstraction (wgpu/WGSL) with no native platform fork, so the crate remains portable across target GPUs rather than pinned to one vendor stack.
12. **Public-interface-only validation** — The harness proves external-consumer readiness through the same public surfaces a game would use.

### First proof — Product One (undeniable when first delivery ships)

When Product One is done, these claims should be undeniable without needing a full game:

1. **Postcard material region** — A generated natural region (meadow, forest, river/lake, cliffs with readable strata, walkable cave, micro objects, at least one stamped structure) that reads as continuous and good.
2. **Mesh is a view** — Mid-sprint dig/place carves real tunnels with cut faces that look like cut earth; collision and queries run on voxel truth; remesh stays interactive and hitch-free under ordinary digs.
3. **Deep-Z honesty (slice)** — Continuous traversal from surface into underground geology; dig-down can hit true subsurface bands (e.g. aquifer, ore).
4. **Sparsity and streaming are load-bearing** — Demo region large enough that raw full-volume residency is not the design; idle and far volume stay cheap; GPU residency for the active world stays bounded under streaming.
5. **Real-time walkable presentation** — Sustained interactive frame rate while exploring and mutating the seed region; cold-start reaches a walkable world quickly enough that lazy materialization is visibly doing its job.
6. **Exact restore of terrain defacement** — Reload the same seed plus edit deltas **restores exactly** (single-slot validation save); scars stay compact.
7. **API sliver enforced** — Dig/place and mirror queries are the only path the harness uses to touch voxels—establishing exclusive verb/query access from day one, as the first slice of the broader consumer boundary.
8. **Measured credibility** — Scripted flythrough/carve benchmarks publish the product’s success properties (frame rate under load, dig-to-remesh responsiveness, cold-start to walkable, GPU-resident memory under streaming, save compactness and exact restore) with machine profile so regressions stay comparable across hardware.
9. **Portable GPU path** — Load-bearing work stays on the portable GPU path (wgpu/WGSL), with no native Metal or other vendor fork in substrate layers.

Hardware-specific numeric gates and machine-local kernel rules (e.g. particular frame-rate floors, memory caps, or buffer-atomic widths on a given GPU) are **design and implementation constraints**, not vision-level product identity. The outcome families above are binding; the numbers and kernel tactics that achieve them are decided downstream.

Open technical choices (final voxel size, distant presentation strategy, object-registry scaling, exact storage/meshing mechanisms, harness controls and diagnostic presentation) are **decision beds** for design and measurement—not product-identity claims. Downstream design chooses those means; this vision binds the outcomes above.

---

## What this deliberately is not

Moria is not Moria-the-game, not a fortress sim, not an ARPG, and not a System showcase. Future modes (descent fantasy, DF-style building, System-authored content) motivate **capabilities** of the substrate—geology depth, full mutability including movement and object lifecycle, queryable matter, cellular/fluid/integrity behavior, and ambient temporal/weather progression—but their gameplay, content, characters, assets, and rules are **reference context only** and do not enter current scope.

Product One’s curated postcard world and milestone sequence are the **binding first proof**, not a content franchise and not a redefinition of the substrate’s mandate. Concrete seed features (species counts, material lists, exact region dimensions, demo route beats, listed storage structures, meshing algorithms, control schemes) belong in design/implementation plans derived from this vision; they are not restated here as the product’s identity.

---

## Unresolved questions for humans

The seeds agree on product identity, purpose, and the split between full substrate mandate and Product One’s first proof. Timber/felling is settled: absent from Product One, stretch afterward—not an open product question.

One product-scope constraint remains genuinely open in the seeds:

1. **Multiplayer readiness guarantee** — The substrate’s command/mirror architecture is described as server-authoritative-ready by construction, and the substrate seed asks whether that readiness should remain an explicit scope claim even though multiplayer is not built. Does product scope require that readiness guarantee as a binding constraint on the public API and ownership model, or is multiplayer-readiness an incidental property of good layering with no product-level commitment?

Multiplayer *implementation* remains excluded either way.

---

## Seed contributions (traceability)

| Source | What it contributed to this vision |
|---|---|
| **README.md** | Names the product (Moria), roles (substrate crate vs walkable harness), and points at `docs/seeds/` as preserved inputs. |
| **project-boundary.md** | Binding product identity: reusable Rust substrate; game is external; harness must use public APIs; Cargo workspace split; game rules / System / LLM / spell / gas / combat / AI / building layers out of scope (seams only). |
| **product-one-seed.md** | Binding *first* slice: walkable generated region, dig/place proof, third-person harness, generation full / matter partial / API sliver, **performance outcome families** (real-time presentation, hitch-free interactive remesh, fast walkable startup, bounded GPU residency, compact saves, exact restore) as product success properties—not mere observability—plus portable wgpu/WGSL path without native forks. Explicit first-delivery non-goals (including CA, dynamic fluids, integrity, granular, felling, weather/seasons/growth) defer rather than cancel substrate work. Demo content and milestones treated as validation fixture requirements at outcome level, not as a game design. |
| **voxel-world-substrate.md** | Full substrate mandate at outcome altitude: mutability including move; object lifecycle; cellular/ambient behavior; **temporal and weather-driven ambient world** (day/night, seasons, weather fronts, growth/ecological progression—§7, thin but present); dynamic fluids; granular and integrity; fire/ecology; commands in / mirror + events out / object model / exclusive verb access; persistence of edits and moved/felled substrate objects; GPU residency and sparse large-world practicality. Design Goals 2, 4, 5 and §§5–8, 11–13 supply deferred outcome families kept in product scope. Game-layer examples (ARPG, fortress, System) remain reference only. §14 multiplayer readiness left as the sole open product-scope question. |

**Omitted on purpose (present in seeds, not imported as current product *implementation* or first-proof scope):** running CA/fire/integrity/granular sims, full ambient day/night–weather–growth, and dynamic fluids in Product One; building UI/mechanisms/rooms/work orders (outside product entirely); System attachment and content authorship; multi-game layering examples as deliverables; viral-clip milestone storytelling; exact bit layouts, storage structures, meshing mechanisms, harness control maps, hardware-specific numeric gates, M4-local kernel rules (e.g. 64-bit buffer atomics), and open engineering questions reserved for technical design.

**No seed conflicts** on which product is current: all four documents describe the same substrate product; Product One is the first proof with a partial Matter layer and API sliver, not a second product and not a permanent narrowing of substrate-owned outcomes. The substrate doc’s wider catalog is the product mandate staged after the first proof; project-boundary exclusions (game/System/AI/building layers) remain true non-goals.
