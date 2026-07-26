# Project vision

## What we are building now

**Moria** is a reusable, **GPU-resident voxel-world substrate** exposed as a **Rust crate** (or a small family of tightly scoped Rust crates). Games and tools consume it as a library foundation for material worlds—not as a finished game.

## Purpose

Moria exists so multiple games can share one engine-grade world foundation: a natural-looking surface and deep underground whose **voxels are the material truth**, fully mutable, queryable, and free of game rules, LLM dependency, or genre policy. The substrate stands alone; game systems live above it.

## Product boundary

**This product owns** the reusable world substrate: matter representation and mutation/query surfaces, geology-first generation, non-authoritative presentation of that truth, streaming and persistence of world material state, and the matter-system capabilities (interactable matter, fluids as matter, structural behavior hooks, ambient material response) that keep the world honest without encoding gameplay.

**This product does not own** any finished game. The actual game is a separate downstream consumer outside this repository. Game rules and the System, LLM, spell, gas, combat, AI, and building layers are out of scope here; compatibility seams may be designed where substrate requirements demand them, but those layers are not implemented in this product.

**Adjacent, not identity:** a walkable-world executable may exist as a validation harness. If present, it must use the same public interfaces available to an external game and must not own privileged or game-specific substrate paths. Character control, camera, authored demo regions, presentation polish, harness workloads, and acceptance scenes belong to that harness (or other consumers)—not to the substrate’s product identity. Whether shipping such a harness is part of current delivery is open (see Q1).

## Required product outcomes

1. **Library substrate for external games** — Downstream games integrate Moria as Rust crate(s) through public APIs only; the substrate remains reusable across sandbox, adventure, fortress-style, and similar consumers without embedding their rules.
2. **Material world truth** — Consumers get a continuous natural surface world and first-class deep underground where the voxel field is authoritative matter; extracted mesh and dressing are views of that truth, not a separate decorative world.
3. **Everywhere-mutable matter** — Any material volume can be destroyed, altered, or placed through public dig/place and related mutation/query verbs so play and tools operate on the same truth as rendering.
4. **Geology-first, sparse worlds** — Generation yields real geology (terrain, strata, voids, materials, placement metadata) with lazy presence so large regions stay tractable without eager full-volume cost.
5. **Coherent matter system** — The substrate supports interactable voxel-backed world features, static and dynamic fluid as matter, structural/support behavior, and related material simulation hooks without implementing game policy, UI, or content authorship.
6. **Streamable, restorable world state** — World truth is generation plus edits, streamable around activity and restorable for consumers that leave and re-enter altered material worlds.

## Future products and enabling implications

Future **consumers** (not this product) include a System-driven ARPG, fortress/colony-style play, a Moria-style descent experience, and pure sandbox modes. They motivate—but do not ship inside—this substrate.

**Enabling implications (high level only):** shared dig/build-anywhere material worlds; deep-Z geology as play space; clean layering so gas pricing, spells, labor, combat, and AI stay game-side; and a command/query boundary suited to sandboxed tools and later multiplayer-minded architectures. Gameplay, controllers, characters, authored content, presentation, and game-specific policy remain consumer-owned.

## Non-goals

- Implementing game rules or the System, LLM, spell, gas, combat, AI, or building layers in this repository
- Shipping the actual game (or treating the validation harness as a game layer)
- Making decorative heightmap-and-props scenes the product identity when the claim is a fully material world
- Absorbing harness-only scope: demo routes, character/camera feel, curated seed content inventories, device-specific performance gates, or consumer-chosen graphics backends as substrate identity

## Confirmed vision constraints

- Product form is a **Rust** crate or small family of tightly scoped Rust crates
- **GPU-resident** world substrate (library product, not a CPU-only world service)
- Consumers—including any validation harness—use **public interfaces only**; no privileged in-repo game paths
- **Zero LLM dependency** in the substrate; the System is a future game-layer client
- Game and building layers are **not implemented here**; seams only where substrate needs demand them
- The **consumer boundary** between substrate and adjacent executables is mandatory at product level (enforcement mechanism is design)

## Deferred design decisions

- Exact crate split, internal module boundaries, and storage/meshing/simulation mechanisms
- Capability depth and delivery sequence for generation, matter sim, fluids, integrity, objects, and API surface (first demo slice vs full substrate responsibility)
- Voxel resolution, LOD strategy, object-layer scaling, and fluid-model fidelity
- Whether and how a walkable-world harness is structured, what it contains, and on what platforms it is measured
- Persistence encodings, streaming ring policy, and any multiplayer deployment beyond the public command/query boundary

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a **required current delivery** of this effort, or only a **permitted adjacent artifact** beside the substrate crates?

- **Proposed safe answer:** Permitted adjacent artifact—product identity and committed delivery remain the substrate crates; a harness may exist and, if built, must use public APIs only.
- **If answered differently:** Making the harness required adds an adjacent deliverable (still not game content or product identity) that design must plan to ship; “permitted only” allows a substrate-only repository without a walkable executable.

## Seed synthesis

- **`README.md`** — Names Moria as the reusable GPU-resident voxel-world substrate (Rust crate) and positions the walkable-world executable as a separate validation consumer, not a game layer.
- **`docs/seeds/project-boundary.md`** — Binds identity to the substrate crates, excludes the actual game and named game layers from the repo, permits a public-API-only validation harness, and makes the consumer boundary non-optional.
- **`docs/seeds/product-one-seed.md`** — Motivates first-slice proof of a material walkable world and dig/place honesty; its demo content, controller, platforms, and performance gates stay harness/consumer scope and do not redefine product identity.
- **`docs/seeds/voxel-world-substrate.md`** — Authorizes substrate outcome families (natural material worlds, deep-Z geology, mutability, GPU-resident matter systems, streaming/persistence, reusable layering) for design without importing game layers or mechanism inventories into this brief.
