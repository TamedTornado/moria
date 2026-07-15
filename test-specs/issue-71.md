# Issue 71 — Materialize and evict sparse bricks

References: `docs/tdd/systems.md §Focus and streaming systems/materialize_bricks`; `docs/tdd/states.md §Chunk lifecycle`.

## Properties that must hold

- For every materialization result, installation requires exact (brick,revision,purpose) token and occurs at most once; starts and install bytes stay within budgets.
- For every brick, a 4096 array is allocated only for boundary, delta, raw inspection or collision; eviction preserves deltas/index/shared assets.

## Entity configurations to test

- Uniform air/water/geology, surface/cave/material boundaries, edited inactive brick, raw inspection, collision, multiple purposes/revisions.
- Worker completion permutations; focus removed while job runs; edit during job; pinned/unpinned eviction.

## Edge cases and type boundaries

- Stale/duplicate result is discarded; allocation/byte/count overflow rejects before install and can be retried.

## Error paths

- Task failure leaves compact authoritative truth intact; eviction cannot run while pinned and cannot remove BrickDelta.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

