# Issue 535 — Audit canonical kernels for nondeterministic contamination

References: `gpu-runtime.md` TECH-035; `validation.md` TECH-061; issue M-085.

## Properties

- Build a complete transitive inventory for every canonical WGSL entry/helper from matching Naga 29 validated IR: atomics, workgroup storage/barriers, slot assignment, compaction/sort, padding/unused writes, allocator/free-list inputs, hash inputs, and host/callback dataflow.
- Every canonical byte/identity/revision/order/outcome/hash input has a stable logical source independent of races, physical layout, arrival, subgroup/lane, or unordered iteration.

## Configurations

- Positive fixtures include noncanonical work-slot atomics/failure flags with stable output selection and uniform-barrier mark/scan/scatter.
- Negative fixtures inject atomic-winner identity, race-produced/uninitialized padding, physical slot/free-list order, arrival/callback order, subgroup/lane identity, order-dependent compaction, forbidden float/transcendental, and helper outside `moria-fixed-v1`.

## Error paths

- Missing entry/helper, unclassified atomic, incomplete output-byte proof, unavailable IR, or failed Naga parse/validation invalidates audit.
- Custom analysis must not parse WGSL or become a general validator.
- Passing sampled replays does not waive a contamination finding; audit pass does not replace real-GPU parity.
