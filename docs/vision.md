# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate delivered as public crates. This repository ships that substrate for external games and tools—not a game, not Product One, and not a privileged demo world.

## Purpose

Provide multiple downstream consumers a shared foundation for generated voxel worlds so each product can own its own gameplay, presentation, and content without reimplementing the world substrate.

## Product boundary

- **In product:** the reusable substrate consumed only through public crate interfaces—seed-based generation of authoritative material truth, bounded streaming and mutation, surface extraction for render-capable consumers, persistence of authoritative state, and read-only diagnostics useful across consumers.
- **Adjacent delivery, outside product identity:** a minimal validation executable and fixtures that exercise those same public interfaces (no privileged world path). Their existence as public-API consumers is a current repository commitment; their specific controls, presentation, routes, workloads, and gates are not product scope.
- **Out of product:** any particular game’s rules, UX, controllers, characters, animation, authored levels, production content, or policy.

## Future products and enabling implications

A separate later Product One may be a third-person explorer in a generated region. That vision pressures the substrate to stay multi-consumer and externally integrable; it does not place player control, character presentation, curated traversal, forest or cave content, or game assets inside Moria.

## Non-goals

- Game systems (rules, combat, inventory, AI, narrative) and production or authored game content.
- Single-game ownership of the world stack or privileged internal access for demos and harnesses.
- Absorbing Product One (or any other game) into this repository’s product identity.

## Confirmed vision constraints

- Consumers—including validation—integrate only through public crate interfaces; they must not reach storage, meshing, or scheduler internals.
- This repository delivers the substrate (and public-interface validation), not any particular game.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- **README.md:** Establishes Moria as the reusable voxel-world substrate whose only repository deliverables are current substrate commitments, and treats the interface reference as non-expanding support.
- **docs/seeds/mixed-project-brief.md:** Supplies the binding current product identity, public consumer boundary, required adjacent validation via public APIs, non-goals, and Product One as embedded future-consumer context; its detailed correctness and fixture requirements remain subordinate input to downstream design.
- **docs/seeds/substrate-interface-reference.md:** Supports the consumer-facing surface without adding deliverables; operation-level interface detail remains subordinate input to downstream design.
