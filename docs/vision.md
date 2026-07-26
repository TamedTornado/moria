# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is the world-and-matter engine layer external games and tools consume—not a game, demo, or content pack.

## Purpose

Moria exists so multiple games can share one material world foundation: a natural-looking surface over fully mutable voxel truth, with deep underground as first-class space, without each title rebuilding geology, matter behavior, dig/build mutability, observation, derived services, streaming, or persistence. It stands alone with no dependency on any game’s rules, LLM, or economy.

## Product boundary

- **In product:** the reusable GPU-resident voxel-world substrate as Rust crate(s); public command, query, and event interfaces; geology-backed world state; material matter and object coupling; world-physics and ambient environmental behavior; universal dig/build and reusable construction/structure representation; mutation-safe derived services including multi-affordance 3D navigation; streaming; durable restoration of world state.
- **Out of product:** the actual game (any title on Moria)—a separate downstream consumer outside this product’s identity and repository scope.
- **Out of product:** game rules and System, LLM, spell, gas, combat, AI, and building-*gameplay* layers (work orders, labor, economy, fortress policy, building UI). Compatibility seams may be designed where required; those layers are not implemented here.
- **Out of product:** gameplay, UX, controllers, cameras, characters, authored demo routes, presentation policy, and game-specific acceptance scenarios—even when they motivate substrate work.
- **Adjacent only:** a walkable-world executable, if present, is a validation harness—not a game layer—and must use public interfaces only, with no privileged paths. Whether it is a required repository delivery is unresolved (**Q1**). Product One’s first-slice exclusions constrain that adjacent story only; they do not shrink substrate required outcomes.

## Required product outcomes

- **Reusable public boundary.** Downstream games and hand-authored tools integrate through the same public Rust interfaces, shared material/content registries, substrate verbs and queries, and injected policy—not privileged paths into voxel storage.
- **Natural material world, deep Z, and universal dig/build.** The world can read as ordinary outdoor terrain while remaining fully material: any voxel can be destroyed, moved, or placed (Minecraft-grade dig/build anywhere). Underground volume is first-class content space, produced by geology-first generation that can materialize on demand so large regions need not live fully expanded when untouched.
- **Reusable construction and structure substrate.** Generic placement, reusable structure/stamp representation, matter-coupled mechanism objects, and semantic structure metadata belong here so building and fortress-style consumers can attach later. Construction UI, work policy, labor, economy, and fortress gameplay remain consumer-owned.
- **Commands in; stale mirror and events out.** Consumers mutate exclusively through substrate verbs. Observation separates a potentially stale, non-authoritative mirror from event output. Physics, collision, and other authoritative world operations use voxel truth—not render meshes, dressing, or the mirror. Meshes and dressing are views of that truth, never sources of authority.
- **Dynamic material behavior, ambient world, and matter-vs-dressing.** Material worlds and interactable voxel-backed objects behave and stay consistent with voxel truth—including fluids, granular settle, ambient fire and wetness, structural support and collapse, and dynamic matter coupling. Environmental time, seasons, weather effects, growth, and fire ecology are thin but present ambient substrate behavior. Anything that can burn, break, or block is voxel-backed; non-material dressing stays anchored to and responsive to voxel truth. Mechanism detail and delivery sequence remain design choices.
- **Streaming, persistence, and mutation-safe multi-affordance navigation.** Scalable streaming of independently active regions; durable cross-run restoration of generated truth plus persistent mutations, object state, and other relevant world state. Substrate-owned spatial/structural metadata and mutation-safe navigation across different 3D traversal media and movement affordances (continuous-Z, not surface-walker alone) stay synchronized as the world mutates. Consuming AI, work policy, UI, economy, and other game behavior stay outside the product.

## Future products and enabling implications

Future *consumers* (not this product) include a System-driven ARPG, a DF-style fortress/colony game, a Moria-style descent adventure, and pure sandbox tools. They own rules, content, presentation, agents, pricing policy, and gameplay. Substrate outcomes above enable them without reimplementing the material world; game layers are not built here.

A walkable “product one” region with a character controller is an adjacent consumer/validation story. Its route, content, debug keys, performance gates, hardware targets, and first-slice cuts are not Moria outcomes (**Q1**).

## Non-goals

- Shipping a playable commercial game, ARPG, fortress mode, or descent roguelike.
- Implementing System/LLM authorship, spells, gas metering, combat, AI agents, or building-*gameplay* systems (work orders, labor, economy, fortress UX)—without excluding substrate dig/build, placement, stamps, mechanism objects, or structure metadata.
- Treating the harness’s character, camera, demo seed, or benchmark protocol as substrate features.
- Making the mesh, dressing, or mirror authoritative over matter or physics.
- Treating Product One’s first-slice omissions (e.g. weather, seasons, growth) as substrate non-goals or as proof that ambient cycles, dynamic matter, construction, streaming, persistence, events, or multi-affordance navigation are optional.

## Confirmed vision constraints

- **Integration form:** exposed as a Rust crate or small family of tightly scoped Rust crates for game consumption.
- **Consumer isolation:** any adjacent harness or external game uses only public interfaces; privileged substrate paths are forbidden.
- **GPU-resident observation model:** GPU-resident voxel-world infrastructure with commands in and a stale mirror plus events out; voxel truth remains authoritative for physics and world operations.
- **Repository scope:** the actual game lives outside this product; game-layer systems under Non-goals are not implemented here.
- **First-slice ≠ product identity:** exclusions that limit an adjacent validation slice do not make independently assigned substrate outcomes optional.

## Deferred design decisions

- Mechanism, tier, tuning, and delivery sequence for material behavior, fluids, integrity, ambient cycles, dressing, object coupling, construction/stamp representation, and related world-physics detail—the outcomes remain required.
- Representations and encodings for streaming, persistence, and save restoration.
- Exact public API shape, crate split, and expression of compatibility seams and injected policy without implementing game layers.
- Depth of any first validation slice; harness-only concerns if delivered (controllers, demo content, platforms, performance gates).

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world executable a **required current repository delivery**, or only a **permitted** adjacent validation artifact?

- **Proposed safe answer:** Permitted only—a harness may be delivered and, if present, must use public substrate interfaces, but is not mandatory for the product to exist as reusable crate(s). Either answer leaves substrate required outcomes unchanged.
- **If different:** Making the harness mandatory adds a current adjacent deliverable (still outside product identity) that design must plan; it does not move controller, content, or performance gates into the substrate.

## Seed synthesis

- **`README.md`:** Names Moria as a reusable GPU-resident voxel-world substrate consumed as a Rust crate; situates the walkable-world executable as a separate consumer/validation harness, not a game layer.
- **`docs/seeds/project-boundary.md`:** Binding boundary—product is the substrate crate(s); game out of repo; harness only via public APIs; game/System/LLM/spell/gas/combat/AI/building layers stay out of implementation scope.
- **`docs/seeds/product-one-seed.md`:** Adjacent first-slice demo motivating material mutability and a walkable natural region; controller, content, hardware, metrics, and first-slice exclusions stay consumer/harness-owned and do not shrink substrate mandate.
- **`docs/seeds/voxel-world-substrate.md`:** Authoritative substrate outcomes—natural look over voxel truth, universal dig/build, construction/structure substrate, deep Z, geology-first world, dynamic material and ambient environmental behavior, matter-vs-dressing rule, streaming and persistence, commands-in / stale-mirror-and-events-out, mutation-safe multi-affordance 3D navigation and derived services, policy-independent reuse—without importing mechanism inventory.
