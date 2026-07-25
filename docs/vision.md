# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate** delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer foundation for material voxel worlds: generation, sparse brick storage, smooth mesh views of voxel truth, mutation and query APIs, and the seams later games need—without implementing those games here.

## Purpose

Games that want a natural-looking surface world over a fully mutable, deep-Z voxel truth should not each reimplement geology, sparsity, meshing, streaming, and matter APIs. Moria exists so downstream games (ARPG, fortress/colony, descent, sandbox) can share one substrate: the world *reads* as ordinary terrain while remaining editable matter all the way down. The substrate must stand alone with **zero LLM dependency**; any “System” or game rules live above it as consumers.

## Product boundary

**This product (Moria)** owns the reusable substrate and its public interfaces: world generation pipeline hooks, brick/matter representation, GPU-oriented meshing and dirty-region updates, dig/place and related matter verbs, mirror-style queries/events, streaming and persistence of worldgen-plus-deltas at the engine level, and optional compatibility seams demanded by substrate requirements.

**Adjacent, not the product:** a walkable-world **validation harness** executable may live in this repository (Cargo workspace boundary). It must consume the substrate only through the same public APIs an external game would use. It is not a game layer and does not own privileged substrate paths.

**Out of this repository’s product:** the actual game(s); game rules; System/LLM authorship; spells, gas/pricing policy, combat, AI/agents as gameplay; building UI, blueprints-as-gameplay, mechanisms-as-gameplay; and full game-mode semantic layers (rooms, work orders, economy). Those are future or external consumers.

Harness-owned specifics—third-person controller and character, camera presentation, curated demo route and seed-world postcard content, debug key bindings, scripted benchmark choreography, platform/performance acceptance gates—are **not** transferred into substrate product scope merely because they exercise the crate.

## Future products and enabling implications

Described future consumers (not current Moria scope): a System-backed ARPG, a Dwarf Fortress–style fortress/colony mode, a Moria-style deep descent, and pure sandbox tools. They share the substrate; they are not this repo’s game.

**Enabling implications the substrate should keep open (high level only):** continuous 3D mutable matter with deep underground play; smooth isosurface *views* while physics/queries use voxels; geology-first generation (strata, caves, ore, aquifers) with lazy materialization and sparsity; surface dressing and voxel-backed objects as matter, not fake props; tiered fluid and integrity hooks later games can turn on; verb/query boundaries suitable for sandbox, multiplayer-readiness, and priced policies injected above the substrate. These are foundation implications, not a committed full-feature roadmap for the current phase.

## Non-goals

- Shipping a playable commercial game, combat loop, stats, or AI cast in this repository
- Implementing System/LLM, spells, gas metering, or game-policy layers inside the substrate
- Treating the validation harness’s character, UI, content set, or trailer route as product features
- Full multi-tier fluid CA, structural integrity/cave-ins, fire ecology, weather/seasons sim, granular settle, tree felling/rigid conversion, or fortress machinery as **required current deliverables** unless later vision feedback expands scope
- Native graphics API forks in load-bearing layers when portability is the point of the crate stack

## Confirmed vision constraints

- **Substrate, not game** — clean layering; game rules live above
- **GPU-resident matter path** with command-in / mirror-and-events-out style coupling for consumers
- **Voxel truth, mesh view** — render meshes are regenerated and non-authoritative
- **Public-API consumer boundary** — harness and external games share the same surface; no privileged game paths in-repo
- **Zero LLM dependency** in the substrate; System is a future client, not a substrate feature
- **Cargo workspace separation** between reusable crates and any validation executable

## Assumptions proposed for approval

1. **Current phase depth** matches the first substrate vertical slice needed to prove a walkable, diggable, smooth-looking natural region (generation, sparse bricks, meshing, dig/place, static water bodies, surface dressing, basic streaming/persistence APIs)—not the entire long-horizon matter sim catalog in one commitment.
2. **A validation harness is an expected repository deliverable** for proving the crate, but only as an adjacent consumer of public APIs; its scenario content does not redefine product identity.
3. **“Moria” names the substrate product** in this repository; fantasy “Moria” descent gameplay remains a future consumer name/association, not a second current product.

## Questions for human review

**Q1.** For *this* repository’s current product commitment, is the substrate limited to the first vertical slice (walkable mutable natural region: gen, bricks, meshing, dig/place, static water, dressing, basic stream/save), or is the full long-horizon matter stack (multi-tier fluids, integrity/cave-ins, fire CA, granular settle, building/mechanisms APIs, nav/entity substrate) also in-scope as current product?

- **Proposed safe answer:** First vertical slice only; deeper matter systems are future substrate expansion.
- **If different:** A “full substrate now” answer expands identity from a focused foundation crate to a near-complete world-sim engine and changes boundary, milestones, and what “done” means before any game exists.

**Q2.** Must the repository ship a playable walkable-world harness as part of accepting the current product, or is a library-only substrate (with tests/tools that are not a third-person walkable demo) sufficient?

- **Proposed safe answer:** A walkable harness is required for product proof, but remains a validation consumer—not a game product.
- **If different:** Library-only acceptance narrows delivery and purpose toward pure engine packaging; requiring a specific demo fantasy (character, route, postcard world) would pull consumer presentation into product scope.

**Q3.** Are dig-and-place mutation verbs part of the *substrate’s* public product surface from day one (with the harness merely calling them), or only harness/debug conveniences until a later API phase?

- **Proposed safe answer:** Dig/place (and “nothing touches voxels except through verbs/queries”) are substrate public surface; the harness only exercises them.
- **If different:** Harness-only mutation would weaken the reusable-engine claim and push the core mutability proof outside the product boundary.

## Seed synthesis

- **`README.md`** — Named the product Moria; stated reusable GPU-resident voxel-world substrate as Rust crate; framed the walkable-world executable as consumer/validation harness, not game layer.
- **`docs/seeds/project-boundary.md`** — Binding boundary: product is the substrate crate(s); actual game is out of repo; harness must use public APIs; workspace split required; System/LLM/spell/gas/combat/AI/building layers out of scope with seams only where substrate needs them.
- **`docs/seeds/product-one-seed.md`** — First-slice intent and proof narrative (smooth material world, dig as proof, non-goals for game systems); contributed harness-shaped validation scenario and performance/demo ambitions treated here as consumer/validation context, not automatic product scope; reinforced dig/place and layering “bottom two layers + API sliver.”
- **`docs/seeds/voxel-world-substrate.md`** — Design-altitude substrate goals (normal look, mutability, deep Z, reusable layering, GPU bricks, geology-first gen, objects vs dressing, fluid/integrity/building horizons); used for purpose, non-goals, and enabling implications; concrete sizes, algorithms, milestones, and acceptance numbers deferred downstream.
