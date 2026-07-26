# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is the material-world engine layer that downstream games consume—not a game, ARPG, or fortress product.

## Purpose

Moria exists so multiple games can share one trustworthy world of matter: continuous 3D space whose voxel truth supports a natural-looking surface, deep underground play, dig/place mutability, and matter that behaves under physical and ambient rules—without embedding any game’s rules, presentation, controllers, or content. Games inject policy above; the substrate provides matter, generation, queries, mutation, and world services, and must stand alone with no LLM dependency.

## Product boundary

**In product:** the reusable substrate and its public interfaces for generation and materialization; matter representation and mutation; presentation views regenerated from voxel truth; world queries; collision against voxel truth; edit-coherent navigation and spatial information for continuous 3D movement; streaming; durable restorable world state; matter physics and ambient simulation at capability altitude; and consumer-neutral command, query, event, and authoring facilities (including substrate registries) shared by external games and any optional System client—no privileged System or game path.

**Adjacent, not product identity:** a walkable-world executable may exist as a validation harness on those same public interfaces. Whether shipping it is a required current delivery is open (see Q1). If delivered, its validation purpose is generation, streaming, meshing, editing, collision, persistence, and performance against a generated natural region with continuous walkable surface-to-underground traversal and dig/place proof. Its character, controller, camera, authored route and content, presentation, debug UX, platform, and machine-specific performance gates are not substrate scope.

**Out of this product and repository:** the actual game; game rules; System/LLM features; spells; gas/pricing policy; combat; AI; and building-as-gameplay layers (UI, work orders, designations, game economy).

## Required product outcomes

Downstream design must make these true for the substrate:

1. **Voxel truth with a natural look.** The world is a fully material voxel volume. Presentation (smooth terrain, surface dressing) is a view regenerated from that truth, never the source of physics or mutation. Surface and underground can read as ordinary terrain—hills, forest, water, rock, caves—rather than a cube aesthetic as the default look, while remaining editable matter underneath.
2. **Mutable continuous 3D, deep Z first-class.** Consumers can destroy, place, and reshape matter throughout the volume, surface through deep Z. Continuous 3D space supports meaningful underground content and traversal; depth is not a fake floor under a heightmap, and mutability is not decorative geometry outside the material model.
3. **Geology-oriented generation and sparse materialization.** Worlds can be produced so digging and exploration encounter real strata, voids, and materials, with lazy materialization and sparse representation so large regions remain practical.
4. **Matter that behaves.** Substrate-owned matter services—not game rules—include: voxel-backed objects that burn, break, block, move, and remain matter through detachment and falling (including falling trees as a non-negotiable consequence of felling); granular settling; fluid behavior; material-dependent structural failure and cave-ins; and thin-but-present time, weather, wetness, growth, and fire ecology so the natural world behaves rather than only looking natural. Mechanisms, fidelity tradeoffs, and delivery order are design choices; these outcome families are mandatory for the product.
5. **Edit-coherent spatial services.** Mutation-safe navigation and spatial information for continuous 3D movement are derived from voxel truth and stay coherent under world edits. AI path policy and game movement rules remain consumer-owned.
6. **Durable world state and one public consumer surface.** The world is reproducible and restorable across runs via generated truth plus durable deltas for scars and journals for moved or felled world objects and associated state, enabling cross-run reuse (storage encoding and game save policy not prescribed). External games and optional System clients use the same command, query, event, and authoring facilities, including substrate registries—no private voxel access above the matter layer, no privileged System path.

## Future products and enabling implications

Future consumers—not current product—include a System-driven ARPG, a fortress/colony-style game, a Moria-style descent experience, and pure sandbox modes. The substrate remains a reusable Rust crate stack those titles can sit on without forking privileged world implementation. Their gameplay, UX, controllers, content, and presentation are not imported here. First-slice depth for any consumer does not narrow substrate outcome families or product identity.

## Non-goals

- Shipping a playable game, campaign, or game-mode ruleset in this repository
- Implementing System/LLM, spell, gas, combat, AI, or building-game layers here
- Folding harness demo content, controls, routes, platform choices, or machine-specific performance gates into product identity
- Making the substrate depend on an LLM or game System to function

## Confirmed vision constraints

- **Identity:** reusable GPU-resident voxel-world substrate, not a game layer
- **Integration:** Rust crate or small family of Rust crates for external game consumers
- **Consumer isolation:** any validation executable and any external game share one public interface surface; adjacent consumers get no privileged substrate access
- **Standalone engine:** zero LLM/System dependency for the substrate to operate
- **Repository scope:** the actual game is out of this repo; excluded game-rule layers are not implemented here (seams only where required)

## Deferred design decisions

- Crate split, API shape, storage layout, meshing approach, and simulation algorithms
- Delivery depth and order among authorized substrate outcome families (which matter and ambient services ship when, at what fidelity)
- Open technical tradeoffs left by the seeds (resolution, LOD, object-layer scale, fluid fidelity, multiplayer readiness)
- Whether, when, and how a walkable-world harness is built beyond the validation purpose under Product boundary (see Q1)
- Platform backends and machine-specific performance targets

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** of this repository, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted only—the product promise is the substrate crates and public interface; a harness may exist to exercise them but is not itself a committed deliverable until design plans it.
- **If answered differently:** Requiring the harness keeps product identity on the substrate but adds an adjacent delivery obligation. That obligation’s validation purpose remains generation, streaming, meshing, editing, collision, persistence, and performance against a generated natural region with continuous walkable surface-to-underground traversal and dig/place proof—still without importing its controller, content, presentation, platform, or machine-specific gates into the product boundary. Treating it as merely permitted leaves harness work optional relative to substrate outcomes.

## Seed synthesis

- **README.md** — Names Moria as a GPU-resident voxel-world substrate consumed as a Rust crate, and frames the walkable-world executable as a separate consumer/harness for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md** — Binds product identity to the reusable Rust substrate, keeps the real game out of repo, permits a public-API-only validation harness, and excludes game-rule, System/LLM, spell, gas, combat, AI, and building layers here.
- **docs/seeds/product-one-seed.md** — Motivates first-slice walkable validation (generated natural region, continuous surface-to-underground traversal, dig/place proof); controller, content, platform, and performance detail stay harness/design depth and do not demote substrate outcome families—hence Q1 for harness delivery only.
- **docs/seeds/voxel-world-substrate.md** — Authorizes substrate outcome families: natural look over voxel truth, full mutability, deep Z, geology generation, matter physics and ambient behavior, edit-coherent spatial services, durable restorable world state, multi-game reuse, and consumer-neutral integration with no LLM dependency.
