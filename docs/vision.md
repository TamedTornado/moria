# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer world foundation—not a game, not a character demo, and not the future ARPG, fortress, or descent titles that will consume it.

## Purpose

Moria exists so multiple games can share one material world stack: a natural-looking surface world whose appearance is a view of fully mutable voxel truth, with deep underground geology as first-class content. Game rules, progression, presentation, and policy live above the crate; the substrate alone must be enough to generate, stream, query, mutate, mesh, and persist a walkable material world.

## Product boundary

**This product owns**

- The reusable substrate crates and their public interfaces for world generation, matter representation, meshing as a non-authoritative view, mutation and query verbs, surface dressing and voxel-object registration, sparse residency/streaming, and edit-delta persistence.
- Optional compatibility seams that future game layers may need, without implementing those layers.

**Adjacent, not this product**

- A walkable-world executable, if present, is only a validation harness. It must use the same public interfaces available to an external game and must not own privileged or game-specific substrate paths.
- The actual game(s) live in other repositories and own controllers, cameras, combat, AI, UX, authored campaigns, and game policy.

**Explicitly out of this repository’s product**

- Game rules and the System / LLM, spell, gas, combat, AI, and building *layers* (blueprints-as-gameplay, mechanisms-as-game systems, room economy, and similar). Seams may be designed where substrate requirements demand them; those layers are not implemented here.

## Required product outcomes

A downstream design must make these consumer-visible guarantees true for the substrate:

1. **Material world, not props.** Generated natural terrain (hills, forest, water, cliffs, caves, and related geology) is continuous voxel matter. What the player would walk on or dig is the same truth the mesh depicts; the mesh is regenerated from voxels and is never authoritative or the save source of truth.
2. **Mutable everywhere.** Consumers can dig and place through public verbs so any reachable volume can be scarred; cut faces remain believable matter, not broken decoration. Nothing above the matter boundary reaches voxels except through those verbs and queries.
3. **Deep Z is real.** World generation is geology-first (columns, strata, caves, ores/aquifers as material fact) with lazy materialization so large sparse regions idle cheaply and underground play is content, not a painted floor.
4. **Reads as a normal world.** Smooth isosurface-style terrain presentation plus matter-bound surface dressing and voxel-backed objects (trees, boulders, and similar) so the overworld does not read as a cube aesthetic while remaining fully material underneath.
5. **Static water bodies.** Lakes and river channels with water surfaces as tier-1 bodies; full flow simulation is not required for the current product promise.
6. **Equal external clients.** Harness and future games are peers on the public API. Persistence is worldgen seed plus edit deltas (reload restores mutations); streaming keeps only active neighborhoods resident so a full large region need not live as raw voxels in memory.

## Future products and enabling implications

Future **consumers** (not current Moria scope) include a System/LLM ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent roguelike, and pure sandbox modes. They motivate—but do not transfer into this product—gameplay, content, presentation, controllers, characters, and policy.

**Enabling implications** already described for the same substrate over a longer horizon (not a committed current roadmap): richer matter simulation (fire, wetness, granular settle, multi-tier fluids), structural integrity and cave-ins, vegetation growth and felling/rigid conversion, weather/time ambient rules, richer building/placement and mechanism hooks, nav/entity support derived from bricks, and multiplayer-ready command boundaries. Product Two and later start from a proven walkable material world, not a whiteboard.

## Non-goals

- Shipping a game, campaign, or fortress/ARPG feature set in this repository.
- Combat, stats, AI, entities beyond what a harness may bring as *its own* test client, System/LLM authorship, spells, gas/pricing policy, building UI, blueprints-as-gameplay, or mechanisms-as-game systems.
- Fluids beyond static bodies, weather/season growth simulation, and full structural/fire/CA matter sim as *current* product mandates.
- Treating the validation harness’s character, camera, debug controls, seed-route content list, or performance poster as substrate product scope.

## Confirmed vision constraints

- **Delivery form:** Rust crate or small family of tightly scoped Rust crates for integration by external games.
- **Residency model:** GPU-resident world matter substrate; portable GPU-compute intent so the crate is not locked to a single vendor API.
- **Consumer equality:** No privileged access path for in-repo harnesses versus external games; the public interface is the only interface.
- **Independence:** The substrate stands alone with zero LLM/System dependency.
- **Repository boundary:** The actual game is a separate downstream consumer, not part of this product’s identity.

## Deferred design decisions

- Precise crate split and module boundaries within the substrate family.
- Voxel size, LOD strategy, object-layer capacity limits, and meshing/extraction algorithm choices.
- How far generation, matter simulation, fluids, integrity, and object physics extend in the first implementable slice versus later substrate growth.
- Streaming-ring policy, persistence encoding, and benchmark/regression harness design.
- Hardware baselines and numeric performance gates for any adjacent validation artifact.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required current delivery** alongside the substrate crates, or only **permitted** as an adjacent validation harness?

- **Proposed answer:** Permitted and expected for proving generation, streaming, meshing, editing, collision, persistence, and performance through public APIs, but **not** part of product identity; its controller, character, content, presentation, and acceptance gates stay harness-owned.
- **If different:** Making it mandatory adds an adjacent deliverable the program must ship without changing what the substrate *is*; forbidding it leaves crates-only delivery and moves all proof burden to external consumers.

## Seed synthesis

- **README.md** — Names Moria as the GPU-resident voxel substrate Rust crate and frames the walkable-world executable as a separate validation consumer, not a game layer.
- **docs/seeds/project-boundary.md** — Locks product identity to the reusable substrate crates, requires public-API-only harness access, excludes game/System/LLM/spell/gas/combat/AI/building layers from this repo, and treats workspace separation as enforcement of a non-optional consumer boundary.
- **docs/seeds/product-one-seed.md** — Pins the first product-shaped proof: material walkable world outcomes, dig/place as mutability proof, partial matter scope, tier-1 water, and harness-oriented demo/performance detail that does not redefine the product as a game.
- **docs/seeds/voxel-world-substrate.md** — Supplies the long-horizon substrate purpose and capability envelope (natural look, full mutability, deep Z, geology-first gen, matter/objects/fluids/integrity/building seams) while confirming the layer is game-agnostic and LLM-independent.
