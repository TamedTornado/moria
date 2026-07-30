# Issue 440 — Implement immutable sparse world roots

References: `architecture.md` TECH-010; issue M-019.

## Valid transitions

- Genesis creates an immutable world root with `FrontierPosition::Genesis`; confirmed ticks create new roots with `Confirmed(t)` through copy-on-write.
- Scar key absence means exact base-authority lookup. A complete uniform/full leaf may replace or be removed when byte-identical to verified base.
- Static volume placement is immutable; dynamic lifecycle changes produce a new volume-state root.

## Invalid transitions

- Reject writes to an installed root, absent/retired IDs, invalid 104-bit scar keys, wrong base identity, static placement change, and Genesis/Confirmed substitution.
- No failure may install a partially copied path, mutate an old reader’s view, or encode physical slot identity.

## Lifecycle, ordering, and concurrency

- Build generated sparse maps with many volumes/negative brick coordinates; compare exact logical cells, stable volume order, allocator state, simulation domain, participant commitments, and frontier position.
- Hold old roots while editing disjoint and shared prefixes; old queries remain byte-stable, unchanged paths are shared, and changed paths have exactly 26 radix levels.
- Race readers with candidate construction/publication; each reader sees one complete immutable root, never mixed revisions.
- Confirm no dense/sparse full CPU cell mirror is retained and no presentation/cache readiness enters the root.
