# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate exposed as a Rust crate (or a small family of tightly scoped Rust crates). It is an engine-layer product for world matter, generation, presentation of that matter, queries, and mutation—not a game.

## Purpose

Games need a shared foundation where a natural-looking overworld and deep underground are the same mutable material world. Moria exists so downstream games can consume that foundation without embedding game rules, LLM systems, or presentation policy inside the world layer. The substrate must stand alone with zero dependency on any System or LLM client.

## Product boundary

**Belongs to Moria**

- The reusable voxel-world substrate, its public interfaces, and high-level engine responsibilities: material world state, geology-oriented generation, smooth presentation of voxel truth, mutation and query surfaces, and deep continuous sparse regions workable for external consumers.
- Optional inclusion of a walkable-world **validation harness** only as an adjacent executable that exercises the substrate through those public interfaces—never as a privileged or game-specific fork.

**Does not belong to Moria**

- The actual game (any mode) as a product of this repository, and implementation of game rules or future layers (System/LLM, spells, gas policy, combat, AI, building/gameplay); compatibility seams may be designed where substrate requirements demand them, but those layers are not delivered here.
- Harness- or game-owned concerns: character controllers, cameras, authored demo routes and scenery inventories, UI, acceptance scenes, and consumer-chosen performance or platform gates—unless later explicitly pulled into product scope by human decision.

Adjacent consumers (external games and any validation harness) must use the same public interfaces. No privileged in-repo path into substrate internals.

## Required product outcomes

A downstream design must make these product-level outcomes true:

1. **Reusable engine identity.** The product ships as Rust crate(s) consumable by external games; it provides matter, world generation, queries, and mutation without embedding game rules.
2. **Voxel truth, natural look.** The world is fully material and mutable; primary presentation reads as a continuous natural surface world, not a cube aesthetic. Rendered geometry is a view of matter, not authoritative world state.
3. **Everywhere-mutable matter.** Any material cell can be destroyed, changed, or placed through substrate mutation surfaces; dig/place is first-class proof of material honesty, not a game feature.
4. **Deep continuous space.** Underground volume is first-class content space (caves, strata, buried material), not a decorative floor under a heightmap.
5. **Geology-first generation.** Worlds are generated as material geology that supports honest digging and discovery, materializing work on demand so large regions remain tractable.
6. **Shared public contract.** All consumers—including any in-repo harness—mutate and observe the world only through public verbs and queries; nothing above the matter surface touches voxels directly. This is the reuse, sandbox, and multiplayer-readiness boundary at product altitude.

## Future products and enabling implications

Described games (System-backed ARPG, fortress/colony play, Moria-style descent, pure sandbox) are **future or external consumers**, not current Moria deliverables.

High-level enabling implications already owned at substrate altitude (not a committed feature roadmap): games will rely on mutable material worlds, deep-Z play space, natural surface reading, sparse/streamable residence, and seed-plus-delta style world truth so abandoned or shared places can persist across sessions and modes. Gameplay, content, controllers, presentation policy, and economy/pricing remain consumer-owned.

Longer substrate surfaces motivated by those consumers (richer fluid behavior, structural integrity, granular settle, fire ecology, object felling/rigid coupling, mechanism entities, nav aggregates, building stamps as game-facing systems) are **not** declared current delivery merely because a broad seed describes them; they remain design-horizon capability unless later scoped.

## Non-goals

- Shipping a playable game, combat, stats, AI, entities-as-gameplay, System/LLM integration, spells, or gas.
- Implementing building, fortress, or adventure game layers in this product.
- Treating the validation harness’s character, camera, demo route, content palette, or benchmark theater as the product identity.
- Making the substrate depend on an LLM or game policy object to function.

## Confirmed vision constraints

- **Ecosystem:** delivered for Rust consumers as a crate or small crate family.
- **Consumer equality:** harness and external games share public interfaces only; no privileged access.
- **Layering:** game rules live above the substrate; gas/pricing and System clients are policy or game-layer concerns, not substrate features.
- **LLM independence:** the substrate stands alone with zero LLM dependency.
- **GPU-resident world substrate:** product identity includes a GPU-resident material world (mechanism and device targets are design).

## Deferred design decisions

- Exact crate split and packaging layout (workspace mechanics are enforcement; the consumer boundary is not).
- Capability depth and vertical-slice order for the first shippable substrate surface (meshing approach, generation completeness, sim breadth, object-layer depth).
- Numeric environment promises (frame time, memory, platforms, backends) and harness workloads.
- Open fidelity questions (voxel scale, distant presentation, object-layer scaling, fluid fidelity) left unresolved by seeds for measurement-led design.
- Whether and how compatibility seams for excluded layers appear in public APIs.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **mandatory current delivery** of this repository, or only a **permitted** adjacent validation harness?

- **Proposed safe answer:** Permitted and expected as an adjacent harness that consumes public APIs, but not part of product identity; its specific controls, content, presentation, and acceptance gates stay out of Moria scope.
- **If different:** Mandatory delivery keeps substrate identity but adds a required adjacent artifact to the current release obligation (still without importing that artifact’s gameplay/content details into the substrate product). “Neither / not now” narrows repository deliverables to crates only.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident voxel-world substrate consumed as a Rust crate, and frames the walkable-world executable as a separate validation consumer.
- **docs/seeds/project-boundary.md** — Binding product identity and repository boundary: substrate crates in; game and excluded layers out; harness allowed only via public interfaces.
- **docs/seeds/product-one-seed.md** — Motivates dig/place honesty, material-world proof, and an adjacent walkable demo/harness path; its controllers, region content, milestones, and performance/platform gates do not expand current product identity.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate purpose and outcome families (natural look over voxel truth, full mutability, deep-Z, reusable layering, generation and material world responsibilities) without transferring full mechanism inventory or future game features into current scope.
