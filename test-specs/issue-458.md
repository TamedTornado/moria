# Issue 458 — Order and finalize query results

References: `interfaces.md` TECH-024 and TECH-070 `QueryResult`; issue M-026.

## Properties

- Material facts order by `(volume_id, local z, local y, local x)`.
- Collision facts order by `(time_of_impact, volume_id, local z, local y, local x, face_id)`; equal TOI retains remaining key order.
- Every ready result binds one pinned frontier/root, sorted volume revisions, exact inspected/missing partition, matching `QueryKind` data, and explicit freshness.

## Configurations

- Exercise empty and populated sample/region/collision results, equal-time/equal-cell ties, multiple volumes, exact result capacity, and a later-discovered one-over-capacity result.
- Complete after one or more later ticks and assert original frontier remains reported.
- Test Latest/Retained × Wait/ReturnStale for met/unmet revisions, including absent volume current revision.

## Edge and error paths

- Pre-admission oversize returns `ResultCapacityExceeded` with exact required/supported `QueryCapacity`. Post-admission proof failure terminates `QueryUnavailable::ResultCapacityExceeded`; neither truncates.
- Reclaimed/hash-mismatched/device-lost root returns unavailable. Failed/cold/missing truth is listed or unavailable, never empty.
- Inspected and missing ranges cannot overlap, escape requested scope, or omit requested space under complete semantics.
