use naga::{front::wgsl, valid::Validator};
use winit::{event_loop::EventLoop, window::WindowBuilder};


pub mod gradient_triangle_pipeline;


pub fn parse_wgsl_shader(shader_as_str: &str) {
    let mut module = wgsl::parse_str(shader_as_str).unwrap();

    //println!("validating shader:");
    //println!("{:#?}", module);
    let validation_results = Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    )
    .validate(&module);
    match validation_results {
        Ok(module_info) => {
            //println!("valid shader: printing shader");
            //println!("{:#?}", module);
            for entry in module.entry_points.iter_mut() {
                //println!("entry point: {:#} \n {:#?}", entry.name, entry.function);
                entry.name += "_AMMENDTEST";
                //println!("function arguements \n {:#?}", entry.function.arguments);
            }
            //outputs edited wgsl

            let mut wgsl_out = String::new();
            let flags = naga::back::wgsl::WriterFlags::empty();
            let mut writer = naga::back::wgsl::Writer::new(&mut wgsl_out, flags);
            writer.write(&module, &module_info).unwrap();
            //fs::write("src/wgsl_out.wgsl", wgsl_out);
        }
        Err(err) => {
            panic!("SHADER INVALID: Reason: {:#?}", err);
        }
    }
}

pub async fn run_shader(shader_as_str: &str) {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // State::new uses async code, so we're going to wait for it to finish
    let mut state = State::new(&window, shader_as_str).await;
    let mut surface_configured = false;

    event_loop
        .run(move |event, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => {
                    if !state.input(event) {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                event:
                                    KeyEvent {
                                        state: ElementState::Pressed,
                                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => control_flow.exit(),
                            WindowEvent::Resized(physical_size) => {
                                surface_configured = true;
                                state.resize(*physical_size);
                            }
                            WindowEvent::RedrawRequested => {
                                // This tells winit that we want another frame after this one
                                state.window().request_redraw();

                                if !surface_configured {
                                    return;
                                }

                                state.update();
                                match state.render() {
                                    Ok(_) => {}
                                    // Reconfigure the surface if it's lost or outdated
                                    Err(
                                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                                    ) => state.resize(state.size),
                                    // The system is out of memory, we should probably quit
                                    Err(wgpu::SurfaceError::OutOfMemory) => {
                                        log::error!("OutOfMemory");
                                        control_flow.exit();
                                    }

                                    // This happens when the a frame takes too long to present
                                    Err(wgpu::SurfaceError::Timeout) => {
                                        log::warn!("Surface timeout")
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        })
        .unwrap();
}