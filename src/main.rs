mod objects;

use gl;
use std::f32;
use std::time;
use std::path::Path;
use rand::Rng;

use cgmath::prelude::InnerSpace;
use cgmath::{ Matrix4, Vector3, vec3 };
use objects::{ RenderObject, Camera, ViewPort, Light, LightType, obj_loader };

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

    // ----- Модели ------
    let mut render_objects: Vec<RenderObject> = vec![];

    let plane_model_path = Path::new("Models/Plane/Model.obj").to_str().unwrap();
    let plane = obj_loader::load_model(plane_model_path);
    render_objects.push(plane);
    

    let model_path = Path::new("Models/KabutoKatana/Model.obj").to_str().unwrap();
    let mut rend_obj = obj_loader::load_model(model_path);
    rend_obj.set_position(vec3(0.0, 0.3, 0.0));
    render_objects.push(rend_obj);

    let model_path = Path::new("Models/Cube/Model.obj").to_str().unwrap();
    let mut rend_obj = obj_loader::load_model(model_path);
    rend_obj.set_position(vec3(-5.0, 1.0, 0.0));
    render_objects.push(rend_obj);

    let model_path = Path::new("Models/Sphere/Model.obj").to_str().unwrap();
    let mut rend_obj = obj_loader::load_model(model_path);
    rend_obj.set_position(vec3(5.0, 1.0, 0.0));
    let light_meshes = rend_obj.meshes().clone();
    render_objects.push(rend_obj);

    let model_path = Path::new("Models/TestSmooth/Model.obj").to_str().unwrap();
    let mut rend_obj = obj_loader::load_model(model_path);
    rend_obj.set_position(vec3(10.0, 1.0, 0.0));
    render_objects.push(rend_obj);

    let model_path = Path::new("Models/TestGroups/Model.obj").to_str().unwrap();
    let mut rend_obj = obj_loader::load_model(model_path);
    rend_obj.set_position(vec3(-10.0, 1.0, 0.0));
    render_objects.push(rend_obj);
    // ---------------------------------------------------


    // ----- Светильники ------
    let mut light_objects: Vec<Light> = vec![];

    // let mut new_object = Light::new();
    // new_object.set_ambient(vec3(0.0, 0.0, 0.0));
    // new_object.set_diffuse(vec3(1.0, 1.0, 1.0));
    // new_object.set_specular(vec3(1.0, 1.0, 1.0));
    // new_object.set_cut_off(15.0);
    // new_object.set_outer_cut_off(25.0);
    // new_object.set_light_type(LightType::Spotlight);
    // light_objects.push(new_object);

    let mut new_object = Light::new();
    new_object.set_direction(vec3(4.0, -5.0, -4.0));
    new_object.set_ambient(vec3(0.2, 0.2, 0.2));
    new_object.set_diffuse(vec3(1.0, 1.0, 1.0));
    new_object.set_specular(vec3(1.0, 1.0, 1.0));
    new_object.set_light_type(LightType::Directional);
    light_objects.push(new_object);

    let mut new_object = Light::new();
    new_object.set_position(vec3(0.0, 1.0, -5.0));
    new_object.set_ambient(vec3(0.2, 0.2, 0.2));
    new_object.set_diffuse(vec3(0.2, 0.2, 1.0));
    new_object.set_specular(vec3(0.2, 0.2, 1.0));
    new_object.set_constant(1.0);
    new_object.set_linear(0.022);
    new_object.set_quadratic(0.0019);
    new_object.set_light_type(LightType::Point);
    new_object.set_meshes(light_meshes);
    light_objects.push(new_object);
    // ---------------------------------------------------

    let mut view_port = ViewPort::new();
    let mut camera = Camera::new();
    camera.set_is_look_at(false);
    camera.set_is_ortho(false);
    camera.set_position(vec3(2.0, 1.0, 0.0));

    let now = time::Instant::now();
    let mut old_since_time = now.elapsed().as_secs_f32();

    let mut window_is_focused = true;
    let mut is_fullscreen = false;
    let mut draw_mode = 0;
    let sensitivity = 3.0;
    let mut is_look_at = false;

    let normal_speed_step = 0.02;
    let fast_speed_step = 0.07;
    let mut current_speed_step = normal_speed_step;
    let max_normal_speed = 0.6;
    let max_fast_speed = 10.0;
    let mut current_max_speed = max_normal_speed;
    let mut speed = 0.0;
    
    let mut forward = false;
    let mut back = false;
    let mut left = false;
    let mut right = false;
    let mut up = false;
    let mut down = false;
    let mut camera_up = false;
    let mut camera_down = false;
    let mut camera_left = false;
    let mut camera_right = false;
    let mut camera_speed = 1.0;

    let mut yaw = 0.0f32;
    let mut pitch = 0.0f32;
    let mut delta_x = 0.0;
    let mut delta_y = 0.0;

    let mut is_light_togle = true;
    let mut is_half_light = false;

    // Первоначальная настройка рендера
    unsafe { 
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);
        gl::PointSize(3.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::FrontFace(gl::CCW);
        gl::CullFace(gl::BACK);
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = event_loop::ControlFlow::Poll;
        match event {
            event::Event::LoopDestroyed => return,
            event::Event::WindowEvent { event, .. } => {
                match event {
                    event::WindowEvent::CloseRequested =>
                        *control_flow = event_loop::ControlFlow::Exit,
                    event::WindowEvent::Resized(physical_size) => unsafe {
                        let view_width = physical_size.width as i32;
                        let view_height = physical_size.height as i32;
                        view_port.set_position((0, 0));
                        view_port.set_size((view_width, view_height));
                    },
                    event::WindowEvent::Focused(is_focus) => 
                        window_is_focused = is_focus,
                    _ => ()
                }
            },
            event::Event::DeviceEvent { event, .. } => {
                if (window_is_focused) {
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
                            event::KeyboardInput { scancode: 57416, state: event::ElementState::Released, ..} =>
                                is_light_togle = !is_light_togle,
                            event::KeyboardInput { scancode: 57424, state: event::ElementState::Released, ..} =>
                                is_half_light = !is_half_light,
                            event::KeyboardInput { scancode: 42, state: event::ElementState::Released, ..} =>
                                { 
                                    current_speed_step = normal_speed_step; 
                                    current_max_speed = max_normal_speed;
                                },
                            event::KeyboardInput { scancode: 72, state: event::ElementState::Released, ..} =>
                                camera_up = false,
                            event::KeyboardInput { scancode: 80, state: event::ElementState::Released, ..} =>
                                camera_down = false,
                            event::KeyboardInput { scancode: 75, state: event::ElementState::Released, ..} =>
                                camera_left = false,
                            event::KeyboardInput { scancode: 77, state: event::ElementState::Released, ..} =>
                                camera_right = false,


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
                            event::KeyboardInput { scancode: 72, state: event::ElementState::Pressed, ..} =>
                                camera_up = true,
                            event::KeyboardInput { scancode: 80, state: event::ElementState::Pressed, ..} =>
                                camera_down = true,
                            event::KeyboardInput { scancode: 75, state: event::ElementState::Pressed, ..} =>
                                camera_left = true,
                            event::KeyboardInput { scancode: 77, state: event::ElementState::Pressed, ..} =>
                                camera_right = true,
                            event::KeyboardInput { scancode: 78, state: event::ElementState::Pressed, ..} =>
                                camera_speed += 0.1,
                            event::KeyboardInput { scancode: 74, state: event::ElementState::Pressed, ..} =>
                                { camera_speed -= 0.1; if camera_speed < 0.0 { camera_speed = 0.0 } },
                            
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
            }
            event::Event::MainEventsCleared => {
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    gl::PolygonMode(gl::FRONT_AND_BACK, to_draw_mode(draw_mode));
                    set_cullface_mode(draw_mode);
                }

                // Дельта времени
                let since_time = now.elapsed().as_secs_f32();
                let delta_time = (since_time - old_since_time) * 10.0;
                old_since_time = since_time;

                let view_width = windowed_context.window().inner_size().width as i32;
                let view_height = windowed_context.window().inner_size().height as i32;
                view_port.set_position((0, 0));
                view_port.set_size((view_width, view_height));

                // Управление
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
                if camera_up { delta_y = -camera_speed as f64; }
                if camera_down { delta_y = camera_speed as f64; }
                if camera_left { delta_x = -camera_speed as f64; }
                if camera_right { delta_x = camera_speed as f64; }


                if !is_half_light && is_light_togle {
                    for i in light_objects.iter_mut() {
                        if i.power() < 1.1 {
                            let pos = i.position();
                            let lenght = 2100.0 - (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
                            i.set_power(i.power() + lenght / (2100.0 * 25.0));
                        }
                    }
                }
                if !is_light_togle {
                    for i in light_objects.iter_mut() {
                        if i.power() > 0.0001 && (i.radius() > 11.0 || i.radius() < 10.0) {
                            let pos = i.position();
                            let lenght = 2100.0 - (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
                            i.set_power(i.power() - lenght / (2100.0 * 8.0));
                        }
                    }
                }
                if is_half_light {
                    let mut light = &mut light_objects[0];
                    if light.power() > 0.0001 {
                        light.set_power(light.power() - 2100.0 / (2100.0 * 25.0));
                    }
                }


                let offset_x = delta_x * sensitivity * delta_time as f64;
                let offset_y = delta_y * sensitivity * delta_time as f64;
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

                // // Движение фонаря за камерой
                // light_objects[0].set_direction(-camera.direction());
                // light_objects[0].set_position(camera.position());

                // // Вращение по кругу
                // let elapsed_time = now.elapsed();
                // let rotate_value = elapsed_time.as_millis() as f32;
                // for i in light_objects.iter_mut() {
                //     if i.radius() > 10.0 {
                //         let slow_multiple = i.radius() * 20.0;
                //         let camx = (rotate_value / slow_multiple).sin() * i.radius();
                //         let camy = ((rotate_value / slow_multiple) + i.radius()).cos() * i.radius();
                //         let camz = (rotate_value / slow_multiple).cos() * i.radius();
                //         i.set_position(vec3(camx, camy, camz));
                //     }
                // }

                shader_program.use_();
                if draw_mode == 0 { shader_program.set_uniform_int("wire_mode", 0); }
                else { shader_program.set_uniform_int("wire_mode", 1); }

                view_port.draw(&shader_program, &light_shader_program, &mut camera, &render_objects, &light_objects);
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
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
    let random1 = (rng.gen_range(-1000..1000) as f32) / 100.0;
    let random2 = (rng.gen_range(-1000..1000) as f32) / 100.0;
    let theta1 = random1 * 2.0 * f32::consts::PI;
    let theta2 = random2 * 2.0 * f32::consts::PI;
    let radius = (rng.gen_range(0..0_500_000) as f32).cbrt();

    let x = radius * (theta1.cos() * theta2.sin());
    let y = radius * theta1.sin();
    let z = radius * (theta1.cos() * theta2.cos());
    vec3(x, y, z)
}

fn generate_normal_vector() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let mut generator = || -> f32 { (rng.gen_range(0..1000) as f32) / 10.0 };
    vec3(generator() / 100.0, generator() / 100.0, generator() / 100.0)
}
