use cgmath::{ Matrix, SquareMatrix, Matrix3, Matrix4, Vector3, vec3, Rad };
use super::{ Mesh };

#[derive(Clone)]
pub struct RenderObject {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: f32,
    meshes: Vec<Mesh>
}
impl RenderObject {
    pub fn from_mesh(mesh: Mesh) -> Self {
        RenderObject { 
            position: vec3(0.0, 0.0, 0.0), 
            rotation: vec3(0.0, 0.0, 0.0), 
            scale: 1.0,
            meshes: vec![mesh]
        }
    }

    pub fn from_meshes(meshes: Vec<Mesh>) -> Self {
        RenderObject { 
            position: vec3(0.0, 0.0, 0.0), 
            rotation: vec3(0.0, 0.0, 0.0), 
            scale: 1.0,
            meshes
        }
    }

    pub fn position(&self) -> Vector3<f32> { self.position }
    pub fn set_position(&mut self, value: Vector3<f32>) { self.position = value }
    pub fn rotation(&self) -> Vector3<f32> { self.rotation }
    pub fn set_rotation(&mut self, value: Vector3<f32>) { self.rotation = value } 
    pub fn scale(&self) -> f32 { self.scale }
    pub fn set_scale(&mut self, value: f32) { self.scale = value }
    pub fn meshes(&self) -> &Vec<Mesh> { &self.meshes }
    pub fn set_meshes(&mut self, value: Vec<Mesh>) { self.meshes = value; }

    pub fn transform_matrix(&self) -> Matrix4<f32> {
        let pos_matrix = Matrix4::from_translation(self.position.clone());
        let rot_matrix = Matrix4::from_angle_x(Rad(self.rotation.x.to_radians())) * 
            Matrix4::from_angle_y(Rad(self.rotation.y.to_radians())) *
            Matrix4::from_angle_z(Rad(self.rotation.z.to_radians()));
        let sca_matrix = Matrix4::from_scale(self.scale.clone());
        pos_matrix * rot_matrix * sca_matrix
    }

    pub fn normal_matrix(&self, view_matrix: &Matrix4<f32>) -> Matrix3<f32> {
        let matrix = (view_matrix * self.transform_matrix()).invert().unwrap().transpose();
        let Matrix4{x, y, z, w: _} = matrix;
        Matrix3::from_cols(x.truncate(), y.truncate(), z.truncate())
    }
}
