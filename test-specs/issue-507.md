# Issue 507 — Generate arbitrary-precision scalar math oracle data

References: `validation.md` TECH-059 scalar obligations; `architecture.md` TECH-071; issue M-079.

## Properties

- The oracle uses independent arbitrary-precision rational/integer calculations and imports no production fixed-math implementation.
- For all generated accepted cases across splits 0..=16, oracle rounding/output bytes equal both CPU and WGSL production results.

## Configurations

- Generate zero, ±1, extrema, exact half ties with even/odd retained candidates, product/numerator near wide limits, exact/nonexact squares, narrowing/shift boundaries, and random full-domain samples for each split.
- Retain deterministic fixture identity/seed and checked-in expected bytes so regeneration is reviewable.
- Use exactly `rug = "=1.27.0"` as required by the issue.

## Error paths

- Generate division by zero, negative root, invalid split/shift, wide overflow, and unrepresentable output; require the exact stable failure tag on both implementations.
- Changing oracle precision/rounding/source fixture must change its artifact identity and cannot silently bless production output.
- A production implementation mismatch, missing split, skipped case, malformed artifact, or oracle import of production math fails the evidence row.

No rendering behavior is in scope.
