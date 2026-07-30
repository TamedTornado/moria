# Issue 566 — Aggregate scenario and durability evidence

References: `validation.md` TECH-069 and TECH-064–066; issue M-131.

## Properties

- Closed catalog is exactly: `public_boundary`, `deep_volume`, `dynamic_volume`, `post_admission_atomicity`, `truth_view_dressing`, `failure_canonical_query_content`, `failure_participant_lifecycle`, `failure_durable_capability_presentation`, `checkpoint_cold_restore`, `participant_restart`, `correction_branch`, `cold_public_replay`, `rollback_chain`.
- Rows bind respectively to M-092–M-104 and retain typed identity, immutable digest, fixture/public-facade identity, expected/actual roots/lifecycle, durability/replay/correction state, and validity.
- Domain pass requires every independent scenario/failure/durability/rollback proof; presentation is never truth and live memory is never cold restart.

## Configurations

- Validate one complete correct set in permuted input order.
- For each row: omit, duplicate, corrupt, relabel, cross-wire, mark unavailable/incomplete/fail, change fixture/lineage/active-history/facade identity.
- Attempt substitutions across the three failure slices, checkpoint/participant restart, correction/replay, and rollback timing/correctness.

## Error paths

- Missing/skipped/unavailable/failed row invalidates domain. Rollback correctness must pass before its timing row can be interpreted.
- Wrong lineage/root/active-history, surviving-live-state cold evidence, presentation-only truth, or incomplete failure matrix fails exact owning row.
- Unknown rows/versions/fields/digest references and trailing data fail closed.
