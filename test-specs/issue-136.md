# Issue 136 — Extend the public world facade across Product One

References: `docs/tdd/overview.md §Plugin composition`; `docs/tdd/systems.md §Schedule model`.

## Boundary contracts

- MoriaWorldPlugin adds complete non-persistence generation/streaming/mutation feature plugins behind the unchanged public facade and named schedule sets.
- WorldPluginConfig presentation flag retains its exact enabled/headless semantics; no private implementation type becomes public.

## Multi-system scenarios

- External enabled/headless consumers open complete route and exercise existing reads/edits/focus/telemetry/readiness.
- Plugin registration/introspection proves every complete-route feature present and no persistence behavior yet.

## Failure propagation

- Missing feature/digest yields typed startup failure; private module/import compile-fail fixtures remain inaccessible.
- Incidental plugin registration or worker completion order cannot change behavior because explicit set ordering is asserted.

## Ordering guarantees

- Documented facade plugin order plus systems.md edges; complete feature validation precedes one readiness event.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

