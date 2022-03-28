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
        let view_matrix = &camera.view_matrix();
        shader_program.set_uniform_matrix4("view", view_matrix);
        shader_program.set_uniform_matrix4("projection", &camera.projection_matrix());

        for i in 0..render_objects.len() {
            let current_object = &render_objects[i];
            shader_program.set_uniform_matrix4("model", &current_object.transform_matrix());
            shader_program.set_uniform_matrix3("normalMatrix", &current_object.normal_matrix(view_matrix));

            shader_program.set_uniform_vector("material.ambient", &current_object.material().ambient);
            shader_program.set_uniform_vector("material.diffuse", &current_object.material().diffuse);
            shader_program.set_uniform_vector("material.specular", &current_object.material().specular);
            shader_program.set_uniform_float("material.shininess", current_object.shininess());
            // unsafe {
            //     gl::DrawElements(gl::TRIANGLES, current_object.mesh().indices().len() as i32,
            //         gl::UNSIGNED_SHORT, 0 as *const gl::types::GLvoid);
            // }

            unsafe {
                gl::BindVertexArray(current_object.mesh().render_data().vao);
                //gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, current_object.mesh().render_data().ebo);
                gl::DrawArrays(gl::TRIANGLES, 0, (current_object.mesh().vertices().len() / 6) as i32);
            }
        }
    }
}
