# Issue 509 — Implement the independent collision oracle

References: `collision-presentation.md` TECH-051; `validation.md` TECH-059; issue M-083.

## Properties

- For all valid primitive/volume/cell configurations, the independent exact-integer oracle and production collision path agree on hit membership, winner ordering, TOI, local cell/material, world contact point/normal, and failure class.
- Oracle code does not call production transform, overlap, SAT, sweep, witness, normalization, or TOI functions.

## Configurations

- Cover every TECH-051 fixture: closed-low/open-high, face/edge/corner touch, inside witness, zero radius/extent/capsule/delta, every SAT axis/tie, capsule breakpoint/clamp state, singular 2×2 system, rational/irrational TOI, support tie, translated/rotated dynamic placement, and extrema.
- Compare exact `CollisionFact` bytes for Genesis and Confirmed(0).

## Error paths

- Generate transform, wide arithmetic, discriminant, fraction, witness, normal, and wire-range failures; query becomes unavailable and participant tick no-advance exactly as specified.
- Missing/cold/corrupt content is not passed as empty/no-hit.
- On mismatch, retain exact input plus independently computed intermediate candidates sufficient to identify the earliest differing contract step.
