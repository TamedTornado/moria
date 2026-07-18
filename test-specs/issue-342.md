# Issue 342 — Recovery PR #312: Preserve sparse brick revision coherence

References: `docs/tdd/data-model.md` §Sparse brick store and §Relationship invariants; `docs/tdd/systems.md` §Mutation systems and §`materialize_bricks`; `docs/tdd/states.md` §Chunk lifecycle.

## Properties that must hold

- For every nonempty atomic commit batch, the global revision increments exactly once and every changed `BrickRecord`, delta observation, dirty record, and emitted `EditBatchCommitted` names that same revision; no observer can see a mixture within the batch.
- For every coordinate and revision, `current_voxel == delta.unwrap_or(regenerated_base)`; evicting/recreating detail must preserve the current delta and may install materialization only for the matching brick, revision, and purpose.
- For every no-op or exact reversion, no-op does not increment revision, base-equal voxel deltas are removed, empty brick deltas disappear, and exact reversion remains coherent after eviction/reactivation.

## Entity configurations to test

- One changed cell, multiple cells in one brick, one batch spanning multiple bricks, successive batches touching the same brick, and concurrent batches touching disjoint bricks.
- Procedural/uniform/detailed bricks; edited inactive brick; resident brick remeshing while old visual remains; evict/reactivate after edit and after exact reversion.
- Complete materialization results in old/new/reversed completion order with matching and stale request tokens/revisions/purposes.

## Edge cases

- Revision zero/startup, revision increment near checked `u64` boundary, empty staged batch, and changed brick at every region face.
- Untouched bricks need not be rewritten to the newest global revision, but any derived result must prove the relevant content revision it sampled; tests must not infer freshness from global equality alone.

## Error paths

- Stale or mismatched materialization/mesh results are discarded and rescheduled without changing truth, revision, pin counts, or deltas.
- Revision overflow/invariant failure must fail atomically rather than wrap or expose partial state.

