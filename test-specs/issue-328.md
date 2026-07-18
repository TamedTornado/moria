# Issue 328 — Recovery PR #244: Tighten water shader placeholder contract

References: `docs/tdd/assets.md` §Shaders, §Placeholder strategy, §Production asset registries, and §Import and validation pipeline; `docs/tdd/rendering.md` §Water and §Portability and shader rules.

## Properties that must hold

- For every accepted `assets/shaders/water.wgsl`, parsing/validation must find every entry point declared by its `AssetBudgetContract::Wgsl`, reject undeclared unsupported bindings, and prove `forbids_i64_atomics: true`.
- For all water-shader inputs, presentation time may move normals only; it must not write authoritative voxel truth, change `WaterBodyDef.surface_y`, or introduce flow/pressure state.
- For every platform path, the source must be portable WGSL with no backend-specific include, vendor subgroup assumption, 64-bit atomic, or runtime platform fork.

## Entity configurations to test

- The checked-in shader; missing/renamed entry point; syntax error; unsupported binding; `atomic<i64>`/`atomic<u64>` token; backend-specific include/fork; and registry path/digest mismatch.
- A development shader-load failure and a benchmark/release shader-load failure.

## Edge cases

- Extra harmless helper functions may exist, but the declared required entry-point set must be present exactly once each.
- Comments or identifier substrings containing `i64` must not create false positives; actual unsupported atomic types must be rejected by parsing/validation.

## Error paths

- Development may select only the shared magenta fallback and must emit an observable asset warning.
- Benchmark/release acceptance must treat shader load/validation failure or fallback use as fatal and must not reach a passing report.

