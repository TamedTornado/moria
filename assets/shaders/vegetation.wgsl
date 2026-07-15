// Development placeholder for the shared vegetation material extension.
// Instance transforms come from Bevy's shared mesh buffer; variation is
// derived from the 32-bit instance index until authored variation is wired in.

#import bevy_pbr::{
    forward_io::Vertex,
    mesh_bindings::mesh,
    mesh_functions,
    view_transformations::position_world_to_clip,
}

struct VegetationVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) variation: vec3<f32>,
}

fn variation_from_instance(instance_index: u32) -> vec3<f32> {
    let seed = instance_index * 1664525u + 1013904223u;
    return vec3<f32>(
        f32(seed & 255u) / 255.0,
        f32((seed >> 8u) & 255u) / 255.0,
        f32((seed >> 16u) & 255u) / 255.0,
    );
}

@vertex
fn vertex(vertex: Vertex) -> VegetationVertexOutput {
    let instance_transform = mesh_functions::get_world_from_local(vertex.instance_index);
    let world_position = mesh_functions::mesh_position_local_to_world(
        instance_transform,
        vec4<f32>(vertex.position, 1.0),
    );

    var output: VegetationVertexOutput;
    output.position = position_world_to_clip(world_position.xyz);
    output.variation = variation_from_instance(vertex.instance_index);
    return output;
}

@fragment
fn fragment(input: VegetationVertexOutput) -> @location(0) vec4<f32> {
    let base_color = mix(vec3<f32>(0.12, 0.28, 0.08), vec3<f32>(0.34, 0.58, 0.18), input.variation);
    return vec4<f32>(base_color, 1.0);
}
