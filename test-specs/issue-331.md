# Issue 331 — Recovery PR #290: Validate terrain ORM KTX2 transcoding

References: `docs/tdd/assets.md` §Textures, §Production asset registries, and §Import and validation pipeline; `docs/tdd/rendering.md` §Material transitions and §Portability and shader rules.

## Properties that must hold

- For every accepted `assets/materials/terrain_orm.ktx2`, the KTX2 contains a cross-platform Basis Universal payload that can be transcoded through the declared runtime path; metadata alone is insufficient.
- For all accepted ORM arrays, dimensions, layer ordering, and complete mip count must equal terrain albedo and normal arrays; sampling is linear and channel semantics are R=ambient occlusion, G=roughness, B=metallic.
- For every registry validation, path/digest/size/color-space/Basis fields must agree across installed bytes, license entry, budget entry, and immutable asset report.

## Entity configurations to test

- Checked-in texture; valid header with corrupt Basis data; unsupported supercompression/transcode target; truncated final mip; wrong dimensions/layers/mips; sRGB declaration; digest mismatch.
- Transcode representative first, middle, and final mip/layer images, including the neutral air/water slots.

## Edge cases

- Exact byte limit and complete smallest mip pass; one byte above the limit or one missing payload byte fails.
- A decoder that can inspect metadata but cannot transcode image data must not mark the asset ready.

## Error paths

- Any decode/transcode/layout mismatch must fail asset validation before terrain material readiness; benchmark/release cannot pass using a texture fallback.

