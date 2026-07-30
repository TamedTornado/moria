# Issue 552 — Aggregate canonical and GPU evidence

References: `validation.md` TECH-069, TECH-059/061/062; issue M-129.

## Properties

- Closed row catalog is exactly: `fixed_math_oracle`, `orientation_oracle`, `sparse_transition_oracle`, `commitment_replay_oracle`, `collision_oracle`, `wgsl_validation`, `kernel_contamination`, `gpu_sparse_directory`, `gpu_canonical_publication`, `gpu_collision`, `gpu_participant`, `same_machine_replay`, `decoder_fuzz`, `schedule_configuration_perturbation`, `participant_rng`, `canonical_rust_boundary`, `canonical_wgsl_boundary`.
- Each row binds only the correspondingly ordered M-079–M-089, M-091, M-113–M-115, M-125, or M-126 artifact and records typed identity/digest, contract/source/fixture, execution context, expected/actual layer/result.
- Valid domain requires all rows complete/pass; Naga, submission, render, mock, software adapter, or one oracle never substitutes for another layer.

## Configurations

- Construct one valid immutable identity-matching set in permuted input order.
- For every row, test omission, duplicate, mislabel, cross-wire, corruption, wrong identity/version/context, downgrade from physical to software/mock, skipped/unavailable/incomplete/fail.
- Inject unexpected GPU error, incomplete contamination inventory, missing math split/vector, and one of eight replay runs divergent/unavailable.

## Error paths

- Any named defect invalidates the report before domain pass; report retains exact failed row/layer without rewriting producer evidence.
- Same-machine replay context must not become cross-machine qualification.
- Unknown rows/fields, digest mismatch, or trailing serialization fail closed.
