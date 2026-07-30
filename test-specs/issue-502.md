# Issue 502 — Wire the complete consumer facade

References: `overview.md` TECH-001/002; `interfaces.md` TECH-070; issue M-077.

## Boundary contract

- From an external crate, call exactly every TECH-070 method on `MoriaClient`, `WorldBuilder`, and `ObservationSubscription`, plus every receipt/provider/participant method referenced by those routes.
- Accepted calls copy/move requests into the owning bounded operation and return one receipt/handle; rejected calls return the exact owned request and builder where specified.
- Synchronous telemetry reads only cached bounded state; every other progress-producing route is nonblocking and receipt-driven.

## Multi-system scenarios

- Exercise construction, tick, interest, query, observation/resnapshot, checkpoint, correction, recovery, restore, public replay, telemetry, and shutdown through the facade and verify routing to the matching budget/lifecycle/error family.
- Drop caller request/receipt handles after admission and prove operation ownership/lifetime follows the TDD.
- Compile and use CPU/GPU participant and content/store/replay provider traits solely through public paths.

## Failure propagation and closure

- For each admission code, assert unchanged ownership, exact context, and no callback/GPU/storage side effect before acceptance.
- Compile-fail private modules, internal buffers/slots/entities, raw unticked mutation, unbounded result owners, test-only mutation/fallback, undefined aliases, and game/physics/damage/network vocabulary.
- Public exports contain no general wgpu type except the deliberately coupled GPU participant module.
