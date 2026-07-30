# Issue 508 — Implement the independent sparse transition oracle

References: `validation.md` TECH-059; issue M-081.

## Properties

- For all generated valid command sequences, the independent CPU oracle and public Moria path produce identical canonical order, outcomes, revisions, allocator state, simulation-domain bytes, logical cells, root commitments, and rollback frontiers.
- The oracle shares public wire definitions only, not production transition, storage, compaction, or hashing implementation.

## Entity configurations

- Generate create/retire/move/erase/place/patch over static/dynamic volumes, negative coordinates, overlapping/disjoint bricks, stale hash/revision preconditions, no-ops, ID/revision exhaustion, and command capacity edges.
- After every edit compare sparse maps; after compaction, rollback, checkpoint logical round trip, and restore compare again.
- Include overlapping commands and participant phase-four effects to verify staged canonical composition.

## Edge and error paths

- Any failed command changes no cells/revision but appears in deterministic outcomes; tick-global failure advances nothing.
- Missing/unknown content remains unavailable and is never entered into oracle as empty.
- Random sequence shrinking must retain the first divergent step and exact input prefix.
- A generated case outside TDD bounds is rejected by the generator/admission, not silently normalized into a comparison.
