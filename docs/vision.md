# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate, delivered as public crate interfaces for games and other adjacent consumers. This repository ships the substrate, not a game.

## Purpose

Give multiple downstream products a shared, deterministic foundation for seed-based voxel worlds they can stream, query, edit, persist, and inspect—so each consumer does not reimplement world truth and its lifecycle.

## Product boundary

**In scope:** the substrate’s public world outcomes—deterministic generation from versioned parameters and seed, sparse material truth, bounded streaming with observable readiness, admitted mutation, surface extraction for consumers, persistence of authoritative deltas, and read-only diagnostics—usable by many independent consumers.

**Out of scope as product identity:** any particular game, player experience, authored content, or game-facing systems. Game rules, combat, inventory, AI, narrative, characters, animation, controllers, curated routes, production assets, and presentation policy belong to consumers.

**Adjacent, not identity:** a minimal validation executable and headless fixtures are required current deliveries that exercise only the public interfaces. They validate the substrate; they are not the product and own no privileged world path. Their specific controls, camera, characters, content, routes, platforms, or performance gates are not product scope.

## Future products and enabling implications

A later Product One—in a separate repository—may be a third-person explorer demo in a generated region (hills, mixed forest, river, cave) with skeletal animation and a curated cliff-to-cave traversal. That is future-consumer context only. Moria enables such worlds by remaining a multi-consumer substrate; it does not ship Product One’s gameplay, characters, animation, forests, routes, or assets.

## Non-goals

- Shipping a playable game, demo prototype, or production content pack in this repository.
- Owning consumer UX, controllers, characters, animation, narrative, combat, inventory, AI, or authored levels.
- Treating derived meshes or diagnostics as authoritative world state, or granting consumers privileged access to internals.

## Confirmed vision constraints

- Consumers integrate only through public crate interfaces; validation and games share that boundary with no privileged world path.
- Same versioned parameters and seed yield deterministic generation; mutation commits atomically; persistence restores authoritative material truth—not derived views.

## Assumptions proposed for approval

None.

## Questions for human review

None. The seeds agree that the current product is the Rust voxel-world substrate; validation is a required adjacent delivery outside product identity; Product One is a later separate consumer.

## Seed synthesis

- `README.md` names Moria as the reusable voxel-world substrate, limits repository deliverables to current substrate commitments, and treats the interface reference as non-expanding support—compatible detail stays subordinate to downstream design.
- `docs/seeds/mixed-project-brief.md` supplies product identity, purpose, public consumer boundary, validation-as-adjacent-delivery, correctness outcomes at vision altitude, non-goals, and the Product One future-consumer framing—its operational and fixture detail remains subordinate input to later design, not vision inventory.
- `docs/seeds/substrate-interface-reference.md` corroborates the public consumer surface (identity, query, mutation, streaming lifecycle, persistence of deltas, registered objects, diagnostics) without adding deliverables—compatible interface detail remains subordinate to downstream design.
