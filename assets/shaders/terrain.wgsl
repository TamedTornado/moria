// Portable terrain-material placeholder. Terrain material IDs are direct array
// layers, while every terrain chunk shares these texture arrays and sampler.

struct TerrainVertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) material_ids: vec4<u32>,
    @location(3) material_weights: vec4<f32>,
};

struct TerrainVertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) @interpolate(flat) material_ids: vec4<u32>,
    @location(3) material_weights: vec4<f32>,
};

@group(0) @binding(0)
var terrain_albedo_layers: texture_2d_array<f32>;

@group(0) @binding(1)
var terrain_normal_layers: texture_2d_array<f32>;

@group(0) @binding(2)
var terrain_orm_layers: texture_2d_array<f32>;

@group(0) @binding(3)
var terrain_sampler: sampler;

fn triplanar_layer_sample(
    texture_layers: texture_2d_array<f32>,
    world_position: vec3<f32>,
    surface_normal: vec3<f32>,
    material_id: u32,
) -> vec4<f32> {
    let projection_weights = abs(surface_normal) /
        max(dot(abs(surface_normal), vec3<f32>(1.0)), 0.0001);
    let layer = i32(material_id);
    let x_projection = textureSample(texture_layers, terrain_sampler, world_position.yz, layer);
    let y_projection = textureSample(texture_layers, terrain_sampler, world_position.xz, layer);
    let z_projection = textureSample(texture_layers, terrain_sampler, world_position.xy, layer);
    return x_projection * projection_weights.x + y_projection * projection_weights.y +
        z_projection * projection_weights.z;
}

fn blend_material_layers(
    texture_layers: texture_2d_array<f32>,
    world_position: vec3<f32>,
    surface_normal: vec3<f32>,
    material_ids: vec4<u32>,
    material_weights: vec4<f32>,
) -> vec4<f32> {
    let normalized_weights = material_weights /
        max(dot(material_weights, vec4<f32>(1.0)), 0.0001);
    var blended_sample = vec4<f32>(0.0);
    for (var layer_index = 0u; layer_index < 4u; layer_index = layer_index + 1u) {
        blended_sample += triplanar_layer_sample(
            texture_layers,
            world_position,
            surface_normal,
            material_ids[layer_index],
        ) * normalized_weights[layer_index];
    }
    return blended_sample;
}

@vertex
fn vertex(input: TerrainVertexInput) -> TerrainVertexOutput {
    return TerrainVertexOutput(
        vec4<f32>(input.position, 1.0),
        input.position,
        normalize(input.normal),
        input.material_ids,
        input.material_weights,
    );
}

@fragment
fn fragment(input: TerrainVertexOutput) -> @location(0) vec4<f32> {
    let albedo = blend_material_layers(
        terrain_albedo_layers,
        input.world_position,
        input.normal,
        input.material_ids,
        input.material_weights,
    );
    let normal_detail = blend_material_layers(
        terrain_normal_layers,
        input.world_position,
        input.normal,
        input.material_ids,
        input.material_weights,
    ).xyz * 2.0 - vec3<f32>(1.0);
    let orm = blend_material_layers(
        terrain_orm_layers,
        input.world_position,
        input.normal,
        input.material_ids,
        input.material_weights,
    );
    let detail_normal = normalize(normal_detail);
    let lighting = 0.6 + 0.4 * max(dot(input.normal, detail_normal), 0.0);
    let roughness = clamp(orm.g, 0.0, 1.0);
    return vec4<f32>(albedo.rgb * lighting * mix(0.8, 1.0, roughness), albedo.a);
}
