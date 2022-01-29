mod render_gl;
mod figures;
mod glwindow;

use std::time;
use cgmath::{ Matrix, Matrix4, Rad };
use gl;

fn main() {
    // Запуск окна
    let mut gl_window = glwindow::GLWindow::from_parameters("RustGraphics", 800, 700);

    // Загрузка модели
    let figure: figures::Figure = figures::square_texture();

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
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);
        gl::PointSize(3.0);
    }

    
    let now = time::Instant::now();
    let mut rotate_matrix = Matrix4::from_angle_z(Rad(0.0));
    let mut scale_matrix = Matrix4::from_scale(1.0);


    // Цикл отрисовки
    let mut is_running = true;
    while is_running {
        is_running = gl_window.event_check();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::PolygonMode(gl::FRONT_AND_BACK, to_draw_mode(gl_window.draw_mode));

            gl::BindVertexArray(figure.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, figure.ebo);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1.id());
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2.id());

            
            let elapsed_time = now.elapsed();
            if ((elapsed_time.as_millis() as f32) / 500.0) == 0.0 { rotate_matrix = Matrix4::from_angle_z(Rad(0.1)) }
            let scale_value = ((elapsed_time.as_millis() as f32) / 500.0).sin().abs() + 0.2;
            scale_matrix = Matrix4::from_scale(scale_value);
            let matrix = &rotate_matrix * &scale_matrix;


            if gl_window.draw_mode != 0 { 
                shader_wire_program.run(); 
                shader_wire_program.set_uniform_matrix("transform", &matrix);
            }
            else {
                shader_program.run();
                shader_program.set_uniform_matrix("transform", &matrix);
                shader_program.set_uniform_int("texture1", 0);
                shader_program.set_uniform_int("texture2", 1);
            }

            gl::DrawElements(gl::TRIANGLES, figure.indices.len() as i32,
                gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);
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
