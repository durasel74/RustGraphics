mod objects;

use gl;
use std::ffi;
use std::path::Path;

use std::time;
use rand::Rng;
use cgmath::prelude::InnerSpace;
use cgmath::{ Rad, Matrix, Matrix4, Vector3, vec3, PerspectiveFov, Ortho };
use objects::{ Mesh, RenderObject, Camera, Texture, ViewPort, figures };

use glutin;
use glutin::window;
use glutin::event;
use glutin::event_loop;
use glutin::dpi;
use glutin::monitor;

fn main() {
    // Создание окна
    let event_loop = event_loop::EventLoop::new();
    let window_builder = window::WindowBuilder::new()
        .with_visible(true)
        .with_inner_size(dpi::LogicalSize::new(800, 600))
        .with_min_inner_size(dpi::LogicalSize::new(400, 300))
        .with_resizable(true)
        .with_title("Rust Graphics");
    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    windowed_context.window().set_cursor_grab(true).unwrap();
    windowed_context.window().set_cursor_visible(false);

    let fullscreen = window::Fullscreen::Exclusive(prompt_for_video_mode(
        &prompt_for_monitor(&event_loop)));

    // Создание контекста OpenGl
    let gl_context = windowed_context.context();
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    // Загрузка модели
    let mesh: Mesh = figures::create_sphere(3.0, 60, 20);

    // Загрузка текстур
    let texture_loadresult = Texture::from_file(Path::new("Pictures/container.jpg").to_str().unwrap());
    let texture1 = match texture_loadresult {
        Ok(texture) => texture,
        Err(err) => { println!("{}", err); return }
    };

    // Пути к файлам шейдеров
    let vert_filename = Path::new("Shaders/triangles.vert").to_str().unwrap();
    let frag_filename = Path::new("Shaders/triangles.frag").to_str().unwrap();

    // Загрузка и компиляция шейдеров
    let shader_loadresult = objects::ShaderProgram::from_files(
        vert_filename, frag_filename);
    let shader_program = match shader_loadresult {
        Ok(program) => program,
        Err(err) => { println!("{}", err); return }
    };

    let mut view_port = ViewPort::new();

    let mut camera = Camera::new();
    camera.set_is_look_at(true);
    camera.set_is_ortho(false);
    camera.set_position(vec3(0.0, 0.0, 1.0));

    let mut render_objects: Vec<RenderObject> = vec![];
    let mut rng = rand::thread_rng();
    let mut generator = || -> f32 { (rng.gen_range(-1000..1000) as f32) / 10.0 };
    for i in 1..1000 {
        let mut new_object = RenderObject::from_mesh(mesh.clone());
        new_object.set_position(vec3(generator(), generator(), generator()));
        render_objects.push(new_object);
    }
    render_objects.push(RenderObject::from_mesh(mesh.clone()));

    // let mult = 50;
    // for i in 1..10 {
    //     for j in 1..10 {
    //         for k in 1..10 {
    //             let x = (i * mult) as f32;
    //             let z = (j * mult) as f32;
    //             let y = (k * mult) as f32;
    //             let mut new_object = RenderObject::from_mesh(mesh.clone());
    //             new_object.set_position(vec3(x, y, z));
    //             new_object.set_scale(20.0);
    //             render_objects.push(new_object);
    //         }
    //     }
    // }
    
    // Первоначальная настройка пайплайна
    unsafe { 
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::PointSize(3.0);
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut is_fullscreen = false;
    let mut draw_mode = 0;
    let sensitivity = 1.0;
    let mut camera_number = 0;

    let normal_speed = 0.2;
    let fast_speed = 1.0;
    let mut speed = normal_speed;
    
    let mut forward = false;
    let mut back = false;
    let mut left = false;
    let mut right = false;
    let mut up = false;
    let mut down = false;

    let mut yaw = 0.0f32;
    let mut pitch = 0.0f32;
    let mut delta_x = 0.0;
    let mut delta_y = 0.0;
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = event_loop::ControlFlow::Poll;

        match event {
            event::Event::LoopDestroyed => return,
            event::Event::WindowEvent { event, .. } =>
                window_event_handler(event, control_flow),
            event::Event::DeviceEvent { event, .. } => {
                match event {
                    event::DeviceEvent::Key(keyboard_input) => match keyboard_input {
                        event::KeyboardInput { scancode: 1, state: event::ElementState::Released, .. } => 
                            *control_flow = event_loop::ControlFlow::Exit,
                        event::KeyboardInput { scancode: 15, state: event::ElementState::Released, .. } => 
                            draw_mode = (draw_mode + 1) % 3,
                        event::KeyboardInput { scancode: 28, state: event::ElementState::Released, .. } => 
                        {
                            if !is_fullscreen {
                                windowed_context.window().set_fullscreen(Some(fullscreen.clone()));
                                is_fullscreen = true;
                            }
                            else {
                                windowed_context.window().set_fullscreen(None);
                                is_fullscreen = false;
                            }
                        },
                        event::KeyboardInput { scancode: 25, state: event::ElementState::Released, ..} =>
                        {
                            if camera.is_ortho() { camera.set_is_ortho(false); }
                            else { camera.set_is_ortho(true); }
                        },
                        event::KeyboardInput { scancode: 51, state: event::ElementState::Released, ..} =>
                        {
                            camera_number -= 1;
                            if camera_number < 0 { camera_number = 1; }
                        },
                        event::KeyboardInput { scancode: 52, state: event::ElementState::Released, ..} =>
                        {
                            camera_number += 1;
                            if camera_number > 1 { camera_number = 0; }
                        },

                        event::KeyboardInput { scancode: 17, state: event::ElementState::Released, ..} =>
                            forward = false,
                        event::KeyboardInput { scancode: 31, state: event::ElementState::Released, ..} =>
                            back = false,
                        event::KeyboardInput { scancode: 30, state: event::ElementState::Released, ..} =>
                            left = false,
                        event::KeyboardInput { scancode: 32, state: event::ElementState::Released, ..} =>
                            right = false,
                        event::KeyboardInput { scancode: 29, state: event::ElementState::Released, ..} =>
                            down = false,
                        event::KeyboardInput { scancode: 57, state: event::ElementState::Released, ..} =>
                            up = false,
                        event::KeyboardInput { scancode: 42, state: event::ElementState::Released, ..} =>
                            speed = normal_speed,

                        event::KeyboardInput { scancode: 17, state: event::ElementState::Pressed, ..} =>
                            forward = true,
                        event::KeyboardInput { scancode: 31, state: event::ElementState::Pressed, ..} =>
                            back = true,
                        event::KeyboardInput { scancode: 30, state: event::ElementState::Pressed, ..} =>
                            left = true,
                        event::KeyboardInput { scancode: 32, state: event::ElementState::Pressed, ..} =>
                            right = true,
                        event::KeyboardInput { scancode: 29, state: event::ElementState::Pressed, ..} =>
                            down = true,
                        event::KeyboardInput { scancode: 57, state: event::ElementState::Pressed, ..} =>
                            up = true,
                        event::KeyboardInput { scancode: 42, state: event::ElementState::Pressed, ..} =>
                            speed = fast_speed,
                        
                        //event::KeyboardInput { scancode, state, .. } => println!("{:?} {:?}", scancode, state),
                        _ => ()
                    },

                    event::DeviceEvent::MouseMotion { delta } =>
                    {
                        delta_x = delta.0;
                        delta_y = delta.1;
                        //println!("{} {}", delta_x, delta_y);
                    },
                    event::DeviceEvent::MouseWheel { delta } => match delta {
                        event::MouseScrollDelta::LineDelta(_, y) => 
                            camera.set_ortho_factor(camera.ortho_factor() - y),
                        _ => (),
                    },
                    _ => ()
                }
            }
            event::Event::MainEventsCleared => {
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    gl::PolygonMode(gl::FRONT_AND_BACK, to_draw_mode(draw_mode));

                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, texture1.id());
                }

                let view_width = windowed_context.window().inner_size().width as i32;
                let view_height = windowed_context.window().inner_size().height as i32;
                view_port.set_position((0, 0));
                view_port.set_size((view_width, view_height));

                let offset_x = delta_x * sensitivity;
                let offset_y = delta_y * sensitivity;
                delta_x = 0.0;
                delta_y = 0.0;
                yaw += offset_x as f32;
                pitch += offset_y as f32;
                if (pitch > 89.0) { pitch = 89.0; }
                if (pitch < -89.0) { pitch = -89.0; }

                let radians_yaw = yaw.to_radians();
                let radians_pitch = pitch.to_radians();
                let direct_x = radians_yaw.cos() * radians_pitch.cos();
                let direct_y = radians_pitch.sin();
                let direct_z = radians_yaw.sin() * radians_pitch.cos();

                if !camera.is_look_at() {
                    let direction = vec3(direct_x, direct_y, direct_z).normalize();
                    camera.set_direction(direction);
                }
                else {
                    let factor = camera.ortho_factor();
                    camera.set_position(vec3(
                        direct_x * factor,
                        direct_y * factor,
                        direct_z * factor
                    ));
                }

                let mut matrix = Matrix4::from_scale(1.0);
                if forward {
                    matrix = Matrix4::from_translation(-camera.direction() * speed);
                    camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                }
                if back {
                    matrix = Matrix4::from_translation(camera.direction() * speed);
                    camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                }
                if right {
                    matrix = Matrix4::from_translation(camera.right() * speed);
                    camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                }
                if left {
                    matrix = Matrix4::from_translation(-camera.right() * speed);
                    camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                }
                if up {
                    matrix = Matrix4::from_translation(camera.up() * speed);
                    camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                }
                if down {
                    matrix = Matrix4::from_translation(-camera.up() * speed);
                    camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                }

                shader_program.use_();
                shader_program.set_uniform_int("texture1", 0);
                if draw_mode == 0 { shader_program.set_uniform_int("wire_mode", 0); }
                else { shader_program.set_uniform_int("wire_mode", 1); }

                view_port.draw(&shader_program, &mut camera, &render_objects);
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}

fn window_event_handler(event: event::WindowEvent, 
control_flow: &mut event_loop::ControlFlow) {
    match event {
        event::WindowEvent::CloseRequested =>
            *control_flow = event_loop::ControlFlow::Exit,
        event::WindowEvent::Resized(physical_size) => unsafe {
            //gl::Viewport(0, 0, physical_size.width as i32, physical_size.height as i32); 
        },
        _ => ()
    }
}

fn to_draw_mode(value: u32) -> gl::types::GLenum {
    match value {
        0 => gl::FILL,
        1 => gl::LINE,
        2 => gl::POINT,
        _ => gl::FILL
    }
}

fn prompt_for_monitor(event_loop: &event_loop::EventLoop<()>) -> monitor::MonitorHandle {
    event_loop.available_monitors().nth(0).unwrap()
}

fn prompt_for_video_mode(monitor: &monitor::MonitorHandle) -> monitor::VideoMode {
    monitor.video_modes().nth(0).unwrap()
}
