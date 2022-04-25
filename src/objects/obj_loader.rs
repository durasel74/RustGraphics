use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use obj::{ load_obj, Obj, TexturedVertex };
use obj::raw::material;
use super::{ Mesh, Material };

pub fn load_with_paths(model_path: &str, material_path: &str) -> Mesh {
    let mut mesh = load_model(model_path);
    let material = load_material(material_path);
    mesh.set_material(material);
    return mesh
}

pub fn load_model(model_path: &str) -> Mesh {
    let input = BufReader::new(File::open(model_path).unwrap());
    let obj_model: Obj<TexturedVertex, u16> = load_obj(input).unwrap();
    Mesh::from_obj(&obj_model)
}

pub fn load_material(material_path: &str) -> Material {
    let input = BufReader::new(File::open(material_path).unwrap());
    let mtl = material::parse_mtl(input).unwrap();
    let mut mat: Option<&material::Material> = None;
    for i in mtl.materials.values() { mat = Some(i); break; };
    Material::from_mtl(mat.unwrap())
}
