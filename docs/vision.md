# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** exposed as a **Rust crate** (or a small family of tightly scoped Rust crates). It supplies natural-looking, fully material, deep-Z worlds that external games and tools consume through public interfaces. It is an engine layer, not a game.

## Purpose

Give future adventure, fortress, sandbox, and related games a shared material world they can trust: continuous terrain that still digs, places, and streams as real volume; underground geology that is content rather than a painted floor; and a clean consumer boundary so game rules, presentation, and policy stay outside the substrate. The substrate must stand alone with no LLM or System dependency.

## Product boundary

**This product owns** the reusable world substrate: geology-oriented generation of material volume; sparse GPU-resident matter truth; non-authoritative smooth surface presentation derived from that truth; mutation and query surfaces for dig, place, and inspection; streaming and seed-plus-delta persistence foundations; and material/object support needed so a consumer can present an honest natural region.

**Adjacent, not identity:** a walkable-world executable may exist as a **validation harness**. If present, it must use the same public substrate interfaces an external game would use—no privileged or game-specific paths inside the substrate.

**Not this product:** the actual game (separate repository/consumer); game rules; System/LLM features; spells; gas/pricing policy; combat; AI; building/gameplay layers (UI, blueprints-as-gameplay, mechanisms-as-gameplay). Compatibility seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

- **Natural continuous world, voxel truth underneath.** Surface terrain reads as ordinary landscape (hills, forest, water, cliffs, caves)—not a cube aesthetic as the primary look—while everything visible remains backed by mutable matter, not decorative geometry outside the material world.
- **Mutable everywhere, all the way down.** Any volume can be destroyed or placed; dig and place are first-class proofs that the mesh is a regenerated view, not authority. Collision and gameplay-facing queries against matter truth must be supportable by consumers.
- **Deep Z is first-class.** Strata, caves, ore, aquifers, and subsurface materials are real generated content so descent and dig-down honesty work without special-case floors.
- **Sparse, streamable, GPU-resident volume.** Large regions remain feasible without holding the full raw field in memory; idle and distant volume stays cheap; cold start and local mutation stay interactive for consumers.
- **Public verbs and queries only.** Nothing above the matter layer touches voxels directly. Adjacent consumers (including any harness) have no privileged access—the sandbox, reuse, and multiplayer-readiness boundary of the product.
- **Seed world plus edit deltas.** Untouched volume is regenerable; scars and objects persist as deltas so reload restores the same material world without shipping full voxel dumps.

## Future products and enabling implications

Future **consumers** (not current scope) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent experience, and pure sandbox tools. They own gameplay, UX, controllers, characters, authored content, presentation, and policy.

**Enabling implications** the substrate is meant to support over time (not a committed delivery catalog here): richer matter simulation (fire, wetness, granular settle, multi-tier fluids), structural integrity and cave-ins, voxel-object dynamics (e.g. felling), building placement and stamp/blueprint data paths, nav derived from matter, ambient weather/time hooks, and injectable pricing/gas policy at the script boundary. Product-one–style walkable demos and future games motivate these; they do not transfer consumer content or controls into Moria’s identity.

## Non-goals

- Shipping a game, combat, stats, AI, entities-as-gameplay, or System/LLM behavior inside this repository’s product.
- Building UI, blueprints-as-gameplay, mechanisms-as-gameplay, or other building-game layers as current product features.
- Full fluid flow simulation, weather/season/growth simulation, or multiplayer networking as current product commitments.
- Treating harness-specific character control, camera, authored demo route, art direction, or benchmark scene choreography as substrate scope.
- Native Metal (or other API) forks in load-bearing layers; the portable GPU stack is part of the product promise.

## Confirmed vision constraints

- **Rust crate (or small crate family)** as the integration form for external consumers.
- **GPU-resident** world substrate with a command-in / mirror-and-events-out style consumer coupling intent.
- **wgpu/WGSL** for load-bearing GPU work—portability across backends is intentional; no native Metal fork in those layers.
- **Supported dev environment includes Apple silicon (M4-class) via Metal through wgpu**, including the practical limit of **no 64-bit buffer atomics** on that path; bandwidth, not only compute, shapes feasible design.
- **Strict consumer boundary:** validation harness and games share public interfaces only; workspace separation of harness from substrate is the intended enforcement shape (exact package layout is design).

## Deferred design decisions

- Voxel scale, brick/LOD strategy, and object-layer capacity limits.
- Depth and order of matter features beyond the outcome mandates (CA rules, integrity, fluid tiers, granular settle, particle coupling).
- Exact public API shape, crate split within the substrate family, and persistence/streaming ring details.
- Numeric performance gates, benchmark choreography, and harness-only presentation or control choices.
- Whether and how multiplayer-authoritative deployment is pursued later (architecture may stay ready without current delivery).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world **validation harness** a **mandatory current delivery** alongside the substrate crates, or only a **permitted adjacent artifact**?

- **Proposed safe answer:** Permitted and encouraged for proving public interfaces, but **not** required for the substrate product to be considered complete; product identity remains the crate(s) alone.
- **If different:** Making the harness mandatory keeps identity on the substrate but adds a required adjacent deliverable (still without importing its controls, content, or acceptance choreography into substrate scope). Treating it as out of repository entirely would remove even the permitted harness path from this project’s boundary.

## Seed synthesis

- **`README.md`:** Names Moria as the reusable GPU-resident voxel-world Rust substrate and positions the walkable-world executable as consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binding identity and boundary—substrate crates in-repo; game out of repo; optional harness via public APIs only; game/System/spell/gas/combat/AI/building layers excluded.
- **`docs/seeds/product-one-seed.md`:** Motivates first proof of a material walkable region and dig/place honesty; harness/demo content, controllers, routes, and numeric gates inform validation intent, not substrate identity.
- **`docs/seeds/voxel-world-substrate.md`:** Defines substrate purpose and outcome families (natural look, full mutability, deep Z, reusable layering, generation/matter/API responsibilities) and long-horizon enabling capabilities for future games.
