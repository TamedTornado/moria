# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate, delivered as a Rust crate or a small family of tightly scoped Rust crates. It is an engine-layer library that supplies a fully material, mutable voxel world for downstream games—not a game, character experience, or authored content pack.

## Purpose

Future games need a shared foundation for natural-looking surface worlds, continuous deep underground space, and dig-anywhere mutability where voxel matter is the truth. Moria exists so that matter, world foundation, queries, and mutation live in one reusable substrate while game rules, economy, and fiction stay above it.

## Product boundary

**This product owns** the substrate consumed through public Rust interfaces: the reusable voxel world, its material mutability, and engine-level support those worlds require so external games can build on them without forking privileged paths.

**Adjacent, not identity:** A walkable-world executable may live in-repo only as a validation harness. It must use the same public interfaces an external game would use. Its controller, camera, demo route, seed composition, presentation, and performance gates do not define the product.

**Not this repository:** The actual game is a separate downstream consumer. Game rules and System/LLM, spell, gas, combat, AI, and building layers are out of scope here; compatibility seams may be designed where the substrate needs them, but those layers are not implemented in this product.

## Future products and enabling implications

Described future consumers include an ARPG with a separate System/LLM layer, fortress or colony-style games, Moria-style descent experiences, and pure sandboxes. The substrate enables them by remaining a stand-alone world and matter layer they can share. Their gameplay, content, controllers, characters, and presentation stay consumer-owned.

## Non-goals

- Shipping a playable game, combat, agents, or game policy in this product.
- Implementing System/LLM, spells, gas/pricing, or building-game layers here.
- Equating harness-specific demo content or controls with substrate scope.

## Confirmed vision constraints

- Any in-repo validation harness is an adjacent consumer of the same public interfaces available to external games, with no privileged access path.
- The substrate stands alone with zero LLM or System dependency.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1:** Is an in-repo walkable-world validation executable a **mandatory current delivery**, or only **permitted** beside the substrate crates?

*Proposed safe answer:* Permitted as an adjacent harness that exercises public APIs; not required to define substrate “done.”

*If different:* Mandatory delivery adds a required adjacent artifact outside product identity without importing its controls or content into the substrate; optional keeps current commitment to the library alone.

## Seed synthesis

- **README.md:** Established Moria as a reusable, GPU-resident voxel-world Rust substrate with a separate walkable-world validation consumer; further harness wording remains subordinate.
- **docs/seeds/project-boundary.md:** Bound the repository to the substrate, excluded the actual game and named game layers, and required public-interface-only harness consumption; workspace or crate-split mechanics remain subordinate design.
- **docs/seeds/product-one-seed.md:** Motivated a first product-shaped validation experience and material-mutability proof; region, controller, content, and benchmark detail remain subordinate harness or design input and do not expand product identity.
- **docs/seeds/voxel-world-substrate.md:** Set substrate purpose—natural material worlds, deep-Z mutability, multi-game reuse—and named future consumer families; storage, meshing, fluids, integrity, and milestone mechanisms remain subordinate design input.
