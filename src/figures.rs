use std::f32::consts::PI;
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
impl Drop for Figure {
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

pub fn square_texture() -> Figure {
    let mut new_figure = Figure {
        vertices: vec![
           -0.5,  0.5, 0.0,   1.0, 1.0, 1.0,   0.0, 1.0,
            0.5,  0.5, 0.0,   0.9, 0.9, 0.9,   1.0, 1.0,
            0.5, -0.5, 0.0,   0.8, 0.8, 0.8,   1.0, 0.0,
           -0.5, -0.5, 0.0,   0.9, 0.9, 0.9,   0.0, 0.0,
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

pub fn create_circle(vertex_count: u32, radius: u32) -> Figure {
    let normal_radius = (radius as f32) / 100.0;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut vertices: Vec<f32> = vec![x, y, 0.0,  0.4, 0.2, 0.7];
    let mut indices: Vec<u32> = vec![];
    
    let vertex_count_f32 = vertex_count as f32;
    let mut vertex_index = 1;
    for i in 0..vertex_count {
        let angle = (2.0 * PI * i as f32) / vertex_count_f32;
        x = normal_radius * angle.cos();
        y = normal_radius * angle.sin();
        let gradient = ((-x + y) / 2.0) + normal_radius;
        vec_push_range(&mut vertices, vec![x, y, 0.0,  0.4, 0.2, gradient + 0.2]);

        let mut next_index = (vertex_index + 1) % vertex_count;
        next_index = next_index + vertex_count * (next_index == 0) as u32;
        vec_push_range(&mut indices, vec![vertex_index, next_index, 0]);
        vertex_index += 1;
    }
    let mut new_figure = Figure { vertices, indices, vbo: 0, vao: 0, ebo: 0 };
    new_figure.create_vao();
    new_figure.create_ebo();
    return new_figure;
}

pub fn create_thor(vertex_count: u32, radius: u32, inner_radius: u32) -> Figure {
    let normal_radius = (radius as f32) / 100.0;
    let normal_inner_radius = (inner_radius as f32) / 100.0;
    let mut x: f32;
    let mut y: f32;
    let mut vertices: Vec<f32> = vec![];
    let mut indices: Vec<u32> = vec! [];

    let vertex_count_f32 = vertex_count as f32;
    let mut vertex_index = 0;
    for i in 0..vertex_count {
        let angle = (2.0 * PI * i as f32) / vertex_count_f32;
        x = normal_radius * angle.cos();
        y = normal_radius * angle.sin();
        let gradient = ((-x + y) / 2.0) + normal_radius;
        vec_push_range(&mut vertices, vec![x, y, 0.0,  0.2, gradient + 0.2, 1.0 - gradient / 2.0]);
        x = normal_inner_radius * angle.cos();
        y = normal_inner_radius * angle.sin();
        let gradient = ((-x + y) / 2.0) + normal_radius;
        vec_push_range(&mut vertices, vec![x, y, 0.0,  0.1, gradient + 0.1, 0.9 - gradient / 2.0]);

        let next_index = (vertex_index + 2) % (vertex_count * 2);
        vec_push_range(&mut indices, vec![vertex_index, next_index, next_index + 1]);
        vec_push_range(&mut indices, vec![vertex_index, vertex_index + 1, next_index + 1]);
        vertex_index += 2;
    }
    let mut new_figure = Figure { vertices, indices, vbo: 0, vao: 0, ebo: 0 };
    new_figure.create_vao();
    new_figure.create_ebo();
    return new_figure;
}

pub fn vec_push_range<T>(vect: &mut Vec<T>, values: Vec<T>) {
    for i in values { vect.push(i); }
}
