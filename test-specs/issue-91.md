# Issue 91 — Wire initial activation into WorldReady

References: `docs/tdd/api.md §Startup and readiness`; `docs/tdd/states.md §LoadingWorld`; `docs/tdd/systems.md §begin_initial_activation`.

## Boundary contracts

- WorldReady{identity,control_ready_at} emits once only after identity/assets, solid supported spawn, collision neighborhood, high-priority work, camera-frustum terrain/water and fallback-ready object handles; public access remains guarded before it.
- PresentationConfig.enabled=false omits render/water/object-visual/dressing systems and only their readiness keys, retaining generation/query/edit/streaming truth/persistence paths.

## Multi-system scenarios

- External consumer startup enabled and disabled; hold each readiness key false separately, then release out of order.
- Verify spawn sample/capsule and installed frustum at the exact event; repeat app updates cannot duplicate event.

## Failure propagation

- Each Asset, ManifestIdentity, InvalidConfig, GenerationContract, Save and InitialActivation failure transitions terminally without early event.
- Task failure/stale result cannot satisfy barrier; post-failure read/edit stays unavailable.

## Ordering guarantees

- Identity validation -> activation requests -> collision/support -> required presentation (if enabled) -> Ready/event. Concurrent completions are order-independent.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.

