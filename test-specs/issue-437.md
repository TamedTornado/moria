# Issue 437 — Implement point, sphere, and capsule overlap

References: `collision-presentation.md` TECH-051 steps 2–3 and 8–9; issue M-053.

## Properties

- For all points, overlap uses closed-low/open-high cell bounds. For positive-radius/extent shapes, exact geometric high-plane touch is overlap.
- Sphere/box overlap is exactly `dot(c-clamp(c,box), same) <= r²`; interior witness ties follow axis then negative-before-positive face.
- Capsule/box evaluates all sorted rational breakpoints and each fixed-clamp quadratic at endpoints and feasible vertex; winner is least `u`, then axis/sign order.

## Entity configurations

- Test outside, interior, face, edge, corner, tangent, and strictly separated cases for each primitive.
- Test zero-radius sphere, zero-length capsule, zero-radius segment, parallel segment axes, each clamp-state interval, and equal-distance candidate ties.
- Repeat on translated/rotated volumes and assert exact world contact point/normal bytes.

## Edge and error paths

- Checked cross multiplication controls breakpoint/candidate order; no float/epsilon or premature rounding is allowed.
- Overflow, invalid transform, impossible rational, or unrepresentable witness/normal returns typed arithmetic failure and no fact.
- A no-hit result is legal only when every required authoritative brick was inspected; missing content is unavailable.
