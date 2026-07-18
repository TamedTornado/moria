# Issue 335 — Recovery PR #299: Fix presentation config schema layout

References: `docs/tdd/config.md` §Configuration ownership, §Streaming and task configuration, §Mutation configuration, §Player and camera configuration, §Rendering and environment configuration, and §Benchmark configuration; `docs/tdd/api.md` §Startup and readiness.

## Properties that must hold

- For every checked-in `assets/config/presentation.ron`, the production loader must deserialize the complete render/runtime tuning document with `deny_unknown_fields` and preserve the normative values and types from `config.md`; no present field may fall back silently.
- For all successful load/serialize/load cycles, streaming bands/budgets, mutation cadence/bounds, player/camera/light values, rendering/environment values, and benchmark thresholds must retain identical typed values.
- For every presentation-only edit, world `parameters_digest` must remain unchanged even though runtime presentation behavior changes.

## Entity configurations to test

- Load the checked-in document and assert representative values from every section, including four bands, `fixed_hz=60`, player capsule/camera limits, `2560x1440`, time range/default/step, Horizon cell/member fields, and all benchmark progress/query thresholds.
- Missing section, misplaced field, duplicated field, unknown field, wrong nesting, wrong scalar type, non-finite float, and invalid cross-field value.
- Default-generated document versus checked-in document must deserialize to the same normative typed configuration.

## Edge cases

- Empty maps/arrays, three or five bands, numeric narrowing overflow, and exact boundary values must reject or pass according to their typed config contracts.

## Error paths

- A schema/layout error must be reported as invalid config before world creation and must not be repaired by moving/ignoring fields at runtime.
- TDD gap: `config.md` assigns sections to `presentation.ron` but does not publish the exact outer RON container/type or literal nesting keys. Tests may assert the production loader's canonical round trip and all typed fields, but must not invent an undocumented textual nesting layout; that layout should be documented if byte-shape compatibility is intended.

