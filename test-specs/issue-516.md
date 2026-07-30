# Issue 516 — Exercise configuration boundaries headlessly

References: `validation.md` TECH-060 configuration suite; `gpu-runtime.md` TECH-036; issue M-121.

## Properties

- For every TECH-017 budget field, accepted configuration reports exactly that capacity in telemetry and never allocates beyond it.
- Every cross-limit equation is checked with `u128` before callback/device allocation; first violation identifies the exact field path.

## Generated configurations

- Vary each field independently at zero, minimum, default, maximum, and maximum+1; test exact fixed fields and all eleven TECH-036 cross-limit rules plus arithmetic overflow.
- Exercise participant representation/RNG/input/effect/event/state/snapshot/artifact sums, ordering, uniqueness, zero IDs, digest changes, placement splits/extents, and per-provider maxima.
- Default smoke uses exact normative values and independently computes `required_20_bytes == 1_988_100_096` while allowing 4,096 direct inputs plus 4,096 participant effects.
- Change only changed bricks to 16,384 under 2 GiB defaults; reject pre-callback/allocation.

## Error paths

- Every invalid configuration invokes no consumer code, creates no GPU page/world, and returns exact `ConfigError`.
- Checked overflow cannot wrap into acceptance. Adapter-lowered physical byte ceilings may reject genesis but cannot silently lower logical canonical counts.
