use gl;
use super::{ RenderObject, Mesh, ShaderProgram, Camera, Light, LightType };

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

    pub fn draw(&self, shader_program: &ShaderProgram, light_shader_program: &ShaderProgram,
    camera: &mut Camera, render_objects: &Vec<RenderObject>, light_objects: &Vec<Light>) {
        unsafe { gl::Viewport(self.position.0, self.position.1, self.size.0, self.size.1); }
        camera.set_view_size((self.size.0 as f32, self.size.1 as f32));
        self.configure_light(shader_program, light_objects);
        self.draw_render_objects(shader_program, camera, render_objects);
        self.draw_light_objects(light_shader_program, camera, light_objects);
    }

    fn draw_render_objects(&self, shader_program: &ShaderProgram, camera: &mut Camera, 
    render_objects: &Vec<RenderObject>) {
        shader_program.set_uniform_matrix4("view", &camera.view_matrix());
        shader_program.set_uniform_matrix4("projection", &camera.projection_matrix());

        for current_object in render_objects {
            shader_program.set_uniform_matrix4("model", &current_object.transform_matrix());
            shader_program.set_uniform_matrix3("normalMatrix", &current_object.normal_matrix(&camera.view_matrix()));

            let not_transperent_meshes = current_object.meshes().iter()
                .filter(|m| if m.material().dissolve == 1.0 { true } else { false } );
            let transperent_meshes = current_object.meshes().iter()
                .filter(|m| if m.material().dissolve < 1.0 { true } else { false } );
            for mesh in not_transperent_meshes {
                self.draw_mesh(shader_program, mesh);
            }
            for mesh in transperent_meshes {
                self.draw_mesh(shader_program, mesh);
            }
        }
    }

    fn draw_mesh(&self, shader_program: &ShaderProgram, mesh: &Mesh) {
        shader_program.set_uniform_vector3("material.ambient", &mesh.material().ambient);
        shader_program.set_uniform_vector3("material.diffuse", &mesh.material().diffuse);
        shader_program.set_uniform_vector3("material.specular", &mesh.material().specular);
        shader_program.set_uniform_float("material.shininess", mesh.material().specular_exponent);
        shader_program.set_uniform_float("material.dissolve", mesh.material().dissolve);

        unsafe {
            match &mesh.material().diff_tex {
                Some(texture) => {
                    shader_program.set_uniform_int("texture_diffuse", 0);
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, texture.id);
                },
                None => {
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, 0);
                }
            }
            match &mesh.material().spec_tex {
                Some(texture) => {
                    shader_program.set_uniform_int("texture_specular", 1);
                    gl::ActiveTexture(gl::TEXTURE1);
                    gl::BindTexture(gl::TEXTURE_2D, texture.id);
                },
                None => {
                    gl::ActiveTexture(gl::TEXTURE1);
                    gl::BindTexture(gl::TEXTURE_2D, 0);
                }
            }

            gl::BindVertexArray(mesh.render_data().vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.render_data().ebo);
            gl::DrawElements(gl::TRIANGLES, mesh.indices_count() as i32,
                gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);
        }
    }

    fn draw_light_objects(&self, light_shader_program: &ShaderProgram, 
    camera: &mut Camera, light_objects: &Vec<Light>) {
        light_shader_program.use_();
        light_shader_program.set_uniform_matrix4("view", &camera.view_matrix());
        light_shader_program.set_uniform_matrix4("projection", &camera.projection_matrix());

        for i in 0..light_objects.len() {
            let current_light = &light_objects[i];
            light_shader_program.set_uniform_matrix4("model", &current_light.transform_matrix());
            light_shader_program.set_uniform_vector3("lightColor", &current_light.specular());

            for mesh in current_light.meshes() {
                unsafe {
                    gl::BindVertexArray(mesh.render_data().vao);
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.render_data().ebo);
                    gl::DrawElements(gl::TRIANGLES, mesh.indices_count() as i32,
                        gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);
                }
            }
        }
    }

    fn configure_light(&self, shader_program: &ShaderProgram, light_objects: &Vec<Light>) {
        let mut dir_light_count = 0;
        let mut point_light_count = 0;
        let mut spot_light_count = 0;

        for i in 0..light_objects.len() {
            let current_light = &light_objects[i];
            let field_name: String;
            match current_light.light_type() {
                LightType::Directional => {
                    field_name = format!("dirLights[{}]", dir_light_count);
                    dir_light_count += 1;
                },
                LightType::Point => {
                    field_name = format!("pointLights[{}]", point_light_count);
                    point_light_count += 1;
                },
                LightType::Spotlight => {
                    field_name = format!("spotLights[{}]", spot_light_count);
                    spot_light_count += 1;
                }
            }
            current_light.configure_shader(shader_program, &field_name);
        }
        shader_program.set_uniform_int("dirLightCount", dir_light_count);
        shader_program.set_uniform_int("pointLightCount", point_light_count);
        shader_program.set_uniform_int("spotLightCount", spot_light_count);
    }

}
