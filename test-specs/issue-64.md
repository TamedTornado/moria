# Issue 64 — Implement world lifecycle transitions

References: `docs/tdd/states.md §World lifecycle`; `docs/tdd/api.md §Startup and readiness`.

## Valid transitions

- `Uninitialized + start -> Loading`. Assert the target state and all specified entry/exit effects.
- `Loading + one legal ready transition request -> Ready`. Assert the lifecycle state changes once and enables the documented public access guards. This issue does not construct presentation collaborators or assert `WorldReady`; issue 91 owns the cross-system readiness barrier and event.
- `Loading + Asset|ManifestIdentity|InvalidConfig|GenerationContract|Save|InitialActivation failure -> Failed(error)`. Assert the target state and all specified entry/exit effects.
- `Ready + fatal internal corruption -> Failed(error)`. Assert the target state and all specified entry/exit effects.

## Invalid transitions

- Uninitialized->Ready/Failed without Loading; Loading->Uninitialized; Ready->Loading/Ready; Failed->any state; duplicate start/ready/fail events. Each is ignored, records invariant error, and preserves state.
- Before Ready all public reads return QueryError::NotReady and edits reject; Failed never enables ordinary Ready/Playing access.

## Lifecycle ordering, guards, and concurrency

- Exercise the lifecycle's ready transition request as absent, valid, duplicate, and received after failure. Cross-system spawn/collision/presentation guards are out of scope here and are exercised by issue 91.
- Race the legal ready transition request and fatal error: deterministic scheduling may select only a legal result, must never return from `Failed`, and must accept readiness at most once. Assert no `WorldReady` directly in this unit slice.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
