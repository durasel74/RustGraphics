mod render_gl;
mod figures;
mod glwindow;

use gl;

use image::{ GenericImage, GenericImageView, ImageBuffer, RgbImage };
use std::ffi;

fn main() {
    // Запуск окна
    let mut gl_window = glwindow::GLWindow::from_parameters("RustGraphics", 800, 700);

    let figure: figures::Figure = figures::square_texture();

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
    



    ///////////////////////////////////////////////////////////////////////////

    let img = image::open("Pictures\\container.jpg").unwrap();
    let img2 = image::open("Pictures\\awesomeface.png").unwrap();

    let img = img.flipv();
    let img2 = img2.flipv();
    
    let img_size = img.dimensions();
    let img2_size = img2.dimensions();

    let mut data = vec![];
    for (x, y, pixel) in img.pixels() {
        match pixel {
            image::Rgba(values) => { 
                data.push(values[0]);
                data.push(values[1]);
                data.push(values[2]);
            }
        }
    }

    let mut data2 = vec![];
    for (x, y, pixel) in img2.pixels() {
        match pixel {
            image::Rgba(values) => { 
                data2.push(values[0]);
                data2.push(values[1]);
                data2.push(values[2]);
                data2.push(values[3]);
            }
        }
    }

    // Первоначальная настройка пайплайна
    let mut texture1: u32 = 0;
    let mut texture2: u32 = 0;
    unsafe { 
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);
        gl::PointSize(3.0);

        gl::GenTextures(1, &mut texture1);
        gl::BindTexture(gl::TEXTURE_2D, texture1);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, img_size.0 as i32, 
            img_size.1 as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, 
            data.as_ptr() as *const gl::types::GLvoid);
        gl::GenerateMipmap(gl::TEXTURE_2D);



        gl::GenTextures(1, &mut texture2);
        gl::BindTexture(gl::TEXTURE_2D, texture2);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, img2_size.0 as i32, 
            img2_size.1 as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, 
            data2.as_ptr() as *const gl::types::GLvoid);
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    ///////////////////////////////////////////////////////////////////////////


    // Цикл отрисовки
    let mut is_running = true;
    while is_running {
        is_running = gl_window.event_check();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::PolygonMode(gl::FRONT_AND_BACK, to_draw_mode(gl_window.draw_mode));

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            gl::BindVertexArray(figure.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, figure.ebo);

            if gl_window.draw_mode != 0 { shader_wire_program.run(); }
            else { 
                shader_program.run(); 
                gl::Uniform1i(gl::GetUniformLocation(shader_program.id(), 
                    ffi::CString::new("texture1").unwrap().as_ptr()), 0);
                gl::Uniform1i(gl::GetUniformLocation(shader_program.id(), 
                    ffi::CString::new("texture2").unwrap().as_ptr()), 1);
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
