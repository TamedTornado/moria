# Issue 92 — Bridge extraction, GPU prepare/free, and queue acknowledgements

References: `docs/tdd/rendering.md §Edit update path`; `docs/tdd/systems.md §Mutation systems/queue_priority_surface_jobs`.

## Boundary contracts

- Each barrier key carries request, logical key, token, revision, operation create|replace|remove through extraction -> GPU prepare/create/write/free -> draw/removal queue acknowledgement.
- Only final queue acknowledgement for the matching tuple may mark renderer-ready/removed; pre-queue completion is insufficient.

## Multi-system scenarios

- Create, replacement, empty removal and eviction through a headless render sub-app; multiple requests/revisions sharing logical key.
- Out-of-order stage completions and acknowledgements; duplicate and delayed free.

## Failure propagation

- Stale token, wrong request/revision/key, duplicate or pre-queue ack is ignored/reported and cannot complete barrier.
- GPU prepare/free failure propagates as unresolved/failure; authoritative truth remains private and unchanged.

## Ordering guarantees

- Strict stage order is extraction, GPU prepare/free, queue ack, barrier completion. Removal and creation have distinct stable acknowledgement identities.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

