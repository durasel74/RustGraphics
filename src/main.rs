mod render_gl;
mod figures;
mod glwindow;

use gl;

fn main() {
    // Запуск окна
    let mut gl_window = glwindow::GLWindow::from_parameters("RustGraphics", 700, 600);

    // let figure: figures::Figure = figures::create_circle(6, 50);

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
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    }

    let mut radius = 55;
    let mut vertices_count = 6;
    let mut inner_radius = 20;
    
    // Цикл отрисовки
    let mut is_running = true;
    while is_running {
        is_running = gl_window.event_check();

        vertices_count += gl_window.arrowv_event;
        if vertices_count < 0 { vertices_count = 0; }
        gl_window.arrowv_event = 0;

        inner_radius += gl_window.arrowh_event;
        if inner_radius < 0 { inner_radius = 0; }
        if inner_radius > radius { inner_radius = radius; }
        gl_window.arrowh_event = 0;

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::PolygonMode(gl::FRONT_AND_BACK, to_draw_mode(gl_window.draw_mode));

            // let figure: figures::Figure = figures::create_circle(vertices_count as u32, 55);
            let figure: figures::Figure = figures::create_thor(vertices_count as u32, 
                radius as u32, inner_radius as u32);

            gl::BindVertexArray(figure.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, figure.ebo);

            if gl_window.draw_mode != 0 { shader_wire_program.run(); }
            else { shader_program.run(); }

            gl::DrawElements(gl::TRIANGLES, figure.indices.len() as i32,
                gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);
        }
        gl_window.update();
    }
    unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); }
}

fn to_draw_mode(value: u32) -> gl::types::GLenum {
    match value {
        0 => gl::FILL,
        1 => gl::LINE,
        2 => gl::POINT,
        _ => gl::FILL
    }
}
