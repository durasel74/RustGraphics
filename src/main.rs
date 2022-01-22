mod render_gl;
mod figures;
mod glwindow;

use gl;

use image::{ GenericImage, GenericImageView, ImageBuffer, RgbImage };

fn main() {
    // Запуск окна
    let mut gl_window = glwindow::GLWindow::from_parameters("RustGraphics", 800, 700);

    let figure: figures::Figure = figures::create_thor(30, 50, 20);

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
    





    let img = image::open("Pictures\\wall.jpg").unwrap();

    let img_size = img.dimensions();


    for (x, y, pixel) in img.pixels() {
        
    }

    // let pixels: Vec<u8> = img.pixels().collect();
    // match img.pixels()[0].2 {
    //     image::Rgba(values) => println!("{} {} {} {}", values[0], values[1], values[2], values[3]),
    // }

    // Первоначальная настройка пайплайна
    unsafe { 
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);
        gl::PointSize(3.0);

        

        let mut texture: u32 = 0;
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture); 

        // gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, img_size.0 as i32, 
        //     img_size.1 as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, data);
        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as i32);
    }

    // let borderColor = [ 1.0, 1.0, 0.0, 1.0 ];
    // gl::TexParameterfv(GL_TEXTURE_2D, GL_TEXTURE_BORDER_COLOR, borderColor); 

    // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
    // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    // gl::NEAREST_MIPMAP_NEAREST
    // gl::LINEAR_MIPMAP_NEAREST
    // gl::NEAREST_MIPMAP_LINEAR
    // gl::LINEAR_MIPMAP_LINEAR
    // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR);
    // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR);

    let texCoords = [
        0.0, 0.0,
        1.0, 0.0,
        0.5, 1.0,
    ];
    






    // Цикл отрисовки
    let mut is_running = true;
    while is_running {
        is_running = gl_window.event_check();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::PolygonMode(gl::FRONT_AND_BACK, to_draw_mode(gl_window.draw_mode));

            gl::BindVertexArray(figure.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, figure.ebo);

            if gl_window.draw_mode != 0 { shader_wire_program.run(); }
            else { shader_program.run(); }

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
