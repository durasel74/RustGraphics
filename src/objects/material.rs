use cgmath::{ Vector3, vec3 };

pub struct Material {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub shininess: f32,
}

impl Material {
    pub fn new() -> Self {
        Material {
            ambient: vec3(1.0, 1.0, 1.0),
            diffuse: vec3(1.0, 1.0, 1.0),
            specular: vec3(1.0, 1.0, 1.0),
            shininess: 0.0,
        }
    }
}
