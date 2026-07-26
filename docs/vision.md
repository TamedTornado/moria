# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a
Rust crate or a small family of tightly scoped Rust crates. It is an
engine-layer world foundation—matter, generation, mutation, observation, and
the public interfaces games consume. It is not a game, not a game ruleset, and
not a character-driven experience product.

This repository’s required first delivery also includes an **adjacent**
walkable-world executable that validates the substrate through the same public
interfaces available to an external game. That executable is a validation
harness, not a game layer and not Moria’s product identity.

## Purpose

Moria exists so multiple downstream games can share one material world
foundation: a natural-looking surface over fully mutable voxel truth, with deep
underground as first-class space, without each title reimplementing world
matter. The substrate stands alone with no LLM or “System” dependency. Game
identity, policy, presentation, and controllers live above it.

## Product boundary

**In product**

- The reusable substrate and its public consumer-facing interfaces (Rust crate
  boundary; workspace separation from any harness is required).
- Substrate-owned world capabilities at product altitude: geology-backed
  material worlds; sparse, lazily materialized matter; full mutability;
  presentation and collision derived from voxel truth (mesh is never
  authoritative); matter-coupled surface features; consumer mutation and
  observation without direct voxel access; streaming of active regions;
  persistence as generation identity plus edit deltas.
- Compatibility seams only where substrate requirements demand them—not
  implementations of excluded game layers.

**Out of product / out of this repository**

- Any actual game (ARPG, fortress, descent, sandbox, or otherwise) and its
  rules, content, UX, controllers, characters, combat, AI, economy,
  gas/pricing policy, spells, or LLM/System layer.
- Game-facing building, designation, work-order, and similar policy layers.
  Matter placement as an engine capability remains substrate; fortress/building
  *gameplay* does not.
- Privileged or game-specific paths that bypass public substrate interfaces.

**Adjacent first delivery (not product identity)**

- A walkable-world validation executable is required as the first proof that
  the substrate works. It must use only public interfaces. Its controller,
  character, camera, demo route, authored region content, presentation,
  platforms, machine targets, milestones, and performance gates are harness
  concerns—not Moria’s product identity.

## Required product outcomes

These are product-level outcomes for the substrate. **First delivery** proves a
usable depth of them (Product One’s binding slice); it does not redefine the
product as a demo.

1. **Material world truth.** Consumers operate on a fully mutable voxel
   material world: matter can be destroyed, moved, or placed; dig and place are
   first-class substrate capabilities. Presentation and collision are derived,
   non-authoritative views of voxel truth—not a heightmap with disconnected
   decoration as world authority.
2. **Natural surface, deep Z first-class.** The world can read as continuous
   natural terrain and structure while remaining voxel-authoritative.
   Underground volume (strata, caves, subsurface features) is real space, not a
   painted floor under a heightmap.
3. **Geology-backed generation with sparsity.** Worlds generate as layered
   geology so digging encounters honest material depth, materializing lazily so
   large regions stay tractable under sparse storage.
4. **Matter-coupled surface and first-slice matter depth.** Surface dressing and
   interactive voxel-backed objects are coupled to matter (not free-floating
   decoration). First delivery includes static fluid bodies and
   placeable/renderable objects at Product One depth—not the full long-horizon
   reactive stack (flowing fluids, fire CA, structural integrity, granular
   settle, object felling) as a current-milestone mandate.
5. **Public mutation and observation contract.** Consumers mutate and observe
   through commands in and a stale mirror plus events out; higher layers do not
   touch voxels directly. The harness and external games share that surface
   with no privileged in-tree path.
6. **Streaming, persistence, and measurable use.** Active regions stream; world
   truth persists as generation identity plus edit deltas so scars and
   substrate-owned object changes can be restored. Performance is observable
   through public use and the harness (numeric gates are harness/design detail,
   not product identity).
7. **Reusable engine boundary.** The same substrate can underpin multiple game
   genres by exposing matter, queries, and mutation without embedding game
   rules, gas policy, or LLM dependency. Load-bearing GPU work stays on a
   portable path (wgpu/WGSL intent); no native Metal-only fork in load-bearing
   layers.

Long-horizon substrate families described in the architecture reference
(multi-tier fluids, integrity, fire ecology, granular settle, mutation-safe
navigation, broader object lifecycle) remain purpose and seam context for the
product’s multi-game role. Their delivery depth and sequence after the first
slice are design concerns, not silent expansions of “now.”

## Future products and enabling implications

Downstream consumers (not built here) may include a System-driven ARPG,
DF-style fortress/colony play, Moria-style descent, or pure sandbox. High-level
enabling implications only: public mutation/query/event interfaces games can
price and script differently; material and placement registries; seams for
game-injected policy and structure metadata without the substrate owning those
policies. Gameplay, content, presentation, controllers, characters, and
game-specific systems remain consumer-owned.

## Non-goals

- Implementing the game, System/LLM, spells, gas/intent pricing, combat, AI, or
  game building layers in this repository
- Treating the validation harness’s demo fantasy (character, route, seed
  inventory, clip goals, machine gates) as Moria’s product definition
- Making the substrate depend on an LLM or an in-process game ruleset
- Shipping the full long-horizon reactive matter stack as part of first
  delivery merely because the architecture reference describes it
- Multiplayer product delivery as a current commitment (command-style
  boundaries may stay design-friendly; online service is not promised here)

## Confirmed vision constraints

- **Identity:** reusable GPU-resident voxel-world substrate
- **Delivery form:** Rust crate or small family of tightly scoped Rust crates
- **Consumer boundary is not optional:** external games and the validation
  harness share public interfaces only
- **Standalone engine:** zero LLM/System dependency inside the substrate
- **Portable GPU path:** load-bearing GPU work stays on a portable wgpu/WGSL
  path; no native Metal-only fork in load-bearing layers
- **Scope exclusion:** game rules and System, LLM, spell, gas, combat, AI, and
  building *layers* stay out of this repository

## Deferred design decisions

- Exact crate split, API shape, and internal layering within the substrate
  family
- Algorithms, resolutions, data layouts, meshing approach, LOD, streaming
  policy, and persistence encodings
- Delivery depth and sequence for long-horizon matter families after the first
  Product One slice
- Concrete command/mirror/event realization; multiplayer beyond the public
  interaction contract
- All harness-specific UX, content, platforms, milestones, and performance
  thresholds

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds agree on product identity and first-delivery authority:

- **Current product** is the reusable substrate (crate(s)), not a game.
- **Product One** binds the first implementation slice and the walkable
  validation harness.
- **voxel-world-substrate** is architecture reference; only the portions
  selected by Product One are required for this milestone.
- Future games and excluded layers stay out of this repository.

If human intent were instead that *all* long-horizon matter/physics families in
the architecture reference are current-milestone mandate (not first-slice depth
plus later growth), that would materially expand first-delivery scope—but that
reading contradicts the seeds’ own authority ordering, so it is not assumed
here.

## Seed contributions

| Seed | Role | Contributed | Not imported as current scope |
|---|---|---|---|
| **README.md** | Top-level identity | Names Moria as GPU-resident Rust substrate; walkable-world executable as separate validation consumer for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer | Harness content, numeric gates |
| **docs/seeds/project-boundary.md** | Binding boundary | Product = substrate crate(s); game downstream and out of repo; harness must use public APIs; workspace boundary required; game/System/LLM/spell/gas/combat/AI/building layers out of scope (seams only) | Crate graph details; harness feature list |
| **docs/seeds/product-one-seed.md** | Binding first slice | What is built first and what “done” means for that proof: curated generated region, partial matter (meshing, static water, dressing, object place/render), dig/place as mutability proof, walkable harness, portable wgpu intent; explicit non-goals that reinforce “no game” | Character/camera fantasy, seed inventory as product promise, milestone schedule, FPS/memory tables as identity, full reactive stack |
| **docs/seeds/voxel-world-substrate.md** | Architecture reference | Purpose and multi-game rationale; layering model; voxel truth vs mesh view; sparsity and lazy materialization; long-horizon matter families and enabling seams for future consumers | Exact voxel size, brick layout, algorithms, bit packs, CA rules, build-order checklists, and full-stack “all now” expansion |

**Contradiction handling.** Product One’s “product-shaped demo” language is
subordinated to the boundary and seeds README: the **product** is the
substrate; the walkable world is the **required adjacent harness**, not a
second product identity. Where Product One and the architecture reference
disagree on depth, first delivery follows Product One; long-horizon purpose
follows the reference without expanding “now.”

---

*Proposed vision for human approval and design handoff—not a GDD, technical
design, requirements catalog, or feature inventory.*
