# Issue 332 — Recovery PR #291: Validate terrain normal Basis payload

References: `docs/tdd/assets.md` §Textures, §Production asset registries, and §Import and validation pipeline; `docs/tdd/rendering.md` §Material transitions.

## Properties that must hold

- For every accepted `assets/materials/terrain_normal.ktx2`, image data must be a valid, transcodable Basis Universal payload with linear sampling.
- For all accepted normal arrays, width, height, layer order, and complete mip chain must exactly match terrain albedo and ORM arrays; the air/water registry slots must contain neutral semantic layers.
- For every installed normal asset, registry path, content SHA-256, file budget, color space, mip/layer counts, and Basis flag must match the decoded file.

## Entity configurations to test

- Checked-in normal array; header-only fake; corrupt Basis slices; missing final mip; wrong layer count/order/dimensions; sRGB declaration; digest mismatch; and non-neutral reserved layers.
- Decode/transcode samples from each layer class and from first/final mips.

## Edge cases

- Exact declared dimensions/mips/byte size pass; any one-unit mismatch fails without truncation or implicit resampling.
- A valid KTX2 container with a non-Basis payload must fail because `basis_payload` is required.

## Error paths

- Failure must prevent terrain material readiness and surface as typed asset validation; benchmark/release must not silently substitute a flat normal texture.

