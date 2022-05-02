use std::collections::hash_map::HashMap;
use std::io::{ Seek, BufReader };
use std::fs::File;
use std::path::Path;
use cgmath::{ Vector2, Vector3 };
use obj::{ TexturedVertex, Obj, load_obj };
use obj::raw::{ self, RawObj, material };
use super::{ Vertex, RenderObject, Material, Mesh };

pub fn load_model(model_path: &str) -> RenderObject {
    let mut file_buf = BufReader::new(File::open(model_path).unwrap());
    let obj_data = raw::parse_obj(&mut file_buf).unwrap();
    file_buf.rewind().unwrap();
    let obj_model: Obj<TexturedVertex, u32> = load_obj(&mut file_buf).unwrap();

    let ranges = get_ranges(&obj_data);
    let material_ranges = get_material_ranges(&obj_data, &ranges);


    let folder_path = Path::new(model_path).parent().unwrap().to_str().unwrap();
    let mtl_path = format!("{}/{}", folder_path, &obj_data.material_libraries[0]);
    let meshes = create_meshes(material_ranges, obj_model, mtl_path);  
    RenderObject::from_meshes(meshes)

    // let material = load_material(&mtl_path);
}

pub fn load_material(material_path: &str) -> Material {
    let input = BufReader::new(File::open(material_path).unwrap());
    let mtl = material::parse_mtl(input).unwrap();
    let mut mat: Option<&material::Material> = None;
    for i in mtl.materials.values() { mat = Some(i); break; };
    let folder_path = Path::new(material_path).parent().unwrap().to_str().unwrap();
    Material::from_mtl(mat.unwrap(), &folder_path)
}

fn get_ranges(obj_data: &RawObj) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = vec![];
    for group in obj_data.groups.iter() {
        // let mesh_name = mesh.0;
        let polygons = group.1.polygons[0];
        ranges.push((polygons.start * 3, polygons.end * 3));
    }
    ranges
}

fn get_material_ranges(obj_data: &RawObj, mesh_ranges: &Vec<(usize, usize)>) 
-> HashMap<(usize, usize), String> {
    let material_groups = &obj_data.meshes;

    let mut material_ranges: HashMap<(usize, usize), String> = HashMap::new();
    for range in mesh_ranges {
        'outer: for material in material_groups.iter() {
            let material_name = material.0.clone();

            for polygons in material.1.polygons.iter() {
                let material_start = polygons.start * 3;
                let material_end = polygons.end * 3;

                if material_start >= range.0 && material_start < range.1 
                && material_end > range.0 && material_end <= range.1 {
                    material_ranges.entry(range.to_owned()).or_insert(material_name);
                    break 'outer;
                }
            }
        }
    }
    material_ranges
}

fn create_meshes(material_ranges: HashMap<(usize, usize), String>, 
obj_model: Obj<TexturedVertex, u32>, mtl_path: String) -> Vec<Mesh> {
    let input = BufReader::new(File::open(&mtl_path).unwrap());
    let mtl = material::parse_mtl(input).unwrap();

    let mut meshes: Vec<Mesh> = vec![];
    for i in material_ranges {
        let material_name = i.1;
        let range = i.0;

        let range_indices = &obj_model.indices[range.0..range.1];
        let min_index = *range_indices.iter().min().unwrap() as usize;
        let max_index = *range_indices.iter().max().unwrap() as usize;

        let vertices: Vec<TexturedVertex> = obj_model.vertices[min_index..=max_index].to_vec();
        let mut indices: Vec<u32> = vec![];
        for j in range_indices { indices.push(*j - min_index as u32); }
        let vertices = obj_vertex_to_vertex(vertices);

        let mtl_material = mtl.materials.get(&material_name).unwrap();
        let folder_path = Path::new(&mtl_path).parent().unwrap().to_str().unwrap();
        let material = Material::from_mtl(mtl_material, &folder_path);

        let mut new_mesh = Mesh::from_vertices(vertices, indices);
        new_mesh.set_material(material);
        meshes.push(new_mesh);
    }
    meshes
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







////////////////////////////////////////////////////////////////////////////////
pub fn load_with_paths(model_path: &str, material_path: &str) -> Mesh {
    let mut mesh = load_model_old(model_path);
    let material = load_material_old(material_path);
    mesh.set_material(material);
    return mesh
}
pub fn load_model_old(model_path: &str) -> Mesh {
    let input = BufReader::new(File::open(model_path).unwrap());
    let obj_model: Obj<TexturedVertex, u32> = load_obj(input).unwrap();
    Mesh::from_obj(&obj_model)
}
pub fn load_material_old(material_path: &str) -> Material {
    let input = BufReader::new(File::open(material_path).unwrap());
    let mtl = material::parse_mtl(input).unwrap();
    let mut mat: Option<&material::Material> = None;
    for i in mtl.materials.values() { mat = Some(i); break; };
    let folder_path = Path::new(material_path).parent().unwrap().to_str().unwrap();
    Material::from_mtl(mat.unwrap(), &folder_path)
}
////////////////////////////////////////////////////////////////////////////////



////////////////////////////////////////////////////////////////////////////////
pub fn load_model_shredded(model_path: &str) -> RenderObject {
    let mut file_buf = BufReader::new(File::open(model_path).unwrap());
    let obj_data = raw::parse_obj(&mut file_buf).unwrap();
    file_buf.rewind().unwrap();
    let obj_model: Obj<TexturedVertex, u32> = load_obj(&mut file_buf).unwrap();

    let ranges = get_ranges(&obj_data);
    let meshes = create_meshes_by_ranges_shredded(ranges, obj_model);
    RenderObject::from_meshes(meshes)
}

fn create_meshes_by_ranges_shredded(ranges: Vec<(usize, usize)>, 
obj_model: Obj<TexturedVertex, u32>) -> Vec<Mesh> {
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
    meshes
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
////////////////////////////////////////////////////////////////////////////////
