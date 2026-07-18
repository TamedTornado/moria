# Issue 333 — Recovery PR #295: Fix terrain albedo Basis placeholder

References: `docs/tdd/assets.md` §Textures, §Placeholder strategy, §Production asset registries, and §Import and validation pipeline; `docs/tdd/rendering.md` §Material transitions.

## Properties that must hold

- For every accepted `assets/materials/terrain_albedo.ktx2`, the payload must be valid cross-platform Basis Universal image data, not a signature/header placeholder, and must successfully transcode.
- For all accepted albedo arrays, sampling is sRGB; dimensions, canonical 14-layer material ordering, and complete mip count match normal/ORM arrays; air and water slots are neutral/blank semantic layers.
- For every accepted file, decoded metadata and exact bytes agree with both production registries and the immutable validation report digests.

## Entity configurations to test

- Checked-in albedo; header-only/truncated payload; corrupted Basis blocks; wrong color space; 13/15 layers; mismatched mip count/dimensions; reordered layers; digest/path mismatch.
- Decode/transcode first and final mip for ordinary solid layers and reserved air/water layers.

## Edge cases

- Exact declared file budget passes and one byte above fails before decode.
- A parseable container with zero usable image levels or an untranscodable Basis payload must fail.

## Error paths

- Invalid albedo prevents terrain material readiness; development may only use the declared keyed checker fallback with a warning, while benchmark/release acceptance fails on fallback.

