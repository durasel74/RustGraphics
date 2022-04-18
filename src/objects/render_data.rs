use gl;
use gl::types::{ GLint, GLuint, GLsizeiptr, GLvoid };
use super::Vertex;

#[derive(Clone)]
pub struct RenderData {
    pub vbo: GLuint,
    pub vao: GLuint,
    pub ebo: GLuint,
}
impl RenderData {
    pub fn from_verteices(vertices: &Vec<Vertex>, indices: &Vec<u16>) -> Self {
        let vbo = Self::create_vbo(vertices);
        let vao = Self::create_vao(vbo);
        let ebo = Self::create_ebo(indices);
        RenderData { vbo, vao, ebo }
    }

    pub fn create_vbo(vertices: &Vec<Vertex>) -> GLuint {
        let mut vbo: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut vbo); }
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
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
                (8 * std::mem::size_of::<f32>()) as GLint,
                std::ptr::null()
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE,
                (8 * std::mem::size_of::<f32>()) as GLint,
                (3 * std::mem::size_of::<f32>()) as *const GLvoid,
            );
            gl::EnableVertexAttribArray(2); 
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 
                (8 * std::mem::size_of::<f32>()) as GLint, 
                (6 * std::mem::size_of::<f32>()) as *const GLvoid
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        vao
    }

    pub fn create_ebo(indices: &Vec<u16>) -> GLuint {
        let mut ebo: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut ebo); }
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u16>()) as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
        ebo
    }
}
impl Drop for RenderData {
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteVertexArrays(1, &self.vao);
         }
    }
}
