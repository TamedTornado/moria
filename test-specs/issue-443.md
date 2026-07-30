# Issue 443 — Implement translating sphere and box sweeps

References: `collision-presentation.md` TECH-051 steps 5, 7–9; issue M-055.

## Properties

- For every sweep, exact rational candidates remain unreduced to wire time until winner selection; returned TOI is the greatest Q0.32 word not later than the crossing.
- Initial overlap returns time zero. Zero delta executes static overlap. No interval through time one returns `NoHit`.
- Irrational quadratic roots are floored by the fixed 32-step high-to-low test against the original polynomial.

## Entity configurations

- Exercise sphere/box and translating AABB/oriented box with rational crossings, irrational discriminants, one-word-below/at/above TOI boundaries, clamp-state changes, tangency, initial overlap, no hit, and exact time one.
- Create equal-time candidates in multiple cells/volumes and verify remaining TECH-024 order keys resolve ties.
- Repeat under translated/rotated dynamic volume placement and assert final world contact bytes.

## Error paths

- Negative discriminant follows no-crossing semantics; checked discriminant, coefficient, rational comparison, support, transform, or output overflow is typed failure.
- No epsilon, iterative advancement, float, early root rounding, or reordered polynomial evaluation is conforming.
- Missing/cold/corrupt matter yields unavailable, not no-hit.
