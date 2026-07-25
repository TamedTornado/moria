# Project vision

## What we are building now

Moria is a reusable, GPU-resident voxel-world substrate delivered as a Rust crate (or a small family of tightly scoped Rust crates). It is the world-and-matter foundation external consumers integrate through public interfaces—not a game, not authored content, and not a player-facing demo product.

## Purpose

Downstream games need one engine-level world layer where a natural surface reads as ordinary terrain, every visible surface is backed by continuous mutable voxel truth, deep underground is first-class space, and generation, matter representation, presentation derived from that truth, and mutation/query surfaces live below game rules. Moria exists so multiple products can share that foundation with no LLM dependency and no game policy baked into the substrate.

## Product boundary

**This product owns** the substrate: a reusable voxel world that supports natural-looking generated terrain over fully material, diggable volume; deep-Z as ordinary world space; and public surfaces for queries and mutation that keep higher layers from touching voxels directly.

**Adjacent, not identity:** a walkable-world executable may live in-repo only as a validation harness. It must use the same public interfaces available to an external game and must not own privileged or game-specific implementation paths. Harness controls, characters, cameras, routes, content, presentation, and acceptance workloads are not product scope.

**Downstream / out of this repo:** the actual game(s). Game rules and System, LLM, spell, gas, combat, AI, and building layers are not implemented here; compatibility seams may be designed only where the substrate itself requires them.

## Future products and enabling implications

Described future consumers—not current product—include a System-driven ARPG, a fortress/colony-style game, a descent-style experience, and pure sandbox modes. The substrate’s high-level enabling implication is one shared mutable material world those games can price, rule, and present differently. Their gameplay, UX, controllers, characters, authored content, and policy remain consumer-owned.

## Non-goals

- Shipping game rules or System, LLM, spell, gas, combat, AI, or building-layer implementations in this product
- Absorbing harness or demo ownership (player control, cameras, seed showcase content, trailer/milestone presentation) into substrate identity
- Privileged in-repo consumer paths external games cannot use

## Confirmed vision constraints

- Integration is a Rust crate boundary: the validation harness and any external game consume the same public interfaces, with no privileged access for in-repo consumers.
- Higher game and semantic layers stay out of this product; only substrate-required compatibility seams may be designed here, not those layers’ implementations.

## Assumptions proposed for approval

None.

## Questions for human review

**Q1.** Is a walkable-world validation harness a required current delivery beside the substrate, or only a permitted adjacent artifact?
- **Proposed answer:** Required as an adjacent delivery outside product identity—present to exercise public interfaces and make the mutable material-world claim credible—without importing harness controls, content, or acceptance detail into the substrate.
- **If different:** If only permitted, the substrate crates alone can be a complete current delivery; any walkable proof moves fully downstream and product-one’s demo-shaped “done” bar is not a Moria shipping obligation.

## Seed synthesis

- **README.md** — Fixed the name and core identity as a GPU-resident Rust-crate substrate and cast the walkable-world executable as a separate consumer/validation harness; further harness and terrain wording stays subordinate design input.
- **docs/seeds/project-boundary.md** — Bound current product to the reusable substrate, excluded the real game and higher layers from this repository, and required shared public interfaces for any harness; precise packaging remains subordinate technical design.
- **docs/seeds/product-one-seed.md** — Motivated an early walkable proof and a first credibility horizon for generation, material presentation, and dig/place as evidence of voxel truth; region content, controller, camera, milestones, and performance gates remain harness/consumer design input, not product identity.
- **docs/seeds/voxel-world-substrate.md** — Contributed design goals (natural look, full mutability, deep Z, substrate-not-game, GPU-resident foundation) and long-horizon world/matter responsibility for future consumers; mechanism inventories, layering diagrams, and build-order detail remain subordinate downstream design.
