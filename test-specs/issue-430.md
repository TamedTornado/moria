# Issue 430 — Implement bounded base-content callback cells

References: `content-persistence.md` TECH-041; issue M-037.

## Boundary contract

- Before `BaseContentSource::request`, reserve one request record, callback cell, and all 2,048 possible payload bytes. The immutable request names exact world/volume/brick/root/expected digest and encoding.
- `BaseBrickCompletion` is non-clone, sequential, Moria-owned, and admits exactly one terminal `finish_brick`, `finish_uniform`, or `fail`.

## Multi-system scenarios

- Write a valid brick in one write and many writes; both yield identical verified bytes. A valid uniform completion writes no brick bytes.
- Cancel before invocation (provider is not called) and during a synchronized copy (active copy finishes, closed cell cannot publish).
- Drop, panic, explicit provider failure, and successful retry through a fresh request ID exercise complete permit release.

## Failure propagation and concurrency

- Short finish, byte 2,049, wrong uniform mode, invalid cell, digest mismatch, duplicate/forged completion, late generation, and producer drop fail the owning materialization and never mark content ready.
- Race terminal completion with cancellation: exactly one disposition is accepted; the other is `Cancelled`/`AlreadyCompleted` and cannot revive the request.
- Diagnostics are limited to code, retryability, and 96 UTF-8 bytes; no `Vec`, writer, panic payload, map, chain, or growable allocation crosses the callback.
- No automatic retry occurs; missing/failed content remains unavailable, never empty.
