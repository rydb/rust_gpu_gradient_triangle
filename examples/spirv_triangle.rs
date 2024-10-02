use naga::{back::{spv::Options, wgsl::WriterFlags}, front::wgsl, valid::Validator};
use spirv_builder::{CompileResult, MetadataPrintout, SpirvBuilder, SpirvBuilderError};
use wgpu::{include_spirv_raw, ShaderSource};


fn main()  {
    let spirv_shader = compile_to_spirv().expect("Compile failed, Reason: ");
    let wgsl_shader = transpile_to_wgsl(spirv_shader);
    //let wgsl_shader =
}

pub fn compile_to_spirv() -> Result<CompileResult, SpirvBuilderError> {
            //println!("spirv codegen: {:#}", find_rustc_codegen_spirv());
            let shader = "/home/ry/Projects/rust_gpu_gradient_triangle/assets/shaders/gradient_triangle";
            //let simplest_shader = SHADER_FOLDER.to_owned() + "simplest_shader";
            //let shader_def = include_dir::include_dir!("assets/shaders/gradient_triangle");
            //println!("Shader def: {:#?}", shader_def.path().canonicalize());
            SpirvBuilder::new(shader, "spirv-unknown-spv1.5")
                .print_metadata(MetadataPrintout::Full)
                //.extra_arg(format!("OUT_DIR={:#}", out_dir))
                .build()
}

pub fn transpile_to_wgsl(spirv_result: CompileResult){
    match spirv_result.module {
        spirv_builder::ModuleResult::SingleModule(path_buf) => {
            let shader = include_spirv_raw!("/home/ry/Projects/rust_gpu_gradient_triangle/assets/shaders/gradient_triangle/target/spirv-unknown-spv1.5/release/deps/gradient_triangle.spv");
            //let shader_source = ShaderSource::SpirV(shader.source);

            let shader_code = shader.source.to_vec().clone();
            let module_input = naga::front::spv::Frontend::new(shader_code.into_iter(), &naga::front::spv::Options::default());
            let module = module_input.parse().unwrap();

            println!("naga moudle is {:#?}", module);

            let validation_results = Validator::new(
                naga::valid::ValidationFlags::all(),
                naga::valid::Capabilities::all(),
            )
            .validate(&module);
        
            let module_info = match validation_results {
                Ok(module_info) => module_info,
                Err(err) => {
                    println!("ERROR VALIDATING WGPU");
                    panic!("{:#?}", err.emit_to_stderr(""))
                }
            };
            //println!("module info: {:#?}", module_info);
        
            let wgsl_shader =
                naga::back::wgsl::write_string(&module, &module_info, WriterFlags::EXPLICIT_TYPES).unwrap();

            println!("wgsl_shader: \n {:#}", wgsl_shader)
        },
        spirv_builder::ModuleResult::MultiModule(btree_map) => todo!(),
    }
        
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
