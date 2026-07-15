# Issue 77 — Discover edit dependencies and reject stale object work

References: `docs/tdd/systems.md §Generation systems/surface_dependency_ids_for_changes` and `§Mutation systems`.

## Properties that must hold

- For every committed changed mask, exact affected IDs equal the explicit dependency oracle, sorted/deduplicated, with <=256 broad and <=64 exact IDs and no retained dependency set/whole-operation allocation.
- For every async object/Horizon payload, installation requires current token/revision; later batches can never resurrect an earlier revision.

## Entity configurations to test

- Object-owned edit, adjacent terrain and ruin halo edit, outside dependency, multi-batch overlap, fully removed object, exact reversion, region-edge stencil.
- Maximum F1 stress target and batches affecting multiple objects/Horizon cells.

## Edge cases and type boundaries

- Candidate/ID/config cap overflow rejects manifest/admission rather than truncating discovery; arithmetic failure yields no partial dirty set.

## Error paths

- Stale results are discarded/rescheduled; reverting object cells while an adjacent dependency delta remains must not restore authored/card form.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
