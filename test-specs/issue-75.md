# Issue 75 — Commit edit batches atomically into base-relative deltas

References: `docs/tdd/api.md §World edit protocol`; `docs/tdd/systems.md §Mutation systems/commit_edits`.

## Properties that must hold

- For every staged batch, all writes become visible atomically; a nonempty batch increments world revision exactly once and assigns it to sorted/deduplicated changed bricks.
- For every current cell equal to regenerated base, its delta is absent; empty BrickDelta is removed and collision/query sees committed truth immediately.

## Entity configurations to test

- Single/multi-brick, overlapping progressive batches, multiple requests, changes that cross density 128, complete and partial base reversion.
- Valid no-effect request; reader probes before/at/after fixed commit boundary.

## Edge cases and type boundaries

- Malformed/stale staged batch fails validation with zero mutation/message/revision change; write failure cannot expose a prefix.

## Error paths

- No-op emits Accepted, one zero-change BatchCommitted, normal terminal Reconciliation, preserves revision and creates no presentation keys.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

