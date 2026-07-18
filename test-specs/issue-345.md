# Issue 345 — Recovery PR #318: Enforce diagnostic snapshot bounds

References: `docs/tdd/api.md` §Read-only world observations/diagnostic observation; `docs/tdd/systems.md` §Camera and diagnostics; `docs/tdd/api.md` §Errors and observability.

## Properties that must hold

- For every accepted diagnostic page, bricks are lexicographically ordered owned values; metadata pages contain at most 256 bricks, cell pages at most 2 bricks/8,192 cells, and intersecting/contributing render chunks/focuses at most 512/16 without truncation.
- For all cell-bearing bricks, exactly all 4,096 cells are returned with material, density, and the three distinct authoritative predicates; empty and partial cells remain inspectable.
- For every snapshot token, activation, dirty/task state, focus, or revision changes invalidate later pages with `SnapshotExpired`; unchanged state yields a stable nonoverlapping page sequence.

## Entity configurations to test

- `max_bricks` 1/256 metadata pages and 1/2 cell pages; multi-page `after_brick` traversal; active bricks, explicitly inspected bricks, and inactive/uninspected bricks.
- Empty active index and terminal empty page with remaining zero-brick focus markers exactly once.
- Pages intersecting exactly 512 chunks/16 focuses and fixtures producing 513/17; external consumer and demo adapter must observe identical sequences.

## Edge cases

- `max_bricks=0` returns `InvalidInput`; 257 metadata bricks returns `LimitExceeded(DiagnosticBricks)`; `include_cells && max_bricks>2` returns `LimitExceeded(DiagnosticCells)` before snapshot creation.
- 513 chunks or 17 focuses returns the matching typed limit and requires retry with smaller `max_bricks`; it never slices results to fit.

## Error paths

- Invalid/over-limit/expired requests return no partial `DiagnosticPage` and do not leak a token, entity, store, or mutable handle.
- Complexity counters must remain within `O(log A+B+C+F)` metadata or `O(log A+B*4096+C+F)` cells and must not scan inactive wilderness.

