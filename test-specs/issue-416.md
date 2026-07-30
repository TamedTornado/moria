# Issue 416 — Implement canonical CORDIC and axis normalization

References: `architecture.md` TECH-071 and TECH-007; issue M-015.

## Properties

- For every `TurnQ32`, quadrant reduction, all 32 simultaneous iterations, final ties-even reduction, and quadrant remap are byte-identical on CPU and WGSL.
- Zero residual always takes the `zi >= 0` branch. Every shift is floor division, including negative `x/y`.
- Axis normalization of every nonzero `[i32;3]` yields signed Q1.30 components selected by the exact squared-comparison rule; no runtime transcendental function is used.

## Exact configurations

- Retain per-iteration `(x,y,z)` goldens for all quadrant centers, all reduction midpoints, one word on either side, `0`, `0xffff_ffff`, and the eight named octant/quarter-turn words.
- Test zero, positive/negative basis, diagonal, `i32::MIN`, and maximum nonzero axes. Cover equality ties and `q == 2^30`.
- Regenerate gain/arctangent table values at the TDD’s precision and require byte equality with both CPU and WGSL tables.

## Error paths

- Zero axis returns exactly `ZeroAxis`, even for zero/full-turn identity angles.
- Injected checked-helper corruption or impossible/out-of-range intermediate returns `UnrepresentableAxis`, never a partial vector.
- Unknown table length, reordered iteration, nonsimultaneous update, alternate midpoint ownership, or final clamp must fail golden parity.

There is no rendering state.
