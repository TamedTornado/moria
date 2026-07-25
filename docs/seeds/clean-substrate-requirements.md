# Moria substrate requirements

## Identity and generation

A versioned parameter set and seed define world identity. Generation is
deterministic and can materialize bounded regions without eagerly allocating
the complete world.

## Storage and mutation

Sparse voxel storage preserves authoritative material truth. Consumers submit
bounded mutations through a public command API. Admission, commit, and failure
states are explicit.

## Streaming and derived views

Streaming bounds resident work and rejects stale background results. Meshing is
a derived view of voxel truth and can be regenerated. Registered objects can
participate in deterministic world queries without becoming game entities.

## Persistence

Persistence records versioned authoritative deltas and restores identical
query behavior. Derived meshes and transient scheduling state are not saved as
truth.

## Public validation and diagnostics

A headless fixture exercises generation, streaming, mutation, queries, and
persistence. A minimal visual fixture with a free-fly camera exercises meshing
through the public interface. Diagnostics expose lifecycle, revision, and
bounded-work observations without mutable internal handles.

Performance measurements include machine identity and are evidence, not
portable correctness thresholds.
