# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident voxel-world substrate, exposed as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer material world for external game consumers—not a game, not game rules, and not the walkable validation harness that may sit beside it.


## Purpose

Give multiple future games one shared foundation: a natural-looking surface world over continuous, fully mutable voxel matter that stays honest underground, with matter physics, generation, observation, mutation, and the seams consumers need. Game rules, economy policy, AI, controllers, and game-specific presentation live above this product.

## Product boundary

**In product:** geology-first world generation; GPU-resident voxel matter with mutation and substrate-assigned material/physics outcomes; non-authoritative terrain presentation plus voxel-backed natural objects and voxel-coupled dressing; derived mutation-aware spatial support over continuous 3D matter; public command/observation contract (commands in; potentially stale mirror plus events out; nothing above matter touches voxels directly); streaming and exact cross-run persistence as generation plus matter deltas and object/entity journals; reusable Rust-crate integration for external games.

**Out of product:** the actual game (downstream consumer, not this repository); game rules; System/LLM; spells; gas policy; combat; AI and agent behavior; building layers as gameplay. Seams may be designed where substrate requirements demand them; those layers are not implemented here. Game-specific presentation, authored consumer content, controllers, characters, and UX remain consumer-owned.

**Adjacent artifact:** a walkable-world executable may exist as a validation harness. Whether it is a required delivery is open (Q1). If present, it must use the same public interfaces as an external game—no privileged paths. Harness character control, camera, curated route, fixture content, presentation, workloads, device gates, and performance numbers are not substrate identity.

**Established first slice (Product One substrate scope, distinct from harness optionality):** full generation foundation (columns, strata, caves, ore, lazy materialization, neutral POI metadata; continent pass may be stubbed); sparse GPU matter with incremental smooth terrain presentation; grass/clutter dressing and registered voxel objects without felling/rigid conversion; static water only; dig/place/query via the public boundary; streaming and exact-restore persistence. Broader matter-and-physics outcomes remain product identity; the partial matter slice does not remove them.

## Required product outcomes

1. **Reusable Rust substrate.** Downstream games integrate a GPU-resident voxel world as crate consumers without embedding their rules into Moria.
2. **Geology-first material world with deep Z.** Generation yields continuous natural landscape and underground truth—columns, strata, caves, ore, lazy materialization, neutral POI metadata—as walkable, diggable, placeable matter, not a heightmap with non-matter props. Surface and depth share one world model. This foundation is a first-slice deliverable; only the continent pass may be stubbed.
3. **Voxel truth, derived views.** Queries and mutations operate on voxel matter; terrain meshes and voxel-coupled dressing are regenerated views, never saved or authoritative. Substrate owns terrain material presentation, voxel-backed natural objects, and dressing anchored to voxels; game-specific look and authored consumer content do not.
4. **Matter that moves and reacts.** Substrate mandate includes movable matter, interactive voxel-backed objects, material simulation, active fluids, ambient/fire behavior, granular response, and structural failure—consumer-visible physical honesty, not optional later work. Product One exercises a partial matter slice (static water; no CA/fire/active fluids/integrity/granular/felling yet) while preparing for the full mandate.
5. **Mutation-aware spatial support and command/observation contract.** Substrate derives navigation-relevant structure over mutable continuous 3D matter; AI, agent behavior, and labor gameplay remain consumer-owned. Consumers issue commands and observe via a potentially stale mirror plus events; nothing above the matter layer touches voxels directly. Pricing and similar policy inject above the matter core.
6. **Stream and restore exactly.** Large regions idle until touched; persistence is worldgen plus edit deltas and object/entity journals; load reconstructs edited matter and relevant persistent object state exactly.

## Future products and enabling implications

Future consumers—not current Moria scope—include a System-driven ARPG, a fortress/colony game, a descent-style adventure, and a pure sandbox. Moria owns the material world, matter physics, generation, spatial support, and command/observation boundary; each consumer owns gameplay, UX, controllers, game-specific presentation, authored content, and policy.

Enabling implications in the seeds: zero LLM dependency in the engine layer; an optional System attaches as a game-layer client on public mirrors, commands, and registries; gas or labor pricing is consumer-injected policy over shared verbs.

## Non-goals

- Shipping the actual game, combat, AI, spells, gas economy, System/LLM runtime, or building gameplay in this repository
- Treating the walkable demo’s character, camera, seed route, curated content, or benchmark theater as substrate features
- Making the visual mesh authoritative for collision, queries, or saves
- Narrowing substrate identity to Product One’s partial matter slice or postponing authorized generation as “later depth”
- Treating general tools as mandated consumers (seeds establish games/modes and a validation harness)

## Confirmed vision constraints

- Delivery form is a Rust crate or small family of tightly scoped Rust crates for intended game consumers
- World matter is GPU-resident as part of product identity
- Load-bearing substrate code stays on portable GPU backends (wgpu/WGSL); a machine-specific Metal fork of those layers is rejected because cross-backend portability is the crate’s purpose. Device-class targets and numeric gates remain harness concerns
- Substrate stands alone with zero LLM dependency
- Any in-repo harness uses only public interfaces; no privileged game-specific paths
- Game and building layers are not implemented here; seams only where substrate requirements demand them

## Deferred design decisions

- Internal crate split and workspace layout that enforce the consumer boundary
- Delivery sequence and engineering detail for matter-physics capabilities beyond the established first slice
- Representation and resolution choices for voxels, storage, meshing, and runtime structure
- Validation workloads, harness content, and numeric performance budgets once Q1 is settled
- Open engineering questions left for measurement-driven design (for example voxel size and distant presentation strategy)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is the walkable-world executable a **required repository delivery** or **only a permitted** adjacent validation artifact?

- **Conflict:** `docs/seeds/project-boundary.md` says the repository *may* include the harness. `README.md` assumes that separate executable as consumer/validation harness. `docs/seeds/product-one-seed.md` treats the walkable demo and benchmarks as deliverables proving the Product One substrate slice.
- **Proposed answer:** Permitted only—identity stays the reusable substrate; a harness may exist and, if it does, must use public interfaces only.
- **If answered “required”:** the repository must also deliver a walkable-world harness that exercises the Product One substrate slice through public APIs (generation, streaming, meshing, editing, collision, persistence, performance proof)—without absorbing controller, camera, curated content, workloads, device gates, or presentation into substrate identity.

## Seed synthesis

- **README.md** — Names Moria as the reusable GPU-resident voxel-world Rust substrate and assumes a separate walkable-world executable as validation harness, not a game layer.
- **docs/seeds/project-boundary.md** — Binds identity to the Rust crate substrate, places the real game outside the repo, permits (does not require) a public-interface-only harness, and excludes game/System/building layers here.
- **docs/seeds/product-one-seed.md** — Pins the first substrate slice (full generation; partial matter; dig/place/query; streaming; exact restore; portable wgpu/WGSL) and motivates a walkable proof consumer whose controls, content, platforms, and numeric gates stay outside substrate identity.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate outcomes: geology-first generation, mutable matter and physics, terrain presentation and voxel objects/dressing, mutation-aware spatial support, command/observation coupling, generation-plus-deltas persistence, multi-game reuse without embedding game rules.
