use cgmath::{ Vector3, vec3 };
use obj::raw::material;
use super::{ Texture };

#[derive(Clone)]
pub struct Material {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub specular_exponent: f32,
    pub dissolve: f32,
    pub diff_tex: Option<Texture>,
    pub spec_tex: Option<Texture>,
}

impl Material {
    pub fn new() -> Self {
        Material {
            ambient: vec3(0.0, 0.0, 0.0),
            diffuse: vec3(0.8, 0.8, 0.8),
            specular: vec3(0.8, 0.8, 0.8),
            specular_exponent: 0.0,
            dissolve: 0.0,
            diff_tex: None,
            spec_tex: None,
        }
    }

    pub fn from_mtl(material: &material::Material, folder_path: &str) -> Self {
        let mut ambient = vec3(0.0, 0.0, 0.0);
        if let Some(value) = &material.ambient {
            ambient = Self::mtl_color_to_vector(&value);
        }
        let mut diffuse = vec3(0.8, 0.8, 0.8);
        if let Some(value) = &material.diffuse {
            diffuse = Self::mtl_color_to_vector(&value);
        }
        let mut specular = vec3(0.8, 0.8, 0.8);
        if let Some(value) = &material.specular {
            specular = Self::mtl_color_to_vector(&value);
        }
        let mut specular_exponent = 0.0;
        if let Some(value) = material.specular_exponent {
            specular_exponent = value;
        }
        let mut dissolve = 0.0;
        if let Some(value) = material.dissolve {
            dissolve = value;
        }

        let mut diff_tex = None;
        if let Some(tex) = &material.diffuse_map {
            let tex_path = format!("{}/{}", folder_path, &tex.file);
            diff_tex = Some(Texture::from_file(&tex_path).unwrap());
        };
        let mut spec_tex = None;
        if let Some(tex) = &material.specular_map {
            let tex_path = format!("{}/{}", folder_path, &tex.file);
            spec_tex = Some(Texture::from_file(&tex_path).unwrap());
        };
        Material { ambient, diffuse, specular, specular_exponent, dissolve, diff_tex, spec_tex }
    }

    fn mtl_color_to_vector(color: &material::MtlColor) -> Vector3<f32> {
        match color {
            material::MtlColor::Rgb(r, g, b) => vec3(*r, *g, *b),
            _ => vec3(0.0, 0.0, 0.0)
        }
    }
}
