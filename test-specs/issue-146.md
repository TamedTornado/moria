# Issue 146 — Implement benchmark lifecycle and watchdog

References: `docs/tdd/states.md §Benchmark runner states`; `docs/tdd/benchmarks.md §Run lifecycle`.

## Valid transitions

- `Boot + valid args -> VerifyingFeasibilityInput (feasibility only) OR Loading (final scenarios)`. Assert target state and entry/exit effects.
- `VerifyingFeasibilityInput + valid F1 -> Loading; failure -> Failed`. Assert target state and entry/exit effects.
- `Loading + WorldReady/view ready -> Warmup`. Assert target state and entry/exit effects.
- `Warmup + exactly 300 rendered frames -> QueryCostProbe (feasibility) OR Running`. Assert target state and entry/exit effects.
- `QueryCostProbe + passing evidence -> Running; failure -> Failed`. Assert target state and entry/exit effects.
- `Running + flythrough complete -> Reporting`. Assert target state and entry/exit effects.
- `Running + mutation-workloads complete -> Saving -> RoundTrip -> Reporting`. Assert target state and entry/exit effects.
- `Running + feasibility workloads complete -> Reporting`. Assert target state and entry/exit effects.
- `Reporting + valid atomic report -> Complete; validation/write failure -> Failed`. Assert target state and entry/exit effects.
- `Any active runtime state + watchdog timeout -> Failed`. Assert target state and entry/exit effects.

## Invalid transitions

- Warmup before readiness; Running before exact warmup/probe; flythrough -> Saving/RoundTrip; feasibility -> Saving/RoundTrip; mutation-workloads -> Reporting before RoundTrip; Complete/Failed -> ordinary state.
- Reject without fabricating events/metrics; terminal stays terminal.

## Lifecycle ordering, guards, and concurrency

- Advance only on explicit readiness/waypoint/edit/save/load/report events, never sleeps. Warmup counts rendered acknowledged frames exactly.
- At 300 s timeout record last state/waypoint/request/missing metrics, passed=false and exit 1; races resolve terminal exactly once.

## Conformance-harness hook

Where applicable, adapt this case to issue 232's independent dense oracle and rerun with varied completion order, cache residency, batch partitioning, eviction/reactivation, and save/load boundaries.

