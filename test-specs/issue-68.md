# Issue 68 — Add edit submission limits and idempotency

References: `docs/tdd/api.md §World edit protocol`; `docs/tdd/config.md §Mutation configuration`.

## Properties that must hold

- For every drained batch, requests are ordered by request_id; each lifetime-unique accepted ID emits exactly one EditAccepted with stamped frame and reserved bounded capacity.
- For all legal shapes, sphere radius is 64..4096 Q8, boxes are nonempty/clipped/in bounds; Atomic requires <=32 conservative bricks and Progressive <=8192.

## Entity configurations to test

- Min/max radii; atomic count 0/32/33; progressive 8192; clipped edge spheres/boxes; strengths 0/1/255; all placeable and forbidden materials.
- Queue depth 0/32/33, duplicate IDs in same drain and after completion, before Ready and during load.

## Edge cases and type boundaries

- Duplicate, NotReady, LoadInProgress, QueueFull, invalid bounds/material/strength, atomic/progressive overflow reject synchronously with no lifecycle messages or capacity leak.

## Error paths

- API/import inspection forbids designation/worker/spell/mana and direct voxel/mesh operations.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

