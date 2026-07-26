# Moria — Product Vision

*Proposal for human approval and handoff to design. Not a GDD, technical design, requirements catalog, or feature inventory.*

---

## Current product

**Moria** is a reusable, GPU-resident **voxel-world substrate**: a Rust crate (or small family of tightly scoped crates) that owns matter, generation, meshing, streaming, mutation, collision against voxel truth, and persistence. Downstream games consume it; this repository does not ship a game.

A **walkable-world executable** ships only as a **validation harness**. It must use the same public interfaces an external game would use—no privileged or game-specific substrate paths. Product One scopes the first vertical slice: one curated generated region and a third-person character that can traverse it, proving the substrate as a product-shaped demo rather than a whiteboard.

---

## Purpose

Make one claim undeniable: **this is not a heightmap with props—it is a fully material world that reads as a normal natural place, and it looks good.**

The substrate exists so future games (adventure, fortress/colony, sandbox, or otherwise) can stand on shared geology, mutability, deep-Z, and query/mutation APIs without re-implementing the world layer. Product One buys measured proof of that layer: crates with an enforced consumer boundary, a public demo artifact, and a decision bed for open substrate choices (e.g. voxel size, LOD, object-layer scale) answered with numbers instead of guesses.

---

## Boundary

| In this repository | Outside this repository |
|---|---|
| Generation (incl. column index, POI metadata, strata, caves, ore, lazy materialization), matter storage, smooth meshing, dressing, static fluid bodies, dig/place verbs, mirror queries, streaming, persistence deltas | Any finished game, rules, combat, stats, AI, economy |
| Walkable-world harness (controller, camera, debug tools) as consumer of public APIs | System / LLM layer, spells, gas metering, intent; System-directed placement that would *consume* POI metadata |
| Compatibility *seams* where substrate requirements demand them | Building UI, blueprints, mechanisms, work orders; fortress-style Z-slice *gameplay* that would *consume* the column index |
| Cargo workspace separation of reusable substrate vs harness | Privileged harness-only paths into voxel internals |

**Layering intent (high level):** generation and matter are substrate; a thin script/API surface (verbs + queries) is established early so nothing above touches voxels directly. Semantic and game layers live above and are not implemented here. Product One ships the **generation layer as a reusable asset**—including column index and POI metadata alongside strata, caves, ore, and lazy materialization—not a stub that later products must replace.

**Dev-platform constraint that shapes the product identity of the crate:** load-bearing GPU work stays on portable **wgpu/WGSL** (no native Metal fork in those layers), with 32-bit counters/allocators where required by Apple GPU limits.

---

## Required product-level outcomes

These are outcomes the current product must make true—not a content checklist or implementation plan.

1. **Reads as a normal world.** Generated natural terrain (hills, forest, water, cliffs, caves, surface dressing and voxel-backed props) that does not present a blocky “Minecraft aesthetic” as the primary look: the voxel grid is truth; smooth extraction (or equivalent) is the view.
2. **Mutable everywhere that matters for the demo.** Dig and place exist as first-class substrate verbs (debug-driven in the harness). Cut faces read as cut matter; remesh is incremental and hitch-resistant on modest carves. Without this, the demo is interchangeable with ordinary terrain scenes.
3. **Continuous deep Z.** Surface-to-underground traversal in one continuous volume—caves, strata, and subsurface materials are real content, not a painted floor under a skybox.
4. **Collision and queries against voxel truth.** Character motion and interaction prove the mesh is a regenerated view, not the authority.
5. **Sparsity, streaming, and lazy materialization.** A region large enough that raw full-volume residency is not the strategy; homogeneous bricks / sentinels and streaming rings are load-bearing, not deferred polish.
6. **Reusable generation layer, not a one-off demo gen.** Product One’s generation ships as designed for reuse: columns, strata, caves, ore, lazy materialization, and **POI metadata**. Column index and POI metadata are first-class generation outputs in this milestone—not deferred hooks. Future consumers (fortress-style Z-slice views, System-directed placement) remain **out of scope**; the substrate provides the data those layers would later read.
7. **Persistence as seed + deltas.** Reload the same world with player scars restored; saves stay compact after defacement.
8. **Consumer-safe public API.** Harness and future games share the same interfaces; the workspace boundary makes that enforceable.
9. **Credible, comparable performance.** Product One’s numerical bar (frame rate on mid discrete + M4-class unified memory, dig-to-remesh latency, cold start, GPU memory under streaming, save size) and a benchmark path that records machine profile so regressions are comparable across hardware.

**Capabilities preserved for later products (substrate must not preclude them):** priced or policy-pluggable verbs; brick-level aggregates for future CA/fluids/integrity; object-layer vegetation that can later rigidify; multiplayer-ready command/mirror shape. Product One does **not** implement those game or sim layers—it leaves format and API room. Column index and POI metadata are **not** in this “later” set; they ship now as part of the generation layer (outcome 6), while fortress/System use of them does not.

---

## Non-goals (current product)

- Combat, RPG stats, entities beyond the player, AI/agents
- The System / LLM, spells, gas, intent pricing
- Building gameplay: blueprints, work orders, mechanisms, room semantics (stamp/prefab may be exercised once as a generation/API path in the seed world, not as a player building product)
- Fluids beyond static bodies (lakes / river channel with surface); no flow simulation
- Weather, seasons, growth sims (a fixed time-of-day control is enough)
- Cellular automata (fire, wetness propagation, granular settle), structural integrity / cave-ins
- Voxel-object felling → rigid body (stretch only; not required for “done”)
- Embedded scripting language
- Multiplayer networking (architecture may stay server-authoritative-ready; not a ship goal)
- Persistence beyond single-slot seed + deltas (no versioning story)
- Shipping or owning the actual game that will consume Moria

---

## Unresolved human questions

Seeds agree on product identity: **substrate + public-API harness; Product One is the binding first slice; full substrate design is architecture reference.** No identity-level conflict required a guess.

These remain open and may affect design emphasis (not whether Moria is a game vs substrate):

1. **Voxel size final call** — 25 cm assumed; 12.5 cm vs per-region mixed resolution still open; Product One’s region is the benchmark bed.
2. **Distant representation** — chunked mesh LOD vs column-derived impostors (camera assumptions from future games may matter later; free third-person for the harness reduces cheating room).
3. **Object-layer scale** — when tree/prop counts need dedicated spatial acceleration.
4. **How far multiplayer-readiness is a stated scope commitment** vs passive architectural hygiene (seeds lean “ready by construction, not built”).

None of these change the product’s identity, purpose, or repository boundary. Resolve in design when measurements force a choice.

---

## What each seed contributed

| Source | Role in this vision |
|---|---|
| **README.md** | Names the product (Moria), crate-vs-harness split, and that seeds live under `docs/seeds/`. |
| **project-boundary.md** | Binding operator boundary: reusable substrate only; game layers out of scope; harness must consume public APIs; Cargo workspace as immediate expression of that boundary. |
| **product-one-seed.md** | Binding first-slice scope: product statement, non-goals, dig/place as proof, player/harness role, performance targets, milestone spirit, and what Product One buys. Explicitly requires the **generation layer full**—columns, strata, caves, ore, lazy materialization, POI metadata—as the reusable asset. Seed-world content (specific 1 km route, material list, ruin, etc.) is treated as **validation intent and proof points**, not imported here as fixed game content or a feature inventory. |
| **voxel-world-substrate.md** | Architecture reference for look strategy, storage, geology-first generation (including column index and POI metadata purposes), layering, and future-facing capabilities. Only portions selected by Product One are current-scope requirements; broader matter sim, building, weather, entities, fortress/System consumers of columns/POI, and game modes are context for seams and non-goals. |
| **docs/seeds/README.md** *(manifest note)* | Confirms the priority order: Product One binding for this milestone; substrate doc partial; boundary as operator clarification. |

**Omitted from vision on purpose (still in seeds):** milestone schedules and week estimates; exact material palette and region feature tables; meshing algorithm bake-offs and bit layouts; full fluids/integrity/building/entity designs; System attachment recipes; viral-clip / marketing milestone framing beyond “public demo artifact”; stretch timber felling.

---

## Approval

This vision is ready for human review. Downstream design should treat Product One + project boundary as binding scope and the substrate document as the capability envelope—not as a mandate to implement the full envelope in this milestone.
