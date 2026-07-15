# Issue 143 — Batch complete meadow and forest dressing

References: `docs/tdd/config.md §Rendering and environment configuration`; `docs/tdd/rendering.md §Dressing`.

## Properties that must hold

- For every eligible surface area, deterministic complete dressing density is 5/m² Near, 25% Middle, and zero small dressing Far/Horizon; batches share handles.
- For every streamed/edit/load revision, instances remain anchored to current eligible surfaces and counts/transforms are reproducible.

## Entity configurations to test

- Meadow and forest route samples of known area; all band boundaries/hysteresis; topsoil slopes, water/ceiling/masonry/empty/object exclusions.
- Edit moves/removes/restores anchor; load and eviction/reactivation.

## Edge cases and type boundaries

- Batch/GPU u32 overflow or missing shared asset rejects before partial install; declared fallback is observable.

## Error paths

- No floating/unsupported instance; stale old-revision batch removed before replacement and absent from save.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

