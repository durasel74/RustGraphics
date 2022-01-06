mod render_gl;
mod figures;

use std::ffi;
use sdl2;
use sdl2::event::WindowEvent;
use gl;
use std::time::Instant;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("RustGraphics", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem
        .gl_get_proc_address(s) as *const std::os::raw::c_void);

    

    let figure: figures::Figure = figures::triangle90();
    let vbo = create_vbo(&figure.vertices);
    let ebo = create_ebo(&figure.indices);
    let vao = create_vao_position(vbo);

    let figure2: figures::Figure = figures::triangle90alter();
    let vbo2 = create_vbo(&figure2.vertices);
    let ebo2 = create_ebo(&figure2.indices);
    let vao2 = create_vao_position(vbo2);


    // Done
    let vert_filename = "Shaders\\triangles.vert";
    let frag1_filename = "Shaders\\first_triangle.frag";
    let frag2_filename = "Shaders\\second_triangle.frag";

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
    //

        
    let uniform_name = ffi::CString::new("ourColor").unwrap();
    let vertex_color_location = unsafe{ gl::GetUniformLocation(shader_program.id(), 
        uniform_name.as_ptr() as *const gl::types::GLchar) };
    
    let mut figure1color = figures::RGBA::new(0.4, 0.4, 1.0, 1.0);
    
    let now_time = Instant::now();



    unsafe { 
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    }
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut is_running = true;
    while is_running {
        is_running = event_check(&mut event_pump);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BindVertexArray(vao);
            shader_program.run();

            let time = (now_time.elapsed().as_millis() as f32) / 500.0;
            figure1color.r = time.sin().abs();
            figure1color.g = time.sin().abs();
            figure1color.b = time.sin().abs();
            gl::Uniform4f(vertex_color_location, figure1color.r, figure1color.g, 
                figure1color.b, figure1color.a);

            gl::DrawElements(gl::TRIANGLES, figure.indices.len() as i32, 
                gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo2);
            gl::BindVertexArray(vao2);
            shader_program2.run();
            gl::DrawElements(gl::TRIANGLES, figure2.indices.len() as i32, 
                gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);
        }
        window.gl_swap_window();
    }
    unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); }
}

fn event_check(event_pump: &mut sdl2::EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            sdl2::event::Event::Quit {..} => return false,
            sdl2::event::Event::KeyUp { 
                keycode: Some(sdl2::keyboard::Keycode::Escape), ..} => 
                return false,
            sdl2::event::Event::Window { 
                win_event: WindowEvent::Resized(width, height), ..} => 
                update_viewport(width, height),
            _ => (),
        }
    }
    return true;
}

fn update_viewport(width: i32, height: i32) {
    unsafe { gl::Viewport(0, 0, width, height); };
}

fn create_vbo(vertices: &Vec<f32>) -> u32 {
    let mut vbo: gl::types::GLuint = 0;
    unsafe { gl::GenBuffers(1, &mut vbo); }
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    return vbo
}

fn create_ebo(indices: &Vec<u32>) -> u32 {
    let mut ebo: gl::types::GLuint = 0;
    unsafe { gl::GenBuffers(1, &mut ebo); }
    unsafe {
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }
    return ebo
}

fn create_vao_position(vbo: u32) -> u32 {
    let mut vao: gl::types::GLuint = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao); }
    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    return vao
}
