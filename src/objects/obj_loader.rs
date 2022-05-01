use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use obj::{ load_obj, Obj, TexturedVertex };
use obj::raw::{ self, material };
use super::{ Mesh, Material, RenderObject };

pub fn load_model(model_path: &str) -> RenderObject {
    let input = BufReader::new(File::open(model_path).unwrap());
    let obj_model: Obj<TexturedVertex, u32> = load_obj(input).unwrap();

    let input = BufReader::new(File::open(model_path).unwrap());
    println!("{}", model_path);
    let obj = raw::parse_obj(input).unwrap();

    let mut ranges: Vec<(usize, usize)> = vec![];
    println!("meshes");
    for mesh in obj.groups {
        let mesh_name = mesh.0;
        let polygons = mesh.1.polygons[0];
        println!("    {}", mesh_name);
        println!("    {} {}", polygons.start, polygons.end);
        ranges.push((polygons.start * 3, polygons.end * 3));
    }

    let mut meshes: Vec<Mesh> = vec![];
    for i in ranges {
        let range_indices = &obj_model.indices[i.0..i.1];
        let min_index = *range_indices.iter().min().unwrap() as usize;
        let max_index = *range_indices.iter().max().unwrap() as usize;

        let vertices: Vec<TexturedVertex> = obj_model.vertices[min_index..=max_index].to_vec();
        let mut indices: Vec<u32> = vec![];
        for j in range_indices {
            indices.push(*j - min_index as u32);
        }
        meshes.push(Mesh::from_vertices(vertices, indices));
    }
    RenderObject::from_meshes(meshes)
}

pub fn load_with_paths(model_path: &str, material_path: &str) -> Mesh {
    let mut mesh = load_model_old(model_path);
    let material = load_material(material_path);
    mesh.set_material(material);
    return mesh
}

pub fn load_model_old(model_path: &str) -> Mesh {
    let input = BufReader::new(File::open(model_path).unwrap());
    let obj_model: Obj<TexturedVertex, u32> = load_obj(input).unwrap();
    Mesh::from_obj(&obj_model)
}

pub fn load_material(material_path: &str) -> Material {
    let input = BufReader::new(File::open(material_path).unwrap());
    let mtl = material::parse_mtl(input).unwrap();
    let mut mat: Option<&material::Material> = None;
    for i in mtl.materials.values() { mat = Some(i); break; };
    let folder_path = Path::new(material_path).parent().unwrap().to_str().unwrap();
    Material::from_mtl(mat.unwrap(), &folder_path)
}
