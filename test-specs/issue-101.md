# Issue 101 — Expose the public feasibility world facade

References: `docs/tdd/overview.md §Plugin composition`; `docs/tdd/api.md §Boundary`; `docs/tdd/systems.md §Schedule model`.

## Boundary contracts

- MoriaWorldPlugin installs minimal F2 generation, storage, query, mutation, streaming, terrain, object, presentation, telemetry and activation slices; public exports are limited to documented config/lifecycle/read/edit/focus/telemetry types.
- Presentation-disabled registration removes presentation systems only and preserves truth behavior/readiness.

## Multi-system scenarios

- Separate enabled and headless external crates compile, receive one WorldReady and exercise every F2 public contract.
- Inspect plugin/system sets and schedule edges under zero/one/multiple fixed ticks.

## Failure propagation

- Compile-fail private store/render/import/reset/mesh paths; no persistence, broad G1, demo or privileged benchmark type is reachable.
- Missing plugin dependency yields typed startup failure, never incidental-order success.

## Ordering guarantees

- Plugin declaration order plus explicit schedule sets must match TDD; randomized task completion must not bypass validation/readiness.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

