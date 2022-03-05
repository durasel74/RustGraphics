use gl;
use super::ShaderProgram;
use super::Camera;
use super::RenderObject;

pub struct ViewPort {
    position: (i32, i32),
    size: (i32, i32),
}

impl ViewPort {
    pub fn new() -> Self {
        ViewPort {
            position: (0, 0),
            size: (0, 0),
        }
    }

    pub fn position(&self) -> &(i32, i32) { &self.position }
    pub fn set_position(&mut self, value: (i32, i32)) { self.position = value; }

    pub fn size(&self) -> &(i32, i32) { &self.size }
    pub fn set_size(&mut self, value: (i32, i32)) { self.size = value; }

    pub fn draw(&self, shader_program: &ShaderProgram, camera: &mut Camera, 
    render_objects: &Vec<RenderObject>) {
        unsafe {gl::Viewport(self.position.0, self.position.1, self.size.0, self.size.1); }
        camera.set_view_size((self.size.0 as f32, self.size.1 as f32));
        shader_program.set_uniform_matrix("view", &camera.view_matrix());
        shader_program.set_uniform_matrix("projection", &camera.projection_matrix());

        for i in 0..render_objects.len() {
            let current_object = &render_objects[i];
            current_object.bind();
            shader_program.set_uniform_matrix("model", &current_object.transform_matrix());
            unsafe {
                gl::DrawElements(gl::TRIANGLES, current_object.mesh().indices().len() as i32,
                    gl::UNSIGNED_SHORT, 0 as *const gl::types::GLvoid);
            }
        }
    }
}
