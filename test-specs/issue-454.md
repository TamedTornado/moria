# Issue 454 — Admit bounded queries with explicit readiness

References: `interfaces.md` TECH-023; issue M-025.

## Valid transitions

- `Queued -> WaitingForReadiness -> Encoded -> Submitted -> Mapping -> Decoding -> Ready`, or terminal unavailable/cancelled per TECH-021.
- `Complete + Wait` remains pending with an exact blocker until truth/revision/resource guard passes. `ExplicitPartial` may complete with exact inspected/missing ranges.
- `LatestCommitted + ReturnStale` pins the current root and labels all unmet minimum revisions; a retained frontier is pinned at admission.

## Invalid transitions and guards

- Reject invalid scope/kind/shape, duplicate or out-of-scope minimum revisions, request limits above world/device limits, and any exceeded five-dimensional `QueryCapacity`; return the request unchanged with exact required/supported records.
- Retained `Wait` with an unmet floor terminates `FrontierTooOld`; it cannot become newer.

## Lifecycle and concurrency

- Exercise Sample/Region/Occupancy/Trace/Overlap/Sweep, world/volume scopes, empty and maximum output limits, cold/materializing/failed ranges, pressure, and revision floors.
- Query completion after later ticks reports the originally pinned frontier/revisions.
- Cancel before encoding versus after submission; later cancel suppresses delivery but holds root/readback permits through completion.
- Unknown/cold/corrupt/reclaimed/device-lost truth is pending/unavailable, never encoded as empty or `NoHit`.
