# Project vision

## What we are building now

**Moria** is a reusable, GPU-resident **voxel-world substrate**, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is the material-world engine layer that downstream games consume. It is not a game, not an ARPG, and not a fortress or colony product.

## Purpose

Moria exists so multiple games can share one trustworthy world of matter: continuous 3D space whose voxel truth supports a natural-looking surface, deep underground play, and dig/place mutability, without embedding any particular game’s rules, presentation, controllers, or content. Games inject policy above the substrate; the substrate provides matter, generation, queries, mutation, and related world services, and must stand alone with no LLM dependency.

## Product boundary

**In product:** the reusable substrate and its public interfaces for world generation and materialization, matter representation and mutation, derived views of voxel truth, world queries, collision against voxel truth, streaming, persistence of world edits, and compatibility seams where substrate needs demand them so future game layers can attach without living here.

**Adjacent, not product identity:** a walkable-world executable may exist only as a validation harness that consumes those same public interfaces—no privileged or game-specific substrate paths. Whether shipping that executable is a required current delivery is open (see Q1). Its character, controller, camera, authored route and content, presentation, debug UX, workloads, and performance gates are not substrate scope.

**Out of this product and repository:** the actual game; game rules; System/LLM features; spells; gas/pricing policy; combat; AI; and building-as-gameplay layers (UI, work orders, designations, game economy).

## Required product outcomes

Downstream design must make these true for the substrate:

1. **Voxel matter is authoritative truth.** The world is a fully material voxel volume. Presentation (smooth terrain, surface dressing) is a view regenerated from that truth, never the source of physics or mutation.
2. **Reads as a natural world.** Generated surface and underground can present as ordinary terrain—hills, forest, water, rock, caves—rather than a cube aesthetic as the default look, while remaining editable matter underneath.
3. **Mutable everywhere.** Consumers can destroy, place, and reshape matter throughout the volume, surface through deep Z. Mutability is product capability, not decorative geometry outside the material model.
4. **Deep Z is first-class.** Continuous 3D space supports meaningful underground content and traversal; depth is not a fake floor under a heightmap surface.
5. **Geology-oriented generation and sparse materialization.** Worlds can be produced so digging and exploration encounter real strata, voids, and materials, with lazy materialization and sparse representation so large regions remain practical.
6. **Consumer-facing world services.** Public mutation and query paths let external games run, collide, edit, stream, and persist against the same surface; nothing above the matter layer needs private voxel access. Streaming and edit-delta persistence keep large, scarred worlds viable. Matter-level behaviors multiple game styles depend on (for example fluids, structural support, and interactable surface objects versus pure dressing) are substrate responsibilities at capability altitude; delivery depth and sequence are design choices.

## Future products and enabling implications

Future consumers—not current product—include a System-driven ARPG, a fortress/colony-style game, a Moria-style descent experience, and pure sandbox modes. Enabling implication only: the substrate remains a reusable Rust crate stack those titles can sit on without forking privileged world implementation. Their gameplay, UX, controllers, characters, authored content, and presentation are not imported here. Broad long-horizon substrate capabilities described in design seeds are enabling design input for later planning, not a committed current roadmap.

## Non-goals

- Shipping a playable game, campaign, or game-mode ruleset in this repository
- Implementing System/LLM, spell, gas, combat, AI, or building-game layers here
- Folding harness demo content, controls, routes, platform choices, or performance theater into product identity
- Making the substrate depend on an LLM or game System to function

## Confirmed vision constraints

- **Identity:** reusable GPU-resident voxel-world substrate, not a game layer
- **Integration:** Rust crate or small family of Rust crates for external game consumers
- **Consumer isolation:** any validation executable and any external game share one public interface surface; adjacent consumers get no privileged substrate access
- **Standalone engine:** zero LLM/System dependency for the substrate to operate
- **Repository scope:** the actual game is out of this repo; excluded game-rule layers are not implemented here (seams only where required)

## Deferred design decisions

- Crate split, API shape, storage layout, meshing approach, and simulation algorithms
- First delivery depth and order for substrate capabilities (which matter services ship when)
- Open technical tradeoffs left by the seeds (resolution, LOD, object-layer scale, fluid fidelity, multiplayer readiness) that do not change product identity
- Whether, when, and how a walkable-world harness is built, and what content or benchmarks it uses (delivery obligation only under Q1)
- Platform backends and machine-specific performance targets

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation executable a **required current delivery** of this repository, or only a **permitted** adjacent artifact?

- **Proposed safe answer:** Permitted only—the product promise is the substrate crates and public interface; a harness may exist to exercise them but is not itself a committed deliverable until design plans it.
- **If answered differently:** Requiring the harness keeps product identity on the substrate but adds an adjacent delivery obligation (still without importing its controller, content, or performance gates into the product boundary). Treating it as merely permitted leaves harness work optional relative to substrate outcomes.

## Seed synthesis

- **README.md** — Names Moria as a GPU-resident voxel-world substrate consumed as a Rust crate, and frames the walkable-world executable as a separate consumer/harness for generation, streaming, meshing, editing, collision, persistence, and performance—not a game layer.
- **docs/seeds/project-boundary.md** — Binds product identity to the reusable Rust substrate, keeps the real game out of repo, permits a public-API-only validation harness, and excludes game-rule, System/LLM, spell, gas, combat, AI, and building layers from implementation here.
- **docs/seeds/product-one-seed.md** — Motivates first-slice walkable validation and dig/place proof pressure; its controller, seed region, content, platform, and performance detail stay harness/design depth and do not redefine product identity—hence Q1.
- **docs/seeds/voxel-world-substrate.md** — Supplies substrate purpose and outcome families (natural look over voxel truth, full mutability, deep Z, geology generation, matter services, multi-game reuse, no LLM dependency) without transferring game systems or mechanism inventory into this brief.
