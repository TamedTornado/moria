# Issue 531 — Perturb schedules and configuration fingerprints

References: `validation.md` TECH-059 schedule/configuration tests; issue M-114.

## Properties

- For all repeats on the same physical machine with identical genesis and sealed TickBatch bytes, canonical outcomes, records, participant commitments/events, and hierarchical hashes are byte-identical.
- Physical/cache/schedule variation cannot affect configuration fingerprint or canonical state.

## Perturbation configurations

- With `proptest = "=1.7.0"`, vary producer threads, insertion order before sealing, worker count, callback/completion notification order, submission chunking, staging contents, cache/resident layout, physical slot/free-list history, and compaction input layout.
- Run minimum/midpoint/maximum placement splits with distinct cell extents and simulation-unit IDs.
- Change only split, extent, unit ID, arithmetic/table digest, then another canonical configuration input; each changes fingerprint and rejects replay/restore before transition.
- Change only adapter context; identity/header/root remains unchanged.

## Error paths and claim boundary

- Any repeat divergence retains earliest tick/input/output evidence and fails replay-grade result.
- Missing perturbation dimension or insufficient repeats cannot be reported pass.
- Evidence states same-machine only; no vendor/driver/backend/cross-machine qualification inference is allowed.
