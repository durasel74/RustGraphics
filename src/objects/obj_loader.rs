use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use cgmath::{ Vector2, Vector3 };
use obj::{ TexturedVertex, load_obj, Obj };
use obj::raw::{ self, material };
use super::{ Vertex, RenderObject, Material, Mesh };

pub fn load_model(model_path: &str) -> RenderObject {
    let input = BufReader::new(File::open(model_path).unwrap());
    let obj_model: Obj<TexturedVertex, u32> = load_obj(input).unwrap();

    let input = BufReader::new(File::open(model_path).unwrap());
    let obj = raw::parse_obj(input).unwrap();


    let mut ranges: Vec<(usize, usize)> = vec![];
    for mesh in obj.groups {
        // let mesh_name = mesh.0;
        let polygons = mesh.1.polygons[0];
        ranges.push((polygons.start * 3, polygons.end * 3));
    }

    let mut meshes: Vec<Mesh> = vec![];
    for i in ranges {
        let range_indices = &obj_model.indices[i.0..i.1];
        let min_index = *range_indices.iter().min().unwrap() as usize;
        let max_index = *range_indices.iter().max().unwrap() as usize;

        let vertices: Vec<TexturedVertex> = obj_model.vertices[min_index..=max_index].to_vec();
        let mut indices: Vec<u32> = vec![];
        for j in range_indices { indices.push(*j - min_index as u32); }
        let vertices = obj_vertex_to_vertex(vertices);
        meshes.push(Mesh::from_vertices(vertices, indices));
    }
    RenderObject::from_meshes(meshes)
}

pub fn load_model_shredded(model_path: &str) -> RenderObject {
    let input = BufReader::new(File::open(model_path).unwrap());
    let obj_model: Obj<TexturedVertex, u32> = load_obj(input).unwrap();

    let input = BufReader::new(File::open(model_path).unwrap());
    let obj = raw::parse_obj(input).unwrap();


    let mut ranges: Vec<(usize, usize)> = vec![];
    for mesh in obj.groups {
        // let mesh_name = mesh.0;
        let polygons = mesh.1.polygons[0];
        ranges.push((polygons.start * 3, polygons.end * 3));
    }

    let mut meshes: Vec<Mesh> = vec![];
    for i in ranges {
        let range_indices = &obj_model.indices[i.0..i.1];
        let min_index = *range_indices.iter().min().unwrap() as usize;
        let max_index = *range_indices.iter().max().unwrap() as usize;

        let vertices: Vec<TexturedVertex> = obj_model.vertices[min_index..=max_index].to_vec();
        let mut indices: Vec<u32> = vec![];
        for j in range_indices { indices.push(*j - min_index as u32); }
        let vertices = obj_vertex_to_vertex_shredded(vertices);
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






fn obj_vertex_to_vertex(obj_vertices: Vec<TexturedVertex>) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    for i in obj_vertices {
        let pos = Vector3 { x: i.position[0], y: i.position[1], z: i.position[2] };
        let norm = Vector3 { x: i.normal[0], y: i.normal[1], z: i.normal[2] };
        let tex = Vector2 { x: i.texture[0], y: i.texture[1] };
        let vertex = Vertex { position: pos, normal: norm, tex_coords: tex };
        vertices.push(vertex);
    }
    vertices
}

fn obj_vertex_to_vertex_shredded(obj_vertices: Vec<TexturedVertex>) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    let pos_y = obj_vertices[0].position[1];
    let pos_z = obj_vertices[0].position[2];
    for i in obj_vertices {
        let pos = Vector3 { x: i.position[0], y: i.position[1] + pos_y, z: i.position[2] + (pos_z) * 4.0 };
        let norm = Vector3 { x: i.normal[0], y: i.normal[1], z: i.normal[2] };
        let tex = Vector2 { x: i.texture[0], y: i.texture[1] };
        let vertex = Vertex { position: pos, normal: norm, tex_coords: tex };
        vertices.push(vertex);
    }
    vertices
}
