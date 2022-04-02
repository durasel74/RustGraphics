use cgmath::{ Matrix, SquareMatrix, Matrix3, Matrix4, Vector3, vec3, Rad };
use super::{ Mesh, Material, Texture};

pub struct RenderObject {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: f32,
    mesh: Mesh,
    material: Material,
    shininess: f32,

    texture: Option<Texture>,
    light_map: Option<Texture>,
    
}
impl RenderObject {
    pub fn from_mesh(mesh: Mesh) -> Self {
        RenderObject { 
            position: vec3(0.0, 0.0, 0.0), 
            rotation: vec3(0.0, 0.0, 0.0), 
            scale: 1.0,
            mesh,
            material: Material::new(),
            shininess: 0.0,

            texture: None,
            light_map: None,
        }
    }

    pub fn position(&self) -> Vector3<f32> { self.position }
    pub fn set_position(&mut self, value: Vector3<f32>) { self.position = value }
    pub fn rotation(&self) -> Vector3<f32> { self.rotation }
    pub fn set_rotation(&mut self, value: Vector3<f32>) { self.rotation = value } 
    pub fn scale(&self) -> f32 { self.scale }
    pub fn set_scale(&mut self, value: f32) { self.scale = value }

    pub fn mesh(&self) -> &Mesh { &self.mesh }
    pub fn set_mesh(&mut self, value: Mesh) { self.mesh = value; }
    pub fn material(&self) -> &Material { &self.material }
    pub fn set_material(&mut self, value: Material) { self.material = value; }
    pub fn shininess(&self) -> f32 { self.shininess }
    pub fn set_shininess(&mut self, value: f32) { self.shininess = value; }

    pub fn texture(&self) -> &Option<Texture> { &self.texture }
    pub fn set_texture(&mut self, value: Texture) { self.texture = Some(value); }
    pub fn light_map(&self) -> &Option<Texture> { &self.light_map }
    pub fn set_light_map(&mut self, value: Texture) { self.light_map = Some(value); }

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
