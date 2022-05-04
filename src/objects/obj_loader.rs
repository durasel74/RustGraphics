use std::io::{ Seek, BufReader };
use std::fs::File;
use std::path::Path;
use cgmath::{ Vector2, Vector3 };
use obj::{ TexturedVertex, Obj, load_obj };
use obj::raw::{ self, RawObj, RawMtl, material };
use obj::raw::object::{ Range };
use super::{ Vertex, RenderObject, Material, Mesh };

pub fn load_model(model_path: &str) -> RenderObject {
    let mut file_buf = BufReader::new(File::open(model_path).unwrap());
    let obj_data = raw::parse_obj(&mut file_buf).unwrap();
    file_buf.rewind().unwrap();
    let obj_model: Obj<TexturedVertex, u32> = load_obj(&mut file_buf).unwrap();

    let ranges = get_ranges(&obj_data);
    let material_ranges = get_material_ranges(&obj_data, &ranges);
    let mtl_data = load_mtl(&obj_data, model_path);
    
    let meshes = create_meshes(&obj_model, &material_ranges, &mtl_data, model_path);  
    RenderObject::from_meshes(meshes)
}

fn get_ranges(obj_data: &RawObj) -> Vec<Range> {
    let mut ranges: Vec<Range> = vec![];
    for group in obj_data.groups.iter() {
        // let mesh_name = mesh.0;
        let polygons = group.1.polygons[0];
        ranges.push(Range { start: polygons.start * 3, end: polygons.end * 3 });
    }
    ranges
}

fn get_material_ranges(obj_data: &RawObj, mesh_ranges: &Vec<Range>) 
-> Vec<(Range, String)> {
    let mut material_ranges: Vec<(Range, String)> = Vec::with_capacity(mesh_ranges.len());

    for range in mesh_ranges {
        let mut material_name = "".to_string();
        'outer: for material in obj_data.meshes.iter() {
            for polygons in material.1.polygons.iter() {
                let material_start = polygons.start * 3;
                let material_end = polygons.end * 3;

                if (material_start >= range.start && material_end <= range.end)
                || (range.start >= material_start && range.end <= material_end) {
                    material_name = material.0.clone();
                    break 'outer;
                }
            }
        }
        material_ranges.push((range.to_owned(), material_name));
    }
    material_ranges
}

fn load_mtl(obj_data: &RawObj, obj_path: &str) -> RawMtl {
    let folder_path = Path::new(obj_path).parent().unwrap().to_str().unwrap();
    let mtl_path = format!("{}/{}", folder_path, &obj_data.material_libraries[0]);
    let input = BufReader::new(File::open(&mtl_path).unwrap());
    material::parse_mtl(input).unwrap()
}

fn create_meshes(obj_model: &Obj<TexturedVertex, u32>, ranges: &Vec<(Range, String)>, 
mtl_data: &RawMtl, obj_path: &str) -> Vec<Mesh> {
    let mut meshes: Vec<Mesh> = vec![];
    for i in ranges {
        let material_name = i.1.clone();
        let range = i.0;
        let mut new_mesh = select_mesh(obj_model, range);
        attach_material(&mut new_mesh, mtl_data, &material_name, obj_path);
        meshes.push(new_mesh);
    }
    meshes
}

fn select_mesh(obj_model: &Obj<TexturedVertex, u32>, range: Range) -> Mesh {
    let range_indices = &obj_model.indices[range.start..range.end];
    let min_index = *range_indices.iter().min().unwrap() as usize;
    let max_index = *range_indices.iter().max().unwrap() as usize;
    let vertices: Vec<TexturedVertex> = obj_model.vertices[min_index..=max_index].to_vec();
    let vertices = obj_vertex_to_vertex(vertices);

    let mut indices: Vec<u32> = vec![];
    for j in range_indices { 
        indices.push(*j - min_index as u32); 
    }
    Mesh::from_vertices(vertices, indices)
}

fn attach_material(mesh: &mut Mesh, mtl_data: &RawMtl, material_name: &str, obj_path: &str) {
    let mtl_material = mtl_data.materials.get(material_name);
    if let Some(material) = mtl_material {
        let folder_path = Path::new(obj_path).parent().unwrap().to_str().unwrap();
        let material = Material::from_mtl(material, &folder_path);
        mesh.set_material(material);
    }
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
