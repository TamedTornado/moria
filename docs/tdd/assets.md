# Asset Inventory and Pipeline

All runtime assets remain under the repository-root `assets/` directory so Bevy’s default `AssetServer` path works without custom packaging. Every source asset has a checked-in license/provenance entry. Generated runtime meshes are cache artifacts and are not checked in or saved as world truth.

## Required inventory

```text
assets/
  generation/product_one.ron
  materials/
    material_registry.ron
    terrain_albedo_array.*
    terrain_normal_array.*
    terrain_roughness_array.*
    water_normal.*
  vegetation/
    grass_albedo_alpha.*
    grass_normal.*
    vegetation_templates.ron
  stamps/
    ruin.ron
  ui/
    debug_font.*                 # only if Bevy default font is unsuitable
  licenses/
    ASSETS.md
```

`product_one.ron` contains the curated seed/generator parameters and feature constraints. It does not contain raw terrain voxels. `material_registry.ron` declares the stable 14-material IDs and visual/hardness/granular/phase/placeable metadata described in `data-model.md`.

Terrain texture arrays have one aligned layer per visible material (air has no sampled layer). Each layer supplies tileable albedo, tangent-space normal, and roughness with identical square dimensions/mips. The implementation target is 1024 x 1024 per layer in a GPU-supported compressed distribution format, with source-quality images retained only if repository policy permits. Color textures are sRGB; normals/roughness are linear. Final compression must work through Bevy/wgpu on Metal, Vulkan, and DirectX-class backends; no Apple-only texture format is accepted.

The water asset is a tileable normal map used by one shared static-water material. It does not encode flow. Grass textures include alpha and use shared card/cluster materials. The two tree species, bushes, boulders, stumps, and rocks are defined as deterministic voxel-template parameters in `vegetation_templates.ron`; `GenerationConfig.vegetation_templates` carries its logical path and exact-byte digest, and `generation_fingerprint` incorporates both. Their render meshes/LODs are derived and cached at boot, which keeps them registered material objects rather than disconnected authored scenery.

The validation player uses a code-generated neutral low-poly/capsule presentation with shared palette material, sized to the authoritative kinematic capsule. It needs no authored character, animation, equipment, stats, or inventory asset; movement truth remains the capsule rather than the visible mesh.

`stamps/ruin.ron` is the sole hand-authored sparse voxel shape. It contains local integer voxel coordinates/runs, cut-stone material IDs, template bounds, a quantized origin, and stair metadata used by validation. It contains no scripts, mechanisms, room semantics, construction blueprint, or bespoke collision mesh.

## Asset contracts

- Asset loading hashes the exact loaded vegetation-template and ruin-stamp bytes, compares them with their `GenerationConfig` expected hashes, then recomputes `generation_fingerprint`; it also validates material layer count/dimensions, stable IDs, stamp bounds, and template material references before `ControlReady`. No base sample or compatible-save application occurs before those checks pass.
- Assets are referenced by logical path from library configuration; absolute developer paths are forbidden.
- Repeated instances clone shared `Handle<Mesh>`/material handles. Per-instance material allocation is forbidden unless a measured visual requirement truly varies; Product One defines none.
- Missing/invalid acceptance assets are a boot failure, never silently replaced in release/benchmark mode.
- Derived mesh caches key on source asset hash, `generation_fingerprint`, LOD, and mesher revision. Cache misses rebuild; caches may be deleted without changing truth.

## Placeholder strategy

During implementation only, checked-in procedural flat-color material layers, low-poly/card vegetation, generated voxel object templates, and the real sparse ruin stamp keep every world/query/render path functional. Placeholders use the final IDs, paths, dimensions, and shared-handle contracts so replacement requires no code or schema change. Placeholder mode is marked in the configuration/benchmark report and cannot produce the final visual baseline or public milestone captures.

No audio assets are required: the design specifies no sound experience. No NPC, combat, inventory, construction, weather, fluid-effect, or narrative assets are created.

## Public milestone outputs

The first terrain image, tunnel-carving clip, geology cutaway, dressed-world shot, playable run, and benchmark results are outputs captured from the validation demo, not runtime input assets. Their capture coordinates/modes reference named `RegionManifest` waypoints so regeneration and later comparisons frame the same generated features. Media encoding/publishing is outside the runtime world API.
