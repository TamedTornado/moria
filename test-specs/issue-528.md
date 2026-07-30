# Issue 528 — Implement the deep-volume truth scenario

References: `validation.md` TECH-064 scenario 2; issue M-093.

## Universal properties

- For every queried or edited cell in the admitted deep domain, returned truth must derive from exact sparse material state at the named frontier, never from a height-field or missing-as-empty shortcut.

## Entity configurations

- Consumer fixture supplies one maximum-depth legal static volume with negative/positive coordinates, voids, signed-density boundaries, multiple material bands, authored structures, and features varying independently along x/y/z.
- Include uniform sparse regions plus a small set of detailed/scarred bricks to demonstrate sparse behavior without a heightmap.

## Properties

- Deepest legal point/region/collision queries bind exact volume/revision/root/material facts and match independent oracle results.
- Edits at min/max domain boundaries obey half-open rules, exact brick/cell caps, atomic revision change, and honest exposed cuts.
- Untouched theoretical cells consume no dense canonical mirror/resident allocation; telemetry shows bounded sparse residency consistent with configured interest.

## Edge and error paths

- One-beyond domain, pivot/radius, coordinate, query, and edit bounds reject without clamping or partial mutation.
- Cold/unmaterialized deep regions remain unavailable, not empty. Materialize then re-query exact truth.
- No Moria generator, height-field assumption, player/camera, physics, or privileged fixture path is used.
- Presentation may visualize the fixture but cannot establish matter/collision correctness.
