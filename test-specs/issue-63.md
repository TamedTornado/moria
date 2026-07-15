# Issue 63 — Expose focus and paged diagnostic contracts

References: `docs/tdd/api.md §Activation and inspection` and `§Errors and observability`.

## Properties that must hold

- For every snapshot page, bricks/chunks/focuses use stable lexicographic order and owned immutable values; page construction respects 256 metadata bricks, 2 cell bricks/8192 cells, 512 chunks and 16 focuses.
- For every change to activation, dirty/task/focus/revision generation, an old token expires; telemetry getters are O(1), allocation-free, and the edit ring is chronological at <=256.

## Entity configurations to test

- Empty terminal page; 1/256 metadata bricks; 1/2 cell bricks with all 4096 cells each; pagination via after_brick; active and explicitly inspected bricks.
- Ring entries 0,256,257; page with 512 chunks/16 focuses and one above; unchanged versus changed between pages.

## Edge cases and type boundaries

- max_bricks 0 => InvalidInput; 257 metadata, 3 cell bricks, 513 chunks or 17 focuses => typed LimitExceeded without truncation/snapshot creation.

## Error paths

- Changed generation => SnapshotExpired; caller restart cannot combine old/new pages.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
