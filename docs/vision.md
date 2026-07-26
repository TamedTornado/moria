# Moria — Product Vision

*Proposal for human approval and handoff to downstream design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped crates.

It is consumed by games and tools as a library. This repository also includes a **walkable-world executable** that is only a validation harness: it must exercise the substrate through the same public interfaces an external game would use. It is not a game layer and must not own privileged or game-specific paths.

The first binding delivery is a product-shaped proof of the substrate: one curated generated region a player can run through in third person, with dig/place as the proof that the world is fully material matter—not a heightmap with props.

---

## Purpose

Moria exists so that downstream games can stand on a world that is:

1. **Visually continuous** — terrain reads as a natural surface world (hills, forest, water, cliffs, caves), with the voxel grid as truth rather than as the aesthetic.
2. **Mutable everywhere** — any solid volume can be destroyed or placed; the render mesh is a regenerated view, never authoritative.
3. **Fully three-dimensional** — deep underground is first-class content (strata, caves, aquifers, ore), not a painted floor under a heightmap.
4. **Reusable** — clean public verbs and queries so the same substrate can later support adventure, fortress/colony, sandbox, or other modes without forking world truth.
5. **Credible under load** — sparsity, streaming, incremental remesh, and seed-plus-delta persistence make large regions practical; measured performance is part of the product claim.

The substrate provides **matter, generation, meshing/dressing, mutation, collision against voxel truth, streaming, and persistence**. Game rules live above it.

---

## Boundary

### In scope (this repository)

| Layer | Role |
|---|---|
| **Generation** | Geology-first pipeline: columns, strata, caves, ore/aquifer bands, lazy brick materialization, POI/stamp metadata. Continent-scale pass may be stubbed to one curated region’s parameters. |
| **Matter** | Brick pool, homogeneous sentinels, density-aware occupancy, GPU dirty-brick meshing (smooth isosurface with sharp cut faces), grass/clutter dressing from surface data, voxel-object placement and rendering for trees/micro objects, static water bodies (tier-1 surfaces). |
| **Script/API (sliver)** | Dig/place verbs and mirror queries as the only path to voxels—enforced from day one even if only debug tools call them. No embedded scripting language required yet. |
| **Validation harness** | Third-person character controller, free-orbit camera, collision vs voxel occupancy (not vs mesh), debug keys (dig/place, wireframe/brick views, streaming visualizer, time-of-day), plus benchmarks that report frame time, remesh latency, cold start, memory, and save/load against a machine profile. |
| **Workspace split** | Cargo boundary between reusable substrate crates and the harness executable. |

### Out of scope (non-goals)

- The actual game(s), and any game-rule layer (combat, stats, economy, intent).
- The System / LLM, spells, gas metering or pricing policy.
- AI, NPCs, or entities beyond the single player avatar in the harness.
- Building UI, player blueprints, mechanisms (doors, pumps, floodgates), rooms/work orders.
- Cellular automata and ambient sims: fire, wetness propagation, granular settle, structural integrity/cave-ins, weather, seasons, growth.
- Fluids beyond static bodies (no coarse flow, pressure, or fine splash).
- Voxel-object felling / rigid-body conversion (stretch only; not required for first delivery).
- Multiplayer, versioned multi-slot saves, or cross-mode fortress reclaim loops.
- Native Metal (or other platform) forks in load-bearing layers; substrate stays on portable GPU abstraction (wgpu/WGSL).

Compatibility *seams* may be designed where substrate requirements demand them; those upper layers must not be implemented here.

---

## Required product-level outcomes

When the first delivery is done, these claims should be undeniable without needing a full game:

1. **Normal-looking material world** — A generated natural region (meadow, forest, river/lake, cliffs with readable strata, walkable cave, micro objects, and at least one stamped structure) that looks continuous and good, not like a cube world.
2. **Mesh is a view** — Mid-sprint dig/place carves real tunnels with cut faces that look like cut earth; collision and queries run on voxel truth; remesh stays interactive (no hitch on a multi-meter carve).
3. **Deep-Z honesty** — Continuous traversal from surface features into underground geology; dig-down can hit true subsurface bands (e.g. aquifer, ore), not decorative underground.
4. **Sparsity and streaming are load-bearing** — The demo region is large enough that raw full-volume residency is not the design; idle and far volume stay cheap via sentinels and lazy materialization.
5. **Persistence is seed + deltas** — Reload the same seed plus edit deltas restores the defaced world exactly enough for a single-slot validation save; scars stay compact.
6. **Public API boundary** — The harness proves external-consumer readiness: nothing above matter touches voxels except through verbs/queries.
7. **Measured credibility** — Scripted flythrough/carve benchmarks publish numbers (frame rate, dig-to-remesh latency, cold-start to walkable, GPU-resident memory under streaming, save size) with machine profile, so later substrate changes can regress against this bed.
8. **Portable GPU path** — Development constraints (e.g. no 64-bit buffer atomics, bandwidth-aware sparsity) are respected so the crate remains portable across target GPUs, not pinned to one vendor stack.

Open technical choices (final voxel size, distant LOD strategy, object-registry scaling) are **decision beds** for this delivery: answer them with measurements on the validation region, not as separate product forks.

---

## What this deliberately is not

Moria is not Moria-the-game, not a fortress sim, not an ARPG, and not a System showcase. Future modes (descent fantasy, DF-style building, System-authored content) motivate **capabilities** of the substrate—geology depth, mutability, object-backed vegetation, queryable matter—but their gameplay, content, characters, assets, and rules are **reference context only** and do not enter current scope.

Product One’s curated postcard world and milestone sequence are the **binding first proof**, not a content franchise. Concrete seed features (species counts, material lists, exact region dimensions, demo route beats) belong in design/implementation plans derived from this vision; they are not restated here as the product’s identity.

---

## Unresolved questions for humans

The seeds agree on product identity, purpose, and boundary. No ambiguity requires guessing before design can proceed.

Optional confirmations that would only refine emphasis (not identity):

1. **Stretch timber** — Is tree felling / rigid fall explicitly deferred until after the first public demo, or welcome if it stays cheap on the critical path?
2. **Multiplayer readiness** — Should “command/mirror architecture remains server-authoritative-ready” stay as an explicit product claim in scope statements, even though multiplayer is not built?

If unstated, default from seeds: felling is stretch; multiplayer is not a current claim.

---

## Seed contributions (traceability)

| Source | What it contributed to this vision |
|---|---|
| **README.md** | Names the product (Moria), roles (substrate crate vs walkable harness), and points at `docs/seeds/` as preserved inputs. |
| **project-boundary.md** | Binding product identity: reusable Rust substrate; game is external; harness must use public APIs; Cargo workspace split; game rules / System / LLM / spell / gas / combat / AI / building layers out of scope (seams only). |
| **product-one-seed.md** | Binding first slice: walkable generated region, dig/place proof, third-person harness, generation+matter+API-sliver scope, performance outcomes, explicit non-goals, and “substrate crates exist with API boundary enforced.” Demo content and milestone list treated as validation fixture requirements at product-outcome level, not as a game design. |
| **voxel-world-substrate.md** | Architecture reference for high-level capabilities (smooth meshing over voxel truth, brick sparsity, geology-first gen, object vs dressing split, fluid tiers, integrity, building, nav, streaming). Only the Product One–selected subset is required now; fuller layers motivate seams and non-goals, not current implementation. |

**Omitted on purpose (present in seeds, not imported as current product scope):** full CA/fire/integrity/granular sims; fluids tier 2–3; weather/seasons/growth; building/mechanisms/rooms; System attachment and content authorship; multi-game layering examples (ARPG / fortress / descent); viral-clip milestone storytelling; exact bit layouts, kernel choices, and open engineering questions reserved for technical design.

**No seed conflicts** on which product is current: all four documents describe the same substrate product; Product One is the first proof, not a second product; the substrate doc’s wider catalog is reference for future consumers and later substrate milestones.
