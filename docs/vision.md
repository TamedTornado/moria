# Project vision

## What we are building now

**Moria** is a reusable Rust voxel-world substrate for external consumers. It delivers substrate crates that generate, stream, query, mutate, mesh, save, and restore an authoritative voxel world through public interfaces, plus a minimal in-repo validation harness that exercises those same interfaces.

## Purpose

Moria exists so other products can depend on a stable, reusable foundation for natural-looking mutable voxel worlds without each consumer re-implementing world identity, authoritative material truth, or the public lifecycle of generation through persistence. The substrate owns engine-level world capability; games and tools remain separate products.

## Product boundary

**In product:** the reusable substrate and a minimal public-interface validation harness. The harness is an adjacent consumer of the product under test—it proves the public API and may include headless and minimal visual checks with diagnostics—not a game layer.

**Out of product:** game rules, characters, player controllers, animation, authored routes, production assets, consumer-specific content, presentation policy, and any gameplay or UX owned by downstream products. Harness-specific controls, content, routes, workloads, platforms, and performance gates are not product scope beyond the requirement that validation use public interfaces.

## Future products and enabling implications

No named future game or tool is in current scope. External consumers (other repositories and products) will build on Moria. Enabling implications supported by the seeds: deterministic, identity-bearing worlds; bounded resident work rather than eager full-world materialization; mutation and query through public commands; durable authoritative state distinct from regenerable derived views; and diagnostics suitable for integration without exposing mutable internals. Gameplay, characters, controllers, animation, and authored content remain consumer-owned.

## Non-goals

- Shipping a playable game, character, controller, or content pipeline as Moria.
- Treating the validation harness as a game layer or importing consumer presentation into the substrate.
- Portable performance pass/fail thresholds as product correctness criteria.

## Confirmed vision constraints

- Current scope is the substrate (and its public-interface validation), defined without downstream product vision.
- World identity is defined by a versioned parameter set and seed; generation is deterministic.
- Authoritative material truth is distinct from derived views (for example meshing), which can be regenerated and are not saved as truth.
- Persistence restores identical query behavior from versioned authoritative deltas.
- Performance measurements include machine identity and are evidence, not portable correctness thresholds.

## Assumptions proposed for approval

- **A1.** “Repository delivers … a minimal public-interface validation harness” means the harness is required current delivery for proving public fitness, not an optional demo. Its internal control scheme and presentation remain non-product detail.

## Questions for human review

_None._ The supplied seeds give one coherent current-product identity (reusable Rust voxel substrate for external consumers), an explicit repository boundary (substrate crates plus minimal validation harness, not a game layer), and clear ownership of game/content/UX by consumers. Capability depth, slice order, architecture, and acceptance detail belong downstream.

## Seed synthesis

- **`README.md`:** Names the product Moria; states it is a reusable voxel-world substrate; points current scope to the two clean seed docs and excludes treating broader downstream vision as current authority.
- **`docs/seeds/clean-project-boundary.md`:** Fixes current identity as a reusable Rust voxel-world substrate for external consumers; repository delivers substrate crates and a minimal public-interface validation harness; harness is not a game layer and may use free-fly/diagnostics only as validation; lists consumer-owned exclusions; requires public completeness for generate, stream, query, mutate, mesh, save, and restore.
- **`docs/seeds/clean-substrate-requirements.md`:** Contributes vision-level substrate responsibilities—deterministic identity-bearing generation, authoritative truth vs derived views, public mutation/query lifecycle, streaming of resident work, persistence of authoritative deltas, public validation (headless and minimal visual), and diagnostics—without transferring harness controls, content, or performance gates into product identity. Detailed mechanisms remain downstream design.
