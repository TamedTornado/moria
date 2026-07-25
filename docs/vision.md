# Project vision

## What we are building now

Moria is a reusable Rust voxel-world substrate: a library for external consumers that generates, streams, queries, mutates, meshes, saves, and restores an authoritative voxel world through public interfaces.

## Purpose

Enable independent game and tooling consumers to build on a stable, deterministic voxel-world foundation without embedding game rules, characters, or consumer-specific content in the substrate.

## Product boundary

Moria owns the reusable substrate and its public world lifecycle. The repository also delivers a minimal public-interface validation harness as an adjacent consumer of that API; the harness is not a game layer and does not define product identity. Free-fly camera and diagnostic overlays may appear only as harness means. Game rules, characters, controllers, animation, authored routes, production assets, and consumer-specific content stay outside this product.

## Future products and enabling implications

These seeds deliberately omit named downstream products. Intended consumers are external games, tools, and other repositories. Enabling implication only: those consumers can own gameplay, presentation, controllers, and content while relying on Moria for authoritative voxel-world outcomes.

## Non-goals

- Shipping a game, playable character layer, or production content pack as this product
- Embedding game rules, controllers, animation, or authored routes in the substrate
- Treating harness presentation, workloads, or performance numbers as portable product acceptance thresholds

## Confirmed vision constraints

- Adjacent consumers, including the validation harness, use only the public interface available to external repositories.
- World generation is deterministic from versioned identity parameters and seed; performance measurements are machine-bound evidence, not portable correctness gates.

## Assumptions proposed for approval

None.

## Questions for human review

None.

## Seed synthesis

- `README.md`: Names the product Moria and directs current scope to the clean boundary and requirements seeds without a separate downstream product vision.
- `docs/seeds/clean-project-boundary.md`: Settles identity as a reusable Rust voxel-world substrate, repository delivery of substrate plus a minimal public-interface validation harness, exclusion of game-layer concerns, and must-complete world lifecycle outcomes through public interfaces.
- `docs/seeds/clean-substrate-requirements.md`: Contributes vision-level outcomes for identity, generation, authoritative mutation, streaming, derived meshing, persistence of truth rather than derived views, public validation, and diagnostics; its detailed operational and performance requirements remain subordinate input to downstream design.
