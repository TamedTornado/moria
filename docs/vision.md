# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is the world-and-matter engine layer that external games and tools consume—not a game, demo, or content pack.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface over fully mutable voxel truth, with deep underground as first-class space, without each title rebuilding geology, matter behavior, mutation, observation, derived services, streaming, or persistence. The substrate stands alone with no dependency on any game’s rules, LLM, or economy layer.

## Product boundary

- **In product:** the reusable GPU-resident voxel-world substrate as Rust crate(s); public command, query, and event interfaces; ownership of geology-backed world state, material matter and object coupling, world-physics and ambient matter behavior assigned to this layer, mutation-safe derived world services, streaming, and durable restoration of relevant world state.
- **Out of product:** the actual game (any title on Moria)—a separate downstream consumer outside this product’s identity and repository scope.
- **Out of product:** game rules and System, LLM, spell, gas, combat, AI, and building *game* layers (compatibility seams may be designed where required; those layers are not implemented here).
- **Out of product:** gameplay, UX, controllers, cameras, characters, authored demo routes, presentation policy, and game-specific acceptance scenarios—even when they motivate substrate work.
- **Adjacent only:** a walkable-world executable, if present, is a validation harness—not a game layer—and must use public interfaces only, with no privileged paths. Whether it is a required repository delivery is unresolved (**Q1**). Product One’s first-slice exclusions constrain that adjacent story only; they do not shrink the substrate’s required outcomes.

## Required product outcomes

- **Reusable public boundary and policy-independent extension.** Downstream games and hand-authored tools integrate through the same public Rust interfaces, shared material/content registries, substrate verbs and queries, and injected policy—not privileged or game-specific paths into voxel storage.
- **Natural material world with deep Z.** The world can read as ordinary outdoor terrain while remaining fully material and diggable; underground volume (geology, caves, strata, depth) is first-class content space, produced by geology-first generation that can materialize on demand so large regions need not live fully expanded when untouched.
- **Command, query, and event observation.** Consumers mutate exclusively through substrate verbs; they observe through mirror-style queries and events whose views are non-authoritative reconstructions of voxel truth. Render meshes and dressing are views of that truth, never the source of authority.
- **Dynamic material behavior.** Material worlds and interactable, voxel-backed objects behave, react, and stay consistent with voxel truth—including matter-responsive dressing, fluids, granular settle, ambient fire and wetness behavior, structural support and collapse, and dynamic matter coupling—while mechanism detail, tiers, tuning, and delivery sequence remain design choices.
- **Streaming and durable persistence.** Scalable streaming of independently active regions and durable cross-run restoration of generated truth plus persistent mutations, object state, and other relevant world state; representations and encodings are design choices.
- **Mutation-safe derived world services.** Substrate-owned spatial and structural metadata and navigation derived from voxel truth remain synchronized as the world mutates. Consuming AI, work policy, UI, economy, and other game behavior stay outside the product.

## Future products and enabling implications

Future *consumers* (not this product) include a System-driven ARPG, a Dwarf Fortress–style fortress/colony game, a Moria-style descent adventure, and pure sandbox tools. They own rules, content, presentation, agents, pricing policy, and gameplay systems.

The substrate’s required outcomes above already enable those consumers to attach later without reimplementing the material world. Compatibility seams may be designed where substrate requirements demand them; game layers themselves are not built here.

A walkable “product one” region with a character controller is an adjacent consumer/validation story that can prove substrate claims. Its route, content, debug keys, performance gates, hardware targets, and first-slice capability cuts are not Moria outcomes and do not redefine the substrate’s full mandate (see **Q1**).

## Non-goals

- Shipping a playable commercial game, ARPG, fortress mode, or descent roguelike in this product.
- Implementing System/LLM authorship, spells, gas metering, combat, AI agents, or building-game systems (blueprints-as-gameplay, work orders, mechanisms-as-game entities, room economy) inside Moria.
- Treating the validation harness’s character, camera, demo seed content, or benchmark protocol as substrate features.
- Making the mesh, dressing, or any render or mirror view authoritative over matter.
- Treating Product One’s first-slice omissions as substrate non-goals or as proof that dynamic matter, streaming, persistence, events, or derived services are optional.

## Confirmed vision constraints

- **Integration form:** exposed as a Rust crate or small family of tightly scoped Rust crates for game consumption.
- **Consumer isolation:** any adjacent harness or external game uses only public interfaces; privileged or game-specific substrate paths are forbidden.
- **GPU-resident world substrate:** the product is defined as GPU-resident voxel-world infrastructure with a consumer-facing command-in and query/event-out observation model, not a CPU-only offline toolkit.
- **Repository scope:** the actual game lives outside this product; game-layer systems listed under Non-goals are not implemented here.
- **First-slice ≠ product identity:** exclusions that limit an adjacent validation slice do not make independently assigned substrate outcomes optional or merely future.

## Deferred design decisions

- Mechanism, tier, tuning, and delivery sequence for material behavior, fluids, integrity, dressing, object coupling, and related world-physics detail—the outcomes remain required.
- Representations and encodings for streaming, persistence, and save restoration.
- Exact public API shape, crate split within the family, and how compatibility seams and injected policy are expressed without implementing game layers.
- How far any first validation slice goes relative to the full substrate responsibility set; harness-only concerns if a harness is delivered (controllers, demo content, platforms, performance gates).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world executable a **required current repository delivery**, or only a **permitted** adjacent validation artifact?

- **Proposed safe answer:** Permitted only—a harness may be delivered and, if present, must use public substrate interfaces, but it is not mandatory for the product to exist as the reusable crate(s). Either answer leaves the substrate’s full required outcomes unchanged.
- **If different:** Making the harness mandatory adds a current adjacent deliverable (still outside product identity) that design must plan and validate; it does not move controller, content, or performance gates into the substrate, and does not change the substrate mandate above.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate, and situates the walkable-world executable as a separate consumer/validation harness—not a game layer.
- **`docs/seeds/project-boundary.md`:** Binding boundary—product is the substrate crate(s); the game is out of repo; harness may exist only via public APIs; game/System/LLM/spell/gas/combat/AI/building layers stay out of implementation scope.
- **`docs/seeds/product-one-seed.md`:** Adjacent first-slice demo narrative that motivates proving material mutability and a walkable natural region; its controller, content, hardware, metrics, and first-slice exclusions stay consumer/harness-owned and do not redefine or shrink Moria’s full substrate mandate.
- **`docs/seeds/voxel-world-substrate.md`:** Authoritative substrate purpose and outcome altitude—natural look over voxel truth, full mutability, deep Z, geology-first world, dynamic material behavior, streaming and persistence, command/query/event observation, mutation-safe derived services, policy-independent reuse—without importing its mechanism inventory into this brief.
