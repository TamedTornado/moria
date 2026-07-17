# Virtual-world conformance harness

The `moria_world::testing::conformance` module is the test-only dense oracle
for virtual-world integration tests. It expands a deliberately small caller
chosen region into a complete `x, z, y`-ordered array, applies registered base
features by the documented precedence order, and holds only explicit values
that differ from that composed base as edit overlays. It does not use a sparse
store, spatial index, dirty-discovery implementation, task scheduler, or save
codec.

Run the bounded CI profile with:

```sh
PROPTEST_CASES=64 cargo test -p moria-world --test conformance_harness
```

Run the extended deterministic profile locally with:

```sh
PROPTEST_CASES=4096 cargo test -p moria-world --test conformance_harness
```

`proptest` records and replays a failing generated history through its standard
regression file mechanism. Check in the minimized generated fixture when a
production adapter uncovers a defect.

## Extending a subsystem

Each later production subsystem adds its case to
`crates/moria-world/tests/conformance_harness.rs`. Drive the actual public
`MoriaWorldPlugin` facade and normal schedules; only clock advancement and
external completion/error boundaries may be controlled by a test. At every
committed or otherwise observable boundary, call `DenseWorld::compare_voxels`
with the public scalar query. Its callback returns the real public query error,
so an unavailable or failed production observation cannot be mistaken for
oracle agreement.

Add subsystem-specific adapters beside that test for public column, ray,
capsule, feature/dependency, dirty/derived-key, lifecycle, persistence-snapshot,
and terminal-acknowledgement observations. Compare each observable value after
every generated operation; do not replace the production store, indexes,
mutation/invalidation pipeline, scheduler, or codec with a fake. Metamorphic
cases must rerun the same history with changed completion order, legal batch
partitioning, cache residency/eviction, and save/load boundaries. Fault cases
must retain the seed, operation history, completion schedule, and injected
fault point in a deterministic regression fixture.
