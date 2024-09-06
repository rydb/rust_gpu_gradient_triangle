use std::{
    error,
    fs::{self, File},
    io::Read,
    path::PathBuf,
};
use parse_wgsl_shader;

fn main() {
    let shader_file_name = "minimal_triangle.wgsl";
    let shader_dir = "assets/shaders";

    let shader_as_str = fs::read_to_string(shader_dir.to_owned() + "/" + shader_file_name).unwrap();

    parse_wgsl_shader(&shader_as_str);

    pollster::block_on(run_shader(&shader_as_str));
}
