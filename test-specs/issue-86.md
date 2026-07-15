# Issue 86 — Add portable shared world materials and shaders

References: `docs/tdd/assets.md §Format specifications/Shaders`; `docs/tdd/rendering.md §Material transitions` and `§Portability and shader rules`.

## Properties that must hold

- For every terrain vertex, one shared terrain material consumes four u8 IDs/normalized weights and matching triplanar arrays; water/vegetation/raw paths use their single declared shared materials.
- For every GPU-visible counter/index/allocation, representation is 32-bit and checked before upload; WGSL contains no 64-bit atomic/vendor-only path.

## Entity configurations to test

- All material IDs, 1..4 weights, each shader path, placeholder and production arrays, Metal/Vulkan parse/smoke and DirectX-class validation.
- Allocation/count exactly u32::MAX-equivalent guard and one beyond; shader missing/invalid.

## Edge cases and type boundaries

- Oversized or mismatched texture/vertex/binding data rejects before GPU mutation.

## Error paths

- Shader failure is observable shared magenta fallback in development and fatal in benchmark/release evidence.

## Conformance-harness hook

Where applicable, run the same public-path case against issue 232's dense oracle; vary scheduling, cache residency, and batch partitioning without changing the expected authoritative result.
