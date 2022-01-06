use gl;
use gl::types::{ GLint, GLuint, GLsizeiptr, GLvoid };

pub struct Figure {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub vbo: GLuint,
    pub vao: GLuint,
    pub ebo: GLuint,
}
impl Figure {
    pub fn create_vao(&mut self) {
        self.create_vbo();
        let mut vao: GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao); }
        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

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
        self.vao = vao;
    }

    pub fn create_ebo(&mut self) {
        let mut ebo: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut ebo); }
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
                self.indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
        self.ebo = ebo;
    }

    fn create_vbo(&mut self) {
        let mut vbo: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut vbo); }
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                self.vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        self.vbo = vbo;
    }
}

pub fn triangle90() -> Figure {
    let mut new_figure = Figure {
        vertices: vec![
            -0.55, 0.45, 0.0,   0.8, 0.2, 0.8,
            0.45, -0.55, 0.0,   0.8, 0.2, 1.0,
            -0.55, -0.55, 0.0,  0.9, 0.2, 0.8,
        ],
        indices: vec![
            0, 1, 2,
        ],
        vbo: 0,
        vao: 0,
        ebo: 0,
    };
    new_figure.create_vao();
    new_figure.create_ebo();
    return new_figure;
}

pub fn triangle90alter() -> Figure {
    let mut new_figure = Figure {
        vertices: vec![
            -0.45, 0.55, 0.0,  0.2, 0.8, 0.8,
            0.55, 0.55, 0.0,   0.2, 1.0, 0.8,
            0.55, -0.45, 0.0,  0.2, 0.9, 0.8,
        ],
        indices: vec![
            0, 1, 2,
        ],
        vbo: 0,
        vao: 0,
        ebo: 0,
    };
    new_figure.create_vao();
    new_figure.create_ebo();
    return new_figure;
}

pub fn triangle() -> Figure {
    let mut new_figure = Figure {
        vertices: vec![
            0.5, -0.5, 0.0,   0.8, 0.2, 0.8,
            -0.5, -0.5, 0.0,  0.8, 0.2, 1.0,
            0.0,  0.5, 0.0,   0.9, 0.2, 0.8,
        ],
        indices: vec![
            0, 1, 2,
        ],
        vbo: 0,
        vao: 0,
        ebo: 0,
    };
    new_figure.create_vao();
    new_figure.create_ebo();
    return new_figure;
}

pub fn square() -> Figure {
    let mut new_figure = Figure {
        vertices: vec![
            -0.5,  0.5, 0.0,   1.0, 1.0, 1.0,
            0.5, 0.5, 0.0,     0.9, 0.9, 0.9,
            0.5, -0.5, 0.0,    0.8, 0.8, 0.8,
            -0.5,  -0.5, 0.0,  0.9, 0.9, 0.9,
        ],
        indices: vec![
            0, 1, 2,
            2, 3, 0,
        ],
        vbo: 0,
        vao: 0,
        ebo: 0,
    };
    new_figure.create_vao();
    new_figure.create_ebo();
    return new_figure;
}
