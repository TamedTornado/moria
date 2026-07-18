# Issue 337 — Recovery PR #306: Enforce presentation asset contracts

References: `docs/tdd/assets.md` §Format specifications, §Shared render asset resources, §Placeholder strategy, §Production asset registries, and §Import and validation pipeline; `docs/tdd/api.md` §Startup and readiness.

## Properties that must hold

- For every declared content asset, startup/test validation must resolve exactly one canonical license entry and budget entry with equal stable ID, immutable path, and SHA-256; the 28 declared content assets are covered exactly once in sorted order.
- For all accepted GLB/KTX2/WGSL/RON assets, the registry contract variant matches the extension and every format-specific size, bounds, primitive/clip, mip/layer/color-space/Basis, entry-point, schema, and portability condition is validated before readiness.
- For all repeated objects/dressing/chunks, globally unique `WorldRenderAssets` supplies shared mesh/material handles; per-instance spawning never allocates a material and dynamic allocations are removed/accounted on eviction.

## Entity configurations to test

- Complete production set; each asset missing/corrupt/digest-mismatched; duplicate, unknown, missing, out-of-order, path-mismatched, or cross-registry-mismatched entry; invalid schema version; empty/TBD provenance; invalid external URL/SPDX/license path.
- GLB at/over triangle or byte budget, mismatched LOD origin/bounds, missing player clip; KTX2 mismatch/transcode failure; WGSL missing entry point/64-bit atomic; invalid ruin stamp/config RON.
- Two and thousands of repeated placements to assert stable shared handle IDs and no per-instance asset growth.

## Edge cases

- Exact positive file/triangle/member limits pass; one unit over fails before costly decode/upload.
- Absent driver/GPU pipeline evidence does not invalidate pure schema validation, but pure validation cannot claim headed pipeline compilation.

## Error paths

- Fatal required-asset validation leaves lifecycle non-ready and spawns no authored roots or partial material resources.
- Declared development fallback must emit sorted warning/fallback evidence; shader/material fallback or missing acceptance art makes benchmark/release reports fail.

