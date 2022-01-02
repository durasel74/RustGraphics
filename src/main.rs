mod render_gl;

use sdl2;
use sdl2::event::WindowEvent;
use gl;

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

    use std::ffi::CString;
    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("Shaders\\triangle.vert")).unwrap()).unwrap();
    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("Shaders\\triangle.frag")).unwrap()).unwrap();
    let shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]).unwrap();

    // // Triangle
    // let vertices: Vec<f32> = vec![
    //     0.5, -0.5, 0.0,   0.8, 0.2, 0.8,
    //     -0.5, -0.5, 0.0,  0.8, 0.2, 1.0,
    //     0.0,  0.5, 0.0,   0.9, 0.2, 0.8,
    // ];

    // Square
    let vertices: Vec<f32> = vec![
        -0.5,  0.5, 0.0,   1.0, 1.0, 1.0,
        0.5, 0.5, 0.0,   0.9, 0.9, 0.9,
        0.5, -0.5, 0.0,   0.8, 0.8, 0.8,
        -0.5,  -0.5, 0.0,   0.9, 0.9, 0.9,
    ];

    // // Violet gradient
    // -0.5,  0.5, 0.0,   0.8, 0.2, 0.8,
    // -0.5, -0.5, 0.0,   0.9, 0.2, 0.8,
    // 0.5, -0.5, 0.0,   0.8, 0.2, 0.8,
    // 0.5,  0.5, 0.0,   0.8, 0.2, 1.0,

    let indices: Vec<u32> = vec![
        0, 1, 2,
        2, 3, 0,
    ]; 

    // // Herringbone
    // let vertices: Vec<f32> = vec![
    //     0.0, 0.0, 0.0,   0.0, 0.48, 0.1,
    //     -0.5, -0.5, 0.0,   0.0, 0.35, 0.1,
    //     0.5, -0.5, 0.0,   0.0, 0.35, 0.1,

    //     0.0, 0.4, 0.0,   0.0, 0.52, 0.1,
    //     -0.4, -0.1, 0.0,   0.0, 0.45, 0.1,
    //     0.4, -0.1, 0.0,   0.0, 0.45, 0.1,

    //     0.0, 0.7, 0.0,   0.1, 0.6, 0.2,
    //     -0.3, 0.3, 0.0,   0.0, 0.5, 0.1,
    //     0.3, 0.3, 0.0,   0.0, 0.5, 0.1,
    // ];

    let vbo = create_vbo(vertices);
    let ebo = create_ebo(indices);
    let vao = create_vao(vbo, ebo);

    unsafe { 
        gl::ClearColor(0.1, 0.1, 0.1, 1.0); 
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL)
    };
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut is_running = true;
    while is_running {
        is_running = event_check(&mut event_pump);

        let ok = 0 as gl::types::GLuint;

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); };
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BindVertexArray(vao);
            shader_program.run();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ok as *const gl::types::GLvoid);
        }
        window.gl_swap_window();
    }
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

fn create_vbo(vertices: Vec<f32>) -> u32 {
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

fn create_ebo(indices: Vec<u32>) -> u32 {
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

fn create_vao(vbo: u32, ebo: u32) -> u32 {
    let mut vao: gl::types::GLuint = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao); }
    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    return vao
}
