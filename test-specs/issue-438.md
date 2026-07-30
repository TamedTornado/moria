# Issue 438 — Implement box SAT overlap and trace

References: `collision-presentation.md` TECH-051 steps 4–5 and 7–9; issue M-054.

## Properties

- For every accepted box/trace input, the selected axis, interval, witness, normal, and hit result are determined solely by the exact ordered integer rules below.
- SAT axes are cell x/y/z, shape x/y/z, then nine cross axes in lexicographic pair order. Zero axes are skipped; strictly disjoint separates, touching overlaps.
- Minimum penetration compares exact rational depth, ties by axis-list order; zero center projection selects the negative direction.
- Slab/continuous-SAT fractions retain exact signed-normalized rationals through interval intersection.

## Entity configurations

- Exercise separation/contact on every one of 15 axes, parallel axes, zero cross axes, equal-depth ties, rotated boxes, zero extents, initial overlap, high-plane touch, and contact exactly at time one.
- For trace, cover axis delta zero inside/outside slab, positive/negative delta, entry/exit swaps, and coincident entry times.
- Verify exact world witness and directed-normal bytes after dynamic-volume transform.

## Edge and error paths

- Any projection, vertex, fraction, cross multiplication, transform, or normal conversion overflow returns typed arithmetic failure.
- Empty interval through time one yields `NoHit`; missing/cold/corrupt bricks never do.
- Axis reordering, epsilon comparison, float transform, or rounded-before-compare depth is nonconforming.
