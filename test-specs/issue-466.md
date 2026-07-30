# Issue 466 — Expose bounded noncanonical telemetry

References: `interfaces.md` TECH-026 and TECH-070 telemetry shapes; issue M-028.

## Input validation

- `telemetry(world)` returns only a bounded cached `TelemetrySnapshot`; unknown, closed, and busy worlds return their distinct concrete `TelemetryError` variants.
- Counter/failure vectors fit identity/operation-record budgets and use only closed keys/metrics; reject unknown ordinals during decode.

## Transformation correctness

- For every tracked resource, `0 <= current <= capacity` and `high_water >= current`; high-water never decreases during world life.
- Snapshot includes exact latest copied frontier/durable/device generation/execution summary and after-the-fact queue/residency/failure/timing data.
- Changing adapter context changes only diagnostic execution context; it cannot change configuration fingerprint/root/replay identity.

## Edge and error paths

- Saturate each pool, create/release resources, induce failures and latencies, and verify exact current/capacity/high-water/failure counts.
- Telemetry must not wait, poll GPU, map, pin a root, drive callbacks, or advance any receipt.
- Compile/inspection tests reject buffer handles, addresses, physical slots, mutable ECS entities, storage iterators, strings, and dynamic extension maps.

Loading/empty/error rendering states are not part of telemetry; an empty counter vector is bounded data, not authority absence.
