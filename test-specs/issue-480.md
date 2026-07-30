# Issue 480 — Derive crisp-cell presentation meshes

References: `collision-presentation.md` TECH-055; issue M-073.

## Input validation

- Consume only source-root/revision-pinned material bricks plus one-cell halo and registered presentation keys.
- Reject output counts/bytes above presentation job limits; failure is a derived chunk failure and cannot affect matter.

## Transformation correctness

- Greedy quads merge only coplanar exposed faces with identical material/presentation/source-face identity.
- Exercise all ±x/±y/±z faces, holes, concavities, adjacent equal/different materials, crisp/smooth boundaries, and stable material-ID duplicate resolution.
- A changed brick dirties itself and all 26 halo neighbors. Dynamic-volume movement changes only the Bevy transform; local mesh bytes are unchanged for unchanged matter/revision.

## Rendering states

- Empty/uniform-empty input produces an empty installed mesh or absent chunk per presentation lifecycle, not an error.
- Populated input produces bounded indexed local-space geometry.
- Overflow/shader/upload error yields `PresentationStatus::Failed`; stale source remains/vanishes/marks according to policy.
- While building, old/current truth remains authoritative; current is emitted only after matching revision installation.

Visual pixel identity is not specified. Tests assert mesh topology/counts/source facts and canonical isolation, not exact rendered pixels.
