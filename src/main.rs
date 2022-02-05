mod render_gl;
mod glwindow;

use std::time;
use cgmath::{ Rad, Matrix, Matrix4, vec3, PerspectiveFov };
use gl;
use render_gl::Mesh;
use render_gl::figures;

fn main() {
    let window_width = 800.0;
    let window_height = 700.0;

    // Запуск окна
    let mut gl_window = glwindow::GLWindow::from_parameters("RustGraphics", 
        window_width as u32, window_height as u32);

    // Загрузка модели
    let mesh: Mesh = figures::cube();

    // Загрузка текстур
    let texture_loadresult = render_gl::Texture::from_file("Pictures\\container.jpg");
    let texture1 = match texture_loadresult {
        Ok(texture) => texture,
        Err(err) => { println!("{}", err); return }
    };

    let texture_loadresult = render_gl::Texture::from_file("Pictures\\awesomeface.png");
    let texture2 = match texture_loadresult {
        Ok(texture) => texture,
        Err(err) => { println!("{}", err); return }
    };

    // Пути к файлам шейдеров
    let vert_filename = "Shaders\\triangles.vert";
    let frag_filename = "Shaders\\triangles.frag";
    let frag_wire_filename = "Shaders\\triangles_wire.frag";

    // Загрузка и компиляция шейдеров
    let shader_loadresult = render_gl::ShaderProgram::from_files(
        vert_filename, frag_filename);
    let shader_program = match shader_loadresult {
        Ok(program) => program,
        Err(err) => { println!("{}", err); return }
    };

    let shader_loadresult = render_gl::ShaderProgram::from_files(
        vert_filename, frag_wire_filename);
    let shader_wire_program = match shader_loadresult {
        Ok(program) => program,
        Err(err) => { println!("{}", err); return }
    };

    // Первоначальная настройка пайплайна
    unsafe { 
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::PointSize(3.0);
        gl::Enable(gl::DEPTH_TEST);  
    }



    let now = time::Instant::now();

    let field_of_view = 45.0f32;

    let mut model_matrix = Matrix4::from_translation(vec3(0.0, 0.0, 0.0));

    let mut view_matrix = Matrix4::from_translation(vec3(0.0, 0.0, -3.0));

    let projection_matrix = Matrix4::from(PerspectiveFov { 
        fovy: Rad(field_of_view.to_radians()),
        aspect: window_width / window_height, 
        near: 0.1,
        far: 100.0
    });

    let cube_positions = vec![ 
        vec3( 0.0,  0.0,  0.0),
        vec3( 2.0,  5.0, -15.0),
        vec3(-1.5, -2.2, -2.5),
        vec3(-3.8, -2.0, 12.3),
        vec3( 2.4, -0.4, -3.5),
        vec3(-1.7,  3.0, -7.5),
        vec3( 1.3, -2.0, 2.5),
        vec3( 1.5,  2.0, -2.5),
        vec3( 1.5,  0.2, 1.5),
        vec3(-1.3,  0.0, -1.5),
    ];

    
    // Цикл отрисовки
    let mut is_running = true;
    while is_running {
        is_running = gl_window.event_check();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::PolygonMode(gl::FRONT_AND_BACK, to_draw_mode(gl_window.draw_mode));

            gl::BindVertexArray(mesh.render_data().vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.render_data().ebo);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1.id());
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2.id());

            let elapsed_time = now.elapsed();
            let rotate_value = (elapsed_time.as_millis() as f32) / 999.0;
            view_matrix = Matrix4::from_translation(vec3(0.0, 0.0, -6.0)) * Matrix4::from_angle_y(Rad(-rotate_value));

            for i in 0..10 {
                model_matrix = Matrix4::from_translation(cube_positions[i].clone());
                let angle = 20.0 * i as f32;
                model_matrix = model_matrix * Matrix4::from_axis_angle(
                    vec3(1.0, 0.3, 0.5), Rad(angle.to_radians()));

                if gl_window.draw_mode != 0 { 
                    shader_wire_program.run(); 
                    shader_wire_program.set_uniform_matrix("model", &model_matrix);
                    shader_wire_program.set_uniform_matrix("view", &view_matrix);
                    shader_wire_program.set_uniform_matrix("projection", &projection_matrix);
                }
                else {
                    shader_program.run();
                    shader_program.set_uniform_matrix("model", &model_matrix);
                    shader_program.set_uniform_matrix("view", &view_matrix);
                    shader_program.set_uniform_matrix("projection", &projection_matrix);

                    shader_program.set_uniform_int("texture1", 0);
                    shader_program.set_uniform_int("texture2", 1);
                }
                gl::DrawElements(gl::TRIANGLES, mesh.indices().len() as i32,
                    gl::UNSIGNED_SHORT, 0 as *const gl::types::GLvoid);
            }
        }
        gl_window.update();
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
