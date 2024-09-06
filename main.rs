use std::fs;

use spirv_builder::{MetadataPrintout, SpirvBuilder};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shader = "/home/ry/Projects/triangle_shader_test/triangle_shader/assets/shaders/gradient_triangle";
    //let simplest_shader = SHADER_FOLDER.to_owned() + "simplest_shader";
    //let shader_def = include_dir::include_dir!("assets/shaders/gradient_triangle");
    //println!("Shader def: {:#?}", shader_def.path().canonicalize());
    SpirvBuilder::new(shader, "spirv-unknown-spv1.5")
        .print_metadata(MetadataPrintout::Full)
        //.extra_arg(format!("OUT_DIR={:#}", out_dir))
        .build()?;
    Ok(())
}



// pub fn recompile_shaders(shader_crate_name: &str, out_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
//     //let simplest_shader = SHADER_FOLDER.to_owned() + "simplest_shader";
//     let post_processing = "assets/shaders/rust_gpu_shaders/".to_owned() + &shader_crate_name;
//     SpirvBuilder::new(post_processing, "spirv-unknown-spv1.5")
//         .print_metadata(MetadataPrintout::Full)
//         //.extra_arg(format!("OUT_DIR={:#}", out_dir))
//         .build()?;
//     Ok(())
// }

// fn compile_shaders(){
//     let Ok(out_dir_check) = env::current_dir() else {panic!("out dir doesn't exist??")};

//     let out_dir = out_dir_check.to_str().unwrap().to_owned();
//     let spirv_out_folder = out_dir + &"/assets/shaders";


//     //let paths = fs::read_dir("./").unwrap();

//     //FIXME: get proper crate directory at some point. 
//     let this_crate_path = spirv_out_folder.clone() + &"/rust_gpu_shaders";

//     //recompile_all_shaders(&this_crate_path, &spirv_out_folder);

//     println!("recompiling shaders from: {:#?}", this_crate_path);
//     let recompile_results = recompile_shaders("simplest", &spirv_out_folder);

//     println!("{:#?}", recompile_results);
// }
