mod render_gl;

use sdl2;
use gl;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);

    let window = video_subsystem
        .window("RustGraphics", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem
        .gl_get_proc_address(s) as *const std::os::raw::c_void);

    use std::ffi::CString;
    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("Shaders\\triangle.vert")).unwrap()).unwrap();
    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("Shaders\\triangle.frag")).unwrap()).unwrap();
    let shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]).unwrap();

    let vertices: Vec<f32> = vec![
        0.5, -0.5, 0.0,   0.8, 0.2, 0.8,
        -0.5, -0.5, 0.0,  0.8, 0.2, 1.0,
        0.0,  0.5, 0.0,   0.9, 0.2, 0.8,
    ];

    let vbo = create_vbo(vertices);
    let vao = create_vao(vbo);

    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.4, 0.7, 0.8, 1.0);
    };

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => (),
            }
        }

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); };
        shader_program.run();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        
        window.gl_swap_window();
    }
}

fn create_vbo(vertices: Vec<f32>) -> u32{
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

fn create_vao(vbo: u32) -> u32 {
    let mut vao: gl::types::GLuint = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao); }
    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    return vao
}
