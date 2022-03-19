use gl;
use gl::types::{ GLint, GLuint, GLsizeiptr, GLvoid };

#[derive(Clone)]
pub struct RenderDataNormal {
    pub vbo: GLuint,
    pub vao: GLuint,
}
impl RenderDataNormal {
    pub fn from_verteices(vertices: &Vec<f32>) -> Self {
        let vbo = Self::create_vbo(vertices);
        let vao = Self::create_vao(vbo);
        RenderDataNormal { vbo, vao }
    }

    pub fn create_vbo(vertices: &Vec<f32>) -> GLuint {
        let mut vbo: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut vbo); }
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        vbo
    }

    pub fn create_vao(vbo: GLuint) -> GLuint {
        let mut vao: GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao); }
        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as GLint,
                std::ptr::null()
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as GLint,
                (3 * std::mem::size_of::<f32>()) as *const GLvoid,
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        vao
    }
}
impl Drop for RenderDataNormal {
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
         }
    }
}
