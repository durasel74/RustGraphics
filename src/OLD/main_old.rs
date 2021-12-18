pub mod render_gl;
use gl;
use sdl2;

pub fn remain() {
    // let gl_attr = video_subsystem.gl_attr();

    // gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
   // gl_attr.set_context_version(4, 1);

    // use std::ffi::CString;
    // let vert_shader =
    //     render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
    //         .unwrap();

    // let frag_shader =
    //     render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
    //         .unwrap();

    // let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    // // set up vertex buffer object

    // let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    // let mut vbo: gl::types::GLuint = 0;
    // unsafe {
    //     gl::GenBuffers(1, &mut vbo);
    // }

    // unsafe {
    //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,                                                       // target
    //         (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
    //         vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
    //         gl::STATIC_DRAW,                               // usage
    //     );
    //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    // }

    // // set up vertex array object

    // let mut vao: gl::types::GLuint = 0;
    // unsafe {
    //     gl::GenVertexArrays(1, &mut vao);
    // }

    // unsafe {
    //     gl::BindVertexArray(vao);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //     gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
    //     gl::VertexAttribPointer(
    //         0,         // index of the generic vertex attribute ("layout (location = 0)")
    //         3,         // the number of components per generic vertex attribute
    //         gl::FLOAT, // data type
    //         gl::FALSE, // normalized (int-to-float conversion)
    //         (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
    //         std::ptr::null(),                                     // offset of the first component
    //     );
    //     gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    //     gl::BindVertexArray(0);
    // }

    // set up shared state for window

    // unsafe {
    //     gl::Viewport(0, 0, 900, 700);
    // }

    // // main loop
    // let mut event_pump = sdl.event_pump().unwrap();
    // 'main: loop {
    //     // draw triangle
    //     shader_program.set_used();
    //     unsafe {
    //         gl::BindVertexArray(vao);
    //         gl::DrawArrays(
    //             gl::TRIANGLES, // mode
    //             0,             // starting index in the enabled arrays
    //             3,             // number of indices to be rendered
    //         );
    //     }
    // }
}
