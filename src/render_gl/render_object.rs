use cgmath::{ Matrix4, Vector3, vec3, Rad };
use gl;
use super::Mesh;

pub struct RenderObject<'a> {
    mesh: &'a Mesh,
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: f32,
}
impl<'a> RenderObject<'a> {
    pub fn from_mesh(mesh: &'a Mesh) -> Self {
        let position = vec3(0.0, 0.0, 0.0);
        let rotation = vec3(0.0, 0.0, 0.0);
        let scale = 1.0;
        RenderObject { mesh, position, rotation, scale }
    }

    pub fn mesh(&self) -> &Mesh { self.mesh }
    pub fn position(&self) -> Vector3<f32> { self.position }
    pub fn set_position(&mut self, value: Vector3<f32>) { self.position = value }
    pub fn rotation(&self) -> Vector3<f32> { self.rotation }
    pub fn set_rotation(&mut self, value: Vector3<f32>) { self.rotation = value } 
    pub fn scale(&self) -> f32 { self.scale }
    pub fn set_scale(&mut self, value: f32) { self.scale = value }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.mesh.render_data().vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.mesh.render_data().ebo);
        }
    }

    pub fn transform_matrix(&self) -> Matrix4<f32> {
        let pos_matrix = Matrix4::from_translation(self.position.clone());
        let rot_matrix = Matrix4::from_angle_x(Rad(self.rotation.x.to_radians())) * 
            Matrix4::from_angle_y(Rad(self.rotation.y.to_radians())) *
            Matrix4::from_angle_z(Rad(self.rotation.z.to_radians()));
        let sca_matrix = Matrix4::from_scale(self.scale.clone());
        pos_matrix * rot_matrix * sca_matrix
    }
}