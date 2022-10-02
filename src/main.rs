mod objects;

use gl;
use obj::raw::material::Material;
use objects::{ShaderProgram, obj_loader, Vertex};
use std::f32;
use std::ops::Deref;
use std::time;
use std::path::Path;
use rand::Rng;

use cgmath::prelude::InnerSpace;
use cgmath::{ Matrix4, Vector2, Vector3, vec2, vec3 };
use objects::{ RenderObject, Mesh, Camera, ViewPort, Light, LightType, FrameBuffer };

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
    let select_frag_filename = Path::new("Shaders/select.frag").to_str().unwrap();
    let frame_buffer_vert_filename = Path::new("Shaders/frame_buffer.vert").to_str().unwrap();
    let frame_buffer_frag_filename = Path::new("Shaders/frame_buffer.frag").to_str().unwrap();

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

    let shader_loadresult = objects::ShaderProgram::from_files(
        vert_filename, select_frag_filename);
    let select_shader_program = match shader_loadresult {
        Ok(program) => program,
        Err(err) => { println!("{}", err); return }
    };

    let shader_loadresult = objects::ShaderProgram::from_files(
        frame_buffer_vert_filename, frame_buffer_frag_filename);
    let frame_buffer_shader_program = match shader_loadresult {
        Ok(program) => program,
        Err(err) => { println!("{}", err); return }
    };

    // ----- Модели ------
    let mut render_objects: Vec<RenderObject> = vec![];
    let objects_container = load_spawn_objects();

    // let plane_model_path = Path::new("Models/Plane/Model.obj").to_str().unwrap();
    // let plane = obj_loader::load_model(plane_model_path);
    // render_objects.push(plane);
    
    // Снежные горы
    let model_path = Path::new("Models/Hills/Model.obj").to_str().unwrap();
    let mut rend_obj = obj_loader::load_model(model_path);
    rend_obj.set_position(vec3(0.0, 0.0, 40.0));
    rend_obj.set_scale(100.0);
    render_objects.push(rend_obj);
    
    // // Машина в горах
    // let model_path = Path::new("Models/AmericanMuscle/Model.obj").to_str().unwrap();
    // let mut rend_obj = obj_loader::load_model(model_path);
    // // rend_obj.set_position(vec3(0.0, 0.35, 0.0));
    // // rend_obj.set_rotation(vec3(0.0, 180.0, 0.0));
    // render_objects.push(rend_obj);
    
    // // Оружие под фонарем
    // let model_path = Path::new("Models/PliteFlor/Model.obj").to_str().unwrap();
    // let mut rend_obj = obj_loader::load_model(model_path);
    // rend_obj.set_position(vec3(0.0, -1.0, 0.0));
    // rend_obj.set_scale(0.2);
    // render_objects.push(rend_obj);

    // let model_path = Path::new("Models/SteampunkPistol/Model.obj").to_str().unwrap();
    // let mut rend_obj = obj_loader::load_model(model_path);
    // rend_obj.set_position(vec3(0.0, 0.0, 0.0));
    // render_objects.push(rend_obj);

    // // Средневековый город
    // let model_path = Path::new("Models/MeddleCity/Model.obj").to_str().unwrap();
    // let mut rend_obj = obj_loader::load_model(model_path);
    // rend_obj.set_position(vec3(0.0, 0.0, 0.0));
    // render_objects.push(rend_obj);

    // // Магический мост
    // let model_path = Path::new("Models/MagicalBridge/Model.obj").to_str().unwrap();
    // let mut rend_obj = obj_loader::load_model(model_path);
    // rend_obj.set_position(vec3(0.0, 0.0, 0.0));
    // render_objects.push(rend_obj);

    // // Модерн город
    // let model_path = Path::new("Models/ModernCityBlock/Model.obj").to_str().unwrap();
    // let mut rend_obj = obj_loader::load_model(model_path);
    // rend_obj.set_position(vec3(0.0, 0.0, 0.0));
    // render_objects.push(rend_obj);



    // ---------------------------------------------------


    // ----- Светильники ------
    let mut light_objects: Vec<Light> = vec![];

    let mut new_object = Light::new();
    new_object.set_direction(vec3(4.0, -5.0, -4.0));
    new_object.set_ambient(vec3(0.2, 0.2, 0.2));
    new_object.set_diffuse(vec3(1.0, 1.0, 1.0));
    new_object.set_specular(vec3(1.0, 1.0, 1.0));
    new_object.set_light_type(LightType::Directional);
    light_objects.push(new_object);
    
    // let mut new_object = Light::new();
    // new_object.set_ambient(vec3(0.0, 0.0, 0.0));
    // new_object.set_diffuse(vec3(1.0, 1.0, 1.0));
    // new_object.set_specular(vec3(1.0, 1.0, 1.0));
    // new_object.set_cut_off(15.0);
    // new_object.set_outer_cut_off(25.0);
    // new_object.set_light_type(LightType::Spotlight);
    // light_objects.push(new_object);

    let mut new_object = Light::new();
    new_object.set_position(vec3(0.0, 1.0, -5.0));
    new_object.set_ambient(vec3(0.2, 0.2, 0.2));
    new_object.set_diffuse(vec3(0.2, 0.2, 1.0));
    new_object.set_specular(vec3(0.2, 0.2, 1.0));
    new_object.set_constant(1.0);
    new_object.set_linear(0.022);
    new_object.set_quadratic(0.0019);
    new_object.set_light_type(LightType::Point);
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

    let mut max_normal_speed = 0.60;
    let mut max_fast_speed = max_normal_speed * 5.00;
    let mut current_max_speed = max_normal_speed;
    let mut normal_speed_step = max_normal_speed / 100.0;
    let mut fast_speed_step = max_fast_speed / 100.0;
    let mut current_speed_step = normal_speed_step;
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
    let mut camera_closer = false;
    let mut camera_father = false;
    let mut camera_speed = 1.0;

    let mut yaw = 0.0f32;
    let mut pitch = 0.0f32;
    let mut delta_x = 0.0;
    let mut delta_y = 0.0;

    let mut objects_container_index = 0;
    let mut is_spawn_test = false;
    let mut spawning_obj: Vec<RenderObject> = vec![];
    // let mut spawning_obj_index = 0;
    let mut spawning_obj_scale = 1.0;
    let mut spawning_obj_angle = 0.0;

    let win_width = windowed_context.window().inner_size().width;
    let win_height = windowed_context.window().inner_size().height;
    let mut frame_buffer = FrameBuffer::new(win_width, win_height).unwrap();
    let mut frame_buffer_mesh = create_frame_buffer_mesh(&frame_buffer);

    // Первоначальная настройка рендера
    unsafe { 
        gl::ClearColor(0.4, 0.6, 0.8, 1.0);
        gl::PointSize(3.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::STENCIL_TEST);
        gl::StencilOp(gl::KEEP, gl::KEEP, gl::REPLACE);

        gl::Enable(gl::CULL_FACE);
        gl::FrontFace(gl::CCW);
        gl::CullFace(gl::BACK);
        
        gl::Enable(gl::BLEND);
        gl::BlendEquation(gl::FUNC_ADD);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
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
                        let view_width = windowed_context.window().inner_size().width as i32;
                        let view_height = windowed_context.window().inner_size().height as i32;
                        view_port.set_position((0, 0));
                        view_port.set_size((view_width, view_height));
                        frame_buffer = FrameBuffer::new(view_width as u32, view_height as u32).unwrap();
                        frame_buffer_mesh = create_frame_buffer_mesh(&frame_buffer);
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
                                draw_mode = (draw_mode + 1) % 4,
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
                            event::KeyboardInput { scancode: 83, state: event::ElementState::Released, ..} =>
                                camera_father = false,
                            event::KeyboardInput { scancode: 82, state: event::ElementState::Released, ..} =>
                                camera_closer = false,
                            event::KeyboardInput { scancode: 46, state: event::ElementState::Released, ..} =>
                                camera.set_target(camera.position()),

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
                            event::KeyboardInput { scancode: 13, state: event::ElementState::Pressed, ..} =>
                                {
                                    max_normal_speed += 0.05;
                                    max_fast_speed = max_normal_speed * 5.0;
                                    current_max_speed = max_normal_speed;
                                    normal_speed_step = max_normal_speed / 100.0;
                                    fast_speed_step = max_fast_speed / 100.0;
                                    current_speed_step = normal_speed_step;
                                }
                            event::KeyboardInput { scancode: 12, state: event::ElementState::Pressed, ..} =>
                                {
                                    max_normal_speed -= 0.05;
                                    if max_normal_speed < 0.05 { max_normal_speed = 0.0 }
                                    max_fast_speed = max_normal_speed * 5.0;
                                    current_max_speed = max_normal_speed;
                                    normal_speed_step = max_normal_speed / 100.0;
                                    fast_speed_step = max_fast_speed / 100.0;
                                    current_speed_step = normal_speed_step;
                                },
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
                            event::KeyboardInput { scancode: 83, state: event::ElementState::Pressed, ..} =>
                                camera_father = true,
                            event::KeyboardInput { scancode: 82, state: event::ElementState::Pressed, ..} =>
                                camera_closer = true,
                            event::KeyboardInput { scancode: 78, state: event::ElementState::Pressed, ..} =>
                                camera_speed += 0.1,
                            event::KeyboardInput { scancode: 74, state: event::ElementState::Pressed, ..} =>
                                { camera_speed -= 0.1; if camera_speed < 0.0 { camera_speed = 0.0 } },
                            event::KeyboardInput { scancode: 55, state: event::ElementState::Pressed, ..} =>
                                camera.set_field_of_view(camera.field_of_view() + 0.5),
                            event::KeyboardInput { scancode: 57397, state: event::ElementState::Pressed, ..} =>
                                if camera.field_of_view() > 0.5 { camera.set_field_of_view(camera.field_of_view() - 0.5) },


                            // Спавн объектов
                            event::KeyboardInput { scancode: 2, state: event::ElementState::Released, ..} =>
                                objects_container_index = 0,
                            event::KeyboardInput { scancode: 3, state: event::ElementState::Released, ..} =>
                                objects_container_index = 1,
                            event::KeyboardInput { scancode: 4, state: event::ElementState::Released, ..} =>
                                objects_container_index = 2,
                            event::KeyboardInput { scancode: 5, state: event::ElementState::Released, ..} =>
                                objects_container_index = 3,
                            event::KeyboardInput { scancode: 6, state: event::ElementState::Released, ..} =>
                                objects_container_index = 4,
                            event::KeyboardInput { scancode: 7, state: event::ElementState::Released, ..} =>
                                objects_container_index = 5,
                            event::KeyboardInput { scancode: 8, state: event::ElementState::Released, ..} =>
                                objects_container_index = 6,
                            event::KeyboardInput { scancode: 9, state: event::ElementState::Released, ..} =>
                                objects_container_index = 7,
                            event::KeyboardInput { scancode: 10, state: event::ElementState::Released, ..} =>
                                objects_container_index = 8,
                            event::KeyboardInput { scancode: 11, state: event::ElementState::Released, ..} =>
                                objects_container_index = objects_container.len(),

                            event::KeyboardInput { scancode: 57416, state: event::ElementState::Pressed, ..} =>
                                spawning_obj_scale += 0.1,
                            event::KeyboardInput { scancode: 57424, state: event::ElementState::Pressed, ..} =>
                                if spawning_obj_scale > 0.0 { spawning_obj_scale -= 0.1; },
                            event::KeyboardInput { scancode: 57419, state: event::ElementState::Pressed, ..} =>
                                spawning_obj_angle += 5.0,
                            event::KeyboardInput { scancode: 57421, state: event::ElementState::Pressed, ..} =>
                                spawning_obj_angle -= 5.0,

                            event::KeyboardInput { scancode: 19, state: event::ElementState::Pressed, ..} =>
                                {
                                    if !is_spawn_test {
                                        is_spawn_test = true;
                                        if objects_container_index < objects_container.len() {
                                            // let obj = objects_container[objects_container_index].clone();
                                            // let mut mesh = obj.meshes()[0].clone();
                                            // let mut material = mesh.material().clone();
                                            // material.diff_tex = Some(frame_buffer.color_buffer.clone());
                                            // mesh.set_material(material.clone());
                                            // let ok = RenderObject::from_mesh(mesh.clone());
                                            // spawning_obj.push(ok.clone());

                                            spawning_obj.push(objects_container[objects_container_index].clone());
                                        }
                                    }
                                },
                            event::KeyboardInput { scancode: 19, state: event::ElementState::Released, ..} =>
                                {
                                    is_spawn_test = false;
                                    render_objects.push(spawning_obj.remove(0));
                                },

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
            }
            event::Event::MainEventsCleared => {
                unsafe {
                    gl::ClearColor(0.4, 0.6, 0.8, 1.0);
                    gl::Enable(gl::DEPTH_TEST);
                    gl::Enable(gl::STENCIL_TEST);

                    gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer.fbo);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
                    gl::PolygonMode(gl::FRONT_AND_BACK, to_draw_mode(draw_mode));
                    set_cullface_mode(draw_mode);
                    gl::StencilMask(0x00);
                }

                // Дельта времени
                let since_time = now.elapsed().as_secs_f32();
                let delta_time = (since_time - old_since_time) * 10.0;
                old_since_time = since_time;

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
                if camera_father {
                    let c_speed = 0.2 * camera_speed * delta_time;
                    camera.set_ortho_factor(camera.ortho_factor() + c_speed); 
                }
                if camera_closer {
                    let c_speed = 0.2 * camera_speed * delta_time;
                    camera.set_ortho_factor(camera.ortho_factor() - c_speed); 
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
                        (direct_x * factor) + camera.target().x,
                        (direct_y * factor) + camera.target().y,
                        (direct_z * factor) + camera.target().z
                    ));
                }

                // Движение фонаря за камерой
                let flash_light = light_objects.iter_mut().find(
                    |l| l.light_type() == &LightType::Spotlight);
                if let Some(light_object) = flash_light {
                    light_object.set_direction(-camera.direction());
                    light_object.set_position(camera.position());
                }

                // Измененеие положения объекта перед размещением
                if is_spawn_test {
                    let camera_direction = camera.direction() * camera.ortho_factor();
                    let camera_pos = camera.position();

                    let new_pos = Vector3 { 
                        x: camera_pos.x + -camera_direction.x, 
                        y: camera_pos.y + -camera_direction.y, 
                        z: camera_pos.z + -camera_direction.z 
                    };
                    spawning_obj[0].set_position(new_pos);

                    let mut rotate = spawning_obj[0].rotation();
                    rotate.y = -(yaw + spawning_obj_angle);
                    spawning_obj[0].set_rotation(rotate);
                    spawning_obj[0].set_scale(spawning_obj_scale);
                }

                depth_sort_objects(&mut render_objects, &camera);

                shader_program.use_();
                shader_program.set_uniform_int("draw_mode", draw_mode as i32);
                view_port.draw(&shader_program, &light_shader_program, &mut camera, &render_objects, &light_objects);

                shader_program.use_();
                shader_program.set_uniform_int("draw_mode", draw_mode as i32);
                if is_spawn_test {
                    view_port.draw_selected_object(&shader_program,
                        &light_shader_program, &select_shader_program, 
                        &mut camera, &mut spawning_obj, &light_objects);
                }

                view_port.draw_frame_buffer(&frame_buffer_shader_program, &frame_buffer_mesh);
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}

fn to_draw_mode(value: u32) -> gl::types::GLenum {
    match value {
        0 => gl::FILL,
        1 => gl::FILL,
        2 => gl::LINE,
        3 => gl::POINT,
        _ => gl::FILL
    }

}

fn set_cullface_mode(value: u32) {
    unsafe {
        match value {
            0 => gl::Enable(gl::CULL_FACE),
            1 => gl::Enable(gl::CULL_FACE),
            2 => gl::Disable(gl::CULL_FACE),
            3 => gl::Disable(gl::CULL_FACE),
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

fn create_frame_buffer_mesh(frame_buffer: &FrameBuffer) -> Mesh {
    let mut vertices: Vec<Vertex> = vec![];
    let mut indices: Vec<u32> = vec![0, 3, 2,  0, 2, 1];

    vertices.push(
        Vertex {
            position: vec3(-1.0, 1.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
            tex_coords: vec2(0.0, 1.0),
        }
    );
    vertices.push(
        Vertex {
            position: vec3(1.0, 1.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
            tex_coords: vec2(1.0, 1.0),
        }
    );
    vertices.push(
        Vertex {
            position: vec3(1.0, -1.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
            tex_coords: vec2(1.0, 0.0),
        }
    );
    vertices.push(
        Vertex {
            position: vec3(-1.0, -1.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
            tex_coords: vec2(0.0, 0.0),
        }
    );

    let mut new_mesh = Mesh::from_vertices(vertices, indices);
    let mut new_material = objects::Material::new();
    new_material.diff_tex = Some(frame_buffer.color_buffer.clone());
    new_mesh.set_material(new_material);
    return new_mesh;
}

fn load_spawn_objects() -> Vec<RenderObject> {
    let mut container: Vec<RenderObject> = vec![];

    let model_path = Path::new("Models/GlassCube/Blue/Model.obj").to_str().unwrap();
    let rend_obj = obj_loader::load_model(model_path);
    container.push(rend_obj);

    let model_path = Path::new("Models/GlassCube/Red/Model.obj").to_str().unwrap();
    let rend_obj = obj_loader::load_model(model_path);
    container.push(rend_obj);

    let model_path = Path::new("Models/GlassCube/Green/Model.obj").to_str().unwrap();
    let rend_obj = obj_loader::load_model(model_path);
    container.push(rend_obj);

    let model_path = Path::new("Models/GlassCube/Purple/Model.obj").to_str().unwrap();
    let rend_obj = obj_loader::load_model(model_path);
    container.push(rend_obj);

    let model_path = Path::new("Models/GlassSphere/Blue/Model.obj").to_str().unwrap();
    let rend_obj = obj_loader::load_model(model_path);
    container.push(rend_obj);

    let model_path = Path::new("Models/GlassSphere/Red/Model.obj").to_str().unwrap();
    let rend_obj = obj_loader::load_model(model_path);
    container.push(rend_obj);

    let model_path = Path::new("Models/GlassSphere/Green/Model.obj").to_str().unwrap();
    let rend_obj = obj_loader::load_model(model_path);
    container.push(rend_obj);

    // let model_path = Path::new("Models/GlassSphere/Purple/Model.obj").to_str().unwrap();
    // let rend_obj = obj_loader::load_model(model_path);
    // container.push(rend_obj);

    let model_path = Path::new("Models/GlassWindow/Model.obj").to_str().unwrap();
    let rend_obj = obj_loader::load_model(model_path);
    container.push(rend_obj);

    let model_path = Path::new("Models/AmericanMuscle/Model.obj").to_str().unwrap();
    let rend_obj = obj_loader::load_model(model_path);
    container.push(rend_obj);

    return container;
}

fn depth_sort_objects(objects: &mut Vec<RenderObject>, camera: &Camera) {
    for i in 1..objects.len() {
        for j in (1..i + 1).rev() {
            let first_distance = get_vector_length(
                camera.position(), 
                objects[j - 1].position());
            let second_distance = get_vector_length(
                camera.position(), 
                objects[j].position());

            if first_distance >= second_distance { break; }
            objects.swap(j - 1, j);
        }
    }
}

fn get_vector_length(a: Vector3<f32>, b: Vector3<f32>) -> f32 {
    let x = (b.x - a.x).powi(2);
    let y = (b.y - a.y).powi(2);
    let z = (b.z - a.z).powi(2);
    (x + y + z).sqrt()
}
