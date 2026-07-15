// Raw-voxel diagnostic placeholder. Material IDs select a stable false color.

struct RawVoxelVertexInput {
    @location(0) position: vec3<f32>,
    @location(1) material_id: u32,
}

struct RawVoxelVertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) @interpolate(flat) material_id: u32,
}

@vertex
fn raw_voxel_vertex(input: RawVoxelVertexInput) -> RawVoxelVertexOutput {
    var output: RawVoxelVertexOutput;
    output.clip_position = vec4<f32>(input.position, 1.0);
    output.material_id = input.material_id;
    return output;
}

@fragment
fn raw_voxel_fragment(input: RawVoxelVertexOutput) -> @location(0) vec4<f32> {
    let material_id = input.material_id;
    let diagnostic_band = f32(material_id & 3u) / 3.0;
    let diagnostic_layer = f32((material_id >> 2u) & 3u) / 3.0;
    return vec4<f32>(diagnostic_band, diagnostic_layer, 1.0 - diagnostic_band, 1.0);
}
