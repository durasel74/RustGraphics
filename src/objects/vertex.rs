use cgmath::{ Vector2, Vector3, vec2, vec3 };

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: Vector2<f32>,
}
