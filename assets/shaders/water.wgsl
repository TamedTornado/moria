// Placeholder water material extension.
//
// Water surfaces are static generated geometry. `water_time` is presentation
// state only and never contributes to authoritative world state.

struct WaterTime {
    seconds: f32,
    _padding: vec3<f32>,
};

struct WaterVertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct WaterVertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) surface_normal: vec3<f32>,
};

@group(1) @binding(0)
var<uniform> water_time: WaterTime;

@vertex
fn vertex(input: WaterVertex) -> WaterVertexOutput {
    return WaterVertexOutput(
        vec4<f32>(input.position, 1.0),
        normalize(input.normal),
    );
}

@fragment
fn fragment(input: WaterVertexOutput) -> @location(0) vec4<f32> {
    let normal_light = max(input.surface_normal.y, 0.0);
    let normal_motion = fract(water_time.seconds * 0.05);
    let color = vec3<f32>(0.03, 0.23, 0.36) + vec3<f32>(0.02) * (normal_light + normal_motion);
    return vec4<f32>(color, 0.82);
}
