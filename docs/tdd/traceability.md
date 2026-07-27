# Product-to-Technical Traceability

This matrix maps the approved design to normative technical decisions and
evidence. The design document remains the authority; this file prevents
implementation work from becoming detached from it.

| Approved design capability | Technical contract | Evidence |
| --- | --- | --- |
| Matter is authority; views disposable (§2.1) | `matter-and-storage.md` §§1–3; `presentation-and-extensions.md` §1 | C02, C11 |
| One contract for every consumer (§2.2) | `overview.md` §§2–4; `public-api.md` §1 | C01 and compile-fail tests |
| Volume-general static/dynamic bodies (§2.3, §3) | `public-api.md` §§2, 7; `matter-and-storage.md` §6 | C04, C07 |
| Sparse cost follows interest/change (§2.4) | `matter-and-storage.md` §§2–4; `resources-and-portability.md` §§2–4 | C05 and sparse threshold |
| Explicit asynchronous boundary (§2.5) | `public-api.md` §4; `runtime.md` §§1, 4 | C01, C09 |
| External behavior, no behavior vocabulary (§2.6) | `public-api.md` §3; `presentation-and-extensions.md` §§5–8 | C10 |
| GPU-first, correctness then optimization (§2.7) | `overview.md` §2; `matter-and-storage.md` §§3–5; `resources-and-portability.md` §§5–7 | GPU suite, C03, performance suite |
| Stable world/material/volume/revision concepts (§3) | `public-api.md` §§2–3 | unit/property tests, C01 |
| Consumer-owned base content and exact lineage (§3, §4.1) | `matter-and-storage.md` §4; `persistence.md` §2 | C01, C06 |
| Bounded interest/readiness/withdrawal (§4.2) | `public-api.md` §5; `runtime.md` §3 | C05 |
| Sample/region/occupancy/trace/overlap/sweep (§4.3) | `public-api.md` §6; `matter-and-storage.md` §§7–8 | GPU property suite, C02, C07 |
| No unknown-as-empty or silent clipping (§4.3, §8) | `public-api.md` §§4, 6; `matter-and-storage.md` §7 | C05, C09 |
| Remove/place/patch/stamp/create/retire/move (§4.4) | `public-api.md` §7; `matter-and-storage.md` §§5–6 | C03, C07 |
| Atomic bounded matter command (§4.4, §12; D1) | `matter-and-storage.md` §5; `runtime.md` §4 | C03 injected post-staging failure |
| Revision preconditions/conflicts (§4.4, §5.2) | `public-api.md` §§2, 7; `runtime.md` §6 | C03, C09 |
| Bounded observations and explicit gaps (§4.5) | `public-api.md` §8; `runtime.md` §7 | C08 |
| Organic/constructed presentation and honest stale state (§4.6) | `presentation-and-extensions.md` §§1–2 | C02, C03, C11 |
| Matter-backed assemblies vs derived dressing (§4.6) | `presentation-and-extensions.md` §§3–4 | C11 |
| Scar checkpoint and exact restore (§4.7) | `persistence.md` §§1–6 | C06 and golden fixtures |
| Explicit shutdown/dirty behavior (§4.8) | `runtime.md` §§9–10 | C09 |
| Region lifecycle (§5.1) | `runtime.md` §3 | C05 |
| Independent volume ordering (§5.2) | `public-api.md` §§2, 7; `runtime.md` §6 | C07 |
| Bounded pressure behavior (§5.3) | `resources-and-portability.md` §§2–4; `runtime.md` §8 | C05, C09 |
| Collision owns truth, not response (§6) | `matter-and-storage.md` §§7–8 | C02, C07, C10 |
| Consumer content; genuine 3D; no generator (§7) | `matter-and-storage.md` §4; conformance fixture ownership | C04 |
| Structured, retryable, commit-aware failures (§8) | `public-api.md` §4; `runtime.md` §5 | C09 |
| Reviewable telemetry without storage access (§9) | `public-api.md` §9; `resources-and-portability.md` §§8–9 | every evidence report, C05/C10 |
| Public-boundary proof (§10) | `validation.md` C01 | C01 |
| Truth-versus-view proof (§10) | `validation.md` C02 | C02 |
| Mutation honesty/deep-volume proof (§10) | `validation.md` C03–C04 | C03, C04 |
| Sparse/lifecycle and persistence proof (§10) | `validation.md` C05–C06 | C05, C06 |
| Dynamic-volume/behavior proof (§10) | `validation.md` C07, C10 | C07, C10 |
| Failure and quality evidence (§10) | `validation.md` C08–C11; `resources-and-portability.md` §7 | contract/performance reports |

## Completion-criteria cross-check

The approved design's eleven completion criteria are discharged as follows:

1. Non-heightmap static/dynamic content: base source, volume domain, C04/C07.
2. Cheap homogeneous regions and bounded interest: sparse descriptors, C05.
3. Bounded revisioned inspection: query contract, C01/C02.
4. Atomic asynchronous mutation: transaction protocol, C03.
5. Dynamic movement/edit without policy: placement commit, C07.
6. Matter collision independent of view: collision kernels, C02.
7. Observation gap recovery: subscription snapshot/resume, C08.
8. Organic/constructed presentation and anchored dressing: dual pipelines,
   C11.
9. Scar checkpoint against exact reconstructable base: Merkle identity,
   checkpoint cut, C06.
10. CPU/GPU external behavior without storage privilege: ordinary facade and
    snapshot/effect exchange, C10.
11. Fail-closed measurement: evidence schema, C09 and release gate.

## Explicit non-capabilities

No TDD module owns a player, camera, controls, generator recipe, heightmap,
physics solver, gravity, velocity integration, contact response, damage,
fracture, health, resistance, fluids, fire, growth, AI, navigation, game
progression, economy, multiplayer service, or web target. Fixture implementations
of content or behavior live only in ordinary consumer packages and cannot be
imported by `moria`.
