# Moria substrate program brief

## Current product

Moria is a reusable Rust voxel-world substrate. It is consumed through public
crate interfaces by games and by a minimal validation executable. This
repository delivers the substrate, not any particular game.

The current product owns deterministic seed-based generation, sparse voxel
storage, bounded streaming, mutation, surface extraction, persistence, and
read-only diagnostics. These capabilities must remain useful to multiple
downstream consumers.

## Current public boundary

External consumers can create and identify a world, request bounded regions,
observe readiness, query material truth, submit bounded edits, and persist
deltas. A consumer must not reach into storage, meshing, or scheduler internals.

The validation executable uses exactly these public interfaces. It may provide
a free-fly camera and diagnostics sufficient to exercise the crate, but it is
not a game prototype and owns no privileged world path.

## Current correctness commitments

Generation is deterministic for the same versioned parameters and seed.
Mutation is admitted through a bounded command API and committed atomically.
Persistence restores the same authoritative material state. Derived meshes and
diagnostics never become authoritative world state.

Streaming must bound resident work and expose observable lifecycle states.
Background results carry generation identities so stale work cannot replace
newer truth. Failures remain typed and observable to public consumers.

## Current validation commitments

Headless fixtures cover generation, query, mutation, persistence, and lifecycle
behavior. A small visual fixture demonstrates that a relocated external
consumer can render and edit through the public API. Performance is reported
with machine identity; this brief does not establish a machine-specific
correctness threshold.

## Current non-goals

Moria does not implement game rules, combat, inventory, AI, narrative systems,
characters, animation, authored levels, or production content. References to
possible consumers explain interface pressure only.

## Later consumer vision

After the reusable substrate ships, a separate Product One repository may
place a third-person explorer in a generated region with hills, a dense mixed
forest, a river, and a cave. That later game-facing demo may use skeletal
animation and a curated cliff-to-cave traversal to communicate the world.

Those later paragraphs are future-consumer context embedded in this binding
program brief. They do not authorize a player controller, character mesh,
animation clips, forest population workload, curated route, or game asset in
Moria.
