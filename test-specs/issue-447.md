# Issue 447 — Implement bounded capsule sweep enumeration

References: `collision-presentation.md` TECH-051 step 6–9; issue M-056.

## Properties

- For every capsule sweep, enumerate exactly the finite 27 coordinate clamp-state combinations and their feasible interior/boundary candidates.
- Nonsingular 2×2 normal equations use checked integer arithmetic; singular systems reduce to stated one-dimensional boundary cases.
- Winner order is earliest exact `t`, then least exact `u`, then clamp-state axis/sign order.

## Entity configurations

- Exercise every clamp class with feasible and infeasible interiors, all `t/u` boundaries, singular and nearly singular coefficients, zero-length capsule, zero radius, zero delta, tangency, initial overlap, and time-one contact.
- Construct equal-time/equal-u cases to verify state ordering; construct earlier `t` with later `u` to prove time precedence.
- Cover maximum coordinate/radius/delta values still representable.

## Edge and error paths

- Reject candidates outside their clamp region. Checked determinant, normal equation, polynomial, rational comparison, transform, witness, TOI, or normal overflow is typed arithmetic failure.
- No iterative conservative advancement, convergence tolerance, float/epsilon, or skipped clamp class is conforming.
- Missing truth yields unavailable, never `NoHit`.
