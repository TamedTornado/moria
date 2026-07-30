# Issue 476 — Execute atomic mutation and COW kernels

References: `architecture.md` TECH-012/013; `gpu-runtime.md` TECH-035; issue M-063.

## Properties

- For every command, mark/capacity/materialize/apply/hash/validate phases are ordered submissions on one queue; no candidate reference becomes live before full validation and publication.
- All old bricks are materialized completely in unreferenced slots; all commands for a brick apply in canonical order; ancestors are private copy-on-write nodes.
- Atomic winner, workgroup/lane identity, physical slot, or arrival order never affects canonical output.

## Entity configurations

- Run empty/no-op, single-cell, overlapping commands, multi-brick, 64-brick/32,768-cell maximum, multiple volumes, and old-root readers.
- Perturb workgroup pressure, slot/free-list history, command producer order, and staging contents; compare exact outcomes/root/hash.

## Edge and error paths

- Inject allocation, scan/output/diagnostic overflow, missing base, invalid command, failure after reservation, after brick construction, and before publication.
- Every failure retires private slots after last use and preserves old cells/revisions/root/participants/log/observations with zero partial publication.
- Over-dispatch is guarded; barriers are uniform; no spin/last-workgroup/subgroup/64-bit-CAS assumption is accepted.
- GPU error scope, mapping, decode, and diagnostic validation failures are distinct terminal no-advance causes.
