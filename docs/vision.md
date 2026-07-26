# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is the world-and-matter engine layer that external games and tools consume—not a game, demo, or content pack.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface over fully mutable voxel truth, with deep underground as first-class space, without each title rebuilding geology, matter, mutation, and world queries. The substrate stands alone with no dependency on any game’s rules, LLM, or economy layer.

## Product boundary

- **In product:** the reusable GPU-resident voxel-world substrate as Rust crate(s); public interfaces; ownership of geology-backed world state, material matter, mutation/query surfaces, and world-physics responsibilities assigned to this layer so consumers need not reimplement the material world.
- **Out of product:** the actual game (any title on Moria)—a separate downstream consumer outside this product’s identity and repository scope.
- **Out of product:** game rules and System, LLM, spell, gas, combat, AI, and building *game* layers (compatibility seams may be designed where required; those layers are not implemented here).
- **Out of product:** gameplay, UX, controllers, cameras, characters, authored demo routes, presentation policy, and game-specific acceptance scenarios—even when they motivate substrate work.
- **Adjacent only:** a walkable-world executable, if present, is a validation harness—not a game layer—and must use public interfaces only, with no privileged paths. Whether it is a required repository delivery is unresolved (**Q1**).

## Required product outcomes

- **Reusable crate boundary.** Downstream games integrate through the same public Rust interfaces; nothing above the substrate’s public surface needs private access to voxel storage or internal paths.
- **Natural look, material truth.** The world can read as ordinary outdoor terrain (hills, forest, water, cliffs, caves) while remaining fully material: what the player would treat as solid world is mutable voxel matter, not decorative geometry outside the material model.
- **Mutation everywhere.** Consumers can destroy, place, and reshape matter through substrate-owned verbs and queries; rendering views are non-authoritative reconstructions of that truth.
- **Deep Z is first-class.** Underground space is real content volume—geology, caves, strata, and depth—not a thin floor under a heightmap.
- **Geology-first generation under load.** World generation produces a diggable, continuous 3D world that can materialize on demand so large regions need not reside fully as expanded voxels when untouched.
- **Substrate services for consumers.** The product provides matter, world-physics responsibilities assigned to this layer, queries, and mutation so ARPG, fortress, descent, or sandbox games can sit above it without embedding game policy in the crate.

## Future products and enabling implications

Future *consumers* (not this product) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent adventure, and pure sandbox tools. They own rules, content, presentation, and agents.

Enabling implications for the substrate (not a committed feature roadmap): keep public mutation/query boundaries suitable for multiple pricing and rule policies; keep geology and matter rich enough that fortress, descent, and adventure fantasies can attach later; allow compatibility seams without implementing those game layers here.

A walkable “product one” region with a character controller is described only as an adjacent consumer/validation story that can prove substrate claims. Its route, dressing inventory, debug keys, performance gates, and hardware targets are not Moria outcomes (see **Q1** for delivery status of any harness).

## Non-goals

- Shipping a playable commercial game, ARPG, fortress mode, or descent roguelike in this product.
- Implementing System/LLM authorship, spells, gas metering, combat, AI agents, or building-game systems (blueprints-as-gameplay, work orders, mechanisms-as-game entities, room economy) inside Moria.
- Treating the validation harness’s character, camera, demo seed content, or benchmark protocol as substrate features.
- Making the mesh, dressing, or any render view authoritative over matter.

## Confirmed vision constraints

- **Integration form:** exposed as a Rust crate or small family of tightly scoped Rust crates for game consumption.
- **Consumer isolation:** any adjacent harness or external game uses only public interfaces; privileged or game-specific substrate paths are forbidden.
- **GPU-resident world substrate:** the product is defined as GPU-resident voxel-world infrastructure, not a CPU-only offline toolkit.
- **Repository scope:** the actual game lives outside this product; game-layer systems listed under Non-goals are not implemented here.

## Deferred design decisions

- Delivery depth and sequence of substrate capabilities (meshing approach, generation stages, fluid tiers, integrity, dressing, object coupling, streaming/persistence encodings, and related open technical choices).
- Exact public API shape, crate split within the family, and how compatibility seams are expressed without implementing game layers.
- How far any first validation slice goes relative to the full substrate responsibility set.
- Harness-only concerns if a harness is delivered: controllers, demo content, platforms, and performance gates.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world executable a **required current repository delivery**, or only a **permitted** adjacent validation artifact?

- **Proposed safe answer:** Permitted only—repository delivery of a harness is allowed and, if present, must use public substrate interfaces, but it is not mandatory for the product to exist as the reusable crate(s).
- **If different:** Making the harness mandatory adds a current adjacent deliverable (still outside product identity) that design must plan and validate; it does not move controller, content, or performance gates into the substrate.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and situates the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binding boundary—product is the substrate crate(s); the game is out of repo; harness may exist only via public APIs; game/System/LLM/spell/gas/combat/AI/building layers stay out of implementation scope.
- **`docs/seeds/product-one-seed.md`:** Adjacent first-slice demo narrative that motivates proving material mutability and a walkable natural region; its controller, content, hardware, and metrics stay consumer/harness-owned and do not redefine Moria as that demo.
- **`docs/seeds/voxel-world-substrate.md`:** Authoritative substrate purpose and outcome altitude—natural look over voxel truth, full mutability, deep Z, geology-first world, GPU-resident layering, multi-game reuse—without importing its mechanism inventory into this brief.
