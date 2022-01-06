mod render_gl;
mod figures;
mod glwindow;

use gl;

fn main() {
    // Запуск окна
    let mut gl_window = glwindow::GLWindow::from_parameters("RustGraphics", 800, 600);

    // Загрузка моделей
    let figure: figures::Figure = figures::triangle90();
    let figure2: figures::Figure = figures::triangle90alter();

    // Пути к файлам шейдеров
    let vert_filename = "Shaders\\triangles.vert";
    let frag1_filename = "Shaders\\first_triangle.frag";
    let frag2_filename = "Shaders\\second_triangle.frag";

    // Загрузка и компиляция шейдеров
    let shader_loadresult = render_gl::ShaderProgram::from_files(
        vert_filename, frag1_filename);
    let shader_program = match shader_loadresult {
        Ok(program) => program,
        Err(err) => { println!("{}", err); return }
    };

    let shader_loadresult = render_gl::ShaderProgram::from_files(
        vert_filename, frag2_filename);
    let shader_program2 = match shader_loadresult {
        Ok(program) => program,
        Err(err) => { println!("{}", err); return }
    };
    
    // Первоначальная настройка пайплайна
    unsafe { 
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    }
    
    // Цикл отрисовки
    let mut is_running = true;
    while is_running {
        is_running = gl_window.event_check();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(figure.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, figure.ebo);
            shader_program.run();
            gl::DrawElements(gl::TRIANGLES, figure.indices.len() as i32, 
                gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);

            gl::BindVertexArray(figure2.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, figure2.ebo);
            shader_program2.run();
            gl::DrawElements(gl::TRIANGLES, figure2.indices.len() as i32, 
                gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);
        }
        gl_window.update();
    }
    unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); }
}
