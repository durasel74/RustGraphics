mod render_objects;

use gl;

use std::time;
use rand::Rng;
use cgmath::prelude::InnerSpace;
use cgmath::{ Rad, Matrix, Matrix4, Vector3, vec3, PerspectiveFov, Ortho };
use render_objects::{ Mesh, RenderObject, Camera, figures };

use glutin;
use glutin::window;
use glutin::event;
use glutin::event_loop;
use glutin::dpi;

fn main() {
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

    let gl_context = windowed_context.context();
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    // let version = unsafe {
    //     let data = ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const _).to_bytes().to_vec();
    //     String::from_utf8(data).unwrap()
    // };
    // println!("OpenGL version {}", version);

    // Self::set_event_handler(event_loop);



    // Загрузка модели
    let mesh: Mesh = figures::cube();

    // Загрузка текстур
    // let texture_loadresult = render_gl::Texture::from_file("Pictures\\container.jpg");
    // let texture1 = match texture_loadresult {
    //     Ok(texture) => texture,
    //     Err(err) => { println!("{}", err); return }
    // };

    // Пути к файлам шейдеров
    let vert_filename = "Shaders\\triangles.vert";
    let frag_filename = "Shaders\\triangles.frag";

    // Загрузка и компиляция шейдеров
    let shader_loadresult = render_objects::ShaderProgram::from_files(
        vert_filename, frag_filename);
    let shader_program = match shader_loadresult {
        Ok(program) => program,
        Err(err) => { println!("{}", err); return }
    };


    let mut render_objects: Vec<RenderObject> = vec![];
    let mut rng = rand::thread_rng();
    let mut generator = || -> f32 { (rng.gen_range(-1000..1000) as f32) / 10.0 };
    for i in 1..1000 {
        let mesh: Mesh = figures::cube();
        let mut new_object = RenderObject::from_mesh(mesh);
        new_object.set_position(vec3(generator(), generator(), generator()));
        render_objects.push(new_object);
    }
    let mesh: Mesh = figures::cube();
    render_objects.push(RenderObject::from_mesh(mesh));

    let mut projection_matrix: Matrix4<f32> = Matrix4::from_scale(1.0);
    let mut camera = Camera::new();
    camera.set_position(vec3(0.0, 0.0, 3.0));
    
    let field_of_view = 70.0f32;
    let speed = 0.5;
    let sensitivity = 0.1;

    let mut yaw = 0.0f32;
    let mut pitch = 0.0f32;
    let mut last_x = 0.0;
    let mut last_y = 0.0;

    let now = time::Instant::now();
    
    // Первоначальная настройка пайплайна
    unsafe { 
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::PointSize(3.0);
        gl::Enable(gl::DEPTH_TEST);  
    }

    let mut draw_mode = 0;


    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = event_loop::ControlFlow::Wait;

        match event {
            event::Event::LoopDestroyed => return,
            event::Event::WindowEvent { event, .. } =>
                window_event_handler(event, control_flow),
            event::Event::MainEventsCleared => {
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    gl::PolygonMode(gl::FRONT_AND_BACK, to_draw_mode(draw_mode));
                }

                let aspect_width = windowed_context.window().inner_size().width as f32;
                let mut aspect_height = windowed_context.window().inner_size().height as f32;
                let mut aspect: f32;
                if aspect_height == 0.0 { aspect_height = 1.0 }
                if aspect_height > aspect_width { aspect = aspect_height / aspect_width }
                else { aspect = aspect_width / aspect_height }
                projection_matrix = Matrix4::from(PerspectiveFov { 
                    fovy: Rad(field_of_view.to_radians()),
                    aspect, 
                    near: 0.1,
                    far: 200.0
                });

                // projection_matrix = Matrix4::from(Ortho {
                //     left: -aspect_width / 5.0,
                //     right: aspect_width / 5.0,
                //     bottom: -aspect_height / 5.0,
                //     top: aspect_height / 5.0,
                //     near: 0.1,
                //     far: 200.0,
                // });

                // gl::BindVertexArray(self.mesh.render_data().vao);
                // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.mesh.render_data().ebo);
                // gl::ActiveTexture(gl::TEXTURE0);
                // gl::BindTexture(gl::TEXTURE_2D, texture1.id());

                // let elapsed_time = now.elapsed();
                // let rotate_value = (elapsed_time.as_millis() as f32) / 999.0;
                
                // let camx = rotate_value.sin() * radius;
                // let camy = rotate_value.cos() * radius;
                // camera.set_position(vec3(camx, 0.0, camy));



                // let mut offset_x = gl_window.delta_x * sensitivity;
                // let mut offset_y = gl_window.delta_y * sensitivity;
                // // last_x = gl_window.delta_x as f32;
                // // last_y = gl_window.delta_y as f32;

                // yaw += offset_x;
                // pitch += offset_y;
                // if (pitch > 89.0) { pitch = 89.0; }
                // if (pitch < -89.0) { pitch = -89.0; }

                let radians_yaw = yaw.to_radians();
                let radians_pitch = pitch.to_radians();
                let direct_x = (radians_yaw * radians_pitch.cos()).cos();
                let direct_y = radians_pitch.sin();
                let direct_z = (radians_yaw * radians_pitch.cos()).sin();
                let direction = vec3(direct_x, direct_y, direct_z).normalize();
                camera.set_direction(direction);


                // if window.arrow_h > 0 {
                //     let matrix = Matrix4::from_translation(camera.right() * speed);
                //     camera.set_target((matrix * camera.target().extend(1.0)).truncate());
                //     camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                //     window.arrow_h = 0;
                // }
                // else if window.arrow_h < 0 {
                //     let matrix = Matrix4::from_translation(-camera.right() * speed);
                //     camera.set_target((matrix * camera.target().extend(1.0)).truncate());
                //     camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                //     window.arrow_h = 0;
                // }
                // if window.arrow_v > 0 {
                //     let matrix = Matrix4::from_translation(-camera.direction() * speed);
                //     camera.set_target((matrix * camera.target().extend(1.0)).truncate());
                //     camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                //     window.arrow_v = 0;
                // }
                // else if window.arrow_v < 0 {
                //     let matrix = Matrix4::from_translation(camera.direction() * speed);
                //     camera.set_target((matrix * camera.target().extend(1.0)).truncate());
                //     camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                //     window.arrow_v = 0;
                // }

                for i in 0..render_objects.len() {
                    let current_object = &render_objects[i];
                    current_object.bind();

                    shader_program.run();
                    shader_program.set_uniform_matrix("model", &current_object.transform_matrix());
                    shader_program.set_uniform_matrix("view", &camera.lookat_matrix());
                    shader_program.set_uniform_matrix("projection", &projection_matrix);
                    // shader_program.set_uniform_int("texture1", 0);

                    if draw_mode == 0 { shader_program.set_uniform_int("wire_mode", 0); }
                    else { shader_program.set_uniform_int("wire_mode", 1); }

                    unsafe {
                        gl::DrawElements(gl::TRIANGLES, current_object.mesh().indices().len() as i32,
                            gl::UNSIGNED_SHORT, 0 as *const gl::types::GLvoid);
                    }
                }
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
            gl::Viewport(0, 0, physical_size.width as i32, physical_size.height as i32); },
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
