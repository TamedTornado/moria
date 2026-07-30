# Issue 441 — Fold replay records into one active semantic history

References: `content-persistence.md` TECH-047 active-history fold; issue M-046.

## Valid transitions

- `Header -> TickRecords*` appends consecutive semantic ticks.
- At the current nonzero physical sequence, a valid `CorrectionBranch(target,present)` removes semantic records after target and appends its exact contiguous embedded replacement through the same present.
- Physical prefix digest advances for every physical record; active-history digest reflects only the current semantic projection.

## Invalid transitions

- Reject missing/duplicate headers, tick at sequence zero, header later, nonconsecutive tick, branch with unavailable target, wrong previous digest/root, gaps/overlaps/nesting, invalid range/count, or mismatched corrected digest.
- Rejection leaves the prior active projection and both published digests unchanged.

## Lifecycle and ordering

- Independently recompute both BLAKE3 digest domains after each record; verify deterministic results under equivalent input storage/order.
- Fold ordinary stream, one correction, and adversarial multiple branch sequences. Only corrected tick frames remain active; superseded physical bytes remain immutable diagnostic evidence.
- Retain `(physical sequence, subrecord offset)` locators for every active frame, including branch-embedded frames.
- Concurrent readers pinned before a fold keep their old projection until release; publication installs the new projection atomically.
