# Issue 572 — Aggregate performance and presentation evidence

References: `validation.md` TECH-069 and TECH-067/068; issue M-133.

## Properties

- Closed catalog is exactly: `device_loss_recovery`, `presentation_truth_isolation`, `presentation_visual_capture`, `presentation_human_review`, `rollback_correctness`, `rollback_tier_measurement`, `benchmark_sparse_lifecycle`, `benchmark_materialization_mutation_hashing`, `benchmark_query_collision`, `benchmark_presentation_lifecycle`, `benchmark_checkpoint_restore`, `benchmark_participant_recovery`, `benchmark_replay_correction`.
- Visual capture binds only M-096 capture manifest; human review binds only M-132 attestation; both are validated by M-112 schema/digest path and attestation names exact capture digest.
- Correctness, capture, human review, named tier, and measurements remain separate typed statuses.

## Configurations

- Valid report records immutable row digest plus hardware/software/budget/fixture context, correctness prerequisite, and measurement/review status.
- For each row/artifact: omit, duplicate, reorder, corrupt, cross-wire, mark unavailable/incomplete/fail, or alter identity/context.
- Test correct depth-20 fixture at threshold, slower correct result, incorrect fast result, timestamps unavailable, software adapter, absent/mismatched capture/attestation, and non-PASS human claim.

## Error paths

- `TIER_MET` is legal only for exact correct TECH-067 fixture; slower correct stays measured. Correctness failure cannot become performance success.
- Missing TECH-068 path/context/status, device loss neither reconstructed nor terminal, revival of P1–P10, or universal hardware claim invalidates report.
- Visual/timestamp/software evidence cannot substitute for canonical/physical evidence.
