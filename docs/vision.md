# Project vision

## What we are building now

**Moria** is a reusable Rust voxel-world substrate: library crates that let external consumers generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world through public interfaces. This repository also delivers a minimal public-interface validation harness so the substrate can be exercised the same way another repository would use it. The harness is required delivery and an adjacent consumer under test; it is not the product identity and is not a game layer.

## Purpose

Moria exists so independent products can share one deterministic, authoritative voxel-world foundation without each reimplementing world generation, sparse truth, streaming, meshing as a derived view, mutation admission, and versioned persistence. The substrate owns reusable world matter and engine-facing capability at that altitude; games and other apps remain free to own rules, presentation, and content.

## Product boundary

**In scope:** the substrate’s public outcome—authoritative voxel worlds usable only through public interfaces—and a minimal harness that validates those interfaces without privileged access.

**Out of scope (current product):** game rules, characters, player controllers, animation, authored routes, production assets, consumer-specific content, and any game or UX layer. The harness may present enough to exercise the API; its camera, overlays, workloads, and acceptance scenarios are harness-owned, not substrate identity or product feature scope.

**Ownership line:** substrate responsibility stops at enabling reusable world truth and public operations on it. Gameplay, UX, controllers, authored content, presentation, and game-specific policy stay with downstream consumers.

## Future products and enabling implications

No named future game or app is in the supplied seeds. External repositories are the intended consumers. Enabling implication only: a complete public substrate for authoritative voxel worlds lets those consumers build games or tools without absorbing their rules, characters, presentation, or content into Moria.

## Non-goals

- Shipping a playable game, character, controller, animation system, or production content pack in this product.
- Treating the validation harness as a product surface that defines gameplay, presentation, or consumer policy.
- Expanding “reusable substrate” into a committed multi-game engine roadmap or feature catalog beyond the current world-substrate boundary.

## Confirmed vision constraints

- Current delivery is substrate crates for external Rust consumers plus a minimal public-interface validation harness; both harness and other consumers use only public interfaces.
- Game rules, characters, controllers, animation, authored routes, production assets, and consumer-specific content are outside the current product.

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds agree on product identity (Rust voxel-world substrate), purpose (reusable authoritative worlds for external consumers), boundary (substrate plus adjacent harness; no game layer), and that detailed generation, storage, streaming, meshing, persistence, and diagnostic behavior belong to downstream design rather than this brief.

## Seed synthesis

- `README.md` named the product Moria and pointed current scope at the two clean boundary/requirements seeds, excluding separate downstream product vision from authority for this brief.
- `docs/seeds/clean-project-boundary.md` fixed identity as a reusable Rust voxel-world substrate, required substrate crates and a minimal public-interface validation harness, forbade treating the harness as a game layer, and stated the public world outcomes the product must enable; its compatible harness and exclusion details remain subordinate design input.
- `docs/seeds/clean-substrate-requirements.md` reinforced deterministic identity, sparse authoritative truth, streaming and derived views, versioned persistence, public fixtures/diagnostics, and non-portable performance evidence as subordinate requirements for later design—not as vision inventory or a second product definition.
