// Development placeholder for the shared terrain material extension.
// Terrain mesh vertices carry four material layers; every terrain chunk uses
// the same texture-array bindings rather than allocating a material per chunk.

struct TerrainVertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) material_ids: vec4<u32>,
    @location(3) material_weights: vec4<f32>,
};

struct TerrainVertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) surface_normal: vec3<f32>,
    @location(2) @interpolate(flat) material_ids: vec4<u32>,
    @location(3) material_weights: vec4<f32>,
};

@group(1) @binding(0)
var terrain_sampler: sampler;

@group(1) @binding(1)
var terrain_albedo_layers: texture_2d_array<f32>;

@group(1) @binding(2)
var terrain_normal_layers: texture_2d_array<f32>;

@group(1) @binding(3)
var terrain_orm_layers: texture_2d_array<f32>;

fn triplanar_coordinates(position: vec3<f32>, normal: vec3<f32>) -> vec2<f32> {
    let weights = abs(normal) / max(dot(abs(normal), vec3<f32>(1.0)), 0.0001);
    return position.yz * weights.x + position.xz * weights.y + position.xy * weights.z;
}

fn sample_layer_color(
    material_id: u32,
    coordinates: vec2<f32>,
) -> vec3<f32> {
    return textureSample(
        terrain_albedo_layers,
        terrain_sampler,
        coordinates,
        i32(material_id),
    ).rgb;
}

fn sample_layer_normal(
    material_id: u32,
    coordinates: vec2<f32>,
) -> vec3<f32> {
    return textureSample(
        terrain_normal_layers,
        terrain_sampler,
        coordinates,
        i32(material_id),
    ).xyz;
}

fn sample_layer_orm(
    material_id: u32,
    coordinates: vec2<f32>,
) -> vec3<f32> {
    return textureSample(
        terrain_orm_layers,
        terrain_sampler,
        coordinates,
        i32(material_id),
    ).rgb;
}

@vertex
fn terrain_vertex(input: TerrainVertexInput) -> TerrainVertexOutput {
    var output: TerrainVertexOutput;
    output.clip_position = vec4<f32>(input.position, 1.0);
    output.world_position = input.position;
    output.surface_normal = normalize(input.normal);
    output.material_ids = input.material_ids;
    output.material_weights = input.material_weights;
    return output;
}

@fragment
fn terrain_fragment(input: TerrainVertexOutput) -> @location(0) vec4<f32> {
    let coordinates = triplanar_coordinates(input.world_position, input.surface_normal);
    let weights = input.material_weights / max(dot(input.material_weights, vec4<f32>(1.0)), 0.0001);
    let albedo = sample_layer_color(input.material_ids.x, coordinates) * weights.x
        + sample_layer_color(input.material_ids.y, coordinates) * weights.y
        + sample_layer_color(input.material_ids.z, coordinates) * weights.z
        + sample_layer_color(input.material_ids.w, coordinates) * weights.w;
    let normal_detail = sample_layer_normal(input.material_ids.x, coordinates) * weights.x
        + sample_layer_normal(input.material_ids.y, coordinates) * weights.y
        + sample_layer_normal(input.material_ids.z, coordinates) * weights.z
        + sample_layer_normal(input.material_ids.w, coordinates) * weights.w;
    let orm = sample_layer_orm(input.material_ids.x, coordinates) * weights.x
        + sample_layer_orm(input.material_ids.y, coordinates) * weights.y
        + sample_layer_orm(input.material_ids.z, coordinates) * weights.z
        + sample_layer_orm(input.material_ids.w, coordinates) * weights.w;
    let light = max(input.surface_normal.y, 0.0);
    let detail = 0.85 + 0.15 * normal_detail.z;
    return vec4<f32>(albedo * (0.25 + 0.75 * light) * detail * (0.5 + 0.5 * orm.y), 1.0);
}
