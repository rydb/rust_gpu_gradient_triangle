// struct VertexInput {
//     @location(0) position: vec3<f32>,
//     @location(1) color: vec3<f32>,
// };

// struct VertexOutput {
//     @builtin(position) clip_position: vec4<f32>,
//     @location(0) color: vec3<f32>,
// };

// @vertex
// fn vs_main(
//     model: VertexInput, 
// ) -> VertexOutput {
//     var out: VertexOutput;
//     out.color = model.color;
//     out.clip_position = vec4<f32>(
//         model.position.x,
//         model.position.y,
//         model.position.z,
//         1.0
//     );
//     return out;
// }

// // Fragment shader

// @fragment
// fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//     return vec4<f32>(in.color.x, in.color.y, in.color.z, 1.0);
// }

#![cfg_attr(target_arch = "spirv", no_std)]
//use shared::glam::{vec4, Vec4};
use spirv_std::{glam::{vec4, Vec3, Vec4}, spirv};

pub struct VertexInput {
    pub position: Vec3,
    pub color: Vec3,
}

pub struct VertexOutput {
    pub clip_position: Vec4,
    pub color: Vec3
}

#[spirv(vertex)]
pub fn main_vs(
    model: VertexInput, 
) -> VertexOutput {
    return VertexOutput {
        clip_position: vec4(model.position.x, model.position.y, model.position.z, 1.0),
        color: model.color
    }
}

#[spirv(fragment)]
pub fn main_fs(
    input: &mut VertexOutput
) {
    input.color = input.color;
}