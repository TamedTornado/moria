# Issue 413 — Implement canonical fixed-point scalar arithmetic

References: `architecture.md` TECH-071; issue M-014.

## Properties

- For all valid splits `F in 0..=16`, add/subtract/negate/absolute are exact checked integer operations.
- For all representable operands, multiplication and division equal the exact rational result rounded nearest, ties to an even retained raw integer. CPU and WGSL bytes must match.
- Square root returns the nearest integer root of `raw << F`, ties-even, and never uses floating point or saturation.
- Floor division/shift are explicitly distinct from ties-even reduction and round negative values toward negative infinity.

## Configurations and exact cases

- Exercise every split with zero, ±1, extrema, products around `i64` bounds, positive/negative half ties, exact/nonexact squares, and narrowing at target-width edges.
- Compare portable two-word WGSL helpers with signed `i64`/`u64` oracle operations for carry, borrow, sign, compare, shift, divide, and overflow.

## Error paths

- Division by zero, negative square root, split 17, invalid shifts, intermediate overflow, and unrepresentable output return the exact `CanonicalFailure` tag and no partial result.
- Generated/canonical code must contain no float types/literals, libm/transcendentals, implicit casts, reassociation, or undocumented saturation.

Rendering states are outside this arithmetic component.
