# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a
Rust crate or a small family of tightly scoped Rust crates. It is an
engine-layer world product—matter, generation, mutation, observation, and the
public interfaces games consume. It is not a game, not a game ruleset, and not
a character-driven experience product.

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

**In scope**

- The reusable substrate and its public consumer-facing interfaces (Rust crate
  boundary; workspace separation from the harness is required).
- Product-level world capabilities: geology-backed generated material worlds;
  sparse, lazy-materialized matter; full mutability; presentation regenerated
  from matter (not authoritative); surface dressing and interactive voxel
  objects as matter-coupled features; static fluid bodies; command/mirror-style
  mutation and observation; streaming; persistence as generation identity plus
  edit deltas.
- Compatibility seams only where substrate requirements demand them—not
  implementations of excluded game layers.

**Out of scope**

- Any actual game (ARPG, fortress, descent, sandbox, or otherwise) and its
  rules, content, UX, controllers, characters, combat, AI, economy, gas/pricing
  policy, spells, or LLM/System layer.
- Game-facing building, designation, work-order, and similar policy layers.
  Matter placement as an engine verb remains substrate; fortress/building
  *gameplay* does not.
- Privileged or game-specific paths that bypass public substrate interfaces.

**Adjacent first delivery (not product identity)**

- A walkable-world validation executable is required. It must use only public
  interfaces. Its controller, character, camera, demo content, presentation,
  and performance gates are harness concerns—not Moria’s product identity.

## First-delivery outcomes

The first delivery proves a usable depth of substrate, not a full multi-game
matter stack:

1. **Material world truth** — fully mutable voxel matter; dig and place as
   first-class capabilities; mesh and presentation are regenerated views, never
   save or collision authority.
2. **Natural surface, deep Z** — continuous natural terrain that remains
   voxel-authoritative; underground volume is real space, not a painted floor.
3. **Geology-backed generation with sparsity** — layered geology so digging
   encounters honest material depth; large regions stay tractable via lazy
   materialization and sparse storage.
4. **Matter-coupled surface** — dressing and placeable/renderable voxel objects
   at first-slice depth, plus static fluid bodies; not the full reactive stack
   (fire CA, flowing fluids, integrity, granular settle, object felling).
5. **Public mutation and observation contract** — consumers mutate and observe
   through commands and a stale mirror; harness and external games share that
   surface with no privileged in-tree path.
6. **Streaming, persistence, and measurable performance** — active regions
   stream; truth persists as seed/function plus edit deltas; performance is
   observable through public use and the harness.
7. **Reusable engine boundary** — matter, queries, and mutation without
   embedding game rules, gas policy, or LLM dependency.

First delivery is a deliberately partial slice of the long-horizon substrate
(generation for a curated region, core matter/meshing/dressing/objects/static
water, dig/place and mirror queries). Later reactive-matter families remain
purpose and seam context, not current-milestone mandate.

## Future products and separation

Downstream consumers (not built here) may include a System-driven ARPG,
DF-style fortress play, Moria-style descent, or pure sandbox. They need public
mutation/query interfaces, material/placement registries, and seams for
game-injected policy. Gameplay, content, presentation, controllers, characters,
and game-specific systems remain consumer-owned and out of this repository.

## Non-goals

- Implementing game, System/LLM, spells, gas/intent, combat, AI, or game
  building layers here
- Treating harness demo fantasy (character, route, seed inventory, clip goals,
  machine gates) as product definition
- LLM or in-process game-ruleset dependency inside the substrate
- Shipping the full reactive matter stack as part of first delivery
- Multiplayer product delivery as a current commitment

## Questions for human review

None. Seeds agree: current product is the substrate; the walkable world is a
required adjacent harness; Product One binds first-delivery depth; future games
and excluded layers stay out of this repository.

## Seed contributions (brief)

| Seed | Contributed |
|---|---|
| **README** | Names Moria as GPU-resident substrate crate; harness as separate consumer. |
| **project-boundary** | Product identity, public-interface rule, workspace boundary, out-of-scope game layers. |
| **product-one-seed** | First-delivery depth and proof obligations; harness is validation, not identity. |
| **voxel-world-substrate** | Purpose, layering, multi-game rationale; architecture reference—not full “now” scope. |

Where Product One and the architecture reference disagree on depth, first
delivery follows Product One; long-horizon purpose follows the reference without
expanding “now.”

---

*Proposed vision for human approval and design handoff—not a GDD, technical
design, requirements catalog, or feature inventory.*
