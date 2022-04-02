use cgmath::{ Matrix, SquareMatrix, Matrix3, Matrix4, Vector3, vec3, Rad };
use super::{ Mesh };

pub struct Light {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: f32,

    ambient: Vector3<f32>,
    diffuse: Vector3<f32>,
    specular: Vector3<f32>,

    mesh: Option<Mesh>,
}
impl Light {
    pub fn new() -> Self {
        Light {
            position: vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0, 0.0, 0.0),
            scale: 1.0,

            ambient: vec3(0.0, 0.0, 0.0),
            diffuse: vec3(0.0, 0.0, 0.0),
            specular: vec3(0.0, 0.0, 0.0),

            mesh: None,
        }
    }

    pub fn position(&self) -> Vector3<f32> { self.position }
    pub fn set_position(&mut self, value: Vector3<f32>) { self.position = value }
    pub fn rotation(&self) -> Vector3<f32> { self.rotation }
    pub fn set_rotation(&mut self, value: Vector3<f32>) { self.rotation = value } 
    pub fn scale(&self) -> f32 { self.scale }
    pub fn set_scale(&mut self, value: f32) { self.scale = value }

    pub fn ambient(&self) -> Vector3<f32> { self.ambient }
    pub fn set_ambient(&mut self, value: Vector3<f32>) { self.ambient = value; }
    pub fn diffuse(&self) -> Vector3<f32> { self.diffuse }
    pub fn set_diffuse(&mut self, value: Vector3<f32>) { self.diffuse = value; }
    pub fn specular(&self) -> Vector3<f32> { self.specular }
    pub fn set_specular(&mut self, value: Vector3<f32>) { self.specular = value; }

    pub fn mesh(&self) -> &Option<Mesh> { &self.mesh }
    pub fn set_mesh(&mut self, value: Mesh) { self.mesh = Some(value); }

    pub fn transform_matrix(&self) -> Matrix4<f32> {
        let pos_matrix = Matrix4::from_translation(self.position.clone());
        let rot_matrix = Matrix4::from_angle_x(Rad(self.rotation.x.to_radians())) * 
            Matrix4::from_angle_y(Rad(self.rotation.y.to_radians())) *
            Matrix4::from_angle_z(Rad(self.rotation.z.to_radians()));
        let sca_matrix = Matrix4::from_scale(self.scale.clone());
        pos_matrix * rot_matrix * sca_matrix
    }
}
