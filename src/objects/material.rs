use cgmath::{ Vector3, vec3 };
use obj::raw::material;

#[derive(Clone)]
pub struct Material {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub specular_exponent: f32,
}

impl Material {
    pub fn new() -> Self {
        Material {
            ambient: vec3(0.0, 0.0, 0.0),
            diffuse: vec3(0.5, 0.5, 0.5),
            specular: vec3(0.5, 0.5, 0.5),
            specular_exponent: 0.0,
        }
    }

    pub fn from_mtl(material: &material::Material) -> Self {
        let mut ambient = vec3(0.0, 0.0, 0.0);
        if let Some(value) = &material.ambient {
            ambient = Self::mtl_color_to_vector(&value);
        }
        let mut diffuse = vec3(0.5, 0.5, 0.5);
        if let Some(value) = &material.diffuse {
            diffuse = Self::mtl_color_to_vector(&value);
        }
        let mut specular = vec3(0.5, 0.5, 0.5);
        if let Some(value) = &material.specular {
            specular = Self::mtl_color_to_vector(&value);
        }
        let mut specular_exponent = 0.0;
        if let Some(value) = material.specular_exponent {
            specular_exponent = value;
        }
        Material { ambient, diffuse, specular, specular_exponent }
    }

    fn mtl_color_to_vector(color: &material::MtlColor) -> Vector3<f32> {
        match color {
            material::MtlColor::Rgb(r, g, b) => vec3(*r, *g, *b),
            _ => vec3(0.0, 0.0, 0.0)
        }
    }
}
