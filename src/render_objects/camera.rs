use cgmath::prelude::InnerSpace;
use cgmath::{ Matrix, Matrix4, Vector3, vec3, vec4 };

pub struct Camera {
    position: Vector3<f32>,
    target: Vector3<f32>,
    direction: Vector3<f32>,
}
impl Camera {
    pub fn new() -> Self {
        let position = vec3(0.0, 0.0, 0.0);
        let target = vec3(0.0, 0.0, 0.0);
        let direction = vec3(0.0, 0.0, 1.0);
        Camera { position, target, direction }
    }

    pub fn position(&self) -> Vector3<f32> { self.position }
    pub fn set_position(&mut self, value: Vector3<f32>) { self.position = value }
    pub fn target(&self) -> Vector3<f32> { self.target }
    pub fn set_target(&mut self, value: Vector3<f32>) { self.target = value }

    // pub fn direction(&self) -> Vector3<f32> {
    //     (&self.position - &self.target).normalize()
    // }

    pub fn direction(&self) -> Vector3<f32> { self.direction }
    pub fn set_direction(&mut self, value: Vector3<f32>) { self.direction = value; }

    pub fn right(&self) -> Vector3<f32> {
        let cross = vec3(0.0, 1.0, 0.0).cross(self.direction());
        cross.normalize()
    }
    
    pub fn up(&self) -> Vector3<f32> {
        self.direction().cross(self.right())
    }

    pub fn lookat_matrix(&self) -> Matrix4<f32> {
        let direction_matrix = Matrix4::from_cols(
            self.right().extend(0.0),
            self.up().extend(0.0),
            self.direction().extend(0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        );
        let pos = vec3(-self.position.x, -self.position.y, -self.position.z);
        let position_matrix = Matrix4::from_translation(pos);
        return direction_matrix.transpose() * position_matrix;
    }
}
