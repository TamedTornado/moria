# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a
Rust crate or a small family of tightly scoped Rust crates. It is an
engine-layer world product—matter, generation, mutation, observation, and the
public interfaces games consume. It is not a game, not a game ruleset, and not
a character-driven experience product.

This repository’s required first delivery also includes an **adjacent**
walkable-world executable that validates generation, streaming, meshing,
editing, collision, persistence, and performance through the same public
substrate interfaces available to an external game. That executable is a
validation harness, not a game layer and not Moria’s product identity.

## Purpose

Moria exists so multiple downstream games can share one material world
foundation: a natural-looking surface over fully mutable voxel truth, with deep
underground as first-class space, without each title reimplementing world
matter. The substrate stands alone with no LLM or “System” dependency. Game
identity, policy, presentation, and controllers live above it.

## Product boundary

**Belongs to Moria**

- The reusable voxel-world substrate and its public consumer-facing interfaces
  (Rust crate boundary; workspace separation from the harness is required).
- World capabilities at product altitude: geology-backed generated material
  worlds; sparse, lazy-materialized matter; full mutability of that matter;
  non-authoritative presentation regenerated from matter; substrate-owned
  surface dressing and interactive voxel objects as matter-coupled world
  features; static fluid bodies; command/mirror-style mutation and observation;
  streaming of large regions; persistence of generation identity plus edit
  deltas.
- Compatibility seams only where substrate requirements demand them—not
  implementations of excluded game layers.

**Does not belong to Moria**

- The actual game (any ARPG, fortress, descent, or sandbox title) and its
  rules, content, UX, controllers, characters, combat, AI, economy, gas/pricing
  policy, spells, or LLM/System layer.
- Game-facing building, designation, work-order, room-economy, and similar
  policy layers. Matter placement as an engine verb remains substrate;
  fortress/building *gameplay* does not.
- Privileged or game-specific implementation paths that bypass the public
  substrate interfaces.

**Adjacent first delivery (not product identity)**

- A walkable-world validation executable is a **required** adjacent first
  delivery. It must use only public interfaces available to an external game.
  Its controller, character, camera, demo route, curated seed content,
  presentation, machine-specific gates, and numeric performance thresholds are
  harness concerns—not Moria’s product identity. That slice proves a first
  usable depth of substrate outcomes; it does not redefine the product as a
  demo title.

**Binding first-delivery depth (Product One)**

Seeds bind the *current milestone* to a deliberately partial substrate slice:

- **Generation:** full for the harness’s curated region (columns, strata,
  caves, ore, lazy materialization, POI metadata); continent-scale pass may be
  stubbed to that region’s parameters.
- **Matter:** brick pool, homogeneous sentinels, lazy materialization, GPU
  dirty-brick smooth meshing, grass/clutter dressing, voxel-object placement
  and rendering, static water bodies (tier 1). Not in this slice: CA/fire,
  flowing fluids, structural integrity, granular settle, or object felling /
  rigid conversion (stretch only).
- **API surface:** dig/place verbs and mirror queries as the public “nothing
  touches voxels directly” boundary; no embedded scripting language yet.

Omissions apply to this first delivery slice, not to the long-horizon purpose
of remaining a multi-game material foundation.

## Required product outcomes

Downstream design must make these product-level outcomes true for the substrate
and its first adjacent proof:

1. **Material world truth.** Consumers operate on a fully mutable voxel
   material world: any matter can be destroyed or placed. Dig and place are
   first-class substrate capabilities. Presentation is a regenerated view of
   that truth—never save authority and never collision authority.

2. **Natural surface, deep Z.** The world can read as continuous natural
   terrain and structure while remaining voxel-authoritative. Underground
   volume (caves, strata, depth) is real space, not a painted floor under a
   heightmap.

3. **Geology-backed generation with sparsity.** Worlds generate as layered
   geology and related structure so digging and exploration encounter honest
   material depth. Large regions stay tractable through lazy materialization
   and homogeneous-sparse storage—not by keeping the full volume hot as raw
   voxels.

4. **Matter-coupled surface and objects (first-slice depth).** Interactive
   surface features that burn, break, or block are voxel-backed; non-interactive
   dressing is derived from or anchored to voxels. First delivery includes
   dressing, placeable/renderable voxel objects, and static fluid bodies at the
   depth above—not the full reactive stack (fire CA, flowing fluids, integrity,
   granular settle, felling).

5. **Public mutation and observation contract.** Consumers mutate and observe
   through commands and a stale mirror (plus events as the architecture
   requires); upper layers do not touch voxels directly. The validation harness
   and any external game share that surface with no privileged in-tree path.

6. **Streaming, persistence, and measurable performance.** Active regions
   stream; truth persists as generation seed/function plus edit deltas so a
   saved world can be restored. Performance is a first-class concern of the
   delivery (observable and regression-testable through public use and the
   harness); specific numeric gates and machine tables remain design/harness
   detail, not vision identity.

7. **Reusable engine boundary.** The same substrate is intended to support
   multiple game genres by exposing matter, queries, and mutation without
   embedding game rules, gas policy, or LLM dependency.

## Future products and enabling implications

Future or separate products that may consume Moria (not built here):

- System-driven ARPG and related spell/gas/combat experiences
- Dwarf Fortress–style fortress/colony play
- Moria-style descent / adventure and pure sandbox titles

High-level enabling implications only: consumers need public mutation and query
interfaces under the command/mirror contract, material and placement registries,
and seams for game-injected policy (for example pricing of verbs) without the
substrate owning those policies. The architecture reference also describes
deeper matter families (multi-tier fluids, integrity, granular behavior, fire
ecology, object dynamics, building stamps/mechanisms, derived nav) as the kind
of world services such games eventually want. Those families shape long-horizon
layering and seams; they are **not** imported as current-milestone gameplay,
content, or required implementation scope beyond the Product One slice above.

Gameplay, content, presentation, controllers, characters, and game-specific
systems remain consumer-owned.

## Non-goals

- Implementing the game, System/LLM, spells, gas/intent pricing, combat, AI, or
  game building layers in this repository
- Treating the validation harness’s demo fantasy (specific character, route,
  postcard seed inventory, clip goals, or machine gates) as Moria’s product
  definition
- Making the substrate depend on an LLM or in-process game ruleset
- Shipping CA/fire, flowing fluids, structural integrity, granular settle, or
  object felling as part of the binding first delivery (format/seams may
  anticipate them; implementation is not promised in Product One)
- Multiplayer product delivery as a current commitment (command-style
  boundaries may remain design-friendly; online service is not promised here)

## Confirmed vision constraints

- **Ecosystem:** Moria is a Rust crate (or small family of tightly scoped Rust
  crates) for integration by Rust consumers, with a Cargo workspace boundary
  between substrate and validation harness.
- **GPU residency and observability:** World matter is GPU-resident with a
  FleX-pattern consumer contract—commands in, stale mirror plus events out—so
  consumers never treat live voxel storage as a direct upper-layer surface.
- **Portable GPU path:** Load-bearing GPU work stays on a portable wgpu/WGSL
  path rather than a native Metal-only fork; portability across backends is
  part of the crate promise.
- **Consumer isolation:** The walkable-world validation executable and any
  other in-repo harness must use only public substrate interfaces available to
  an external game—no privileged harness-only world paths.
- **Standalone engine:** Zero LLM/System dependency inside the substrate; those
  remain optional game-layer clients.
- **Scope exclusion:** Game rules and the listed future game layers stay out of
  this repository’s product.
- **Milestone authority:** For the current delivery, Product One selects which
  portions of the architecture reference are required; the reference does not
  silently expand first-delivery scope.

## Product One harness governance

Harness-specific decisions already bound by Product One—controls, demo content
and seed inventory, presentation, character/camera, platforms, benchmarks, and
acceptance thresholds—stay outside this vision’s main narrative. They are
**not** deferred open questions. They remain governed by
`docs/seeds/product-one-seed.md` for the first delivery and its validation
harness.

## Deferred design decisions

- Crate split, API surface shape, and internal layering within the substrate
  family
- Representation, meshing algorithm, storage layout, generation pipeline
  detail, and sim schemes that realize the outcomes above
- Capability depth and delivery sequence after the Product One slice (including
  when and how reactive-matter families grow)
- Whether and how far multiplayer-authoritative deployment is pursued later
- Open technical questions left for measurement under the Product One
  benchmark bed (voxel size, LOD strategy, object-layer scaling, and similar)

## Assumptions proposed for approval

None beyond the seed reading summarized in **Seed synthesis**. Product
identity, boundary, and first-delivery depth are treated as settled by the
manifest; no silent choice of human intent was required.

## Questions for human review

None. The seeds agree that:

1. The **current product** is the reusable Moria voxel-world substrate (not a
   game).
2. The **walkable-world executable** is a required adjacent validation harness
   that must consume public APIs only.
3. **Product One** binds first-delivery substrate depth and harness proof
   obligations; the architecture reference supplies purpose, layering, and
   long-horizon context without authorizing full-stack implementation “now.”
4. Future games and excluded layers (System/LLM, spells, gas, combat, AI,
   building gameplay) remain out of this repository.

If human review intends a different authority order—for example treating the
full architecture reference as current-delivery mandate, or demoting the
walkable harness to optional—that would change this brief’s boundary and should
be stated explicitly.

## Seed synthesis

| Seed | Contribution | Held as context / not imported as current product |
|---|---|---|
| **`README.md`** | Names Moria as the reusable GPU-resident voxel-world Rust substrate; identifies the walkable-world executable as a separate consumer and validation harness (generation, streaming, meshing, editing, collision, persistence, performance)—not a game layer. | Harness implementation, content, and numbers. |
| **`docs/seeds/project-boundary.md`** | Binding product identity: substrate crate(s) only; game is downstream; harness must use public interfaces; workspace boundary required; game rules and System/LLM/spell/gas/combat/AI/building **layers** out of scope (seams allowed, implementations not). | Crate graph details; full matter roadmap. |
| **`docs/seeds/product-one-seed.md`** | Binding first delivery: substrate slice depth; dig/place as mutability proof; curated-region validation spirit; portable wgpu/WGSL and M4-oriented engineering notes as design pressure; milestone arc (hill → carve → geology → dress → traverse → numbers). Also binds harness decisions (controls, content, presentation, platforms, benchmarks, thresholds) that this vision keeps outside its main narrative—see **Product One harness governance**. | Character/camera/seed inventory, performance tables, clip goals, and milestone schedule—not product identity, and not deferred; governed by this seed as harness detail. Demo content (specific biomes, ruin, material list) does not define the crate’s public product identity. |
| **`docs/seeds/voxel-world-substrate.md`** | Architecture reference: purpose (normal-looking mutable world, deep Z, substrate-not-game); layering; voxel truth vs mesh view; sparsity/lazy materialization; multi-game rationale; enabling families (fluids tiers, integrity, building verbs, nav, persistence model); zero LLM dependency; GPU command/mirror pattern. | Mechanism inventory (brick size, bit layouts, algorithms), full reactive stack as current-milestone requirements, and future-game content/fantasy. Per seed manifest: only portions selected by Product One are required for this milestone. |
| **`docs/seeds/README.md` (manifest)** | Authority order: Product One binds implementation + harness for this milestone; architecture doc is reference; boundary doc is operator clarification; broader game intent deliberately out of scope. | — |

**Contradiction handling:** Product One’s “product-shaped demo” language is
subordinated to the explicit boundary and README: the **current product** is
the substrate; the walkable world is the **harness**, not a second product
identity. Where Product One and the full substrate reference disagree on depth,
first-delivery requirements follow Product One; long-horizon purpose and seams
follow the reference without expanding “now.”

---

*This document is a proposed canonical vision for human approval and design
handoff. It is not a GDD, technical design, requirements catalog, or feature
inventory.*
