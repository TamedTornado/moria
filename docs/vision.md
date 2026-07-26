# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine layer for mutable natural worlds—not a game, not a demo, and not the System/LLM stack.

## Purpose

Future games need the same foundation: a world that reads as ordinary landscape while remaining fully material voxel truth underneath—mutable anywhere, including deep underground—with generation, matter, queries, and mutation available to any consumer through a clean public boundary. Moria exists so those games share one substrate instead of each re-implementing geology, matter, and world mutation.

## Product boundary

**In product**

- The reusable voxel-world substrate and its public integration surface for external consumers.
- High-level substrate responsibilities: world generation (geology-first), material matter representation, mutation and query verbs, surface presentation of voxel truth as a non-authoritative view, sparse residency/streaming behavior, and persistence as generation plus edit deltas—as engine capabilities, not game content.
- Compatibility seams where substrate requirements demand them for later game layers—without implementing those layers.

**Adjacent / not this product**

- The actual game (ARPG, fortress/colony, descent roguelike, sandbox, or other) is a separate downstream consumer and is not part of this repository’s product identity.
- A walkable-world executable may exist in the repository only as a validation harness. If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific implementation paths. Whether that harness is a required repository delivery is unresolved (see Q1). Its character, camera, debug controls, authored seed route, presentation, benchmarks, and platform gates are not current-product scope.
- Game rules and the System, LLM, spell, gas, combat, AI, and building layers are out of scope here.

## Required product outcomes

1. **Natural look, voxel truth.** Consumers get a world that reads as ordinary terrain (rolling ground, forest, water, cliffs, caves) while the voxel grid remains the authoritative material state. Presentation is a regenerated view; physics, queries, and gameplay-facing mutation run against matter, not against the mesh.
2. **Mutable everywhere, deep Z first-class.** Any material volume in the world can be destroyed, moved, or placed. Underground is real content space (strata, caves, ore, voids)—not a decorative floor under a surface shell.
3. **Geology-first generation with sparse residency.** Worlds are produced as geology (columns, strata, caves, materials) that materializes lazily so large regions remain tractable; idle volume does not require full dense residency.
4. **Matter substrate for multiple games.** The product provides matter, world structure, queries, and mutation so the same crate stack can underpin distinct game modes. Game policy (pricing, combat, AI, LLM direction, building UX) lives above the substrate.
5. **Consumer-safe public boundary.** Adjacent consumers—including any validation harness—integrate only through public substrate interfaces; nothing above the matter/API boundary depends on privileged internal voxel access.
6. **Standalone engine layer.** The substrate has zero LLM dependency and stands alone without the System or any particular game ruleset.

## Future products and enabling implications

Described future consumers of Moria (not current product):

- A System/LLM-driven ARPG and related adventure modes.
- Dwarf Fortress–style fortress/colony play and Moria-style deep descent.
- Pure sandbox and other modes that share the same matter world.

**Enabling implications (substrate-side, not consumer features):** a material world that digs and places honestly; deep continuous Z; sparse large-world residency; generation and persistence that leave scars and deltas reusable across runs/modes; public verbs/queries so games inject their own policy. Gameplay, controllers, authored content, presentation, economy, spells, gas, agents, and building UX remain consumer-owned.

A first “walkable world” slice (one region, character traversal, dig/place proof, demo route) is an adjacent consumer/validation story that motivates substrate depth; it does not redefine Moria as that demo.

## Non-goals

- Shipping a game, combat, stats, AI, spells, gas/pricing policy, or System/LLM features in this product.
- Implementing building, blueprint, mechanism, room, or designation *game layers* here (seams only if the substrate itself requires them).
- Treating mesh, props, or heightmap-only scenery as authoritative world state.
- Owning harness-specific content, UI, character control, benchmark scenes, or machine-specific performance gates as product identity.

## Confirmed vision constraints

- **Rust crate delivery:** exposed as a Rust crate or small family of tightly scoped Rust crates.
- **GPU-resident world/matter path:** substrate design assumes GPU-resident matter and related residency patterns (consumer-selected backends and device models are not product promises).
- **Layering:** game rules and System/LLM are clients or upper layers, not substrate features; gas/policy and similar rules inject above matter.
- **Harness privilege ban:** any in-repo walkable-world executable is validation only and shares the external public interface (delivery mandate itself: Q1).

## Deferred design decisions

- Voxel resolution, meshing strategy, LOD, object-layer capacity, fluid/integrity/CA depth, and related algorithms.
- Exact crate split, API surface shape, persistence encoding, streaming ring policy, and synchronization patterns.
- Delivery sequence and depth of substrate capabilities versus any first validation slice.
- Supported runtimes, hardware targets, and performance budgets (including any harness benchmark numbers).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is an in-repository walkable-world executable a *required* repository delivery for this effort, or only *permitted* as an adjacent validation harness?

- **Proposed safe answer:** Permitted only—product identity stays the reusable substrate; a harness may exist and must use public interfaces if present, but is not a mandated product deliverable.
- **If different:** Requiring the harness makes a repository deliverable of “substrate + validation executable” without folding harness content into product identity; declining it leaves validation entirely outside the repo. Either answer changes delivery boundary, not substrate purpose.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world substrate (Rust crate) and frames the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Fixes product identity on the substrate crate(s), places the real game outside the repo, permits a public-API-only harness, and excludes game/System/LLM/spell/gas/combat/AI/building layers.
- **docs/seeds/product-one-seed.md** — Adjacent first-slice and validation story (walkable region, dig/place proof, demo route, targets); motivates substrate outcomes without transferring harness content, controls, or platform gates into current product scope.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate purpose and outcomes: natural look over voxel truth, full mutability, deep Z, geology-first sparse worlds, reusable matter/query/mutation engine, standalone of LLM/game rules; mechanism detail deferred to design.
