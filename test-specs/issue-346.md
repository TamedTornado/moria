# Issue 346 — Recovery PR #317: Require ready lifecycle for reads

References: `docs/tdd/states.md` §World lifecycle; `docs/tdd/api.md` §Startup and readiness and §Read-only world observations; `docs/tdd/systems.md` §`validate_world_identity`.

## Properties that must hold

- For every fallible `WorldRead` operation before `WorldLifecycle::Ready`, the result is `QueryError::NotReady` and no procedural/cache/store traversal is performed.
- For all failed world openings, no `WorldStore` capable of accepting queries exists and no fallible read can expose partial config, manifest, generated voxel, active-band, or diagnostic truth.
- For every valid lifecycle, fallible reads become available only after the single `WorldReady` barrier includes identity validation, collision neighborhood, initial presentation conditions (when enabled), object fallback readiness, and support sample.

## Entity configurations to test

- Lifecycle `Uninitialized`, `Loading` before assets, loading after identity but before activation, ready, and startup `Failed` for each `WorldOpenError` class.
- Invoke `sample_voxel`, `sample_point`, `sample_column`, `ray_cast`, `sweep_capsule`, `overlap_capsule`, `water_surface_at`, and `diagnostic_snapshot` in every pre-ready phase; each must return `NotReady` even with otherwise invalid/out-of-bounds inputs, establishing lifecycle guard precedence.
- Presentation enabled and disabled readiness paths; headless mode may omit presentation conditions but not truth/query readiness.

## Edge cases

- A read scheduled in the same update as the transition may observe either consistently pre-transition `NotReady` or post-transition ready state according to explicit system ordering, never partially initialized truth.
- Duplicate/invalid ready transition requests do not open reads twice or bypass invariant errors.

## Error paths

- Pre-ready reads must have no side effects such as memoized partial values, activation, task spawning, telemetry fabrication, or mutation.
- TDD gap: `identity`, `bounds`, `route`, and `active_band` have infallible/non-`Result` signatures while the lifecycle text broadly says queries return `NotReady`. This spec applies the typed `NotReady` assertion to the fallible methods listed above; readiness semantics for those four observations require a TDD/API amendment before a stronger assertion.

