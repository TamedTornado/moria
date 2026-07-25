# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate for external games and tools. It is delivered as a Rust crate or small family of tightly scoped crates. The product outcome is a fully material voxel world—generated natural terrain with continuous depth, mutable matter, and public query and mutation surfaces—without implementing any game.

## Purpose

Enable multiple games and tools to share one trustworthy material-world foundation: a world that can read as a natural surface environment, remains authoritative voxel truth for interaction and mutation, and is consumed through clean public interfaces rather than forked or reimplemented per title.

## Product boundary

- **This product:** The substrate library surface—geological world generation, GPU-resident matter representation, mutation and query interfaces, and non-authoritative presentation of voxel truth to consumers.
- **Adjacent, not identity:** A walkable-world executable may exist only as a validation harness that exercises the same public interfaces an external game would use.
- **Downstream / out of repo:** The actual game and all game rules live elsewhere. Controllers, characters, cameras, authored content, demo routes, presentation policy, UX, and game-specific policy are consumer-owned.

## Future products and enabling implications

Described future consumers include a System-backed ARPG, a fortress or colony-style game, a descent-style experience, and pure sandboxes. The substrate must stand alone without LLM dependency and enable deep, fully mutable material worlds those titles need. Their gameplay, content, controllers, characters, and presentation are not current scope.

## Non-goals

- Implementing System/LLM features, spells, gas policy, combat, AI, or building and semantic game layers in this product
- Making the validation harness's character, controls, content, route, or presentation part of product identity
- Decorative non-material worlds or a heightmap-with-props product mistaken for voxel truth

## Confirmed vision constraints

- Any included harness or external game consumes the substrate only through public interfaces; no privileged or game-specific implementation paths
- The substrate has zero LLM dependency and must stand alone as an engine layer
- Game rules and future System, spell, gas, combat, AI, and building layers are out of implementation scope here (compatibility seams only where substrate requirements demand them)

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a required current delivery alongside the substrate, or only a permitted adjacent artifact?
- **Proposed safe answer:** Permitted only—current commitment is the substrate crates; a harness may be added later to validate public interfaces without becoming part of product identity.
- **If different:** Making the harness mandatory adds an adjacent delivery obligation while still keeping it outside product identity; treating it as product identity would redefine Moria as a demo game rather than a reusable substrate.

## Seed synthesis

- **README.md:** Named the product Moria; established reusable GPU-resident voxel-world substrate as Rust crate; framed the walkable-world executable as separate consumer and validation harness, not a game layer.
- **docs/seeds/project-boundary.md:** Bound product identity to the substrate crate(s); placed the real game outside the repository; allowed a harness only as public-interface validation; excluded game rules and System/LLM, spell, gas, combat, AI, and building layers from implementation here.
- **docs/seeds/product-one-seed.md:** Motivated an early walkable proof of a material (not heightmap) world and dig/place as honesty proof; contributed first-slice demo intent, harness-shaped player loop, and seed-world flavor—treated as consumer/harness material, not transferred into substrate identity; conflicted with the boundary seed on whether that harness is required delivery.
- **docs/seeds/voxel-world-substrate.md:** Established design goals for natural-looking mutable worlds, deep Z, substrate-not-game layering, and GPU-resident matter; listed future game consumers and long-horizon matter capabilities as enabling context without committing mechanism inventory or roadmap into current vision.
