mod objects;

use gl;
use std::ffi;
use std::path::Path;

use std::time;
use rand::Rng;
use cgmath::prelude::InnerSpace;
use cgmath::{ Matrix, Matrix4, Vector3, Vector4, vec3, vec4 };
use objects::{ Mesh, RenderObject, Camera, Texture, ViewPort, Material, Light, LightType, figures };

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
    // windowed_context.window().set_cursor_grab(true).unwrap();
    // windowed_context.window().set_cursor_visible(false);

    let fullscreen = window::Fullscreen::Exclusive(prompt_for_video_mode(
        &prompt_for_monitor(&event_loop)));

    // Создание контекста OpenGl
    let gl_context = windowed_context.context();
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    // Загрузка модели
    let mesh: Mesh = figures::normal_cube();

    // Загрузка текстур
    let texture_loadresult = Texture::from_file(Path::new("Pictures/Plite.png").to_str().unwrap());
    let texture3 = match texture_loadresult {
        Ok(texture) => texture,
        Err(err) => { println!("{}", err); return }
    };
    let texture_loadresult = Texture::from_file(Path::new("Pictures/Plite.png").to_str().unwrap());
    let light_map3 = match texture_loadresult {
        Ok(texture) => texture,
        Err(err) => { println!("{}", err); return }
    };

    // Пути к файлам шейдеров
    let vert_filename = Path::new("Shaders/object.vert").to_str().unwrap();
    let frag_filename = Path::new("Shaders/object.frag").to_str().unwrap();
    let light_frag_filename = Path::new("Shaders/light.frag").to_str().unwrap();

    // Загрузка и компиляция шейдеров
    let shader_loadresult = objects::ShaderProgram::from_files(
        vert_filename, frag_filename);
    let shader_program = match shader_loadresult {
        Ok(program) => program,
        Err(err) => { println!("{}", err); return }
    };

    let shader_loadresult = objects::ShaderProgram::from_files(
        vert_filename, light_frag_filename);
    let light_shader_program = match shader_loadresult {
        Ok(program) => program,
        Err(err) => { println!("{}", err); return }
    };

    let mut view_port = ViewPort::new();

    let mut camera = Camera::new();
    camera.set_is_look_at(false);
    camera.set_is_ortho(false);
    camera.set_position(vec3(0.0, 0.0, 1.0));

    // ----- Рандомные кубы ------
    let mut render_objects: Vec<RenderObject> = vec![];
    let mut rend_obj = RenderObject::from_mesh(mesh.clone());
    let mut material = Material::new();
    material.ambient = vec3(0.0, 0.0, 0.0);
    material.diffuse = vec3(0.3, 0.3, 0.3);
    material.specular = vec3(1.0, 1.0, 1.0);
    rend_obj.set_shininess(32.0);
    rend_obj.set_material(material);
    render_objects.push(rend_obj);
    
    for i in 1..500 {
        let mut new_object = RenderObject::from_mesh(mesh.clone());
        let mut material = Material::new();
        material.ambient = vec3(0.0, 0.0, 0.0);
        material.diffuse = generate_normal_vector();
        material.specular = generate_normal_vector();
        // material.ambient = vec3(0.1, 0.1, 0.1);
        // material.diffuse = vec3(0.3, 0.3, 0.3);
        // material.specular = vec3(1.0, 1.0, 1.0);

        //new_object.set_texture(texture3.clone());
        new_object.set_shininess(generate_float());
        new_object.set_position(generate_vector());
        new_object.set_scale(generate_float() / 100.0);
        new_object.set_material(material);
        render_objects.push(new_object);
    }
    // ---------------------------------------------------


    // ----- Рандомные светильники ------
    let mut light_objects: Vec<Light> = vec![];

    // // Фонарик
    // let mut light_obj = Light::new();
    // light_obj.set_direction(vec3(1.0, 0.0, 0.0));
    // light_obj.set_ambient(vec3(0.0, 0.0, 0.0));
    // light_obj.set_diffuse(vec3(0.4, 0.4, 0.4));
    // light_obj.set_specular(vec3(0.8, 0.8, 0.8));
    // light_obj.set_cut_off(5.0);
    // light_obj.set_outer_cut_off(20.0);
    // light_obj.set_light_type(LightType::Spotlight);
    // light_objects.push(light_obj);

    for i in 1..50 {
        let mut new_object = Light::new();
        new_object.set_position(generate_vector());
        new_object.set_scale(0.2);

        new_object.set_ambient(generate_normal_vector());
        new_object.set_diffuse(generate_normal_vector());
        new_object.set_specular(generate_normal_vector());

        new_object.set_constant(1.0);
        new_object.set_linear(0.022);
        new_object.set_quadratic(0.0019);
        
        new_object.set_light_type(LightType::Point);
        new_object.set_mesh(mesh.clone());
        light_objects.push(new_object);
    }
    // ---------------------------------------------------

    let now = time::Instant::now();
    let mut old_since_time = now.elapsed().as_nanos();
    let radius = 50.0;

    let mut is_fullscreen = false;
    let mut draw_mode = 0;
    let sensitivity = 0.7;
    let mut camera_number = 0;
    let mut is_look_at = false;

    let normal_speed_step = 0.004;
    let fast_speed_step = 0.007;
    let mut current_speed_step = normal_speed_step;
    let max_normal_speed = 0.1;
    let max_fast_speed = 1.0;
    let mut current_max_speed = max_normal_speed;
    let mut speed = 0.0;
    
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

    // let mut is_light_up = true;
    // let mut is_light_down = false;

    // Первоначальная настройка пайплайна
    unsafe { 
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::PointSize(3.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::FrontFace(gl::CW);
        gl::CullFace(gl::BACK);
    }

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
                                windowed_context.window().set_cursor_grab(true).unwrap();
                                windowed_context.window().set_cursor_visible(false);
                                windowed_context.window().set_fullscreen(Some(fullscreen.clone()));
                                is_fullscreen = true;
                            }
                            else {
                                windowed_context.window().set_cursor_grab(false).unwrap();
                                windowed_context.window().set_cursor_visible(true);
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
                        event::KeyboardInput { scancode: 38, state: event::ElementState::Released, .. } => 
                            is_look_at = !is_look_at,

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
                            { 
                                current_speed_step = normal_speed_step; 
                                current_max_speed = max_normal_speed;
                            },

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
                            {
                                current_speed_step = fast_speed_step;
                                current_max_speed = max_fast_speed;
                            },
                        
                        // event::KeyboardInput { scancode, state, .. } => println!("{:?} {:?}", scancode, state),
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
                    set_cullface_mode(draw_mode);
                }

                // Дельта времени
                let since_time = now.elapsed().as_nanos();
                let delta_time = ((since_time - old_since_time) as f32) / 10000000.0;
                old_since_time = since_time;

                let view_width = windowed_context.window().inner_size().width as i32;
                let view_height = windowed_context.window().inner_size().height as i32;
                view_port.set_position((0, 0));
                view_port.set_size((view_width, view_height));

                let offset_x = delta_x * sensitivity * delta_time as f64;
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

                camera.set_is_look_at(is_look_at);
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

                // light_objects[0].set_position(camera.position());
                // light_objects[0].set_direction(-camera.direction());

                // Вращение по кругу
                let elapsed_time = now.elapsed();
                let rotate_value = (elapsed_time.as_millis() as f32) / 5000.0;
                let camx = rotate_value.sin() * radius;
                let camy = rotate_value.cos() * radius;
                light_objects[0].set_position(vec3(camx, (camx + camy) / 2.0, camy));
                //light_objects[0].set_direction(vec3(camx, (camx + camy) / 2.0, camy));
                for i in light_objects.iter_mut() {
                    let pos = i.position();
                    let length = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
                    i.set_power(((elapsed_time.as_millis() as f32) / (length * 10.0)).sin() + 0.5);
                }

                let mut matrix = Matrix4::from_scale(1.0);
                if forward || back || right || left || up || down {
                    if speed > current_max_speed { speed -= fast_speed_step * 4.0; }
                    else { speed += current_speed_step; }
                }
                else { speed = 0.0; }
                if forward {
                    matrix = Matrix4::from_translation(-camera.direction() * speed * delta_time);
                    camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                }
                if back {
                    matrix = Matrix4::from_translation(camera.direction() * speed * delta_time);
                    camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                }
                if right {
                    matrix = Matrix4::from_translation(camera.right() * speed * delta_time);
                    camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                }
                if left {
                    matrix = Matrix4::from_translation(-camera.right() * speed * delta_time);
                    camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                }
                if up {
                    matrix = Matrix4::from_translation(camera.up() * speed * delta_time);
                    camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                }
                if down {
                    matrix = Matrix4::from_translation(-camera.up() * speed * delta_time);
                    camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                }

                shader_program.use_();
                if draw_mode == 0 { shader_program.set_uniform_int("wire_mode", 0); }
                else { shader_program.set_uniform_int("wire_mode", 1); }
                shader_program.set_uniform_int("material.diffuse", 0);
                shader_program.set_uniform_int("material.specular", 1);

                view_port.draw(&shader_program, &light_shader_program, &mut camera, &render_objects, &light_objects);
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

fn set_cullface_mode(value: u32) {
    unsafe {
        match value {
            0 => gl::Enable(gl::CULL_FACE),
            1 => gl::Disable(gl::CULL_FACE),
            2 => gl::Disable(gl::CULL_FACE),
            _ => gl::Enable(gl::CULL_FACE)
        }
    }
}

fn prompt_for_monitor(event_loop: &event_loop::EventLoop<()>) -> monitor::MonitorHandle {
    event_loop.available_monitors().nth(0).unwrap()
}

fn prompt_for_video_mode(monitor: &monitor::MonitorHandle) -> monitor::VideoMode {
    monitor.video_modes().nth(0).unwrap()
}

fn generate_float() -> f32 {
    let mut rng = rand::thread_rng();
    let result = rng.gen_range(0.0..256.0);
    return result;
}

fn generate_vector() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let mut generator = || -> f32 { (rng.gen_range(-1200..1200) as f32) / 10.0 };
    vec3(generator(), generator(), generator())
}

fn generate_normal_vector() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let mut generator = || -> f32 { (rng.gen_range(0..1000) as f32) / 10.0 };
    vec3(generator() / 100.0, generator() / 100.0, generator() / 100.0)
}
