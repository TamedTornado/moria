# Issue 64 — Implement world lifecycle transitions

References: `docs/tdd/states.md §World lifecycle`; `docs/tdd/api.md §Startup and readiness`.

## Valid transitions

- `Uninitialized + start -> Loading`. Assert the target state and all specified entry/exit effects.
- `Loading + all readiness collaborators satisfied -> Ready (WorldReady emitted exactly once)`. Assert the target state and all specified entry/exit effects.
- `Loading + Asset|ManifestIdentity|InvalidConfig|GenerationContract|Save|InitialActivation failure -> Failed(error)`. Assert the target state and all specified entry/exit effects.
- `Ready + fatal internal corruption -> Failed(error)`. Assert the target state and all specified entry/exit effects.

## Invalid transitions

- Uninitialized->Ready/Failed without Loading; Loading->Uninitialized; Ready->Loading/Ready; Failed->any state; duplicate start/ready/fail events. Each is ignored, records invariant error, and preserves state.
- Before Ready all public reads return QueryError::NotReady and edits reject; Failed never enables ordinary Ready/Playing access.

## Lifecycle ordering, guards, and concurrency

- Exercise each readiness guard false individually, then all true; this issue must not synthesize unavailable cross-system WorldReady collaborators.
- Race ready and fatal events: deterministic scheduling may select only a legal result, must not emit WorldReady after failure, and may emit WorldReady at most once.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

